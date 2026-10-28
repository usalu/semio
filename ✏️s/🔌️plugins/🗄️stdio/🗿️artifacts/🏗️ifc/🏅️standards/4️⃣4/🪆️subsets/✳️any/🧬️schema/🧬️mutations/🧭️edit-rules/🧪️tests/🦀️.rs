//! 🧪️ Every details-pane pointer of the IFC4 editor resolves to the concrete kind of its gesture, and replaying it reaches the edited document.

use super::*;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

fn entity(id: u64, name: &str, args: Vec<IfcValue>) -> IfcEntity {
    IfcEntity { id, name: name.into(), args, complex: Vec::new() }
}

fn base() -> IfcSnapshot {
    IfcSnapshot { entities: vec![entity(1, "IFCCARTESIANPOINT", vec![IfcValue::Real(1.0)]), entity(2, "IFCWALL", vec![IfcValue::Reference(1), IfcValue::Integer(3)]), entity(3, "IFCSLAB", vec![IfcValue::Reference(1)])], ..Default::default() }
}

fn resolve(event: SnapshotEditEvent) -> Vec<IfcMutation> {
    complex_edit(&event, &base()).unwrap().unwrap_or_else(|| EDIT_RULES.resolve::<IfcSnapshot, IfcMutation>(&base(), &event).unwrap())
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
    let inserted = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1/args/1".into(), value: IfcValue::Integer(8).to_value() });
    assert_eq!(replay(&inserted).entities[1].args, vec![IfcValue::Reference(1), IfcValue::Integer(8), IfcValue::Integer(3)]);
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/entities/1/args/0".into() });
    assert_eq!(replay(&removed).entities[1].args, vec![IfcValue::Integer(3)]);
    let entity_inserted = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1".into(), value: entity(50, "IFCDOOR", vec![IfcValue::Reference(1)]).to_value() });
    assert_eq!(replay(&entity_inserted).entities.iter().map(|entity| entity.id).collect::<Vec<_>>(), vec![1, 50, 2, 3]);
    let entity_removed = resolve(SnapshotEditEvent::RemoveValue { path: "/entities/2".into() });
    assert_eq!(entity_removed, vec![IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 3 })]);
}

#[test]
fn an_unnamed_pointer_is_refused() {
    let refused = SnapshotEditEvent::SetValue { path: "/schema".into(), value: DslValue::String("other".into()) };
    assert_eq!(EDIT_RULES.resolve::<IfcSnapshot, IfcMutation>(&base(), &refused).unwrap_err().code, "snapshot-edit.unsupported-path");
}

#[test]
fn complex_constituents_are_replaced_by_entity_id() {
    let complex = vec![semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: "IFCOBJECT".into(), args: vec![IfcValue::String("Member".into())] }];
    let mutations = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1/complex".into(), value: complex.to_value() });
    assert_eq!(mutations, vec![IfcMutation::SetEntityComplex(set_entity_complex::SetEntityComplex { id: 2, complex: complex.clone() })]);
    assert_eq!(replay(&mutations).entities[1].complex, complex);
}

#[test]
fn the_whole_complex_field_and_nested_members_edit_and_remove() {
    let member = semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcComplexType { name: "IFCOBJECT".into(), args: vec![] };
    let inserted = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1/complex".into(), value: vec![member].to_value() });
    let mut snapshot = replay(&inserted);
    let changed = complex_edit(&SnapshotEditEvent::SetValue { path: "/entities/1/complex/0/name".into(), value: DslValue::String("IFCPRODUCT".into()) }, &snapshot).unwrap().unwrap();
    for mutation in &changed {
        apply_ifc_mutation(&mut snapshot, mutation);
    }
    assert_eq!(snapshot.entities[1].complex[0].name, "IFCPRODUCT");
    let removed = complex_edit(&SnapshotEditEvent::RemoveValue { path: "/entities/1/complex".into() }, &snapshot).unwrap().unwrap();
    assert_eq!(removed, vec![IfcMutation::SetEntityComplex(set_entity_complex::SetEntityComplex { id: 2, complex: vec![] })]);
    for mutation in &removed {
        apply_ifc_mutation(&mut snapshot, mutation);
    }
    assert!(snapshot.entities[1].complex.is_empty());
    assert_eq!(snapshot.entities[1].args, base().entities[1].args);
}
