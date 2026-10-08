//! 🧪️ The pointer → kind table resolves every edit shape to ONE concrete kind, refuses what no rule names and publishes nothing for a no-op.

use super::*;

const RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/fmt", "set-fmt", "fmt").carrying(&[Carried { payload: "layers", pointer: "/layers" }]),
        EntityRule::new("/layers/*", "set-layer", "layer").selecting(&[Selector::Field { payload: "name", field: "name" }]),
        EntityRule::new("/layers/*/tags/*", "set-tag", "tag").selecting(&[Selector::Index("layer"), Selector::Index("tag")]),
    ],
    inserts: &[InsertRule::new("/layers", "insert-layer", "layer").at("index")],
    removes: &[RemoveRule::by_key("/layers", "remove-layer", "name", "name"), RemoveRule::by_index("/layers/*/tags", "remove-tag", "tag").selecting(&[Selector::Index("layer")])],
};

fn tree() -> DslValue {
    let source = r#"{"fmt":{"channels":1,"rate":8000},"layers":[{"name":"A","color":1,"tags":[{"code":1},{"code":2}]},{"name":"B","color":2,"tags":[]}]}"#;
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())
}

fn json(source: &str) -> DslValue {
    semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())
}

fn entry<'a>(plan: &'a EditPlan, name: &str) -> &'a DslValue {
    &plan.entries.iter().find(|(key, _)| key == name).unwrap_or_else(|| panic!("payload entry {name}")).1
}

#[test]
fn a_set_inside_an_entity_carries_the_whole_new_entity() {
    let plan = RULES.plan(&tree(), &SnapshotEditEvent::SetValue { path: "/fmt/channels".into(), value: json("2") }).unwrap().expect("a plan");
    assert_eq!(plan.kind, "set-fmt");
    assert_eq!(entry(&plan, "fmt"), &json(r#"{"channels":2,"rate":8000}"#));
    let document = tree();
    assert_eq!(entry(&plan, "layers"), value_at(&document, &["layers".to_string()], "").unwrap(), "the carried field is the current document value");
}

#[test]
fn a_row_keyed_entity_names_the_row_by_its_key_field() {
    let plan = RULES.plan(&tree(), &SnapshotEditEvent::SetValue { path: "/layers/1/color".into(), value: json("9") }).unwrap().expect("a plan");
    assert_eq!(plan.kind, "set-layer");
    assert_eq!(entry(&plan, "name"), &json(r#""B""#));
    assert_eq!(entry(&plan, "layer"), &json(r#"{"name":"B","color":9,"tags":[]}"#));
}

#[test]
fn the_longest_template_governs_and_wildcards_land_as_positions() {
    let plan = RULES.plan(&tree(), &SnapshotEditEvent::SetValue { path: "/layers/0/tags/1/code".into(), value: json("7") }).unwrap().expect("a plan");
    assert_eq!(plan.kind, "set-tag");
    assert_eq!((entry(&plan, "layer"), entry(&plan, "tag")), (&json("0"), &json(r#"{"code":7}"#)));
}

#[test]
fn inserts_and_removes_resolve_to_their_own_kinds() {
    let inserted = RULES.plan(&tree(), &SnapshotEditEvent::InsertValue { path: "/layers/1".into(), value: json(r#"{"name":"C"}"#) }).unwrap().expect("a plan");
    assert_eq!((inserted.kind, entry(&inserted, "index")), ("insert-layer", &json("1")));
    let appended = RULES.plan(&tree(), &SnapshotEditEvent::InsertValue { path: "/layers/-".into(), value: json(r#"{"name":"C"}"#) }).unwrap().expect("a plan");
    assert_eq!(entry(&appended, "index"), &json("2"));
    let removed = RULES.plan(&tree(), &SnapshotEditEvent::RemoveValue { path: "/layers/0".into() }).unwrap().expect("a plan");
    assert_eq!((removed.kind, entry(&removed, "name")), ("remove-layer", &json(r#""A""#)));
    let tag = RULES.plan(&tree(), &SnapshotEditEvent::RemoveValue { path: "/layers/0/tags/1".into() }).unwrap().expect("a plan");
    assert_eq!((tag.kind, entry(&tag, "layer"), entry(&tag, "tag")), ("remove-tag", &json("0"), &json("1")));
}

#[test]
fn moves_and_renames_stay_inside_one_entity() {
    let moved = RULES.plan(&tree(), &SnapshotEditEvent::MoveValue { from: "/layers/0/tags/1".into(), path: "/layers/0/tags/0".into() }).unwrap();
    assert_eq!(moved.expect("a plan").kind, "set-layer");
    let refused = RULES.plan(&tree(), &SnapshotEditEvent::MoveValue { from: "/layers/0".into(), path: "/layers/1".into() }).unwrap_err();
    assert_eq!(refused.code, "snapshot-edit.unsupported-path");
    let renamed = RULES.plan(&tree(), &SnapshotEditEvent::RenameKey { path: "/fmt/rate".into(), key: "hz".into() }).unwrap().expect("a plan");
    assert_eq!(entry(&renamed, "fmt"), &json(r#"{"channels":1,"hz":8000}"#));
}

#[test]
fn an_edit_that_changes_nothing_publishes_nothing_and_an_unnamed_path_is_refused() {
    assert_eq!(RULES.plan(&tree(), &SnapshotEditEvent::SetValue { path: "/fmt/rate".into(), value: json("8000") }).unwrap(), None);
    let refused = RULES.plan(&tree(), &SnapshotEditEvent::SetValue { path: "/fmt/nothing".into(), value: json("1") });
    assert_eq!(refused.unwrap_err().code, "snapshot-edit.path-missing");
    let unnamed = RULES.plan(&json(r#"{"extra":1}"#), &SnapshotEditEvent::SetValue { path: "/extra".into(), value: json("2") }).unwrap_err();
    assert_eq!((unnamed.code, unnamed.path.as_str()), ("snapshot-edit.unsupported-path", "/extra"));
    let source = RULES.plan(&tree(), &SnapshotEditEvent::ReplaceSource { source: "{}".into() }).unwrap_err();
    assert_eq!(source.code, "snapshot-edit.unsupported-path");
}
