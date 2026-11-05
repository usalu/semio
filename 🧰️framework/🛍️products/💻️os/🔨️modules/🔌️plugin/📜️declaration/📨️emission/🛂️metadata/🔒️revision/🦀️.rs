//! 🔒️ Staged commits retain the complete canonical document frontier.
#[test]
fn staged_revision_preconditions_match_the_native_frontier() {
    let cases: serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let decode=|value:&serde_json::Value|{let text=value.as_str().unwrap();let mut bytes=[0u8;32];for (i,pair) in text.as_bytes().chunks_exact(2).enumerate(){bytes[i]=u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap();}bytes};
        let meta=crate::ActionMeta{instance_id:row["instance"].as_u64().unwrap() as u32,expected_document_revision:row["expected"].as_str().map(|_|decode(&row["expected"])),..Default::default()};
        let live=row["live"].as_u64().map(|value|value as u32);
        assert_eq!(meta.admit_document_revision(&decode(&row["current"]),live).is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["name"]);
    }
}
