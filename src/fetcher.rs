use super::models::Configuration;
use jfs::Store;

use std::collections::BTreeMap;

pub struct Fetcher {
  db: Store,
}

impl Fetcher {
  fn new(path: &str) -> Fetcher {
    Fetcher {
      db: Store::new(path).unwrap(),
    }
  }

  fn save_configuration(&self, configuration: &Configuration) {
    self
      .db
      .save_with_id(configuration, configuration.get_name())
      .unwrap();
  }

  fn get_configurations(&self) -> BTreeMap<String, Configuration> {
    self.db.all().unwrap()
  }

  fn get_configuration(&self, name: &str) -> Option<Configuration> {
    self.db.get(name).ok()
  }
}

#[cfg(test)]
mod test {

  use super::super::models::*;
  use super::*;

  #[test]
  pub fn create_fetcher_should_work() {
    Fetcher::new("data");
  }

  #[test]
  pub fn save_get_should_work() {
    let fetcher = Fetcher::new("data");
    let mut configuration = Configuration::empty("test");
    configuration.upsert_airport_configuration(AirportConfiguration::new(
      "airport_1",
      "airport_1_name",
      "https://gist.githubusercontent.com/nearsyh/b581e7fa0f007d104336fad5ac124be7/raw/94c9c9b4ad024f6874ad7310d5a24fa1d79dc2c9/surge_config_airport_1"));
    configuration.upsert_airport_configuration(AirportConfiguration::new(
      "airport_2",
      "airport_2_name",
      "https://gist.githubusercontent.com/nearsyh/45695b3332f02609c71a1a084dbfb5bf/raw/67c0c6b1ae2c5a8f044a5f7ea10d009c990c5469/surge_config_airport_2"));
    configuration.upsert_group_configuration(GroupConfiguration::new(
      "group_1",
      "group_1_name",
      "Media",
    ));
    fetcher.save_configuration(&configuration);
    let saved = fetcher.get_configuration(configuration.get_name()).unwrap();
    assert_eq!(configuration, saved);
  }

  #[test]
  pub fn get_non_exist_configuration_should_work() {
    let fetcher = Fetcher::new("data");
    assert!(fetcher.get_configuration("aaaa").is_none());
  }
}
