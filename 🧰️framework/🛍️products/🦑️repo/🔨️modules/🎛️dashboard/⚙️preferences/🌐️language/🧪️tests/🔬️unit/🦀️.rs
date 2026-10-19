use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🌐️language/🔣️.json")).unwrap() }

fn id(locale: Option<Locale>) -> serde_json::Value { locale.map_or(serde_json::Value::Null, |locale| locale.as_str().into()) }

#[test]
fn system_values_name_a_supported_language_per_the_shared_vectors() {
    for row in fixture()["negotiate"].as_array().unwrap() {
        assert_eq!(id(negotiate(row["value"].as_str().unwrap())), row["language"], "{row}");
    }
}

#[test]
fn the_declared_chain_resolves_the_shared_environments() {
    let chain = declared();
    for row in fixture()["resolve"].as_array().unwrap() {
        let environment = row["environment"].as_object().unwrap();
        let lookup = |name: &str| environment.get(name).and_then(|value| value.as_str()).map(String::from);
        assert_eq!(chain.resolve(&lookup).as_str(), row["expected"].as_str().unwrap(), "{row}");
    }
}

#[test]
fn the_first_declared_fallback_is_used_when_the_system_names_none() {
    for row in fixture()["fallbackOrders"].as_array().unwrap() {
        let chain = Chain::parse(&row["declaration"].to_string()).unwrap();
        assert_eq!(chain.resolve(&|_| None).as_str(), row["expected"].as_str().unwrap(), "{row}");
    }
}

#[test]
fn a_declaration_without_a_supported_fallback_is_refused() {
    for row in fixture()["invalidDeclarations"].as_array().unwrap() { assert!(Chain::parse(&row.to_string()).is_err(), "{row}"); }
    assert!(Chain::parse(DECLARATION).is_ok());
}
