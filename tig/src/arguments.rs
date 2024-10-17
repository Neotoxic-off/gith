use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum GitCommand {
    Commit { kind: GitCommitKind, message: String},
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum GitCommitKind {
    Feature,
    Fix,
    Release,
    Deploy,
    Documentation,
}

#[derive(Parser)]
#[command(name = "tig")]
#[command(about = "🧹 Git harmonizer")]
pub struct Arguments {
    #[command(subcommand)]
    pub command: GitCommand,
}
