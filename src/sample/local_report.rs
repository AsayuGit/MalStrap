use std::{collections::HashSet, fmt};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocalReport {
    pub custom_tags: Option<HashSet<String>>,
}

impl LocalReport {
    pub fn new() -> Self {
        return Self {
            custom_tags: None,
        };
    }

    pub fn add_tag(&mut self, tag: &str) {
        match &mut self.custom_tags {
            Some(custom_tags) => {
                custom_tags.insert(tag.to_string());
            },
            None => {
                let mut custom_tags: HashSet<String> = HashSet::new();
                custom_tags.insert(tag.to_string());
                self.custom_tags = Some(custom_tags);
            }
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        match &mut self.custom_tags {
            Some(custom_tags) => {
                custom_tags.remove(tag);
                if custom_tags.is_empty() {
                    self.custom_tags = None;
                }
            },
            None => {}
        }
    }
}

impl fmt::Display for LocalReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Workspace Report:")?;

        if let Some(tags) = &self.custom_tags {
            write!(f, "\n    Custom tags:")?;
            for tag in tags {
                write!(f, "\n        - {}", tag)?;
            }
        }

        return Ok(());
    }
}