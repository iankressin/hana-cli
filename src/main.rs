use std::env;

mod client;
mod meta_handler;
mod server;
mod types;
mod ui;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1].as_str();

    match command {
        &"init" => {
            println!("Configuring folder...");
            ui::Ui::init().expect("Could not initialize the folder");
        }

        &"share" => {
            println!("Starting to share ...");
            ui::Ui::share(args[1..].to_vec())
                .expect("Something went wrong while sending the files");
        }

        &"update" => {
            ui::Ui::update()
                .expect("Could not update metadata");
        }

        &"server" => {
            ui::Ui::server().expect("Could not start the server");
        }

        _ => println!("Wrong command"),
    }
}
