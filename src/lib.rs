use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MakefileDependency {
    target: String,
    dependencies: Vec<String>,
}

pub fn parse_makefile(file_path: &str) -> Result<Vec<MakefileDependency>, std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let mut dependencies = Vec::new();
    
    for line in content.lines() {
        if line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let target = parts[0].trim().to_string();
            let deps = parts[1].split_whitespace()
                               .map(|s| s.to_string())
                               .collect::<Vec<String>>();
            dependencies.push(MakefileDependency { target, dependencies: deps });
        }
    }
    
    Ok(dependencies)
}
