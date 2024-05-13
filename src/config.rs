use std::{collections::HashMap, fs};
use serde::{Deserialize, Serialize};

use std::error::Error;
use crate::sample::Sample;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub samples: HashMap<String, Sample>,
}

impl Config {
    pub fn new(path: &str, name: &str) -> Self {
        /*
        Creates a new config file at the specified location.
        */

        let new_config: Self = Self {
            name: String::from(name),
            samples: HashMap::new(),
        };

        let _ = new_config.save(path);
        return new_config;
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        /*
        Load the specified config file.
        */

        let config_data: String = fs::read_to_string(path)?;
        return Ok(serde_json::from_str(config_data.as_str())?);
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        /*
        Save the config state to disk.
        */

        let config_data: String = serde_json::to_string_pretty(self)?;
        fs::write(path, config_data)?;
        return Ok(());
    }
}