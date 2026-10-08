//! 🧪️ Every details-pane pointer of the STEP class 4 editor resolves to the class's mutations, and replaying them reaches the edited document.

use super::*;
use crate::schema::snapshot::{StepEntity, StepValue};
use semio_framework_value::{DslValue, ToValue};

fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
    StepEntity { id, name: name.into(), args, complex: Vec::new() }
}

fn base() -> StepSnapshot {
    StepSnapshot { entities: vec![entity(1, "POINT", vec![StepValue::Real(1.0)]), entity(2, "LINE", vec![StepValue::Reference(1)]), entity(3, "CIRCLE", vec![StepValue::Reference(1)])], ..Default::default() }
}

fn resolve(event: SnapshotEditEvent) -> Vec<StepCc4Mutation> {
    let snapshot = base();
    match special(&event, &snapshot).unwrap() {
        Some(mutations) => mutations,
        None => EDIT_RULES.resolve::<StepSnapshot, StepCc4Mutation>(&snapshot, &event).unwrap(),
    }
}

fn replay(mutations: &[StepCc4Mutation]) -> StepSnapshot {
    let mut snapshot = base();
    for mutation in mutations {
        assert!(<StepCc4Mutation as protocol::Mutation<StepSnapshot>>::diff(mutation, &snapshot).messages().is_empty());
        snapshot = protocol::apply_diff(<StepCc4Mutation as protocol::Mutation<StepSnapshot>>::diff(mutation, &snapshot).diff(), &snapshot).expect("the mutation applies");
    }
    snapshot
}

#[test]
fn an_argument_edit_writes_the_edited_entity_through_one_restore_row() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/entities/0/args/0".into(), value: StepValue::Real(7.0).to_value() });
    assert_eq!(mutations.len(), 1);
    assert_eq!(replay(&mutations).entities[0].args, vec![StepValue::Real(7.0)]);
}

#[test]
fn entities_insert_and_remove_at_their_position() {
    let inserted = resolve(SnapshotEditEvent::InsertValue { path: "/entities/1".into(), value: entity(50, "NEW", vec![StepValue::Reference(1)]).to_value() });
    assert_eq!(replay(&inserted).entities.iter().map(|entity| entity.id).collect::<Vec<_>>(), vec![1, 50, 2, 3]);
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/entities/2".into() });
    assert_eq!(replay(&removed).entities.iter().map(|entity| entity.id).collect::<Vec<_>>(), vec![1, 2]);
}

#[test]
fn an_unchanged_edit_publishes_nothing_and_the_file_name_is_refused() {
    assert!(resolve(SnapshotEditEvent::SetValue { path: "/entities/0/name".into(), value: DslValue::String("POINT".into()) }).is_empty());
    let refused = SnapshotEditEvent::SetValue { path: "/header/fileName".into(), value: DslValue::Null };
    assert_eq!(EDIT_RULES.resolve::<StepSnapshot, StepCc4Mutation>(&base(), &refused).unwrap_err().code, "snapshot-edit.unsupported-path");
}
