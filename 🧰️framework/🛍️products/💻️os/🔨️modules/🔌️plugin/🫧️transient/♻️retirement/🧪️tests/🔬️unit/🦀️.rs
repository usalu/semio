use super::*;
use crate::PluginCloseStep;

#[test]
fn empty_transient_retirement_waits_for_read_release_and_matches_neutral_vectors() {
    let vector: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️read-retirement/🔣️.json")).expect("neutral transient read retirement vector");
    let maximum_steps = vector["maximumSteps"].as_u64().unwrap() as usize;
    let maximum_bytes = vector["maximumBytes"].as_u64().unwrap() as usize;
    let mut owner = Store::new(NoTransient::default());
    let read = owner.current_read().expect("tracked empty transient read");
    let mut disposer = no_transient_store_disposer();
    let zero = disposer.close_step(&mut owner, 0, maximum_bytes).expect("zero grant");
    let released_items = match zero {
        PluginCloseStep::Pending { released_items, released_bytes: 0 } => released_items,
        other => panic!("zero grant advanced retirement: {other:?}"),
    };
    let mut held_read_blocks = false;
    for _ in 0..maximum_steps {
        match disposer.close_step(&mut owner, 1, maximum_bytes).expect("close with held read") {
            PluginCloseStep::Blocked { .. } => { held_read_blocks = true; break; }
            PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= maximum_bytes);
            }
            PluginCloseStep::AwaitingInput { reason } => panic!("transient read retirement unexpectedly awaited input: {reason}"),
            PluginCloseStep::Complete => panic!("transient read authority was discarded before release"),
        }
    }
    drop(read);
    let mut released_read_completes = false;
    for _ in 0..maximum_steps {
        match disposer.close_step(&mut owner, 1, maximum_bytes).expect("close after read release") {
            PluginCloseStep::Complete => { released_read_completes = true; break; }
            PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= maximum_bytes);
            }
            PluginCloseStep::AwaitingInput { reason } => panic!("released transient read unexpectedly awaited input: {reason}"),
            PluginCloseStep::Blocked { .. } => {}
        }
    }
    let actual = serde_json::json!({
        "heldReadBlocks": held_read_blocks,
        "releasedReadCompletes": released_read_completes,
        "terminalIsEmpty": disposer.terminal_is_empty(&owner),
        "zeroGrantReleasedItems": released_items,
    });
    let expected = serde_json::json!({
        "heldReadBlocks": vector["heldReadBlocks"],
        "releasedReadCompletes": vector["releasedReadCompletes"],
        "terminalIsEmpty": vector["terminalIsEmpty"],
        "zeroGrantReleasedItems": vector["zeroGrantReleasedItems"],
    });
    eprintln!("[DEBUG] Empty transient read retirement: {actual}");
    assert_eq!(actual, expected);
}
