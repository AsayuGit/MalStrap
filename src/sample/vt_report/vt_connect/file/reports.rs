use super::super::VTClient;

pub enum FileRelation {
    Behaviors,
    BundledFiles,
    Collections,
    Comments,
    ContactedDomains,
    ContactedIps,
    ContactedUrls,
    DroppedFiles,
    ExectionParents,
    Graphs,
    PeRessourceChildren,
    PeRessourceParents,
    Votes,
}

impl VTClient {
    pub fn file_relation(&self, hash_str: &str, relation: FileRelation) {

    }
}