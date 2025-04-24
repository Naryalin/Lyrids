use globset::{Glob, GlobSet, GlobSetBuilder};
use std::fs;
use std::path::Path;

pub fn get_ignore_matcher(ignore_file: &str) -> GlobSet {
    let mut builder = GlobSetBuilder::new();

    // Ignora siempre .lyrids (esto es fijo)
    builder.add(Glob::new("**/.lyrids/**").unwrap());

    if let Ok(lines) = fs::read_to_string(ignore_file) {
        for line in lines.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let glob_pattern = if trimmed.starts_with('/') {
                // Trata como relativo a raíz del repo
                format!("{}/**", trimmed.trim_start_matches('/'))
            } else if trimmed.ends_with('/') {
                // Si termina en /, es un directorio
                format!("**/{}**", trimmed)
            } else {
                trimmed.to_string()
            };

            if let Ok(glob) = Glob::new(&glob_pattern) {
                builder.add(glob);
            }
        }
    }

    builder.build().unwrap()
}

pub fn is_ignored(path: &Path, matcher: &GlobSet) -> bool {
    // Convierte a una ruta estilo Unix antes de comparar
    if let Some(path_str) = path.to_str() {
        let unix_style_path = path_str.replace("\\", "/");
        matcher.is_match(unix_style_path)
    } else {
        false
    }
}