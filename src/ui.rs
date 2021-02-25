use crate::client::Client;
use crate::server::Server;
use drive_client::types::Metadata;
use std::fs::{self};

use crate::meta_handler::MetaHandler;

pub struct Ui;

impl Ui {
    pub fn init() -> std::io::Result<()> {
        println!("Configuring current directory...");

        MetaHandler::init_dir().unwrap();

        Ok(())
    }

    pub fn share(file_names: Vec<String>) -> std::io::Result<()> {
        let metadata = MetaHandler::get_metadata().unwrap();
        let mut client = Client::new(metadata);

        client.pick_files(file_names);
        client.send();

        Ok(())
    }

    pub fn update() -> std::io::Result<()> {
        MetaHandler::update().unwrap();
        Ok(())
    }

    pub fn server() -> std::io::Result<()> {
        Server::listen().unwrap();
        Ok(())
    }
}

