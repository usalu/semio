//! 🧪️ The borrowed space-history canonical JSON matches the owned JSON printer, parsed back by an independent serde_json, for every leaf shape.
use super::*;
use serde_json::Value;

fn render(value: V<'_>) -> Value {
    match value {
        V::Scalar(N::Null) => Value::Null,
        V::Scalar(N::U64(value)) => value.into(),
        V::Scalar(N::String(value)) => value.into(),
        V::Scalar(_) | V::Source(_) => panic!("space history canonical JSON borrows only null, unsigned and text scalars"),
        V::Array(values) => Value::Array(values.map(render).collect()),
        V::Object(fields) => Value::Object(fields.map(|(key, value)| (match key { T::Contiguous(key) => key.to_string(), T::Native(_) => panic!("space history keys are contiguous") }, render(value))).collect()),
    }
}

fn indexed(source: &SpaceHistoryMutation, path: &mut Vec<usize>) -> Value {
    match source.canonical_json_node(path).expect("indexed canonical node") {
        N::Null => Value::Null,
        N::U64(value) => value.into(),
        N::String(value) => value.into(),
        N::Array(length) => Value::Array((0..length).map(|index| { path.push(index); let value = indexed(source, path); path.pop(); value }).collect()),
        N::Object(length) => Value::Object((0..length).map(|index| {
            let key = match source.canonical_json_key(path, index).expect("indexed canonical key") { T::Contiguous(key) => key.to_string(), T::Native(_) => panic!("space history keys are contiguous") };
            path.push(index);
            let value = indexed(source, path);
            path.pop();
            (key, value)
        }).collect()),
        _ => panic!("space history canonical JSON has only null, unsigned, text, array and object nodes"),
    }
}

fn canonical(mutation: &SpaceHistoryMutation) -> Value {
    let borrowed = render(mutation.canonical_json_borrowed_root().expect("canonical traversal").expect("space history mutations borrow a root"));
    assert_eq!(borrowed, indexed(mutation, &mut Vec::new()), "the borrowed root and the indexed traversal are one document");
    assert!(mutation.canonical_json_node(&[usize::MAX]).is_err() && mutation.canonical_json_key(&[0], 9).is_err(), "unknown positions are refused");
    let printed: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(mutation)).expect("the printed mutation is JSON");
    assert_eq!(borrowed, printed, "the borrowed canonical document is the printed document");
    borrowed
}

fn checkpoint(parent: Option<&str>, avatar: Option<&str>) -> SpaceCheckpoint {
    SpaceCheckpoint {
        id: "cp".into(),
        parent_id: parent.map(Into::into),
        message: "m\"\u{e9}".into(),
        authors: vec![Author { id: "a".into(), name: "A".into(), avatar: avatar.map(Into::into) }],
        timestamp: HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 },
        members: vec![SpaceMemberPin { document_id: "d".into(), checkpoint_id: "c".into(), alternative_id: String::new() }],
    }
}

#[test]
fn every_leaf_shape_matches_the_printed_document() {
    let alternative = SpaceAlternative { id: "alt".into(), name: "Alt".into(), checkpoint_ids: vec!["x".into(), "y".into()] };
    for mutation in [
        SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint(None, None) }),
        SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint(Some("parent"), Some("avatar")) }),
        SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative: alternative.clone() }),
        SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative: SpaceAlternative { checkpoint_ids: Vec::new(), ..alternative } }),
        SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: "alt".into() }),
        SpaceHistoryMutation::RemoveSpaceCheckpoint(RemoveSpaceCheckpoint { checkpoint_id: "cp".into() }),
        SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: "alt".into() }),
        SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: Some("alt".into()) }),
        SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: None }),
    ] {
        canonical(&mutation);
    }
}

#[test]
fn the_catalog_names_the_mutation_itself_as_its_wire() {
    use super::super::super::SpaceHistorySnapshot;
    use crate::os_store::{ArtifactCanonicalAuthoringFactory, ArtifactPreparedOperationSource, ArtifactStoreOneItemPreparationFactory};
    let factory = ArtifactCanonicalAuthoringFactory::<SpaceHistorySnapshot, SpaceHistoryMutation>::new();
    let mutation = SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: "alt".into() });
    assert!(matches!(factory.operation_wire_source(&mutation), Some(ArtifactPreparedOperationSource::CanonicalJson { .. })));
}
