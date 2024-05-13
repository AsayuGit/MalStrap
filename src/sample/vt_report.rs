use core::fmt;

use serde::{Deserialize, Serialize};
use vt3::VtClient;

use chrono::DateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
pub struct VtReport {
    pub alternative_names: Option<Vec<String>>,
    pub first_seen: Option<i64>,


    // Detected by which antivirus
    // Undected by which antivirus

    // Any related doamains ?
    // Any related ip ?
    // Any related files ?
}

impl VtReport {
    pub fn new(vt_key: &str,  hash: [u8; 32]) -> Option<Self> {
        let vt: VtClient = VtClient::new(vt_key);

        let mut alternative_names: Option<Vec<String>> = None;
        let mut first_seen: Option<i64> = None;

        if let Ok(file_info) = vt.file_info(hex::encode(hash).as_str()) {
            if let Some(data) = file_info.data {
                if let Some(attributes) = data.attributes {
                    alternative_names = attributes.names;
                    first_seen = attributes.first_seen_itw_date;
                }
            }
        }

        return Some(Self {
            alternative_names,
            first_seen,
        });
    }
}

impl fmt::Display for VtReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Virus-Total Report:")?;

        if let Some(first_seen) = &self.first_seen {
            let time: DateTime<Utc> = DateTime::from_timestamp(*first_seen, 0).unwrap();
            write!(f, "\n    First seen : {}", time)?;
        }

        if let Some(alternative_names) = &self.alternative_names {
            write!(f, "\n    Alternative names :")?;
            for name in alternative_names {
                write!(f, "\n        - {}", name)?;
            }
        }

        return Ok(());
    }
}