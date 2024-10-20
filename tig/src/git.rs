use std::{fmt::format, process::{Command, Stdio}};
use crate::arguments;

pub struct Git {
    base: Command,
    arguments: Vec<String>,
    raw_arguments: arguments::Arguments,
}

impl Git {
    pub fn new(raw_arguments: arguments::Arguments) -> Git {
        Git {
            base: Command::new("git"),
            arguments: Vec::new(),
            raw_arguments,
        }
    }

    pub fn exec(&mut self) -> std::io::Result<()> {
        self.build_arguments();

        println!("{}", &self.arguments.concat());
        self.base.args(&self.arguments).stdout(Stdio::inherit()).spawn()?;

        Ok(())
    }

    fn build_arguments(&mut self) {
        match &self.raw_arguments.command {
            arguments::GitCommand::Commit { message, kind } => {
                self.arguments = self.get_commit_arguments(message, kind);
            }

            _ => {
                println!("Unsupported command");
            }
        }
    }

    fn get_commit_arguments(&self, message: &String, kind: &arguments::GitCommitKind) -> Vec<String> {
        let mut args = vec!["commit".to_string()];

        args.push("-m".to_string());
        args.push(format!("{:?}: {}", kind, message));

        args
    }

    fn get_add_arguments(&self, files: &Vec<String>) -> Vec<String> {
        let mut args = vec!["add".to_string()];  // Start with "git add"
        args.extend(files.iter().cloned());       // Add the files to the arguments
        args
    }
}
