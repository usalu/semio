use super::*;
use crate::schema::scene_identity::parse_scene_identity_u64;
use serde_json::Value;
fn decode(value:&Value)->SceneIdentity{SceneIdentity{instance:u32::try_from(value["instance"].as_u64().unwrap()).unwrap(),base:parse_scene_identity_u64(value["base"].as_str().unwrap()).unwrap(),generation:parse_scene_identity_u64(value["generation"].as_str().unwrap()).unwrap(),revision:value["revision"].as_array().unwrap().iter().map(|byte|u8::try_from(byte.as_u64().unwrap()).unwrap()).collect::<Vec<_>>().try_into().unwrap()}}
fn observed(base:&Value,patch:&Value)->Option<SceneIdentity>{if patch.is_null(){return None;}let mut value=base.clone();for(key,field)in patch.as_object().unwrap(){value[key]=field.clone();}Some(decode(&value))}
#[test]
fn mounted_vector_query_admission_requires_the_exact_complete_live_source(){
 let fixture:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let captured=decode(&fixture["identity"]);
 for row in fixture["cases"].as_array().unwrap(){let live=observed(&fixture["identity"],&row["live"]);let visual=observed(&fixture["identity"],&row["visual"]);let preparing=row["preparing"].as_bool().unwrap();let failed=row["failed"].as_bool().unwrap();let status=scene_admission(captured,live,visual,preparing,failed);let name=match status{SceneAdmissionStatus::Ready=>"ready",SceneAdmissionStatus::Pending=>"pending",SceneAdmissionStatus::Stale=>"stale",SceneAdmissionStatus::Failed=>"failed",SceneAdmissionStatus::Unavailable=>"unavailable"};assert_eq!(name,row["status"].as_str().unwrap(),"{}",row["name"]);println!("[DEBUG] Mounted query admission {} status={name} base={} generation={}",row["name"],captured.base,captured.generation);}
 assert_eq!(scene_admission(SceneIdentity{instance:0,..captured},None,None,false,false),SceneAdmissionStatus::Unavailable);
}
