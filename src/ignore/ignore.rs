use globset::{Glob, GlobSet, GlobSetBuilder};
use std::fs;
use std::path::Path;

pub fn get_ignore_matcher(ignore_file: &str) -> GlobSet {
    let mut builder = GlobSetBuilder::new();

    // Ignora siempre .lyrids (esto es fijo)
    builder.add(Glob::new(".lyrids/**").unwrap());

    if let Ok(lines) = fs::read_to_string(ignore_file) {
        for line in lines.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let normalized = if trimmed.starts_with('/') {
                // Si empieza con '/', es relativo a raíz
                format!(".{}", trimmed) // ejemplo: `/target` -> `./target`
            } else {
                // Si no, ignoramos en cualquier subdirectorio
                format!("**/{}", trimmed)
            };

            if let Ok(glob) = Glob::new(&normalized) {
                builder.add(glob);
            }
        }
    }

    builder.build().unwrap()
}

pub fn is_ignored(path: &Path, matcher: &GlobSet) -> bool {
    matcher.is_match(path)
}