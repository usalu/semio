use super::*;
use serde::Deserialize;
use std::cell::Cell;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
thread_local!{static DROPS:Cell<usize>=const{Cell::new(0)};}
struct Operation(u64);
impl Drop for Operation{fn drop(&mut self){DROPS.set(DROPS.get()+1);}}
struct Ledger(Vec<Edit<Operation>>);
impl ReplayEdits<Operation> for Ledger{fn replay_edit(&self,id:&str)->Option<&Edit<Operation>>{self.0.iter().find(|edit|edit.id==id)}}
#[derive(Deserialize)]struct Row{id:String,rows:usize}
#[derive(Deserialize,PartialEq,Debug)]struct Selected{id:String,offset:usize,rows:usize}
#[derive(Deserialize)]struct Case{id:String,ledger:Vec<Row>,batch:Vec<Row>,order:Vec<String>,selected:Option<Vec<Selected>>,committed:Vec<String>}
#[derive(Deserialize)]struct Corpus{cases:Vec<Case>}
fn edit(row:&Row)->Edit<Operation>{Edit{id:row.id.clone(),actor:None,line:None,forwards:(0..row.rows).map(|index|Operation(index as u64)).collect(),inverse:Default::default(),mutation_meta:Vec::new(),verb:None,sequence_number:0,started_at:String::new(),finished_at:None}}
#[test]
fn original_ingress_borrows_exact_rows_and_moves_only_accepted_owners(){
 let corpus:Corpus=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for law in corpus.cases{
  DROPS.set(0);let ledger=Ledger(law.ledger.iter().map(edit).collect());let batch:Vec<_>=law.batch.iter().map(edit).collect();let count=ledger.0.iter().chain(batch.iter()).map(|edit|edit.forwards.len()).sum::<usize>();
  let view=OriginalReplayEdits::new(&ledger,&batch);
  let ((),heap)=observe(||{for id in &law.order{let actual=view.replay_edit(id);let expected=batch.iter().find(|edit|edit.id==*id).or_else(||ledger.replay_edit(id));assert_eq!(actual.map(std::ptr::from_ref),expected.map(std::ptr::from_ref));if let Some(actual)=actual{for(index,operation)in actual.forwards.iter().enumerate(){assert_eq!(operation.0,index as u64);}}}});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0), "{}",law.id);assert_eq!(DROPS.get(),0);
  let selected=original_edits_for_ids(&law.order,&view).map(|edits|{let mut offset=0;edits.into_iter().map(|edit|{let row=Selected{id:edit.id.clone(),offset,rows:edit.forwards.len()};offset+=edit.forwards.len();row}).collect::<Vec<_>>()});
  assert_eq!(selected.ok(),law.selected,"{}",law.id);
  let pointers:Vec<_>=batch.iter().map(|edit|edit.forwards.as_ptr()).collect();drop(view);
  let mut accepted=Vec::new();for(index,edit)in batch.into_iter().enumerate(){if law.committed.contains(&edit.id){assert_eq!(edit.forwards.as_ptr(),pointers[index]);accepted.push(edit);}}
  assert_eq!(accepted.iter().map(|edit|edit.id.as_str()).collect::<Vec<_>>(),law.committed.iter().map(String::as_str).collect::<Vec<_>>());
  for edit in &accepted{let index=law.batch.iter().position(|row|row.id==edit.id).unwrap();assert_eq!(edit.forwards.as_ptr(),pointers[index]);}
  drop(accepted);drop(ledger);assert_eq!(DROPS.get(),count,"{}",law.id);
 }
}
