//! 🧪️ The paged natural source contract bounds output by its selected extent and admits the large fixture.
use semio_plugin_decl_app::tool_contract::{NATURAL_SOURCE_TOOL_CONTRACT as CONTRACT, admits_paged_natural_request};

fn law() -> serde_json::Value { serde_json::from_str(include_str!("natural-source-limits.json")).unwrap() }

fn number(law: &serde_json::Value, name: &str) -> usize { law[name].as_u64().unwrap() as usize }

#[test]
fn natural_output_is_derived_from_the_selected_extent_and_covers_the_fixture() {
    let law = law();
    assert_eq!(CONTRACT.max_output_bytes, number(&law, "naturalOutputBytes"));
    assert_eq!(CONTRACT.max_output_bytes, number(&law, "selectedMaximumBytes") * number(&law, "outputPerSourceByte"));
    assert!(number(&law, "imageBytes") <= CONTRACT.max_output_bytes);
    assert!(number(&law, "sourceBytes") > number(&law, "ordinaryOutputBytes"));
    assert_eq!(CONTRACT.max_raw_wire_bytes, number(&law, "naturalRawWireBytes"));
    assert_eq!(CONTRACT.max_decoded_items, number(&law, "naturalDecodedItems"));
    assert_eq!(CONTRACT.max_work_units_per_step, number(&law, "naturalWorkUnitsPerStep") as u64);
}

#[test]
fn natural_admission_requires_the_exact_key_schema_contract_and_an_empty_raw_body() {
    use semio_plugin_decl_app::tool_contract::{NATURAL_SOURCE_TOOL_ID as ID, NATURAL_SOURCE_TOOL_SCHEMA as SCHEMA};
    assert!(admits_paged_natural_request(ID, SCHEMA, CONTRACT, 0));
    assert!(!admits_paged_natural_request("import-media", SCHEMA, CONTRACT, 0));
    assert!(!admits_paged_natural_request(ID, "framework.reserved.import-media.v1", CONTRACT, 0));
    assert!(!admits_paged_natural_request(ID, SCHEMA, CONTRACT, 1));
}
