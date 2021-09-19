use crate::analysis::compiled_la_dfa::CompiledDFA;
use crate::analysis::LookaheadDFA;
use crate::conversions::dot::render_dfa_dot_string;
use crate::errors::*;
use crate::generators::GrammarConfig;
use crate::{Pr, Symbol, Terminal};
use log::trace;
use std::collections::{BTreeMap, BTreeSet};

use crate::StrVec;
use std::fmt::Debug;

#[derive(BartDisplay, Debug, Default)]
#[template = "templates/parser_dfa_template.rs"]
struct Dfa {
    states: StrVec,
    state_count: usize,
    transitions: StrVec,
    transition_count: usize,
    k: usize,
    nt_index: usize,
    nt_name: String,
}

impl Dfa {
    fn from_la_dfa(la_dfa: &LookaheadDFA, nt_index: usize, nt_name: String) -> Self {
        let compiled_dfa = CompiledDFA::from_lookahead_dfa(la_dfa);
        let states =
            compiled_dfa
                .states
                .iter()
                .fold(StrVec::new(4).first_line_no_indent(), |mut acc, s| {
                    acc.push(format!("{:?},", s));
                    acc
                });
        let state_count = compiled_dfa.states.len();
        let transitions = compiled_dfa.transitions.iter().fold(
            StrVec::new(4).first_line_no_indent(),
            |mut acc, t| {
                acc.push(format!("DFATransition{:?},", t));
                acc
            },
        );
        let transition_count = compiled_dfa.transitions.len();
        let k = compiled_dfa.k;

        Self {
            states,
            state_count,
            transitions,
            transition_count,
            k,
            nt_index,
            nt_name,
        }
    }
}

#[derive(BartDisplay, Debug, Default)]
#[template = "templates/parser_dfas_template.rs"]
struct Dfas {
    dfa_count: usize,
    lookahead_dfa_s: String,
}

#[derive(BartDisplay, Debug, Default)]
#[template = "templates/parser_production_template.rs"]
struct Production {
    lhs: usize,
    production_len: usize,
    production: StrVec,
    prod_num: usize,
    prod_string: String,
}

impl Production {
    fn from_cfg_production(
        pr: &Pr,
        prod_num: usize,
        non_terminals: &[&str],
        terminals: &[&str],
    ) -> Self {
        let get_non_terminal_index =
            |nt: &str| non_terminals.iter().position(|n| *n == nt).unwrap();
        let get_terminal_index = |tr: &str| terminals.iter().position(|t| *t == tr).unwrap();
        let lhs = get_non_terminal_index(pr.get_n_str());
        let production_len = pr.len();
        let production =
            pr.get_r()
                .iter()
                .rev()
                .fold(StrVec::new(4).first_line_no_indent(), |mut acc, s| {
                    match s {
                        Symbol::N(n) => {
                            acc.push(format!("ParseType::N({}),", get_non_terminal_index(n)))
                        }
                        Symbol::T(Terminal::Trm(t)) => {
                            acc.push(format!("ParseType::T({}),", get_terminal_index(t)))
                        }
                        _ => panic!("Unexpected symbol type in production!"),
                    }
                    acc
                });
        let prod_string = format!("{}", pr);
        Self {
            lhs,
            production_len,
            production,
            prod_num,
            prod_string,
        }
    }
}

#[derive(BartDisplay, Debug, Default)]
#[template = "templates/parser_productions_template.rs"]
struct Productions {
    production_count: usize,
    productions: String,
}

#[derive(BartDisplay, Debug, Default)]
#[template = "templates/parser_template.rs"]
struct ParserData<'a> {
    start_symbol_index: usize,
    lexer_source: &'a str,
    non_terminals: StrVec,
    non_terminal_count: usize,
    dfa_source: String,
    productions: String,
    max_k: usize,
    ast_type_name: String,
    ast_trait_module_name: String,
}

pub fn generate_parser_source(
    grammar_config: &GrammarConfig,
    lexer_source: &str,
    la_dfa: &BTreeMap<String, LookaheadDFA>,
    ast_type_name: &str,
    ast_trait_module_name: &str,
) -> Result<String> {
    let original_augmented_terminals = grammar_config.generate_augmented_terminals();
    let augmented_terminals = original_augmented_terminals
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();
    let original_non_terminals = grammar_config.cfg.get_non_terminal_set();
    let non_terminal_count = original_non_terminals.len();
    let width = (non_terminal_count as f32).log10() as usize + 1;

    let non_terminals = original_non_terminals.iter().collect::<Vec<_>>();
    let start_symbol_index: usize = non_terminals
        .iter()
        .position(|n| *n == grammar_config.cfg.get_start_symbol())
        .chain_err(|| {
            format!(
                "Start symbol '{}' is not part of the given grammar!",
                grammar_config.cfg.get_start_symbol()
            )
        })?;

    let non_terminals = non_terminals
        .iter()
        .enumerate()
        .fold(StrVec::new(4), |mut acc, (i, n)| {
            acc.push(format!(r#"/* {:w$} */ "{}","#, i, n, w = width));
            acc
        });

    let dfa_source = generate_dfa_source(la_dfa);

    let productions = generate_productions(
        grammar_config,
        &original_non_terminals,
        &augmented_terminals,
    );

    let max_k = grammar_config.lookahead_size;

    let parser_data = ParserData {
        start_symbol_index,
        lexer_source,
        non_terminals,
        non_terminal_count,
        dfa_source,
        productions,
        max_k,
        ast_type_name: ast_type_name.to_string(),
        ast_trait_module_name: ast_trait_module_name.to_string(),
    };

    Ok(format!("{}", parser_data))
}

fn generate_dfa_source(la_dfa: &BTreeMap<String, LookaheadDFA>) -> String {
    let lookahead_dfa_s = la_dfa
        .iter()
        .enumerate()
        .fold(StrVec::new(0), |mut acc, (i, (n, d))| {
            trace!("{}", render_dfa_dot_string(d, n));
            let dfa = Dfa::from_la_dfa(d, i, n.clone());
            acc.push(format!("{}", dfa));
            acc
        });
    let dfa_count = la_dfa.len();

    let dfas = Dfas {
        dfa_count,
        lookahead_dfa_s: format!("{}", lookahead_dfa_s),
    };

    format!("{}", dfas)
}

fn generate_productions(
    grammar_config: &GrammarConfig,
    non_terminals: &BTreeSet<String>,
    terminals: &[&str],
) -> String {
    let non_terminals = non_terminals
        .iter()
        .map(|n| n.as_str())
        .collect::<Vec<&str>>();
    let production_count = grammar_config.cfg.pr.len();
    let productions =
        grammar_config
            .cfg
            .pr
            .iter()
            .enumerate()
            .fold(String::new(), |mut acc, (i, p)| {
                let production = Production::from_cfg_production(p, i, &non_terminals, terminals);
                acc.push_str(format!("{}", production).as_str());
                acc
            });

    let productions = Productions {
        production_count,
        productions,
    };

    format!("{}", productions)
}