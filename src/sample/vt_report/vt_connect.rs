pub mod file;
pub mod responses;

pub struct VTClient {
    api_key: String,
}

impl VTClient {
    pub fn new(api_key: &str) -> Self {
        return Self {
            api_key: api_key.to_string(),
        };
    }
}