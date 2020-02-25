use std::collections::HashMap;

use super::surge::SurgeConfiguration;

pub struct Configuration {
  airports: Vec<AirportConfiguration>,
  rules: Vec<Rule>,
  group_configurations: Vec<GroupConfiguration>,
}

struct AirportConfiguration {
  airport_id: String,
  airport_name: String,
  url: String
}

struct Rule {
  rule_id: String,
  rule_name: String,
  rule: String,
  order: u32
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
      airports: vec![],
      rules: vec![],
      group_configurations: vec![]
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