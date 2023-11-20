mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use args::Args;
use clap::Parser;
use commands::{decode, encode, print, remove};

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.commands {
        args::Commands::Encode {
            path,
            chunk_type,
            message,
        } => {
            encode(path, chunk_type, message)?;
        }
        args::Commands::Decode { path, chunk_type } => {
            decode(path, chunk_type)?;
        }
        args::Commands::Remove { path, chunk_type } => {
            remove(path, chunk_type)?;
        }
        args::Commands::Print { path } => {
            print(path)?;
        }
    }

    Ok(())
}
