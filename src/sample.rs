use std::fmt;
use std::fs;
use std::io::Read;
use std::path::Path;

use openssl::hash::{Hasher, MessageDigest};

mod vt_report;
use vt_report::VtReport;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod local_report;
use local_report::LocalReport;


#[derive(Serialize, Deserialize)]
pub struct Sample {
    pub name: String,
    pub path: String,
    pub magic: String,
    #[serde(serialize_with = "as_hex", deserialize_with = "from_hex_16")]
    pub md5: [u8; 16],
    #[serde(serialize_with = "as_hex", deserialize_with = "from_hex_32")]
    pub sha256: [u8; 32],
    #[serde(serialize_with = "as_hex", deserialize_with = "from_hex_64")]
    pub sha512: [u8; 64],
    pub local_report: Option<LocalReport>,
    pub vt_report: Option<VtReport>,
}

fn as_hex<T, S>(key: &T, serializer: S) -> Result<S::Ok, S::Error> where T: AsRef<[u8]>, S: Serializer {
    return serializer.serialize_str(&hex::encode(key.as_ref()).as_str());
}

// TODO: Colapse bottom 3
fn from_hex_16<'de, D>(deserializer: D) -> Result<[u8; 16], D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    String::deserialize(deserializer)
    .and_then(|string: String| hex::decode(&string).map_err(|error| Error::custom(error.to_string())))
    .map(|bytes: Vec<u8>| bytes.try_into())
    .and_then(|op| op.map_err(|_| Error::custom("Couldn't parse hex string.")))
}

fn from_hex_32<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    String::deserialize(deserializer)
    .and_then(|string: String| hex::decode(&string).map_err(|error| Error::custom(error.to_string())))
    .map(|bytes: Vec<u8>| bytes.try_into())
    .and_then(|op| op.map_err(|_| Error::custom("Couldn't parse hex string.")))
}

fn from_hex_64<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    String::deserialize(deserializer)
    .and_then(|string: String| hex::decode(&string).map_err(|error| Error::custom(error.to_string())))
    .map(|bytes: Vec<u8>| bytes.try_into())
    .and_then(|op| op.map_err(|_| Error::custom("Couldn't parse hex string.")))
}

impl Sample {
    pub fn new(path: &str, vt_key: &Option<String>) -> Result<Self, String> {
        let mut sha512: Hasher = Hasher::new(MessageDigest::sha512()).expect("Couln't unit SHA512.");
        let mut sha256: Hasher = Hasher::new(MessageDigest::sha256()).expect("Couldn't init SHA256.");
        let mut md5: Hasher = Hasher::new(MessageDigest::md5()).expect("Couldn't init MD5");

        let file_path: &Path = Path::new(&path);
        let name: String = String::from(file_path.file_name().unwrap().to_str().unwrap());

        // Figure out the mime type of the sample.
        let magic: String = tree_magic::from_filepath(file_path);

        const BUFFER_SIZE: usize = 4096;
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        if let Ok(mut sample_file) = fs::File::open(&path) {
            while let Ok(size_read) = sample_file.read(&mut buffer) {
                if size_read == 0 { break; }

                let buffer_slice: &[u8] = &buffer[0..size_read];

                sha256.update(buffer_slice).unwrap();
                sha512.update(buffer_slice).unwrap();
                md5.update(buffer_slice).unwrap();
            }

            let md5digest:    [u8; 16] = md5.finish().unwrap().as_ref().try_into().unwrap();
            let sha256digest: [u8; 32] = sha256.finish().unwrap().as_ref().try_into().unwrap();
            let sha512digest: [u8; 64] = sha512.finish().unwrap().as_ref().try_into().unwrap();

            let mut new_sample: Self = Self {
                name,
                path: path.to_string(),
                magic,
                sha256: sha256digest,
                sha512: sha512digest,
                md5: md5digest,
                local_report: None,
                vt_report: None,
            };

            if let Some(key) = vt_key {
                new_sample.vt_report = VtReport::new(key, sha256digest);
            }

            return Ok(new_sample);
        } else {
            return Err("Unable to open sample !".to_string());
        }
    }
}

impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"Sample name: {}
Path: {}
File type: {}
Hashs:
    MD5:    {}
    SHA256: {}
    SHA512: {}",
            self.name,
            self.path,
            self.magic,
            hex::encode(self.md5),
            hex::encode(self.sha256),
            hex::encode(self.sha512),
        )?;
        
        if let Some(local_report) = &self.local_report {
            write!(f, "\n{}", local_report)?;
        }

        if let Some(vt_report) = &self.vt_report {
            write!(f, "\n{}", vt_report)?;
        }

        return fmt::Result::Ok(());
    }
}