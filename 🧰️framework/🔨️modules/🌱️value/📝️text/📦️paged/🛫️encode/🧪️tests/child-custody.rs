//! 🧪️ A caller-resident original text producer recovers the same byte reply without replay.
use semio_framework_value_paged_text::{self as value,RetireOwned};
use value::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,OriginalSourceTurnWitness,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError,OriginalRetirementFailure};
use value::paged_text::{PagedText,PagedTextEncodeProducer,PagedTextProducerLoan,PagedTextEncodeOperation,InlineTextBuffer,TextReadSource,TextSourceByteReply,TextSourceByteComplete};
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};

struct Probe{body:String,before:bool,first:AtomicBool,pending:AtomicBool,calls:AtomicUsize,recoveries:AtomicUsize}
impl TextReadSource for Probe{
 fn byte_len(&self)->usize{self.body.len()}
 fn byte_at(&self,index:usize)->Option<u8>{self.body.as_bytes().get(index).copied()}
 fn read_recorded(&self,mut reply:TextSourceByteReply<'_,'_>)->TextSourceByteComplete{
  self.calls.fetch_add(1,Ordering::SeqCst);
  if self.first.swap(false,Ordering::SeqCst){if self.before{self.pending.store(true,Ordering::SeqCst);panic!("original byte before reply")}assert!(reply.performed(1).unwrap());let byte=self.byte_at(reply.index());let complete=reply.complete(Ok(byte),1);std::hint::black_box(&complete);panic!("original byte after reply")}
  assert!(reply.performed(1).unwrap());let byte=self.byte_at(reply.index());reply.complete(Ok(byte),1)
 }
 fn recover_recorded(&self,mut reply:TextSourceByteReply<'_,'_>)->Option<TextSourceByteComplete>{self.recoveries.fetch_add(1,Ordering::SeqCst);if !self.pending.swap(false,Ordering::SeqCst){return None}assert!(reply.performed(1).unwrap());let byte=self.byte_at(reply.index());Some(reply.complete(Ok(byte),1))}
}
impl RetireOwned for Probe{
 type RetirementCursor=<(String,bool,bool,bool,usize,usize) as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.body.retirement_ready()}
 fn retirement_seed(self)->Self::RetirementCursor{(self.body,self.before,self.first.into_inner(),self.pending.into_inner(),self.calls.into_inner(),self.recoveries.into_inner()).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{<(String,bool,bool,bool,usize,usize) as RetireOwned>::controlled_retirement_supported()}
}
type Producer<'i,'o>=PagedTextEncodeProducer<'i,'o,Probe,1024>;
type Loan<'p,'i,'o>=PagedTextProducerLoan<'p,'i,'o,Probe,1024>;
struct CloseLoan<'p,'i,'o>{producer:&'p mut Producer<'i,'o>}
impl OriginalEffectProducer for CloseLoan<'_,'_,'_>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;
 type Context<'a,'b>=OriginalSourceTurnWitness<'a> where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if c.identity()==self.producer.source_identity(){Ok(())}else{Err(ValueError::invariant("original text close Source changed"))}}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.identity())}
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{self.producer.close_into(4096,effect,reply)}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{self.producer.recover_close_into(4096,effect,reply)}
}
#[track_caller]
fn grant(bank:&EffectBank,d:value::RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=4096,"actual selected text metadata exceeds original turn: {d:?}");let left=bank.wallet().remaining();assert!(left.maximum_items>=1&&left.maximum_copy_bytes>=d.copy_bytes&&left.maximum_capacity_bytes>=d.capacity_bytes&&left.maximum_release_bytes>=d.release_bytes&&left.maximum_depth>=d.depth,"original demand {d:?} exceeds remaining {left:?}");RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn wallet()->Wallet{Wallet::new(RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64},10000)}
fn post(bank:&mut EffectBank,grant:RetainedCloneGrant,progress:RetainedCloneProgress,actual:Option<usize>,heap:(usize,usize)){if let Some(actual)=actual{assert_eq!(actual,heap.0)}assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,grant,heap)}
#[test]
fn caller_resident_source_recovers_original_byte_without_replay(){
 let law:serde_json::Value=serde_json::from_str(include_str!("child-custody.json")).unwrap();assert!(!law[0]["publish"].as_bool().unwrap());assert!(law[2]["publish"].as_bool().unwrap());
 for before in [true,false]{
  let mut bank=EffectBank::new(wallet());let mut source=RetainedCloneSource::from_owner(Probe{body:"x".to_owned(),before,first:AtomicBool::new(true),pending:AtomicBool::new(false),calls:AtomicUsize::new(0),recoveries:AtomicUsize::new(0)});let mut output:Option<PagedText<1024>>=None;let mut inline:Option<InlineTextBuffer>=None;
  {
   let mut producer=PagedTextEncodeProducer::new_inline(bank.source_witness(),source.borrow(),&mut output,&mut inline,1024,"","").unwrap();
   {
    let loan=PagedTextProducerLoan::new(bank.source_witness(),&mut producer).unwrap();let mut authority=Some(OriginalEffectAuthority::new(loan));let mut completed=false;let mut interruptions=0;let foreign=value::OriginalSourceTurnIssuer::new().issue().unwrap();let mut foreign_action=None;let d=value::RetirementDemand{depth:1,..Default::default()};let g=grant(&bank,d);let ((turn,actual),heap)=observe_retirement_allocations(||bank.run_with_source(g,1,|effect,_|{let before=effect.performed_work();assert!(authority.as_mut().unwrap().drive(&mut foreign.witness(),effect,&mut foreign_action).is_err());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(()),work:effect.performed_work()-before})}));let RetainedWorkPoll::Complete(turn)=turn else{panic!("foreign Source test pending")};turn.result.unwrap();assert!(foreign_action.is_none());assert_eq!(bank.take_admitted(),None);if let Some(actual)=actual{assert_eq!(actual,heap.0)}bank.post(Default::default(),g,heap);assert_eq!(source.borrow().get().calls.load(Ordering::SeqCst),0);
    for _ in 0..32{
     let selected=authority.as_ref().unwrap().producer().producer().operation().unwrap();let (d,work,_)=authority.as_ref().unwrap().producer().producer().demand().unwrap();assert!(d.copy_bytes<=4096,"selected {selected:?} demand {d:?}");let g=grant(&bank,d);let mut action=None;let mut observed=(0usize,0usize);
     let progress=loop{let hook=std::panic::take_hook();std::panic::set_hook(Box::new(|_|{}));let (attempt,heap)=observe_retirement_allocations(||match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||bank.run_with_source(g,work,|effect,mut witness|{let before=effect.performed_work();let ready=authority.as_mut().unwrap().drive(&mut witness,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}))){Ok(turn)=>Some(turn),Err(payload)=>{drop(payload);None}});std::panic::set_hook(hook);let Some((poll,actual))=attempt else{assert_eq!(heap.0,heap.1,"unwind infrastructure must release its transient allocations");interruptions+=1;continue};observed.0+=heap.0;observed.1+=heap.1;if let RetainedWorkPoll::Complete(turn)=poll{turn.result.unwrap();let progress=action.as_ref().unwrap().recorded_reply().unwrap().as_ref().unwrap().progress;post(&mut bank,g,progress,actual,observed);break progress}};
     let d=OriginalEffectAuthority::<Loan<'_,'_,'_>>::transfer_reply_demand().unwrap();let g=grant(&bank,d);let mut reply=None;
     let ((turn,actual),heap)=observe_retirement_allocations(||bank.run_with_source(g,1,|effect,witness|RetainedWorkPoll::Complete(authority.as_mut().unwrap().transfer_reply_into(&witness,&mut action,&mut reply,effect))));let RetainedWorkPoll::Complete(turn)=turn else{panic!("paid text reply remained pending")};let transfer=turn.result.unwrap().unwrap();post(&mut bank,g,transfer,actual,heap);assert_eq!(reply.as_ref().unwrap().as_ref().unwrap().progress,progress);
     let d=value::RetirementDemand{copy_bytes:std::mem::size_of_val(&reply),depth:1,..Default::default()};let g=grant(&bank,d);let ((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());drop(reply.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:effect.performed_work()-before})}));let RetainedWorkPoll::Complete(turn)=turn else{panic!("reply clear remained pending")};post(&mut bank,g,turn.result.unwrap(),actual,heap);
     if selected==PagedTextEncodeOperation::Publish{completed=true;break}
    }
    assert!(completed);assert_eq!(interruptions,1);assert_eq!(source.borrow().get().calls.load(Ordering::SeqCst),2);assert_eq!(source.borrow().get().recoveries.load(Ordering::SeqCst),usize::from(before));
    let d=OriginalEffectAuthority::<Loan<'_,'_,'_>>::producer_transfer_demand().unwrap();let g=grant(&bank,d);let mut returned=None;
    let ((turn,actual),heap)=observe_retirement_allocations(||bank.run_with_source(g,1,|effect,witness|RetainedWorkPoll::Complete(OriginalEffectAuthority::transfer_producer(&mut authority,&witness,&mut returned,effect))));let RetainedWorkPoll::Complete(turn)=turn else{panic!("loan return remained pending")};post(&mut bank,g,turn.result.unwrap().unwrap(),actual,heap);assert!(authority.is_none());
    let d=value::RetirementDemand{copy_bytes:std::mem::size_of_val(&returned),depth:1,..Default::default()};let g=grant(&bank,d);let ((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());drop(returned.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:effect.performed_work()-before})}));let RetainedWorkPoll::Complete(turn)=turn else{panic!("loan release remained pending")};post(&mut bank,g,turn.result.unwrap(),actual,heap);
   }
   producer.begin_close();
   {
    let mut authority=Some(OriginalEffectAuthority::new(CloseLoan{producer:&mut producer}));
    for _ in 0..1000{if authority.as_ref().unwrap().producer().producer.terminal_is_empty(){break}let d=authority.as_ref().unwrap().producer().producer.close_demand(4096).unwrap();let units=authority.as_ref().unwrap().producer().producer.close_work_demand(4096).unwrap();let g=grant(&bank,d);let mut action=None;let mut observed=(0,0);
     let progress=loop{let ((poll,actual),heap)=observe_retirement_allocations(||bank.run_with_source(g,units,|effect,mut witness|{let before=effect.performed_work();let ready=authority.as_mut().unwrap().drive(&mut witness,effect,&mut action).unwrap();let work=effect.performed_work()-before;if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));observed.0+=heap.0;observed.1+=heap.1;if let RetainedWorkPoll::Complete(turn)=poll{turn.result.unwrap();let progress=action.as_ref().unwrap().recorded_reply().unwrap().as_ref().unwrap().progress();post(&mut bank,g,progress,actual,observed);break progress}};
     let d=OriginalEffectAuthority::<CloseLoan<'_,'_,'_>>::transfer_reply_demand().unwrap();let g=grant(&bank,d);let mut reply=None;let ((turn,actual),heap)=observe_retirement_allocations(||bank.run_with_source(g,1,|effect,witness|RetainedWorkPoll::Complete(authority.as_mut().unwrap().transfer_reply_into(&witness,&mut action,&mut reply,effect))));let RetainedWorkPoll::Complete(turn)=turn else{panic!("close reply remained pending")};post(&mut bank,g,turn.result.unwrap().unwrap(),actual,heap);assert_eq!(reply.as_ref().unwrap().as_ref().unwrap().progress(),progress);
     let d=value::RetirementDemand{copy_bytes:std::mem::size_of_val(&reply),depth:1,..Default::default()};let g=grant(&bank,d);let ((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());drop(reply.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:effect.performed_work()-before})}));let RetainedWorkPoll::Complete(turn)=turn else{panic!("close reply clear pending")};post(&mut bank,g,turn.result.unwrap(),actual,heap);
    }
    assert!(authority.as_ref().unwrap().producer().producer.terminal_is_empty());let d=OriginalEffectAuthority::<CloseLoan<'_,'_,'_>>::producer_transfer_demand().unwrap();let g=grant(&bank,d);let mut returned=None;let ((turn,actual),heap)=observe_retirement_allocations(||bank.run_with_source(g,1,|effect,witness|RetainedWorkPoll::Complete(OriginalEffectAuthority::transfer_producer(&mut authority,&witness,&mut returned,effect))));let RetainedWorkPoll::Complete(turn)=turn else{panic!("close loan return pending")};post(&mut bank,g,turn.result.unwrap().unwrap(),actual,heap);let d=value::RetirementDemand{copy_bytes:std::mem::size_of_val(&returned),depth:1,..Default::default()};let g=grant(&bank,d);let ((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());drop(returned.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:effect.performed_work()-before})}));let RetainedWorkPoll::Complete(turn)=turn else{panic!("close loan release pending")};post(&mut bank,g,turn.result.unwrap(),actual,heap);
   }
   assert!(producer.terminal_is_empty());
  }
  assert!(output.is_none());assert_eq!(inline.as_ref().unwrap().borrow().unwrap(),"x");
  let d=value::RetirementDemand{copy_bytes:std::mem::size_of_val(&inline),depth:1,..Default::default()};let g=grant(&bank,d);let ((turn,actual),heap)=observe_retirement_allocations(||bank.run(g,1,|effect|{let before=effect.performed_work();assert!(effect.performed(1).unwrap());assert!(inline.as_mut().unwrap().close_one(1));drop(inline.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),work:effect.performed_work()-before})}));let RetainedWorkPoll::Complete(turn)=turn else{panic!("inline close pending")};post(&mut bank,g,turn.result.unwrap(),actual,heap);
  for _ in 0..10000{if source.terminal_is_empty(){break}source.close_once().unwrap();}assert!(source.terminal_is_empty());assert!(bank.is_idle());
 }
}
