use std::fs;
use std::path::{PathBuf};
use walkdir::WalkDir;
use serde_yaml::Value;
use crate::error::TemplaturrError;
use crate::core::TemplateEngine;

pub struct FileProcessor;

impl FileProcessor {
    pub fn process_file(
        input_path: &str,
        data: &Value,
        output_path: Option<&str>,
        strict: bool,
    ) -> Result<(), TemplaturrError> {
        let content = fs::read_to_string(input_path)?;
        let (processed, missing) = TemplateEngine::replace_variables(&content, data)?;

        if !missing.is_empty() {
            eprintln!("⚠️  Missing variables in {}: {:?}", input_path, missing);
            if strict {
                return Err(TemplaturrError::MissingVariables(missing));
            }
        }

        match output_path {
            Some(path) => {
                fs::write(path, processed)?;
                println!("Generated: {}", path);
            }
            None => println!("{}", processed),
        }
        Ok(())
    }

    pub fn process_directory(
        input_dir: &str,
        data: &Value,
        output_dir: Option<&str>,
        strict: bool,
    ) -> Result<(), TemplaturrError> {
        let mut all_missing = Vec::new();

        for entry in WalkDir::new(input_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let input_path = entry.path();
                let relative_path = input_path.strip_prefix(input_dir)?;
                
                let output_path = output_dir.map(|od| {
                    PathBuf::from(od).join(relative_path)
                });

                let (processed, missing) = TemplateEngine::replace_variables(
                    &fs::read_to_string(input_path)?,
                    data
                )?;
                
                all_missing.extend(missing);

                if let Some(ref op) = output_path {
                    fs::create_dir_all(op.parent().unwrap())?;
                    fs::write(op, processed)?;
                    println!("Generated: {}", op.display());
                }
            }
        }

        if !all_missing.is_empty() {
            eprintln!("⚠️  Global missing variables: {:?}", all_missing);
            if strict {
                return Err(TemplaturrError::MissingVariables(all_missing));
            }
        }
        
        Ok(())
    }
}