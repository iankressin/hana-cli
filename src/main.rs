// use std::thread;

// use drive_client::drive_client::DriveClient;
// use drive_client::types::Metadata as ClientMeta;
// use drive_server::drive_server::DriveServer;
// use drive_server::types::Metadata as ServerMeta;
// use std::sync::mpsc::channel;
// use std::sync::Arc;
// use std::sync::RwLock;

use std::env;

mod client;
mod server;
mod ui;
mod types;


fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1].as_str();

    match command {
         &"init" => {
            println!("Configuring folder...");
            ui::Ui::init()
                .expect("Could not initialize the folder");
        }
        
         &"share" => {
            println!("Starting to share ...");
            ui::Ui::share(args[1..].to_vec())
                .expect("Something went wrong while sending the files");
         }

         &"server" => {

         }

         _ => println!("Wrong command")
    }
}

// fn main() {
//     thread::spawn(|| {
//         // Source of truth
//         let lock = Arc::new(RwLock::new(vec![ServerMeta {
//             name: "fuji".to_string(),
//             extension: "jpg".to_string(),
//             name_extension: "fuji.jpg".to_string(),
//             hash: "b0e490e762234567eabc74fade854476fe692e320".to_string(),
//             size: 124093,
//         }]));

//         let c_lock = Arc::clone(&lock);

//         let (tx, rx) = channel();

//         thread::spawn(move || {
//             for received in rx {
//                 let mut meta = lock.write().unwrap();
//                 println!("File received: {:?}", received);
//                 meta.push(received);
//             }
//         });

//         DriveServer::listen(&c_lock, tx).unwrap();
//     });

//     thread::spawn(|| {
//         let c_meta = vec![ClientMeta {
//             name: "fuji".to_string(),
//             extension: "jpg".to_string(),
//             name_extension: "fuji.jpg".to_string(),
//             hash: "b0e490e762234567eabc74fade854476fe692e320".to_string(),
//             size: 124093,
//         }];

//         DriveClient::send(c_meta);
//     });
// }
