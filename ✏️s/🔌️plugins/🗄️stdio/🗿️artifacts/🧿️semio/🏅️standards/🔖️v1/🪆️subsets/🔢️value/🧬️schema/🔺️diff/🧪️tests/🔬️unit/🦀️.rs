use super::*;
use crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
use std::collections::HashMap;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn listv(items: Vec<SemioValue>) -> SemioValue {
    SemioValue::List { items }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
    SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioValueEntry { key: k.into(), value: v }).collect() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intv(lexeme: &str) -> SemioValue {
    SemioValue::Int { lexeme: lexeme.into() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn floatv(lexeme: &str) -> SemioValue {
    SemioValue::Float { lexeme: lexeme.into() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn strv(s: &str) -> SemioValue {
    SemioValue::Str { value: s.into() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn refv(id: &str) -> SemioValue {
    SemioValue::Ref { id: ValueId::new(id) }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn node(id: &str, value: SemioValue) -> SemioValueNode {
    SemioValueNode { id: ValueId::new(id), value }
}

//#endregion inverse_law

//#region absorb_law canonical cases (list/index-keyed)
// NOTE: these construct `d1`/`d2` DIRECTLY as genuine Insert/Remove/Modify list diffs (matching
// exactly what `InsertListItem`/`RemoveListItem`/`SetValue` would produce).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn list_diff(d: IndexedTripleDiff<SemioValueDiff, SemioValue>) -> SemioValueTreeDiff {
    SemioValueTreeDiff { root: Some(SemioValueDiff::List { diff: d }), nodes: None }
}

#[test]
fn absorb_list_insert_then_remove_before() {
    // base = [a,b,c]; d1 = Insert(2,f) -> mid=[a,b,f,c]; d2 = Remove(0) -> after=[b,f,c].
    let base = snap(listv(vec![strv("a"), strv("b"), strv("c")]), vec![]);
    let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("f") }], ..Default::default() });
    let d2 = list_diff(IndexedTripleDiff { removed: vec![0], ..Default::default() });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), sequential);
    assert_eq!(sequential.root, listv(vec![strv("b"), strv("f"), strv("c")]));
    match &combined.root {
        Some(SemioValueDiff::List { diff }) => {
            assert_eq!(diff.removed, vec![0]);
            assert_eq!(diff.added, vec![IndexAdded { index: 1, item: strv("f") }]);
        }
        other => panic!("expected list diff, got {other:?}"),
    }
}

#[test]
fn absorb_list_insert_insert_same_index_both_survive() {
    let base = snap(listv(vec![strv("a"), strv("b")]), vec![]);
    let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("f") }], ..Default::default() });
    let d2 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: strv("g") }], ..Default::default() });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), sequential);
    assert_eq!(sequential.root, listv(vec![strv("a"), strv("b"), strv("g"), strv("f")]));
    match &combined.root {
        Some(SemioValueDiff::List { diff }) => assert_eq!(diff.added.len(), 2, "both inserts must survive"),
        other => panic!("expected list diff, got {other:?}"),
    }
}

#[test]
fn absorb_list_insert_then_remove_of_same_added_item_cancels() {
    let base = snap(listv(vec![strv("a")]), vec![]);
    let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: strv("f") }], ..Default::default() });
    let d2 = list_diff(IndexedTripleDiff { removed: vec![1], ..Default::default() });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), sequential);
    assert_eq!(sequential, base);
    assert!(combined.is_empty(), "cancelling insert+remove must coalesce to an empty diff");
}

#[test]
fn absorb_list_add_then_setfield_patches_added_payload() {
    let base = snap(listv(vec![]), vec![]);
    let d1 = list_diff(IndexedTripleDiff { added: vec![IndexAdded { index: 0, item: mapv(vec![("x", intv("1"))]) }], ..Default::default() });
    let d2 = list_diff(IndexedTripleDiff {
        modified: vec![IndexModified { index: 0, diff: SemioValueDiff::Map { diff: NamedTripleDiff { added: vec![NamedAdded { index: 1, item: SemioValueEntry { key: "y".into(), value: intv("2") } }], ..Default::default() } } }],
        ..Default::default()
    });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), sequential);
    assert_eq!(sequential.root, listv(vec![mapv(vec![("x", intv("1")), ("y", intv("2"))])]));
    match &combined.root {
        Some(SemioValueDiff::List { diff }) => {
            assert!(diff.modified.is_empty(), "the patch must land INSIDE the carried added payload, not as a separate modified entry");
            assert_eq!(diff.added.len(), 1);
            assert_eq!(diff.added[0].item, mapv(vec![("x", intv("1")), ("y", intv("2"))]));
        }
        other => panic!("expected list diff, got {other:?}"),
    }
}

#[test]
fn absorb_list_modify_then_remove_drops_pending_patch() {
    let base = snap(listv(vec![intv("1"), intv("2")]), vec![]);
    let d1 = list_diff(IndexedTripleDiff { modified: vec![IndexModified { index: 0, diff: SemioValueDiff::Int { lexeme: "9".into() } }], ..Default::default() });
    let d2 = list_diff(IndexedTripleDiff { removed: vec![0], ..Default::default() });
    let sequential = protocol::apply_diff(&d2, &protocol::apply_diff(&d1, &base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), sequential);
    assert_eq!(sequential.root, listv(vec![intv("2")]));
    match &combined.root {
        Some(SemioValueDiff::List { diff }) => {
            assert_eq!(diff.removed, vec![0]);
            assert!(diff.modified.is_empty(), "the pending modify on the removed base index must be dropped");
        }
        other => panic!("expected list diff, got {other:?}"),
    }
}

//#endregion absorb_law canonical cases (list/index-keyed)

//#region absorb_law canonical cases (map/name-keyed)
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn map_diff(d: NamedTripleDiff<String, SemioValueDiff, NamedAdded<SemioValueEntry>>) -> SemioValueTreeDiff {
    SemioValueTreeDiff { root: Some(SemioValueDiff::Map { diff: d }), nodes: None }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn nodes_diff(d: NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>>) -> SemioValueTreeDiff {
    SemioValueTreeDiff { root: None, nodes: Some(d) }
}

#[test]
fn absorb_map_add_then_setfield_patches_added_payload() {
    let base = snap(mapv(vec![]), vec![]);
    let after = snap(mapv(vec![("config", mapv(vec![("x", intv("5"))]))]), vec![]);
    let d1 = map_diff(NamedTripleDiff { added: vec![NamedAdded { index: 0, item: SemioValueEntry { key: "config".into(), value: mapv(vec![]) } }], ..Default::default() });
    let d2 = map_diff(NamedTripleDiff { modified: vec![NamedModified { key: "config".to_string(), diff: SemioValueDiff::Map { diff: NamedTripleDiff { added: vec![NamedAdded { index: 0, item: SemioValueEntry { key: "x".into(), value: intv("5") } }], ..Default::default() } } }], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), after);
    match &combined.root {
        Some(SemioValueDiff::Map { diff }) => {
            assert!(diff.modified.is_empty());
            assert_eq!(diff.added.len(), 1);
            assert_eq!(diff.added[0].item.value, mapv(vec![("x", intv("5"))]));
        }
        other => panic!("expected map diff, got {other:?}"),
    }
}

#[test]
fn absorb_map_modify_then_remove_drops_pending_patch() {
    let base = snap(mapv(vec![("a", intv("1")), ("b", intv("2"))]), vec![]);
    let after = snap(mapv(vec![("b", intv("2"))]), vec![]);
    let d1 = map_diff(NamedTripleDiff { modified: vec![NamedModified { key: "a".to_string(), diff: SemioValueDiff::Int { lexeme: "9".into() } }], ..Default::default() });
    let d2 = map_diff(NamedTripleDiff { removed: vec!["a".to_string()], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), after);
    match &combined.root {
        Some(SemioValueDiff::Map { diff }) => {
            assert_eq!(diff.removed, vec!["a".to_string()]);
            assert!(diff.modified.is_empty());
        }
        other => panic!("expected map diff, got {other:?}"),
    }
}

#[test]
fn absorb_map_insert_insert_both_survive() {
    let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
    let after = snap(mapv(vec![("a", intv("1")), ("f", intv("2")), ("g", intv("3"))]), vec![]);
    let d1 = map_diff(NamedTripleDiff { added: vec![NamedAdded { index: 1, item: SemioValueEntry { key: "f".into(), value: intv("2") } }], ..Default::default() });
    let d2 = map_diff(NamedTripleDiff { added: vec![NamedAdded { index: 2, item: SemioValueEntry { key: "g".into(), value: intv("3") } }], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), after);
    match &combined.root {
        Some(SemioValueDiff::Map { diff }) => assert_eq!(diff.added.len(), 2),
        other => panic!("expected map diff, got {other:?}"),
    }
}

#[test]
fn absorb_map_insert_then_remove_of_same_added_item_cancels() {
    let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
    let d1 = map_diff(NamedTripleDiff { added: vec![NamedAdded { index: 1, item: SemioValueEntry { key: "f".into(), value: intv("2") } }], ..Default::default() });
    let d2 = map_diff(NamedTripleDiff { removed: vec!["f".to_string()], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), base);
    assert!(combined.is_empty());
}

//#endregion absorb_law canonical cases (map/name-keyed)

//#region absorb_law canonical cases (nodes graph / id-keyed)
#[test]
fn absorb_nodes_add_then_setfield_patches_added_payload() {
    let base = snap(SemioValue::Null, vec![]);
    let after = snap(SemioValue::Null, vec![node("n1", mapv(vec![("x", intv("5"))]))]);
    let d1 = nodes_diff(NamedTripleDiff { added: vec![NamedAdded { index: 0, item: node("n1", mapv(vec![])) }], ..Default::default() });
    let d2 = nodes_diff(NamedTripleDiff { modified: vec![NamedModified { key: ValueId::new("n1"), diff: SemioValueDiff::Map { diff: NamedTripleDiff { added: vec![NamedAdded { index: 0, item: SemioValueEntry { key: "x".into(), value: intv("5") } }], ..Default::default() } } }], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), after);
    match &combined.nodes {
        Some(diff) => {
            assert!(diff.modified.is_empty());
            assert_eq!(diff.added.len(), 1);
            assert_eq!(diff.added[0].item.value, mapv(vec![("x", intv("5"))]));
        }
        None => panic!("expected an nodes diff"),
    }
}

#[test]
fn absorb_nodes_modify_then_remove_drops_pending_patch() {
    let base = snap(SemioValue::Null, vec![node("a", intv("1")), node("b", intv("2"))]);
    let after = snap(SemioValue::Null, vec![node("b", intv("2"))]);
    let d1 = nodes_diff(NamedTripleDiff { modified: vec![NamedModified { key: ValueId::new("a"), diff: SemioValueDiff::Int { lexeme: "9".into() } }], ..Default::default() });
    let d2 = nodes_diff(NamedTripleDiff { removed: vec![ValueId::new("a")], ..Default::default() });
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(protocol::apply_diff(&combined, &base).expect("apply must succeed for a well-formed fixture"), after);
    match &combined.nodes {
        Some(diff) => {
            assert_eq!(diff.removed, vec![ValueId::new("a")]);
            assert!(diff.modified.is_empty());
        }
        None => panic!("expected an nodes diff"),
    }
}

//#endregion absorb_law canonical cases (nodes graph / id-keyed)

//#region field_sweep
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> SemioValueSnapshot {
    snap(
        mapv(vec![
            ("keepBool", SemioValue::Bool { value: true }),
            ("keepInt", intv("1")),
            ("keepFloat", floatv("1.5")),
            ("keepStr", strv("base")),
            ("keepBytes", SemioValue::Bytes { value: vec![1, 2, 3] }),
            ("keepRef", refv("n1")),
            ("kindChange", intv("1")),
            ("nullToValue", SemioValue::Null),
            ("removedMember", strv("gone")),
            ("modifiedMember", intv("10")),
            ("nestedList", listv(vec![intv("1"), intv("2"), intv("3")])),
            ("nestedMap", mapv(vec![("inner", strv("x"))])),
        ]),
        vec![node("n1", strv("kept")), node("n2", strv("removed-node")), node("n3", intv("10"))],
    )
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> SemioValueSnapshot {
    snap(
        mapv(vec![
            ("keepBool", SemioValue::Bool { value: false }),
            ("keepInt", intv("2")),
            ("keepFloat", floatv("2.75e3")),
            ("keepStr", strv("changed")),
            ("keepBytes", SemioValue::Bytes { value: vec![4, 5] }),
            ("keepRef", refv("n3")),
            ("kindChange", strv("now a string")),
            ("nullToValue", SemioValue::Bool { value: true }),
            ("modifiedMember", intv("99")),
            ("nestedList", listv(vec![intv("1"), intv("20"), intv("30"), intv("4")])),
            ("nestedMap", mapv(vec![("inner", strv("y")), ("extra", SemioValue::Bool { value: true })])),
            ("addedMember", strv("new")),
        ]),
        vec![node("n1", strv("kept")), node("n3", intv("99")), node("n4", strv("added-node"))],
    )
}

//#endregion field_sweep

//#region 🔖️HandcraftedDiffCodecTests
/// 🧪️ diff_codec_text_binary_roundtrip_law: exercises every `SemioValueDiff` variant (incl.
/// the `Replace` kind-change fallback), nested list/map/nodes-graph collection triples, and
/// the empty (`None`/`None`) diff.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    use protocol::{DiffBinary,DiffCodec,DiffText};

    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioValueTreeDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioValueTreeDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
//#endregion 🔖️HandcraftedDiffCodecTests
