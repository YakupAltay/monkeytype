use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "monkeytype")]
#[command(about = "A terminal typing game with DA submission", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(
            long,
            value_enum,
            value_delimiter = ',',
            help = "Choose DA layers (e.g., --da avail,celestia)",
            required = true
        )]
        da: Vec<DALayer>,
    },
    Address {
        #[arg(
            long,
            value_enum,
            value_delimiter = ',',
            help = "Choose DA layers (e.g., --da avail,celestia)",
            required = true
        )]
        da: Vec<DALayer>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum DALayer {
    Celestia,
    Avail,
}
