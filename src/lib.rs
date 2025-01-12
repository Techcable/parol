//#![forbid(missing_docs)]
//!
//! Main module of this crate
//!

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate bart_derive;

extern crate serde;
extern crate serde_json;

///
/// Basic grammar data structures and algorithms
///
pub mod grammar;

pub use grammar::{Cfg, Pos, Pr, Rhs, Symbol, SymbolString, SymbolStrings, Terminal};

///
/// Module with functionalities for grammar analysis
///
pub mod analysis;

pub use analysis::{
    calculate_lookahead_dfas, detect_left_recursions, CompiledTerminal, KTuple, KTuples,
    NtEdgeType, NtGrammarGraph, NtNodeType,
};

///
/// Module with functionalities for grammar conversion
///
pub mod conversions;

pub use conversions::{render_dfa_dot_string, render_nt_dot_string, render_par_string};

///
/// Module with functionalities for lexer and parser generation
///
pub mod generators;
pub use generators::{
    check_and_transform_grammar, generate_lexer_source, generate_parser_source,
    generate_user_trait_source, try_format, GrammarConfig, ScannerConfig,
};

pub mod parser;
pub use parser::{parse, ParolGrammar};

///
/// Module with functionalities for grammar transformation
///
pub mod transformation;
pub use transformation::left_factor;

mod utils;
pub use utils::{generate_tree_layout, obtain_grammar_config};

pub(crate) use utils::str_vec::StrVec;
pub(crate) use utils::{generate_name, group_by};

///
/// Internal lookahead limit
///
pub const MAX_K: usize = 10;
