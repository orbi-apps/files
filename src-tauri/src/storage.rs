use nucleus_rs::interfaces::filesystem::{FileSystem, File, ObjectId};
use nucleus_rs::providers::google_drive::Token;
use nucleus_rs::providers::one_drive::OneDrive;
use nucleus_rs::providers::{s3::S3, google_drive::GoogleDrive, native_fs::NativeFs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
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
}

pub struct ProvidersMap (HashMap<ProviderId, Box<dyn FileSystem + Send + Sync>>);

impl ProvidersMap {
    pub async fn new() -> ProvidersMap {
        let storage = NativeFs { root : "".to_string() };

        let mut providers: HashMap<ProviderId, Box<dyn FileSystem + Send + Sync>> = HashMap::new();

        if let Some(proj_dirs) = ProjectDirs::from("", "Orbital", "Files") {
            let data_dir = (proj_dirs.data_dir().to_string_lossy() + "/").to_string();
            let x = &data_dir.clone();
            if !std::path::Path::new(data_dir.as_str()).exists() {
                fs::create_dir_all(data_dir.clone()).expect(format!("Unable to create directory {}", data_dir).as_str());
            }
            let files = storage.list_folder_content(ObjectId::directory(data_dir.clone())).await.unwrap();

            for file in files {
                let path = x.clone() + "/" + file.name.as_str();
                let content = storage.read_file(ObjectId::plain_text(path.clone())).await.unwrap();
                let file_name_split: Vec<&str> = file.name.splitn(2, ".").collect();

                let provider = ProviderId {
                    id: file_name_split[0].to_string(),
                    provider_type: file_name_split[1].to_string(),
                };

                let content_string = String::from_utf8(content).unwrap();
    
                match file_name_split[1] {
                    "S3" => {
                        let credentials: S3 = serde_json::from_str(content_string.as_str()).unwrap();
                        providers.insert(provider, Box::new(credentials));
                    },
                    "Google" => {
                        let tokens: HashMap<String, Token> = serde_json::from_str(content_string.as_str()).unwrap();
                        let google_drive = GoogleDrive::new(env!("GOOGLE_DRIVE_CLIENT_KEY").to_string(), tokens).await.unwrap();
                        providers.insert(provider, Box::new(google_drive));
                    },
                    "OneDrive" => {
                        let credentials: OneDrive = serde_json::from_str(content_string.as_str()).unwrap();
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
                id: "My local files".to_string(),
                provider_type: "native_fs".to_string(),
            };

            providers.insert(provider, Box::new(native_fs));
        }

        ProvidersMap(providers)
    }

    pub fn get_provider<'a>(&self, provider: ProviderId) -> Result<&Box<dyn FileSystem + Send + Sync>, String> {
        match self.0.get(&provider) {
            Some(x) => Ok(x),
            None => Err(String::from("Provider not found"))
        }
    }

    pub async fn add_provider(&mut self, provider_id: ProviderId, provider_infos: serde_json::Value) -> Result<(), ()> {
        let provider: (Box<dyn FileSystem + Send + Sync>, String) = match provider_id.provider_type.as_str() {
            "Google" => {
                let google_drive = GoogleDrive::new(env!("GOOGLE_DRIVE_CLIENT_KEY").to_string(), HashMap::new()).await.unwrap();
                google_drive.list_folder_content(ObjectId::directory("".to_string())).await.unwrap();
                let serialized = serde_json::to_string(&google_drive.tokens_map()).unwrap();

                (Box::new(google_drive), serialized)
            },
            "OneDrive" => {
                let mut onedrive = OneDrive::new(None, env!("ONEDRIVE_CLIENT_ID").to_string());
                onedrive.fetch_credentials().unwrap();
                let serialized = serde_json::to_string(&onedrive).unwrap();

                (Box::new(onedrive), serialized)
            },
            "S3" => {
                
                let s3 = S3::new(
                    provider_infos.get("bucket").unwrap().to_string(),
                    provider_infos.get("region").unwrap().to_string(),
                    provider_infos.get("endpoint").unwrap().to_string(),
                    provider_infos.get("access_key").unwrap().to_string(),
                    provider_infos.get("secret_key").unwrap().to_string());
                let serialized = serde_json::to_string(&s3).unwrap();

                (Box::new(s3), serialized)
            },
            _ => { return Err(()) }
        };

        self.0.insert(provider_id.clone(), provider.0);

        let storage = NativeFs { root : "".to_string() };
        if let Some(proj_dirs) = ProjectDirs::from("", "Orbital", "Files") {
            let path = (proj_dirs.data_dir().to_string_lossy() + "/").to_string();
    
            let file_name = provider_id.id + "." + provider_id.provider_type.as_str();
    
            let file: File = File {
                id: path.clone() + file_name.as_str(),
                name: file_name.clone(),
                mime_type: Some("text/plain".to_string())
            };
    
            storage.create(ObjectId::plain_text(path.clone()), file.clone()).await.expect(format!("Unable to create provider {}", path.clone() + file_name.as_str()).as_str());
    
            storage.write_file(ObjectId::plain_text(file.id), provider.1.as_bytes().to_vec()).await.expect("Unable to write new provider to storage");
        }

        Ok(())
    }

    pub async fn remove_provider(&self, provider_id: ProviderId) -> ProvidersMap {
        if let Some(proj_dirs) = ProjectDirs::from("", "Orbital", "Files") {
            let storage = NativeFs { root : "".to_string() };
            let path = (proj_dirs.data_dir().to_string_lossy() + "/").to_string();
    
            storage.delete(ObjectId::plain_text(path + provider_id.id.as_str() + "." + provider_id.provider_type.as_str())).await.expect("Unable to remove provider.");
        }

        ProvidersMap::new().await
    }

    pub fn list_providers(&self) -> Vec<ProviderId> {
        let mut providers_vec: Vec<ProviderId> = Vec::new();
        
        for provider in self.0.keys() {
            providers_vec.push((*provider).clone());
        }

        providers_vec
    }
}