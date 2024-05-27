use core::fmt;

use serde::{Deserialize, Serialize};
use vt3::VtClient;

use chrono::DateTime;
use chrono::Utc;

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

    // Detected by which antivirus
    // Undected by which antivirus

    // Any related doamains ?
    // Any related ip ?
    // Any related files ?
}

impl VtReport {
    fn step(stat: Option<i64>, verdict_value: AnalysisVerdict, total: &mut i64, max_value: &mut i64, verdict: &mut AnalysisVerdict) {
        if let Some(value) = stat {
            *total += value;
            if value > *max_value {
                *max_value = value;
                *verdict = verdict_value;
            }
        }
    }

    pub fn new(vt_key: &str,  hash: [u8; 32]) -> Option<Self> {
        let vt: VtClient = VtClient::new(vt_key);

        let mut alternative_names: Option<Vec<String>> = None;
        let mut first_seen_date: Option<i64> = None;
        let mut last_analysis_date: Option<i64> = None;
        let mut last_analysis: Option<Analysis> = None;

        if let Ok(file_info) = vt.file_info(hex::encode(hash).as_str()) {
            if let Some(data) = file_info.data {
                if let Some(attributes) = data.attributes {
                    alternative_names = attributes.names;
                    first_seen_date = attributes.first_seen_itw_date;
                    last_analysis_date = attributes.last_analysis_date;

                    // TODO: Move away in implementation
                    if let Some(stats) = attributes.last_analysis_stats {
                        let mut total: i64 = 0;
                        let mut max_value: i64 = 0;
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
        });
    }
}

impl fmt::Display for VtReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Virus-Total Report:")?;

        if let Some(alternative_names) = &self.alternative_names {
            write!(f, "\n    Alternative names :")?;
            for name in alternative_names {
                write!(f, "\n        - {}", name)?;
            }
        }

        if let Some(first_seen_date) = &self.first_seen_date {
            let time: DateTime<Utc> = DateTime::from_timestamp(*first_seen_date, 0).unwrap();
            write!(f, "\n    First seen at : {}", time)?;
        }

        if let Some(last_analysis_date) = &self.last_analysis_date {
            let time: DateTime<Utc> = DateTime::from_timestamp(*last_analysis_date, 0).unwrap();
            write!(f, "\n    Last analysed : {}", time)?;
        }

        if let Some(last_analysis) = &self.last_analysis {
            writeln!(f, "\n    Analysis:")?;
            writeln!(f, "        Verdict: {:?}", last_analysis.verdict)?;
            writeln!(f, "        Confidence: {:3} %", (100.0 * last_analysis.confidence) as u8)?;
        }

        return Ok(());
    }
}