use reqwest::blocking::{Client, RequestBuilder};

use super::super::responses::VTRelationResponse;
use super::super::VTClient;

pub enum FileRelation {
    Behaviours,
    BundledFiles,
    Collections,
    Comments,
    ContactedDomains,
    ContactedIps,
    ContactedUrls,
    DroppedFiles,
    ExecutionParents,
    Graphs,
    PeResourceChildren,
    PeResourceParents,
    Votes,
}

impl FileRelation {
    fn as_str(&self) -> &'static str {
        return match self {
            Self::Behaviours => "behaviours",
            Self::BundledFiles => "bundled_files",
            Self::Collections => "collections",
            Self::ContactedDomains => "contacted_domains",
            Self::ContactedIps => "contacted_ips",
            Self::ContactedUrls => "contacted_urls",
            Self::ExecutionParents => "execution_parents",
            Self::Graphs => "graphs",
            Self::PeResourceChildren => "pe_resource_children",
            Self::PeResourceParents => "pe_resource_parents",
            Self::Votes => "votes",
            _ => "none",
        };
    }
}

impl VTClient {
    pub fn file_relation(&self, hash_str: &str, relation: FileRelation) -> Result<VTRelationResponse, ()> {
        let request_client: Client = Client::new();
        let request_builder: RequestBuilder = request_client.get(format!("https://www.virustotal.com/api/v3/files/{}/{}", hash_str, relation.as_str()))
            .header("x-apikey", &self.api_key);

        if let Ok(response) = request_builder.send() {
            if let Ok(text) = response.text() {
                if let Ok(vt_response) = serde_json::from_str::<VTRelationResponse>(text.as_str()) {
                    return Ok(vt_response);
                }
            }
        }

        return Err(());
    }
}