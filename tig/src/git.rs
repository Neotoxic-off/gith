use std::{fmt::format, process::{Command, Stdio}};
use std::collections::HashMap;
use crate::arguments;

pub struct Git {
    base: Command,
    arguments: Vec<String>,
    raw_arguments: arguments::Arguments,
    emoji: HashMap<arguments::GitCommitKind, String>
}

impl Git {
    pub fn new(arguments: arguments::Arguments) -> Git {
        Git {
            base: Command::new("git"),
            arguments: Vec::new(),
            raw_arguments: arguments,
            emoji: HashMap::from([
                (arguments::GitCommitKind::clean, "🧽".to_string()),
                (arguments::GitCommitKind::deploy, "🚀".to_string()),
                (arguments::GitCommitKind::documentation, "📃".to_string()),
                (arguments::GitCommitKind::feature, "🌱".to_string()),
                (arguments::GitCommitKind::fix, "🔧".to_string()),
                (arguments::GitCommitKind::init, "🎀".to_string()),
                (arguments::GitCommitKind::library, "📚".to_string()),
                (arguments::GitCommitKind::patch,  "🐜".to_string()),
                (arguments::GitCommitKind::performance, "🌡".to_string()),
                (arguments::GitCommitKind::revert,  "♻️".to_string()),
                (arguments::GitCommitKind::rework, "🔸".to_string()),
                (arguments::GitCommitKind::style, "🧁".to_string()),
                (arguments::GitCommitKind::test, "🧪".to_string())
            ])
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
        args.push(format!("{:?} {:?}: {}", self.emoji.get(kind), kind, message));

        args
    }

    fn get_add_arguments(&self, files: &Vec<String>) -> Vec<String> {
        let mut args = vec!["add".to_string()];  // Start with "git add"
        args.extend(files.iter().cloned());       // Add the files to the arguments
        args
    }
}
