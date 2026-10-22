use semio_framework_os_spr::ColdMutationKind;
use super::*;
use semio_framework_os_spr::SemanticMutation;

#[test]
fn aggregate_roster_is_structural_and_exact() {
    assert_eq!(
        SpaceHistoryMutation::kinds().iter().map(|value| value.kind).collect::<Vec<_>>(),
        ["commit-space-checkpoint", "create-space-alternative", "switch-space-alternative", "remove-space-checkpoint", "remove-space-alternative", "set-active-space-alternative"]
    );
}

fn walk(tree: &dyn semio_framework_pack_json::ArtifactCanonicalJsonTree) -> serde_json::Value {
    use semio_framework_pack_json::{ArtifactCanonicalJsonNode as N, ArtifactCanonicalJsonText as T};
    match tree.canonical_tree_node().expect("canonical node") {
        N::Null => serde_json::Value::Null,
        N::U64(value) => value.into(),
        N::String(value) => value.into(),
        N::Array(length) => serde_json::Value::Array((0..length).map(|ordinal| walk(tree.canonical_tree_child(ordinal).expect("canonical child"))).collect()),
        N::Object(length) => serde_json::Value::Object((0..length).map(|ordinal| {
            let T::Contiguous(key) = tree.canonical_tree_key(ordinal).expect("canonical key") else { panic!("space history keys are contiguous") };
            (key.to_string(), walk(tree.canonical_tree_child(ordinal).expect("canonical child")))
        }).collect()),
        other => panic!("space history canonical JSON has only null, unsigned, text, array and object nodes, not {other:?}"),
    }
}

fn checkpoint(parent: Option<&str>, avatar: Option<&str>) -> crate::space_history_document::SpaceCheckpoint {
    use crate::space_history_document::{SpaceCheckpoint, SpaceMemberPin};
    SpaceCheckpoint {
        id: "cp".into(),
        parent_id: parent.map(Into::into),
        message: "m\"\u{e9}".into(),
        authors: vec![semio_framework_os_vcs::Author { id: "a".into(), name: "A".into(), avatar: avatar.map(Into::into) }],
        timestamp: semio_framework_os_spr::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 },
        members: vec![SpaceMemberPin { document_id: "d".into(), checkpoint_id: "c".into(), alternative_id: String::new() }],
    }
}

#[test]
fn every_leaf_shape_has_a_canonical_tree_equal_to_its_value_document_parsed_by_an_independent_serde_json() {
    use crate::space_history_document::SpaceAlternative;
    let alternative = SpaceAlternative { id: "alt".into(), name: "Alt".into(), checkpoint_ids: vec!["x".into(), "y".into()] };
    for mutation in [
        SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint(None, None), index: None }),
        SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint(Some("parent"), Some("avatar")), index: Some(1) }),
        SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative: alternative.clone() }),
        SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative: SpaceAlternative { checkpoint_ids: Vec::new(), ..alternative } }),
        SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: "alt".into() }),
        SpaceHistoryMutation::RemoveSpaceCheckpoint(RemoveSpaceCheckpoint { checkpoint_id: "cp".into() }),
        SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: "alt".into() }),
        SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: Some("alt".into()) }),
        SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: None }),
    ] {
        let printed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&mutation)).expect("the printed mutation is JSON");
        assert_eq!(walk(&mutation), printed, "{mutation:?}");
        let source = semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::ArtifactCanonicalTreeSource::view(&mutation);
        assert_eq!(indexed(source, &mut Vec::new()), printed, "the indexed wire is the same canonical tree");
        assert!(semio_framework_os_store_canonical_json::ArtifactCanonicalJson::canonical_json_node(source, &[usize::MAX]).is_err());
        assert!(semio_framework_os_store_canonical_json::ArtifactCanonicalJson::canonical_json_key(source, &[0], 9).is_err());
    }
}

fn indexed(source: &impl semio_framework_os_store_canonical_json::ArtifactCanonicalJson, path: &mut Vec<usize>) -> serde_json::Value {
    use semio_framework_pack_json::{ArtifactCanonicalJsonNode as N, ArtifactCanonicalJsonText as T};
    match source.canonical_json_node(path).expect("indexed canonical node") {
        N::Null => serde_json::Value::Null,
        N::U64(value) => value.into(),
        N::String(value) => value.into(),
        N::Array(length) => serde_json::Value::Array((0..length).map(|ordinal| { path.push(ordinal); let value = indexed(source, path); path.pop(); value }).collect()),
        N::Object(length) => serde_json::Value::Object((0..length).map(|ordinal| {
            let T::Contiguous(key) = source.canonical_json_key(path, ordinal).expect("indexed canonical key") else { panic!("space history keys are contiguous") };
            path.push(ordinal);
            let value = indexed(source, path);
            path.pop();
            (key.to_string(), value)
        }).collect()),
        other => panic!("unsupported space history canonical node: {other:?}"),
    }
}

#[test]
fn the_space_history_wire_borrows_the_canonical_mutation_tree() {
    let wire = semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::ArtifactOperationWire::<SpaceHistoryMutation>::semantic::<SpaceHistorySnapshot>();
    let mutation = SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: "alt".into() });
    assert!(matches!(wire.source(&mutation), Some(semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::ArtifactPreparedOperationSource::CanonicalJson { .. })));
}
