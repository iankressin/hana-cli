use drive_server::drive_server::DriveServer;
use drive_server::types::Metadata;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Server;

impl Server {
    fn read_metadata() -> Vec<Metadata> {
        let bytes = fs::read("./.drive/metadata.json").unwrap();
        let json = String::from_utf8_lossy(&bytes);
        let metadata: Vec<Metadata> = serde_json::from_str(&json).unwrap();

        metadata
    }

    fn push_metadata(meta: Metadata) -> Result<(), std::io::Error> {
        let mut metadata = Server::read_metadata();
        metadata.push(meta.clone());

        let json = serde_json::to_string(&metadata).unwrap();
        fs::write("./.drive/metadata.json", &json).unwrap();

        Ok(())
    }

    pub fn listen() -> std::io::Result<()> {
        let metadata = Server::read_metadata();

        let t = thread::spawn(|| {
            // Source of truth
            let lock = Arc::new(RwLock::new(metadata));

            let c_lock = Arc::clone(&lock);

            let (tx, rx) = channel();

            thread::spawn(move || {
                for received in rx {
                    let mut meta = lock.write().unwrap();
                    println!("File received: {:?}", received);
                    Server::push_metadata(received).unwrap();
                    // meta.push(received);
                }
            });

            DriveServer::listen(&c_lock, tx).unwrap();
        });

        t.join().unwrap();

        Ok(())
    }
}
