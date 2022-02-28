use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(arg_required_else_help = true)]
struct Cli {
    file: Option<String>,
}

pub fn run() {
    let cli = Cli::parse();
    println!("{:?}", cli.file);
}
