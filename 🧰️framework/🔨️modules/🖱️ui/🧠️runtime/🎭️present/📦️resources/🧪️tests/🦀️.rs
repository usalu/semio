//! 🧪️ One schema-issued caller proves original native pages survive paid aliases and terminal closure.

use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_ui_runtime::{UiNativeMeshResource, UiNativeResourceLoans, UiNativeResources};
use ui_scene::math::*;

#[global_allocator]
static NATIVE_RESOURCE_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 };

fn clock() -> Option<u64> { Some(1000) }

fn grant(value: &serde_json::Value) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: value["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: value["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: value["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: value["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: value["maximumDepth"].as_u64().unwrap() as usize }
}

fn paid<R>(caller: &mut OriginalJobFixtureCaller, run: impl FnOnce(RetainedCloneGrant) -> (R, RetainedCloneProgress)) -> R {
    caller.turn(TURN, |cx| {
        assert_eq!(cx.operation(), OperationId(941));
        assert_eq!(cx.generation(), Generation(17));
        assert!(!cx.should_yield());
        let ((result, receipt), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| run(cx.retained_grant()));
        assert_eq!((receipt.retained_capacity_bytes, receipt.released_bytes), (heap.requested_bytes, heap.released_bytes));
        cx.consume_retained(receipt)?;
        if receipt != RetainedCloneProgress::default() { cx.consume_fuel(1); }
        Ok(result)
    }).unwrap()
}

fn paid_context<R>(caller: &mut OriginalJobFixtureCaller, run: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> Result<R, semio_framework_job::OriginalTurnFailure>) -> R {
    caller.turn(TURN, |cx| {
        let before = cx.retained_progress();
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| run(cx));
        let after = cx.retained_progress();
        assert_eq!((after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes), (heap.requested_bytes, heap.released_bytes));
        result
    }).unwrap()
}

#[test]
fn native_resource_owner_and_loan_bundle_preserve_original_pages_and_full_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/🧫️fixtures/🔣️.json")).unwrap();
    let policy = &fixture["caller"];
    assert_eq!(TURN, grant(&policy["turn"]));
    assert_eq!(clock(), policy["clock"].as_u64());
    let stock = grant(&policy["stock"]);
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()), Generation(policy["generation"].as_u64().unwrap()), stock, clock, policy["deadline"].as_u64().unwrap(), policy["fuel"].as_u64().unwrap());
    let token = paid(&mut caller, |grant| mesh3d_begin(17, 23, Mesh3dSchema::triangle_mesh(3, 3), grant).unwrap());
    loop { if paid(&mut caller, |grant| { let step = mesh3d_allocate_step(token, grant).unwrap(); (step.complete, step.progress) }) { break; } }
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    for position in positions { paid(&mut caller, |grant| { assert!(grant.maximum_copy_bytes >= 12); mesh3d_write_vec3(token, Mesh3dField::Positions, position).unwrap(); ((), RetainedCloneProgress { copied_items: 1, copied_bytes: 12, ..Default::default() }) }); }
    for _ in 0..3 { paid(&mut caller, |grant| { assert!(grant.maximum_copy_bytes >= 12); mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0]).unwrap(); ((), RetainedCloneProgress { copied_items: 1, copied_bytes: 12, ..Default::default() }) }); }
    for index in 0..3 { paid(&mut caller, |grant| { assert!(grant.maximum_copy_bytes >= 4); mesh3d_write_u32(token, Mesh3dField::Indices, index).unwrap(); ((), RetainedCloneProgress { copied_items: 1, copied_bytes: 4, ..Default::default() }) }); }
    let lease = paid(&mut caller, |_| (mesh3d_seal(token).unwrap(), RetainedCloneProgress { copied_items: 1, ..Default::default() }));
    let original_page = lease.with_original_page(0, |page| page.as_ptr() as usize).unwrap();
    assert_eq!(lease.original_page_count().unwrap(), 1);
    let identity = Mesh3dResourceIdentity { operation: 941, generation: 17, revision: 23, cache_epoch: 1, source_slot: 0 };
    let mut source = Some(lease);
    let owner = paid(&mut caller, |grant| { let (owner, receipt) = Mesh3dResourceOwner::admit(&mut source, identity, grant).unwrap().unwrap(); (owner, receipt) });
    assert!(source.is_none());
    assert_eq!(mesh3d_begin_close(lease), Err(Mesh3dFault::Order));
    let mut surface = ui_contract::UiText::default();
    let mut mesh_key = ui_contract::UiText::default();
    for (text, target) in [("original-preview", &mut surface), ("original-native-source-with-an-incremental-key-longer-than-sixty-four-original-bytes", &mut mesh_key)] {
        for chunk in text.as_bytes().chunks(TURN.maximum_copy_bytes) {
            let text = std::str::from_utf8(chunk).unwrap();
            paid(&mut caller, |grant| { assert!(grant.maximum_copy_bytes >= text.len()); std::fmt::Write::write_str(target, text).unwrap(); ((), RetainedCloneProgress { copied_items: 1, copied_bytes: text.len(), ..Default::default() }) });
        }
    }
    let mut row = Some(paid(&mut caller, |_| (UiNativeMeshResource { surface, mesh_key, owner }, RetainedCloneProgress { copied_items: 1, ..Default::default() })));
    let mut resources = UiNativeResources::default();
    while row.is_some() { paid(&mut caller, |grant| { let step = resources.admit_row(&mut row, grant).unwrap(); (step.complete, step.progress) }); }
    let mut first = UiNativeResourceLoans::default();
    while !first.capture_complete() { paid_context(&mut caller, |cx| { first.capture_step_on_original(&resources, cx)?; Ok(()) }); }
    let mut prepared = UiNativeResourceLoans::default();
    while !prepared.capture_complete() { paid_context(&mut caller, |cx| { prepared.capture_step_on_original(&first, cx)?; Ok(()) }); }
    let descriptor = paid_context(&mut caller, |cx| first.describe_mesh_on_original(0, cx)).unwrap();
    assert_eq!(descriptor.geometry_bytes, 84);
    assert_eq!(descriptor.component_counts, [0; 3]);
    let transport: serde_json::Value = serde_json::from_str(include_str!("../📤️transport/🧫️fixtures/🔣️.json")).unwrap();
    let expected: Vec<u8> = transport["geometry"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let mut geometry = [0u8; 84];
    let mut offset = 0usize;
    while offset < geometry.len() {
        let chunk = paid_context(&mut caller, |cx| first.read_mesh_chunk_on_original(0, descriptor, semio_framework_ui_runtime::UiNativeMeshChannel::Geometry, offset, &mut geometry[offset..], cx)).unwrap();
        assert!(chunk.bytes > 0 && chunk.bytes <= 64);
        offset += chunk.bytes;
    }
    assert_eq!(geometry.as_slice(), expected.as_slice());
    let original_geometry = geometry;
    paid_context(&mut caller, |cx| cx.with_original_work_loan(0, 1500, |same| {
        assert!(first.read_mesh_chunk_on_original(0, descriptor, semio_framework_ui_runtime::UiNativeMeshChannel::Geometry, 0, &mut geometry, same)?.is_none());
        Ok(())
    }));
    assert_eq!(geometry, original_geometry);
    assert_eq!(first.row(0).unwrap().mesh_key.as_str(), resources.row(0).unwrap().mesh_key.as_str());
    assert_eq!(prepared.row(0).unwrap().loan.lease(), Some(lease));
    assert_eq!(prepared.row(0).unwrap().loan.lease().unwrap().with_original_page(0, |page| page.as_ptr() as usize).unwrap(), original_page);
    let before = caller.cumulative_receipt();
    caller.turn(TURN, |cx| cx.with_original_work_loan(0, 1500, |same| { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| resources.close_step_on_original(same)); assert_eq!(heap.requested_bytes, 0); assert_eq!(heap.released_bytes, 0); assert_eq!(step.unwrap(), None); Ok(()) })).unwrap();
    assert_eq!(caller.cumulative_receipt(), before);
    paid_context(&mut caller, |cx| { assert_eq!(resources.close_step_on_original(cx)?, Some(false)); Ok(()) });
    let before = caller.cumulative_receipt();
    caller.turn(TURN, |cx| { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| resources.close_step_on_original(cx)); assert_eq!(heap.requested_bytes, 0); assert_eq!(heap.released_bytes, 0); assert_eq!(step.unwrap(), Some(false)); Ok(()) }).unwrap();
    assert_eq!(caller.cumulative_receipt(), before);
    assert_eq!(prepared.row(0).unwrap().loan.loan(TURN).unwrap_err(), Mesh3dFault::Closing);
    caller.cancel().cancel();
    for _ in 0..4096 { if first.terminal_is_empty() { break; } paid_context(&mut caller, |cx| { first.close_step_on_original(cx)?; Ok(()) }); }
    assert!(first.terminal_is_empty());
    assert_eq!(lease.with_original_page(0, |page| page.as_ptr() as usize).unwrap(), original_page);
    for _ in 0..4096 { if prepared.terminal_is_empty() { break; } paid_context(&mut caller, |cx| { prepared.close_step_on_original(cx)?; Ok(()) }); }
    assert!(prepared.terminal_is_empty());
    for _ in 0..4096 { if resources.terminal_is_empty() { break; } paid_context(&mut caller, |cx| { resources.close_step_on_original(cx)?; Ok(()) }); }
    assert!(resources.terminal_is_empty());
    assert!(mesh3d_terminal_is_empty(lease));
    let spent = caller.cumulative_receipt();
    let roots = caller.born_bytes();
    assert_eq!(spent.retained_capacity_bytes - roots, spent.released_bytes);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop((first, prepared, resources, row, source)));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    caller.retire(TURN);
    println!("[DEBUG] native same-arena original pages/paid aliases/canceled complete row-page close/one finite caller/Drop0");
}
