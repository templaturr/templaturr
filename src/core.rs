use regex::Regex;
use serde_yaml::Value;
use crate::error::TemplaturrError;

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn replace_variables(
        template: &str,
        data: &Value,
    ) -> Result<(String, Vec<String>), TemplaturrError> {
        let re = Regex::new(r"\{\[\s*(\w+)\s*\]\}")?;
        let mut result = String::new();
        let mut last_end = 0;
        let mut missing_vars = Vec::new();

        for cap in re.captures_iter(template) {
            let m = cap.get(0).unwrap();
            result.push_str(&template[last_end..m.start()]);
            
            let var_name = &cap[1];
            match data.get(var_name) {
                Some(Value::String(s)) => result.push_str(s),
                Some(Value::Number(n)) => result.push_str(&n.to_string()),
                Some(_) => result.push_str(m.as_str()), // Handle other types
                None => {
                    result.push_str(m.as_str());
                    missing_vars.push(var_name.to_string());
                }
            }
            
            last_end = m.end();
        }
        
        result.push_str(&template[last_end..]);
        Ok((result, missing_vars))
    }
}