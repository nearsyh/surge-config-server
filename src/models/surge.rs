use std::collections::HashMap;

#[derive(Debug)]
pub struct SurgeConfiguration {
  head: String,
  general: Vec<String>,
  proxies: Vec<Proxy>,
  proxy_groups: Vec<ProxyGroup>,
  rules: Vec<String>,
  url_rewrites: Vec<String>,
}

#[derive(Debug)]
struct Proxy {
  name: String,
  proto: String,
  host: String,
  port: u32,
  parameters: HashMap<String, String>,
}

impl Proxy {
  fn _parameters_from_str(param_strs: &[&str]) -> HashMap<String, String> {
    let mut ret = HashMap::new();
    for param_entry in param_strs {
      let pairs: Vec<_> = param_entry.split("=").collect();
      if let [name, value] = &pairs[..] {
        ret.insert(String::from(*name), String::from(*value));
      }
    }
    ret
  }

  fn from_strs(
    name: &str,
    proto: &str,
    host: &str,
    port_str: &str,
    param_strs: &[&str]
  ) -> Option<Proxy> {
    match port_str.parse::<u32>() {
      Ok(port) => Some(Proxy {
        name: String::from(name),
        proto: String::from(proto),
        host: String::from(host),
        port,
        parameters: Proxy::_parameters_from_str(param_strs),
      }),
      _ => None,
    }
  }

  fn from_name_definition(name: &str, definition: &str) -> Option<Proxy> {
    let def_parts: Vec<_> = definition.split(",").collect();
    match &def_parts[0..3] {
      [proto, host, port_str] => {
        Proxy::from_strs(name, *proto, *host, *port_str, &def_parts[3..])
      }
      _ => None,
    }
  }

  fn from_str(proxy_str: &str) -> Option<Proxy> {
    let components: Vec<_> = proxy_str.split(" = ").collect();
    match &components[..] {
      [name, definition] => Proxy::from_name_definition(*name, *definition),
      _ => None,
    }
  }
}

#[derive(Debug)]
enum ProxyGroupType {
  Select,
  UrlTest(String, u32, u32, u32),
}

#[derive(Debug)]
struct ProxyGroup {
  name: String,
  group_type: ProxyGroupType,
  proxy_names: Vec<String>,
}

impl Default for SurgeConfiguration {
  fn default() -> Self {
    SurgeConfiguration {
      head: String::from(""),
      general: vec![],
      proxies: vec![],
      proxy_groups: vec![],
      rules: vec![],
      url_rewrites: vec![],
    }
  }
}

impl SurgeConfiguration {
  fn to_config(&self) -> String {
    let mut ret = String::new();
    ret.push_str(&self.head);
    ret
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  pub fn default_should_work() {
    let surge_config = SurgeConfiguration::default();
    assert_eq!(surge_config.head, "");
    assert!(surge_config.general.is_empty());
    assert!(surge_config.proxies.is_empty());
  }

  #[test]
  pub fn proxy_from_str_should_work() {
    let proxy = Proxy::from_str("🇨🇳 PandaFan.website | CN2 香港高级线路 ⚡ = https,cn2.gmdns.net,3389,229464,c40b4311,tls13=false")
      .expect("Parsing should work");
    assert_eq!(proxy.name, "🇨🇳 PandaFan.website | CN2 香港高级线路 ⚡");
  }

  #[test]
  pub fn to_config_should_work() {
    let mut surge_config = SurgeConfiguration::default();
    surge_config.head = String::from("!MANAGED-CONFIG https://abc.com");
    surge_config
      .general
      .push(String::from("http-listen = 0.0.0.0:8888"));
  }
}
