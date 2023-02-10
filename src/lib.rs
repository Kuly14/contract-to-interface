use anyhow::Result;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Contract {
    pub name: String,
    pub functions: Vec<Function>,
}

static FUNCTION_SIG_KEYWORDS: &[&str] = &["external", "view", "pure"];

impl Contract {
    pub fn build<T: AsRef<Path>>(path: T) -> Result<Contract> {
        let file = fs::read_to_string(path)?;
        let mut contract = Contract::default();
        let mut lines = file.lines();
        while let Some(line) = lines.next() {
            // Gets contract name
            if line.trim().starts_with("contract") {
                let mut contract_sig = line.to_string();
                if !line.contains('{') {
                    loop {
                        let next = lines.next().unwrap();
                        contract_sig.push_str(next.trim());
                        if next.contains('{') {
                            break;
                        }
                    }
                    contract.name = contract_sig
                        .split_whitespace()
                        .skip(1)
                        .take(1)
                        .collect::<String>();
                } else {
                    contract.name = contract_sig
                        .split_whitespace()
                        .skip(1)
                        .take(1)
                        .collect::<String>();
                }
            }

            // Push all external and public functions to functions vector.
            if line.trim().starts_with("function") {
                let mut function = Function {
                    signature: line.trim().to_string(),
                };
                if !line.contains('{') {
                    loop {
                        let next = lines.next().unwrap();
                        function.signature.push(' ');
                        function.signature.push_str(next.trim());
                        if next.contains('{') {
                            break;
                        }
                    }
                }
                if function.signature.contains("internal") || function.signature.contains("private")
                {
                    continue;
                }

                function.adjust();
                contract.functions.push(function);
            }
        }

        Ok(contract)
    }

    pub fn format(&self) -> String {
        let mut fin = format!("interface {}", self.name);
        fin.push(' ');
        fin.push('{');
        for i in &self.functions {
            fin.push_str(&format!("\n\t{}", &i.signature))
        }

        fin.push('\n');
        fin.push('}');
        fin
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Function {
    pub signature: String,
}

impl Function {
    pub fn adjust(&mut self) {
        self.signature = self.signature.replace("public", "external");

        let keywords = self
            .signature
            .chars()
            .skip_while(|x| *x != ')')
            .collect::<String>()
            .split_whitespace()
            .take_while(|x| !x.contains("returns"))
            .filter(|x| !FUNCTION_SIG_KEYWORDS.contains(&x) && x.len() > 1)
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for i in keywords {
            self.signature = self.signature.replace(&i, "");
        }
        self.signature = self.signature.replace("  ", " ");
        self.signature = self.signature.replace(" {", ";");
    }
}
