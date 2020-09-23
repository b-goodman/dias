use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use console::style;

extern crate handlebars;
use handlebars::Handlebars;

pub fn merge(template_filepath: &Path, project_variables: &HashMap<&String, String>) -> String {
    let reg = Handlebars::new();
    let template_string =
        fs::read_to_string(template_filepath).expect("Error reading template file.");
    reg.render_template(&template_string, &json!(project_variables))
        .expect("Error")
}

pub fn write_file(template_dest_full: PathBuf, template_content: String) {
    let template_dest = template_dest_full
        .parent()
        .unwrap()
        .join(template_dest_full.file_stem().unwrap());

    let project_sub_dir = template_dest
        .parent()
        .expect("Could not determine parent dir.");
    if !project_sub_dir.exists() {
        fs::create_dir_all(project_sub_dir).expect("Error creating parent dir.");
    }
    fs::write(&template_dest, template_content).expect("Error writing template file.");

    println!("{} {:?}", style("Created").yellow(), &template_dest);
}

pub fn walk_directory<F: Fn(&Path) -> ()>(template_root: &PathBuf, handler: F) {
    for entry in WalkDir::new(template_root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();
        if file_path.is_file() {
            handler(file_path)
        }
    }
}
