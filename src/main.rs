use anyhow::Result;
mod cli;
mod vm;
mod compiler;

fn main() -> Result<()> {
    cli::run()
}
