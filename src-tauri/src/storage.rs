use nucleus_rs::interfaces::filesystem::{FileSystem, File, ObjectId};
use nucleus_rs::providers::{s3::S3, google_drive::GoogleDrive, native_fs::NativeFs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use serde_json::json;
use directories::{ProjectDirs, UserDirs};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
struct ArbitraryData(serde_json::Value);

impl Hash for ArbitraryData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state);
    }
}


#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct ProviderId {
    pub id: String,
    pub provider_type: String,
    data: ArbitraryData
}

pub struct ProvidersMap (HashMap<ProviderId, Box<dyn FileSystem + Send>>);

impl ProvidersMap {
    pub fn new() -> ProvidersMap {
        let storage = NativeFs { root : "".to_string() };

        let mut providers: HashMap<ProviderId, Box<dyn FileSystem + Send>> = HashMap::new();

        if let Some(proj_dirs) = ProjectDirs::from("", "Orbital", "Files") {
            let data_dir = (proj_dirs.data_dir().to_string_lossy() + "/").to_string();
            let x = data_dir.clone();
            let files = match storage.list_folder_content(ObjectId::directory(data_dir.clone())) {
                Ok(val) => val,
                Err(err) => {
                    fs::create_dir_all(data_dir);
                    storage.list_folder_content(ObjectId::directory(x.clone())).unwrap()
                }
            };

            for file in files {
                let path = x.clone() + "/" + file.name.as_str();
                let content = storage.read_file(ObjectId::plain_text(path.clone())).unwrap();
                let file_name_split: Vec<&str> = file.name.splitn(2, ".").collect();
    
                match file_name_split[1] {
                    "S3" => {
                        let credentials: S3 = serde_json::from_str(String::from_utf8(content).unwrap().as_str()).unwrap();
    
                        let provider = ProviderId {
                            id: file_name_split[0].to_string(),
                            provider_type: file_name_split[1].to_string(),
                            data: ArbitraryData(json!({
                                "bucket": credentials.bucket,
                                "endpoint": credentials.credentials.endpoint
                            }))
                        };
        
                        providers.insert(provider, Box::new(credentials));
                    },
                    "Google" => {
                        let credentials: GoogleDrive = serde_json::from_str(String::from_utf8(content).unwrap().as_str()).unwrap();
    
                        let provider = ProviderId {
                            id: file_name_split[0].to_string(),
                            provider_type: file_name_split[1].to_string(),
                            data: ArbitraryData(json!({"scope": "Drive"}))
                        };
    
                        providers.insert(provider, Box::new(credentials));
                    },
                    _ => ()
                }
                
            }
        }

        if let Some(user_dirs) = UserDirs::new() {
            let home_path = (user_dirs.home_dir().to_string_lossy() + "/").to_string();
            let native_fs = NativeFs { root: home_path.clone() };
            let provider = ProviderId {
                id: "native".to_string(),
                provider_type: "native_fs".to_string(),
                data: ArbitraryData(json!("{}"))
            };

            providers.insert(provider, Box::new(native_fs));
        }

        ProvidersMap(providers)
    }

    pub fn get_provider(&self, provider: ProviderId) -> Result<&Box<dyn FileSystem + Send>, String> {
        match self.0.get(&provider) {
            Some(x) => Ok(x),
            None => Err(String::from("Provider not found"))
        }
    }

    pub fn add_provider(&self, provider_id: ProviderId, provider_infos: String) -> ProvidersMap {
        let storage = NativeFs { root : "".to_string() };
        if let Some(proj_dirs) = ProjectDirs::from("", "Orbital", "Files") {
            let path = (proj_dirs.data_dir().to_string_lossy() + "/").to_string();
    
            let file_name = provider_id.id + "." + provider_id.provider_type.as_str();
    
            let file: File = File {
                id: path.clone() + file_name.as_str(),
                name: file_name,
                mime_type: "text/plain".to_string()
            };
    
            storage.create(ObjectId::plain_text(path), file.clone());
    
            storage.write_file(ObjectId::plain_text(file.id), provider_infos.as_bytes().to_vec()).expect("Unable to write new provider to storage");
    
        }
        ProvidersMap::new()
    }

    pub fn remove_provider(&self, provider_id: ProviderId) -> ProvidersMap {
        if let Some(proj_dirs) = ProjectDirs::from("", "Orbital", "Files") {
            let storage = NativeFs { root : "".to_string() };
            let path = (proj_dirs.data_dir().to_string_lossy() + "/").to_string();
    
            storage.delete(ObjectId::plain_text(path + provider_id.id.as_str() + "." + provider_id.provider_type.as_str())).expect("Unable to remove provider.");
        }

        ProvidersMap::new()
    }

    pub fn list_providers(&self) -> Vec<ProviderId> {
        let mut providers_vec: Vec<ProviderId> = Vec::new();
        
        for provider in self.0.keys() {
            providers_vec.push((*provider).clone());
        }

        providers_vec
    }
}