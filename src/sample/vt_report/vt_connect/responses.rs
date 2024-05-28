use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct VTVotes {
    pub harmless: u64,
    pub malicious: u64,
}

#[derive(Debug, Deserialize)]
pub struct VTStats {
    pub harmless: Option<u64>,
    pub malicious: Option<u64>,
    pub suspicious: Option<u64>,
    pub undetected: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct VTAttributes {
    pub magic: Option<String>,
    pub vhash: Option<String>,
    pub ssdeep: Option<String>,
    pub names: Option<Vec<String>>,
    pub reputation: Option<i64>,
    pub last_analysis_stats: Option<VTStats>,
    pub total_votes: Option<VTVotes>,
    pub type_tag: Option<String>,
    pub type_tags: Option<Vec<String>>,
    pub last_submission_date: Option<i64>,
    pub sha1: Option<String>,
    pub size: Option<usize>,
    pub md5: Option<String>,
    pub sha256: Option<String>,
    pub meaningful_name: Option<String>,
    pub last_analysis_date: Option<i64>,
    pub last_modification_date: Option<i64>,
    pub times_submitted: Option<i64>,
    pub type_description: Option<String>,
    pub type_extension: Option<String>,
    pub first_submission_date: Option<i64>,
    pub first_seen_itw_date: Option<i64>,
    pub tags: Option<Vec<String>>,
}

impl VTAttributes {
    pub fn new(json_data: &Value) -> Option<Self> {
        return serde_json::from_value(json_data.clone()).unwrap();
    }
}

#[derive(Debug)]
pub struct VTData {
    pub id: Option<String>,
    pub file_type: Option<String>,
    pub links: Option<HashMap<String, String>>,
    pub attributes: Option<VTAttributes>,
}

impl VTData {
    pub fn new(json_data: &Value) -> Option<Self> {
        return Some(Self {
            id: json_data["id"].as_str().map(str::to_string),
            file_type: json_data["type"].as_str().map(str::to_string),
            links: None,
            attributes: VTAttributes::new(&json_data["attributes"]),
        });
    }
}

#[derive(Debug)]
pub struct VTResponse {
    pub data: Option<VTData>,
    pub meta: Option<()>,
    pub links: Option<()>,
}