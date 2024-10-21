use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum GitCommand {
    Commit {
        message: String,
        #[clap(flatten)]
        kind: GitCommitKindFlags
    },
    Add {
        content: Vec<String>
    },
    Push {
        content: Vec<String>
    }
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct GitCommitKindFlags {
    #[arg(long)]
    pub clean: bool,
    #[arg(long)]
    pub deploy: bool,
    #[arg(long)]
    pub documentation: bool,
    #[arg(long)]
    pub feature: bool,
    #[arg(long)]
    pub fix: bool,
    #[arg(long)]
    pub init: bool,
    #[arg(long)]
    pub library: bool,
    #[arg(long)]
    pub patch: bool,
    #[arg(long)]
    pub performance: bool,
    #[arg(long)]
    pub revert: bool,
    #[arg(long)]
    pub rework: bool,
    #[arg(long)]
    pub style: bool,
    #[arg(long)]
    pub test: bool
}

#[derive(Parser)]
#[command(name = "gith")]
#[command(about = "🧹 Git harmonizer")]
pub struct Arguments {
    #[command(subcommand)]
    pub command: GitCommand,
}