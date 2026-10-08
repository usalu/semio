//! 🧪️ Every details-pane pointer of the STEP editor resolves to the concrete kind of its gesture, and replaying it reaches the edited document.

use super::*;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
    StepEntity { id, name: name.into(), args, complex: Vec::new() }
}

fn base() -> StepSnapshot {
    StepSnapshot { entities: vec![entity(1, "POINT", vec![StepValue::Real(1.0), StepValue::Real(2.0)]), entity(2, "LINE", vec![StepValue::Reference(1), StepValue::Integer(3)]), entity(3, "CIRCLE", vec![StepValue::Reference(1)])], ..Default::default() }
}

fn resolve(event: SnapshotEditEvent) -> Vec<StepMutation> {
    EDIT_RULES.resolve::<StepSnapshot, StepMutation>(&base(), &event).unwrap()
}

fn replay(mutations: &[StepMutation]) -> StepSnapshot {
    let mut snapshot = base();
    for mutation in mutations {
        assert!(apply_step_mutation(&mut snapshot, mutation).messages().is_empty());
    }
    snapshot
}

#[test]
fn an_entity_name_and_an_argument_are_addressed_by_id_and_position() {
    let renamed = resolve(SnapshotEditEvent::SetValue { path: "/entities/1/name".into(), value: DslValue::String("SEGMENT".into()) });
    assert_eq!(renamed, vec![StepMutation::SetEntityName(set_entity_name::SetEntityName { id: 2, name: "SEGMENT".into() })]);
    let set = resolve(SnapshotEditEvent::SetValue { path: "/entities/0/args/1".into(), value: StepValue::Real(9.0).to_value() });
    assert_eq!(set, vec![StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, arg_index: 1, value: StepValue::Real(9.0) })]);
    assert_eq!(replay(&set).entities[0].args[1], StepValue::Real(9.0));
}

#[test]
fn arguments_and_entities_insert_and_remove_at_their_position() {
    let inserted = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1/args/1".into(), value: StepValue::Enum("F".into()).to_value() });
    assert_eq!(replay(&inserted).entities[1].args, vec![StepValue::Reference(1), StepValue::Enum("F".into()), StepValue::Integer(3)]);
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/entities/1/args/0".into() });
    assert_eq!(replay(&removed).entities[1].args, vec![StepValue::Integer(3)]);
    let entity_inserted = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1".into(), value: entity(50, "NEW", vec![StepValue::Reference(1)]).to_value() });
    assert_eq!(replay(&entity_inserted).entities.iter().map(|entity| entity.id).collect::<Vec<_>>(), vec![1, 50, 2, 3]);
    let entity_removed = resolve(SnapshotEditEvent::RemoveValue { path: "/entities/2".into() });
    assert_eq!(entity_removed, vec![StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: 3 })]);
}

#[test]
fn an_edit_below_an_argument_replaces_that_argument_and_an_unchanged_one_publishes_nothing() {
    let nested = base().entities[0].args[0].to_value();
    assert!(resolve(SnapshotEditEvent::SetValue { path: "/entities/0/args/0".into(), value: nested }).is_empty());
    let refused = SnapshotEditEvent::SetValue { path: "/schema".into(), value: DslValue::String("other".into()) };
    assert_eq!(EDIT_RULES.resolve::<StepSnapshot, StepMutation>(&base(), &refused).unwrap_err().code, "snapshot-edit.unsupported-path");
}
