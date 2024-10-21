use core::str;
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

    fn get_current_branch(&self) -> Result<String, String> {
        let output = Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "HEAD"])
            .stdout(Stdio::piped())
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let branch_name = str::from_utf8(&output.stdout)
                        .map_err(|e| e.to_string())?
                        .trim()
                        .to_string();
                    Ok(branch_name)
                } else {
                    // Handle non-zero exit status
                    Err(format!(
                        "Error: {}", 
                        str::from_utf8(&output.stderr).unwrap_or("unknown error")
                    ))
                }
            }
            Err(e) => Err(format!("Failed to execute git command: {}", e)),
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
            },
            arguments::GitCommand::Add { content } => {
                self.arguments = self.get_add_arguments(content);
            },
            arguments::GitCommand::Push { content } => {
                self.arguments = self.get_push_arguments(content);
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

    fn get_add_arguments(&self, content: &Vec<String>) -> Vec<String> {
        let mut args: Vec<String> = vec!["add".to_string()];

        if content.is_empty() == false {
            args.extend(content.iter().cloned());
        } else {
            args.push(".".to_string());
        }

        args
    }

    fn get_push_arguments(&self, content: &Vec<String>) -> Vec<String> {
        let mut args: Vec<String> = vec!["push".to_string()];
        let mut current_branch: String;

        if content.is_empty() == true {
            current_branch = self.get_current_branch().unwrap();
        } else {
            args.extend(content.iter().cloned());
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

