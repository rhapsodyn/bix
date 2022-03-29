use crate::{vm, compiler::compile};
use anyhow::{Result, bail};
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    file: Option<String>,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();
    if let Some(path) = args.file {
        let content = fs::read_to_string(path)?;
        let code = compile(&content);
        vm::run(code)
    } else {
        bail!("Read souce code err")
    }
}
