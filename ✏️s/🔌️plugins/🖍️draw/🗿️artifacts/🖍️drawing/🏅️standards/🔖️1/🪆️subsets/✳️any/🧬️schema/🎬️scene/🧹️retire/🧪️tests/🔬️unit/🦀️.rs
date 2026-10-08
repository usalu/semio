use super::*;
use crate::schema::scene_preparation::{DocumentSceneJob,DocumentSceneLimits};
use serde_json::Value;
#[test]
fn painted_scenes_retirement_drains_shared_contract_owners_under_grants(){
 let sources:Vec<Value>=serde_json::from_str(include_str!("../../../📋️prepare/🧫️fixtures/🔣️.json")).unwrap();let rows:Vec<Value>=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for row in rows{for grant in [1,7,4096]{let mut plan=DocumentScenePlan::default();if let Some(name)=row["source"].as_str(){let source=sources.iter().find(|r|r["name"]==name).unwrap();let document:crate::DrawingSnapshot=serde_json::from_value(source["document"].clone()).unwrap();let v=&source["limits"];let mut job=DocumentSceneJob::new(&document,DocumentSceneLimits{max_nodes:v["maxNodes"].as_u64().unwrap()as usize,max_depth:v["maxDepth"].as_u64().unwrap()as usize,max_segments:v["maxSegments"].as_u64().unwrap()as usize,max_references:v["maxReferences"].as_u64().unwrap()as usize,max_source_bytes:v["maxSourceBytes"].as_u64().unwrap()as usize}).unwrap();while !job.advance(4096).unwrap().done{}plan=job.result().unwrap();}
  let retained=plan.assets.iter().map(|a|a.image.clone()).collect::<Vec<_>>();let expected=retained.iter().map(|s|s.pixels.clone()).collect::<Vec<_>>();let mut close=ScenePlanCloseJob::new(plan);let(mut work,mut owners)=(0,0);while !close.terminal_is_empty(){let p=close.advance(grant).unwrap();assert!(p.work>work&&p.work-work<=grant as u64);assert!(p.owners>=owners&&p.owners<=p.work);work=p.work;owners=p.owners;}
  assert_eq!(owners,row["owners"].as_u64().unwrap(),"{}",row["name"]);assert_eq!(work,row["work"].as_u64().unwrap(),"{}",row["name"]);let terminal=close.advance(1).unwrap();assert_eq!((terminal.work,terminal.owners,terminal.done),(work,owners,true));assert!(close.terminal_is_empty());assert!(close.stack.is_empty());for(i,alias)in retained.iter().enumerate(){assert_eq!(&alias.pixels,&expected[i]);assert_eq!(Arc::strong_count(alias),1);}
 }eprintln!("[DEBUG] Native scene retirement reaches its exact empty shell: {}",row["name"]);}
}
#[test]
fn painted_scenes_retirement_rejects_empty_grants_without_consuming_owners(){let mut close=ScenePlanCloseJob::new(DocumentScenePlan::default());assert!(close.advance(0).is_err());assert!(!close.terminal_is_empty());let p=close.advance(3).unwrap();assert_eq!((p.work,p.owners,p.done),(3,3,true));}
