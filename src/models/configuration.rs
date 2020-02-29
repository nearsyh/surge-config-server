use serde::{Serialize, Deserialize};

use futures;
use std::collections::HashMap;

use super::surge::ProxyGroup;
use super::surge::SurgeConfiguration;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Configuration {
  name: String,
  airports: HashMap<String, AirportConfiguration>,
  rules: String,
  group_configurations: HashMap<String, GroupConfiguration>,
}

impl Configuration {
  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn upsert_airport_configuration(&mut self, config: AirportConfiguration) {
    self.airports.insert(config.airport_id.clone(), config);
  }

  pub fn update_rules(&mut self, rules: &str) {
    self.rules = String::from(rules);
  }

  pub fn upsert_group_configuration(&mut self, config: GroupConfiguration) {
    self
      .group_configurations
      .insert(config.group_id.clone(), config);
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AirportConfiguration {
  airport_id: String,
  airport_name: String,
  url: String,
}

impl AirportConfiguration {
  async fn fetch_surge_configuration(&self) -> Option<SurgeConfiguration> {
    SurgeConfiguration::from_url(&self.url).await
  }

  pub fn new(id: &str, name: &str, url: &str) -> AirportConfiguration {
    AirportConfiguration {
      airport_id: String::from(id),
      airport_name: String::from(name),
      url: String::from(url),
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct GroupConfiguration {
  group_id: String,
  group_name: String,
  pattern: String,
}

impl GroupConfiguration {
  pub fn new(id: &str, name: &str, pattern: &str) -> GroupConfiguration {
    GroupConfiguration {
      group_id: String::from(id),
      group_name: String::from(name),
      pattern: String::from(pattern),
    }
  }
}

impl Configuration {
  pub fn empty(name: &str) -> Self {
    Configuration {
      name: String::from(name),
      airports: HashMap::new(),
      rules: String::new(),
      group_configurations: HashMap::new(),
    }
  }
}

impl Configuration {
  pub async fn fetch_surge_configuration(&self) -> Option<SurgeConfiguration> {
    let config_futures: Vec<_> = self
      .airports
      .values()
      .map(|airport_config| airport_config.fetch_surge_configuration())
      .collect();
    let surge_configurations: Vec<_> = futures::future::join_all(config_futures)
      .await
      .iter()
      .filter(|option| option.is_some())
      .map(|option| option.as_ref().unwrap().clone())
      .collect();
    match Configuration::merge_surge_configurations(&surge_configurations[..]) {
      Some(mut surge_configuration) => {
        self.populate_surge_head(&mut surge_configuration);
        self.populate_surge_rules(&mut surge_configuration);
        self.populate_surge_proxy_groups(&mut surge_configuration);
        Some(surge_configuration)
      }
      _ => None,
    }
  }

  fn populate_surge_head(&self, surge_configuration: &mut SurgeConfiguration) {
    surge_configuration.set_head(String::from(""));
  }

  fn populate_surge_rules(&self, surge_configuration: &mut SurgeConfiguration) {
    for rule in self.rules.split("\n") {
      let clean_rule = rule.trim();
      if !clean_rule.is_empty() {
        surge_configuration.add_rule(String::from(clean_rule));
      }
    }
  }

  fn populate_surge_proxy_groups(&self, surge_configuration: &mut SurgeConfiguration) {
    for (group_name, group_config) in self.group_configurations.iter() {
      let mut group = ProxyGroup::with_name(group_name);
      let regex = regex::Regex::new(&group_config.pattern).unwrap();
      for proxy in surge_configuration.get_proxies() {
        if regex.is_match(proxy.get_name()) {
          group.add_proxy(proxy.get_name());
        }
      }
      surge_configuration.add_proxy_group(group);
    }
  }

  fn merge_surge_configurations(
    surge_configurations: &[SurgeConfiguration],
  ) -> Option<SurgeConfiguration> {
    if surge_configurations.is_empty() {
      return None;
    }

    let mut ret = SurgeConfiguration::default();
    for config in surge_configurations {
      ret.merge(config);
    }
    Some(ret)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[tokio::test]
  async fn empty_config_to_surge_configuration_works() {
    let configuration = Configuration::empty("empty");
    let surge_configuration_opt = configuration.fetch_surge_configuration().await;
    assert!(surge_configuration_opt.is_none());
  }

  #[tokio::test]
  async fn config_to_surge_configuration_works() {
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
    let surge_configuration = configuration.fetch_surge_configuration().await.unwrap();
    assert_eq!(surge_configuration.get_proxies().len(), 4);
    assert_eq!(surge_configuration.get_proxy_groups().len(), 1);
    assert_eq!(
      surge_configuration.get_proxy_groups()[0]
        .get_proxies()
        .len(),
      1
    );
    assert_eq!(
      surge_configuration.get_proxy_groups()[0].get_proxies()[0],
      "Proxy_1_1 | Media"
    );
  }
}
