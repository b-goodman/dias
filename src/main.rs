use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use dialoguer::{theme::ColorfulTheme, Input, Select};
use pathdiff::diff_paths;

#[macro_use]
extern crate serde_json;

mod manifest;
mod template;

fn discover_templates() -> HashMap<String, PathBuf> {
    let current_dir = env::current_dir().ok();

    let paths = fs::read_dir(current_dir.expect("Cannot locate working directory")).unwrap();

    let mut templates: HashMap<String, PathBuf> = HashMap::new();

    for path_result in paths {
        let path = path_result.unwrap().path();
        let metadata = std::fs::metadata(&path).unwrap();
        if metadata.is_dir() {
            if path.join("manifest.yml").exists() {
                let manifest_path = path.join("manifest.yml");
                templates.insert(manifest::load(&manifest_path).project_type, path);
            }
        }
    }

    return templates;
}

fn main() {
    let working_dir = env::current_dir().expect("Error");

    let templates_registry = discover_templates();
    let mut selections = vec![];

    for project_type in templates_registry.keys() {
        selections.push(project_type);
    }

    let project_type_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select project to scaffold")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let project_name = Input::<String>::new()
        .with_prompt("Enter project name")
        .interact()
        .unwrap();

    let selected_project = selections[project_type_selection];

    let template_root = templates_registry
        .get(selected_project)
        .expect("Error locating template root");

    let template_files_root = template_root.join("template");

    let template_variables = manifest::load(&template_root.join("manifest.yml")).variables;
    let mut template_variables_resp: HashMap<&String, String> = HashMap::new();

    for var in &template_variables {
        let value = Input::<String>::new().with_prompt(var).interact();
        template_variables_resp.insert(var, value.unwrap());
    }

    let template_file_handler = |file_path: &Path| -> () {
        let template_output = template::merge(file_path, &template_variables_resp);

        let filepath_relative =
            diff_paths(file_path, &template_files_root).expect("Error subtracting template path.");

        let template_dest = working_dir.join(&project_name).join(filepath_relative);

        template::write_file(template_dest, template_output);
    };

    template::walk_directory(&template_files_root, template_file_handler);
}
