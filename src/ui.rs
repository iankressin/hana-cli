use crate::client::Client;
use crate::server::Server;

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

        match client.pick_files(file_names) {
            Ok(_) => {
                println!("Starting to share files ... ");
                client.send();
            }
            Err(err) => {
                println!("{}", err)
            }
        }

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
