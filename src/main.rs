use clap::Parser;

mod cli;
mod encoders;

use cli::{Algo, Cli, Commands};
use encoders::Encoder;
use Commands::*;
fn main() {
    let args = Cli::parse();
    let res = match args.command {
        Encode {
            algo: Some(Algo::Morse),
            input,
        } => encoders::Morse::encode(&input),
        Decode {
            algo: Some(Algo::Morse),
            input,
        } => encoders::Morse::decode(&input),
        Encode {
            algo: Some(Algo::Rev),
            input,
        } => encoders::Reverse::encode(&input),
        Decode {
            algo: Some(Algo::Rev),
            input,
        } => encoders::Reverse::decode(&input),
        Encode {
            algo: Some(Algo::Snail),
            input,
        } => encoders::Snail::encode(&input),
        Decode {
            algo: Some(Algo::Snail),
            input,
        } => encoders::Snail::decode(&input),
        Encode { algo: None, input } => {
            let encoders: Vec<Box<dyn Encoder>> = vec![
                Box::new(encoders::Morse {}),
                Box::new(encoders::Reverse {}),
                Box::new(encoders::Snail {}),
            ];
            let mut res = String::new();
            for encoder in encoders {
                res.push_str(&format!(
                    "[{}]: {}\n",
                    &encoder.name(),
                    &encoder.description()
                ));
                res.push_str(&encoder.encode(&input));
                res.push_str("\n\n");
            }
            res.to_string()
        }
        Decode { algo: None, input } => encoders::Morse::decode(&input),
    };
    println!("{}", res)
}
