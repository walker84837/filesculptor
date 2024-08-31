use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

/// Defines errors that can occur during the string replacement process.
#[derive(Debug, Error)]
pub enum ProcessingError {
    /// Error variant for cases where an invalid regular expression is provided.
    #[error("Invalid regular expression: {0}")]
    InvalidRegex(String),
}

/// Represents a replacement strategy, which can either be a direct string replacement or a regex-based replacement.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Replacement {
    /// A direct string replacement. The entire match of the key will be replaced with this string.
    Exact(String),

    /// A regex-based replacement, where `pattern` is the regex pattern to match against, and `replacement` is the string to replace matches with.
    Regex {
        /// The regex pattern to match in the text.
        pattern: String,

        /// The replacement string to use for each match found by the regex pattern.
        replacement: String,
    },
}

/// Configuration structure that holds a map of keys to their corresponding replacements.
/// Each key-value pair specifies a replacement rule, which can either be an exact match or a regex-based substitution.
#[derive(Deserialize)]
pub struct Config {
    /// A HashMap where the key is a string to find, and the value is a `Replacement` specifying how to replace it.
    pub changes: HashMap<String, Replacement>,
}

/// Replaces occurrences of keys in the provided text with their corresponding replacements defined in the `changes` map.
pub fn replace_strings(
    text: &str,
    changes: &HashMap<String, Replacement>,
) -> Result<String, ProcessingError> {
    let mut result = text.to_string();

    for (key, replacement) in changes {
        match replacement {
            Replacement::Exact(replacement) => {
                result = result.replace(key, replacement);
            }
            Replacement::Regex {
                pattern,
                replacement,
            } => {
                let re = Regex::new(pattern)
                    .map_err(|e| ProcessingError::InvalidRegex(e.to_string()))?;

                result = re.replace_all(&result, replacement.as_str()).into_owned();
            }
        }
    }

    Ok(result)
}
