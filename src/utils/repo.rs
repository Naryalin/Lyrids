use std::path::Path;

/// Verifica si `.lyrids/` existe. Retorna `true` si es un repo válido.
pub fn repo_exists() -> bool {
    Path::new(".lyrids").exists()
}