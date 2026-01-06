use crate::error::{FrontmatterError, Result};
use serde_yaml::Value;
use std::path::Path;

const FRONTMATTER_DELIMITER: &str = "---";

/// Represents parsed frontmatter with its source file
#[derive(Debug, Clone)]
pub struct Frontmatter {
    pub path: std::path::PathBuf,
    pub data: Value,
}

impl Frontmatter {
    /// Convert frontmatter to JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.data)
    }
}

/// Extract raw frontmatter string from content
fn extract_frontmatter_str(content: &str) -> Option<&str> {
    let content = content.trim_start();

    if !content.starts_with(FRONTMATTER_DELIMITER) {
        return None;
    }

    let after_first = &content[FRONTMATTER_DELIMITER.len()..];
    let after_first = after_first
        .strip_prefix('\n')
        .or_else(|| after_first.strip_prefix("\r\n"))?;

    let end_pos = after_first
        .find(&format!("\n{}", FRONTMATTER_DELIMITER))
        .or_else(|| after_first.find(&format!("\r\n{}", FRONTMATTER_DELIMITER)))?;

    Some(&after_first[..end_pos])
}

/// Parse frontmatter from a file
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Frontmatter> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).map_err(|e| FrontmatterError::IoError {
        path: path.to_path_buf(),
        source: e,
    })?;

    parse_content(path, &content)
}

/// Parse frontmatter from content with associated path
pub fn parse_content<P: AsRef<Path>>(path: P, content: &str) -> Result<Frontmatter> {
    let path = path.as_ref();

    let frontmatter_str = extract_frontmatter_str(content)
        .ok_or_else(|| FrontmatterError::NoFrontmatter(path.to_path_buf()))?;

    let data: Value =
        serde_yaml::from_str(frontmatter_str).map_err(|e| FrontmatterError::YamlError {
            path: path.to_path_buf(),
            source: e,
        })?;

    Ok(Frontmatter {
        path: path.to_path_buf(),
        data,
    })
}

/// Parse all markdown files in a directory
pub fn parse_directory<P: AsRef<Path>>(dir: P, recursive: bool) -> Vec<Result<Frontmatter>> {
    let walker = if recursive {
        walkdir::WalkDir::new(dir)
    } else {
        walkdir::WalkDir::new(dir).max_depth(1)
    };

    walker
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(e) => {
                let path = e.path();
                if path.is_file() && is_markdown_file(path) {
                    Some(parse_file(path))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(FrontmatterError::WalkDirError(e))),
        })
        .collect()
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter() {
        let content = r#"---
title: Test
date: 2024-01-01
---
# Content here"#;

        let fm = extract_frontmatter_str(content).unwrap();
        assert!(fm.contains("title: Test"));
        assert!(fm.contains("date: 2024-01-01"));
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Just a heading\nSome content";
        assert!(extract_frontmatter_str(content).is_none());
    }

    #[test]
    fn test_parse_content() {
        let content = r#"---
title: My Post
tags:
  - rust
  - cli
---
# Content"#;

        let fm = parse_content("test.md", content).unwrap();
        assert_eq!(fm.data["title"], "My Post");
    }
}
