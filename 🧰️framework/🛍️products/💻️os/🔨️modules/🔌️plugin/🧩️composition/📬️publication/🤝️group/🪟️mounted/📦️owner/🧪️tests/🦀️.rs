pub(crate) fn test_mounted_private_child_frame<A:ArtifactApp,M:SpaceMember+MemberFactory+'static>(app:&mut VcsArtifactApp<A,M>){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let frame=MountedPrivateChildGroup::<M>::frame_birth_bytes();assert!(frame<=262_144);
    let backing=app.private_child_groups.empty_backing_byte_demand().unwrap();assert!(backing>0&&backing<=262_144);
    for(index,row)in law["cases"].as_array().unwrap().iter().enumerate(){
        let operation=7391+index as u64;let grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:64,maximum_capacity_bytes:if row["capacity"]=="frame"{frame}else{frame-1},maximum_release_bytes:0,maximum_depth:64};
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.admit_private_child_group_frame(operation,0,1_000_000,grant).unwrap());
        assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_eq!(app.private_child_groups.get(operation).is_some(),row["admitted"].as_bool().unwrap());
        if !row["admitted"].as_bool().unwrap(){assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));continue;}
        assert_eq!(heap.requested_bytes,frame);let original=app.private_child_groups.get(operation).unwrap().as_ref()as *const _;
        let context_extent=semio_framework_job::StepContextOwner::birth_bytes();
        let(context,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::StepContextOwner::new(semio_framework_job::OperationId(operation),semio_framework_job::Generation(1),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:context_extent,maximum_depth:1,..Default::default()}));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(context_extent,0));
        let(context,progress)=context.unwrap();assert_eq!(progress.retained_capacity_bytes,context_extent);app.private_child_groups.get_mut(operation).unwrap().context=Some(context);
        let mut sequence=0;
        let mut actual_retained_progress=Default::default();let context=app.private_child_groups.get(operation).unwrap().context.as_ref().unwrap().context(semio_framework_job::StepBudget::new(1,semio_framework_job::default_now_us().unwrap().saturating_add(INTERACTIVE_TURN_WORKER_WALL_US),app.mounted_policy.maintenance),semio_framework_job::root_cancel_token(),semio_framework_job::default_now_us,&mut sequence,&mut actual_retained_progress).unwrap();
        let demand=app.private_child_group_operation_close_demands(operation,64).unwrap();assert_eq!(demand.release_bytes,context_extent);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_operation_step(operation,grant).unwrap());
        assert_eq!(step,semio_framework_job::InteractiveJobCloseStep::Blocked);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert_eq!(app.private_child_groups.get(operation).unwrap().as_ref()as *const _,original);
        drop(context);
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_operation_step(operation,grant).unwrap());
        assert_eq!(step.progress().copied_items,1);assert_eq!(step.progress().released_bytes,context_extent);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,context_extent));
        assert!(app.private_child_groups.get(operation).unwrap().context.is_none());
        assert_eq!(app.private_child_groups.get(operation).unwrap().as_ref()as *const _,original);
        for _ in 0..100000{
            if app.private_child_groups.get(operation).is_none(){break;}
            assert_eq!(app.private_child_groups.get(operation).unwrap().as_ref()as *const _,original);
            let demand=app.private_child_group_close_demands(64).unwrap();assert!(demand.capacity_bytes<=262_144&&demand.release_bytes<=262_144&&demand.copy_bytes<=64&&demand.depth<=64);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];
            if demand.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}
            if demand.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}
            if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}
            for denied in denied{
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(denied).unwrap());
                assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                assert_eq!(app.private_child_groups.get(operation).unwrap().as_ref()as *const _,original);
            }
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(grant).unwrap());
            assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            if step.progress().released_bytes>0{assert_eq!(step.progress().released_bytes,frame);}
        }
        assert!(app.private_child_groups.get(operation).is_none());println!("[DEBUG] Mounted private frame={} originalStablePointer/fundedbirth; bothinlineissuers terminal beforewholeframefree; zero/onebelow retain",frame);
    }
    assert!(app.private_child_groups.is_empty());assert_eq!(app.private_child_group_close_demands(64).unwrap().release_bytes,backing);
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:backing,maximum_depth:1};
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(RetainedCloneGrant{maximum_release_bytes:backing-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(grant).unwrap());assert_eq!(step.progress().copied_items,1);assert_eq!(step.progress().released_bytes,backing);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,backing));assert!(app.private_child_groups_terminal_is_empty());
}
