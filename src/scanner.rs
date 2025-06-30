use anyhow::Result;
use regex::Regex;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_target_dirs(root: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
    let re = Regex::new(pattern)?;
    let mut result = vec![];

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
    fn test_find_target_dirs_with_four_digit_pattern() -> Result<()> {
        // Create a temporary directory for testing.
        let temp_dir = tempdir()?;
        let root = temp_dir.path();

        // Create test directories.
        fs::create_dir_all(root.join("1234"))?; // Should match
        fs::create_dir_all(root.join("5678"))?; // Should match
        fs::create_dir_all(root.join("123"))?;  // Should not match
        fs::create_dir_all(root.join("12345"))?; // Should not match
        fs::create_dir_all(root.join("abcd"))?; // Should not match
        fs::create_dir_all(root.join("nested").join("9876"))?; // Should match

        // Call the function with a pattern for 4-digit directories.
        let pattern = r"^\d{4}$";
        let result = find_target_dirs(root, pattern)?;

        // Check that the results are correct.
        let mut expected: Vec<PathBuf> = vec![
            root.join("1234"),
            root.join("5678"),
            root.join("nested/9876"),
        ];
        expected.sort();
        let mut actual = result;
        actual.sort();

        assert_eq!(actual, expected);

        Ok(())
    }
}
