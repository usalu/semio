use super::*;
use crate::STDIO_JSON_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap(value: JsonValue) -> JsonSnapshot {
    JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn arr(items: Vec<JsonValue>) -> JsonValue {
    JsonValue::Array { items }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn objv(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
    JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn num(lexeme: &str) -> JsonValue {
    JsonValue::Number { lexeme: lexeme.into() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn str_(s: &str) -> JsonValue {
    JsonValue::String { value: s.into() }
}

//#region inverse_law
#[test]
fn inverse_law_diff_level() {
    let a = objv(vec![("x", num("1")), ("y", arr(vec![num("1"), num("2")]))]);
    let b = objv(vec![("x", num("2")), ("z", str_("new"))]);
    let (sa, sb) = (snap(a), snap(b));
    let d = object_diff(JsonObjectDiff {
        removed: vec!["y".into()],
        modified: vec![JsonObjectModified { key: "x".into(), diff: JsonValueDiff::Number { lexeme: "2".into() } }],
        added: vec![JsonObjectAdded { index: 1, key: "z".into(), item: str_("new") }],
    });
    let mid = protocol::apply_diff(&d, &sa).unwrap();
    assert_eq!(mid, sb);
    let inv = d.inverse(&sa);
    assert_eq!(protocol::apply_diff(&inv, &mid).unwrap(), sa);
}
//#endregion inverse_law

//#region absorb_law canonical cases (array/index-keyed)
// NOTE: these construct `d1`/`d2` DIRECTLY as genuine Insert/Remove/Modify array diffs (matching exactly what
// `JsonMutation::InsertArrayElement`/`RemoveArrayElement`/`SetScalar` would produce).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn array_diff(d: JsonArrayDiff) -> JsonDiff {
    JsonDiff { value: Some(JsonValueDiff::Array { diff: d }) }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn object_diff(d: JsonObjectDiff) -> JsonDiff {
    JsonDiff { value: Some(JsonValueDiff::Object { diff: d }) }
}

#[test]
fn absorb_array_insert_then_remove_before() {
    // base = [a,b,c]; d1 = Insert(2,f) -> mid=[a,b,f,c]; d2 = Remove(0) -> after=[b,f,c].
    let base = snap(arr(vec![str_("a"), str_("b"), str_("c")]));
    let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 2, item: str_("f") }], ..Default::default() });
    let d2 = array_diff(JsonArrayDiff { removed: vec![0], ..Default::default() });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).unwrap(), sequential);
    assert_eq!(sequential.value, arr(vec![str_("b"), str_("f"), str_("c")]));
    match &combined.value {
        Some(JsonValueDiff::Array { diff }) => {
            assert_eq!(diff.removed, vec![0]);
            assert_eq!(diff.added, vec![JsonArrayAdded { index: 1, item: str_("f") }]);
        }
        other => panic!("expected array diff, got {other:?}"),
    }
}

#[test]
fn absorb_array_insert_insert_same_index_both_survive() {
    // base = [a,b]; d1 = Insert(2,f); d2 = Insert(2,g) (against mid=[a,b,f]) -> [a,b,g,f].
    let base = snap(arr(vec![str_("a"), str_("b")]));
    let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 2, item: str_("f") }], ..Default::default() });
    let d2 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 2, item: str_("g") }], ..Default::default() });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).unwrap(), sequential);
    assert_eq!(sequential.value, arr(vec![str_("a"), str_("b"), str_("g"), str_("f")]));
    match &combined.value {
        Some(JsonValueDiff::Array { diff }) => assert_eq!(diff.added.len(), 2, "both inserts must survive"),
        other => panic!("expected array diff, got {other:?}"),
    }
}

#[test]
fn absorb_array_insert_then_remove_of_same_added_item_cancels() {
    // base = [a]; d1 = Insert(1,f) -> mid=[a,f]; d2 = Remove(1) -> after=[a].
    let base = snap(arr(vec![str_("a")]));
    let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 1, item: str_("f") }], ..Default::default() });
    let d2 = array_diff(JsonArrayDiff { removed: vec![1], ..Default::default() });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).unwrap(), sequential);
    assert_eq!(sequential, base);
    assert!(combined.is_empty(), "cancelling insert+remove must coalesce to an empty diff");
}

#[test]
fn absorb_array_add_then_setfield_patches_added_payload() {
    // base = []; d1 = Insert(0,{x:1}) -> mid=[{x:1}]; d2 = SetMember([0],y,2) -> [{x:1,y:2}].
    let base = snap(arr(vec![]));
    let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 0, item: objv(vec![("x", num("1"))]) }], ..Default::default() });
    let d2 = array_diff(JsonArrayDiff {
        modified: vec![JsonArrayModified { index: 0, diff: JsonValueDiff::Object { diff: JsonObjectDiff { added: vec![JsonObjectAdded { index: 1, key: "y".into(), item: num("2") }], ..Default::default() } } }],
        ..Default::default()
    });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).unwrap(), sequential);
    assert_eq!(sequential.value, arr(vec![objv(vec![("x", num("1")), ("y", num("2"))])]));
    match &combined.value {
        Some(JsonValueDiff::Array { diff }) => {
            assert!(diff.modified.is_empty(), "the patch must land INSIDE the carried added payload, not as a separate modified entry");
            assert_eq!(diff.added.len(), 1);
            assert_eq!(diff.added[0].item, objv(vec![("x", num("1")), ("y", num("2"))]));
        }
        other => panic!("expected array diff, got {other:?}"),
    }
}

#[test]
fn absorb_array_modify_then_remove_drops_pending_patch() {
    // base = [1,2]; d1 = Modify(0,9) -> mid=[9,2]; d2 = Remove(0) -> after=[2].
    let base = snap(arr(vec![num("1"), num("2")]));
    let d1 = array_diff(JsonArrayDiff { modified: vec![JsonArrayModified { index: 0, diff: JsonValueDiff::Number { lexeme: "9".into() } }], ..Default::default() });
    let d2 = array_diff(JsonArrayDiff { removed: vec![0], ..Default::default() });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).unwrap(), sequential);
    assert_eq!(sequential.value, arr(vec![num("2")]));
    match &combined.value {
        Some(JsonValueDiff::Array { diff }) => {
            assert_eq!(diff.removed, vec![0]);
            assert!(diff.modified.is_empty(), "the pending modify on the removed base index must be dropped");
        }
        other => panic!("expected array diff, got {other:?}"),
    }
}

#[test]
fn absorb_array_associativity() {
    let s0 = snap(arr(vec![num("1"), num("2"), num("3")]));
    let s1 = snap(arr(vec![num("1"), num("9"), num("3")]));
    let s2 = snap(arr(vec![num("9"), num("3"), num("4")]));
    let s3 = snap(arr(vec![num("9"), num("4")]));
    let d1 = array_diff(JsonArrayDiff { modified: vec![JsonArrayModified { index: 1, diff: JsonValueDiff::Number { lexeme: "9".into() } }], ..Default::default() });
    let d2 = array_diff(JsonArrayDiff { removed: vec![0], added: vec![JsonArrayAdded { index: 2, item: num("4") }], ..Default::default() });
    let d3 = array_diff(JsonArrayDiff { removed: vec![1], ..Default::default() });

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut right_tail = d2.clone();
    right_tail.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(right_tail);

    assert_eq!(protocol::apply_diff(&left, &s0).unwrap(), s3);
    assert_eq!(protocol::apply_diff(&right, &s0).unwrap(), s3);
    assert_eq!(left, right);
}
//#endregion absorb_law canonical cases (array/index-keyed)

//#region absorb_law canonical cases (object/name-keyed)
#[test]
fn absorb_object_add_then_setfield_patches_added_payload() {
    let base = objv(vec![]);
    let mid = objv(vec![("config", objv(vec![]))]);
    let after = objv(vec![("config", objv(vec![("x", num("5"))]))]);
    let (sbase, smid, safter) = (snap(base), snap(mid), snap(after.clone()));
    let d1 = object_diff(JsonObjectDiff { added: vec![JsonObjectAdded { index: 0, key: "config".into(), item: objv(vec![]) }], ..Default::default() });
    let d2 = object_diff(JsonObjectDiff {
        modified: vec![JsonObjectModified { key: "config".into(), diff: JsonValueDiff::Object { diff: JsonObjectDiff { added: vec![JsonObjectAdded { index: 0, key: "x".into(), item: num("5") }], ..Default::default() } } }],
        ..Default::default()
    });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &sbase).unwrap(), snap(after));
    match &combined.value {
        Some(JsonValueDiff::Object { diff }) => {
            assert!(diff.modified.is_empty());
            assert_eq!(diff.added.len(), 1);
            assert_eq!(diff.added[0].item, objv(vec![("x", num("5"))]));
        }
        other => panic!("expected object diff, got {other:?}"),
    }
}

#[test]
fn absorb_object_modify_then_remove_drops_pending_patch() {
    let base = objv(vec![("a", num("1")), ("b", num("2"))]);
    let mid = objv(vec![("a", num("9")), ("b", num("2"))]);
    let after = objv(vec![("b", num("2"))]);
    let (sbase, smid, safter) = (snap(base), snap(mid), snap(after));
    let d1 = object_diff(JsonObjectDiff { modified: vec![JsonObjectModified { key: "a".into(), diff: JsonValueDiff::Number { lexeme: "9".into() } }], ..Default::default() });
    let d2 = object_diff(JsonObjectDiff { removed: vec!["a".into()], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &sbase).unwrap(), safter);
    match &combined.value {
        Some(JsonValueDiff::Object { diff }) => {
            assert_eq!(diff.removed, vec!["a".to_string()]);
            assert!(diff.modified.is_empty());
        }
        other => panic!("expected object diff, got {other:?}"),
    }
}

#[test]
fn absorb_object_insert_insert_both_survive() {
    let base = objv(vec![("a", num("1"))]);
    let mid = objv(vec![("a", num("1")), ("f", num("2"))]);
    let after = objv(vec![("a", num("1")), ("f", num("2")), ("g", num("3"))]);
    let (sbase, smid, safter) = (snap(base), snap(mid), snap(after.clone()));
    let d1 = object_diff(JsonObjectDiff { added: vec![JsonObjectAdded { index: 1, key: "f".into(), item: num("2") }], ..Default::default() });
    let d2 = object_diff(JsonObjectDiff { added: vec![JsonObjectAdded { index: 2, key: "g".into(), item: num("3") }], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &sbase).unwrap(), snap(after));
    match &combined.value {
        Some(JsonValueDiff::Object { diff }) => assert_eq!(diff.added.len(), 2),
        other => panic!("expected object diff, got {other:?}"),
    }
}

#[test]
fn absorb_object_insert_then_remove_of_same_added_item_cancels() {
    let base = objv(vec![("a", num("1"))]);
    let mid = objv(vec![("a", num("1")), ("f", num("2"))]);
    let after = objv(vec![("a", num("1"))]);
    let (sbase, smid, safter) = (snap(base.clone()), snap(mid), snap(after));
    let d1 = object_diff(JsonObjectDiff { added: vec![JsonObjectAdded { index: 1, key: "f".into(), item: num("2") }], ..Default::default() });
    let d2 = object_diff(JsonObjectDiff { removed: vec!["f".into()], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &sbase).unwrap(), snap(base));
    assert!(combined.is_empty());
}

#[test]
fn absorb_object_associativity() {
    let s0 = snap(objv(vec![("a", num("1"))]));
    let s1 = snap(objv(vec![("a", num("1")), ("b", num("2"))]));
    let s2 = snap(objv(vec![("a", num("9")), ("b", num("2"))]));
    let s3 = snap(objv(vec![("b", num("2")), ("c", num("3"))]));
    let d1 = object_diff(JsonObjectDiff { added: vec![JsonObjectAdded { index: 1, key: "b".into(), item: num("2") }], ..Default::default() });
    let d2 = object_diff(JsonObjectDiff { modified: vec![JsonObjectModified { key: "a".into(), diff: JsonValueDiff::Number { lexeme: "9".into() } }], ..Default::default() });
    let d3 = object_diff(JsonObjectDiff { removed: vec!["a".into()], added: vec![JsonObjectAdded { index: 1, key: "c".into(), item: num("3") }], ..Default::default() });

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut right_tail = d2.clone();
    right_tail.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(right_tail);

    assert_eq!(protocol::apply_diff(&left, &s0).unwrap(), s3);
    assert_eq!(protocol::apply_diff(&right, &s0).unwrap(), s3);
    assert_eq!(left, right);
}
//#endregion absorb_law canonical cases (object/name-keyed)

//#region field_sweep
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> JsonSnapshot {
    snap(objv(vec![
        ("keepBool", JsonValue::Bool { value: true }),
        ("keepNumber", num("1")),
        ("keepString", str_("base")),
        ("kindChange", num("1")),
        ("nullToValue", JsonValue::Null),
        ("removedMember", str_("gone")),
        ("modifiedMember", num("10")),
        ("nestedArray", arr(vec![num("1"), num("2"), num("3")])),
        ("nestedObject", objv(vec![("inner", str_("x"))])),
    ]))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> JsonSnapshot {
    snap(objv(vec![
        ("keepBool", JsonValue::Bool { value: false }),
        ("keepNumber", num("2.5e3")),
        ("keepString", str_("changed")),
        ("kindChange", str_("now a string")),
        ("nullToValue", JsonValue::Bool { value: true }),
        ("modifiedMember", num("99")),
        ("nestedArray", arr(vec![num("1"), num("20"), num("30"), num("4")])),
        ("nestedObject", objv(vec![("inner", str_("y")), ("extra", JsonValue::Bool { value: true })])),
        ("addedMember", str_("new")),
    ]))
}

//#endregion field_sweep

//#region 🔖️HandcraftedDiffCodecTests
/// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `JsonDiff` grammar — exercises
/// every `JsonValueDiff` variant (incl. the `Replace` kind-change fallback), nested
/// array/object collection triples, and the empty (`None`) diff.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    use protocol::{DiffBinary,DiffCodec,DiffText};

    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = JsonDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = JsonDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
//#endregion 🔖️HandcraftedDiffCodecTests
