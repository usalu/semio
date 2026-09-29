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
    assert!(with(&[(ALLOWED_ORIGINS, "https://quizzes.example")]).expect_err("no proxy").0.contains(TRUSTED_FORWARDING));
    let config = with(&[(ALLOWED_ORIGINS, "https://quizzes.example, https://QUIZZES.example"), (TRUSTED_FORWARDING, "proxy"), (PORT, "9000")]).expect("all three statements");
    assert_eq!((config.mode, config.port, config.forwarding), (ProctorMode::Production, 9000, Forwarding::TerminatingProxy));
    assert_eq!(config.origins, CrossOriginPolicy::Allowlist(vec!["https://quizzes.example".to_string()]));
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
fn the_presence_tick_is_configurable_within_bounds() {
    let tick = |millis: Option<&str>| ProctorConfig::from_environment(environment(&[&REQUIRED[..], &millis.map(|millis| (PRESENCE_TICK_MS, millis)).into_iter().collect::<Vec<_>>()[..]].concat())).map(|config| config.presence());
    let defaults = tick(None).expect("the default tick");
    assert_eq!(defaults, PresenceSettings::default());
    assert_eq!(defaults.tick, Duration::from_millis(100));
    let fast = tick(Some("40")).expect("a faster tick");
    assert_eq!(fast, PresenceSettings { tick: Duration::from_millis(40), ..PresenceSettings::default() });
    assert_eq!(tick(Some("10")).expect("the lower bound").tick, Duration::from_millis(10));
    assert_eq!(tick(Some("1000")).expect("the upper bound").tick, Duration::from_secs(1));
    for refused in ["9", "1001", "0", "-5", "fast", "100ms"] {
        assert!(tick(Some(refused)).expect_err(refused).0.contains(PRESENCE_TICK_MS), "{refused}");
    }
}

#[test]
fn origins_are_admitted_by_policy() {
    let development = CrossOriginPolicy::LoopbackDevelopment;
    assert!(development.admits("http://localhost:6061"));
    assert!(development.admits("http://127.0.0.1:8791"));
    assert!(development.admits("http://[::1]:5173"));
    assert!(!development.admits("https://quizzes.example"));
    assert!(!development.admits("http://localhost.evil.example"));
    let allowlist = CrossOriginPolicy::Allowlist(vec!["https://quizzes.example".to_string()]);
    assert!(allowlist.admits("HTTPS://quizzes.example"));
    assert!(!allowlist.admits("https://quizzes.example:444"));
    assert!(!CrossOriginPolicy::Closed.admits("http://localhost:6061"));
    assert!(is_browser_origin("https://quizzes.example:8443"));
    assert!(!is_browser_origin("https://quizzes.example:port"));
    assert!(!is_browser_origin("ftp://quizzes.example"));
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
