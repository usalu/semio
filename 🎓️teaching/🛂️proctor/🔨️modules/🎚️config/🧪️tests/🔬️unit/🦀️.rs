use super::*;
use std::collections::BTreeMap;

fn environment(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
    let values: BTreeMap<String, String> = pairs.iter().map(|(name, value)| ((*name).to_string(), (*value).to_string())).collect();
    move |name| values.get(name).cloned()
}

const REQUIRED: [(&str, &str); 2] = [(DATA, "data"), (CATALOG, "catalog.json")];

#[test]
fn a_loopback_proctor_defaults_to_development() {
    let config = ProctorConfig::from_environment(environment(&REQUIRED)).expect("valid");
    assert_eq!((config.bind, config.port, config.mode, config.forwarding), (IpAddr::V4(Ipv4Addr::LOCALHOST), DEFAULT_PORT, ProctorMode::Development, Forwarding::Untrusted));
    assert_eq!(config.origins, CrossOriginPolicy::LoopbackDevelopment);
    assert_eq!(config.site, None);
}

#[test]
fn data_and_catalog_are_required() {
    let missing = ProctorConfig::from_environment(environment(&[(CATALOG, "catalog.json")])).expect_err("data is required");
    assert!(missing.0.contains(DATA));
    let missing = ProctorConfig::from_environment(environment(&[(DATA, "data")])).expect_err("catalog is required");
    assert!(missing.0.contains(CATALOG));
}

#[test]
fn a_network_bind_needs_production_an_allowlist_and_a_proxy() {
    let bind = [(BIND, "0.0.0.0")];
    let with = |extra: &[(&str, &str)]| ProctorConfig::from_environment(environment(&[&REQUIRED[..], &bind[..], extra].concat()));
    assert!(with(&[(MODE, "development")]).expect_err("development binds loopback").0.contains("loopback"));
    assert!(with(&[]).expect_err("no allowlist").0.contains(ALLOWED_ORIGINS));
    assert!(with(&[(ALLOWED_ORIGINS, "https://quizze.example")]).expect_err("no proxy").0.contains(TRUSTED_FORWARDING));
    let config = with(&[(ALLOWED_ORIGINS, "https://quizze.example, https://QUIZZE.example"), (TRUSTED_FORWARDING, "proxy"), (PORT, "9000"), (SITE, "dist")]).expect("all three statements");
    assert_eq!((config.mode, config.port, config.forwarding), (ProctorMode::Production, 9000, Forwarding::TerminatingProxy));
    assert_eq!(config.origins, CrossOriginPolicy::Allowlist(vec!["https://quizze.example".to_string()]));
    assert_eq!(config.site, Some(PathBuf::from("dist")));
}

#[test]
fn malformed_values_are_refused_by_name() {
    let with = |extra: (&str, &str)| ProctorConfig::from_environment(environment(&[REQUIRED[0], REQUIRED[1], extra])).expect_err("refused").0;
    assert!(with((PORT, "eighty")).contains(PORT));
    assert!(with((BIND, "localhost")).contains(BIND));
    assert!(with((MODE, "staging")).contains(MODE));
    assert!(with((TRUSTED_FORWARDING, "always")).contains(TRUSTED_FORWARDING));
    assert!(with((ALLOWED_ORIGINS, "https://example.com/path")).contains(ALLOWED_ORIGINS));
    assert!(with((ALLOWED_ORIGINS, "*")).contains(ALLOWED_ORIGINS));
}

#[test]
fn origins_are_admitted_by_policy() {
    let development = CrossOriginPolicy::LoopbackDevelopment;
    assert!(development.admits("http://localhost:6061"));
    assert!(development.admits("http://127.0.0.1:8791"));
    assert!(development.admits("http://[::1]:5173"));
    assert!(!development.admits("https://quizze.example"));
    assert!(!development.admits("http://localhost.evil.example"));
    let allowlist = CrossOriginPolicy::Allowlist(vec!["https://quizze.example".to_string()]);
    assert!(allowlist.admits("HTTPS://quizze.example"));
    assert!(!allowlist.admits("https://quizze.example:444"));
    assert!(!CrossOriginPolicy::Closed.admits("http://localhost:6061"));
    assert!(is_browser_origin("https://quizze.example:8443"));
    assert!(!is_browser_origin("https://quizze.example:port"));
    assert!(!is_browser_origin("ftp://quizze.example"));
}

#[test]
fn a_trusted_proxy_must_report_https() {
    assert!(Forwarding::Untrusted.secure(None));
    assert!(Forwarding::Untrusted.secure(Some("http")));
    assert!(Forwarding::TerminatingProxy.secure(Some("https")));
    assert!(Forwarding::TerminatingProxy.secure(Some("HTTPS, http")));
    assert!(!Forwarding::TerminatingProxy.secure(Some("http")));
    assert!(!Forwarding::TerminatingProxy.secure(None));
}
