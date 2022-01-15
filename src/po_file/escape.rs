pub(super) fn escape(unescaped: &str) -> String {
    if unescaped.find('\\').is_none()
        && unescaped.find('"').is_none()
        && unescaped.find('\n').is_none()
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
            _ => {
                escaped.push(c);
            }
        }
    }
    escaped
}

pub(super) fn unescape(escaped: &str) -> Result<String, &str> {
    if escaped.find('\\').is_none() {
        return Ok(escaped.to_string());
    }
    let escaped: Vec<char> = escaped.to_string().chars().collect();
    let mut unescaped = String::new();
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
                '"' => {
                    unescaped.push('"');
                }
                _ => {
                    return Err("Bad string escape sequence");
                }
            }
        }
        i += 1;
    }
    Ok(unescaped)
}

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
