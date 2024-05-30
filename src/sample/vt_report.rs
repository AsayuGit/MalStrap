use core::fmt;

use serde::{Deserialize, Serialize};

use chrono::DateTime;
use chrono::Utc;

mod vt_connect;
use vt_connect::VTClient;
use vt_connect::file::reports::FileRelation;

#[derive(Serialize, Deserialize, Debug)]
pub enum AnalysisVerdict {
    UNDECTED,
    HARMLESS,
    SUSPICIOUS,
    MALICIOUS,
}

#[derive(Serialize, Deserialize)]
pub struct Analysis {
    verdict: AnalysisVerdict,
    confidence: f32,
}

#[derive(Serialize, Deserialize)]
pub struct VtReport {
    pub alternative_names: Option<Vec<String>>,
    pub first_seen_date: Option<i64>,
    pub last_analysis_date: Option<i64>,
    pub last_analysis: Option<Analysis>,
    pub reputation: Option<i64>,

    pub contacted_domains: Option<Vec<String>>, // TODO: Reputation
    pub contacted_ips: Option<Vec<String>>,
    pub dropped_files: Option<Vec<String>>,
}

impl VtReport {
    fn step(stat: Option<u64>, verdict_value: AnalysisVerdict, total: &mut u64, max_value: &mut u64, verdict: &mut AnalysisVerdict) {
        if let Some(value) = stat {
            *total += value;
            if value > *max_value {
                *max_value = value;
                *verdict = verdict_value;
            }
        }
    }

    pub fn new(vt_key: &str,  hash: [u8; 32]) -> Option<Self> {
        let vt: VTClient = VTClient::new(vt_key);

        let mut alternative_names: Option<Vec<String>> = None;
        let mut first_seen_date: Option<i64> = None;
        let mut last_analysis_date: Option<i64> = None;
        let mut last_analysis: Option<Analysis> = None;
        let mut reputation: Option<i64> = None;
        let mut contacted_domains: Option<Vec<String>> = None;
        let mut contacted_ips: Option<Vec<String>> = None;
        let mut dropped_files: Option<Vec<String>> = None;

        let hash_str: String = hex::encode(hash);
        if let Ok(file_info) = vt.file_info(&hash_str) {
            if let Ok(file_contacted_domains) = vt.file_relation(&hash_str, FileRelation::ContactedDomains) {
                // TODO: Could be condenced using macros ?
                if let Some(data) = file_contacted_domains.data {
                    let mut domains: Vec<String> = Vec::new();
                    for entry in data {
                        if let Some(id) = entry.id {
                            domains.push(id);
                        }
                    }
                    contacted_domains = Some(domains);
                }
            }

            if let Ok(file_contacted_ips) = vt.file_relation(&hash_str, FileRelation::ContactedIps) {
                if let Some(data) = file_contacted_ips.data {
                    let mut ips: Vec<String> = Vec::new();
                    for entry in data {
                        if let Some(id) = entry.id {
                            ips.push(id);
                        }
                    }
                    contacted_ips = Some(ips);
                }
            }

            if let Ok(file_dropped_files) = vt.file_relation(&hash_str, FileRelation::DroppedFiles) {
                if let Some(data) = file_dropped_files.data {
                    let mut files: Vec<String> = Vec::new();
                    for entry in data {
                        if let Some(name) = entry.attributes
                            .and_then(|attributes| attributes.names)
                            .and_then(|names| names.get(0).map(|name| name.clone()))
                        {
                            files.push(name);
                        } else if let Some(id) = entry.id {
                            files.push(id);
                        }
                    }
                    dropped_files = Some(files);
                }
            }

            if let Some(data) = file_info.data {
                if let Some(attributes) = data.attributes {
                    alternative_names = attributes.names;
                    first_seen_date = attributes.first_seen_itw_date;
                    last_analysis_date = attributes.last_analysis_date;
                    reputation = attributes.reputation;

                    if let Some(stats) = attributes.last_analysis_stats {
                        let mut total: u64 = 0;
                        let mut max_value: u64 = 0;
                        let mut verdict: AnalysisVerdict = AnalysisVerdict::UNDECTED;

                        VtReport::step(stats.harmless, AnalysisVerdict::HARMLESS, &mut total, &mut max_value, &mut verdict);
                        VtReport::step(stats.malicious, AnalysisVerdict::MALICIOUS, &mut total, &mut max_value, &mut verdict);
                        VtReport::step(stats.suspicious, AnalysisVerdict::SUSPICIOUS, &mut total, &mut max_value, &mut verdict);
                        //VtReport::step(stats.timeout, AnalysisVerdict::UNDECTED, &mut total, &mut max_value, &mut verdict);
                        //VtReport::step(stats.type_unsupported, AnalysisVerdict::UNDECTED, &mut total, &mut max_value, &mut verdict);
                        VtReport::step(stats.undetected, AnalysisVerdict::UNDECTED, &mut total, &mut max_value, &mut verdict);

                        last_analysis = Some(Analysis {
                            verdict,
                            confidence: (max_value as f32) / (total as f32),
                        });
                    }
                }
            }
        }

        return Some(Self {
            alternative_names,
            first_seen_date,
            last_analysis_date,
            last_analysis,
            reputation,
            contacted_domains,
            contacted_ips,
            dropped_files,
        });
    }
}

impl fmt::Display for VtReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Virus-Total Report:")?;

        if let Some(alternative_names) = &self.alternative_names {
            if !alternative_names.is_empty() {
                write!(f, "\n    Alternative names:")?;
                for name in alternative_names {
                    write!(f, "\n        - {}", name)?;
                }
            }
        }

        if let Some(first_seen_date) = &self.first_seen_date {
            let time: DateTime<Utc> = DateTime::from_timestamp(*first_seen_date, 0).unwrap();
            write!(f, "\n    First seen at: {}", time)?;
        }

        if let Some(last_analysis_date) = &self.last_analysis_date {
            let time: DateTime<Utc> = DateTime::from_timestamp(*last_analysis_date, 0).unwrap();
            write!(f, "\n    Last analysed: {}", time)?;
        }

        if let Some(reputation) = &self.reputation {
            write!(f, "\n    Reputation: {}", reputation)?;
        }

        if let Some(last_analysis) = &self.last_analysis {
            writeln!(f, "\n    Analysis:")?;
            writeln!(f, "        Verdict: {:?}", last_analysis.verdict)?;
            writeln!(f, "        Confidence: {:3} %", (100.0 * last_analysis.confidence) as u8)?;
        }

        if let Some(related_domains) = &self.contacted_domains {
            if !related_domains.is_empty() {
                write!(f, "\n    Contacted domains:")?;
                for domain in related_domains {
                    write!(f, "\n        {}", domain)?;
                } writeln!(f)?;
            }
        }

        if let Some(related_ips) = &self.contacted_ips {
            if !related_ips.is_empty() {
                write!(f, "\n    Contacted ips:")?;
                for ip in related_ips {
                    write!(f, "\n        {}", ip)?;
                } writeln!(f)?;
            }
        }

        if let Some(dropped_files) = &self.dropped_files {
            if !dropped_files.is_empty() {
                write!(f, "\n    Dropped files:")?;
                for ip in dropped_files {
                    write!(f, "\n        {}", ip)?;
                } writeln!(f)?;
            }
        }

        return Ok(());
    }
}