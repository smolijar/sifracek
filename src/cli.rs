use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
pub enum Commands {
    /// Encode input with selected algo (or all available)
    Encode {
        #[arg(short, long, value_enum)]
        algo: Option<Algo>,
        input: String,
    },
    /// Decode input with selected algo (or all available)
    Decode {
        #[arg(short, long, value_enum)]
        algo: Option<Algo>,
        input: String,
    },
}

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Algo {
    Morse,
    Rev,
    Snail,
}

