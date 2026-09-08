//! 🧪️ Standalone playbook package laws against a third-party JSON implementation.
use crate::*;

#[test]
fn ordered_document_fixture_matches_serde_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../📦️package-contract/📜️cases.json")).expect("language-neutral fixture");
    assert_eq!(fixture["package"], env!("CARGO_PKG_NAME"));
    let encoded = serde_json::to_string(&fixture["document"]).expect("fixture JSON");
    let oracle: PlaybookSpec = serde_json::from_str(&encoded).expect("third-party typed decoder");
    let document: PlaybookSpec = dsl::os_pack::json::from_json_str(&encoded).expect("domain decoder");
    assert_eq!(document, oracle);
    let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&document)).expect("domain output");
    assert_eq!(actual, serde_json::to_value(&oracle).expect("third-party typed encoder"));
    let order: Vec<_> = flatten_playbook_blocks(&document).iter().map(|block| block.id.as_str()).collect();
    assert_eq!(serde_json::to_value(order).expect("block order"), fixture["expectedBlockOrder"]);
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    println!("[DEBUG] Standalone playbook package preserves the fixture block order and JSON/DSL/pack forms");
}
