use clap::Parser;

mod git;
mod arguments;

fn main() {
    let args: arguments::Arguments = arguments::Arguments::parse();
    let mut git: git::Git = git::Git::new(args);

    git.exec();
}
