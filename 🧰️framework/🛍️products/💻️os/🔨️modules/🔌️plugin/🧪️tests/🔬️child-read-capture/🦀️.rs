//! 🧪️ Unmounted host capture law belongs inside the existing plugin builder contract module.
#[semio_framework_async_macros::async_test]
async fn child_read_capture_retains_exact_existing_lease_across_publication_and_release(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪆️child/👁️capture/🔣️.json")).expect("closed capture contract");
 let slot=contract["slot"].as_str().expect("slot");let id=contract["childId"].as_str().expect("child");
 let mut app=contract_composed_app_raw().await;
 app.register_child(slot,id,test_child_dialect().await,new_test_child(id).await.expect("actual child")).await.expect("register exact child");
 let old_label=format!("{}{}",contract["owners"][0]["label"].as_str().expect("old label"),"x".repeat(contract["retainedTailBytes"].as_u64().expect("tail")as usize));
 let TestMembers::Child(child)=&mut app.children.get_mut(&(slot.to_owned(),id.to_owned())).expect("child entry").member;
 child.dispatch(store::ArtifactCommand::Apply{mutations:vec![TestMutation::SetLabel(SetLabel{value:old_label.clone()})],description:None,transaction:None}).await.expect("publish old full owner");
 let generation=app.admit_child_content_publication().expect("old publication");
 app.publish_child_content_member(generation,slot,id).await.expect("old entry lease");
 let view=ChildContentView::clone(&app.child_content_root);
 let borrowed=view.typed_read::<TestSnapshot>(slot,id).expect("existing immutable owner");
 let pointer=&*borrowed as *const TestSnapshot;
 MAXIMUM_CHILD_CLONES.store(0,std::sync::atomic::Ordering::Release);MAXIMUM_CHILD_ENCODINGS.store(0,std::sync::atomic::Ordering::Release);
 let capture=view.capture_read::<TestSnapshot>(slot,id,&test_child_dialect().await).expect("capture actual existing entry");
 assert_eq!(capture.snapshot().expect("captured owner")as *const TestSnapshot,pointer);
 assert_eq!(capture.snapshot().expect("captured owner").label,old_label);
 let alias=capture.clone();assert_eq!(alias.snapshot().expect("same lease alias")as *const TestSnapshot,pointer);drop(alias);
 assert_eq!(MAXIMUM_CHILD_CLONES.load(std::sync::atomic::Ordering::Acquire),0);
 assert_eq!(MAXIMUM_CHILD_ENCODINGS.load(std::sync::atomic::Ordering::Acquire),0);
 for refusal in contract["refusals"].as_array().expect("refusals"){
  let mut dialect=test_child_dialect().await;
  let error=match refusal.as_str().expect("refusal"){
   "missingSlot"=>view.capture_read::<TestSnapshot>("absent",id,&dialect).err(),
   "missingChild"=>view.capture_read::<TestSnapshot>(slot,"absent",&dialect).err(),
   "wrongDialect"=>{dialect.subset="wrong".into();view.capture_read::<TestSnapshot>(slot,id,&dialect).err()},
   "wrongType"=>view.capture_read::<String>(slot,id,&dialect).err(),
   _=>panic!("closed refusal fixture"),
  }.expect("actual producer refusal");
  assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::InvalidValue);
 }
 drop(view);
 let new_label=contract["owners"][1]["label"].as_str().expect("new label");
 let TestMembers::Child(child)=&mut app.children.get_mut(&(slot.to_owned(),id.to_owned())).expect("child entry").member;
 child.dispatch(store::ArtifactCommand::Apply{mutations:vec![TestMutation::SetLabel(SetLabel{value:new_label.to_owned()})],description:None,transaction:None}).await.expect("publish new full owner");
 let generation=app.admit_child_content_publication().expect("new publication");
 app.publish_child_content_member(generation,slot,id).await.expect("new entry lease");
 assert_eq!(capture.snapshot().expect("old retained lease")as *const TestSnapshot,pointer);
 assert_eq!(capture.snapshot().expect("old retained lease").label,old_label);
 assert_eq!(app.child_content_root.typed_read::<TestSnapshot>(slot,id).expect("current child").label,new_label);
 install_test_snapshot_retirement(&mut app,id,false);
 assert_maintenance_reports_block(&mut app,4,"captured old child entry lease");
 drop(capture);
 drain_and_close_composed_fixture(&mut app);
}
