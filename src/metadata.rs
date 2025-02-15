//! Defines `CatalogMetadata` struct.

use std::collections::HashMap;

use crate::plural::*;

/// Metadata of a translation catalog.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

/// Error in parsing metadata of a catalog
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataParseError {
    message: String,
}

impl From<PluralRulesError> for MetadataParseError {
    fn from(e: PluralRulesError) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl std::fmt::Display for MetadataParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid catalog metadata: {}", self.message)
    }
}

impl std::error::Error for MetadataParseError {}

impl CatalogMetadata {
    /// Create a new empty catalog metadata.
    pub fn new() -> Self {
        CatalogMetadata::default()
    }

    /// Parse catalog metadata from string form stored in PO file.
    pub fn parse(metadata: &str) -> Result<Self, MetadataParseError> {
        let mut key_values = HashMap::new();
        for line in metadata.split('\n') {
            if let Some((key, value)) = line.split_once(':') {
                key_values.insert(key, value.trim());
            }
        }
        let res = CatalogMetadata {
            project_id_version: key_values
                .get("Project-Id-Version")
                .unwrap_or(&"")
                .to_string(),
            pot_creation_date: key_values
                .get("POT-Creation-Date")
                .unwrap_or(&"")
                .to_string(),
            po_revision_date: key_values
                .get("PO-Revision-Date")
                .unwrap_or(&"")
                .to_string(),
            last_translator: key_values.get("Last-Translator").unwrap_or(&"").to_string(),
            language_team: key_values.get("Language-Team").unwrap_or(&"").to_string(),
            mime_version: key_values.get("MIME-Version").unwrap_or(&"").to_string(),
            content_type: key_values.get("Content-Type").unwrap_or(&"").to_string(),
            content_transfer_encoding: key_values
                .get("Content-Transfer-Encoding")
                .unwrap_or(&"")
                .to_string(),
            language: key_values.get("Language").unwrap_or(&"").to_string(),
            plural_rules: CatalogPluralRules::parse(
                key_values
                    .get("Plural-Forms")
                    .unwrap_or(&"nplurals=1; plural=0;"),
            )?,
        };
        Ok(res)
    }

    fn dump(&self, include_pot_creation_date: bool) -> String {
        let mut buffer = String::new();
        if !self.project_id_version.is_empty() {
            buffer.push_str(format!("Project-Id-Version: {}\n", self.project_id_version).as_str());
        }
        if include_pot_creation_date && !self.pot_creation_date.is_empty() {
            buffer.push_str(format!("POT-Creation-Date: {}\n", self.pot_creation_date).as_str());
        }
        if !self.po_revision_date.is_empty() {
            buffer.push_str(format!("PO-Revision-Date: {}\n", self.po_revision_date).as_str());
        }
        if !self.last_translator.is_empty() {
            buffer.push_str(format!("Last-Translator: {}\n", self.last_translator).as_str());
        }
        if !self.language_team.is_empty() {
            buffer.push_str(format!("Language-Team: {}\n", self.language_team).as_str());
        }
        if !self.mime_version.is_empty() {
            buffer.push_str(format!("MIME-Version: {}\n", self.mime_version).as_str());
        }
        if !self.content_type.is_empty() {
            buffer.push_str(format!("Content-Type: {}\n", self.content_type).as_str());
        }
        if !self.content_transfer_encoding.is_empty() {
            buffer.push_str(
                format!(
                    "Content-Transfer-Encoding: {}\n",
                    self.content_transfer_encoding
                )
                .as_str(),
            );
        }
        if !self.language.is_empty() {
            buffer.push_str(format!("Language: {}\n", self.language).as_str());
        }
        buffer.push_str(format!("Plural-Forms: {}\n", self.plural_rules.dump()).as_str());
        buffer
    }

    /// Export metadata for writing to a PO file.
    pub fn export_for_po(&self) -> String {
        self.dump(true)
    }

    /// Export metadata for writing to a MO file.
    pub fn export_for_mo(&self) -> String {
        self.dump(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_parse() {
        let input_data = "Plural-Forms: nplurals=1; plural=0;\n";
        let res = CatalogMetadata::parse(input_data);
        let catalog_metadata = res.unwrap();
        assert_eq!(catalog_metadata.project_id_version, "");
        assert_eq!(catalog_metadata.pot_creation_date, "");
        assert_eq!(catalog_metadata.po_revision_date, "");
        assert_eq!(catalog_metadata.last_translator, "");
        assert_eq!(catalog_metadata.language_team, "");
        assert_eq!(catalog_metadata.mime_version, "");
        assert_eq!(catalog_metadata.content_type, "");
        assert_eq!(catalog_metadata.content_transfer_encoding, "");
        assert_eq!(catalog_metadata.language, "");
    }

    #[test]
    fn test_metadata_dump() {
        let input_data = "Plural-Forms: nplurals=1; plural=0;\n";
        let res = CatalogMetadata::parse(input_data);
        let catalog_metadata = res.unwrap();
        let dump_res = catalog_metadata.dump(true);
        assert_eq!(dump_res, input_data);
    }
}
