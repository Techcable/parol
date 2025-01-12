<!-- markdownlint-disable first-line-h1 -->
[![Rust](https://github.com/jsinger67/parol/actions/workflows/rust.yml/badge.svg)](https://github.com/jsinger67/parol/actions/workflows/rust.yml)
[![Docs.rs](https://docs.rs/parol/badge.svg)](https://docs.rs/parol)
[![Crates.io](https://img.shields.io/crates/v/parol.svg)](https://crates.io/crates/parol)
<!-- markdownlint-enable first-line-h1 -->

# About `parol`

<!-- markdownlint-disable Inline HTML -->
<img src="./logo/Parol.svg" alt="Logo" height=300 with=300>
<!-- markdownlint-enable Inline HTML -->

`parol` is a LL(k) parser generator **for Rust written in Rust** with the following features

Generated parsers

* are true LL(k) parsers implemented by push down automata (PDAs).
* are predictive, i.e. they implement a **non-backtracking** parsing technique. This often results in much faster parsers.
* are clean and easy to read. For instance, terminal names are automatically deduced from literal string tokens found in the grammar description.
* can use **different lookahead sizes from 0 to k** for each non-terminal
* are generated from **a single grammar description** file.

Other properties of `parol` are:

* Selection of production is done by a deterministic finite **lookahead automaton** for each non-terminal.
* **Semantic actions** with empty default implementations are generated as a trait. You can implement this trait for your grammar processing item and overwrite needed actions. This provides a loose coupling between your language definition and the language processing.
* As a result semantic actions are strictly separated from the grammar definition in contrast to Bison. No parser generation step is needed when you merely change the implementation of a semantic action.
* The grammar description is provided in a **Yacc/Bison-like style** with additional features known from EBNF such as grouping, optional elements and repetitions.
* You can define multiple scanner states (aka start conditions) and define switches between them directly in the productions of your grammar.
* You can opt out the default handling of whitespace and newlines for each scanner state separately.
* The grammar description supports definition of language comments via **%line_comment** and **%block_comment** declarations for each scanner state.
* The crate provides several tools for **grammar analysis**, **transformation** and **parse tree visualization** to support your grammar implementation.
* The parser generator **detects direct and indirect left recursions** in your grammar description.
* `parol`'s parser is generated by `parol` itself.

## Why should you use LL(k) parsers in your language implementation?

LL parsing technique is a top-down parsing strategy that always starts from the start symbol of your grammar. This symbol becomes the root node of the parse tree. Then it tries to derive the left-most symbol first. All such symbols are then processed in a pre-order traversal. During this process the parse tree is created from the root downwards.

Both, processing the input and producing the parse tree in 'natural' direction ensures that at every point during parsing you can see where you came from and what you want to derive next. `parol`'s parse stack contains 'End of Production' markers which reflect the 'call hierarchy' of productions.

This tremendously helps to put your language processing into operation. In contrast, anyone who has ever debugged a LR parser will remember the effect of 'coming out of nowhere'.

Although LL grammars are known to be a subset of LR grammars many use cases exist where LL grammars are sufficient. By supporting more than one lookahead token the abilities of traditional LR(1) grammars and LL(k) grammars become more and more indistinct.

## Why should you use `parol`?

`parol` is simple. You can actually understand all parts of it without broader knowledge in parsing theory.

`parol` is fast. The use of deterministic automata ensures a minimal overhead during parsing, no backtracking needed.

`parol` is a true LL(k) parser. You won't find much working LL(k) parsers out there.

`parol` generates beautiful code that is easy to read which fosters debugging.

`parol` is young. Although this might be a problem some times, especially regarding the stability of the API, the best is yet to come.

`parol` is actively developed. Thus new features are likely to be added as the need arises.

## Documentation

This project contains some introductory grammar examples from entry level up to a more complex C-like expression language and an acceptor for Oberon-0 grammar.
Some of the examples describe the principles of language processing by using semantic actions in the way `parol` advocates it.

A [Tutorial](docs/Tutorial.md) explains step by step how to use `parol` by implementing a [JSON parser](https://github.com/jsinger67/json_parser.git).

`parol`'s input language processing is an additional and very practical example.

## State of the project

`parol` has proved its ability in many examples and tests during its development. Early adopters can quite safely use it.

But `parol` is not ready for production yet. Features are still in development and the crate's interface can change at any time. Many work is still to do and help is appreciated.

## Runtime library

Parsers generated by `parol` have to add a dependency to the [parol_runtime](https://crates.io/crates/parol_runtime) crate. It provides the scanner and parser implementations needed. The parol_runtime crate is very lightweight.

## Further readings

* [Introduction to `parol`](docs/Introduction.md)
* [How `parol` works](docs/Approach.md)
* [Tutorial](docs/Tutorial.md)
