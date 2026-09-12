fn verify_empty_state<T>()
where
    T: dsl::FromValue + store::ArtifactDsl + store::ArtifactPack + Default + PartialEq + std::fmt::Debug,
{
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("empty-state fixture");
    for row in fixture["json"].as_array().expect("JSON vectors") {
        assert_eq!(dsl::json::from_json_str::<T>(&row["value"].to_string()).is_ok(), row["accepted"].as_bool().expect("acceptance"), "{}: JSON {}", std::any::type_name::<T>(), row["value"]);
    }
    for row in fixture["text"].as_array().expect("text vectors") {
        assert_eq!(T::parse_dsl(row["value"].as_str().expect("text")).is_ok(), row["accepted"].as_bool().expect("acceptance"), "{}: text {}", std::any::type_name::<T>(), row["value"]);
    }
    for row in fixture["pack"].as_array().expect("Pack vectors") {
        let bytes: Vec<u8> = row["value"].as_array().expect("bytes").iter().map(|value| u8::try_from(value.as_u64().expect("byte")).expect("u8")).collect();
        assert_eq!(T::decode_pack(&bytes).is_ok(), row["accepted"].as_bool().expect("acceptance"), "{}: Pack {:?}", std::any::type_name::<T>(), bytes);
    }
    let state = T::default();
    assert!(state.encode_pack().is_empty());
    assert_eq!(T::decode_pack(&state.encode_pack()).expect("empty Pack round trip"), state);
}

#[test]
fn framework_empty_state_contract_rejects_foreign_fields_and_bytes() {
    verify_empty_state::<crate::NoConfig>();
    verify_empty_state::<crate::NoDraft>();
    verify_empty_state::<crate::NoPresence>();
    verify_empty_state::<crate::NoTransient>();
    eprintln!("[DEBUG] Framework NoConfig/NoDraft/NoPresence/NoTransient accepted only empty state across 13 neutral admission vectors each");
}

