
use super::*;
use crate::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
use protocol::MutationDiff;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap(root: SemioValue, nodes: Vec<SemioValueNode>) -> SemioValueSnapshot {
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root, nodes }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn mapv(pairs: Vec<(&str, SemioValue)>) -> SemioValue {
    SemioValue::Map { entries: pairs.into_iter().map(|(k, v)| SemioValueEntry { key: k.into(), value: v }).collect() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn listv(items: Vec<SemioValue>) -> SemioValue {
    SemioValue::List { items }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn intv(lexeme: &str) -> SemioValue {
    SemioValue::Int { lexeme: lexeme.into() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn strv(s: &str) -> SemioValue {
    SemioValue::Str { value: s.into() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn node(id: &str, value: SemioValue) -> SemioValueNode {
    SemioValueNode { id: ValueId::new(id), value }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_fixture() -> SemioValueSnapshot {
    snap(mapv(vec![("a", intv("1")), ("list", listv(vec![intv("1"), intv("2")]))]), vec![node("n1", strv("hello"))])
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn apply_and_check(base: &SemioValueSnapshot, mutation: SemioValueMutation) -> (SemioValueSnapshot, protocol::MutationOutcome<SemioValueTreeDiff>) {
    let mut via_apply = base.clone();
    let returned = apply_semio_value_mutation(&mut via_apply, &mutation);
    let expected_diff = mutation.diff(base);
    assert_eq!(returned, expected_diff, "apply_semio_value_mutation must return mutation.diff(base)");
    let via_diff_apply = expected_diff.diff().apply(base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(via_apply, via_diff_apply, "m.diff(base).diff().apply(base) must equal apply_semio_value_mutation's result");
    (via_apply, returned)
}

//#region mutation_diff_law
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_all_variants() {
    let base = base_fixture();

    apply_and_check(&base, SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snap(SemioValue::Bool { value: true }, vec![]) }));
    apply_and_check(&base, SemioValueMutation::SetValue(set_value::SetValue { path: vec![SemioValuePathSegment::Key { key: "a".into() }], value: intv("2") }));
    apply_and_check(&base, SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") }));
    apply_and_check(&base, SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "new".into(), value: strv("fresh") }));
    apply_and_check(&base, SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: vec![], key: "a".into() }));
    apply_and_check(&base, SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: intv("99") }));
    apply_and_check(&base, SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 0 }));
    apply_and_check(&base, SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n1"), value: strv("updated") }));
    apply_and_check(&base, SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n2"), value: strv("brand-new") }));
    apply_and_check(&base, SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: ValueId::new("n1") }));
}

#[semio_framework_async_macros::async_test]
async fn set_map_entry_on_missing_key_adds_at_end() {
    let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
    let (result, _) = apply_and_check(&base, SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "b".into(), value: intv("2") }));
    assert_eq!(result.root, mapv(vec![("a", intv("1")), ("b", intv("2"))]));
}

#[semio_framework_async_macros::async_test]
async fn remove_map_entry_missing_key_is_noop() {
    let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
    let (result, diff) = apply_and_check(&base, SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: vec![], key: "missing".into() }));
    assert_eq!(result, base);
    assert!(diff.diff().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn nested_path_targets_inner_entry() {
    let base = snap(mapv(vec![("outer", mapv(vec![("inner", intv("1"))]))]), vec![]);
    let (result, _) = apply_and_check(&base, SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![SemioValuePathSegment::Key { key: "outer".into() }], key: "inner".into(), value: intv("42") }));
    assert_eq!(result.root, mapv(vec![("outer", mapv(vec![("inner", intv("42"))]))]));
}

#[semio_framework_async_macros::async_test]
async fn set_node_on_missing_id_inserts() {
    let base = snap(SemioValue::Null, vec![node("n1", strv("a"))]);
    let (result, _) = apply_and_check(&base, SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n2"), value: strv("b") }));
    assert_eq!(result.nodes, vec![node("n1", strv("a")), node("n2", strv("b"))]);
}
//#endregion mutation_diff_law

//#region inverse_law
#[semio_framework_async_macros::async_test]
async fn inverse_law_mutation_level_round_trips() {
    let base = base_fixture();
    let mutations = vec![
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") }),
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "new".into(), value: strv("fresh") }),
        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: vec![], key: "a".into() }),
        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 1, value: intv("99") }),
        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: vec![SemioValuePathSegment::Key { key: "list".into() }], index: 0 }),
        SemioValueMutation::SetValue(set_value::SetValue { path: vec![SemioValuePathSegment::Key { key: "a".into() }], value: strv("replaced") }),
        SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n1"), value: strv("updated") }),
        SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n9"), value: strv("brand-new") }),
        SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: ValueId::new("n1") }),
    ];
    for mutation in mutations {
        let mut state = base.clone();
        apply_semio_value_mutation(&mut state, &mutation);
        for undo in <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(&mutation, &base) {
            apply_semio_value_mutation(&mut state, &undo);
        }
        assert_eq!(state, base, "mutation {mutation:?} did not round-trip via its inverse");
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_law_diff_level_matches_mutation_diff() {
    let base = snap(mapv(vec![("a", intv("1"))]), vec![]);
    let mutation = SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: vec![], key: "a".into(), value: intv("2") });
    let diff = mutation.diff(&base);
    let mid = diff.diff().apply(&base).expect("apply must succeed for a well-formed fixture");
    let inv = diff.diff().inverse(&base);
    assert_eq!(inv.apply(&mid).expect("apply must succeed for a well-formed fixture"), base);
}
//#endregion inverse_law

//#region 🔖️OpCodecTests
/// 🧪️ op_text_binary_roundtrip_law: exercises every variant, incl. nested/list/map payload
/// values, a `Ref`/`Bytes` payload, and a multi-segment `SemioValuePath` mixing both segment
/// kinds.
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    use protocol::{OpBinary, OpText};

    for m in demo_mutation_cases() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = <SemioValueMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch (printed {printed:?})");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
        let decoded = <SemioValueMutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch");
    }
}
//#endregion 🔖️OpCodecTests

//#region 🔖️CatalogLaw
/// 🏷️ The wildcard-free spelling map that makes [`KINDS`] compiler-checked: a new variant has
/// no arm here, so the crate stops building until both this match and `KINDS` name it.
fn kind_of(mutation: &SemioValueMutation) -> &'static str {
    match mutation {
        SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => "set-snapshot",
        SemioValueMutation::SetValue(set_value::SetValue { .. }) => "set-value",
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { .. }) => "set-map-entry",
        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { .. }) => "remove-map-entry",
        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { .. }) => "insert-list-item",
        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { .. }) => "remove-list-item",
        SemioValueMutation::SetNode(set_node::SetNode { .. }) => "set-node",
        SemioValueMutation::RemoveNode(remove_node::RemoveNode { .. }) => "remove-node",
    }
}

/// 🏷️ `KINDS` must name every declared variant, in declaration order and in the exact spelling
/// the committed `semio-v1-value` catalog carries — the framework never parses Rust, so this is
/// the only thing that keeps the catalog honest against the enum.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let one_per_variant = [
        SemioValueMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SemioValueSnapshot::default() }),
        SemioValueMutation::SetValue(set_value::SetValue { path: Vec::new(), value: SemioValue::Null }),
        SemioValueMutation::SetMapEntry(set_map_entry::SetMapEntry { path: Vec::new(), key: "status".into(), value: SemioValue::Null }),
        SemioValueMutation::RemoveMapEntry(remove_map_entry::RemoveMapEntry { path: Vec::new(), key: "status".into() }),
        SemioValueMutation::InsertListItem(insert_list_item::InsertListItem { path: Vec::new(), index: 0, value: SemioValue::Null }),
        SemioValueMutation::RemoveListItem(remove_list_item::RemoveListItem { path: Vec::new(), index: 0 }),
        SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n-1"), value: SemioValue::Null }),
        SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: ValueId::new("n-1") }),
    ];
    assert_eq!(KINDS.len(), one_per_variant.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, mutation) in KINDS.iter().zip(one_per_variant.iter()) {
        assert_eq!(*kind, kind_of(mutation), "KINDS must follow the enum's own declaration order and kebab-case spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🔖️CatalogLaw
