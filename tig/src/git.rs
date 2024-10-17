use std::process::{Command, Stdio};

pub struct Git {
    base: Command,
    arguments: Vec<(String, String)>
}

impl Git {
    pub fn new() -> Git {
        Git {
            base: Command::new("git"),
            arguments: Vec::new(),
        }
    }

    pub fn exec(&mut self) -> std::io::Result<()> {
        let arguments = self.flattern_arguments();

        self.base.args(&arguments).stdout(Stdio::inherit()).spawn()?;

        Ok(())
    }

    fn flattern_arguments(&self) -> Vec<String> {
        let mut arguments: Vec<String> = Vec::new();

        for (key, value) in self.arguments.iter() {
            if !key.is_empty() {
                arguments.push(key.clone());
            }
            if !value.is_empty() {
                arguments.push(value.clone());
            }
        }

        arguments
    }
}
