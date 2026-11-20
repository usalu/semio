//! 🧪️ Every concrete component and nonempty UI patch receives with real Source custody and paid cancellation.
use crate::*;
#[path="../../🖼️presentation/🛬️receive/🦀️.rs"]mod leaf;
#[test]
fn native_ui_graph_preserves_every_component_patch_and_presence_owner(){
 let _arena=semio_ui_contract_values::testkit::UiArenaFixture::new();
 let c:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in c["cases"].as_array().unwrap(){for cut in std::iter::once(None).chain(c["cancelPrefixes"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize))){match row["family"].as_str().unwrap(){"component"=>leaf::run::<Component>(row,cut,&c),"patch"=>leaf::run::<UiPatch>(row,cut,&c),"presence"=>leaf::run::<PresenceUpdate>(row,cut,&c),_=>panic!("declared full UI family")}}}
}
