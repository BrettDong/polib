//! Defines `MessageFlags` struct.

use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Represents the set of flags in a message
#[derive(Debug)]
pub struct MessageFlags {
    /// Vector of individual flags
    pub entries: Vec<String>,
}

impl Default for MessageFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for MessageFlags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let flags = s.replace('\n', "");
        let segments = flags.split(',');
        let mut result = Self::new();
        for x in segments {
            if !x.is_empty() {
                result.entries.push(String::from(x.trim()));
            }
        }
        Ok(result)
    }
}

impl Display for MessageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "")
        } else {
            write!(f, "{}", self.entries.join(", "))
        }
    }
}

impl MessageFlags {
    /// Create an empty set of flags
    pub fn new() -> Self {
        MessageFlags { entries: vec![] }
    }

    /// Is the set of flags empty?
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Is a flag present?
    pub fn contains(&self, flag: &str) -> bool {
        let flag = flag.to_string();
        self.entries.contains(&flag)
    }

    /// Is fuzzy flag present?
    pub fn is_fuzzy(&self) -> bool {
        self.contains("fuzzy")
    }
}

#[cfg(test)]
mod test {
    use crate::message::MessageFlags;
    use std::str::FromStr;

    #[test]
    fn test_flags_from_string() {
        assert_eq!(
            MessageFlags::from_str("").unwrap().entries,
            Vec::<String>::new()
        );
        assert_eq!(
            MessageFlags::from_str("fuzzy").unwrap().entries,
            vec!["fuzzy"]
        );
        assert_eq!(
            MessageFlags::from_str("c-format, fuzzy").unwrap().entries,
            vec!["c-format", "fuzzy"]
        );
    }

    #[test]
    fn test_flags_to_string() {
        assert_eq!(MessageFlags { entries: vec![] }.to_string(), "");
        assert_eq!(
            MessageFlags {
                entries: vec![String::from("fuzzy")]
            }
            .to_string(),
            "fuzzy"
        );
        assert_eq!(
            MessageFlags {
                entries: vec![String::from("c-format"), String::from("fuzzy")]
            }
            .to_string(),
            "c-format, fuzzy"
        );
    }
}
