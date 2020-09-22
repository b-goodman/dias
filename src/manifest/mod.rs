use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub project_type: String,
    pub variables: Vec<String>,
}

pub fn load(manifest_path: &PathBuf) -> Manifest {
    match fs::read_to_string(&manifest_path) {
        Ok(str) => serde_yaml::from_str(&str).unwrap(),
        Err(e) => panic!(
            "Error loading manifest {} - {}",
            &manifest_path.display(),
            e
        ),
    }
}
