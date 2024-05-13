use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocalReport {

}

impl LocalReport {
    pub fn _new() -> Self {
        // Any locally related domains ?
        // Any locally related ip ?
        // Any locally related files ?

        return Self {

        };
    }
}

impl fmt::Display for LocalReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", "");
    }
}