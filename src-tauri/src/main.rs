#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nucleus_rs::interfaces::filesystem::{File, ObjectId};
use serde_json::Value;
use tauri::Manager;
use tauri::async_runtime::{Mutex, block_on};

mod storage;

use crate::storage::*;

pub struct FileSystemInstances (Mutex<ProvidersMap>);

#[tauri::command]
async fn read_file(provider_id: ProviderId, path: ObjectId, instance: tauri::State<'_, FileSystemInstances>) -> Result<String, String> {
    let mutex = instance.0.lock().await;
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().read_file(path.clone()).await;
    
        if result.is_ok() {
            return Ok(String::from_utf8(result.unwrap()).unwrap())
        }
    }

    Err(String::from("Error reading ") + path.as_str())
}

#[tauri::command]
async fn write_file(provider_id: ProviderId, path: ObjectId, parent: ObjectId, content: Vec<u8>, instance: tauri::State<'_, FileSystemInstances>) -> Result<(), String> {
    let mutex = instance.0.lock().await;
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().write_file(path, content).await;

        if result.is_err() {
            return Err(result.unwrap_err().to_string())
        }
    }

    Ok(())
}

#[tauri::command]
async fn list_folder_content(provider_id: ProviderId, path: ObjectId, instance: tauri::State<'_, FileSystemInstances>) -> Result<Vec<File>, String> {
    let mutex = instance.0.lock().await;

    match mutex.get_provider(provider_id) {
        Ok(provider) => {
            let result = provider.list_folder_content(path).await;

            if result.is_ok() {
                return Ok(result.unwrap())
            } else {
                return Err(result.unwrap_err().to_string())
            }
        },
        Err(provider_error) => {
            dbg!(provider_error);
        }
    }

    Ok(vec![])
}

#[tauri::command]
async fn open(provider_id: ProviderId, path: ObjectId, instance: tauri::State<'_, FileSystemInstances>) -> Result<(), String> {
    let mutex = instance.0.lock().await;
    if let Ok(provider) = mutex.get_provider(provider_id.clone()) {
        if let Ok(metadata) = provider.get_metadata(path).await {
            open::that(metadata.open_path).expect("Unable to open file with default application");
        }
    }

    Ok(())
}

#[tauri::command]
async fn move_to(provider_id: ProviderId, path: ObjectId, new_provider_id: ProviderId, new_path: ObjectId, instance: tauri::State<'_, FileSystemInstances>) -> Result<(), String> {
    let mutex = instance.0.lock().await;
    if let Ok(provider) = mutex.get_provider(provider_id.clone()) {
        if provider_id == new_provider_id {
            provider.move_to(path, new_path).await.unwrap();
        } else {
            if let Ok(new_provider) = mutex.get_provider(new_provider_id) {
                let file = provider.read_file(path).await.unwrap();
                new_provider.write_file(new_path, file).await.unwrap();
            }
        }
    }

    Ok(())
}

#[tauri::command]
async fn rename(provider_id: ProviderId, path: ObjectId, new_name: String, instance: tauri::State<'_, FileSystemInstances>) -> Result<(), String> {
    let mutex = instance.0.lock().await;
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().rename(path, new_name).await;

        if result.is_err() {
            return Err(result.unwrap_err().to_string())
        }
    }

    Ok(())
}

#[tauri::command]
async fn create(provider_id: ProviderId, path: ObjectId, file: File, instance: tauri::State<'_, FileSystemInstances>) -> Result<(), String> {
    let mutex = instance.0.lock().await;
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().create(path, file).await;

        if result.is_err() {
            return Err(result.unwrap_err().to_string())
        }
    }

    Ok(())
}

#[tauri::command]
async fn delete(provider_id: ProviderId, path: ObjectId, instance: tauri::State<'_, FileSystemInstances>) -> Result<(), String> {
    let mutex = instance.0.lock().await;
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().delete(path).await;
        if result.is_err() {
            return Err(result.unwrap_err().to_string());
        }
    }

    Ok(())
}

#[tauri::command]
async fn add_provider(provider_id: ProviderId, credentials: Value, instance: tauri::State<'_, FileSystemInstances>) -> Result<(), ()> {
    let mut providers_map = instance.0.lock().await;
    providers_map.add_provider(provider_id, credentials).await.unwrap();

    Ok(())
}

#[tauri::command]
async fn list_providers(instance: tauri::State<'_, FileSystemInstances>) -> Result<Vec<ProviderId>, ()> {
    Ok(instance.0.lock().await.list_providers())
}

#[tauri::command]
async fn remove_provider(provider_id: ProviderId, instance: tauri::State<'_, FileSystemInstances>) -> Result<(), ()> {
    instance.0.lock().await.remove_provider(provider_id).await;
    Ok(())
}

fn main() {
    let providers_map = block_on(ProvidersMap::new());
    let mutex = Mutex::new(providers_map);
    let instance = FileSystemInstances (mutex);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(instance)
        .invoke_handler(tauri::generate_handler![read_file, list_folder_content, add_provider, list_providers, remove_provider, write_file, open, rename, create, delete, move_to])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
