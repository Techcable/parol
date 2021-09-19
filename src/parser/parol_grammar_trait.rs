// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use crate::parser::parol_grammar::ParolGrammar;
use id_tree::Tree;
use parol_runtime::parser::errors::*;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};

///
/// The `ParolGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait ParolGrammarTrait {
    /// Semantic action for production 0:
    ///
    /// Grammar: Prolog GrammarDefinition;
    ///
    fn grammar_0(
        &mut self,
        _prolog_0: &ParseTreeStackEntry,
        _grammar_definition_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// Prolog: StartDeclaration PrologSuffix;
    ///
    fn prolog_1(
        &mut self,
        _start_declaration_0: &ParseTreeStackEntry,
        _prolog_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// PrologSuffix: PrologRest;
    ///
    fn prolog_suffix_2(
        &mut self,
        _prolog_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// PrologSuffix: ;
    ///
    fn prolog_suffix_3(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// PrologRest: Declaration PrologRestSuffix;
    ///
    fn prolog_rest_4(
        &mut self,
        _declaration_0: &ParseTreeStackEntry,
        _prolog_rest_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// PrologRestSuffix: PrologRest;
    ///
    fn prolog_rest_suffix_5(
        &mut self,
        _prolog_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// PrologRestSuffix: ;
    ///
    fn prolog_rest_suffix_6(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// StartDeclaration: "%start" Identifier;
    ///
    fn start_declaration_7(
        &mut self,
        _percent_start_0: &ParseTreeStackEntry,
        _identifier_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// Declaration: "%title" String;
    ///
    fn declaration_8(
        &mut self,
        _percent_title_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// Declaration: "%comment" String;
    ///
    fn declaration_9(
        &mut self,
        _percent_comment_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// Declaration: "%line_comment" String;
    ///
    fn declaration_10(
        &mut self,
        _percent_line_underscore_comment_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// Declaration: "%block_comment" String String;
    ///
    fn declaration_11(
        &mut self,
        _percent_block_underscore_comment_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _string_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// GrammarDefinition: "%%" Production GrammarDefinitionSuffix;
    ///
    fn grammar_definition_12(
        &mut self,
        _percent_percent_0: &ParseTreeStackEntry,
        _production_1: &ParseTreeStackEntry,
        _grammar_definition_suffix_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// GrammarDefinitionSuffix: GrammarDefinitionRest;
    ///
    fn grammar_definition_suffix_13(
        &mut self,
        _grammar_definition_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// GrammarDefinitionSuffix: ;
    ///
    fn grammar_definition_suffix_14(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// GrammarDefinitionRest: Production GrammarDefinitionRestSuffix;
    ///
    fn grammar_definition_rest_15(
        &mut self,
        _production_0: &ParseTreeStackEntry,
        _grammar_definition_rest_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// GrammarDefinitionRestSuffix: GrammarDefinitionRest;
    ///
    fn grammar_definition_rest_suffix_16(
        &mut self,
        _grammar_definition_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// GrammarDefinitionRestSuffix: ;
    ///
    fn grammar_definition_rest_suffix_17(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// Production: Identifier ":" Alternations ";";
    ///
    fn production_18(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _colon_1: &ParseTreeStackEntry,
        _alternations_2: &ParseTreeStackEntry,
        _semicolon_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// Alternations: Alternation AlternationsSuffix;
    ///
    fn alternations_19(
        &mut self,
        _alternation_0: &ParseTreeStackEntry,
        _alternations_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// AlternationsSuffix: AlternationsRest;
    ///
    fn alternations_suffix_20(
        &mut self,
        _alternations_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 21:
    ///
    /// AlternationsSuffix: ;
    ///
    fn alternations_suffix_21(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 22:
    ///
    /// AlternationsRest: "\|" Alternation AlternationsRestSuffix;
    ///
    fn alternations_rest_22(
        &mut self,
        _or_0: &ParseTreeStackEntry,
        _alternation_1: &ParseTreeStackEntry,
        _alternations_rest_suffix_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 23:
    ///
    /// AlternationsRestSuffix: AlternationsRest;
    ///
    fn alternations_rest_suffix_23(
        &mut self,
        _alternations_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 24:
    ///
    /// AlternationsRestSuffix: ;
    ///
    fn alternations_rest_suffix_24(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 25:
    ///
    /// Alternation: AlternationRest;
    ///
    fn alternation_25(
        &mut self,
        _alternation_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 26:
    ///
    /// Alternation: ;
    ///
    fn alternation_26(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 27:
    ///
    /// AlternationRest: Factor AlternationRestSuffix;
    ///
    fn alternation_rest_27(
        &mut self,
        _factor_0: &ParseTreeStackEntry,
        _alternation_rest_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 28:
    ///
    /// AlternationRestSuffix: AlternationRest;
    ///
    fn alternation_rest_suffix_28(
        &mut self,
        _alternation_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 29:
    ///
    /// AlternationRestSuffix: ;
    ///
    fn alternation_rest_suffix_29(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 30:
    ///
    /// Factor: Group;
    ///
    fn factor_30(
        &mut self,
        _group_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 31:
    ///
    /// Factor: Repeat;
    ///
    fn factor_31(
        &mut self,
        _repeat_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 32:
    ///
    /// Factor: Optional;
    ///
    fn factor_32(
        &mut self,
        _optional_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 33:
    ///
    /// Factor: Symbol;
    ///
    fn factor_33(
        &mut self,
        _symbol_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 34:
    ///
    /// Symbol: Identifier;
    ///
    fn symbol_34(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 35:
    ///
    /// Symbol: String;
    ///
    fn symbol_35(
        &mut self,
        _string_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 36:
    ///
    /// Group: "\(" Alternations "\)";
    ///
    fn group_36(
        &mut self,
        _l_paren_0: &ParseTreeStackEntry,
        _alternations_1: &ParseTreeStackEntry,
        _r_paren_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 37:
    ///
    /// Optional: "\[" Alternations "\]";
    ///
    fn optional_37(
        &mut self,
        _l_bracket_0: &ParseTreeStackEntry,
        _alternations_1: &ParseTreeStackEntry,
        _r_bracket_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 38:
    ///
    /// Repeat: "\{" Alternations "\}";
    ///
    fn repeat_38(
        &mut self,
        _l_brace_0: &ParseTreeStackEntry,
        _alternations_1: &ParseTreeStackEntry,
        _r_brace_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 39:
    ///
    /// Identifier: "[a-zA-Z_]\w*";
    ///
    fn identifier_39(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 40:
    ///
    /// String: "\u{0022}([^\\]|\\.)*?\u{0022}";
    ///
    fn string_40(
        &mut self,
        _string_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait for ParolGrammar {
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry],
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        match prod_num {
            0 => self.grammar_0(&children[0], &children[1], parse_tree),

            1 => self.prolog_1(&children[0], &children[1], parse_tree),

            2 => self.prolog_suffix_2(&children[0], parse_tree),

            3 => self.prolog_suffix_3(parse_tree),

            4 => self.prolog_rest_4(&children[0], &children[1], parse_tree),

            5 => self.prolog_rest_suffix_5(&children[0], parse_tree),

            6 => self.prolog_rest_suffix_6(parse_tree),

            7 => self.start_declaration_7(&children[0], &children[1], parse_tree),

            8 => self.declaration_8(&children[0], &children[1], parse_tree),

            9 => self.declaration_9(&children[0], &children[1], parse_tree),

            10 => self.declaration_10(&children[0], &children[1], parse_tree),

            11 => self.declaration_11(&children[0], &children[1], &children[2], parse_tree),

            12 => self.grammar_definition_12(&children[0], &children[1], &children[2], parse_tree),

            13 => self.grammar_definition_suffix_13(&children[0], parse_tree),

            14 => self.grammar_definition_suffix_14(parse_tree),

            15 => self.grammar_definition_rest_15(&children[0], &children[1], parse_tree),

            16 => self.grammar_definition_rest_suffix_16(&children[0], parse_tree),

            17 => self.grammar_definition_rest_suffix_17(parse_tree),

            18 => self.production_18(
                &children[0],
                &children[1],
                &children[2],
                &children[3],
                parse_tree,
            ),

            19 => self.alternations_19(&children[0], &children[1], parse_tree),

            20 => self.alternations_suffix_20(&children[0], parse_tree),

            21 => self.alternations_suffix_21(parse_tree),

            22 => self.alternations_rest_22(&children[0], &children[1], &children[2], parse_tree),

            23 => self.alternations_rest_suffix_23(&children[0], parse_tree),

            24 => self.alternations_rest_suffix_24(parse_tree),

            25 => self.alternation_25(&children[0], parse_tree),

            26 => self.alternation_26(parse_tree),

            27 => self.alternation_rest_27(&children[0], &children[1], parse_tree),

            28 => self.alternation_rest_suffix_28(&children[0], parse_tree),

            29 => self.alternation_rest_suffix_29(parse_tree),

            30 => self.factor_30(&children[0], parse_tree),

            31 => self.factor_31(&children[0], parse_tree),

            32 => self.factor_32(&children[0], parse_tree),

            33 => self.factor_33(&children[0], parse_tree),

            34 => self.symbol_34(&children[0], parse_tree),

            35 => self.symbol_35(&children[0], parse_tree),

            36 => self.group_36(&children[0], &children[1], &children[2], parse_tree),

            37 => self.optional_37(&children[0], &children[1], &children[2], parse_tree),

            38 => self.repeat_38(&children[0], &children[1], &children[2], parse_tree),

            39 => self.identifier_39(&children[0], parse_tree),

            40 => self.string_40(&children[0], parse_tree),

            _ => panic!("Unhandled production number: {}", prod_num),
        }
    }
}