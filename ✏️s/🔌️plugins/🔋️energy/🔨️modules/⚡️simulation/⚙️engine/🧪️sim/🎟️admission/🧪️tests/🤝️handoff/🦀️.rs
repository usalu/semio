//! 🤝️ The public numerical admission handoff leaves no pending constructor state.

use semio_framework_job::{default_now_us, Generation, InteractiveJob, Operation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, RetainedCloneGrant, RevisionId};
use semio_s_plugin_energy_engine_sim::EnergyJobAdmission;
use semio_s_plugin_energy_engine_sim_wire::{EnergyNumericalBounds, EnergyNumericalCensus};

#[path = "../🧫️fixtures/🦀️.rs"]
mod fixture;
use fixture::inputs;

#[test]
fn numerical_admission_handoff_leaves_terminal_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let (model, config) = inputs(&fixture);
    let observed = serde_json::to_value(EnergyNumericalCensus::observe(&model, &config).unwrap()).unwrap();
    for (key, expected) in fixture["expected"].as_object().unwrap() {
        assert_eq!(&observed[key], expected);
    }
    let (mut model, mut config) = (Some(model), Some(config));
    let stock = RetainedCloneGrant { maximum_items: 1 << 22, maximum_copy_bytes: 1 << 29, maximum_capacity_bytes: 1 << 27, maximum_release_bytes: 1 << 27, maximum_depth: 4096 };
    let grant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 128 };
    let operation = Operation::new(OperationId(91002), RevisionId(7), Generation(13), 32);
    let mut caller = OriginalJobFixtureCaller::admit(operation.operation, operation.generation, stock, default_now_us, u64::MAX, 1 << 28);
    let mut decoder = OriginalJobDecodeOwner::empty();
    for _ in 0..16 {
        if caller.turn(grant, |cx| decoder.admit_step(cx)).unwrap() {
            break;
        }
    }
    assert!(decoder.source_loan_admitted());
    assert!(decoder.work_issuer_admitted());
    let mut owner = None;
    assert!(caller.turn_with_decoder(grant, &mut decoder, |cx| EnergyJobAdmission::begin(operation, EnergyNumericalBounds::default(), &mut model, &mut config, &mut owner, cx)).unwrap());
    let owner = owner.as_mut().unwrap();
    for _ in 0..10000 {
        if caller.turn_with_decoder(grant, &mut decoder, |cx| owner.step(cx)).unwrap() {
            break;
        }
    }
    assert!(owner.ready());
    let mut job = None;
    assert!(caller.turn_with_decoder(grant, &mut decoder, |cx| owner.take(&mut job, cx)).unwrap());
    let empty_after_handoff = owner.terminal_is_empty();
    let mut job = job.unwrap();
    for _ in 0..10000 {
        if job.terminal_is_empty() {
            break;
        }
        caller.retirement_turn_with_decoder(grant, &mut decoder, |cx| job.close_step(cx).map(|_| ())).unwrap();
    }
    assert!(job.terminal_is_empty());
    for _ in 0..10000 {
        if owner.terminal_is_empty() {
            break;
        }
        caller.retirement_turn_with_decoder(grant, &mut decoder, |cx| owner.close_step(cx).map(|_| ())).unwrap();
    }
    for _ in 0..10000 {
        if decoder.terminal_is_empty() {
            break;
        }
        caller.retirement_turn(grant, |cx| decoder.close_on_source(cx).map(|_| ())).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(stock);
    assert!(empty_after_handoff, "the numerical job must not move while admission owns constructor state");
}
