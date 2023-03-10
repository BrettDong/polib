#[derive(Debug)]
pub(crate) struct UnescapeError {
    seq: String,
}

impl UnescapeError {
    fn new(seq: char) -> Self {
        Self {
            seq: seq.to_string(),
        }
    }
}

impl std::fmt::Display for UnescapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid unescape sequence {}", self.seq)
    }
}

impl std::error::Error for UnescapeError {}

pub(super) fn escape(unescaped: &str) -> String {
    if unescaped.find('\\').is_none()
        && unescaped.find('"').is_none()
        && unescaped.find('\n').is_none()
        && unescaped.find('\r').is_none()
        && unescaped.find('\t').is_none()
    {
        return unescaped.to_string();
    }
    let mut escaped = String::new();
    for c in unescaped.to_string().chars() {
        match c {
            '\\' => {
                escaped.push_str("\\\\");
            }
            '"' => {
                escaped.push_str("\\\"");
            }
            '\n' => {
                escaped.push_str("\\n");
            }
            '\r' => {
                escaped.push_str("\\r");
            }
            '\t' => {
                escaped.push_str("\\t");
            }
            _ => {
                escaped.push(c);
            }
        }
    }
    escaped
}

pub(super) fn unescape(escaped: &str) -> Result<String, UnescapeError> {
    let first_backslash = escaped.find('\\');
    if let Some(i) = first_backslash {
        let mut unescaped = String::from(&escaped[0..i]);
        let escaped: Vec<char> = escaped[i..].to_string().chars().collect();
        let mut i = 0;
        while i < escaped.len() {
            if escaped[i] != '\\' || i == escaped.len() - 1 {
                unescaped.push(escaped[i]);
            } else {
                i += 1;
                match escaped[i] {
                    '\\' => {
                        unescaped.push('\\');
                    }
                    'n' => {
                        unescaped.push('\n');
                    }
                    'r' => {
                        unescaped.push('\r');
                    }
                    't' => {
                        unescaped.push('\t');
                    }
                    '"' => {
                        unescaped.push('"');
                    }
                    _ => {
                        return Err(UnescapeError::new(escaped[i]));
                    }
                }
            }
            i += 1;
        }
        Ok(unescaped)
    } else {
        Ok(escaped.to_string())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_escape() {
        use crate::po_file::escape::escape;
        let unescaped = "1\n2\n3\n";
        let expected = r"1\n2\n3\n";
        assert_eq!(escape(unescaped), expected);
    }

    #[test]
    fn test_unescape() {
        use crate::po_file::escape::unescape;
        let raw = r"1\n2\n3\n";
        let expected = "1\n2\n3\n";
        assert_eq!(unescape(raw).unwrap(), expected);
    }
}
