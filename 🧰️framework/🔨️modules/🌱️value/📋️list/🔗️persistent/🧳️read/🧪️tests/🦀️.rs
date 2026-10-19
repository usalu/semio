use semio_framework_test_fixtures::fixture_text;
use super::*;
use super::super::tests::{grant,observe,original,close_source,list};
fn make(count:usize)->PersistentList<i64>{if count==0{Default::default()}else{list(count)}}
fn close(cursor:&mut PersistentListRead<i64>){cursor.begin_close();for turn in 0..10000{if cursor.terminal_is_empty(){return;}let g=grant(cursor.close_demand(4096).unwrap());let(step,heap)=observe(||cursor.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<9999,"source-bound read closure stalled");}}
fn corpus()->serde_json::Value{serde_json::from_str(fixture_text!("../../🧳️read/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
/// 🔎️ Genuine original projections preserve ranked pointer identity with no row clone or traversal allocation.
#[test]
fn persistent_retained_read_matches_portable_ranked_rows_and_original_sources(){
 for case in corpus()["cases"].as_array().unwrap(){let count=case["length"].as_u64().unwrap()as usize;let mut source=original(make(count));let mut foreign=original(make(count));let mut cursor=PersistentListRead::new();let mut seen=Vec::new();
  for turn in 0..10000{let d=cursor.demand(source.borrow()).unwrap();let g=grant(d);let(step,heap)=observe(||cursor.advance(source.borrow(),g).unwrap());assert_eq!(heap,(0,0));assert!(step.progress().fits(g));if cursor.original.is_some(){let(result,heap)=observe(||cursor.advance(foreign.borrow(),g));assert!(result.is_err());assert_eq!(heap,(0,0));}
   match step{PersistentListReadStep::Row(_,index,row)=>{assert_eq!(index,seen.len());assert!(std::ptr::eq(row.get(),source.borrow().get().get(index).unwrap()));assert_eq!(*row.get(),(index*2)as i64);seen.push(index);},PersistentListReadStep::Complete(_)=>break,_=>{}}assert!(turn<9999);
  }assert_eq!(serde_json::json!(seen),case["expected"]);close(&mut cursor);assert_eq!(observe(||drop(cursor)).1,(0,0));close_source(&mut foreign);close_source(&mut source);eprintln!("[DEBUG] retained immutable read {} original rank/pointers preserved, heap0, foreign source refused",case["id"]);
 }
}
/// 🛑️ Every page-read frontier rejects denied work before effects and closes its genuine original source.
#[test]
fn persistent_retained_read_cancellation_closes_every_recorded_frontier(){
 let mut source=original(make(17));let mut cursor=PersistentListRead::new();let mut frontiers=0;
 loop{frontiers+=1;let g=grant(cursor.demand(source.borrow()).unwrap());let step=cursor.advance(source.borrow(),g).unwrap();if matches!(step,PersistentListReadStep::Complete(_)){break;}}close(&mut cursor);
 for cut in 0..frontiers{let mut cursor=PersistentListRead::new();for _ in 0..cut{let g=grant(cursor.demand(source.borrow()).unwrap());let before=(cursor.depth,cursor.index,cursor.started,cursor.done);for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:0,..g},RetainedCloneGrant{maximum_depth:0,..g}].into_iter().filter(|denied|denied.maximum_items==0||denied.maximum_depth==0||g.maximum_copy_bytes>0&&denied.maximum_copy_bytes<g.maximum_copy_bytes){let(step,heap)=observe(||cursor.advance(source.borrow(),denied).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());assert_eq!((cursor.depth,cursor.index,cursor.started,cursor.done),before);}cursor.advance(source.borrow(),g).unwrap();}close(&mut cursor);assert_eq!(observe(||drop(cursor)).1,(0,0));}
 close_source(&mut source);eprintln!("[DEBUG] retained immutable read every frontier{frontiers} refused denied axes before effects and settled original source aliases");
}
