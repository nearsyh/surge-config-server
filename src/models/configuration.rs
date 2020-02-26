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
    self
      .group_configurations
      .insert(config.group_id.clone(), config);
  }
}

struct AirportConfiguration {
  airport_id: String,
  airport_name: String,
  url: String,
}

impl AirportConfiguration {
  async fn fetch_surge_configuration(&self) -> Option<SurgeConfiguration> {
    SurgeConfiguration::from_url(&self.url).await
  }
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
      group_configurations: HashMap::new(),
    }
  }
}

impl Configuration {
  async fn fetch_surge_configuration(&self) -> Option<SurgeConfiguration> {
    let surge_configurations: HashMap<String, SurgeConfiguration> = self
      .airports
      .iter()
      .map(async move |(id, airport_config)| (id.clone(), airport_config.fetch_surge_configuration().await?))
      // .filter(|(_, surge_config_opt)| surge_config_opt.is_some())
      // .map(|(id, surge_config_opt)| (id, surge_config_opt.unwrap()))
      .collect();
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
    let surge_configuration: SurgeConfiguration = (&configuration).into();
    println!("{:?}", surge_configuration);
  }
}
