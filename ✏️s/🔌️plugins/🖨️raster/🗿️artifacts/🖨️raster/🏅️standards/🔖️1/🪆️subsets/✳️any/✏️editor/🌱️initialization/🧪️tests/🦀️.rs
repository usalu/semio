//! 🧪️ Raster document genesis retains one original caller through native Pack creation and cleanup.
use super::*;
use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;
#[test]
fn original_document_genesis_matches_authored_empty_shell_and_closes(){
 let fixture=semio_framework_pack_json::parse(include_str!("genesis.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut caller=OriginalWorkFixture::new();let grant=OriginalWorkFixture::stock();let actor=store::ActorId("raster-genesis-law".into());let mut owner=None;
 for _ in 0..1024{if owner.is_some(){break}owner=caller.turn(grant,|cx|semio_framework_plugin_ownership::original_store_genesis::OriginalStoreGenesis::<RasterDocumentGenesis>::admit("raster-original-law",&actor,cx)).unwrap();}
 let mut owner=owner.expect("original genesis controller admitted");let mut result=None;for _ in 0..100000{if result.is_some(){break}result=caller.turn(grant,|cx|owner.step(cx)).unwrap();}
 let(name,mut document)=result.expect("authored document genesis reached native Store");assert_eq!(name,"raster-original-law");let expected:State=semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_string(fixture.get("snapshot").unwrap()),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(document.snapshot().unwrap().get(),&expected);
 for _ in 0..100000{if document.close_owned_store_terminal_is_empty(){break}caller.turn(grant,|cx|{let d=document.close_owned_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|same|{let result=document.close_owned_store_step(same);semio_framework_value::RetainedWorkOutcome{result,work:1}})?;Ok(())}).unwrap();}assert!(document.close_owned_store_terminal_is_empty());drop(document);
 for _ in 0..100000{if owner.terminal_is_empty(){break}caller.turn(grant,|cx|owner.close_step(cx)).unwrap();}assert!(owner.terminal_is_empty());drop(owner);caller.retire_owned(expected,grant);caller.finish();
}
