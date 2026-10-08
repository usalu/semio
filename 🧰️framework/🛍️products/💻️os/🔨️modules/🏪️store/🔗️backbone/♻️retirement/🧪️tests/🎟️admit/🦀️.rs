use super::super::*;

#[test]
fn backbone_retirement_installed_admission_keeps_original_backbone_after_empty_queue() {
    let mut queue = VecDeque::with_capacity(8192);
    let queue_capacity = queue.capacity();
    let uri = String::with_capacity(32768);
    let uri_pointer = uri.as_ptr();
    let uri_capacity = uri.capacity();
    let mut backbone = Some(Backbones::Port(super::super::super::PortBackbone { uri, channel: None }));
    let frame = ArtifactStoreBackboneRetirement::constructor_capacity_bytes();
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: frame, maximum_depth: 1, ..Default::default() };
    let mut released = 0;
    for phase in 0..2 {
        for grant in [RetainedCloneGrant { maximum_items: 0, ..birth }, RetainedCloneGrant { maximum_depth: 0, ..birth }, RetainedCloneGrant { maximum_capacity_bytes: frame - 1, ..birth }] {
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreBackboneRetirement::admit_attached(&mut queue, &mut backbone, grant));
            assert!(result.is_err());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let Some(Backbones::Port(port)) = backbone.as_ref() else { panic!("original backbone remains attached before frame admission") };
            assert_eq!(port.uri.as_ptr(), uri_pointer);
            assert_eq!(port.uri.capacity(), uri_capacity);
        }
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreBackboneRetirement::admit_attached(&mut queue, &mut backbone, birth));
        let (mut owner, progress) = result.unwrap();
        assert_eq!((heap.requested_bytes, heap.released_bytes), (frame, 0));
        assert!(progress.fits(birth));
        assert_eq!(backbone.is_some(), phase == 0);
        assert_eq!(queue.capacity(), 0);
        for turn in 0..100_000 {
            if owner.is_none() { break; }
            let demand = semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(), 17).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 17, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::close_factory_ticket(&mut owner, grant).unwrap());
            assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes);
            assert_eq!(heap.released_bytes, step.progress().released_bytes);
            assert!(step.progress().fits(grant));
            released += heap.released_bytes;
            assert!(turn < 99_999);
        }
        assert!(owner.is_none());
    }
    assert_eq!(released, 2 * frame + queue_capacity * size_of::<BackboneMessage>() + uri_capacity);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop((queue, backbone)));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] installed backbone preserved original URI pointer across empty queue closure; exact release={released}");
}

#[test]
fn backbone_retirement_installed_admission_preserves_empty_queue_capacity_until_full_grant() {
    let mut queue = VecDeque::with_capacity(8192);
    let capacity = queue.capacity();
    let pointer = queue.as_slices().0.as_ptr();
    let mut backbone = None;
    let frame = ArtifactStoreBackboneRetirement::constructor_capacity_bytes();
    let full = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: frame, maximum_depth: 1, ..Default::default() };
    for grant in [RetainedCloneGrant { maximum_items: 0, ..full }, RetainedCloneGrant { maximum_depth: 0, ..full }, RetainedCloneGrant { maximum_capacity_bytes: frame - 1, ..full }] {
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreBackboneRetirement::admit_attached(&mut queue, &mut backbone, grant));
        assert!(result.is_err());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(queue.capacity(), capacity);
        assert_eq!(queue.as_slices().0.as_ptr(), pointer);
    }
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreBackboneRetirement::admit_attached(&mut queue, &mut backbone, full));
    let (mut owner, receipt) = result.unwrap();
    assert!(owner.is_some());
    assert_eq!(queue.capacity(), 0);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (frame, 0));
    assert_eq!(receipt.retained_capacity_bytes, frame);
    assert!(receipt.fits(full));
    let demand = semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(), 0).unwrap();
    assert_eq!(demand.release_bytes, capacity * size_of::<BackboneMessage>());
    let closing = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth, ..Default::default() };
    for grant in [RetainedCloneGrant { maximum_items: 0, ..closing }, RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..closing }] {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::close_factory_ticket(&mut owner, grant).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert!(owner.is_some());
    }
    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::close_factory_ticket(&mut owner, closing).unwrap());
    assert_eq!(heap.released_bytes, demand.release_bytes);
    assert_eq!(step.progress().released_bytes, heap.released_bytes);
    assert!(step.progress().fits(closing));
    let demand = semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(), 0).unwrap();
    assert_eq!(demand.release_bytes, frame);
    let closing = RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: frame, maximum_depth: demand.depth, ..Default::default() };
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::close_factory_ticket(&mut owner, closing).unwrap());
    assert_eq!(heap.released_bytes, frame);
    assert!(owner.is_none());
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreBackboneRetirement::admit_attached(&mut queue, &mut backbone, full));
    assert!(result.unwrap().0.is_none());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] installed backbone original empty queue capacity={capacity} admitted frame={frame} with separate exact release");
}

#[test]
fn backbone_retirement_constructor_preserves_original_queue_on_every_refusal() {
    let mut queue = VecDeque::with_capacity(8192);
    queue.push_back(BackboneMessage::Genesis { pack: vec![1; 65536] });
    let original = queue.as_slices().0.as_ptr();
    let original_capacity = queue.capacity();
    let capacity = ArtifactStoreBackboneRetirement::constructor_capacity_bytes();
    let full = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: capacity, maximum_release_bytes: 0, maximum_depth: 1 };
    for pause in [RetainedCloneGrant { maximum_items: 0, ..full }, RetainedCloneGrant { maximum_depth: 0, ..full }, RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..full }] {
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreBackboneRetirement::admit_queue(queue, pause));
        let Err((_, retained)) = result else { panic!("original queue is preserved before exact frame admission") };
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(retained.as_slices().0.as_ptr(), original);
        assert_eq!(retained.capacity(), original_capacity);
        queue = retained;
    }
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ArtifactStoreBackboneRetirement::admit_queue(queue, full));
    let Ok((owner, receipt)) = result else { panic!("exact original frame is admitted") };
    assert_eq!(heap.requested_bytes, capacity);
    assert_eq!(heap.released_bytes, 0);
    assert_eq!(receipt.retained_capacity_bytes, capacity);
    assert!(receipt.fits(full));
    let mut owner = Some(owner);
    let mut released = 0;
    let mut turns = 0;
    while owner.is_some() {
        let demand = semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(), 0).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_value::close_factory_ticket(&mut owner, grant).unwrap());
        assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes);
        assert_eq!(heap.released_bytes, step.progress().released_bytes);
        assert!(step.progress().fits(grant));
        released += heap.released_bytes;
        turns += 1;
        assert!(turns < 100_000);
    }
    assert_eq!(released, capacity + original_capacity * size_of::<BackboneMessage>() + 65536);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    eprintln!("[DEBUG] backbone constructor refused original queue unchanged then admitted frame={capacity} exact released={released}");
}
