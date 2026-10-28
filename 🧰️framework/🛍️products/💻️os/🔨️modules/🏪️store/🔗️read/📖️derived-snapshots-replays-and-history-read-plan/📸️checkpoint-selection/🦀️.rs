//! 📸️ Exact checkpoint selection follows authored change order independently of the current head.
use super::{ArtifactCheckpointReadSource,ArtifactCheckpointSelection};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use serde_json::Value;
struct Source(Value);
impl ArtifactCheckpointReadSource for Source{
 fn checkpoint_id(&self,p:usize)->Option<&str>{self.0["checkpoints"].get(p)?["id"].as_str()}
 fn checkpoint_change_id(&self,p:usize,c:usize)->Option<&str>{self.0["checkpoints"].get(p)?["changeIds"].get(c)?.as_str()}
 fn change_id(&self,p:usize)->Option<&str>{self.0["changes"].get(p)?["id"].as_str()}
 fn change_edit_id(&self,p:usize,e:usize)->Option<&str>{self.0["changes"].get(p)?["editIds"].get(e)?.as_str()}
 fn edit_id(&self,p:usize)->Option<&str>{self.0["edits"].get(p)?.as_str()}
}
fn funded(selection:&ArtifactCheckpointSelection)->RetainedCloneGrant{let d=selection.next_demand();RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:d.depth}}
#[test]
fn checkpoint_selection_matches_json_oracle_and_preserves_denied_and_foreign_sources(){
 let source=Source(serde_json::from_str(include_str!("🔣️.json")).unwrap());let id=source.0["selection"].as_str().unwrap();let mut selection=ArtifactCheckpointSelection::new(id);let mut actual=Vec::new();
 for _ in 0..100{
  if selection.complete{break}let grant=funded(&selection);
  for axis in 0..3{let before=selection;let mut denied=grant;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes-=1,_=>denied.maximum_depth=0}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||selection.step(&source,id,denied).unwrap());assert_eq!(result,RetainedCloneProgress::default());assert_eq!(selection,before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let foreign=id.to_owned();let before=selection;assert!(selection.step(&source,&foreign,grant).is_err());assert_eq!(selection,before);
  let(progress,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||selection.step(&source,id,grant).unwrap());assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  if let Some(position)=selection.current_edit{actual.push(source.edit_id(position).unwrap());selection.advance_edit();}
 }
 assert!(selection.complete);let checkpoint=source.0["checkpoints"].as_array().unwrap().iter().find(|row|row["id"]==id).unwrap();let oracle:Vec<&str>=checkpoint["changeIds"].as_array().unwrap().iter().flat_map(|change|source.0["changes"].as_array().unwrap().iter().find(|row|row["id"]==*change).unwrap()["editIds"].as_array().unwrap().iter().map(|edit|edit.as_str().unwrap())).collect();assert_eq!(actual,oracle);assert_eq!(serde_json::to_value(actual).unwrap(),source.0["expected"]);
}
#[test]
fn checkpoint_selection_refuses_absent_checkpoint_change_and_edit(){
 for missing in 0..3{let mut source=Source(serde_json::from_str(include_str!("🔣️.json")).unwrap());match missing{0=>source.0["checkpoints"][1]["id"]=Value::from("absent"),1=>source.0["changes"][2]["id"]=Value::from("absent"),_=>source.0["edits"][2]=Value::from("absent")};let id="repinned";let mut selection=ArtifactCheckpointSelection::new(id);let mut refused=false;for _ in 0..100{let grant=funded(&selection);if selection.step(&source,id,grant).is_err(){refused=true;break}assert!(!selection.complete);if selection.current_edit.is_some(){selection.advance_edit()}}assert!(refused);}
}
