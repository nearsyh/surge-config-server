mod models;

use std::collections::HashMap;

struct Configuration {
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