//! 📦️ Portable paged carrier laws checked against independent Serde values.
use super::*;

#[test]
fn portable_paged_carriers_match_independent_serde() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in corpus["vectors"].as_array().unwrap() {
        let input = row["input"].clone();
        let (actual, oracle) = match row["kind"].as_str().unwrap() {
            "text" => (PagedUtf8::<2048>::from_value(DslValue::from(&input)).map(|value| { assert!(value.chunks().all(|chunk| chunk.len() <= PAGED_UTF8_CHUNK_BYTES)); value.to_value() }), serde_json::from_value::<String>(input.clone()).ok().filter(|value| value.len() <= 2048).map(serde_json::Value::String)),
            "list" => (PagedList::<u64, 8>::from_value(DslValue::from(&input)).map(|value| value.to_value()), serde_json::from_value::<Vec<u64>>(input.clone()).ok().filter(|value| value.len() <= 8).map(|value| serde_json::to_value(value).unwrap())),
            "map" => (PagedMap::<u64, 4>::from_value(DslValue::from(&input)).map(|value| value.to_value()), serde_json::from_value::<std::collections::BTreeMap<String, u64>>(input.clone()).ok().filter(|value| value.len() <= 4).map(|value| serde_json::to_value(value).unwrap())),
            _ => unreachable!(),
        };
        assert_eq!(actual.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(actual.is_ok(), oracle.is_some(), "{}", row["id"]);
        if let (Ok(actual), Some(oracle)) = (actual, oracle) { assert_eq!(serde_json::Value::from(actual), oracle); }
    }
}

#[test]
fn paged_array_paths_and_structural_refusal_preserve_owner() {
    let mut value = PagedList::<u64, 8>::from_value(crate::dsl_value!([1u64, 2u64, 3u64])).unwrap();
    for index in ["", "01", "-1", "3"] { assert!(value.value_at_path(&[index]).is_err()); }
    value.edit_value_at_path(&["1"], ValueEdit::Set(crate::dsl_value!(7u64))).unwrap();
    assert_eq!(value.value_at_path(&["1"]).unwrap(), crate::dsl_value!(7u64));
    let expected = value.to_value();
    assert!(value.edit_value_at_path(&["1"], ValueEdit::Remove).is_err());
    assert_eq!(value.to_value(), expected);
}

#[test]
fn paged_map_preserves_insertion_order_and_refuses_duplicate_owners() {
    let input = DslValue::Object(vec![("beta".into(), crate::dsl_value!(2u64)), ("alpha".into(), crate::dsl_value!(1u64))]);
    let mut value = PagedMap::<u64, 4>::from_value(input.clone()).unwrap();
    assert_eq!(value.keys().map(String::as_str).collect::<Vec<_>>(), vec!["beta", "alpha"]);
    assert_eq!(value.value_key_at_path(&[], 0).unwrap(), "beta");
    assert!(value.edit_value_at_path(&["alpha"], ValueEdit::Remove).is_err());
    assert_eq!(value.to_value(), input);
    assert!(PagedMap::<u64, 4>::from_value(DslValue::Object(vec![("same".into(), crate::dsl_value!(1u64)), ("same".into(), crate::dsl_value!(2u64))])).is_err());
}
