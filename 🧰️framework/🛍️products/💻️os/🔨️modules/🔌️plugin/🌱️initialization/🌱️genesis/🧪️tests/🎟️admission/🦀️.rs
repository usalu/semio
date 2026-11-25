//! 🧪️ The original document source admits its fixed recipient before verified genesis under the mounted policy.
use crate::fixture::ScalarGenesis;
use semio_framework_job::{close_original_retained_step, Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome};
use semio_os_kernel::ActorId;
use semio_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_plugin_store_lifecycle::original_store_genesis::{OriginalStoreBirth, OriginalStoreGenesis, OriginalStoreSlot};

type Source = OriginalStoreGenesis<ScalarGenesis<false>>;
const MOUNT: MountedOwnerPolicyV1 = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
const MAXIMUM_TURNS: usize = 65_536;

fn caller() -> (OriginalJobFixtureCaller, OriginalJobDecodeOwner) {
    let stock = RetainedCloneGrant {
        maximum_items: 1 << 26, maximum_copy_bytes: 1 << 30,
        maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 29, maximum_depth: 4096,
    };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(78223), Generation(9), stock, || Some(1000), u64::MAX, 1 << 26);
    caller.retain_dispatch().unwrap();
    let decoder = caller.admitted_decoder();
    (caller, decoder)
}

fn call<T>(caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner, grant: RetainedCloneGrant, run: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.refreshed_turn_with_decoder(grant, decoder, run));
    let receipt = caller.business_receipt().checked_add(caller.construction_receipt()).unwrap();
    assert!(!heap.overflowed);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes), "original source receipt {receipt:?}, failure {:?}", result.as_ref().err());
    result.unwrap()
}

fn born(name: &str, actor: &ActorId, frame: &mut OriginalStoreBirth, caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner) -> Box<Source> {
    let mut candidate = None;
    for _ in 0..MAXIMUM_TURNS {
        call(caller, decoder, MOUNT.preparation, |cx| {
            Source::admit(name, actor, cx, frame, &mut candidate)?;
            if let Some(source) = &mut candidate {
                assert!(source.step(cx)?.is_none());
                assert!(source.step(cx)?.is_none());
            }
            Ok(())
        });
        if let Some(source) = candidate {
            return source;
        }
    }
    panic!("original source birth did not complete")
}

fn finish(source: Box<Source>, frame: &mut OriginalStoreBirth, caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner) {
    let mut source = Some(source);
    for _ in 0..MAXIMUM_TURNS {
        if frame.terminal_is_empty() { break; }
        call(caller, decoder, MOUNT.close, |cx| cx.with_original_retirement(|cx| frame.close_step(cx)));
    }
    assert!(frame.terminal_is_empty());
    for _ in 0..MAXIMUM_TURNS {
        let owner = source.as_mut().unwrap();
        if owner.terminal_is_empty() { break; }
        call(caller, decoder, MOUNT.close, |cx| cx.with_original_retirement(|cx| owner.close_step(cx)));
    }
    assert!(source.as_ref().unwrap().terminal_is_empty());
    let demand = Source::boxed_release_demands();
    call(caller, decoder, MOUNT.close, |cx| cx.with_original_retirement(|cx| {
        close_original_retained_step(cx, demand, |_| {
            source.take();
            RetainedWorkOutcome {
                result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress {
                    copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default()
                })),
                work: 1,
            }
        })
    }));
    assert!(source.is_none());
}

fn retire(mut caller: OriginalJobFixtureCaller, mut decoder: OriginalJobDecodeOwner) {
    for _ in 0..MAXIMUM_TURNS {
        if decoder.terminal_is_empty() { break; }
        caller.refreshed_turn(MOUNT.close, |cx| cx.with_original_retirement(|cx| decoder.close_on_source(cx))).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(MOUNT.close);
}

#[test]
fn original_document_genesis_completes_under_the_actual_mounted_header_policy() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    assert_eq!(law["maximumHeaderCopyBytes"], MOUNT.preparation.maximum_copy_bytes);
    assert_eq!(law["historyShape"], "Document");
    let (mut caller, mut decoder) = caller();
    let actor = ActorId("actual-mount".into());
    let mut frame = OriginalStoreBirth::default();
    let mut source = born("native-document", &actor, &mut frame, &mut caller, &mut decoder);
    let mut result = None;
    for _ in 0..MAXIMUM_TURNS {
        let phase = source.progress().0;
        result = call(&mut caller, &mut decoder, MOUNT.preparation, |cx| source.step(cx));
        if result.is_some() {
            assert_eq!((phase, source.progress().0), (8, 9));
            break;
        }
    }
    let (address, store) = result.unwrap_or_else(|| panic!("native document genesis stalled {:?}", source.progress()));
    assert_eq!(address, "native-document");
    assert_eq!(store.envelope().history_shape, semio_os_kernel::HistoryShape::Document);
    assert_eq!(store.snapshot().unwrap().zoom, 1.25);
    assert!(store.owners.snapshot_read_leases.authority_matches(store.cursor.generation, store.cursor.content_revision));
    finish(source, &mut frame, &mut caller, &mut decoder);
    let mut store = OriginalStoreSlot::from_original(store);
    for _ in 0..MAXIMUM_TURNS {
        if store.terminal_is_empty() { break; }
        call(&mut caller, &mut decoder, MOUNT.close, |cx| cx.with_original_retirement(|cx| store.close_step(cx)));
    }
    assert!(store.terminal_is_empty());
    retire(caller, decoder);
}

#[test]
fn original_pending_initializer_header_denials_and_token_cancellation_keep_actual_custody() {
    let grant = MOUNT.preparation;
    for after_envelope in [false, true] {
        let (mut caller, mut decoder) = caller();
        let actor = ActorId("actual-cancel".into());
        let mut frame = OriginalStoreBirth::default();
        let mut source = born("native-cancel", &actor, &mut frame, &mut caller, &mut decoder);
        for _ in 0..MAXIMUM_TURNS {
            if source.progress().0 == 5 { break; }
            assert!(call(&mut caller, &mut decoder, grant, |cx| source.step(cx)).is_none());
        }
        assert_eq!(source.progress().0, 5);
        for denied in [
            RetainedCloneGrant { maximum_items: 0, ..grant },
            RetainedCloneGrant { maximum_copy_bytes: 0, ..grant },
            RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant },
            RetainedCloneGrant { maximum_depth: 0, ..grant },
        ] {
            assert!(call(&mut caller, &mut decoder, denied, |cx| source.step(cx)).is_none());
            assert_eq!(caller.business_receipt(), RetainedCloneProgress::default());
            assert!(!source.initializer_is_awaiting_envelope());
        }
        for _ in 0..MAXIMUM_TURNS {
            if source.initializer_is_awaiting_envelope() { break; }
            assert!(call(&mut caller, &mut decoder, grant, |cx| source.step(cx)).is_none());
        }
        assert!(source.initializer_is_awaiting_envelope());
        if after_envelope {
            for _ in 0..MAXIMUM_TURNS {
                if !source.initializer_is_awaiting_envelope() { break; }
                assert!(call(&mut caller, &mut decoder, grant, |cx| source.step(cx)).is_none());
            }
            assert!(!source.initializer_is_awaiting_envelope());
            assert_eq!(source.progress().0, 6);
        }
        caller.cancel().cancel_now();
        finish(source, &mut frame, &mut caller, &mut decoder);
        retire(caller, decoder);
    }
}
