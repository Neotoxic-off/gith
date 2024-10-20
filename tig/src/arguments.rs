use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum GitCommand {
    Commit { kind: GitCommitKind, message: String }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum GitCommitKind {
    clean,
    deploy,
    documentation,
    feature,
    fix,
    init,
    library,
    patch,
    performance,
    revert,
    rework,
    style,
    test
}

#[derive(Parser)]
#[command(name = "tig")]
#[command(about = "🧹 Git harmonizer")]
pub struct Arguments {
    #[command(subcommand)]
    pub command: GitCommand,
}
