use super::*;
fn oracle(source:&impl JsonWriteSource,path:&mut Vec<usize>)->serde_json::Value{
 let(node,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.node_at_path(path));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 match node.unwrap(){JsonWriteNode::Null=>serde_json::Value::Null,JsonWriteNode::Bool(v)=>v.into(),JsonWriteNode::Number(Number::UInt(v))=>v.into(),JsonWriteNode::QuotedUnsigned(v)=>v.to_string().into(),JsonWriteNode::String(v)=>v.into(),JsonWriteNode::Array(len)=>serde_json::Value::Array((0..len).map(|index|{path.push(index);let value=oracle(source,path);path.pop();value}).collect()),JsonWriteNode::Object(len)=>{let mut object=serde_json::Map::new();for index in 0..len{let(key,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.object_key_at_path(path,index));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let key=key.unwrap().to_owned();path.push(index);let value=oracle(source,path);path.pop();assert!(object.insert(key,value).is_none());}serde_json::Value::Object(object)},_=>panic!("unexpected Event scalar")}
}
#[test]
fn borrowed_events_match_shared_serde_oracle_without_projection_allocations(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎠️kernel/📥️events/🧪️tests/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let event:Event=serde_json::from_value(row["input"].clone()).unwrap();let events=[event];let source=EventJsonSource(&events);let expected=serde_json::json!([row["input"]]);assert_eq!(serde_json::to_value(&events).unwrap(),expected);assert_eq!(oracle(&source,&mut Vec::new()),expected);}
 let variants=[ActivationEvent::OnCommand{id:"a".into()},ActivationEvent::OnViewVisible{id:"v".into()},ActivationEvent::OnFileType{ext:"json".into()},ActivationEvent::OnArtifactKind{kind:"doc".into()},ActivationEvent::OnExtensionRequest{point:"x".into()},ActivationEvent::OnStartupFinished];
 for reason in variants{let events=[Event::Activate{reason}];assert_eq!(oracle(&EventJsonSource(&events),&mut Vec::new()),serde_json::to_value(&events).unwrap());}
}

#[test]
fn dedicated_poll_pages_match_the_shared_wire_without_allocating_projections() {
    struct Page<'a>(PollPage<'a>);
    impl JsonWriteSource for Page<'_> {
        fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode<'_>, ValueError> { self.0.node(path) }
        fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError> { self.0.key(path, index) }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎠️kernel/📥️events/🧪️tests/🔣️.json")).unwrap();
    for variant in ["commandIngressPage", "coldDocumentPairPage"] {
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["variant"] == variant).unwrap();
        let event: Event = serde_json::from_value(row["input"].clone()).unwrap();
        let (page, expected) = match &event {
            Event::CommandIngressPage { cursor, bytes } => (PollPage::Command(cursor, bytes.as_slice()), serde_json::to_value((cursor, bytes)).unwrap()),
            Event::ColdDocumentPairPage(page) => (PollPage::Cold(page), serde_json::to_value(page).unwrap()),
            _ => panic!("poll fixture must contain a dedicated transport page"),
        };
        assert_eq!(oracle(&Page(page), &mut Vec::new()), expected);
    }
}
#[test]
fn decimal_scalar_sink_waits_for_whole_bounded_token_before_copy(){
 let events=[Event::InstanceOpen{purpose: semio_framework_kernel::ActorInstanceOpenPurpose::Ordinary,request:ActorInstanceOpenRequest{activation_generation:u64::MAX,instance_id:0,request_sequence:1},app_id:semio_framework_kernel_identity::AppInstanceId("app".into()),actor:"editor".into(),config:vec![],assets:vec![],capabilities:vec![BrokerCapabilityGrant{token:semio_framework_kernel_identity::CapabilityToken(u128::MAX),id:semio_framework_kernel_identity::CapabilityId("test".into()),scope:"all".into(),expires_ms:Some(u64::MAX)}],quotas:QuotaSchema::default()}];
 let source=EventJsonSource(&events);let mut cursor=semio_framework_pack_json::JsonBorrowedSinkCursor::empty(32).unwrap();let mut observer=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut observer);let mut output=Vec::new();
 for _ in 0..65536{if cursor.is_complete(){break}let mut bytes=[0;64];let d=cursor.next_demands(bytes.len(),4096).unwrap();let g=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut bytes,g,1,&mut control));let step=step.result.unwrap();assert!(step.progress.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress.retained_capacity_bytes,step.progress.released_bytes));output.extend_from_slice(&bytes[..step.written_bytes]);}
 assert!(cursor.is_complete());assert_eq!(serde_json::from_slice::<serde_json::Value>(&output).unwrap(),serde_json::to_value(&events).unwrap());
 let stock=semio_framework_value::RetainedCloneGrant{maximum_items:100_000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(108),semio_framework_job::Generation(1),stock,||Some(1),1_000_000,1_000_000);
 for _ in 0..65536 {
  if cursor.terminal_is_empty(){break}
  let demand=cursor.close_demands(4096).unwrap();
  let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  caller.refreshed_turn(grant,|cx|cx.with_original_retirement(|cx|{
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::close_original_effect_step(cx,demand,|effect|cursor.close_step(effect)));
   let progress=step?.progress();
   assert!(progress.fits(grant));
   assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
   Ok(())
  })).unwrap();
 }
 assert!(cursor.terminal_is_empty());
 caller.retire(stock);
}

#[test]
fn borrowed_poll_limits_are_request_data_without_authenticated_scalar_identity(){
 let events=[Event::Wake];let budget=semio_framework_kernel_identity::Budget::original(semio_framework_value::RetainedCloneGrant{maximum_items:9,maximum_copy_bytes:17,maximum_capacity_bytes:31,maximum_release_bytes:63,maximum_depth:7},100,1,1,2,3,4);
 let request=PollRequestLimits{retained_policy:budget.retained_policy,work_fuel:budget.work_fuel,fuel:budget.fuel,deadline_us:budget.deadline_us,max_effects:budget.max_effects,max_patch_bytes:budget.max_patch_bytes,max_frames:budget.max_frames};
 let limits=OwnedCorePollLimits::try_from(request).unwrap();
 let source=PollJsonSource{events:&events,limits:&limits};
 let expected=serde_json::json!([9,17,31,63,7,100,100,1001,2,3,4]);
 assert_eq!(serde_json::to_value(limits).unwrap(),expected);
 assert_eq!(oracle(&source,&mut Vec::new()),serde_json::json!({"events":["wake"],"command_page":null,"cold_pair_page":null,"limits":expected}));
 assert!(source.object_key_at_path(&[3],0).is_err());
 assert!(source.node_at_path(&[3,OwnedCorePollLimits::ARITY]).is_err());
 let invalid=[Event::InstanceClose(ActorInstanceCloseRequest{lifetime:ActorInstanceLifetime{activation_generation:0,instance_id:0,guest_lifetime:1},request_sequence:1})];assert!(EventJsonSource(&invalid).node_at_path(&[0,0]).is_err());
}
