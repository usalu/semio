use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn enc_u32(v: &u32) -> String {
    v.to_string()
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn dec_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn enc_str(v: &String) -> String {
    v.clone()
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    Ok(s.to_string())
}

#[semio_framework_async_macros::async_test]
async fn indexed_triple_round_trips_through_hex_shape() {
    let diff: IndexedTripleDiff<u32, String> = IndexedTripleDiff { removed: vec![2, 5], modified: vec![IndexModified { index: 1, diff: 7 }], added: vec![IndexAdded { index: 3, item: "new".to_string() }] };
    let encoded = enc_indexed_triple(&diff, enc_u32, enc_str);
    let decoded = dec_indexed_triple(&encoded, dec_u32, dec_str).expect("decode");
    assert_eq!(decoded, diff);
}

#[semio_framework_async_macros::async_test]
async fn named_triple_round_trips_through_hex_shape() {
    let diff: NamedTripleDiff<String, u32, String> = NamedTripleDiff { removed: vec!["gone".to_string()], modified: vec![NamedModified { key: "kept".to_string(), diff: 9 }], added: vec!["fresh".to_string()] };
    let encoded = enc_named_triple(&diff, enc_str, enc_u32, enc_str);
    let decoded = dec_named_triple(&encoded, dec_str, dec_u32, dec_str).expect("decode");
    assert_eq!(decoded, diff);
}

#[semio_framework_async_macros::async_test]
async fn named_added_round_trips_through_hex_shape() {
    let diff: NamedTripleDiff<String, u32, NamedAdded<String>> = NamedTripleDiff { removed: vec![], modified: vec![], added: vec![NamedAdded { index: 2, item: "reinserted".to_string() }] };
    let encoded = enc_named_triple(&diff, enc_str, enc_u32, |a| enc_named_added(a, enc_str));
    let decoded = dec_named_triple(&encoded, dec_str, dec_u32, |s| dec_named_added(s, dec_str)).expect("decode");
    assert_eq!(decoded, diff);
}

/// 🩹 A non-`Default` item type proves the auto-synthesized `T: ToValue + FromValue` bound
/// actually works standalone (no `Default` needed): `NoDefault` is a single-field tuple struct,
/// so it derives via `#[value(transparent)]` (forwards straight to/from its own `u32` field).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(transparent)]
struct NoDefault(u32);

#[semio_framework_async_macros::async_test]
async fn json_round_trips_a_non_default_item_type() {
    let diff: NamedTripleDiff<String, NoDefault, NoDefault> = NamedTripleDiff { removed: vec!["gone".to_string()], modified: vec![NamedModified { key: "kept".to_string(), diff: NoDefault(9) }], added: vec![NoDefault(3)] };
    let json = pack::to_json_string(&diff);
    let decoded: NamedTripleDiff<String, NoDefault, NoDefault> = pack::from_json_str(&json).expect("deserialize");
    assert_eq!(decoded, diff);

    let idiff: IndexedTripleDiff<NoDefault, NoDefault> = IndexedTripleDiff { removed: vec![1], modified: vec![IndexModified { index: 0, diff: NoDefault(5) }], added: vec![IndexAdded { index: 2, item: NoDefault(7) }] };
    let ijson = pack::to_json_string(&idiff);
    let idecoded: IndexedTripleDiff<NoDefault, NoDefault> = pack::from_json_str(&ijson).expect("deserialize");
    assert_eq!(idecoded, idiff);
}

#[semio_framework_async_macros::async_test]
async fn empty_triples_round_trip_to_empty_brackets() {
    let diff: IndexedTripleDiff<u32, String> = IndexedTripleDiff::default();
    let encoded = enc_indexed_triple(&diff, enc_u32, enc_str);
    assert_eq!(encoded, "[];[];[]");
    let decoded = dec_indexed_triple(&encoded, dec_u32, dec_str).expect("decode");
    assert_eq!(decoded, diff);
}

#[semio_framework_async_macros::async_test]
async fn nested_bracket_payload_does_not_confuse_the_top_level_split() {
    // 🧪️ Depth-awareness proof: an item whose own encoding contains "[a,b]" must not be torn
    // apart by the outer added-list comma split.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn enc_pair(v: &(u32, u32)) -> String {
        format!("[{},{}]", v.0, v.1)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dec_pair(s: &str) -> Result<(u32, u32), String> {
        let inner = strip_brackets(s)?;
        let parts = split_top_level(inner, ',');
        let [a, b] = parts.as_slice() else { return Err("expected 2 fields".to_string()) };
        Ok((dec_u32(a)?, dec_u32(b)?))
    }
    let diff: IndexedTripleDiff<u32, (u32, u32)> = IndexedTripleDiff { removed: vec![], modified: vec![], added: vec![IndexAdded { index: 0, item: (1, 2) }, IndexAdded { index: 1, item: (3, 4) }] };
    let encoded = enc_indexed_triple(&diff, enc_u32, enc_pair);
    let decoded = dec_indexed_triple(&encoded, dec_u32, dec_pair).expect("decode");
    assert_eq!(decoded, diff);
}

#[semio_framework_async_macros::async_test]
async fn indexed_preflight_rejects_missing_and_clamped_targets() {
    let missing: IndexedTripleDiff<(), ()> = IndexedTripleDiff { removed: vec![2], modified: Vec::new(), added: Vec::new() };
    let error = validate_indexed_triple(&missing, 1, ["items"]).unwrap_err();
    assert_eq!(error.code, "mutation.apply.invalid-remove-index");
    assert_eq!(error.target, vec!["items"]);

    let clamped: IndexedTripleDiff<(), ()> = IndexedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec![IndexAdded { index: 2, item: () }] };
    let error = validate_indexed_triple(&clamped, 0, ["items"]).unwrap_err();
    assert_eq!(error.code, "mutation.apply.invalid-add-index");
}

#[semio_framework_async_macros::async_test]
async fn named_preflight_rejects_missing_and_colliding_keys() {
    let missing: NamedTripleDiff<String, (), String> = NamedTripleDiff { removed: Vec::new(), modified: vec![NamedModified { key: "absent".into(), diff: () }], added: Vec::new() };
    let error = validate_named_triple(&["present".to_string()], &missing, Clone::clone, Clone::clone, ["items"]).unwrap_err();
    assert_eq!(error.code, "mutation.apply.invalid-modify-key");

    let collision: NamedTripleDiff<String, (), String> = NamedTripleDiff { removed: Vec::new(), modified: Vec::new(), added: vec!["present".into()] };
    let error = validate_named_triple(&["present".to_string()], &collision, Clone::clone, Clone::clone, ["items"]).unwrap_err();
    assert_eq!(error.code, "mutation.apply.invalid-add-key");
}

/// 🔁️ The whole-collection replacement a reordering `set-snapshot` needs: every base key removed
/// and every target item re-added, which `apply_named` reproduces exactly and this preflight used
/// to reject.
#[semio_framework_async_macros::async_test]
async fn named_preflight_admits_a_removed_key_being_re_added() {
    let base = ["site".to_string(), "building".to_string(), "storey".to_string()];
    let replacement: NamedTripleDiff<String, (), String> = NamedTripleDiff { removed: base.to_vec(), modified: Vec::new(), added: vec!["storey".into(), "site".into()] };
    validate_named_triple(&base, &replacement, Clone::clone, Clone::clone, ["spatial"]).unwrap();

    let mut items = base.to_vec();
    items.retain(|item| !replacement.removed.contains(item));
    items.extend(replacement.added.iter().cloned());
    assert_eq!(items, vec!["storey".to_string(), "site".to_string()]);
}
