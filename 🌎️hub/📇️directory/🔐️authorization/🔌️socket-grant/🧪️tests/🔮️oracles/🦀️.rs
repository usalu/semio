use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use syn::{Attribute, Item};

fn repo_root() -> PathBuf {
    std::env::var_os("SEMIO_REPO_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.."))
}

fn source(path: impl AsRef<Path>) -> String {
    read_to_string(path).expect("socket-grant source must remain readable")
}

fn test_attribute(attribute: &Attribute) -> bool {
    attribute.path().segments.last().is_some_and(|segment| segment.ident == "test")
}

fn test_declarations(source: &str) -> Vec<String> {
    syn::parse_file(source)
        .expect("socket-grant source must parse through syn")
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Fn(function) if function.attrs.iter().any(test_attribute) => Some(function.sig.ident.to_string()),
            _ => None,
        })
        .collect()
}

fn exact_declaration(names: &[String], expected: &str) -> bool {
    names.iter().filter(|name| name.as_str() == expected).count() == 1
}

fn same_scope(left: &Value, right: &Value) -> bool {
    left["spaceId"] == right["spaceId"] && left["documentId"] == right["documentId"]
}

fn decision(vector: &Value) -> (&'static str, Option<u64>, bool, u64) {
    if !same_scope(&vector["grantScope"], &vector["urlScope"]) {
        return ("deny-before-upgrade", Some(4401), false, 0);
    }
    if vector["gateWinner"] == "removal" || vector["binding"] == "unauthorized" || vector["descriptor"] == false || vector["live"] == false {
        return ("close-unauthorized", Some(4401), false, 0);
    }
    if vector["binding"] == "unavailable" {
        return ("close-unavailable", Some(1013), false, 0);
    }
    let class = vector["message"]["class"].as_str().expect("message class");
    let scoped = ["document-announced", "checkpoint", "retention", "rebootstrap", "presence", "connection"].contains(&class);
    if !scoped || vector["message"]["scope"].is_null() || !same_scope(&vector["grantScope"], &vector["message"]["scope"]) {
        return ("skip-unrelated", None, false, 0);
    }
    ("deliver", None, ["document-announced", "checkpoint", "retention"].contains(&class), 1)
}

#[test]
fn hub_socket_grant_fixture_serde_parity() {
    let root = repo_root();
    let contract: Value = serde_json::from_str(&source(root.join("🌎️hub/🧫️fixtures/🧱️socket-grant-command-source/🔣️.json"))).expect("socket-grant source fixture JSON");
    let fixture: Value = serde_json::from_str(&source(root.join("🌎️hub/📇️directory/🧫️fixtures/🔌️scoped-socket-revocation-v1/🔣️.json"))).expect("socket-grant decision fixture JSON");
    let stages = contract["nativeStages"].as_array().expect("native stage array");
    assert_eq!(stages.len(), 19);
    assert_eq!(stages.iter().filter(|stage| stage["source"] == "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs").count(), 15);
    assert_eq!(stages.iter().filter(|stage| stage["source"].is_null()).count(), 1);
    let stage_ids = stages.iter().map(|stage| stage["id"].as_str().expect("stage id")).collect::<HashSet<_>>();
    assert_eq!(stage_ids.len(), stages.len());

    let mut declarations = HashMap::<String, Vec<String>>::new();
    for stage in stages {
        let (Some(path), Some(name)) = (stage["source"].as_str(), stage["declaration"].as_str()) else {
            assert!(stage["source"].is_null() && stage["declaration"].is_null());
            continue;
        };
        let names = declarations.entry(path.to_string()).or_insert_with(|| test_declarations(&source(root.join(path))));
        assert!(exact_declaration(names, name), "missing or duplicate test declaration: {name}");
    }
    let old = ["directory_socket_forced_lag_is_scope_authorized_and_closes_1013", "document_socket_forced_lag_sends_verified_control_then_closes_1013"];
    let selected = stages.iter().filter_map(|stage| stage["declaration"].as_str()).collect::<HashSet<_>>();
    assert!(old.iter().all(|name| !selected.contains(name)));
    let hub_binary = declarations.get("🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs").expect("Hub binary declarations");
    assert!(old.iter().all(|name| !hub_binary.iter().any(|actual| actual == name)));

    let hostile = r###"
// #[test] fn comment_lookalike() {}
const TEXT: &str = "#[test] fn string_lookalike() {}";
#[test]
fn one_real_test() { let _ = r##"{ #[test] fn raw_lookalike() {} }"##; }
"###;
    assert_eq!(test_declarations(hostile), ["one_real_test"]);
    let duplicates = test_declarations("#[test] fn repeated() {} #[test] fn repeated() {}");
    assert_eq!(duplicates.len(), 2);
    assert!(!exact_declaration(&duplicates, "repeated"));

    let vectors = fixture["vectors"].as_array().expect("decision vectors");
    assert_eq!(vectors.len(), 19);
    for vector in vectors {
        let actual = decision(vector);
        assert_eq!(actual.0, vector["expected"].as_str().expect("expected outcome"));
        assert_eq!(actual.1, vector["closeCode"].as_u64());
        assert_eq!(actual.2, vector["cursorAdvance"].as_bool().expect("cursor advance"));
        assert_eq!(actual.3, vector["textFrames"].as_u64().expect("text frames"));
    }
    let closes = fixture["clientCloses"].as_array().expect("client closes");
    assert_eq!(closes.len(), 3);
    for close in closes {
        let terminal = close["code"].as_u64() == Some(4401);
        assert_eq!(terminal, close["terminal"].as_bool().expect("terminal"));
        assert_ne!(terminal, close["reconnect"].as_bool().expect("reconnect"));
    }
}
