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

//#region between_roundtrip_law
#[test]
fn between_roundtrip_law_scalars_and_kind_change() {
    let cases = [(JsonValue::Null, JsonValue::Bool { value: true }), (JsonValue::Bool { value: true }, JsonValue::Bool { value: false }), (num("1"), num("2.5e10")), (str_("a"), str_("b")), (num("1"), str_("1"))];
    for (a, b) in cases {
        let (sa, sb) = (snap(a.clone()), snap(b.clone()));
        assert_eq!(JsonDiff::between(&sa, &sb).apply(&sa).unwrap(), sb, "a={a:?} b={b:?}");
        assert_eq!(JsonDiff::between(&sb, &sa).apply(&sb).unwrap(), sa);
    }
}

#[test]
fn between_roundtrip_law_nested_collections() {
    let a = objv(vec![("tags", arr(vec![str_("x"), str_("y")])), ("n", num("1"))]);
    let b = objv(vec![("tags", arr(vec![str_("x"), str_("z"), str_("w")])), ("n", num("2")), ("extra", JsonValue::Bool { value: true })]);
    let (sa, sb) = (snap(a.clone()), snap(b.clone()));
    assert_eq!(JsonDiff::between(&sa, &sb).apply(&sa).unwrap(), sb);
    assert_eq!(JsonDiff::between(&sb, &sa).apply(&sb).unwrap(), sa);
}

#[test]
fn between_self_is_empty() {
    let a = objv(vec![("x", num("1"))]);
    let sa = snap(a);
    assert!(JsonDiff::between(&sa, &sa).is_empty());
}
//#endregion between_roundtrip_law

//#region inverse_law
#[test]
fn inverse_law_diff_level() {
    let a = objv(vec![("x", num("1")), ("y", arr(vec![num("1"), num("2")]))]);
    let b = objv(vec![("x", num("2")), ("z", str_("new"))]);
    let (sa, sb) = (snap(a), snap(b));
    let d = JsonDiff::between(&sa, &sb);
    let mid = d.apply(&sa).unwrap();
    assert_eq!(mid, sb);
    let inv = d.inverse(&sa);
    assert_eq!(inv.apply(&mid).unwrap(), sa);
}
//#endregion inverse_law

//#region absorb_law canonical cases (array/index-keyed)
// NOTE: these construct `d1`/`d2` DIRECTLY as genuine Insert/Remove/Modify array diffs
// (matching exactly what `JsonMutation::InsertArrayElement`/`RemoveArrayElement`/`SetScalar`
// would produce) rather than via `JsonDiff::between(base, next)` — `between` does a PURE
// POSITIONAL comparison (0..min(len)), so a middle-insertion between two concrete array
// VALUES is represented as a same-position `modified` entry plus a tail `added` entry, not as
// a genuine `Insert` — the right, and separately law-tested, behavior for `between`, but the
// wrong fixture shape for exercising the mandated Insert/Remove canonical absorb cases.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn array_diff(d: JsonArrayDiff) -> JsonDiff {
    JsonDiff { value: Some(JsonValueDiff::Array { diff: d }) }
}

#[test]
fn absorb_array_insert_then_remove_before() {
    // base = [a,b,c]; d1 = Insert(2,f) -> mid=[a,b,f,c]; d2 = Remove(0) -> after=[b,f,c].
    let base = snap(arr(vec![str_("a"), str_("b"), str_("c")]));
    let d1 = array_diff(JsonArrayDiff { added: vec![JsonArrayAdded { index: 2, item: str_("f") }], ..Default::default() });
    let d2 = array_diff(JsonArrayDiff { removed: vec![0], ..Default::default() });
    let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).unwrap(), sequential);
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
    let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).unwrap(), sequential);
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
    let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).unwrap(), sequential);
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
    let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).unwrap(), sequential);
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
    let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).unwrap(), sequential);
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
    let d1 = JsonDiff::between(&s0, &s1);
    let d2 = JsonDiff::between(&s1, &s2);
    let d3 = JsonDiff::between(&s2, &s3);

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut right_tail = d2.clone();
    right_tail.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(right_tail);

    assert_eq!(left.apply(&s0).unwrap(), s3);
    assert_eq!(right.apply(&s0).unwrap(), s3);
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
    let d1 = JsonDiff::between(&sbase, &smid);
    let d2 = JsonDiff::between(&smid, &safter);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&sbase).unwrap(), snap(after));
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
    let d1 = JsonDiff::between(&sbase, &smid);
    let d2 = JsonDiff::between(&smid, &safter);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&sbase).unwrap(), safter);
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
    let d1 = JsonDiff::between(&sbase, &smid);
    let d2 = JsonDiff::between(&smid, &safter);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&sbase).unwrap(), snap(after));
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
    let d1 = JsonDiff::between(&sbase, &smid);
    let d2 = JsonDiff::between(&smid, &safter);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&sbase).unwrap(), snap(base));
    assert!(combined.is_empty());
}

#[test]
fn absorb_object_associativity() {
    let s0 = snap(objv(vec![("a", num("1"))]));
    let s1 = snap(objv(vec![("a", num("1")), ("b", num("2"))]));
    let s2 = snap(objv(vec![("a", num("9")), ("b", num("2"))]));
    let s3 = snap(objv(vec![("b", num("2")), ("c", num("3"))]));
    let d1 = JsonDiff::between(&s0, &s1);
    let d2 = JsonDiff::between(&s1, &s2);
    let d3 = JsonDiff::between(&s2, &s3);

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut right_tail = d2.clone();
    right_tail.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(right_tail);

    assert_eq!(left.apply(&s0).unwrap(), s3);
    assert_eq!(right.apply(&s0).unwrap(), s3);
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

#[test]
fn field_sweep_between_roundtrips_both_directions() {
    let (a, b) = (sweep_a(), sweep_b());
    assert_eq!(JsonDiff::between(&a, &b).apply(&a).unwrap(), b);
    assert_eq!(JsonDiff::between(&b, &a).apply(&b).unwrap(), a);
    assert!(JsonDiff::between(&a, &a).is_empty());
}

#[test]
fn field_sweep_every_field_present_in_diff() {
    let (a, b) = (sweep_a(), sweep_b());
    let diff = JsonDiff::between(&a, &b);
    let object_diff = match diff.value {
        Some(JsonValueDiff::Object { diff }) => diff,
        other => panic!("expected a top-level object diff, got {other:?}"),
    };
    assert_eq!(object_diff.removed, vec!["removedMember".to_string()]);
    assert_eq!(object_diff.added.len(), 1);
    assert_eq!(object_diff.added[0].key, "addedMember");

    let by_key: HashMap<&str, &JsonValueDiff> = object_diff.modified.iter().map(|m| (m.key.as_str(), &m.diff)).collect();
    for key in ["keepBool", "keepNumber", "keepString", "kindChange", "nullToValue", "modifiedMember", "nestedArray", "nestedObject"] {
        assert!(by_key.contains_key(key), "expected a modified entry for `{key}`");
    }
    assert!(matches!(by_key["kindChange"], JsonValueDiff::Replace { .. }), "Number->String must fall back to Replace");
    assert!(matches!(by_key["nullToValue"], JsonValueDiff::Replace { .. }), "Null->Bool must fall back to Replace");
    assert!(matches!(by_key["keepBool"], JsonValueDiff::Bool { .. }));
    assert!(matches!(by_key["keepNumber"], JsonValueDiff::Number { .. }));
    assert!(matches!(by_key["keepString"], JsonValueDiff::String { .. }));
    assert!(matches!(by_key["modifiedMember"], JsonValueDiff::Number { .. }));
    match by_key["nestedArray"] {
        JsonValueDiff::Array { diff } => {
            assert!(!diff.modified.is_empty());
            assert!(!diff.added.is_empty());
        }
        other => panic!("expected array diff, got {other:?}"),
    }
    match by_key["nestedObject"] {
        JsonValueDiff::Object { diff } => {
            assert!(!diff.modified.is_empty());
            assert!(!diff.added.is_empty());
        }
        other => panic!("expected object diff, got {other:?}"),
    }
}
//#endregion field_sweep

//#region 🔖️HandcraftedDiffCodecTests
/// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `JsonDiff` grammar — exercises
/// every `JsonValueDiff` variant (incl. the `Replace` kind-change fallback), nested
/// array/object collection triples, and the empty (`None`) diff.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    use protocol::DiffCodec;

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
