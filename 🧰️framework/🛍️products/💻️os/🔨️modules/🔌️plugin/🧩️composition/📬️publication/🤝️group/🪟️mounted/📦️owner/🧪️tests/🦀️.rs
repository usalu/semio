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
        for _ in 0..100000{
            if app.private_child_groups.get(operation).is_none(){break;}
            assert_eq!(app.private_child_groups.get(operation).unwrap().as_ref()as *const _,original);
            let demand=app.private_child_group_close_byte_demand();assert!(demand<=262_144);
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(0,demand).unwrap());assert_eq!(step,PluginCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            if demand>0{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(1,demand-1).unwrap());assert_eq!(step,PluginCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(1,demand).unwrap());let(released_items,released_bytes)=match step{PluginCloseStep::Pending{released_items,released_bytes}=>(released_items,released_bytes),_=>panic!("one funded original mounted frame step remains pending until its registry backing closes")};assert!(released_items<=1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,released_bytes));if released_bytes>0{assert_eq!(released_bytes,frame);}
        }
        assert!(app.private_child_groups.get(operation).is_none());println!("[DEBUG] Mounted private frame={} originalStablePointer/fundedbirth; bothinlineissuers terminal beforewholeframefree; zero/onebelow retain",frame);
    }
    assert!(app.private_child_groups.is_empty());assert_eq!(app.private_child_group_close_byte_demand(),backing);
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(1,backing-1).unwrap());assert_eq!(step,PluginCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app.close_private_child_group_step(1,backing).unwrap());assert_eq!(step,PluginCloseStep::Pending{released_items:1,released_bytes:backing});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,backing));assert!(app.private_child_groups_terminal_is_empty());
}
