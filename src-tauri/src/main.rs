#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nucleus_rs::interfaces::filesystem::{File, ObjectId};
use tauri::Manager;
use std::sync::Mutex;

mod storage;

use crate::storage::*;

pub struct FileSystemInstances (Mutex<ProvidersMap>);

#[tauri::command]
fn read_file(provider_id: ProviderId, path: ObjectId, instance: tauri::State<FileSystemInstances>) -> String {
    let mutex = instance.0.lock().unwrap();
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().read_file(path);
    
        if result.is_ok() {
            return String::from_utf8(result.unwrap()).unwrap()
        }
    }

    String::from("Error reading file")
}

#[tauri::command]
fn write_file(provider_id: ProviderId, path: ObjectId, parent: ObjectId, content: Vec<u8>, instance: tauri::State<FileSystemInstances>) -> () {
    let mutex = instance.0.lock().unwrap();
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().write_file(path, content);

        if result.is_ok() {
            return ();
        }
    }
}

#[tauri::command]
fn list_folder_content(provider_id: ProviderId, path: ObjectId, instance: tauri::State<FileSystemInstances>) -> Vec<File> {
    let mutex = instance.0.lock().unwrap();

    match mutex.get_provider(provider_id) {
        Ok(provider) => {
            let result = provider.list_folder_content(path);

            if result.is_ok() {
                return result.unwrap()
            } else {
                dbg!(result.err());
            }
        },
        Err(provider_error) => {
            dbg!(provider_error);
        }
    }

    vec![]
}

#[tauri::command]
fn open(provider_id: ProviderId, path: ObjectId, instance: tauri::State<FileSystemInstances>) -> () {
    let mutex = instance.0.lock().unwrap();
    if let Ok(provider) = mutex.get_provider(provider_id.clone()) {
        if let Ok(metadata) = provider.get_metadata(path) {
            open::that(metadata.open_path);
        }
    }
}

#[tauri::command]
fn move_to(provider_id: ProviderId, path: ObjectId, new_provider_id: ProviderId, new_path: ObjectId, instance: tauri::State<FileSystemInstances>) -> () {
    let mutex = instance.0.lock().unwrap();
    if let Ok(provider) = mutex.get_provider(provider_id.clone()) {
        if provider_id == new_provider_id {
            provider.move_to(path, new_path);
        } else {
            if let Ok(new_provider) = mutex.get_provider(new_provider_id) {
                if let Ok(file) = provider.read_file(path) {
                    new_provider.write_file(new_path, file);
                }
            }
        }
    }
}

#[tauri::command]
fn rename(provider_id: ProviderId, path: ObjectId, new_name: String, instance: tauri::State<FileSystemInstances>) -> () {
    let mutex = instance.0.lock().unwrap();
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().rename(path, new_name);

        if result.is_ok() {
            return ();
        }
    }
}

#[tauri::command]
fn create(provider_id: ProviderId, path: ObjectId, file: File, instance: tauri::State<FileSystemInstances>) -> () {
    let mutex = instance.0.lock().unwrap();
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().create(path, file);

        if result.is_ok() {
            return ();
        }
    }
}

#[tauri::command]
fn delete(provider_id: ProviderId, path: ObjectId, instance: tauri::State<FileSystemInstances>) -> () {
    let mutex = instance.0.lock().unwrap();
    let x = mutex.get_provider(provider_id);

    if x.is_ok() {
        let result = x.unwrap().delete(path);

        if result.is_ok() {
            return ();
        }
    }
}

#[tauri::command]
fn add_provider(provider_id: ProviderId, credentials: String, instance: tauri::State<FileSystemInstances>) {
    let providers_map = instance.0.lock().unwrap().add_provider(provider_id, credentials);

    *instance.0.lock().unwrap() = providers_map;
}

#[tauri::command]
fn list_providers(instance: tauri::State<FileSystemInstances>) -> Vec<ProviderId> {
    instance.0.lock().unwrap().list_providers()
}

#[tauri::command]
fn remove_provider(provider_id: ProviderId, instance: tauri::State<FileSystemInstances>) {
    instance.0.lock().unwrap().remove_provider(provider_id);
}

fn main() {
    let mutex = Mutex::new(ProvidersMap::new());
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
