use crate::parser::parol_grammar_trait::ParolGrammarTrait;
use id_tree::Tree;
use log::trace;
use miette::{miette, Result};
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType};
use std::fmt::{Debug, Display, Error, Formatter};

// To rebuild the parser sources from scratch use the command build_parsers.ps1

// Test run:
// cargo run --bin parol -- -f .\src\parser\parol-grammar.par -v

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Factor {
    Group(Alternations),
    Repeat(Alternations),
    Optional(Alternations),
    Terminal(String, Vec<usize>),
    NonTerminal(String),
    ScannerSwitch(usize),
    ScannerSwitchPush(usize),
    ScannerSwitchPop,
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            Self::Group(g) => write!(f, "({})", g),
            Self::Repeat(r) => write!(f, "{{{}}}", r),
            Self::Optional(o) => write!(f, "[{}]", o),
            Self::Terminal(t, s) => write!(
                f,
                "<{}>T({})",
                s.iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<String>>()
                    .join(", "),
                t
            ),
            Self::NonTerminal(n) => write!(f, "N({})", n),
            Self::ScannerSwitch(n) => write!(f, "S({})", n),
            Self::ScannerSwitchPush(n) => write!(f, "Push({})", n),
            Self::ScannerSwitchPop => write!(f, "Pop"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Alternation(pub Vec<Factor>);

impl Display for Alternation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(
            f,
            "Alt({})",
            self.0
                .iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Alternation {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn push(&mut self, fac: Factor) {
        self.0.push(fac)
    }

    fn reverse(&mut self) {
        self.0.reverse()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Alternations(pub Vec<Alternation>);

impl Alternations {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, alt: Alternation) {
        self.0.push(alt)
    }

    fn reverse(&mut self) {
        self.0.reverse()
    }
}

impl Display for Alternations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(
            f,
            "Alts({})",
            self.0
                .iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<String>>()
                .join(" | ")
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Production {
    pub lhs: String,
    pub rhs: Alternations,
}

impl Production {
    fn new(lhs: String, rhs: Alternations) -> Self {
        Self { lhs, rhs }
    }
}

impl Display for Production {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{}: {};", self.lhs, self.rhs)
    }
}

#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub name: String,
    pub line_comments: Vec<String>,
    pub block_comments: Vec<(String, String)>,
    pub auto_newline_off: bool,
    pub auto_ws_off: bool,
}

#[derive(Debug, Clone)]
pub enum ParolGrammarItem {
    Prod(Production),
    Alts(Alternations),
    Alt(Alternation),
    Fac(Factor),
    StateList(Vec<usize>),
}

impl Display for ParolGrammarItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match self {
            Self::Prod(p) => write!(f, "{}", p),
            Self::Alts(a) => write!(f, "{}", a),
            Self::Alt(a) => write!(f, "{}", a),
            Self::Fac(t) => write!(f, "{}", t),
            Self::StateList(s) => write!(
                f,
                "SL<{}>",
                s.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl Display for ScannerConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "name: {};", self.name)?;
        write!(f, "line_comments: {:?};", self.line_comments)?;
        write!(f, "block_comments: {:?};", self.block_comments)?;
        write!(f, "auto_newline_off: {};", self.auto_newline_off)?;
        write!(f, "auto_ws_off: {};", self.auto_ws_off)
    }
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            name: "INITIAL".to_owned(),
            line_comments: Vec::new(),
            block_comments: Vec::new(),
            auto_newline_off: false,
            auto_ws_off: false,
        }
    }
}

///
/// Data structure used to build up a parol::GrammarConfig during parsing.
///
#[derive(Debug, Clone)]
pub struct ParolGrammar {
    pub item_stack: Vec<ParolGrammarItem>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub start_symbol: String,
    pub scanner_configurations: Vec<ScannerConfig>,
    current_scanner: ScannerConfig,
}

impl Default for ParolGrammar {
    fn default() -> Self {
        Self {
            item_stack: Vec::new(),
            title: None,
            comment: None,
            start_symbol: String::default(),
            scanner_configurations: vec![ScannerConfig::default()],
            current_scanner: ScannerConfig::default(),
        }
    }
}

impl ParolGrammar {
    pub fn new() -> Self {
        ParolGrammar::default()
    }

    #[allow(dead_code)]
    // Use this function for debugging purposes:
    // $env:RUST_LOG="parol::parser=trace"
    //
    // trace!("{}", self.trace_item_stack(context));
    fn trace_item_stack(&self, context: &str) -> String {
        format!(
            "Item stack at {}:\n{}",
            context,
            self.item_stack
                .iter()
                .rev()
                .map(|s| format!("  {}", s))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    fn handle_scanner_state(&mut self, context: &str) -> Result<()> {
        let l = self.pop(context);
        let s = self.pop(context);

        match (&l, &s) {
            (
                Some(ParolGrammarItem::StateList(l)),
                Some(ParolGrammarItem::Fac(Factor::NonTerminal(s))),
            ) => {
                if let Some(scanner_state) = self
                    .scanner_configurations
                    .iter()
                    .position(|sc| sc.name == *s)
                {
                    let mut l = l.clone();
                    l.push(scanner_state);
                    self.push(ParolGrammarItem::StateList(l), context);
                    trace!("{}", self.trace_item_stack(context));
                    Ok(())
                } else {
                    Err(miette!("{}: Unknown scanner name '{}'", context, s))
                }
            }
            _ => Err(miette!(
                "{}: Expected [StateList, Factor::NonTerminal] on TOS, found [{:?}, {:?}]",
                context,
                l,
                s
            )),
        }
    }

    fn push(&mut self, item: ParolGrammarItem, context: &str) {
        trace!("push   {}: {}", context, item);
        self.item_stack.push(item)
    }

    fn pop(&mut self, context: &str) -> Option<ParolGrammarItem> {
        if !self.item_stack.is_empty() {
            let item = self.item_stack.pop();
            if let Some(ref item) = item {
                trace!("pop    {}: {}", context, item);
            }
            item
        } else {
            None
        }
    }
}

impl Display for ParolGrammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        writeln!(f, "title: {:?}", self.title)?;
        writeln!(f, "comment: {:?}", self.comment)?;
        writeln!(f, "start_symbol: {}", self.start_symbol)?;
        writeln!(f, "current_scanner: {}", self.current_scanner.name)?;
        writeln!(
            f,
            "{}",
            self.scanner_configurations
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<String>>()
                .join("\n")
        )?;
        writeln!(
            f,
            "{}",
            self.item_stack
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl ParolGrammarTrait for ParolGrammar {
    /// Semantic action for production 2:
    ///
    /// StartDeclaration: "%start" Identifier;
    ///
    fn start_declaration_2(
        &mut self,
        _end_of_input_0: &ParseTreeStackEntry,
        _identifier_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "start_declaration_2";
        if let Some(ParolGrammarItem::Fac(Factor::NonTerminal(s))) = self.pop(context) {
            self.start_symbol = s;
            Ok(())
        } else {
            Err(miette!(
                "{}: Expected 'Fac(Factor::NonTerminal)' on TOS.",
                context
            ))
        }
    }

    /// Semantic action for production 4:
    ///
    /// Declarations: ;
    ///
    fn declarations_4(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        let context = "declarations_4";
        trace!("{}", self.trace_item_stack(context));
        self.scanner_configurations[0] = self.current_scanner.clone();
        self.current_scanner = ScannerConfig::default();
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// Declaration: "%title" String;
    ///
    fn declaration_5(
        &mut self,
        _percent_title_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "declaration_5";
        if let Some(ParolGrammarItem::Fac(Factor::Terminal(s, _))) = self.pop(context) {
            self.title = Some(s);
            Ok(())
        } else {
            Err(miette!(
                "{}: Expected 'Fac(Factor::Terminal)' on TOS.",
                context
            ))
        }
    }

    /// Semantic action for production 6:
    ///
    /// Declaration: "%comment" String;
    ///
    fn declaration_6(
        &mut self,
        _percent_comment_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "declaration_6";
        if let Some(ParolGrammarItem::Fac(Factor::Terminal(s, _))) = self.pop(context) {
            self.comment = Some(s);
            Ok(())
        } else {
            Err(miette!(
                "{}: Expected 'Fac(Factor::Terminal)' on TOS.",
                context
            ))
        }
    }

    /// Semantic action for production 8:
    ///
    /// Declaration: "%line_comment" String;
    ///
    fn scanner_directives_8(
        &mut self,
        _percent_line_underscore_comment_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "scanner_directives_8";
        if let Some(ParolGrammarItem::Fac(Factor::Terminal(s, _))) = self.pop(context) {
            self.current_scanner.line_comments.push(s);
            Ok(())
        } else {
            Err(miette!(
                "{}: Expected 'Fac(Factor::Terminal)' on TOS.",
                context
            ))
        }
    }

    /// Semantic action for production 9:
    ///
    /// ScannerDirectives: "%block_comment" String String;
    ///
    fn scanner_directives_9(
        &mut self,
        _percent_block_underscore_comment_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _string_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "scanner_directives_9";
        if let Some(ParolGrammarItem::Fac(Factor::Terminal(s1, _))) = self.pop(context) {
            if let Some(ParolGrammarItem::Fac(Factor::Terminal(s2, _))) = self.pop(context) {
                self.current_scanner.block_comments.push((s2, s1));
                Ok(())
            } else {
                Err(miette!(
                    "{}: Expected 'Fac(Factor::Terminal)' on TOS.",
                    context
                ))
            }
        } else {
            Err(miette!(
                "{}: Expected 'Fac(Factor::Terminal)' on TOS.",
                context
            ))
        }
    }

    /// Semantic action for production 10:
    ///
    /// ScannerDirectives: "%auto_newline_off";
    ///
    fn scanner_directives_10(
        &mut self,
        _percent_auto_underscore_newline_underscore_off_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let _context = "scanner_directives_10";
        self.current_scanner.auto_newline_off = true;
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// ScannerDirectives: "%auto_ws_off";
    ///
    fn scanner_directives_11(
        &mut self,
        _percent_auto_underscore_ws_underscore_off_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let _context = "scanner_directives_11";
        self.current_scanner.auto_ws_off = true;
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// Production: Identifier ":" Alternations ";";
    ///
    fn production_17(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _colon_1: &ParseTreeStackEntry,
        _alternations_2: &ParseTreeStackEntry,
        _semicolon_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "production_17";
        if let Some(ParolGrammarItem::Alts(mut rhs)) = self.pop(context) {
            if let Some(ParolGrammarItem::Fac(Factor::NonTerminal(lhs))) = self.pop(context) {
                rhs.reverse();
                self.push(ParolGrammarItem::Prod(Production::new(lhs, rhs)), context);
                Ok(())
            } else {
                Err(miette!(
                    "{}: Expected 'Fac(Factor::NonTerminal)' on TOS.",
                    context
                ))
            }
        } else {
            Err(miette!("{}: Expected 'Alts' on TOS.", context))
        }
    }

    /// Semantic action for production 18:
    ///
    /// Alternations: Alternation AlternationsList;
    ///
    fn alternations_18(
        &mut self,
        _alternation_0: &ParseTreeStackEntry,
        _alternations_list_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "alternations_18";
        if let Some(ParolGrammarItem::Alts(mut alts)) = self.pop(context) {
            if let Some(ParolGrammarItem::Alt(mut alt)) = self.pop(context) {
                alt.reverse();
                alts.push(alt);
                self.push(ParolGrammarItem::Alts(alts), context);
                Ok(())
            } else {
                Err(miette!("{}: Expected 'Alt' on TOS.", context))
            }
        } else {
            Err(miette!("{}: Expected 'Alts' on TOS.", context))
        }
    }

    /// Semantic action for production 19:
    ///
    /// AlternationsList: "\|" Alternation AlternationsList;
    ///
    fn alternations_list_19(
        &mut self,
        _or_0: &ParseTreeStackEntry,
        _alternation_1: &ParseTreeStackEntry,
        _alternations_list_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "alternations_list_19";
        if let Some(ParolGrammarItem::Alts(mut alts)) = self.pop(context) {
            if let Some(ParolGrammarItem::Alt(mut alt)) = self.pop(context) {
                alt.reverse();
                alts.push(alt);
                self.push(ParolGrammarItem::Alts(alts), context);
                Ok(())
            } else {
                Err(miette!("{}: Expected 'Alt' on TOS.", context))
            }
        } else {
            Err(miette!("{}: Expected 'Alts' on TOS.", context))
        }
    }

    /// Semantic action for production 20:
    ///
    /// AlternationsList: ;
    ///
    fn alternations_list_20(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        let context = "alternations_list_20";
        self.push(ParolGrammarItem::Alts(Alternations::new()), context);
        Ok(())
    }

    /// Semantic action for production 22:
    ///
    /// AlternationList: Factor AlternationList;
    ///
    fn alternation_list_22(
        &mut self,
        _factor_0: &ParseTreeStackEntry,
        _alternation_list_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "alternation_list_22";
        //trace!("{}", self.trace_item_stack(context));
        if let Some(ParolGrammarItem::Alt(mut alt)) = self.pop(context) {
            if let Some(ParolGrammarItem::Fac(fac)) = self.pop(context) {
                alt.push(fac);
                self.push(ParolGrammarItem::Alt(alt), context);
                Ok(())
            } else {
                Err(miette!("{}: Expected 'Fac' on TOS.", context))
            }
        } else {
            Err(miette!("{}: Expected 'Alt' on TOS.", context))
        }
    }

    /// Semantic action for production 23:
    ///
    /// AlternationList: ;
    ///
    fn alternation_list_23(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        let context = "alternation_list_23";
        self.push(ParolGrammarItem::Alt(Alternation::new()), context);
        Ok(())
    }

    /// Semantic action for production 33:
    ///
    /// TokenWithStates: "<" StateList ">" String;
    ///
    fn token_with_states_33(
        &mut self,
        _l_t_0: &ParseTreeStackEntry,
        _state_list_1: &ParseTreeStackEntry,
        _g_t_2: &ParseTreeStackEntry,
        _string_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "token_with_states_33";
        trace!("{}", self.trace_item_stack(context));
        if let Some(ParolGrammarItem::Fac(Factor::Terminal(s, _))) = self.pop(context) {
            if let Some(ParolGrammarItem::StateList(sc)) = self.pop(context) {
                self.push(ParolGrammarItem::Fac(Factor::Terminal(s, sc)), context);
                Ok(())
            } else {
                Err(miette!("{}: Expected 'StateList' on TOS.", context))
            }
        } else {
            Err(miette!("{}: Expected 'Factor::Terminal' on TOS.", context))
        }
    }

    /// Semantic action for production 34:
    ///
    /// Group: "\(" Factor Alternations "\)";
    ///
    fn group_34(
        &mut self,
        _l_paren_0: &ParseTreeStackEntry,
        _factor_1: &ParseTreeStackEntry,
        _alternations_2: &ParseTreeStackEntry,
        _r_paren_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "group_34";
        trace!("{}", self.trace_item_stack(context));
        if let Some(ParolGrammarItem::Alts(mut alts)) = self.pop(context) {
            if let Some(ParolGrammarItem::Fac(fac)) = self.pop(context) {
                if alts.0.is_empty() {
                    alts.push(Alternation(vec![fac]));
                } else {
                    alts.0[0].0.insert(0, fac);
                }
                self.push(ParolGrammarItem::Fac(Factor::Group(alts)), context);
                Ok(())
            } else {
                Err(miette!("{}: Expected 'Factor' on TOS.", context))
            }
        } else {
            Err(miette!("{}: Expected 'Alts' on TOS.", context))
        }
    }

    /// Semantic action for production 35:
    ///
    /// Optional: "\[" Factor Alternations "\]";
    ///
    fn optional_35(
        &mut self,
        _l_bracket_0: &ParseTreeStackEntry,
        _factor_1: &ParseTreeStackEntry,
        _alternations_2: &ParseTreeStackEntry,
        _r_bracket_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "optional_35";
        trace!("{}", self.trace_item_stack(context));
        if let Some(ParolGrammarItem::Alts(mut alts)) = self.pop(context) {
            if let Some(ParolGrammarItem::Fac(fac)) = self.pop(context) {
                if alts.0.is_empty() {
                    alts.push(Alternation(vec![fac]));
                } else {
                    alts.0[0].0.insert(0, fac);
                }
                self.push(ParolGrammarItem::Fac(Factor::Optional(alts)), context);
                Ok(())
            } else {
                Err(miette!("{}: Expected 'Factor' on TOS.", context))
            }
        } else {
            Err(miette!("{}: Expected 'Alts' on TOS.", context))
        }
    }

    /// Semantic action for production 36:
    ///
    /// Repeat: "\{" Factor Alternations "\}";
    ///
    fn repeat_36(
        &mut self,
        _l_brace_0: &ParseTreeStackEntry,
        _factor_1: &ParseTreeStackEntry,
        _alternations_2: &ParseTreeStackEntry,
        _r_brace_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "repeat_36";
        trace!("{}", self.trace_item_stack(context));
        if let Some(ParolGrammarItem::Alts(mut alts)) = self.pop(context) {
            if let Some(ParolGrammarItem::Fac(fac)) = self.pop(context) {
                if alts.0.is_empty() {
                    alts.push(Alternation(vec![fac]));
                } else {
                    alts.0[0].0.insert(0, fac);
                }
                self.push(ParolGrammarItem::Fac(Factor::Repeat(alts)), context);
                Ok(())
            } else {
                Err(miette!("{}: Expected 'Factor' on TOS.", context))
            }
        } else {
            Err(miette!("{}: Expected 'Alts' on TOS.", context))
        }
    }

    /// Semantic action for production 37:
    ///
    /// Identifier: "[a-zA-Z_]\w*";
    ///
    fn identifier_37(
        &mut self,
        identifier_0: &ParseTreeStackEntry,
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "identifier_37";
        let parse_tree_item = identifier_0.get_parse_tree_type(parse_tree);
        if let ParseTreeType::T(t) = parse_tree_item {
            self.push(
                ParolGrammarItem::Fac(Factor::NonTerminal(t.symbol.to_owned())),
                context,
            );
            Ok(())
        } else {
            Err(miette!(
                "{}: Token expected, found {}",
                context,
                parse_tree_item
            ))
        }
    }

    /// Semantic action for production 38:
    ///
    /// String: "\u{0022}([^\\]|\\.)*?\u{0022}";
    ///
    fn string_38(
        &mut self,
        string_0: &ParseTreeStackEntry,
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "string_38";
        let parse_tree_item = string_0.get_parse_tree_type(parse_tree);
        if let ParseTreeType::T(t) = parse_tree_item {
            // Trim double quotes here
            let s = t.symbol.trim_matches('"').to_owned();
            self.push(ParolGrammarItem::Fac(Factor::Terminal(s, vec![0])), context);
            Ok(())
        } else {
            Err(miette!(
                "{}: Token expected, found {}",
                context,
                parse_tree_item
            ))
        }
    }

    /// Semantic action for production 39:
    ///
    /// ScannerState: "%scanner" Identifier "\{" ScannerStateList "\}";
    ///
    fn scanner_state_39(
        &mut self,
        _percent_scanner_0: &ParseTreeStackEntry,
        _identifier_1: &ParseTreeStackEntry,
        _l_brace_2: &ParseTreeStackEntry,
        _scanner_state_list_3: &ParseTreeStackEntry,
        _r_brace_4: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "scanner_state_39";
        trace!("{}", self.trace_item_stack(context));
        if let Some(ParolGrammarItem::Fac(Factor::NonTerminal(n))) = self.pop(context) {
            trace!("{}", self);
            self.current_scanner.name = n;
            self.scanner_configurations
                .push(self.current_scanner.clone());
            self.current_scanner = ScannerConfig::default();
            trace!("{}", self);
            Ok(())
        } else {
            Err(miette!(
                "{}: Expected 'Factor::NonTerminal' on TOS.",
                context
            ))
        }
    }

    /// Semantic action for production 42:
    ///
    /// StateList: Identifier StateListList;
    ///
    fn state_list_42(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _state_list_list_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "state_list_42";
        trace!("{}", self.trace_item_stack(context));
        self.handle_scanner_state(context)
    }

    /// Semantic action for production 43:
    ///
    /// StateListRest: "," Identifier StateListRest;
    ///
    fn state_list_rest_43(
        &mut self,
        _comma_0: &ParseTreeStackEntry,
        _identifier_1: &ParseTreeStackEntry,
        _state_list_rest_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "state_list_rest_43";
        trace!("{}", self.trace_item_stack(context));
        self.handle_scanner_state(context)
    }

    /// Semantic action for production 44:
    ///
    /// StateListRest: ;
    ///
    fn state_list_rest_44(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        let context = "state_list_rest_44";
        // Start with an empty state list
        self.push(ParolGrammarItem::StateList(vec![]), context);
        Ok(())
    }

    /// Semantic action for production 45:
    ///
    /// ScannerSwitch: "%sc" "\(" ScannerNameOpt "\)";
    ///
    fn scanner_switch_45(
        &mut self,
        _percent_sc_0: &ParseTreeStackEntry,
        _l_paren_1: &ParseTreeStackEntry,
        _scanner_name_opt_2: &ParseTreeStackEntry,
        _r_paren_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "scanner_switch_45";
        if let Some(ParolGrammarItem::Fac(Factor::NonTerminal(s))) = self.pop(context) {
            if let Some(scanner_state) = self
                .scanner_configurations
                .iter()
                .position(|sc| sc.name == *s)
            {
                self.push(
                    ParolGrammarItem::Fac(Factor::ScannerSwitch(scanner_state)),
                    context,
                );
                trace!("{}", self.trace_item_stack(context));
                Ok(())
            } else {
                Err(miette!("{}: Unknown scanner name '{}'", context, s))
            }
        } else {
            Err(miette!(
                "{}: Expected 'Fac(Factor::NonTerminal)' on TOS.",
                context
            ))
        }
    }

    /// Semantic action for production 46:
    ///
    /// ScannerSwitch: "%push" "\(" Identifier "\)";
    ///
    fn scanner_switch_46(
        &mut self,
        _percent_push_0: &ParseTreeStackEntry,
        _l_paren_1: &ParseTreeStackEntry,
        _identifier_2: &ParseTreeStackEntry,
        _r_paren_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "scanner_switch_46";
        if let Some(ParolGrammarItem::Fac(Factor::NonTerminal(s))) = self.pop(context) {
            if let Some(scanner_state) = self
                .scanner_configurations
                .iter()
                .position(|sc| sc.name == *s)
            {
                self.push(
                    ParolGrammarItem::Fac(Factor::ScannerSwitchPush(scanner_state)),
                    context,
                );
                trace!("{}", self.trace_item_stack(context));
                Ok(())
            } else {
                Err(miette!("{}: Unknown scanner name '{}'", context, s))
            }
        } else {
            Err(miette!(
                "{}: Expected 'Fac(Factor::NonTerminal)' on TOS.",
                context
            ))
        }
    }

    /// Semantic action for production 47:
    ///
    /// ScannerSwitch: "%pop" "\(" "\)";
    ///
    fn scanner_switch_47(
        &mut self,
        _percent_pop_0: &ParseTreeStackEntry,
        _l_paren_1: &ParseTreeStackEntry,
        _r_paren_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        let context = "scanner_switch_47";
        self.push(ParolGrammarItem::Fac(Factor::ScannerSwitchPop), context);
        trace!("{}", self.trace_item_stack(context));
        Ok(())
    }

    /// Semantic action for production 49:
    ///
    /// ScannerNameOpt: ;
    ///
    fn scanner_name_opt_49(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        let context = "scanner_name_opt_49";
        self.push(
            ParolGrammarItem::Fac(Factor::NonTerminal("INITIAL".to_string())),
            context,
        );
        Ok(())
    }
}
