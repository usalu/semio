use super::*;

#[test]
fn restored_actor_receives_original_decoder_and_grant_before_arc_birth(){
    use semio_framework_value::{NativeDecodeControl,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,retirement::controlled::ControlledRetirement};
    use semio_framework_os_kernel::io::control::NativeSnapshotDecodeOwner;
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    use std::cell::Cell;
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫴️actor.json")).unwrap();
    let policies:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧫️fixtures/🔣️.json")).unwrap();
    let grant_for=|id:&str|serde_json::from_value::<RetainedCloneGrant>(policies["cases"].as_array().unwrap().iter().find(|row|row["id"]==id).unwrap()["grant"].clone()).unwrap();
    let close_grant=grant_for("full");let maximum=policies["nativeMaximumBytes"].as_u64().unwrap()as usize;let text=law["source"].as_str().unwrap();
    let frame=law["frameLayouts"].as_array().unwrap().iter().find(|row|row["nativeWordBytes"].as_u64().unwrap()as usize==std::mem::size_of::<usize>()).unwrap()["arcBytes"].as_u64().unwrap()as usize;
    let close_text=|text:String,native:&mut NativeDecodeControl<'_>|{let mut cursor=ControlledRetirement::new(text).unwrap_or_else(|_|unreachable!());while !cursor.terminal_is_empty(){native.checkpoint().unwrap();native.charge(cursor.next_capacity_byte_demand(close_grant.maximum_copy_bytes).unwrap()).unwrap();let(step,heap)=observe(||cursor.step(close_grant).unwrap());assert!(step.progress().fits(close_grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}};
    let callbacks=Cell::new(0usize);
    for accepted in [false,true]{for id in law[if accepted{"acceptedGrants"}else{"refusedGrants"}].as_array().unwrap(){
        let grant=grant_for(id.as_str().unwrap());let calls=Cell::new(0usize);let mut callback=|_|{calls.set(calls.get()+1);true};let mut native=NativeDecodeControl::new(maximum,&mut callback);let mut source=Some(native.copy_text(text).unwrap());let pointer=source.as_ref().unwrap().as_ptr();let capacity=source.as_ref().unwrap().capacity();let prior=native.owned_bytes();let native_pointer=&native as*const _;calls.set(0);
        let mut original=NativeSnapshotDecodeOwner::new(&mut native,grant);assert_eq!(original.native()as*const _,native_pointer);let(result,heap)=observe(||super::admit_restored_actor(&mut source,&mut original));let progress=original.progress();assert_eq!(original.grant(),grant);assert_eq!(original.native()as*const _,native_pointer);assert_eq!(serde_json::to_value(original.grant()).unwrap(),serde_json::to_value(grant).unwrap());drop(original);
        if accepted{let actor=result.unwrap();assert!(source.is_none());assert_eq!(actor.0.as_ptr(),pointer);assert_eq!(serde_json::to_value(&actor).unwrap(),law["source"]);assert_eq!((heap.requested_bytes,heap.released_bytes),(frame,0));assert_eq!(progress,RetainedCloneProgress{copied_items:1,copied_bytes:law["semanticMetadataCopyBytes"].as_u64().unwrap()as usize,retained_capacity_bytes:frame,released_bytes:0});assert!(progress.fits(grant));assert_eq!(native.owned_bytes(),prior+frame);if id=="full"{callbacks.set(calls.get());}let(closed,heap)=observe(||actor.0.close_original_lease(close_grant));let(text,receipt)=closed.ok().unwrap();let text=text.unwrap();assert_eq!(text.as_ptr(),pointer);assert_eq!(text.capacity(),capacity);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,receipt.released_bytes));assert_eq!(receipt.released_bytes,frame);close_text(text,&mut native);}
        else{assert!(matches!(result.unwrap_err().kind,ValueRefusalKind::WorkLimit|ValueRefusalKind::OwnershipLimit|ValueRefusalKind::DepthLimit));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(progress,RetainedCloneProgress::default());assert_eq!(native.owned_bytes(),prior);assert_eq!(source.as_ref().unwrap().as_ptr(),pointer);assert_eq!(source.as_ref().unwrap().capacity(),capacity);close_text(source.take().unwrap(),&mut native);}
    }}
    assert!(callbacks.get()>0);
    for boundary in 0..callbacks.get(){let calls=Cell::new(0usize);let cancel=Cell::new(None);let mut callback=|_|{let index=calls.get();calls.set(index+1);cancel.get()!=Some(index)};let mut native=NativeDecodeControl::new(maximum,&mut callback);let mut source=Some(native.copy_text(text).unwrap());let pointer=source.as_ref().unwrap().as_ptr();let capacity=source.as_ref().unwrap().capacity();let prior=native.owned_bytes();calls.set(0);cancel.set(Some(boundary));let mut original=NativeSnapshotDecodeOwner::new(&mut native,close_grant);let(result,heap)=observe(||super::admit_restored_actor(&mut source,&mut original));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(original.progress(),RetainedCloneProgress::default());drop(original);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(native.owned_bytes(),prior);assert_eq!(source.as_ref().unwrap().as_ptr(),pointer);assert_eq!(source.as_ref().unwrap().capacity(),capacity);cancel.set(None);close_text(source.take().unwrap(),&mut native);}
    println!("[DEBUG] restored actor originalDecoder=true independentOriginalGrant=true actualArcBirth=true originalUtf8Pointer=true allPrebirthCancellationRefusalsPreserveSource=true independentSerde=true");
}

#[semio_framework_async_macros::async_test]
async fn checkpoint_of_no_instances_round_trips_through_json() {
    let runtime = plugin_runtime::PluginRuntime::<crate::app::NoPluginApp>::new({ let grant = crate::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; crate::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    let bytes = checkpoint(&runtime, &[], vec![1, 2], vec![7], Vec::new()).await.expect("an empty instance list must still encode");
    let pack: CheckpointPack = serde_json::from_slice(&bytes).expect("checkpoint bytes must be valid CheckpointPack json");
    assert!(pack.instances.is_empty());
    assert_eq!(pack.timers, vec![1, 2]);
    assert_eq!(pack.pending_requests, vec![7]);
    assert!(pack.task_restarts.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn task_restarts_round_trip_through_json_and_are_exposed_by_the_accessor() {
    let runtime = plugin_runtime::PluginRuntime::<crate::app::NoPluginApp>::new({ let grant = crate::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; crate::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    let restarts = vec![TaskRestart { instance: 5, command: vec![1, 2, 3] }, TaskRestart { instance: 6, command: vec![4] }];
    let bytes = checkpoint(&runtime, &[], Vec::new(), Vec::new(), restarts.clone()).await.expect("must encode");
    let pack = restore(&runtime, &bytes).await.expect("must decode back").pack;
    assert_eq!(pack.task_restarts().await.len(), 2);
    assert_eq!(pack.task_restarts().await[0].instance, 5);
    assert_eq!(pack.task_restarts().await[0].command, vec![1, 2, 3]);
    assert_eq!(pack.task_restarts().await[1].instance, 6);
}

#[semio_framework_async_macros::async_test]
async fn checkpoint_requires_its_complete_declared_authority() {
    let incomplete = r#"{"instances":[],"timers":[],"pending_requests":[]}"#;
    assert!(serde_json::from_str::<CheckpointPack>(incomplete).is_err());
    assert!(semio_framework_pack_json::from_json_str::<CheckpointPack>(incomplete, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}

#[test]
fn checkpoint_actor_contract_matches_independent_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral checkpoint fixture");
    for case in fixture["cases"].as_array().expect("checkpoint cases") {
        let source = serde_json::to_string(&case["pack"]).expect("neutral checkpoint JSON");
        let independent = serde_json::from_str::<CheckpointPack>(&source);
        let actual = semio_framework_pack_json::from_json_str::<CheckpointPack>(&source, semio_framework_pack_json::JsonMemberPolicy::Reject);
        let expected = case["expected"].as_bool().expect("checkpoint verdict");
        assert_eq!(independent.is_ok(), expected, "independent: {}", case["id"]);
        assert_eq!(actual.is_ok(), expected, "first-party: {}", case["id"]);
        if let Ok(pack) = actual {
            let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&pack)).expect("first-party checkpoint JSON");
            assert_eq!(encoded, case["pack"], "actor and document authority survive: {}", case["id"]);
        }
    }
}

#[test]
fn checkpoint_preserves_explicit_instance_actor_in_authored_json() {
    let source = include_str!("📸️actor.json");
    let pack: CheckpointPack = semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored checkpoint must decode");
    assert_eq!(pack.instances[0].actor, "checkpoint-owner");
    let expected: serde_json::Value = serde_json::from_str(source).expect("independent checkpoint oracle must decode");
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&pack)).expect("first-party checkpoint must encode JSON");
    assert_eq!(actual, expected);
}
