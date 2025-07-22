use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

use crate::color::Color;

pub struct TemplateParser {
    colors: Vec<Color>,
    variables: HashMap<String, String>,
}

impl TemplateParser {
    pub fn new(colors: Vec<Color>) -> Self {
        let mut parser = TemplateParser {
            colors,
            variables: HashMap::new(),
        };
        
        parser.initialize_variables();
        
        parser
    }
    
    fn initialize_variables(&mut self) {
        for (i, color) in self.colors.iter().enumerate() {
            if i >= 16 {
                break;
            }
            
            self.variables.insert(format!("color{}", i), color.to_hex());
            
            // Stripped
            self.variables.insert(format!("color{}.strip", i), color.to_hex_stripped());
        }
        
        let background = &self.colors[0];
        let foreground = self.colors.get(15).unwrap_or(&self.colors[7]);
        
        self.variables.insert("background".to_string(), background.to_hex());
        self.variables.insert("background.strip".to_string(), background.to_hex_stripped());
        
        self.variables.insert("foreground".to_string(), foreground.to_hex());
        self.variables.insert("foreground.strip".to_string(), foreground.to_hex_stripped());
        
        self.variables.insert("cursor".to_string(), foreground.to_hex());
        self.variables.insert("cursor.strip".to_string(), foreground.to_hex_stripped());
    }
    
    pub fn parse_template(&self, template_content: &str) -> String {
        let re = Regex::new(r"\{([a-zA-Z0-9._]+)\}").unwrap();
        
        let result = re.replace_all(template_content, |caps: &regex::Captures| {
            let var_name = &caps[1];
            self.variables.get(var_name).cloned().unwrap_or_else(|| format!("{{{}}}", var_name))
        });
        
        result.to_string()
    }
    
    pub fn process_template_file(&self, template_path: &Path, output_path: &Path) -> std::io::Result<()> {
        // Read the template file
        let template_content = fs::read_to_string(template_path)?;
        
        let filled_content = self.parse_template(&template_content);
        
        // Write the filled template to the output path
        fs::write(output_path, filled_content)?;
        
        Ok(())
    }
    
    pub fn process_template_directory(&self, template_dir: &Path, output_dir: &Path) -> std::io::Result<Vec<PathBuf>> {
        let mut processed_files = Vec::new();
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }
        
        // Process each file in the template directory
        for entry in fs::read_dir(template_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let output_path = output_dir.join(&file_name);
                
                self.process_template_file(&path, &output_path)?;
                processed_files.push(output_path);
            }
        }
        
        Ok(processed_files)
    }
}