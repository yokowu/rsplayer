use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author = "Yoko", 
version = "v1.0", about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// run test
    Run(Run),
}

#[derive(Args, Debug)]
struct Run {
    #[arg(short, default_value_t = String::from("df"))]
    config: String,

    #[arg(short, value_enum)]
    mode: Option<Mode>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Test,
    Prod,
}

fn main() {
    let cli = Cli::parse();

    match cli.action {
        Action::Run(r) => run(r),
    }
}

fn run(run: Run) {
    println!("{:?}", run.mode)
}
