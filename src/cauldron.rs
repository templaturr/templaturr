use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use regex::Regex;
use walkdir::WalkDir;
use crate::error::TemplaturrError;

pub struct CauldronGenerator;

impl CauldronGenerator {
    pub fn create_cauldron(template_path: &str) -> Result<(), TemplaturrError> {
        let mut vars = HashSet::new();
        let re = Regex::new(r"\{\[\s*(\w+)\s*\]\}")?;

        for entry in WalkDir::new(template_path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let content = std::fs::read_to_string(entry.path())?;
                for cap in re.captures_iter(&content) {
                    vars.insert(cap[1].to_string());
                }
            }
        }

        let mut yaml = String::new();
        for var in &vars {
            yaml.push_str(&format!("{}: REPLACE_ME\n", var));
        }

        let mut file = File::create("cauldron.yaml")?;
        file.write_all(yaml.as_bytes())?;
        
        println!("Generated cauldron.yaml with {} variables", vars.len());
        Ok(())
    }
}