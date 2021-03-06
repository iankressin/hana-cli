// TODO: There is a lot of cloning back and forth
// Need to find a way to reduce duplications
use crate::types::MetaNotFoundError;
use hana_client::HanaClient;
use hana_types::Metadata;

pub struct Client {
    metadata: Vec<Metadata>,
    chosen: Vec<Metadata>,
}

impl Client {
    pub fn new(metadata: Vec<Metadata>) -> Self {
        Client {
            metadata,
            chosen: Vec::new(),
        }
    }

    pub fn pick_files(&mut self, file_names: Vec<String>) -> Result<(), MetaNotFoundError> {
        for name in &file_names {
            match self
                .metadata
                .iter()
                .find(|meta| meta.name_extension == name.clone())
            {
                Some(meta) => self.chosen.push(meta.clone()),
                None => {}
            }
        }

        match file_names.len() == self.chosen.len() {
            true => Ok(()),
            false => {
                self.clear_chosen();
                Err(MetaNotFoundError)
            },
        }
    }

    pub fn send(&self) {
        HanaClient::send(self.chosen.clone(), "./");
    }

    fn clear_chosen(&mut self) {
        self.chosen = Vec::new();
    }
}
