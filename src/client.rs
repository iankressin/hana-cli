// TODO: There is a lot of cloning back and forth
// Need to find a way of reducing duplications
use drive_client::drive_client::DriveClient;
use drive_client::types::Metadata;

pub struct Client {
    metadata: Vec<Metadata>,
    chosen: Vec<Metadata>,
}

impl Client {
    pub fn new(metadata: Vec<Metadata>) -> Self {
        Client { metadata, chosen: Vec::new() }
    }

    pub fn pick_files(&mut self, file_names: Vec<String>) {
        for name in file_names {
            match self
                .metadata
                .iter()
                .find(|meta| {
                    println!("{}", meta.name_extension == name);
                    meta.name_extension == name
                })
            {
                Some(meta) => self.chosen.push(meta.clone()),
                None => println!("No more files"),
            }
        }

        println!("Files to share: {:#?}", self.chosen);
    }

    pub fn send(&self) {
        DriveClient::send(self.metadata.clone());
    }

    fn update_metadata() {}
}
