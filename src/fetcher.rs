use jfs::Store;
use super::models::Configuration;

use std::collections::BTreeMap;

pub struct Fetcher {
  db: Store
}

impl Fetcher {
  fn new() -> Fetcher {
    Fetcher {
      db: Store::new("data").unwrap()
    }
  }

  fn save_configuration(&self, configuration: &Configuration) {
    self.db.save_with_id(configuration, configuration.get_name()).unwrap();
  }

  fn get_configurations(&self) -> BTreeMap<String, Configuration> {
    self.db.all().unwrap()
  }

  fn get_configuration(&self, name: &str) -> Configuration {
    self.db.get(name).unwrap()
  }
}