use super::*;
use crate::app::{plugin_demand_grant, PluginLifecycleStep};
use semio_framework_value::RetirementDemand;

/// 🎟️ Funds one quoted retirement turn on every axis, with `copy` as the smallest payload credit.
fn funded(demand: RetirementDemand, copy: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(copy), ..plugin_demand_grant(demand) }
}

#[test]
fn empty_transient_retirement_waits_for_read_release_and_matches_neutral_vectors() {
    let vector: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📖️read-retirement/🔣️.json")).expect("neutral transient read retirement vector");
    let maximum_steps = vector["maximumSteps"].as_u64().unwrap() as usize;
    let maximum_bytes = vector["maximumBytes"].as_u64().unwrap() as usize;
    let mut owner = Store::new(NoTransient::default());
    let read = owner.current_read().expect("tracked empty transient read");
    let mut disposer = no_transient_store_disposer();
    let unfunded = RetainedCloneGrant { maximum_items: 0, ..funded(disposer.retirement_demands(&owner, maximum_bytes).expect("quoted close"), maximum_bytes) };
    let zero = disposer.close_step(&mut owner, unfunded).expect("zero grant");
    let copied_items = match zero {
        PluginLifecycleStep::Progress(progress) if progress == RetainedCloneProgress::default() => progress.copied_items,
        other => panic!("zero grant advanced retirement: {other:?}"),
    };
    let mut held_read_blocks = false;
    for _ in 0..maximum_steps {
        let grant = funded(disposer.retirement_demands(&owner, maximum_bytes).expect("quoted close with held read"), maximum_bytes);
        match disposer.close_step(&mut owner, grant).expect("close with held read") {
            PluginLifecycleStep::Blocked { .. } => {
                held_read_blocks = true;
                break;
            }
            PluginLifecycleStep::Progress(progress) => assert!(progress.fits(grant)),
            PluginLifecycleStep::AwaitingInput { reason } => panic!("transient read retirement unexpectedly awaited input: {reason}"),
            PluginLifecycleStep::Complete(_) => panic!("transient read authority was discarded before release"),
        }
    }
    drop(read);
    let mut released_read_completes = false;
    for _ in 0..maximum_steps {
        let grant = funded(disposer.retirement_demands(&owner, maximum_bytes).expect("quoted close after read release"), maximum_bytes);
        match disposer.close_step(&mut owner, grant).expect("close after read release") {
            PluginLifecycleStep::Complete(progress) => {
                assert!(progress.fits(grant));
                released_read_completes = true;
                break;
            }
            PluginLifecycleStep::Progress(progress) => assert!(progress.fits(grant)),
            PluginLifecycleStep::AwaitingInput { reason } => panic!("released transient read unexpectedly awaited input: {reason}"),
            PluginLifecycleStep::Blocked { .. } => {}
        }
    }
    let actual = serde_json::json!({
        "heldReadBlocks": held_read_blocks,
        "releasedReadCompletes": released_read_completes,
        "terminalIsEmpty": disposer.terminal_is_empty(&owner),
        "zeroGrantCopiedItems": copied_items,
    });
    let expected = serde_json::json!({
        "heldReadBlocks": vector["heldReadBlocks"],
        "releasedReadCompletes": vector["releasedReadCompletes"],
        "terminalIsEmpty": vector["terminalIsEmpty"],
        "zeroGrantCopiedItems": vector["zeroGrantCopiedItems"],
    });
    assert_eq!(actual, expected);
}
