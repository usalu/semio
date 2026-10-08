//! 🧪️ Every details-pane pointer of the IFC4 editor resolves to the concrete kind of its gesture, and replaying it reaches the edited document.

use super::*;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

fn entity(id: u64, name: &str, args: Vec<IfcValue>) -> IfcEntity {
    IfcEntity { id, name: name.into(), args, complex: Vec::new() }
}

fn base() -> IfcSnapshot {
    IfcSnapshot { entities: vec![entity(1, "IFCCARTESIANPOINT", vec![IfcValue::Real(1.0)]), entity(2, "IFCWALL", vec![IfcValue::Ref(1), IfcValue::Int(3)]), entity(3, "IFCSLAB", vec![IfcValue::Ref(1)])], ..Default::default() }
}

fn resolve(event: SnapshotEditEvent) -> Vec<IfcMutation> {
    EDIT_RULES.resolve::<IfcSnapshot, IfcMutation>(&base(), &event).unwrap()
}

fn replay(mutations: &[IfcMutation]) -> IfcSnapshot {
    let mut snapshot = base();
    for mutation in mutations {
        assert!(apply_ifc_mutation(&mut snapshot, mutation).messages().is_empty());
    }
    snapshot
}

#[test]
fn an_entity_name_and_an_argument_are_addressed_by_id_and_position() {
    let renamed = resolve(SnapshotEditEvent::SetValue { path: "/entities/1/name".into(), value: DslValue::String("IFCBEAM".into()) });
    assert_eq!(renamed, vec![IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 2, name: "IFCBEAM".into() })]);
    let set = resolve(SnapshotEditEvent::SetValue { path: "/entities/0/args/0".into(), value: IfcValue::Real(9.0).to_value() });
    assert_eq!(set, vec![IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, index: 0, value: IfcValue::Real(9.0) })]);
    assert_eq!(replay(&set).entities[0].args[0], IfcValue::Real(9.0));
}

#[test]
fn arguments_and_entities_insert_and_remove_at_their_position() {
    let inserted = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1/args/1".into(), value: IfcValue::Int(8).to_value() });
    assert_eq!(replay(&inserted).entities[1].args, vec![IfcValue::Ref(1), IfcValue::Int(8), IfcValue::Int(3)]);
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/entities/1/args/0".into() });
    assert_eq!(replay(&removed).entities[1].args, vec![IfcValue::Int(3)]);
    let entity_inserted = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1".into(), value: entity(50, "IFCDOOR", vec![IfcValue::Ref(1)]).to_value() });
    assert_eq!(replay(&entity_inserted).entities.iter().map(|entity| entity.id).collect::<Vec<_>>(), vec![1, 50, 2, 3]);
    let entity_removed = resolve(SnapshotEditEvent::RemoveValue { path: "/entities/2".into() });
    assert_eq!(entity_removed, vec![IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 3 })]);
}

#[test]
fn an_unnamed_pointer_is_refused() {
    let refused = SnapshotEditEvent::SetValue { path: "/schema".into(), value: DslValue::String("other".into()) };
    assert_eq!(EDIT_RULES.resolve::<IfcSnapshot, IfcMutation>(&base(), &refused).unwrap_err().code, "snapshot-edit.unsupported-path");
}
