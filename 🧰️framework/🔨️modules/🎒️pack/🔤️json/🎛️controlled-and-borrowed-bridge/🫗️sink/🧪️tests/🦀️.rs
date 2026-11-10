struct Heap{requested_bytes:usize,released_bytes:usize}
fn observe_heap<R>(run:impl FnOnce()->R)->(R,Heap){let(result,(requested_bytes,released_bytes))=semio_framework_value_testing_allocator::observe_retirement_allocations(run);(result,Heap{requested_bytes,released_bytes})}

use semio_framework_pack_json::JsonBorrowedSinkCursor;
use semio_framework_value::{RetainedCloneGrant,RetirementDemand,NativeEncodeControl};
use semio_framework_value_testing_admission::RetainedWorkPoll;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[path="../../../📥️decode/🧳️receiving/🧪️tests/🧰️support/🔐️authorities/🦀️.rs"]
mod authorities;
use authorities::{Authorities,WORK};
struct SinkCloser<'o>{cursor:&'o mut JsonBorrowedSinkCursor,turn:semio_framework_value_testing_admission::OriginalSourceTurnIdentity}
impl semio_framework_value_testing_admission::OriginalEffectProducer for SinkCloser<'_>{
 type Output=semio_framework_value::RetainedCloneStep;type Failure=semio_framework_value::OriginalRetirementFailure;
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),semio_framework_value::ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<semio_framework_value_testing_admission::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
 fn receive<'r,'turn,'source>(&mut self,_:&mut (),effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value_testing_admission::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value_testing_admission::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,semio_framework_value::ValueError>where 'source:'turn{
  match self.cursor.close_step(effect){RetainedWorkPoll::Complete(turn)=>Ok(reply.complete(turn.result)),RetainedWorkPoll::Pending{..}=>Ok(reply.pending())}
 }
 fn recover<'r,'turn,'source>(&mut self,_:&mut (),_:&mut semio_framework_value::RetainedAllocationEffect<'_>,_:semio_framework_value_testing_admission::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<semio_framework_value_testing_admission::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,semio_framework_value::ValueError>where 'source:'turn{Err(semio_framework_value::ValueError::invariant("synchronous sink close has no pending action"))}
}
fn close(cursor:&mut JsonBorrowedSinkCursor){
 let mut payer=Authorities::new();let mut callback=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(1<<20,&mut callback);
 type Producer<'o>=semio_framework_value_testing_admission::OriginalEffectAuthority<SinkCloser<'o>>;
 let reply=Producer::transfer_reply_demand().expect("reply quote");let producer_quote=Producer::producer_transfer_demand().expect("producer quote");
 for _ in 0..65536{
  if cursor.terminal_is_empty(){return}
  let g=grant(cursor.close_demands(4096).unwrap());let turn=payer.turn.identity();
  let wide=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes+reply.copy_bytes+producer_quote.copy_bytes,maximum_capacity_bytes:g.maximum_capacity_bytes+reply.capacity_bytes+producer_quote.capacity_bytes,maximum_release_bytes:g.maximum_release_bytes+reply.release_bytes+producer_quote.release_bytes,maximum_depth:g.maximum_depth+reply.depth+producer_quote.depth,..g};
  let(answer,heap)=observe_heap(||payer.effect(&mut control,wide,WORK,|_,effect|{
   let mut authority=Some(semio_framework_value_testing_admission::OriginalEffectAuthority::new(SinkCloser{cursor:&mut*cursor,turn}));let mut action=None;
   assert!(authority.as_mut().unwrap().drive(&mut (),effect,&mut action).expect("sink close action"),"synchronous sink close stayed pending");
   let mut output=None;let paid=authority.as_mut().unwrap().transfer_reply_into(&(),&mut action,&mut output,effect).result.expect("reply").expect("reply admitted");
   let mut producer=None;let moved=Producer::transfer_producer(&mut authority,&(),&mut producer,effect).result.expect("producer").expect("producer admitted");producer.take();
   match output.expect("transferred sink close reply"){Ok(step)=>Ok(step.progress().checked_add(paid).and_then(|total|total.checked_add(moved)).expect("receipt")),Err(failure)=>Err(format!("{failure:?}"))}
  }));
  let p=match answer{Ok(progress)=>progress,Err(failure)=>panic!("sink close step refused: {failure}")};assert!(p.fits(RetainedCloneGrant{maximum_items:wide.maximum_items+2,..wide}));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));
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
   let (zero,heap)=observe_heap(||cursor.step(&source,&mut bytes,g,0,&mut control));let zero=zero.result.unwrap();assert_eq!(zero.progress,Default::default());assert_eq!(zero.written_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cursor.written_bytes(),before);
   for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{
    let (step,heap)=observe_heap(||cursor.step(&source,&mut bytes,denied,1,&mut control));let p=step.result.unwrap();assert_eq!(p.progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cursor.written_bytes(),before);
   }
   let(step,heap)=observe_heap(||cursor.step(&source,&mut bytes,g,1,&mut control));assert_eq!(step.work,1);let result=step.result.unwrap();assert!(result.progress.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(result.progress.retained_capacity_bytes,result.progress.released_bytes));assert_eq!(cursor.written_bytes()-before,result.written_bytes);output.extend_from_slice(&bytes[..result.written_bytes]);
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
   let(step,heap)=observe_heap(||cursor.step(&source,&mut bytes,g,1,&mut control));
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
  for _ in 0..128{if cursor.is_complete(){break}let mut bytes=[0;7];let g=grant(cursor.next_demands(bytes.len(),4096).unwrap());let(before,heap)=observe_heap(||cursor.step(&source,&mut bytes,RetainedCloneGrant{maximum_items:0,..g},1,&mut control));assert_eq!(before.result.unwrap().progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(step,heap)=observe_heap(||cursor.step(&source,&mut bytes,g,1,&mut control));let step=step.result.unwrap();assert!(step.progress.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress.retained_capacity_bytes,step.progress.released_bytes));output.extend_from_slice(&bytes[..step.written_bytes]);}
  assert!(cursor.is_complete());assert_eq!(output,expected.as_bytes());close(&mut cursor);
 }
}

#[path="../../../👁️borrow/🧪️tests/🦀️.rs"]
mod native_projection;
