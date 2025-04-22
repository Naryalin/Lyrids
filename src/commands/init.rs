use std::fs::{self, File};
use std::path::Path;

pub fn init_repo() {
    let repo_path = Path::new(".lyrids");

    if !repo_path.exists() {
        fs::create_dir_all(repo_path).expect("Failed to create .lyrids directory");
        println!("Initialized Lyrids repository");

        // Create subdirectories and files
        create_file_or_dir("commits");
        create_file_or_dir("snapshots");
        create_file_or_dir("branches.json");
        create_file_or_dir("index.json");
        create_file_or_dir("HEAD");

        println!("Repository structure created!");
    }else{
        println!("Repository already exists");
    }
}

fn create_file_or_dir(name: &str) {
    let path = Path::new(".lyrids").join(name);

    if name == "commits" || name == "snapshots" {
        fs::create_dir_all(&path)
            .unwrap_or_else(|e| panic!("Failed to create {} directory: {}", name, e));
    } else {
        File::create(&path)
            .unwrap_or_else(|e| panic!("Failed to create {} file: {}", name, e));
    }

}