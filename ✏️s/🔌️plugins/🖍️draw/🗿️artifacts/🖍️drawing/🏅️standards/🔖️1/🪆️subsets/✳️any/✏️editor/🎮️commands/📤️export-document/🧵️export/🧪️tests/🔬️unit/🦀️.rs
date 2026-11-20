//! 🧪️ Original export sources survive interruption and every denied retirement currency.
use super::*;
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId, OriginalJobFixtureCaller, OriginalRetirementPump};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::{ErasedSnapshotRetirement, OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand};

type Store = store::ArtifactStore<DrawingSnapshot, semio_s_artifact_draw_drawing_mutations::DrawingMutation>;

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    }
}

fn effect_turn<T>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> T) -> T {
    let mut turn = OriginalRetirementPump::new().turn(1 << 24, u64::MAX, || Some(0), grant).unwrap();
    turn.with_effect(run).unwrap().unwrap().0
}

fn progress(poll: OriginalRetirementPoll) -> RetainedCloneProgress {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.unwrap().progress(),
        RetainedWorkPoll::Pending { .. } => Default::default(),
    }
}

fn pump_returns(issuer: &mut Store, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>) {
    if active.is_none() && issuer.returned_snapshot_read_count() == 0 { return; }
    let body = active.as_ref().map_or(0, |owner| {
        owner.next_copy_byte_demand().unwrap().max(owner.next_release_byte_demand().unwrap())
    });
    let demand = match active.as_ref() {
        Some(owner) => store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body),
        None => issuer.returned_snapshot_read_retirement_demand(),
    }.unwrap();
    let selected = grant(demand);
    let (receipt, physical) = observe(|| effect_turn(selected, |effect| {
        if active.is_some() {
            return progress(store::os_store::snapshot_read::artifact_retirement_box_close_step(active, body, effect));
        }
        let (owner, receipt) = issuer.take_returned_snapshot_read_retirement(effect).unwrap();
        *active = owner;
        receipt
    }));
    assert!(receipt.fits(selected));
    assert!(!physical.overflowed);
    assert_eq!((physical.requested_bytes, physical.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
}

#[semio_framework_async_macros::async_test]
async fn original_export_read_returns_under_exact_physical_grants() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🏪️stores/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &corpus["readReturn"]["exportConsumer"];
    assert_eq!(law["identity"], "original-read");

    let id = "original-export-source";
    let document = DrawingSnapshot { id: id.into(), ..Default::default() };
    let envelope = store::os_store::document_materialize::create_document_envelope::<DrawingSnapshot, semio_s_artifact_draw_drawing_mutations::DrawingMutation>(
        DRAWING_DOCUMENT_SCHEMA, id, document, None, &store::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    );
    let mut issuer = Store::new::<store::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
    issuer.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners().unwrap());
    let read = issuer.snapshot_read().unwrap();
    let original = read.get() as *const DrawingSnapshot;
    let stock = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 128 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(0x44524558), Generation(issuer.generation().0), stock, || Some(0), u64::MAX, 1 << 24);
    let mut work = Work::new(Default::default(), Default::default());
    work.read = Some(read);
    work.begin_close();
    let mut returned = None;

    for _ in 0..100_000 {
        pump_returns(&mut issuer, &mut returned);
        if work.terminal_is_empty() { break; }
        if let Some(read) = work.read.as_ref() { assert_eq!(read.get() as *const DrawingSnapshot, original); }
        let selected = grant(work.close_demands(4096).unwrap());
        for axis in law["denied"].as_array().unwrap() {
            let mut denied = selected;
            let budget = match axis.as_str().unwrap() {
                "items" => &mut denied.maximum_items,
                "copy" => &mut denied.maximum_copy_bytes,
                "capacity" => &mut denied.maximum_capacity_bytes,
                "release" => &mut denied.maximum_release_bytes,
                "depth" => &mut denied.maximum_depth,
                _ => unreachable!(),
            };
            if *budget == 0 { continue; }
            *budget -= 1;
            caller.turn(selected, |cx| cx.with_original_retirement(|cx| {
                let (result, physical) = observe(|| cx.with_retained_loan(denied, |cx| work.close_step(cx)));
                assert!(!physical.overflowed);
                assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
                match result {
                    Ok(step) => assert_eq!(step.progress(), Default::default()),
                    Err(error) => assert_eq!(error.effects, Default::default()),
                }
                Ok(())
            })).unwrap();
            if let Some(read) = work.read.as_ref() { assert_eq!(read.get() as *const DrawingSnapshot, original); }
        }
        caller.turn(selected, |cx| cx.with_original_retirement(|cx| {
            let (result, physical) = observe(|| work.close_step(cx));
            let step = result?;
            assert!(matches!(step, InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. }));
            let receipt = step.progress();
            assert!(receipt.fits(selected));
            assert!(!physical.overflowed);
            assert_eq!((physical.requested_bytes, physical.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
            Ok(())
        })).unwrap();
    }
    assert!(work.terminal_is_empty());
    for _ in 0..100_000 {
        if returned.is_none() && issuer.returned_snapshot_read_count() == 0 { break; }
        pump_returns(&mut issuer, &mut returned);
    }
    assert!(returned.is_none() && issuer.returned_snapshot_read_count() == 0);
    let (_, physical) = observe(|| drop(work));
    assert_eq!((physical.requested_bytes, physical.released_bytes), (0, 0));
    caller.retire_exact();

    for _ in 0..100_000 {
        if issuer.close_owned_terminal_is_empty() { break; }
        let selected = grant(issuer.close_owned_demands(4096).unwrap());
        effect_turn(selected, |effect| progress(issuer.close_owned_step(effect)));
    }
    assert!(issuer.close_owned_terminal_is_empty());
}
