#[derive(Debug)]
pub struct SurgeConfiguration {
  head: String
}

impl Default for SurgeConfiguration {
  fn default() -> Self {
    SurgeConfiguration {
      head: String::from("")
    }
  }
}

impl SurgeConfiguration {
  fn to_config(&self) -> String {
    String::from("")
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  pub fn default_should_work() {
    let surge_config = SurgeConfiguration::default();
    assert_eq!(surge_config.head, "")
  }

  #[test]
  pub fn to_config_should_work() {

  }
}