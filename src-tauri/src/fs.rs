use std::path::Path;

pub fn path_extension_contains(path: &Path, pattern: &str) -> bool {
    match path.as_os_str().to_str() {
        Some(path_str) => path_str.contains(pattern),
        None => false,
    }
}

pub fn path_extension_contains_any(path: &Path, patterns: &[&str]) -> bool {
    patterns
        .iter()
        .map(|pattern| path_extension_contains(path, pattern))
        .reduce(|acc, e| acc && e)
        .unwrap_or(false)
}
