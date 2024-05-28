use reqwest::blocking::{Client, RequestBuilder};
use serde_json::Value;

use crate::sample::vt_report::vt_connect::responses::VTData;

use super::super::responses::VTResponse;
use super::super::VTClient;

impl VTClient {
    pub fn file_info(&self, hash_str: &str) -> Result<VTResponse, ()> {
        let request_client: Client = Client::new();
        let request_builder: RequestBuilder = request_client.get(format!("https://www.virustotal.com/api/v3/files/{}", hash_str))
            .header("x-apikey", &self.api_key);

        // Send request to server
        if let Ok(response) = request_builder.send() {
            // Fetch the response's body
            if let Ok(text) = response.text() {
                // Parse JSON
                if let Ok(json_data) = serde_json::from_str::<Value>(text.as_str()) {
                    // Use JSON to fill struct
                    let vt_response: VTResponse = VTResponse {
                        data: VTData::new(&json_data["data"]),
                        meta: None,
                        links: None,
                    };

                    return Ok(vt_response);
                }
            }
        }

        return Err(());
    }
}