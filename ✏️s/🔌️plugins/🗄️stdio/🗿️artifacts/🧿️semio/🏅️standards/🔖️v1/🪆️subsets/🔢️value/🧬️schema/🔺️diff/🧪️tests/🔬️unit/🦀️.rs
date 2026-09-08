
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

//#region between_roundtrip_law
#[test]
fn between_roundtrip_law_scalars_and_kind_change() {
    let cases = [
        (SemioValue::Null, SemioValue::Bool { value: true }),
        (SemioValue::Bool { value: true }, SemioValue::Bool { value: false }),
        (intv("1"), intv("2")),
        (floatv("1.0"), floatv("2.5e10")),
        (strv("a"), strv("b")),
        (SemioValue::Bytes { value: vec![1, 2] }, SemioValue::Bytes { value: vec![3] }),
        (refv("a"), refv("b")),
        (intv("1"), strv("1")),
    ];
    for (a, b) in cases {
        let (sa, sb) = (snap(a.clone(), vec![]), snap(b.clone(), vec![]));
        assert_eq!(SemioValueTreeDiff::between(&sa, &sb).apply(&sa).expect("apply must succeed for a well-formed fixture"), sb, "a={a:?} b={b:?}");
        assert_eq!(SemioValueTreeDiff::between(&sb, &sa).apply(&sb).expect("apply must succeed for a well-formed fixture"), sa);
    }
}

#[test]
fn between_roundtrip_law_nested_collections_and_graph() {
    let a = snap(mapv(vec![("tags", listv(vec![strv("x"), strv("y")])), ("n", intv("1"))]), vec![node("n1", strv("hello"))]);
    let b = snap(mapv(vec![("tags", listv(vec![strv("x"), strv("z"), strv("w")])), ("n", intv("2")), ("extra", refv("n1"))]), vec![node("n1", strv("world")), node("n2", intv("9"))]);
    assert_eq!(SemioValueTreeDiff::between(&a, &b).apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    assert_eq!(SemioValueTreeDiff::between(&b, &a).apply(&b).expect("apply must succeed for a well-formed fixture"), a);
}

#[test]
fn between_self_is_empty() {
    let a = snap(mapv(vec![("x", intv("1"))]), vec![node("n1", strv("v"))]);
    assert!(SemioValueTreeDiff::between(&a, &a).is_empty());
}
//#endregion between_roundtrip_law

//#region inverse_law
#[test]
fn inverse_law_diff_level() {
    let a = snap(mapv(vec![("x", intv("1")), ("y", listv(vec![intv("1"), intv("2")]))]), vec![node("n1", strv("a"))]);
    let b = snap(mapv(vec![("x", intv("2")), ("z", strv("new"))]), vec![node("n1", strv("b")), node("n2", intv("5"))]);
    let d = SemioValueTreeDiff::between(&a, &b);
    let mid = d.apply(&a).expect("apply must succeed for a well-formed fixture");
    assert_eq!(mid, b);
    let inv = d.inverse(&a);
    assert_eq!(inv.apply(&mid).expect("apply must succeed for a well-formed fixture"), a);
}
//#endregion inverse_law

//#region absorb_law canonical cases (list/index-keyed)
// NOTE: these construct `d1`/`d2` DIRECTLY as genuine Insert/Remove/Modify list diffs (matching
// exactly what `InsertListItem`/`RemoveListItem`/`SetValue` would produce) rather than via
// `SemioValueTreeDiff::between(base, next)` — same rationale `json`'s own absorb tests document.
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
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
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
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
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
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
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
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
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
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
    assert_eq!(sequential.root, listv(vec![intv("2")]));
    match &combined.root {
        Some(SemioValueDiff::List { diff }) => {
            assert_eq!(diff.removed, vec![0]);
            assert!(diff.modified.is_empty(), "the pending modify on the removed base index must be dropped");
        }
        other => panic!("expected list diff, got {other:?}"),
    }
}

#[test]
fn absorb_list_associativity() {
    let s0 = snap(listv(vec![intv("1"), intv("2"), intv("3")]), vec![]);
    let s1 = snap(listv(vec![intv("1"), intv("9"), intv("3")]), vec![]);
    let s2 = snap(listv(vec![intv("9"), intv("3"), intv("4")]), vec![]);
    let s3 = snap(listv(vec![intv("9"), intv("4")]), vec![]);
    let d1 = SemioValueTreeDiff::between(&s0, &s1);
    let d2 = SemioValueTreeDiff::between(&s1, &s2);
    let d3 = SemioValueTreeDiff::between(&s2, &s3);

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut right_tail = d2.clone();
    right_tail.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(right_tail);

    assert_eq!(left.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
    assert_eq!(right.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
    assert_eq!(left, right);
}
//#endregion absorb_law canonical cases (list/index-keyed)

//#region absorb_law canonical cases (map/name-keyed)
#[test]
fn absorb_map_add_then_setfield_patches_added_payload() {
    let base = snap(mapv(vec![]), vec![]);
    let mid = snap(mapv(vec![("config", mapv(vec![]))]), vec![]);
    let after = snap(mapv(vec![("config", mapv(vec![("x", intv("5"))]))]), vec![]);
    let d1 = SemioValueTreeDiff::between(&base, &mid);
    let d2 = SemioValueTreeDiff::between(&mid, &after);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
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
    let mid = snap(mapv(vec![("a", intv("9")), ("b", intv("2"))]), vec![]);
    let after = snap(mapv(vec![("b", intv("2"))]), vec![]);
    let d1 = SemioValueTreeDiff::between(&base, &mid);
    let d2 = SemioValueTreeDiff::between(&mid, &after);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
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
    let mid = snap(mapv(vec![("a", intv("1")), ("f", intv("2"))]), vec![]);
    let after = snap(mapv(vec![("a", intv("1")), ("f", intv("2")), ("g", intv("3"))]), vec![]);
    let d1 = SemioValueTreeDiff::between(&base, &mid);
    let d2 = SemioValueTreeDiff::between(&mid, &after);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
    match &combined.root {
        Some(SemioValueDiff::Map { diff }) => assert_eq!(diff.added.len(), 2),
        other => panic!("expected map diff, got {other:?}"),
    }
}

#[test]
fn absorb_map_insert_then_remove_of_same_added_item_cancels() {
    let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
    let mid = snap(mapv(vec![("a", intv("1")), ("f", intv("2"))]), vec![]);
    let after = snap(mapv(vec![("a", intv("1"))]), vec![]);
    let d1 = SemioValueTreeDiff::between(&base, &mid);
    let d2 = SemioValueTreeDiff::between(&mid, &after);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), base);
    assert!(combined.is_empty());
}

#[test]
fn absorb_map_associativity() {
    let s0 = snap(mapv(vec![("a", intv("1"))]), vec![]);
    let s1 = snap(mapv(vec![("a", intv("1")), ("b", intv("2"))]), vec![]);
    let s2 = snap(mapv(vec![("a", intv("9")), ("b", intv("2"))]), vec![]);
    let s3 = snap(mapv(vec![("b", intv("2")), ("c", intv("3"))]), vec![]);
    let d1 = SemioValueTreeDiff::between(&s0, &s1);
    let d2 = SemioValueTreeDiff::between(&s1, &s2);
    let d3 = SemioValueTreeDiff::between(&s2, &s3);

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut right_tail = d2.clone();
    right_tail.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(right_tail);

    assert_eq!(left.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
    assert_eq!(right.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
    assert_eq!(left, right);
}
//#endregion absorb_law canonical cases (map/name-keyed)

//#region absorb_law canonical cases (nodes graph / id-keyed)
#[test]
fn absorb_nodes_add_then_setfield_patches_added_payload() {
    let base = snap(SemioValue::Null, vec![]);
    let mid = snap(SemioValue::Null, vec![node("n1", mapv(vec![]))]);
    let after = snap(SemioValue::Null, vec![node("n1", mapv(vec![("x", intv("5"))]))]);
    let d1 = SemioValueTreeDiff::between(&base, &mid);
    let d2 = SemioValueTreeDiff::between(&mid, &after);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
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
    let mid = snap(SemioValue::Null, vec![node("a", intv("9")), node("b", intv("2"))]);
    let after = snap(SemioValue::Null, vec![node("b", intv("2"))]);
    let d1 = SemioValueTreeDiff::between(&base, &mid);
    let d2 = SemioValueTreeDiff::between(&mid, &after);
    let mut combined = d1.clone();
    combined.absorb(d2.clone());
    assert_eq!(combined.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
    match &combined.nodes {
        Some(diff) => {
            assert_eq!(diff.removed, vec![ValueId::new("a")]);
            assert!(diff.modified.is_empty());
        }
        None => panic!("expected an nodes diff"),
    }
}

#[test]
fn absorb_nodes_associativity() {
    let s0 = snap(SemioValue::Null, vec![node("a", intv("1"))]);
    let s1 = snap(SemioValue::Null, vec![node("a", intv("1")), node("b", intv("2"))]);
    let s2 = snap(SemioValue::Null, vec![node("a", intv("9")), node("b", intv("2"))]);
    let s3 = snap(SemioValue::Null, vec![node("b", intv("2")), node("c", intv("3"))]);
    let d1 = SemioValueTreeDiff::between(&s0, &s1);
    let d2 = SemioValueTreeDiff::between(&s1, &s2);
    let d3 = SemioValueTreeDiff::between(&s2, &s3);

    let mut left = d1.clone();
    left.absorb(d2.clone());
    left.absorb(d3.clone());

    let mut right_tail = d2.clone();
    right_tail.absorb(d3.clone());
    let mut right = d1.clone();
    right.absorb(right_tail);

    assert_eq!(left.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
    assert_eq!(right.apply(&s0).expect("apply must succeed for a well-formed fixture"), s3);
    assert_eq!(left, right);
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

#[test]
fn field_sweep_between_roundtrips_both_directions() {
    let (a, b) = (sweep_a(), sweep_b());
    assert_eq!(SemioValueTreeDiff::between(&a, &b).apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    assert_eq!(SemioValueTreeDiff::between(&b, &a).apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    assert!(SemioValueTreeDiff::between(&a, &a).is_empty());
}

#[test]
fn field_sweep_every_field_present_in_diff() {
    let (a, b) = (sweep_a(), sweep_b());
    let diff = SemioValueTreeDiff::between(&a, &b);

    let map_diff = match &diff.root {
        Some(SemioValueDiff::Map { diff }) => diff,
        other => panic!("expected a top-level map diff, got {other:?}"),
    };
    assert_eq!(map_diff.removed, vec!["removedMember".to_string()]);
    assert_eq!(map_diff.added.len(), 1);
    assert_eq!(map_diff.added[0].item.key, "addedMember");

    let by_key: HashMap<&str, &SemioValueDiff> = map_diff.modified.iter().map(|m| (m.key.as_str(), &m.diff)).collect();
    for key in ["keepBool", "keepInt", "keepFloat", "keepStr", "keepBytes", "keepRef", "kindChange", "nullToValue", "modifiedMember", "nestedList", "nestedMap"] {
        assert!(by_key.contains_key(key), "expected a modified entry for `{key}`");
    }
    assert!(matches!(by_key["kindChange"], SemioValueDiff::Replace { .. }), "Int->Str must fall back to Replace");
    assert!(matches!(by_key["nullToValue"], SemioValueDiff::Replace { .. }), "Null->Bool must fall back to Replace");
    assert!(matches!(by_key["keepBool"], SemioValueDiff::Bool { .. }));
    assert!(matches!(by_key["keepInt"], SemioValueDiff::Int { .. }));
    assert!(matches!(by_key["keepFloat"], SemioValueDiff::Float { .. }));
    assert!(matches!(by_key["keepStr"], SemioValueDiff::Str { .. }));
    assert!(matches!(by_key["keepBytes"], SemioValueDiff::Bytes { .. }));
    assert!(matches!(by_key["keepRef"], SemioValueDiff::Ref { .. }));
    match by_key["nestedList"] {
        SemioValueDiff::List { diff } => {
            assert!(!diff.modified.is_empty());
            assert!(!diff.added.is_empty());
        }
        other => panic!("expected list diff, got {other:?}"),
    }
    match by_key["nestedMap"] {
        SemioValueDiff::Map { diff } => {
            assert!(!diff.modified.is_empty());
            assert!(!diff.added.is_empty());
        }
        other => panic!("expected map diff, got {other:?}"),
    }

    let nodes_diff = diff.nodes.as_ref().expect("expected an nodes graph diff");
    assert_eq!(nodes_diff.removed, vec![ValueId::new("n2")]);
    assert_eq!(nodes_diff.added.len(), 1);
    assert_eq!(nodes_diff.added[0].item.id, ValueId::new("n4"));
    assert_eq!(nodes_diff.modified.len(), 1);
    assert_eq!(nodes_diff.modified[0].key, ValueId::new("n3"));
}
//#endregion field_sweep

//#region 🔖️HandcraftedDiffCodecTests
/// 🧪️ diff_codec_text_binary_roundtrip_law: exercises every `SemioValueDiff` variant (incl.
/// the `Replace` kind-change fallback), nested list/map/nodes-graph collection triples, and
/// the empty (`None`/`None`) diff.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    use protocol::DiffCodec;

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
