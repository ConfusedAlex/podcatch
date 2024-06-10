use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct PodcatchArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search for a podcast
    Search {
        query: Option<String>,
    },
    /// Add a podcast
    Add {
        url: Option<String>,
    },
    /// Opens an interface to remove a podcast
    Remove,
    List,
}
