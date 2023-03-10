/// The plural form resolution rule of the target language.
pub struct CatalogPluralRules {
    /// Total number of plural forms, including singular form.
    pub nplurals: usize,
    /// The plural form resolution expression in the function of n.
    pub expr: String,
}

impl Default for CatalogPluralRules {
    fn default() -> Self {
        Self {
            nplurals: 1,
            expr: String::from("0"),
        }
    }
}

/// Error type when parsing an invalid plural rules.
#[derive(Debug)]
pub struct PluralRulesError {
    message: String,
}

impl From<&str> for PluralRulesError {
    fn from(s: &str) -> Self {
        Self {
            message: s.to_string(),
        }
    }
}

impl From<String> for PluralRulesError {
    fn from(s: String) -> Self {
        Self { message: s }
    }
}

impl From<std::num::ParseIntError> for PluralRulesError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self {
            message: format!("cannot parse nplurals: {}", value),
        }
    }
}

impl std::fmt::Display for PluralRulesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid plural rules: {}", self.message)
    }
}

impl std::error::Error for PluralRulesError {}

impl CatalogPluralRules {
    /// Parse a plural resolution rules from string form stored in PO file.
    pub fn parse(rules: &str) -> Result<Self, PluralRulesError> {
        let mut nplurals: Option<usize> = None;
        let mut expr: Option<&str> = None;
        for rule in rules.split(';') {
            let rule = rule.trim();
            if rule.is_empty() {
                continue;
            }
            if let Some((key, value)) = rule.split_once('=') {
                match key {
                    "nplurals" => {
                        nplurals = Some(value.parse()?);
                    }
                    "plural" => {
                        expr = Some(value);
                    }
                    unrecognized => {
                        return Err(PluralRulesError::from(format!(
                            "unrecognized entry {}",
                            unrecognized
                        )));
                    }
                }
            } else {
                return Err(PluralRulesError::from(format!("cannot parse {}", rule)));
            }
        }
        if let (Some(nplurals), Some(expr)) = (nplurals, expr) {
            if nplurals == 0 {
                Err(PluralRulesError::from("nplurals equals to zero"))
            } else if expr.is_empty() {
                Err(PluralRulesError::from("plural rule expression is empty"))
            } else {
                Ok(CatalogPluralRules {
                    nplurals,
                    expr: String::from(expr),
                })
            }
        } else if nplurals.is_none() {
            Err(PluralRulesError::from("nplurals does not exist"))
        } else if expr.is_none() {
            Err(PluralRulesError::from(
                "plural rule expression does not exist",
            ))
        } else {
            std::unreachable!();
        }
    }

    /// Dump the plural resolution rules to string form to write to a PO file.
    pub fn dump(&self) -> String {
        format!("nplurals={}; plural={};", self.nplurals, self.expr)
    }
}
