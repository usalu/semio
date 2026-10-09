//! 🧪️ Retained reducers bind the exact captured window-config owner and version.

use super::*;

#[semio_framework_async_macros::async_test]
async fn original_window_constructor_cancellation_preserves_carriers_and_full_physical_receipts(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
    type Owner=super::pack_identity_tests::IdentityWindowOwner;
    type Mutation=super::retained_pack_load_tests::RetainedLoadCameraConfigMutation;
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🪟️window-publication/🔣️.json")).unwrap();
    let original=law["originalGrant"].as_array().unwrap();
    let grant=RetainedCloneGrant{maximum_items:original[0].as_u64().unwrap()as usize,maximum_copy_bytes:original[1].as_u64().unwrap()as usize,maximum_capacity_bytes:original[2].as_u64().unwrap()as usize,maximum_release_bytes:original[3].as_u64().unwrap()as usize,maximum_depth:original[4].as_u64().unwrap()as usize};
    for capacity in [1usize,257,4096]{
        let mut owner=TypedWindowConfigStoreOwner::<Owner>{direct_ingress:ControlledRetirement::new(DirectIngress{mutations:Vec::new(),allocations:Vec::new()}).map_err(|(error,_)|error).unwrap(),partitions:WindowRegistry::new(),partition_close_cursor:0,partition_address_retirement:None,actor:Some(protocol::ActorId(semio_framework_value::SharedUtf8::admit("window-actor",grant).unwrap().0)),actor_retirement:None};
        let authority=owner.capture("original-window").await.unwrap();
        let mut inputs=Vec::with_capacity(1);
        let((mutation,mut actor),source_heap)=observe_heap_allocations_on_this_thread(||{
            let mut address=String::with_capacity(capacity.max("original-window".len()));address.push_str("original-window");
            let mutation=WindowConfigMutation::of::<Owner>(address,Mutation::Snapshot{config:Box::new(Default::default())});
            let(actor,_)=semio_framework_value::SharedUtf8::admit("window-actor",grant).unwrap();
            (mutation,Some(ControlledRetirement::new(actor).map_err(|(error,_)|error).unwrap()))
        });
        assert_eq!(source_heap.released_bytes,0);inputs.push(mutation);
        let input_pointer=inputs[0].window_id.as_ptr();
        let payload_pointer=inputs[0].mutation.as_any().downcast_ref::<Mutation>().unwrap()as *const Mutation;
        let actor_pointer=actor.as_ref().unwrap().original().unwrap().as_ptr();
        let frame=std::mem::size_of::<TypedWindowConfigPublication<Owner>>();
        let copy=frame+std::mem::size_of::<WindowConfigMutation>()+std::mem::size_of::<FactoryBoxedValue<Mutation>>()+std::mem::size_of::<Mutation>();
        let birth=frame+std::mem::size_of::<Mutation>();
        assert!(copy<=grant.maximum_copy_bytes&&birth<=grant.maximum_capacity_bytes);
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:copy-1,..grant},RetainedCloneGrant{maximum_capacity_bytes:birth-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
            let(result,heap)=observe_heap_allocations_on_this_thread(||owner.admit_begin(semio_framework_job::OperationId(17),&mut actor,&mut inputs,&authority,denied).unwrap());
            assert!(result.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert_eq!(inputs[0].window_id.as_ptr(),input_pointer);assert_eq!(inputs[0].mutation.as_any().downcast_ref::<Mutation>().unwrap()as *const Mutation,payload_pointer);assert_eq!(actor.as_ref().unwrap().original().unwrap().as_ptr(),actor_pointer);
        }
        let(admitted,heap)=observe_heap_allocations_on_this_thread(||owner.admit_begin(semio_framework_job::OperationId(17),&mut actor,&mut inputs,&authority,grant).unwrap().unwrap());
        let(mut publication,receipt)=admitted;
        assert_eq!(receipt,RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:birth,released_bytes:0});assert_eq!((heap.requested_bytes,heap.released_bytes),(birth,0));assert!(inputs.is_empty()&&actor.is_none());
        let typed=publication.as_any_mut().downcast_mut::<TypedWindowConfigPublication<Owner>>().unwrap();
        assert_eq!(typed.window_id.original().unwrap().as_ptr(),input_pointer);assert_eq!(typed.actor.as_ref().unwrap().original().unwrap().as_ptr(),actor_pointer);assert!(typed.publication.is_none());assert_eq!(typed.pending.len(),1);
        publication.begin_close();let mut births=birth;let mut releases=0usize;let mut turns=0usize;
        for _ in 0..4096{
            if publication.terminal_is_empty(){break;}
            let(demand,heap)=observe_heap_allocations_on_this_thread(||publication.retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            let mut denials=vec![RetainedCloneGrant{maximum_items:0,..grant}];
            if demand.copy_bytes!=0{denials.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes!=0{denials.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes!=0{denials.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}if demand.depth!=0{denials.push(RetainedCloneGrant{maximum_depth:demand.depth-1,..grant});}
            for denied in denials{let(step,heap)=observe_heap_allocations_on_this_thread(||publication.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!publication.terminal_is_empty());}
            let(step,heap)=observe_heap_allocations_on_this_thread(||publication.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(matches!(step,RetainedCloneStep::Progress(_)));births+=receipt.retained_capacity_bytes;releases+=receipt.released_bytes;turns+=1;
        }
        assert!(publication.terminal_is_empty());
        let released_frame=std::mem::size_of_val(&*publication);assert_eq!(released_frame,frame);assert!(released_frame<=grant.maximum_release_bytes);
        let(_,heap)=observe_heap_allocations_on_this_thread(||drop(publication));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,released_frame));releases+=released_frame;
        assert_eq!(releases,source_heap.requested_bytes+births);
        eprintln!("[DEBUG] original window constructor/cancel capacity={capacity} turns={turns} original={} births={births} release={releases} denied0heap originalAddress={input_pointer:p} originalPayload={payload_pointer:p} terminalBodyDrop0 separateFrame={released_frame}; Store semantic publication excluded",source_heap.requested_bytes);
        drop(authority);
        for _ in 0..65536{if matches!(owner.close_step(grant).unwrap(),PluginLifecycleStep::Complete(_))&&owner.terminal_is_empty(){break;}}
        assert!(owner.terminal_is_empty());
    }
}

#[test]
fn retained_window_config_context_identity_binds_owner_generation_and_revision() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️retained-window-config/🔣️.json")).unwrap();
    let mut digests = std::collections::BTreeSet::new();
    for row in fixture["cases"].as_array().unwrap() {
        let generation = row["generation"].as_u64().unwrap();
        let revision = [row["revisionByte"].as_u64().unwrap() as u8; 32];
        let snapshot = WindowConfigSnapshot { window_id: row["windowId"].as_str().unwrap().into(), window_kind_id: "graph", generation, revision, snapshot: Arc::new(crate::app::NoConfig {}) };
        assert_eq!(snapshot.generation(), generation);
        assert_eq!(snapshot.revision(), revision);
        let digest = crate::app::test_window_config_context_identity(Some(&snapshot));
        assert_eq!(digest, crate::app::test_window_config_context_identity(Some(&snapshot.clone())));
        assert_ne!(digest, crate::app::test_window_config_context_identity(None));
        digests.insert(digest);
    }
    assert_eq!(digests.len(), fixture["expectedUniqueContexts"].as_u64().unwrap() as usize);
}
