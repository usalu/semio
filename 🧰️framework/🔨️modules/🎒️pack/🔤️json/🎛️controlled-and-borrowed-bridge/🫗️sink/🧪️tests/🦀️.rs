#[cfg(test)]
#[global_allocator]
static BORROWED_JSON_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

use semio_framework_pack_json::JsonBorrowedSinkCursor;
use semio_framework_value::{RetainedCloneGrant,RetirementDemand,NativeEncodeControl};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn close(cursor:&mut JsonBorrowedSinkCursor){
 for _ in 0..65536{
  if cursor.terminal_is_empty(){return}
  let g=grant(cursor.close_demands(4096).unwrap());
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(g));
  let p=step.result.unwrap().progress();assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));
 }
 panic!("borrowed sink frontier did not physically close")
}
#[test]
fn borrowed_sink_one_work_resumes_exact_canonical_output_and_physically_closes(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🫗️sink/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let input=serde_json::to_string(&row["input"]).unwrap();let source=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&input,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
  let expected=row["expected"].as_str().unwrap();assert_eq!(semio_framework_pack_json::to_json_string(&source),expected);assert_eq!(serde_json::to_string(&row["input"]).unwrap(),expected);
  let mut cursor=JsonBorrowedSinkCursor::empty(law["maximumDepth"].as_u64().unwrap()as usize).unwrap();
  let mut observer=|_|true;let mut control=NativeEncodeControl::new(65536,&mut observer);let mut output=Vec::new();
  for _ in 0..65536{
   if cursor.is_complete(){break}
   let mut bytes=[0;7];let d=cursor.next_demands(bytes.len(),4096).unwrap();let g=grant(d);let before=cursor.written_bytes();
   let (zero,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut bytes,g,0,&mut control));let zero=zero.result.unwrap();assert_eq!(zero.progress,Default::default());assert_eq!(zero.written_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cursor.written_bytes(),before);
   for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{
    let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut bytes,denied,1,&mut control));let p=step.result.unwrap();assert_eq!(p.progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cursor.written_bytes(),before);
   }
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut bytes,g,1,&mut control));assert_eq!(step.work,1);let result=step.result.unwrap();assert!(result.progress.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(result.progress.retained_capacity_bytes,result.progress.released_bytes));assert_eq!(cursor.written_bytes()-before,result.written_bytes);output.extend_from_slice(&bytes[..result.written_bytes]);
  }
  assert!(cursor.is_complete());assert_eq!(output,expected.as_bytes());close(&mut cursor);
 }
}
#[test]
fn borrowed_sink_cancel_keeps_frontier_and_exact_written_prefix_for_funded_close(){
 let source=semio_framework_value::DslValue::String("雪\\\"\n".repeat(100));
 for cut in[0,1,2,3,4,8,17]{
  let mut cursor=JsonBorrowedSinkCursor::empty(8).unwrap();let calls=std::cell::Cell::new(0usize);let mut observer=|_|{let value=calls.get();calls.set(value+1);value<cut};let mut control=NativeEncodeControl::new(65536,&mut observer);
  let mut output=Vec::new();let mut refused=false;
  for _ in 0..65536{
   let mut bytes=[0;7];let g=grant(cursor.next_demands(bytes.len(),4096).unwrap());
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut bytes,g,1,&mut control));
   let p=match &step.result{Ok(step)=>step.progress,Err(error)=>error.retained_progress()};assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));
   match step.result{Ok(step)=>output.extend_from_slice(&bytes[..step.written_bytes]),Err(_)=>{refused=true;break}}
  }
  assert!(refused);assert_eq!(cursor.written_bytes(),output.len());let canonical=semio_framework_pack_json::to_json_string(&source);assert!(canonical.as_bytes().starts_with(&output));close(&mut cursor);
 }
}

struct QuotedSource(u128);
impl semio_framework_pack_json::JsonWriteSource for QuotedSource{
 fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,semio_framework_value::ValueError>{assert!(path.is_empty());Ok(semio_framework_pack_json::JsonWriteNode::QuotedUnsigned(self.0))}
 fn object_key_at_path(&self,_:&[usize],_:usize)->Result<&str,semio_framework_value::ValueError>{panic!("quoted scalar has no object keys")}
}
#[test]
fn quoted_unsigned_sink_matches_shared_decimal_oracle_with_exact_heap_receipts(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🔢️decimal/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){let source=QuotedSource(row["value"].as_str().unwrap().parse().unwrap());let expected=row["expected"].as_str().unwrap();assert_eq!(serde_json::to_string(row["value"].as_str().unwrap()).unwrap(),expected);let mut cursor=JsonBorrowedSinkCursor::empty(1).unwrap();let mut observer=|_|true;let mut control=NativeEncodeControl::new(128,&mut observer);let mut output=Vec::new();
  for _ in 0..128{if cursor.is_complete(){break}let mut bytes=[0;7];let g=grant(cursor.next_demands(bytes.len(),4096).unwrap());let(before,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut bytes,RetainedCloneGrant{maximum_items:0,..g},1,&mut control));assert_eq!(before.result.unwrap().progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(&source,&mut bytes,g,1,&mut control));let step=step.result.unwrap();assert!(step.progress.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress.retained_capacity_bytes,step.progress.released_bytes));output.extend_from_slice(&bytes[..step.written_bytes]);}
  assert!(cursor.is_complete());assert_eq!(output,expected.as_bytes());close(&mut cursor);
 }
}

#[path="../../../👁️borrow/🧪️tests/🦀️.rs"]
mod native_projection;
