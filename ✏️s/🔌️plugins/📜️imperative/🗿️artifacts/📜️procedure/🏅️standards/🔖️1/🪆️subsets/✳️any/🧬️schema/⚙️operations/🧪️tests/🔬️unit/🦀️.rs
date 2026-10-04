use super::*;
use crate::schema::default_path;
use crate::Dictionary;
use neural_engine::{Atom, Value};
use std::collections::BTreeMap;

fn step(id: &str, kind: &str) -> Step {
    Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
}

fn ids(steps: &[Step]) -> Vec<&str> {
    steps.iter().map(|step| step.id.as_str()).collect()
}

fn body_ref() -> PathRef {
    PathRef { owner: Some("loop".into()), slot: Some("body".into()) }
}

fn with_loop() -> Path {
    let mut path = default_path();
    path.steps.push(step("loop", "control.repeat"));
    path
}

/// 📍️ A command's owner/slot addresses a real step's body; anything else addresses the root scope.
#[semio_framework_async_macros::async_test]
async fn path_ref_in_addresses_only_real_owners() {
    let path = with_loop();
    assert_eq!(path_ref_in(&path, Some("loop"), Some("body")), body_ref());
    assert_eq!(path_ref_in(&path, Some("ghost"), Some("body")), PathRef::default());
    assert_eq!(path_ref_in(&path, Some("loop"), None), PathRef::default());
}

/// 🆔️ The next id is one past the highest `step-N` anywhere in the program, nested bodies included.
#[semio_framework_async_macros::async_test]
async fn next_step_id_counts_nested_bodies() {
    let mut path = with_loop();
    assert_eq!(next_step_id(&path), "step-3");
    insert_step(&mut path, &body_ref(), None, step("step-7", "log.print"));
    assert_eq!(next_step_id(&path), "step-8");
}

/// ➕️ Inserts land at the clamped index of the addressed scope; a duplicate id in that scope is no edit.
#[semio_framework_async_macros::async_test]
async fn insert_step_lands_at_the_clamped_index_and_refuses_duplicates() {
    let mut path = with_loop();
    insert_step(&mut path, &PathRef::default(), Some(0), step("first", "log.print"));
    insert_step(&mut path, &PathRef::default(), Some(99), step("last", "log.print"));
    insert_step(&mut path, &PathRef::default(), None, step("step-1", "log.print"));
    assert_eq!(ids(&path.steps), ["first", "step-1", "step-2", "loop", "last"]);
    insert_step(&mut path, &body_ref(), None, step("inner", "log.print"));
    assert_eq!(ids(&resolve_steps_in_path(&path, &body_ref())), ["inner"]);
}

/// ➖️ Removing the last step of a body prunes the emptied slot; moving clamps within its scope.
#[semio_framework_async_macros::async_test]
async fn remove_prunes_emptied_slots_and_move_clamps() {
    let mut path = with_loop();
    insert_step(&mut path, &body_ref(), None, step("inner", "log.print"));
    remove_step(&mut path, &body_ref(), "inner");
    assert!(path.steps.iter().find(|step| step.id == "loop").expect("loop").bodies.is_empty(), "the emptied body slot is pruned");
    move_step(&mut path, &PathRef::default(), "loop", 0);
    move_step(&mut path, &PathRef::default(), "step-1", 99);
    assert_eq!(ids(&path.steps), ["loop", "step-2", "step-1"]);
    move_step(&mut path, &PathRef::default(), "ghost", 0);
    assert_eq!(ids(&path.steps), ["loop", "step-2", "step-1"]);
}

/// 🎚️ Params replace in place; an unknown step leaves the program untouched.
#[semio_framework_async_macros::async_test]
async fn set_step_params_replaces_in_place() {
    let mut path = default_path();
    set_step_params(&mut path, &PathRef::default(), "step-2", Dictionary::new().insert("message", Value::Atom(Atom::String("hi".into()))));
    let params = &path.steps.iter().find(|step| step.id == "step-2").expect("step-2").params;
    assert_eq!(params.get("message"), Some(&Value::Atom(Atom::String("hi".into()))));
    let before = path.clone();
    set_step_params(&mut path, &PathRef::default(), "ghost", Dictionary::new());
    assert_eq!(path, before);
}
