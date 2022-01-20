//! `CatalogMetadata` and `CatalogPluralRules` struct, and associated functions.

use std::collections::HashMap;

/// The plural form resolution rule of the target language.
pub struct CatalogPluralRules {
    /// Total number of plural forms, including singular form.
    pub nplurals: usize,
    /// The plural form resolution expression in the function of n.
    pub expr: String,
}

/// Error type when parsing an invalid plural rules.
#[derive(Debug)]
pub struct InvalidPluralRulesError;

/// Metadata of a translation catalog.
pub struct CatalogMetadata {
    /// `Project-Id-Version`
    pub project_id_version: String,
    /// `POT-Creation-Date`
    pub pot_creation_date: String,
    /// `PO-Revision-Date`
    pub po_revision_date: String,
    /// `Last-Translator`
    pub last_translator: String,
    /// `Language-Team`
    pub language_team: String,
    /// `MIME-Version`
    pub mime_version: String,
    /// `Content-Type`
    pub content_type: String,
    /// `Content-Transfer-Encoding`
    pub content_transfer_encoding: String,
    /// `Language`
    pub language: String,
    /// `Plural-Forms`
    pub plural_rules: CatalogPluralRules,
}

impl CatalogPluralRules {
    /// Parse a plural resolution rules from string form stored in PO file.
    pub fn parse(rules: &str) -> Result<Self, InvalidPluralRulesError> {
        let mut nplurals: usize = 0;
        let mut expr = String::new();
        for rule in rules.split(';') {
            let rule = rule.trim();
            if rule.is_empty() {
                continue;
            }
            if let Some((key, value)) = rule.split_once('=') {
                match key {
                    "nplurals" => {
                        nplurals = value.parse().unwrap();
                    }
                    "plural" => {
                        expr = value.to_string();
                    }
                    _ => {}
                }
            } else {
                return Err(InvalidPluralRulesError);
            }
        }
        if nplurals == 0 || expr.is_empty() {
            Err(InvalidPluralRulesError)
        } else {
            Ok(CatalogPluralRules { nplurals, expr })
        }
    }

    /// Dump the plural resolution rules to string form to write to a PO file.
    pub fn dump(&self) -> String {
        format!("nplurals={}; plural={};", self.nplurals, self.expr)
    }
}

impl Default for CatalogMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalogMetadata {
    /// Create a new empty catalog metadata.
    pub fn new() -> Self {
        CatalogMetadata {
            project_id_version: String::new(),
            pot_creation_date: String::new(),
            po_revision_date: String::new(),
            last_translator: String::new(),
            language_team: String::new(),
            mime_version: String::new(),
            content_type: String::new(),
            content_transfer_encoding: String::new(),
            language: String::new(),
            plural_rules: CatalogPluralRules {
                nplurals: 1,
                expr: String::from("0"),
            },
        }
    }

    /// Parse catalog metadata from string form stored in PO file.
    pub fn parse(metadata: &str) -> Self {
        let mut key_values = HashMap::new();
        for line in metadata.split('\n') {
            if let Some((key, value)) = line.split_once(':') {
                key_values.insert(key, value.trim());
            }
        }
        CatalogMetadata {
            project_id_version: key_values.get("Project-Id-Version").unwrap().to_string(),
            pot_creation_date: key_values.get("POT-Creation-Date").unwrap().to_string(),
            po_revision_date: key_values.get("PO-Revision-Date").unwrap().to_string(),
            last_translator: key_values.get("Last-Translator").unwrap().to_string(),
            language_team: key_values.get("Language-Team").unwrap().to_string(),
            mime_version: key_values.get("MIME-Version").unwrap().to_string(),
            content_type: key_values.get("Content-Type").unwrap().to_string(),
            content_transfer_encoding: key_values
                .get("Content-Transfer-Encoding")
                .unwrap()
                .to_string(),
            language: key_values.get("Language").unwrap().to_string(),
            plural_rules: CatalogPluralRules::parse(key_values.get("Plural-Forms").unwrap())
                .unwrap(),
        }
    }

    /// Dump the metadata to string form to write to a PO file.
    pub fn dump(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(format!("Project-Id-Version: {}\n", self.project_id_version).as_str());
        buffer.push_str(format!("POT-Creation-Date: {}\n", self.pot_creation_date).as_str());
        buffer.push_str(format!("PO-Revision-Date: {}\n", self.po_revision_date).as_str());
        buffer.push_str(format!("Last-Translator: {}\n", self.last_translator).as_str());
        buffer.push_str(format!("Language-Team: {}\n", self.language_team).as_str());
        buffer.push_str(format!("MIME-Version: {}\n", self.mime_version).as_str());
        buffer.push_str(format!("Content-Type: {}\n", self.content_type).as_str());
        buffer.push_str(
            format!(
                "Content-Transfer-Encoding: {}\n",
                self.content_transfer_encoding
            )
            .as_str(),
        );
        buffer.push_str(format!("Language: {}\n", self.language).as_str());
        buffer.push_str(format!("Plural-Forms: {}\n", self.plural_rules.dump()).as_str());
        buffer
    }
}
