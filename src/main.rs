mod cli;
mod process;
mod core;
mod cauldron;
mod error;

use clap::Parser;
use serde_yaml::Value;
use std::path::Path;
use crate::cli::CliArgs;
use crate::error::TemplaturrError;
use crate::process::FileProcessor;
use crate::cauldron::CauldronGenerator;

fn main() -> Result<(), TemplaturrError> {
    let args = CliArgs::parse();
    
    if args.create_cauldron {
        CauldronGenerator::create_cauldron(&args.template)?;
        return Ok(());
    }

    let data_path = args.data.as_deref().unwrap_or("cauldron.yaml");
    let data: Value = serde_yaml::from_str(
        &std::fs::read_to_string(data_path)?
    )?;

    let template_path = Path::new(&args.template);
    
    if template_path.is_file() {
        FileProcessor::process_file(
            &args.template,
            &data,
            args.output.as_deref(),
            args.strict
        )?;
    } else if template_path.is_dir() {
        FileProcessor::process_directory(
            &args.template,
            &data,
            args.output.as_deref(),
            args.strict
        )?;
    } else {
        return Err(TemplaturrError::InvalidPath(args.template));
    }
    
    Ok(())
}