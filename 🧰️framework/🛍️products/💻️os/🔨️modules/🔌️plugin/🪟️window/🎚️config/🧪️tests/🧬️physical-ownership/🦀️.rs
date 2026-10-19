//! ⚖️ Physical ownership laws of the retained window-config state, hosted by the facade test build (they name the window-config test fixtures).
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_window_config::retained::{ErasedWindowConfigPackLoad, RetainedStatePhase, RetainedWindowConfigStateDecode, RetainedWindowConfigTypedState, ValueFrame, controlled_demands};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_kernel as store;
use crate::window_config_tests::pack_identity_tests::IdentityWindowOwner;

#[test]
fn window_config_paged_registry_actual_authored_root_metadata_producer() {
    use crate::window_config_tests::retained_pack_load_tests::RetainedLoadCameraConfig;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🗂️registry/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["rootMetadata"];
    let ((producer, absent), getter) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| (
        <RetainedLoadCameraConfig as pack::snapshot::ArtifactPack>::record_spec_producer(),
        <semio_framework_value::DslValue as pack::snapshot::ArtifactPack>::record_spec_producer(),
    ));
    assert_eq!(producer.is_some(), law["ownedProducer"].as_bool().unwrap());
    assert_eq!(absent.is_some(), law["schemaLessProducer"].as_bool().unwrap());
    assert_eq!((getter.requested_bytes, getter.released_bytes), (law["getterAllocationBytes"].as_u64().unwrap() as usize, law["getterReleaseBytes"].as_u64().unwrap() as usize));
    assert_eq!(<RetainedLoadCameraConfig as pack::snapshot::ArtifactDsl>::envelope_id(), law["owner"].as_str().unwrap());
    let mut accepted = |_| true;
    let mut native = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut accepted);
    native.admit_turn_capacity(law["maximumCapacityBytes"].as_u64().unwrap() as usize).unwrap();
    let (spec, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| producer.unwrap().decode(&mut native).unwrap());
    assert_eq!(birth.requested_bytes, native.owned_bytes());
    assert_eq!(birth.released_bytes, 0);
    assert_eq!(spec.fields.iter().map(|field| field.key.as_str()).collect::<Vec<_>>(), law["fields"].as_array().unwrap().iter().map(|field| field.as_str().unwrap()).collect::<Vec<_>>());
    let mut owner = ControlledRetirement::new(spec).map_err(|(error, _)| error).unwrap();
    let mut allocated = birth.requested_bytes;
    let mut released = 0;
    let body = fixture["maximumPageBytes"].as_u64().unwrap() as usize;
    while !owner.terminal_is_empty() {
        let demand = controlled_demands(&owner, body).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(grant));
        let progress = result.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        allocated += heap.requested_bytes;
        released += heap.released_bytes;
    }
    assert_eq!(allocated, released);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}

#[test]
fn window_config_paged_registry_retained_source_reservation_uses_capacity_authority() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🗂️registry/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let body = fixture["maximumPageBytes"].as_u64().unwrap() as usize;
    let mut decoder = RetainedWindowConfigStateDecode::<IdentityWindowOwner>::new();
    decoder.phase = RetainedStatePhase::Ingress;
    *decoder.source = Some(store::mounted_pack_rt::RetainedPackSourceCursor::try_new(1, 1, 65_536).unwrap());
    let capacity = decoder.source.as_ref().unwrap().next_allocation_bytes().unwrap();
    let refused_law = &fixture["sourceReservation"][0];
    let admitted_law = &fixture["sourceReservation"][1];
    let refused = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: refused_law["copyBytes"].as_u64().unwrap() as usize, maximum_depth: 64, ..Default::default() };
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| decoder.advance(&[7], refused));
    assert_eq!(result.unwrap(), refused_law["reserved"].as_bool().unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert!(!decoder.source.as_ref().unwrap().has_reserved_page());
    let admitted = RetainedCloneGrant { maximum_copy_bytes: admitted_law["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: capacity, ..refused };
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| decoder.advance(&[7], admitted));
    assert_eq!(result.unwrap(), admitted_law["reserved"].as_bool().unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (capacity, 0));
    assert_eq!(decoder.admitted, admitted_law["copiedBytes"].as_u64().unwrap() as usize);
    let mut released = 0;
    while !decoder.terminal_is_empty() {
        let demand = decoder.retirement_demands(body).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| decoder.close_step(grant));
        let progress = result.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        released += heap.released_bytes;
    }
    assert_eq!(released, capacity);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(decoder));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}

#[test]
fn window_config_paged_registry_actual_inline_parser_retains_original_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🗂️registry/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let body = fixture["maximumPageBytes"].as_u64().unwrap() as usize;
    let mut accepted = |_| true;
    let mut native = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut accepted);
    let (mut parser, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let mut parser = RetainedWindowConfigTypedState::<IdentityWindowOwner>::new(&mut native).unwrap();
        let original_fields = parser.spec.fields.as_ptr();
        parser.begin_container(store::mounted_pack_rt::RetainedValueContainer::Record, 0).unwrap();
        assert_eq!(parser.spec.fields.as_ptr(), original_fields);
        assert!(matches!(parser.stack.last(), Some(ValueFrame::Record { root: true, spec: None, .. })));
        parser
    });
    let mut allocated = birth.requested_bytes;
    let mut released = birth.released_bytes;
    let mut turns = 0;
    while !parser.terminal_is_empty() {
        let demand = parser.retirement_demands(body).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let mut refused = vec![RetainedCloneGrant { maximum_items: 0, ..grant }];
        if demand.copy_bytes > 0 { refused.push(RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant }); }
        if demand.capacity_bytes > 0 { refused.push(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }); }
        if demand.release_bytes > 0 { refused.push(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }); }
        if demand.depth > 0 { refused.push(RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }); }
        for refused in refused {
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parser.close_step(refused));
            assert_eq!(result.unwrap().progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parser.close_step(grant));
        let progress = result.unwrap().progress();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        allocated += heap.requested_bytes;
        released += heap.released_bytes;
        turns += 1;
        assert!(turns < 65_536);
    }
    assert_eq!(allocated, released);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(parser));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
}
