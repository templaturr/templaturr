use clap::Parser;
use serde_yaml::Value;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(name = "templaturr")]
#[command(about = "A high-performance templating engine", long_about = None)]
struct CliArgs {
    /// Path to the input template file or directory
    #[arg(short, long, value_name = "TEMPLATE")]
    template: String,

    /// Path to the input data file (YAML format, optional)
    #[arg(short, long, value_name = "DATA")]
    data: Option<String>,

    /// Path to the output file or directory
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<String>,

    /// Enables verbose logging
    #[arg(short, long, help = "Enable verbose logging")]
    verbose: bool,
}

fn main() {
    let args = CliArgs::parse();

    if args.verbose {
        println!("Verbose mode enabled");
    }

    // Determine data file path (default to cauldron.yaml if not provided)
    let data_path = args.data.unwrap_or_else(|| "cauldron.yaml".to_string());

    if !Path::new(&data_path).exists() {
        eprintln!("Error: Data file '{}' not found!", data_path);
        eprintln!("Create one using the '-create-cauldron' flag or provide it using '--data'.");
        std::process::exit(1);
    }

    let yaml_content = fs::read_to_string(&data_path).expect("Failed to read data file");
    let data: Value = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");

    let template_path = Path::new(&args.template);
    
    if template_path.is_file() {
        process_template_file(&args.template, &data, args.output.as_deref());
    } else if template_path.is_dir() {
        process_template_directory(&args.template, &data, args.output.as_deref());
    } else {
        eprintln!("Error: Template path '{}' is neither a file nor a directory.", args.template);
        std::process::exit(1);
    }
}

fn process_template_file(template_file: &str, data: &Value, output_path: Option<&str>) {
    let template_content = fs::read_to_string(template_file).expect("Failed to read template file");
    let output_content = replace_variables(&template_content, data);

    if let Some(output) = output_path {
        fs::write(output, output_content).expect("Failed to write output file");
        println!("Generated output saved to: {}", output);
    } else {
        println!("Generated Output:\n{}", output_content);
    }
}

fn process_template_directory(template_dir: &str, data: &Value, output_dir: Option<&str>) {
    for entry in WalkDir::new(template_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(template_dir).unwrap();
            let output_path = output_dir
                .map(|od| Path::new(od).join(relative_path))
                .unwrap_or_else(|| path.to_path_buf());
            
            let content = fs::read_to_string(path).expect("Failed to read template file");
            let processed_content = replace_variables(&content, data);

            fs::create_dir_all(output_path.parent().unwrap()).expect("Failed to create output directories");
            fs::write(&output_path, processed_content).expect("Failed to write processed file");

            println!("Processed: {}", path.display());
        }
    }
}

fn replace_variables(template: &str, data: &Value) -> String {
    let re = Regex::new(r"\{\[\s*(\w+)\s*\]\}").unwrap();
    re.replace_all(template, |caps: &regex::Captures| {
        let key = &caps[1];
        match &data[key] {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => caps[0].to_string(),
        }
    }).to_string()
}
