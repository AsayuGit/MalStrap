use reqwest::blocking::{Client, RequestBuilder};

use super::super::responses::VTInfoResponse;
use super::super::VTClient;

impl VTClient {
    pub fn file_info(&self, hash_str: &str) -> Result<VTInfoResponse, ()> {
        let request_client: Client = Client::new();
        let request_builder: RequestBuilder = request_client.get(format!("https://www.virustotal.com/api/v3/files/{}", hash_str))
            .header("x-apikey", &self.api_key);

        // Send request to server
        if let Ok(response) = request_builder.send() {
            // Fetch the response's body
            if let Ok(text) = response.text() {
                // Parse JSON
                if let Ok(vt_response) = serde_json::from_str::<VTInfoResponse>(text.as_str()) {
                    return Ok(vt_response);
                }
            }
        }

        return Err(());
    }
}
