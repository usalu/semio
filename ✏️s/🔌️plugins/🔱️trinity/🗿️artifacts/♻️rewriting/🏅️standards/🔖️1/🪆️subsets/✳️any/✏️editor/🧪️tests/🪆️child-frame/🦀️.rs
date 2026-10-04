//! 🧪️ Full Semio child laws exercise the actual Rewriting editor and registry owner.
use super::*;
use semio_framework_plugin::PluginCloseStep;
use store::os_io::{ArtifactDialect,ArtifactRef};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::{snapshot::SemioGraphSnapshot,mutations::SemioGraphMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::{decode_semio_graph_snapshot_json,encode_semio_graph_snapshot_json};
type RichFrameMembers=semio_s_artifact_stdio_semio::SemioMembers;
type RichFrameApp=VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>,RichFrameMembers>;
fn full_frame_contract()->serde_json::Value{
 serde_json::from_str(include_str!("../../../🧬️schema/📸️snapshot/🧫️fixtures/🪆️child/🧵️lifetime/🔣️.json")).expect("closed frame input")
}
fn full_frame_child(contract:&serde_json::Value,next:bool)->SemioGraphSnapshot{
 let mut vector:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🌳️typed/🔣️.json")).expect("rich owner input");
 let child=&mut vector["childSnapshot"];
 child["nodes"][0]["properties"].as_array_mut().expect("actual ordered properties").extend(contract["intrinsicValues"].as_array().expect("nine intrinsic inputs").iter().cloned());
 if next{child["nodes"][0]["label"]=serde_json::Value::String(format!("{}{}",contract["nextLiteral"].as_str().expect("next literal"),"x".repeat(contract["retainedTailBytes"].as_u64().expect("tail")as usize)));}
 decode_semio_graph_snapshot_json(&child.to_string()).expect("complete declared Semio owner")
}
async fn full_frame_app()->RichFrameApp{
 let contract=full_frame_contract();
 let mut app=artifact_app_laws::new_app_with_registry_and_members::<EditorApp<TrinityRewritingPlayApp>,RichFrameMembers>(trinity_rewriting_manifest_for_tests).await;
 app.bind_instance_id(REWRITING_TEST_INSTANCE).await;
 let mut parent=app.snapshot().expect("actual parent");
 let logical=contract["childId"].as_str().expect("declared member key").to_owned();
 let target=contract["target"]["artifactId"].as_str().expect("target").to_owned();
 let dialect=ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"graph".into()};
 parent.working_graph.content=store::ArtifactChild::new(logical.clone(),ArtifactRef{artifact_id:target.clone(),dialect:dialect.clone()});
 app.test_parent_store_mut().dispatch(store::ArtifactCommand::Apply{mutations:vec![schema::mutations::edit_before_fixture(parent.working_graph)],description:None,transaction:None}).await.expect("declare actual owner identity");
 let mut envelope=store::create_document_envelope::<SemioGraphSnapshot,SemioGraphMutation>("stdio.semio",&target,full_frame_child(&contract,false),None);
 envelope.dialect=Some(dialect.clone());
 let mut child=store::ArtifactStore::new(envelope).await.expect("actual rich member store");
 child.install_document_store_owners_exact(<SemioGraphSnapshot as store::MemberStoreOwner<SemioGraphMutation>>::member_store_owners());
 app.register_child("workingGraph",logical,dialect,RichFrameMembers::Graph(Box::new(child))).await.expect("publish complete member");
 app
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_rewriting_full_child_existing_registry_lifetime(){
 let contract=full_frame_contract();let mut app=full_frame_app().await;
 let view=app.test_child_content_view();
 let id=contract["childId"].as_str().expect("declared member key");
 let old=view.typed_read::<SemioGraphSnapshot>("workingGraph",id).expect("exact existing child read");
 let pointer=&*old as *const SemioGraphSnapshot;
 let expected=encode_semio_graph_snapshot_json(&old).expect("all words and fields");
 let key=("workingGraph".to_owned(),contract["childId"].as_str().expect("declared member key").to_owned());
 let RichFrameMembers::Graph(member)=app.test_child_member_mut(&key).expect("real member") else{panic!("workingGraph member must be the actual Semio Graph owner")};
 member.dispatch(store::ArtifactCommand::Apply{mutations:vec![SemioGraphMutation::SetSnapshot(semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::set_snapshot::SetSnapshot{snapshot:full_frame_child(&contract,true)})],description:None,transaction:None}).await.expect("actual next child publication");
 app.test_publish_child_content(&key.0,&key.1).await.expect("real next root");
 assert_eq!(&*old as *const SemioGraphSnapshot,pointer);
 assert_eq!(encode_semio_graph_snapshot_json(&old).expect("old full owner"),expected);
 {
  let current=app.test_child_content_view();
  let read=current.typed_read::<SemioGraphSnapshot>("workingGraph",id).expect("current publication");
  assert_ne!(&*read as *const SemioGraphSnapshot,pointer);
  assert_eq!(read.nodes[0].label,format!("{}{}",contract["nextLiteral"].as_str().expect("next literal"),"x".repeat(65536)));
  assert_eq!((read.nodes[0].position.x.to_bits(),read.nodes[0].position.y.to_bits()),(old.nodes[0].position.x.to_bits(),old.nodes[0].position.y.to_bits()));
  assert_eq!(read.nodes[0].width.to_bits(),old.nodes[0].width.to_bits());
  assert_eq!(read.nodes[0].height.to_bits(),old.nodes[0].height.to_bits());
  assert_eq!(read.nodes[0].properties,old.nodes[0].properties);
  assert_eq!(read.nodes[0].ports,old.nodes[0].ports);
  assert_eq!(read.edges,old.edges);
  assert_eq!(encode_semio_graph_snapshot_json(&read).expect("complete current typed owner"),encode_semio_graph_snapshot_json(&full_frame_child(&contract,true)).expect("complete declared next owner"));
 }
 assert!(matches!(PluginApp::close_step(&mut app,0,0).expect("zero close grant"),PluginCloseStep::Pending{released_items:0,released_bytes:0}));
 assert_eq!(encode_semio_graph_snapshot_json(&old).expect("zero grant retains child"),expected);
 let mut blocked=false;
 for _ in 0..65536{if matches!(PluginApp::close_step(&mut app,1,65536).expect("bounded close"),PluginCloseStep::Blocked{..}){blocked=true;break}}
 assert!(blocked,"captured full existing registry owner blocks actual close");
 drop(old);drop(view);artifact_app_laws::close_registered_fixture_app(&mut app);
 assert!(PluginApp::close_terminal_is_empty(&app));
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_rewriting_real_command_reads_exact_declared_full_child(){
 let mut app=full_frame_app().await;
 app.dispatch_typed(TrinityRewritingCommand::PatchNodes{node_ids:vec!["a".into()],field:"name".into(),value:"edited !@/\0引用😀".into()},&meta("local")).await.expect("admit actual document command");
 artifact_app_laws::settle_registered_typed_operation(&mut app,REWRITING_TEST_INSTANCE).await.expect("actual reducer obtains the declared full child without local Any materialization");
 let view=app.test_child_content_view();
 let id=full_frame_contract()["childId"].as_str().expect("declared member key").to_owned();
 let child=view.typed_read::<SemioGraphSnapshot>("workingGraph",&id).expect("actual published child edit");
 assert_eq!(child.nodes[0].label,"edited !@/\0引用😀");
 let mut expected=full_frame_child(&full_frame_contract(),false);
 expected.nodes[0].label="edited !@/\0引用😀".into();
 assert_eq!((child.nodes[0].position.x.to_bits(),child.nodes[0].position.y.to_bits()),(expected.nodes[0].position.x.to_bits(),expected.nodes[0].position.y.to_bits()));
 assert_eq!(child.nodes[0].width.to_bits(),expected.nodes[0].width.to_bits());
 assert_eq!(child.nodes[0].height.to_bits(),expected.nodes[0].height.to_bits());
 assert_eq!(child.nodes[0].properties,expected.nodes[0].properties);
 assert_eq!(child.nodes[0].ports,expected.nodes[0].ports);
 assert_eq!(child.edges,expected.edges);
 assert_eq!(encode_semio_graph_snapshot_json(&child).expect("complete retained command owner"),encode_semio_graph_snapshot_json(&expected).expect("complete declared command owner"));
 drop(child);drop(view);artifact_app_laws::close_registered_fixture_app(&mut app);
}
