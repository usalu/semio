use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
struct ScalarJob(bool);
impl InteractiveJob for ScalarJob{
 fn step(&mut self,_:&mut StepContext<'_>)->StepOutcome{StepOutcome::Yield}
 fn begin_close(&mut self){self.0=true;}
 fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep{if grant.maximum_items==0||grant.maximum_depth==0{return InteractiveJobCloseStep::Pending{progress:Default::default()};}self.0=true;InteractiveJobCloseStep::Complete{progress:Default::default()}}
 fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn terminal_is_empty(&self)->bool{self.0}
}
fn source()->WorkerJobSession<ScalarJob>{
 let params=BatchJobParams{operation:OperationId(98201),generation:Generation(1),cancel:root_cancel_token(),config:BatchDriveConfig{retained:crate::retained_work::NO_RETAINED_WORK,site:"test.original-session-return",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:1000},now_us:default_now_us};
 WorkerJobSession::try_new(ScalarJob(false),params).unwrap_or_else(|_|panic!("original session admission"))
}
fn close_body(session:&WorkerJobSession<ScalarJob>){
 session.begin_close();
 for _ in 0..64{if session.terminal_is_empty(){return;}let grant=session.next_close_demands(512).unwrap();session.close_step(grant);}
 panic!("original session body did not close");
}
#[test]
fn original_session_frame_return_has_exact_custody(){
 let _admission=worker_session_slots_shared();
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut session=source();
  if row["phase"]=="terminal"{close_body(&session);}
  let held=(row["aliases"].as_u64().unwrap()>1).then(||SessionHandle::clone(&session.inner));
  let bytes=SessionHandle::<ScalarJob>::frame_bytes()+std::mem::size_of::<WorkerJobRetirementNode<ScalarJob>>();
  let granted_bytes=if row["releaseGrant"]==row["frameBytes"]{bytes}else{bytes-1};
  let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:granted_bytes,maximum_depth:1,..Default::default()};
  if row["waker"].as_bool().unwrap(){
   struct Wake;impl std::task::Wake for Wake{fn wake(self:Arc<Self>){}}
   let waker=std::task::Waker::from(Arc::new(Wake));
   unsafe{*session.inner.waker.get()=Some(waker)};
  }
  let (result,heap)=observe(||session.return_terminal(grant));
  assert_eq!(heap.requested_bytes,0);
  if row["expected"]=="returned"{
   let progress=result.unwrap_or_else(|_|panic!("funded original session return"));
   assert_eq!(progress.released_bytes,bytes);
   assert_eq!(heap.released_bytes,bytes);
  }else{
   let (_,original)=result.err().expect("original session retained");
   session=original;
   assert_eq!(heap.released_bytes,0);
   drop(held);
   if row["waker"].as_bool().unwrap(){unsafe{(&mut*session.inner.waker.get()).take()};}
   close_body(&session);
   let (result,heap)=observe(||session.return_terminal(RetainedCloneGrant{maximum_release_bytes:bytes,..grant}));
   assert_eq!(result.unwrap_or_else(|_|panic!("returned original after refusal")).released_bytes,bytes);
   assert_eq!((heap.requested_bytes,heap.released_bytes),(0,bytes));
  }
  eprintln!("[DEBUG] original session frame {} exact={bytes}",row["name"]);
 }
}
