use super::*;
use crate::os_store::bounded_artifact_store_owners;
use super::super::{tests::DemoSnapshot, fixture_mutations::demo::DemoMutation};

#[test]
fn original_retained_codec_catalog_preserves_input_receipt_and_cancellation_heap() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let policy = &fixture["grant"];
    let n = |axis: &str| policy[axis].as_u64().unwrap() as usize;
    let grant = RetainedCloneGrant { maximum_items: n("items"), maximum_copy_bytes: n("copy"), maximum_capacity_bytes: n("capacity"), maximum_release_bytes: n("release"), maximum_depth: n("depth") };
    for prepared_turns in [0, 1, 2, 32] {
        let pack = b"original-pack";
        let spr = b"original-spr";
        let command = b"original-command";
        let (mut owner, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| RetainedArtifactCodecCatalog::<DemoSnapshot, DemoMutation>::new(pack, spr, command));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let original = owner.sources().map(|source| source.as_ptr());
        let mut births = 0usize;
        let mut releases = 0usize;
        for _ in 0..prepared_turns {
            if owner.is_prepared() { break; }
            let source = || crate::os_store::bounded_artifact_store_owners_source_demands::<DemoSnapshot, DemoMutation>();
            let demand = owner.demands(grant.maximum_copy_bytes, source).unwrap();
            let pointers = owner.original.as_ref().map(|catalog| [std::sync::Arc::as_ptr(catalog.snapshot_retirement.0.as_ref().unwrap()) as *const () as usize, std::sync::Arc::as_ptr(catalog.initial_snapshot_retirement.0.as_ref().unwrap()) as *const () as usize, std::sync::Arc::as_ptr(catalog.mutation_retirement.0.as_ref().unwrap()) as *const () as usize]);
            for axis in 0..5 {
                if axis == 1 && demand.copy_bytes == 0 || axis == 2 && demand.capacity_bytes == 0 || axis == 3 && demand.release_bytes == 0 { continue; }
                let denied = match axis { 0 => RetainedCloneGrant { maximum_items: 0, ..grant }, 1 => RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, 2 => RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }, 3 => RetainedCloneGrant { maximum_release_bytes: 0, ..grant }, _ => RetainedCloneGrant { maximum_depth: 0, ..grant } };
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(denied, source, bounded_artifact_store_owners::<DemoSnapshot, DemoMutation>));
                assert_eq!(result.unwrap(), ArtifactCodecCatalogStep::Blocked);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert!(owner.receipt().is_none());
                assert_eq!(owner.sources().map(|source| source.as_ptr()), original);
                assert_eq!(owner.original.as_ref().map(|catalog| [std::sync::Arc::as_ptr(catalog.snapshot_retirement.0.as_ref().unwrap()) as *const () as usize, std::sync::Arc::as_ptr(catalog.initial_snapshot_retirement.0.as_ref().unwrap()) as *const () as usize, std::sync::Arc::as_ptr(catalog.mutation_retirement.0.as_ref().unwrap()) as *const () as usize]), pointers);
            }
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(grant, source, bounded_artifact_store_owners::<DemoSnapshot, DemoMutation>));
            assert!(matches!(result.unwrap(), ArtifactCodecCatalogStep::Progress | ArtifactCodecCatalogStep::Ready));
            let (blocked, unchanged) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.advance(grant, source, bounded_artifact_store_owners::<DemoSnapshot, DemoMutation>));
            assert_eq!(blocked.unwrap(), ArtifactCodecCatalogStep::Blocked);
            assert_eq!((unchanged.requested_bytes, unchanged.released_bytes), (0, 0));
            assert!(!owner.is_prepared());
            let (received_grant, receipt) = owner.take_receipt().unwrap();
            assert_eq!(received_grant, grant);
            assert!(receipt.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
            births += heap.requested_bytes; releases += heap.released_bytes;
        }
        let mut recipient = None;
        if prepared_turns == 32 && owner.is_prepared() {
            for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: 1, ..grant }] {
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take_prepared(&mut recipient, denied));
                assert_eq!(result.unwrap(), ArtifactCodecCatalogStep::Blocked);
                assert!(recipient.is_none());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.take_prepared(&mut recipient, grant));
            assert_eq!(result.unwrap(), ArtifactCodecCatalogStep::Ready);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!owner.terminal_is_empty());
            let (received_grant, receipt) = owner.take_receipt().unwrap();
            assert_eq!(received_grant, grant);
            assert_eq!(receipt, item());
            assert!(owner.terminal_is_empty());
        } else { owner.cancel(); }
        let mut turns = 0;
        while !owner.terminal_is_empty() {
            assert!(turns < 4096);
            let demand = owner.close_demands(grant.maximum_copy_bytes).unwrap();
            for axis in 0..5 {
                if axis == 1 && demand.copy_bytes == 0 || axis == 2 && demand.capacity_bytes == 0 || axis == 3 && demand.release_bytes == 0 { continue; }
                let denied = match axis { 0 => RetainedCloneGrant { maximum_items: 0, ..grant }, 1 => RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, 2 => RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }, 3 => RetainedCloneGrant { maximum_release_bytes: 0, ..grant }, _ => RetainedCloneGrant { maximum_depth: 0, ..grant } };
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(denied));
                assert_eq!(result.unwrap(), ArtifactCodecCatalogStep::Blocked);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
            result.unwrap();
            assert!(!owner.terminal_is_empty());
            let (blocked, unchanged) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_step(grant));
            assert_eq!(blocked.unwrap(), ArtifactCodecCatalogStep::Blocked);
            assert_eq!((unchanged.requested_bytes, unchanged.released_bytes), (0, 0));
            let (received_grant, receipt) = owner.take_receipt().unwrap();
            assert_eq!(received_grant, grant);
            assert!(receipt.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
            births += heap.requested_bytes; releases += heap.released_bytes; turns += 1;
            assert_eq!(owner.sources().map(|source| source.as_ptr()), original);
        }
        if let Some(mut catalog) = recipient {
            let mut receiver_turns = 0;
            while !catalog.uninstalled_owners_terminal_is_empty() {
                assert!(receiver_turns < 4096);
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| catalog.close_uninstalled_owners_step(grant));
                let receipt = result.unwrap().progress();
                assert!(receipt.fits(grant));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
                births += heap.requested_bytes; releases += heap.released_bytes; receiver_turns += 1;
            }
            let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(catalog));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(births, releases);
        println!("[DEBUG] original retained codec catalog preparedTurns={prepared_turns} closeTurns={turns} births={births} releases={releases} terminalDrop=0 originalInputPointers=unchanged fixedGrant={grant:?}");
    }
}
