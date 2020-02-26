use std::collections::BTreeMap;

fn params_map_from_strs(entries: &[&str]) -> BTreeMap<String, String> {
  let mut ret = BTreeMap::new();
  for entry in entries {
    if entry.contains("=") {
      if let [name, value] = &entry.split("=").collect::<Vec<_>>()[..] {
        ret.insert(String::from(name.trim()), String::from(value.trim()));
      }
    }
  }
  ret
}

fn string_vec_from_strs(elems: &[&str]) -> Vec<String> {
  elems
    .iter()
    .filter(|elem| !elem.contains("="))
    .map(|elem| String::from(elem.trim()))
    .collect()
}

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
  username: Option<String>,
  password: Option<String>,
  parameters: BTreeMap<String, String>,
}

impl Proxy {
  fn from_strs(
    name: &str,
    proto: &str,
    host: &str,
    port_str: &str,
    param_strs: &[&str],
  ) -> Option<Proxy> {
    let mut param_strs = param_strs;
    let username = if !param_strs[0].contains("=") {
      Some(String::from(param_strs[0].trim()))
    } else {
      None
    };
    let password = if username.is_some() {
      Some(String::from(param_strs[1].trim()))
    } else {
      None
    };
    param_strs = if password.is_some() {
      &param_strs[2..]
    } else {
      param_strs
    };

    match port_str.trim().parse::<u32>() {
      Ok(port) => Some(Proxy {
        name: String::from(name.trim()),
        proto: String::from(proto.trim()),
        host: String::from(host.trim()),
        port,
        username,
        password,
        parameters: params_map_from_strs(param_strs),
      }),
      _ => None,
    }
  }

  fn from_name_definition(name: &str, definition: &str) -> Option<Proxy> {
    let def_parts: Vec<_> = definition.split(",").collect();
    match &def_parts[0..3] {
      [proto, host, port_str] => Proxy::from_strs(name, *proto, *host, *port_str, &def_parts[3..]),
      _ => None,
    }
  }

  fn from_str(proxy_str: &str) -> Option<Proxy> {
    let components: Vec<_> = proxy_str.splitn(2, "=").collect();
    match &components[..] {
      [name, definition] => Proxy::from_name_definition(*name, *definition),
      _ => None,
    }
  }
}

impl ToString for Proxy {
  fn to_string(&self) -> String {
    let mut ret = String::new();
    let mut definition_parts: Vec<&str> = vec![];
    definition_parts.push(&self.proto);
    definition_parts.push(&self.host);
    definition_parts.push(&self.port.to_string());
    if let Some(ref username_str) = &self.username {
      definition_parts.push(username_str);
    }
    if let Some(ref password_str) = &self.password {
      definition_parts.push(password_str);
    }
    for (name, value) in &self.parameters {
      definition_parts.push(&[name, "=", value].concat());
    }

    ret.push_str(&self.name);
    ret.push_str(" = ");
    ret.push_str(&definition_parts.join(","));
    ret
  }
}

#[derive(Debug, PartialEq, Eq)]
enum ProxyGroupType {
  Select,
  UrlTest {
    url: String,
    interval: u32,
    tolerance: u32,
    timeout: u32,
  },
}

impl ProxyGroupType {
  fn from_str(type_str: &str, params_map: &BTreeMap<String, String>) -> Option<ProxyGroupType> {
    match type_str.trim() {
      "select" => Some(ProxyGroupType::Select),
      "url-test" => Some(ProxyGroupType::UrlTest {
        url: params_map
          .get("url")
          .map(|s| s.clone())
          .unwrap_or(String::from("www.google.com")),
        interval: params_map
          .get("interval")
          .and_then(|s| s.parse::<u32>().ok())
          .unwrap_or(600),
        tolerance: params_map
          .get("tolerance")
          .and_then(|s| s.parse::<u32>().ok())
          .unwrap_or(100),
        timeout: params_map
          .get("timeout")
          .and_then(|s| s.parse::<u32>().ok())
          .unwrap_or(5),
      }),
      _ => None,
    }
  }
}

#[derive(Debug)]
struct ProxyGroup {
  name: String,
  group_type: ProxyGroupType,
  proxy_names: Vec<String>,
}

impl ProxyGroup {
  fn from_name_definition(name: &str, definition: &str) -> Option<ProxyGroup> {
    let components: Vec<_> = definition.split(",").collect();
    let params_map = params_map_from_strs(&components[..]);
    components
      .get(0)
      .and_then(|type_str| ProxyGroupType::from_str(type_str.trim(), &params_map))
      .map(|group_type| ProxyGroup {
        name: String::from(name.trim()),
        group_type: group_type,
        proxy_names: string_vec_from_strs(&components[1..]),
      })
  }

  fn from_str(proxy_group: &str) -> Option<ProxyGroup> {
    let components: Vec<_> = proxy_group.splitn(2, "=").collect();
    match &components[..] {
      [name, definition] => ProxyGroup::from_name_definition(name.trim(), *definition),
      _ => None
    }
  }
}

impl ToString for ProxyGroup {
  fn to_string(&self) -> String {
    String::new()
  }
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
    ret.push_str("\n");

    ret.push_str("[General]");
    for general in &self.general {
      ret.push_str(&general);
    }
    ret.push_str("\n");

    ret.push_str("[Proxy]");
    for proxy in &self.proxies {
      ret.push_str(&proxy.to_string())
    }
    ret.push_str("\n");

    ret.push_str("[Proxy Group]");
    for proxy_group in &self.proxy_groups {
      ret.push_str(&proxy_group.to_string());
    }
    ret.push_str("\n");

    ret.push_str("[Rule]");
    for rule in &self.rules {
      ret.push_str(&rule);
    }
    ret.push_str("\n");

    ret.push_str("[URL Rewrite]");
    for rewrite in &self.url_rewrites {
      ret.push_str(&rewrite);
    }
    ret.push_str("\n");

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
  pub fn https_proxy_from_str_should_work() {
    let proxy = Proxy::from_str(
      "🇨🇳 CN2 ⚡ = https , a.b.c.net , 3389 , username , password , tls13=false, obfs =  aaa",
    )
    .expect("Parsing should work");
    assert_eq!(proxy.name, "🇨🇳 CN2 ⚡");
    assert_eq!(proxy.proto, "https");
    assert_eq!(proxy.port, 3389);
    assert_eq!(proxy.host, "a.b.c.net");
    assert_eq!(proxy.username.unwrap(), "username");
    assert_eq!(proxy.password.unwrap(), "password");
    assert_eq!(proxy.parameters.get("tls13").unwrap(), "false");
    assert_eq!(proxy.parameters.get("obfs").unwrap(), "aaa");
  }

  #[test]
  pub fn ss_proxy_from_str_should_work() {
    let proxy = Proxy::from_str("🇭🇰 HK Standard A01 | Media | Rate 0.5x = ss, endpoint, 447, encrypt-method=abc, password=ddd, obfs=abc,obfs-host=ddd, tfo=true")
      .expect("Parsing should work");
    assert_eq!(proxy.name, "🇭🇰 HK Standard A01 | Media | Rate 0.5x");
    assert_eq!(proxy.proto, "ss");
    assert_eq!(proxy.port, 447);
    assert_eq!(proxy.host, "endpoint");
    assert!(proxy.username.is_none());
    assert!(proxy.password.is_none());
    assert_eq!(proxy.parameters.get("encrypt-method").unwrap(), "abc");
    assert_eq!(proxy.parameters.get("password").unwrap(), "ddd");
    assert_eq!(proxy.parameters.get("obfs").unwrap(), "abc");
    assert_eq!(proxy.parameters.get("obfs-host").unwrap(), "ddd");
    assert_eq!(proxy.parameters.get("tfo").unwrap(), "true");
  }

  #[test]
  pub fn https_proxy_to_string_should_work() {
    let mut params = BTreeMap::new();
    params.insert(String::from("abd"), String::from("def"));
    params.insert(String::from("abc"), String::from("def"));
    let proxy = Proxy {
      name: String::from("https proxy"),
      proto: String::from("https"),
      host: String::from("www.a.com"),
      port: 447,
      username: Some(String::from("abc")),
      password: Some(String::from("def")),
      parameters: params
    };
    assert_eq!(proxy.to_string(), "https proxy = https,www.a.com,447,abc,def,abc=def,abd=def");
  }

  #[test]
  pub fn select_group_from_str_should_work() {
    let proxy_group = ProxyGroup::from_str("AsianTV = select, Direct, Proxy, 🇭🇰 HK Standard A01 | Media | Rate 0.5x, 🇭🇰 HK Standard A02 | Media | Rate 0.5x")
      .expect("Parsing should work");
    assert_eq!(proxy_group.name, "AsianTV");
    assert_eq!(proxy_group.group_type, ProxyGroupType::Select);
    assert_eq!(proxy_group.proxy_names, vec!["Direct", "Proxy", "🇭🇰 HK Standard A01 | Media | Rate 0.5x", "🇭🇰 HK Standard A02 | Media | Rate 0.5x"])
  }

  #[test]
  pub fn url_test_group_from_str_should_work() {
    let proxy_group = ProxyGroup::from_str("AsianTV = url-test, Direct, Proxy, 🇭🇰 HK Standard A01 | Media | Rate 0.5x, 🇭🇰 HK Standard A02 | Media | Rate 0.5x, url = http://www.qualcomm.cn/generate_204, interval = 1800, tolerance = 200")
      .expect("Parsing should work");
    assert_eq!(proxy_group.name, "AsianTV");
    assert_eq!(proxy_group.group_type, ProxyGroupType::UrlTest {
      url: String::from("http://www.qualcomm.cn/generate_204"),
      interval: 1800,
      tolerance: 200,
      timeout: 5
    });
    assert_eq!(proxy_group.proxy_names, vec!["Direct", "Proxy", "🇭🇰 HK Standard A01 | Media | Rate 0.5x", "🇭🇰 HK Standard A02 | Media | Rate 0.5x"])
  }

  #[test]
  pub fn to_config_should_work() {
    let mut surge_config = SurgeConfiguration::default();
    surge_config.head = String::from("!MANAGED-CONFIG https://abc.com");
    surge_config
      .general
      .push(String::from("http-listen = 0.0.0.0:8888"));
    surge_config.proxies.push(Proxy::from_str("🇭🇰 HK Standard A01 | Media | Rate 0.5x = ss, endpoint, 447, encrypt-method=abc, password=ddd, obfs=abc,obfs-host=ddd, tfo=true").unwrap());
    surge_config.proxy_groups.push(ProxyGroup::from_str("AsianTV = select, Direct, Proxy, 🇭🇰 HK Standard A01 | Media | Rate 0.5x, 🇭🇰 HK Standard A02 | Media | Rate 0.5x").unwrap());
    surge_config.rules.push(String::from("DOMAIN-SUFFIX,gazellegames.net,DIRECT"));
    surge_config.url_rewrites.push(String::from("^https?://(www.)?g.cn https://www.google.com 302"));

    let config = surge_config.to_config();
    assert_eq!(r#"
    !MANAGED-CONFIG https://abc.com

    [General]
    http-listen = 0.0.0.0:8888 

    [Proxy]
    🇭🇰 HK Standard A01 | Media | Rate 0.5x = ss,endpoint,447,encrypt-method=abc,password=ddd,obfs=abc,obfs-host=ddd,tfo=true

    [Proxy Group]
    AsianTV = select,Direct,Proxy,🇭🇰 HK Standard A01 | Media | Rate 0.5x,🇭🇰 HK Standard A02 | Media | Rate 0.5x
    AsianTV = url-test,Direct,Proxy,🇭🇰 HK Standard A01 | Media | Rate 0.5x,🇭🇰 HK Standard A02 | Media | Rate 0.5x,url=http://www.qualcomm.cn/generate_204,interval=1800,tolerance=200

    [Rule]
    DOMAIN-SUFFIX,gazellegames.net,DIRECT

    [URL Rewrite]
    ^https?://(www.)?g.cn https://www.google.com 302
    "#);
  }
}
