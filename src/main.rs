use serde_yaml::Value;
use std::fs;
use regex::Regex;

fn main() {
    let yaml_content = fs::read_to_string("cauldron.yaml")
        .expect("Failed to read cauldron.yaml");

    let data: Value = serde_yaml::from_str(&yaml_content)
        .expect("Failed to parse YAML");

    println!("Parsed YAML data:\n {:?}", data);

    let template_content = fs::read_to_string("template.txt")
        .expect("Failed to read template.txt");

    let output = replace_variables(&template_content, &data);

    println!("{}", output);
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