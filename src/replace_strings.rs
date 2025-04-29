use regex::Regex;
use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessingError {
    #[error("Invalid regular expression: {0}")]
    InvalidRegex(String),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Replacement {
    Exact(String),
    Regex {
        pattern: String,
        replacement: String,
    },
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub changes: HashMap<String, Replacement>,
    pub recursive: bool,
    pub filter: Option<String>,
    pub invert: bool,
}

impl Config {
    pub fn load(config_path: &PathBuf) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

pub fn replace_strings<'a>(
    text: &'a str,
    changes: &HashMap<String, Replacement>,
) -> Result<Cow<'a, str>, ProcessingError> {
    let mut result = Cow::Borrowed(text);
    for (key, replacement) in changes {
        match replacement {
            Replacement::Exact(replacement) => {
                if result.contains(key) {
                    result = Cow::Owned(result.replace(key, replacement));
                }
            }
            Replacement::Regex {
                pattern,
                replacement,
            } => {
                let re = Regex::new(pattern)
                    .map_err(|e| ProcessingError::InvalidRegex(e.to_string()))?;
                if re.is_match(&result) {
                    result = Cow::Owned(re.replace_all(&result, replacement.as_str()).into_owned());
                }
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_exact_replacement() {
        let mut changes = HashMap::new();
        changes.insert("hello".to_string(), Replacement::Exact("hi".to_string()));

        let result = replace_strings("hello world", &changes).unwrap();
        assert_eq!(result, "hi world");
    }

    #[test]
    fn test_regex_replacement() {
        let mut changes = HashMap::new();
        changes.insert(
            "pattern".to_string(),
            Replacement::Regex {
                pattern: r"\d+".to_string(),
                replacement: "#".to_string(),
            },
        );

        let result = replace_strings("abc123def", &changes).unwrap();
        assert_eq!(result, "abc#def");
    }

    #[test]
    fn test_invalid_regex() {
        let mut changes = HashMap::new();
        changes.insert(
            "pattern".to_string(),
            Replacement::Regex {
                pattern: r"(".to_string(), // Invalid regex
                replacement: "#".to_string(),
            },
        );

        let result = replace_strings("test", &changes);
        assert!(matches!(result, Err(ProcessingError::InvalidRegex(_))));
    }

    #[test]
    fn test_no_replacements() {
        let changes = HashMap::new();
        let input = "unchanged";
        let result = replace_strings(input, &changes).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_config_load_invalid_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bad_config.json");

        let invalid_json = r#"{ "changes": "not_a_map" }"#;

        let mut file = File::create(&file_path).unwrap();
        write!(file, "{}", invalid_json).unwrap();

        let result = Config::load(&file_path);
        assert!(result.is_err());
    }
}
