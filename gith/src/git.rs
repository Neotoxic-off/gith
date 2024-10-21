use std::{process::{Command, Stdio}};
use std::collections::HashMap;
use crate::arguments;

pub struct Git {
    base: Command,
    arguments: Vec<String>,
    raw_arguments: arguments::Arguments,
    emoji: HashMap<String, String>
}

impl Git {
    pub fn new(arguments: arguments::Arguments) -> Git {
        Git {
            base: Command::new("git"),
            arguments: Vec::new(),
            raw_arguments: arguments,
            emoji: HashMap::from([
                ("clean".to_string(), "🧽".to_string()),
                ("deploy".to_string(), "🚀".to_string()),
                ("documentation".to_string(), "📃".to_string()),
                ("feature".to_string(), "🌱".to_string()),
                ("fix".to_string(), "🔧".to_string()),
                ("init".to_string(), "🎀".to_string()),
                ("library".to_string(), "📚".to_string()),
                ("patch".to_string(), "🐜".to_string()),
                ("performance".to_string(), "🌡".to_string()),
                ("revert".to_string(), "♻️".to_string()),
                ("rework".to_string(), "🔸".to_string()),
                ("style".to_string(), "🧁".to_string()),
                ("test".to_string(), "🧪".to_string()),
            ])
        }
    }

    pub fn exec(&mut self) -> std::io::Result<()> {
        self.build_arguments();

        self.base.args(&self.arguments).stdout(Stdio::inherit()).spawn()?;

        Ok(())
    }

    fn build_arguments(&mut self) {
        match &self.raw_arguments.command {
            arguments::GitCommand::Commit { message, kind } => {
                self.arguments = self.get_commit_arguments(message, kind);
            }
        }
    }

    fn get_commit_arguments(&self, message: &String, kind: &arguments::GitCommitKindFlags) -> Vec<String> {
        let mut args = vec!["commit".to_string()];

        let commit_kind = self.get_commit_kind(kind);
        if let Some((kind_str, emoji)) = commit_kind {
            args.push("-m".to_string());
            args.push(format!("{} {}: {}", emoji, kind_str, message));
        }

        args
    }

    fn get_commit_kind(&self, kind: &arguments::GitCommitKindFlags) -> Option<(String, String)> {
        let kind_mapping: HashMap<&str, bool> = HashMap::from([
            ("clean", kind.clean),
            ("deploy", kind.deploy),
            ("documentation", kind.documentation),
            ("feature", kind.feature),
            ("fix", kind.fix),
            ("init", kind.init),
            ("library", kind.library),
            ("patch", kind.patch),
            ("performance", kind.performance),
            ("revert", kind.revert),
            ("rework", kind.rework),
            ("style", kind.style),
            ("test", kind.test),
        ]);

        if let Some((kind_str, _)) = kind_mapping.iter().find(|(_, &enabled)| enabled) {
            if let Some(emoji) = self.emoji.get(*kind_str) {
                return Some((kind_str.to_string(), emoji.clone()));
            }
        }
    
        None
    }
    
}

