use std::collections::HashMap;

use super::surge::SurgeConfiguration;

pub struct Configuration {
  airports: HashMap<String, AirportConfiguration>,
  rules: String, 
  group_configurations: HashMap<String, GroupConfiguration>,
}

impl Configuration {
  fn upsert_airport_configuration(&mut self, config: AirportConfiguration) {
    self.airports.insert(config.airport_id.clone(), config);
  }

  fn update_rules(&mut self, rules: String) {
    self.rules = rules;
  }

  fn upsert_group_configuration(&mut self, config: GroupConfiguration) {
    self.group_configurations.insert(config.group_id.clone(), config);
  }
}

struct AirportConfiguration {
  airport_id: String,
  airport_name: String,
  url: String
}

struct GroupConfiguration {
  group_id: String,
  group_name: String,
  patterns: HashMap<String, GroupPattern>,
}

struct GroupPattern {
  name_regex: String,
}

impl Configuration {
  fn empty() -> Self {
    Configuration {
      airports: HashMap::new(),
      rules: String::new(),
      group_configurations: HashMap::new()
    }
  }
}

impl Into<SurgeConfiguration> for Configuration {
  fn into(self) -> SurgeConfiguration {

    SurgeConfiguration::default()
  }
}

#[cfg(test)]
mod tests {

  use super::Configuration;
  use super::SurgeConfiguration;

  #[test]
  fn to_surge_configuration_works() {
      let configuration = Configuration::empty();
      let surge_configuration: SurgeConfiguration = configuration.into();
      println!("{:?}", surge_configuration);
  }
}