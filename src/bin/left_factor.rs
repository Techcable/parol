use miette::Result;
use parol::{left_factor, obtain_grammar_config};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_name = args[1].clone();

        let mut grammar_config = obtain_grammar_config(&file_name, false)?;
        let cfg = left_factor(&grammar_config.cfg);

        // Exchange original grammar with transformed one
        grammar_config.update_cfg(cfg);
    } else {
        println!("Missing arguments <par-file>!");
        println!(
            "Example:\n\
            cargo run --bin {} ./src/parser/parol-grammar-exp.par",
            module_path!()
        );
    }
    Ok(())
}
