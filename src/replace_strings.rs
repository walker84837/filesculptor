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
