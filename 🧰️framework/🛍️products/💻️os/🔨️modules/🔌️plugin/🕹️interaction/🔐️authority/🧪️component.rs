//! 🧪️ Cross-language fixed authority framing and pre-mutation overflow/close rejection.
use super::*;

#[test]
fn local_interaction_topology_input_authority_matches_node_crypto_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/🔐️topology-authority/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let authority = LocalInteractionTopologyAuthority { ui_generation: row["uiGeneration"].as_str().unwrap().parse().unwrap(), closed: false };
        let actual = authority.revision([row["documentByte"].as_u64().unwrap() as u8; 32], [row["configByte"].as_u64().unwrap() as u8; 32]).unwrap();
        let actual = actual.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(actual, row["expected"].as_str().unwrap());
    }
}

#[test]
fn local_interaction_topology_overflow_rejects_before_cache_mutation() {
    let mut authority = LocalInteractionTopologyAuthority { ui_generation: u64::MAX, closed: false };
    let before = authority.revision([1; 32], [2; 32]).unwrap();
    let mut cache = Some("unchanged");
    if authority.before_cache_mutation().is_ok() { cache = None; }
    assert_eq!(cache, Some("unchanged"));
    assert_eq!(authority.ui_generation, u64::MAX);
    assert_eq!(authority.revision([1; 32], [2; 32]).unwrap(), before);
}

#[test]
fn local_interaction_topology_close_invalidates_authority_without_wrapping() {
    let mut authority = LocalInteractionTopologyAuthority::default();
    let initial = authority.revision([1; 32], [2; 32]).unwrap();
    authority.before_cache_mutation().unwrap();
    assert_ne!(authority.revision([1; 32], [2; 32]).unwrap(), initial);
    authority.close();
    assert_eq!(authority.revision([1; 32], [2; 32]), Err("local-interaction.authority-closed"));
    assert_eq!(authority.before_cache_mutation(), Err("local-interaction.authority-closed"));
    assert_eq!(authority.ui_generation, 1);
}
