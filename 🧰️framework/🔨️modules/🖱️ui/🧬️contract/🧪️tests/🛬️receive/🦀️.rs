//! 🧪️ Every concrete component and nonempty UI patch receives with real Source custody and paid cancellation.
use crate::*;
#[path="../../🖼️presentation/🛬️receive/🦀️.rs"]mod leaf;
fn clock()->Option<u64>{Some(1000)}
#[test]
fn native_ui_graph_preserves_every_component_patch_and_presence_owner(){
 use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
 use semio_framework_job::{OperationId,Generation,RetainedCloneGrant};
 use semio_ui_contract_values::testkit::{UiArenaLifetime,ui_value_arena_registry};
 let grant=RetainedCloneGrant{maximum_items:1000000,maximum_copy_bytes:268435456,maximum_capacity_bytes:268435456,maximum_release_bytes:268435456,maximum_depth:64};let physical=RetainedCloneGrant{maximum_items:32,..grant};let mut caller=OriginalJobFixtureCaller::admit(OperationId(9201),Generation(1),grant,clock,10000000,10000000);let mut input=Some(UiArenaLifetime::new(ui_value_arena_registry()));let mut arena=None;
 for _ in 0..100000{if arena.is_some(){break}caller.turn(physical,|cx|UiArenaLifetime::admit_source(&mut input,&mut arena,cx)).unwrap();}assert!(input.is_none()&&arena.is_some());let mut ready=false;for _ in 0..100000{if caller.turn(physical,|cx|UiArenaLifetime::advance_on_source(arena.as_mut().unwrap(),cx)).unwrap(){ready=true;break}}assert!(ready);
 let c:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in c["cases"].as_array().unwrap(){for cut in std::iter::once(None).chain(c["cancelPrefixes"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize))){match row["family"].as_str().unwrap(){"component"=>leaf::run::<Component>(row,cut,&c),"patch"=>leaf::run::<UiPatch>(row,cut,&c),"presence"=>leaf::run::<PresenceUpdate>(row,cut,&c),_=>panic!("declared full UI family")}}}
 for _ in 0..100000{if caller.turn(physical,|cx|semio_framework_job::OriginalNativeLease::return_slot_step(&mut arena,cx)).unwrap(){break}}assert!(arena.is_none());caller.cancel().cancel_now();for _ in 0..100000{if caller.turn(physical,|cx|cx.close_original_native_step()).unwrap(){break}}caller.retire(physical);
}
