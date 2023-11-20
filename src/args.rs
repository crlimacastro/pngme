use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        path: String,
        chunk_type: String,
        message: String,
    },
    Decode {
        path: String,
        chunk_type: String,
    },
    Remove {
        path: String,
        chunk_type: String,
    },
    Print {
        path: String,
    },
}
