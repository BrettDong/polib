use std::collections::HashMap;

pub struct CatalogPluralRules {
    pub nplurals: usize,
    pub expr: String,
}

#[derive(Debug)]
pub struct InvalidPluralRulesError;

pub struct CatalogMetadata {
    pub project_id_version: String,
    pub pot_creation_date: String,
    pub po_revision_date: String,
    pub last_translator: String,
    pub language_team: String,
    pub mime_version: String,
    pub content_type: String,
    pub content_transfer_encoding: String,
    pub language: String,
    pub plural_rules: CatalogPluralRules,
}

impl CatalogPluralRules {
    pub fn parse(rules: &str) -> Result<Self, InvalidPluralRulesError> {
        let mut nplurals: usize = 0;
        let mut expr = String::new();
        for rule in rules.split(';') {
            let rule = rule.trim();
            if rule.is_empty() {
                continue;
            }
            let segments: Vec<&str> = rule.split('=').collect();
            if segments.len() != 2 {
                return Err(InvalidPluralRulesError);
            }
            match *segments.first().unwrap() {
                "nplurals" => {
                    nplurals = segments.last().unwrap().parse().unwrap();
                }
                "plural" => {
                    expr = segments.last().unwrap().to_string();
                }
                _ => {}
            }
        }
        if nplurals == 0 || expr.is_empty() {
            Err(InvalidPluralRulesError)
        } else {
            Ok(CatalogPluralRules { nplurals, expr })
        }
    }

    pub fn dump(&self) -> String {
        format!("nplurals={}; plural={};", self.nplurals, self.expr)
    }
}

impl CatalogMetadata {
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
