//! 🧪️ Original-bound targeting has paid metadata, exact paged identities and all-axis denial without backing work.
use super::*;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value::retained_clone::RetainedCloneSource;
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn source<T:semio_framework_value::retirement::RetireOwned+Sync>(value:T)->RetainedCloneSource<T>{RetainedCloneSource::admit_owned(value,(),grant()).unwrap_or_else(|_|panic!("original Drawing target law source" )).0}
#[test]
fn drawing_original_lookup_preserves_both_sources_and_refuses_every_zero_axis(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️.json")).unwrap();
 for cut in [0,1,4,10000]{let snapshot:DrawingSnapshot=serde_json::from_value(corpus["base"].clone()).unwrap();let target=semio_s_artifact_draw_drawing_model::layer_id(snapshot.layers.get(0).unwrap()).clone();let mut snapshot=source(snapshot);let mut target=source(target);let mut lookup=DrawingOriginalLookup::default();let g=grant();let mut ready=false;
  for _ in 0..cut{let d=lookup.demand(snapshot.borrow(),target.borrow(),g.maximum_copy_bytes).unwrap();for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=0,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=0,3 if d.release_bytes>0=>denied.maximum_release_bytes=0,4 if d.depth>0=>denied.maximum_depth=0,_=>continue}let(step,b,r)=semio_framework_trace_memory_testing::observe_backing(||lookup.advance(snapshot.borrow(),target.borrow(),g.maximum_copy_bytes,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((b,r),(0,0));assert_eq!(lookup.demand(snapshot.borrow(),target.borrow(),g.maximum_copy_bytes).unwrap(),d);}let(step,b,r)=semio_framework_trace_memory_testing::observe_backing(||lookup.advance(snapshot.borrow(),target.borrow(),g.maximum_copy_bytes,g).unwrap());assert_eq!((b,r),(step.progress().retained_capacity_bytes,step.progress().released_bytes));if lookup.ready(){let at=lookup.address().unwrap().unwrap();assert_eq!(at.length,1);assert_eq!(at.index(),0);ready=true;break}}
  if cut==10000{assert!(ready)}lookup.begin_close();for _ in 0..10000{if lookup.terminal_is_empty(){break}let(step,b,r)=semio_framework_trace_memory_testing::observe_backing(||lookup.close_step(g).unwrap());assert_eq!((b,r),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(lookup.terminal_is_empty());while !snapshot.terminal_is_empty(){snapshot.close_step(g).unwrap();}while !target.terminal_is_empty(){target.close_step(g).unwrap();}
 }
}
