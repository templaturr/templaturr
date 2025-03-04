use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "templaturr")]
#[command(about = "A blazing fast templating engine", long_about = None)]
pub struct CliArgs {
    /// Path to the input template file/directory
    #[arg(short, long)]
    pub template: String,

    /// Path to the input data file (YAML)
    #[arg(short, long)]
    pub data: Option<String>,

    /// Output path for rendered templates
    #[arg(short, long)]
    pub output: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Generate cauldron.yaml
    #[arg(long)]
    pub create_cauldron: bool,

    /// Strict mode: Fail on missing variables
    #[arg(long)]
    pub strict: bool,
}