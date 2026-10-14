pub(crate) fn test_mounted_private_child_frame<A:ArtifactApp,M:SpaceMember+MemberFactory+'static>(app:&mut VcsArtifactApp<A,M>){
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let frame=MountedPrivateChildGroup::<M>::frame_birth_bytes();assert!(frame<=262_144);
    let backing=app.private_child_groups.empty_backing_byte_demand().unwrap();assert!(backing>0&&backing<=262_144);
    for(index,row)in law["cases"].as_array().unwrap().iter().enumerate(){
        let operation=7391+index as u64;let grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:64,maximum_capacity_bytes:if row["capacity"]=="frame"{frame}else{frame-1},maximum_release_bytes:0,maximum_depth:64};
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.admit_private_child_group_frame(operation,0,1_000_000,grant).unwrap());
        assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_eq!(app.private_child_groups.get(operation).is_some(),row["admitted"].as_bool().unwrap());
        if !row["admitted"].as_bool().unwrap(){assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));continue;}
        assert_eq!(heap.requested_bytes,frame);let original=app.private_child_groups.get(operation).unwrap().as_ref()as *const _;
        let source:RetainedCloneGrant=serde_json::from_value(law["caller"]["source"].clone()).unwrap();
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(operation),semio_framework_job::Generation(law["caller"]["generation"].as_u64().unwrap()),source,||Some(1),law["caller"]["deadline"].as_u64().unwrap(),law["caller"]["fuel"].as_u64().unwrap());
        for _ in 0..100000{
            if app.private_child_groups.get(operation).is_none(){break;}
            assert_eq!(app.private_child_groups.get(operation).unwrap().as_ref()as *const _,original);
            let demand=app.private_child_group_close_demands(64).unwrap();assert!(demand.capacity_bytes<=262_144&&demand.release_bytes<=262_144&&demand.copy_bytes<=64&&demand.depth<=64);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,..source};
            let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];
            if demand.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}
            if demand.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}
            if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}
            for denied in denied{
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|app.close_private_child_group_step(cx,denied).map_err(Into::into)).unwrap());
                assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                assert_eq!(app.private_child_groups.get(operation).unwrap().as_ref()as *const _,original);
            }
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|app.close_private_child_group_step(cx,grant).map_err(Into::into)).unwrap());
            assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            if step.progress().released_bytes>0{assert_eq!(step.progress().released_bytes,frame);}
        }
        assert!(app.private_child_groups.get(operation).is_none());caller.retire(source);println!("[DEBUG] Mounted private frame={} originalStablePointer/fundedbirth; bothinlineissuers terminal beforewholeframefree; zero/onebelow retain",frame);
    }
    assert!(app.private_child_groups.is_empty());assert_eq!(app.private_child_group_close_demands(64).unwrap().release_bytes,backing);
    let source:RetainedCloneGrant=serde_json::from_value(law["caller"]["source"].clone()).unwrap();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(7390),semio_framework_job::Generation(1),source,||Some(1),1_000_000,1);
    let grant=source;
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(RetainedCloneGrant{maximum_release_bytes:backing-1,..grant},|cx|app.close_private_child_group_step(cx,cx.retained_grant()).map_err(Into::into)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|app.close_private_child_group_step(cx,grant).map_err(Into::into)).unwrap());assert_eq!(step.progress().copied_items,1);assert_eq!(step.progress().released_bytes,backing);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,backing));assert!(app.private_child_groups_terminal_is_empty());caller.retire(source);
}
