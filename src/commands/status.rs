use std::collections::HashMap;
use std::fs;
use crate::utils::repo::repo_exists;
use std::path::Path;

pub fn status_repo() {
    // Paso 1: Verificar si estamos dentro de un repo Lyrids
    if !repo_exists() {
        println!("Not a Lyrids repository (or any of the parent directories)");
        return;
    }

    // Paso 2: Cargar index.json
    let index_path = Path::new(".lyrids/index.json");

    // Si no existe o está vacío, asumimos staging vacío
    let index_data: HashMap<String, String> = if index_path.exists() {
        let content = fs::read_to_string(index_path)
            .expect("Failed to read index.json");

        if content.trim().is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&content)
                .expect("Failed to parse index.json")
        }
    } else {
        HashMap::new()
    };

    // Por ahora solo mostrar lo que está en el index.json
    println!("Staged files:");
    let mut modified_files = vec![];

    for (file, stored_hash) in &index_data {
        if let Ok(content) = fs::read(file) {
            let current_hash = blake3::hash(&content).to_string();
            if &current_hash != stored_hash {
                modified_files.push(file);
            }
        }
    }

    if !modified_files.is_empty() {
        println!("\nChanges not staged for commit:");
        for file in modified_files {
            println!("  modified:   {}", file);
        }
    }

    // Más adelante: comparar contra el sistema de archivos actual
}
