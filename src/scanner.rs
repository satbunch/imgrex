use anyhow::{Context, Result};
use regex::Regex;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_target_dirs(root: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
    let re = Regex::new(pattern).with_context(|| format!("Invalid regex pattern: {}", pattern))?;

    let mut result = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_dir() {
            let file_name = entry.file_name().to_string_lossy();
            if re.is_match(&file_name) {
                result.push(entry.path().to_path_buf());
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_find_matching_dirs() {
        // Create a temporary directory for testing.
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        // Create test directories.
        fs::create_dir(root.join("1234")).unwrap(); // Should match
        fs::create_dir(root.join("5678")).unwrap(); // Should match
        fs::create_dir(root.join("123")).unwrap(); // Should not match
        fs::create_dir(root.join("12345")).unwrap(); // Should not match
        fs::create_dir(root.join("abcd")).unwrap(); // Should not match

        // Call the function with a pattern for 4-digit directories.
        let pattern = r"^\d{4}$";
        let result = find_target_dirs(root, pattern).unwrap();

        let names: Vec<_> = result
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(names.contains(&"1234".to_string()));
        assert!(names.contains(&"5678".to_string()));
        assert!(!names.contains(&"123".to_string()));
        assert!(!names.contains(&"12345".to_string()));
        assert!(!names.contains(&"abcd".to_string()));
    }

    #[test]
    fn test_no_match_dirs() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        fs::create_dir(root.join("test")).unwrap();
        fs::create_dir(root.join("folder")).unwrap();

        let dirs = find_target_dirs(root, r"^d{4}$").unwrap();
        assert_eq!(dirs.len(), 0);
    }

    #[test]
    fn test_invalid_regex() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        let result = find_target_dirs(root, r"[\");
        assert!(result.is_err());
    }
}
