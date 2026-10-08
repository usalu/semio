//! 🧪️ Every details-pane pointer of the IFC 2x3 editor resolves to the concrete kind of its gesture, and replaying it reaches the edited document.

use super::*;
use semio_framework_value::{DslValue, ToValue};
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
use semio_s_artifact_stdio_contract::part21::Part21Value;

fn instance(id: u64, name: &str, args: Vec<Part21Value>) -> Part21Instance {
    Part21Instance { id, entities: vec![(name.into(), args)] }
}

fn base() -> Ifc2x3Snapshot {
    let mut snapshot = Ifc2x3Snapshot::default();
    snapshot.document.instances = vec![instance(1, "IFCCARTESIANPOINT", vec![Part21Value::Real(1.0.into())]), instance(2, "IFCWALL", vec![Part21Value::Ref(1)]), instance(3, "IFCSLAB", vec![Part21Value::Ref(1)])];
    snapshot
}

fn resolve(event: SnapshotEditEvent) -> Vec<Ifc2x3Mutation> {
    EDIT_RULES.resolve::<Ifc2x3Snapshot, Ifc2x3Mutation>(&base(), &event).unwrap()
}

fn replay(mutations: &[Ifc2x3Mutation]) -> Ifc2x3Snapshot {
    let mut snapshot = base();
    for mutation in mutations {
        assert!(apply_ifc2x3_mutation(&mut snapshot, mutation).messages().is_empty());
    }
    snapshot
}

#[test]
fn an_edit_inside_an_instance_upserts_that_instance_in_place() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/document/instances/1/entities/0/arguments/0".into(), value: Part21Value::Ref(3).to_value() });
    assert!(matches!(mutations.as_slice(), [Ifc2x3Mutation::UpsertInstance(_)]));
    let edited = replay(&mutations);
    assert_eq!(edited.document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>(), vec![1, 2, 3]);
    assert_eq!(edited.document.instances[1].entities[0].1, vec![Part21Value::Ref(3)]);
}

#[test]
fn instances_insert_at_their_position_and_remove_by_id() {
    let inserted = resolve(SnapshotEditEvent::InsertValue { path: "/document/instances/1".into(), value: instance(50, "IFCDOOR", vec![Part21Value::Ref(1)]).to_value() });
    assert_eq!(replay(&inserted).document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>(), vec![1, 50, 2, 3]);
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/document/instances/2".into() });
    assert_eq!(removed, vec![Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 3 })]);
}

#[test]
fn the_preamble_is_refused_and_an_unchanged_edit_publishes_nothing() {
    let refused = SnapshotEditEvent::SetValue { path: "/edmPreamble".into(), value: DslValue::Null };
    assert_eq!(EDIT_RULES.resolve::<Ifc2x3Snapshot, Ifc2x3Mutation>(&base(), &refused).unwrap_err().code, "snapshot-edit.unsupported-path");
    let header = base().document.header.to_value();
    assert!(resolve(SnapshotEditEvent::SetValue { path: "/document/header".into(), value: header }).is_empty());
}

#[test]
fn changing_an_instance_id_keeps_its_position_and_refuses_an_id_in_use() {
    let event = SnapshotEditEvent::SetValue { path: "/document/instances/1/id".into(), value: DslValue::uint(77) };
    let rows = special(&event, &base()).unwrap().expect("an id edit is a rename");
    assert!(matches!(rows.as_slice(), [Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { index: Some(1), .. }), Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 2 })]));
    assert_eq!(replay(&rows).document.instances.iter().map(|instance| instance.id).collect::<Vec<_>>(), vec![1, 77, 3]);
    let taken = SnapshotEditEvent::SetValue { path: "/document/instances/1/id".into(), value: DslValue::uint(3) };
    assert_eq!(special(&taken, &base()).unwrap_err().code, "snapshot-edit.schema-invalid");
}
