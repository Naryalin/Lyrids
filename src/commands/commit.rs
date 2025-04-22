use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use crate::utils::repo::repo_exists;
use blake3::hash;
use serde_json;

pub fn commit_repo() {
    // Paso 1: Verificar si estamos dentro de un repo Lyrids
    if !repo_exists() {
        println!("Not a Lyrids repository (or any of the parent directories)");
        return;
    }

    // Paso 2: Cargar index.json
    let index_path = Path::new(".lyrids/index.json");
    let mut index_data: HashMap<String, String> = load_index_data(index_path);

    // Paso 3: Listar todos los archivos en el directorio actual
    let current_files = get_files_in_directory();

    // Paso 4: Detectar archivos nuevos o modificados
    let mut changes = vec![];
    for file in current_files {
        let file_path = Path::new(&file);
        let content = fs::read(file_path).expect("Failed to read file");
        let file_hash = hash(&content).to_string();

        // Si el archivo no está en el index o su hash ha cambiado, lo agregamos a los cambios
        if let Some(stored_hash) = index_data.get(&file) {
            if stored_hash != &file_hash {
                changes.push(file.clone());
                index_data.insert(file, file_hash);
            }
        } else {
            // Si el archivo no está en el index, lo agregamos
            changes.push(file.clone());
            index_data.insert(file, file_hash);
        }
    }

    // Si hay cambios, proceder con el commit
    if !changes.is_empty() {
        println!("Committing changes...");
        create_commit(&changes, &index_data);
        update_index_json(index_path, &index_data);
        println!("Commit successful!");
    } else {
        println!("No changes to commit.");
    }
}

// Carga el contenido de index.json
fn load_index_data(path: &Path) -> HashMap<String, String> {
    if path.exists() {
        let content = fs::read_to_string(path).expect("Failed to read index.json");
        if content.trim().is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&content).expect("Failed to parse index.json")
        }
    } else {
        HashMap::new()
    }
}

// Obtiene todos los archivos en el directorio actual
fn get_files_in_directory() -> Vec<String> {
    let current_dir = Path::new(".");
    let mut files = vec![];

    for entry in fs::read_dir(current_dir).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_str) = file_name.to_str() {
                        files.push(file_str.to_string());
                    }
                }
            }
        }
    }
    files
}

// Crea un commit en el directorio .lyrids/commits/
fn create_commit(changes: &[String], index_data: &HashMap<String, String>) {
    let commit_id = blake3::hash(&format!("{:?}", changes).as_bytes()).to_hex();
    let commit_path = Path::new(".lyrids/commits").join(&*commit_id);

    // Guardar commit en un archivo
    let mut commit_file = File::create(commit_path).expect("Failed to create commit file");
    for file in changes {
        writeln!(commit_file, "{}: {}", file, index_data[file]).expect("Failed to write to commit file");
    }
}

// Actualiza index.json con los nuevos hashes
fn update_index_json(path: &Path, index_data: &HashMap<String, String>) {
    let index_content = serde_json::to_string_pretty(&index_data)
        .expect("Failed to serialize index data");
    fs::write(path, index_content).expect("Failed to write index.json");
}
