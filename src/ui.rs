use crate::client::Client;
use crate::server::Server;
use drive_client::types::Metadata;
use regex::Regex;
use sha1::{Digest, Sha1};
use std::fs::{self};
use std::io;
use std::io::prelude::*;

pub struct Ui;

impl Ui {
    pub fn init() -> std::io::Result<()> {
        println!("Configuring current directory...");

        fs::create_dir("./.drive")?;

        let mut file = fs::File::create("./.drive/metadata.json")?;
        let json = serde_json::to_string(&get_folder_metada().unwrap())?;

        file.write_all(&json.as_bytes())?;

        Ok(())
    }

    pub fn share(file_names: Vec<String>) -> std::io::Result<()> {
        let bytes = fs::read(&"./.drive/metadata.json").unwrap();
        let json = String::from_utf8_lossy(&bytes);
        let metadata: Vec<Metadata> = serde_json::from_str(&json)?;
        let mut client = Client::new(metadata);

        client.pick_files(file_names);
        client.send();

        Ok(())
    }

    pub fn server() -> std::io::Result<()> {
        Server::listen().unwrap();
        Ok(())
    }
}

// TODO: Move somewhere else
fn hash_files(path: &str, buf: &mut [u8]) {
    let mut file = fs::File::open(&path).unwrap();
    let mut hasher = Sha1::new();
    let n = io::copy(&mut file, &mut hasher).unwrap();

    buf.copy_from_slice(&hasher.finalize())
}
// TODO: Fix known proble with filenames that contains multiple dots
// Eg: index.spec.js
fn get_file_name_and_extension(file: &String) -> (String, String) {
    // Looks for a dot at the begining or no dot at all
    let re = Regex::new(r"^\.|^[^.]*$").unwrap();

    if re.is_match(file) {
        (file.to_owned(), String::from(""))
    } else {
        let words = file.split(".").collect::<Vec<&str>>();
        let name = words[..words.len() - 1]
            .into_iter()
            .map(|i| i.to_string())
            .collect();
        let extension = words.last().unwrap().to_string();

        (name, extension)
    }
}

// TODO: Ugly code as hell
fn get_folder_metada() -> std::io::Result<Vec<Metadata>> {
    let mut meta = Vec::new();
    for entry in fs::read_dir("./")? {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                let (name, extension) =
                    get_file_name_and_extension(&entry.file_name().to_str().unwrap().to_string());

                let name_extension = {
                    if extension == "" {
                        name.clone()
                    } else {
                        format!("{}.{}", &name, &extension)
                    }
                };

                if !metadata.is_dir() {
                    let mut buf = [0u8; 20];
                    println!("File entry ==>>> {:?}", entry);
                    hash_files(&entry.path().to_str().unwrap(), &mut buf);
                    meta.push(Metadata {
                        name_extension,
                        name,
                        extension,
                        size: metadata.len() as u32,
                        hash: hex::encode(buf),
                    })
                }
            }
        }
    }

    Ok(meta)
}

#[test]
fn split_file_name() {
    let name_extension = get_file_name_and_extension(&"test.png".to_string());
    assert_eq!(("test".to_string(), "png".to_string()), name_extension);

    let name_extension = get_file_name_and_extension(&".gitignore".to_string());
    assert_eq!((".gitignore".to_string(), "".to_string()), name_extension);

    let name_extension = get_file_name_and_extension(&"metadata".to_string());
    assert_eq!(("metadata".to_string(), "".to_string()), name_extension);
}
