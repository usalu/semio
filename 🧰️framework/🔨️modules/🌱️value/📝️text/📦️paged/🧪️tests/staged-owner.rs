//! 🧪️ The resident formatter keeps the original source, owner and paid publication across an interrupted reply.
use semio_framework_value_paged_text::{self as value,ErasedSnapshotRetirement};
use value::{OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,ValueError};
use value::paged_text::{PagedText,PagedTextEncodeFrame,PagedTextReadStep};
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_retirement::retirement::RetireOwned;
use std::panic::{catch_unwind,AssertUnwindSafe};

#[path="effect.rs"] pub(crate) mod eff;
#[path="native-rig.rs"] mod rig;
use rig::{Work,Physical,grant,source,close_source,close_text,close_encode_failure,decode_turn,close_decode_frame,close_owned_failure};

#[test]
fn recorded_grant_resumes_the_original_formatter_and_paid_publication(){
 use rig::encoder::{EncodeBank,Stock,projection_work,drive_frame,return_loan,close_producer};use value::paged_text::PagedTextEncodeProducer;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../📋️list/🧪️tests/🧫️fixtures/🏷️formatted-exact-extent.json")).unwrap();let body="x".repeat(law["payloadBytes"].as_u64().unwrap()as usize);let expected=format!("Set label to {body}");assert_eq!(expected.len(),law["expectedEnBytes"].as_u64().unwrap()as usize);let original=source(body);let stock=Stock::for_outputs::<16384>(&[(expected.len(),projection_work(original.borrow()).unwrap())],65536,64).unwrap();let mut bank=EncodeBank::new(stock,true);let mut output=None;
 let admitted=bank.run(65536,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,16384>::new(turn.witness(),original.borrow(),&mut output,16384,"Set label to ","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;let(demand,_,_)=frame.demand()?;let g=grant(demand);let interrupted=catch_unwind(AssertUnwindSafe(||frame.advance_body(control,turn,g)));assert!(interrupted.is_err());assert_eq!(frame.demand()?.0,demand);drive_frame(&mut frame,control,turn)?;return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;assert!(producer.terminal_is_empty());drop(producer);Ok(control.owned_bytes())}).unwrap();let paid=bank.finish();assert!(paid.work<stock.work);let text=output.take().expect("funded publication retained the output");assert!(text.borrow().unwrap().bytes().eq(expected.bytes()));assert_eq!(text.allocated_bytes(),admitted);close_text(text);close_source(original);
}
#[test]
fn intrinsic_formatter_publishes_only_the_original_selected_cell(){
 use value::paged_text::InlineTextBuffer;use rig::encoder::{EncodeBank,Stock,Payment,encode_inline_success,projection_work};
 let original=source("short".to_owned());let stock=Stock::for_outputs::<1024>(&[(18,projection_work(original.borrow()).unwrap())],4096,64).unwrap();let mut bank=EncodeBank::new(stock,false);
 let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(4096,64,|turn,control|{let(mut output,mut inline)=encode_inline_success::<_,1024>(&original,control,turn,1024,"Set label to ","")?;assert!(output.is_none());assert_eq!(inline.as_ref().unwrap().borrow()?,"Set label to short");assert_eq!(control.owned_bytes(),0);let d=value::RetirementDemand{copy_bytes:std::mem::size_of::<Option<InlineTextBuffer>>(),depth:1,..Default::default()};let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);let mut cell=inline.take().unwrap();assert!(cell.close_one(1));assert!(cell.terminal_is_empty());drop(cell);Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;assert!(output.take().is_none());assert!(inline.is_none());Ok(())}));result.unwrap();let paid=bank.finish();assert_eq!(heap,(paid.submitted.retained_capacity_bytes,paid.submitted.released_bytes));assert_eq!(heap,(0,0));close_source(original);
}
#[test]
fn decoder_publishes_the_actual_owner_after_separate_funding(){
 use value::paged_text::PagedTextDecodeFrame;
 let original=source("Grüße".to_owned());let mut physical=OriginalConstructionAuthority::new(Physical{interrupt:false,submitted:Default::default()});let mut work=OriginalWorkAuthority::new(Work);let mut callback=|_|true;let mut control=value::NativeDecodeControl::new(4096,&mut callback);let mut output=None;let mut frame=PagedTextDecodeFrame::<1024>::new(&mut output).unwrap();
 let mut done=false;for _ in 0..1000{let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();match turn.result{Ok(PagedTextReadStep::Complete(_))=>{done=true;break},Ok(PagedTextReadStep::Progress(_)|PagedTextReadStep::Pending)=>{},_=>panic!("decoder refused its admitted original turn")}}assert!(done);frame.begin_close();rig::close_decode_frame(&mut frame);assert!(frame.terminal_is_empty());drop(frame);let text=output.take().expect("actual decoded owner was published");assert_eq!(text.borrow().unwrap().chars().collect::<String>(),"Grüße");assert_eq!(text.allocated_bytes(),control.owned_bytes());close_text(text);drop(control);assert!(!work.has_live_actions());assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());close_source(original);
}

struct InterruptedText{body:String,attempts:std::sync::atomic::AtomicUsize}
impl value::paged_text::TextReadSource for InterruptedText{
 fn byte_len(&self)->usize{self.body.len()}
 fn byte_at(&self,index:usize)->Option<u8>{self.body.as_bytes().get(index).copied()}
 fn read_recorded(&self,mut reply:value::paged_text::TextSourceByteReply<'_,'_>)->value::paged_text::TextSourceByteComplete{let _=reply.performed(1).expect("original Work receipt must accept the source report");let index=reply.index();let attempt=self.attempts.fetch_add(1,std::sync::atomic::Ordering::SeqCst);let result=reply.complete(Ok(self.body.as_bytes().get(index).copied()),1);if attempt==0{std::hint::black_box(&result);panic!("original byte producer interrupted after recording its actual reply")}result}
}
impl RetireOwned for InterruptedText{
 type RetirementCursor=<(String,usize) as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{(self.body,self.attempts.into_inner()).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{<(String,usize) as RetireOwned>::controlled_retirement_supported()}
}
#[test]
fn encoder_settles_recorded_byte_after_unwind_then_cancels_without_replay(){
 use value::paged_text::PagedTextEncodeProducer;use rig::encoder::{EncodeBank,Stock,projection_work,drive_one,return_loan,close_producer_observed};
 let original=source(InterruptedText{body:"übrige".to_owned(),attempts:std::sync::atomic::AtomicUsize::new(0)});let stock=Stock::for_outputs::<1024>(&[(7,projection_work(original.borrow()).unwrap())],4096,64).unwrap();let mut bank=EncodeBank::new(stock,false);let mut output=None;
 bank.run(4096,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,1024>::new(turn.witness(),original.borrow(),&mut output,1024,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;let mut interrupted=false;for _ in 0..100{let result=catch_unwind(AssertUnwindSafe(||drive_one(&mut frame,control,turn)));if result.is_err(){interrupted=true;break}result.unwrap()?;}assert!(interrupted);assert_eq!(original.borrow().get().attempts.load(std::sync::atomic::Ordering::SeqCst),1);drive_one(&mut frame,control,turn)?;assert_eq!(original.borrow().get().attempts.load(std::sync::atomic::Ordering::SeqCst),1);return_loan(&mut frame,control,turn)?;drop(frame);let mut scalar_retired=false;close_producer_observed(&mut producer,control,turn,|progress,work|{if progress.copied_bytes==2*std::mem::size_of::<usize>()+1{assert_eq!(work,1);scalar_retired=true}})?;assert!(scalar_retired,"original partial UTF8 scalar must be explicitly retired");assert!(producer.terminal_is_empty());drop(producer);Ok(())}).unwrap();bank.finish();assert!(output.is_none());close_source(original);
}

#[test]
fn decoder_settles_recorded_byte_after_unwind_then_cancels_without_replay(){
 use value::paged_text::PagedTextDecodeFrame;
 let original=source(InterruptedText{body:"übrige".to_owned(),attempts:std::sync::atomic::AtomicUsize::new(0)});let mut physical=OriginalConstructionAuthority::new(Physical{interrupt:false,submitted:Default::default()});let mut work=OriginalWorkAuthority::new(Work);let mut callback=|_|true;let mut control=value::NativeDecodeControl::new(4096,&mut callback);let mut output=None;let mut frame=PagedTextDecodeFrame::<1024>::new(&mut output).unwrap();
 let mut interrupted=false;for _ in 0..100{let g=grant(frame.demand(original.borrow()).unwrap());let result=catch_unwind(AssertUnwindSafe(||physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap()));if result.is_err(){interrupted=true;break}let turn=result.unwrap().unwrap().unwrap();assert!(turn.result.is_ok())}assert!(interrupted);assert_eq!(original.borrow().get().attempts.load(std::sync::atomic::Ordering::SeqCst),1);frame.begin_close();let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();assert!(turn.result.is_ok());assert_eq!(original.borrow().get().attempts.load(std::sync::atomic::Ordering::SeqCst),1);
 let mut scalar_retired=false;rig::close_decode_frame_observed(&mut frame,|progress,work|{if progress.copied_bytes==2*std::mem::size_of::<usize>()+1{assert_eq!(work,1);scalar_retired=true}});assert!(scalar_retired,"original partial UTF8 scalar must be explicitly retired");assert!(frame.terminal_is_empty());drop(frame);assert!(output.is_none());drop(control);assert!(!work.has_live_actions());assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());close_source(original);
}

#[test]
fn decoder_publishes_only_its_original_intrinsic_owner(){
 use value::paged_text::{PagedTextDecodeFrame,InlineTextBuffer};
 let original=source("Grüße".to_owned());let mut physical=OriginalConstructionAuthority::new(Physical{interrupt:false,submitted:Default::default()});let mut work=OriginalWorkAuthority::new(Work);let mut callback=|_|true;let mut control=value::NativeDecodeControl::new(4096,&mut callback);let mut output=None;let mut inline:Option<InlineTextBuffer>=None;let mut frame=PagedTextDecodeFrame::<1024>::new_inline(&mut output,&mut inline).unwrap();
 let mut done=false;for _ in 0..1000{let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();match turn.result{Ok(PagedTextReadStep::Complete(_))=>{done=true;break},Ok(PagedTextReadStep::Progress(_)|PagedTextReadStep::Pending)=>{},_=>panic!("intrinsic decoder refused its admitted original turn")}}assert!(done);frame.begin_close();rig::close_decode_frame(&mut frame);assert!(frame.terminal_is_empty());drop(frame);assert!(output.is_none());let mut cell=inline.take().expect("actual intrinsic owner was published");assert_eq!(cell.borrow().unwrap(),"Grüße");assert!(cell.close_one(1));assert!(cell.terminal_is_empty());assert_eq!(control.owned_bytes(),0);drop(control);assert!(!work.has_live_actions());assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());close_source(original);
}

struct ChangedText{body:Vec<u8>,attempts:std::sync::atomic::AtomicUsize}
impl value::paged_text::TextReadSource for ChangedText{
 fn byte_len(&self)->usize{self.body.len()}
 fn byte_at(&self,index:usize)->Option<u8>{let turn=self.attempts.fetch_add(1,std::sync::atomic::Ordering::SeqCst);if turn>=self.body.len(){Some(255)}else{self.body.get(index).copied()}}
}
impl RetireOwned for ChangedText{
 type RetirementCursor=<(Vec<u8>,usize) as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{(self.body,self.attempts.into_inner()).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{<(Vec<u8>,usize) as RetireOwned>::controlled_retirement_supported()}
}
#[test]
fn changed_second_pass_cannot_publish_invalid_utf8(){
 use value::paged_text::PagedTextEncodeProducer;use rig::encoder::{EncodeBank,Stock,projection_work,drive_one_result,take_frame_reply,return_loan,close_producer,retire_published_failure};
 let original=source(ChangedText{body:b"ok".to_vec(),attempts:std::sync::atomic::AtomicUsize::new(0)});let stock=Stock::for_outputs::<1024>(&[(2,projection_work(original.borrow()).unwrap())],4096,64).unwrap();let mut bank=EncodeBank::new(stock,false);let mut output=None;
 bank.run(4096,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,1024>::new(turn.witness(),original.borrow(),&mut output,1024,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;let mut refused=false;for _ in 0..100{if !drive_one_result(&mut frame,control,turn)?.1{assert_eq!(frame.reply().unwrap().as_ref().unwrap_err().cause.kind,value::ValueRefusalKind::InvalidValue);refused=true;break}}assert!(refused);let mut published=None;take_frame_reply(&mut frame,control,turn,&mut published)?;return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;assert!(producer.terminal_is_empty());drop(producer);retire_published_failure(&mut published,control,turn)?;Ok(())}).unwrap();bank.finish();assert!(output.is_none());close_source(original);
}

struct RefusingText{address:std::sync::atomic::AtomicUsize,attempts:std::sync::atomic::AtomicUsize,reported:u64,error:std::sync::Mutex<Option<ValueError>>}
impl value::paged_text::TextReadSource for RefusingText{
 fn byte_len(&self)->usize{1}
 fn byte_at(&self,_:usize)->Option<u8>{None}
 fn read_recorded(&self,mut reply:value::paged_text::TextSourceByteReply<'_,'_>)->value::paged_text::TextSourceByteComplete{let _=reply.performed(self.reported).expect("original Work receipt must accept the source report");self.attempts.fetch_add(1,std::sync::atomic::Ordering::SeqCst);assert_eq!(reply.index(),0);let error=self.error.lock().unwrap().take().expect("original source error already transferred");self.address.store(error.message.as_ptr()as usize,std::sync::atomic::Ordering::SeqCst);let done=reply.complete(Err(error),self.reported);std::hint::black_box(&done);panic!("actual source error recorded before interruption")}
}
impl RetireOwned for RefusingText{
 type RetirementCursor=<(usize,usize,u64,Option<ValueError>) as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.error.lock().ok().is_some_and(|error|error.retirement_ready())}
 fn retirement_seed(self)->Self::RetirementCursor{(self.address.into_inner(),self.attempts.into_inner(),self.reported,self.error.into_inner().unwrap_or_else(|error|error.into_inner())).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{<(usize,usize,u64,Option<ValueError>) as RetireOwned>::controlled_retirement_supported()}
}

#[test]
fn interrupted_source_error_retains_the_same_owned_message(){
 use value::paged_text::PagedTextEncodeProducer;use rig::encoder::{EncodeBank,Stock,projection_work,drive_one_result,take_frame_reply,return_loan,close_producer,retire_published_failure};
 let original=source(RefusingText{address:std::sync::atomic::AtomicUsize::new(0),attempts:std::sync::atomic::AtomicUsize::new(0),reported:1,error:std::sync::Mutex::new(Some(ValueError::new(value::ValueRefusalKind::Canceled,"actual-source-refusal-0".to_owned())))});let stock=Stock::for_outputs::<1024>(&[(1,projection_work(original.borrow()).unwrap())],4096,64).unwrap();let mut bank=EncodeBank::new(stock,false);let mut output=None;
 bank.run(4096,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,1024>::new(turn.witness(),original.borrow(),&mut output,1024,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;let mut interrupted=false;for _ in 0..100{let result=catch_unwind(AssertUnwindSafe(||drive_one_result(&mut frame,control,turn)));if result.is_err(){interrupted=true;break}assert!(result.unwrap()?.1)}assert!(interrupted);let address=original.borrow().get().address.load(std::sync::atomic::Ordering::SeqCst);assert_ne!(address,0);assert!(!drive_one_result(&mut frame,control,turn)?.1);let failure=frame.reply().unwrap().as_ref().unwrap_err();assert_eq!(failure.cause.kind,value::ValueRefusalKind::Canceled);assert_eq!(failure.cause.message.as_ptr()as usize,address);assert_eq!(original.borrow().get().attempts.load(std::sync::atomic::Ordering::SeqCst),1);let mut published=None;take_frame_reply(&mut frame,control,turn,&mut published)?;return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;assert!(producer.terminal_is_empty());drop(producer);retire_published_failure(&mut published,control,turn)?;Ok(())}).unwrap();bank.finish();assert!(output.is_none());close_source(original);
}

#[test]
fn overrun_retains_original_error_and_exact_three_work_without_replay(){
 use value::paged_text::{PagedTextEncodeProducer,PagedTextReplyPayout};use semio_framework_value_native_encoding::{NativeEncodeProducerReplyStep,NativeEncodeEffectFailureSlot};use rig::encoder::{EncodeBank,Stock,Payment,projection_work,drive_one,return_loan,close_producer,take_frame_reply,retire_published_failure,retire_value};
 let original=source(RefusingText{address:std::sync::atomic::AtomicUsize::new(0),attempts:std::sync::atomic::AtomicUsize::new(0),reported:3,error:std::sync::Mutex::new(Some(ValueError::new(value::ValueRefusalKind::Canceled,"actual-source-refusal-0".to_owned())))});let stock=Stock::for_outputs::<1024>(&[(1,projection_work(original.borrow()).unwrap())],4096,64).unwrap();let mut bank=EncodeBank::new(stock,false);let mut output=None;
 bank.run(4096,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,1024>::new(turn.witness(),original.borrow(),&mut output,1024,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;drive_one(&mut frame,control,turn)?;let(d,work,_)=frame.demand()?;assert_eq!(work,2);let g=grant(d);let before=control.actual_work();let interrupted=catch_unwind(AssertUnwindSafe(||frame.advance_body(control,turn,g)));assert!(interrupted.is_err());assert_eq!(control.actual_work()-before,3);let address=original.borrow().get().address.load(std::sync::atomic::Ordering::SeqCst);assert_ne!(address,0);assert!(frame.producer().unwrap().resident_reply_facts().is_some());let _=frame.advance_body(control,turn,g)?;frame.acknowledge_body_partial_success()?;frame.claim_body_work()?;assert_eq!(original.borrow().get().attempts.load(std::sync::atomic::Ordering::SeqCst),1);
 let(d,_)=frame.resident_reply_recovery_demand()?;let mut recovered=false;for _ in 0..100{let status=frame.advance_resident_reply_recovery(control,turn,grant(d))?;frame.acknowledge_resident_recovery_partial_success()?;frame.claim_resident_recovery_work()?;if matches!(status,NativeEncodeProducerReplyStep::Transferred(_)){recovered=true;break}}assert!(recovered);assert!(frame.failure().unwrap().work_overrun.is_some(),"actual work overrun must remain independent");let mut overrun=None;{let mut payout=PagedTextReplyPayout::new_body_failure(&mut frame,turn,NativeEncodeEffectFailureSlot::WorkOverrun,&mut overrun)?;let mut pay=Payment::default();pay.begin(control,payout.demand()?,1)?;pay.stage(control,|physical,action,effect|payout.stage(physical,action,effect).result)?;assert!(pay.post(control)?);assert!(payout.finish(control,turn,&mut pay.physical,&mut pay.proof,&mut pay.work)?);}
 let d=frame.reply_demand()?;let mut ready=false;for _ in 0..100{let status=frame.advance_reply(control,turn,grant(d))?;frame.acknowledge_reply_partial_success()?;frame.claim_reply_work()?;if matches!(status,NativeEncodeProducerReplyStep::Transferred(_)){ready=true;break}}assert!(ready);let failure=frame.reply().unwrap().as_ref().unwrap_err();assert_eq!(failure.cause.kind,value::ValueRefusalKind::Canceled);assert_eq!(failure.cause.message.as_ptr()as usize,address);assert_eq!(original.borrow().get().attempts.load(std::sync::atomic::Ordering::SeqCst),1);let mut published=None;take_frame_reply(&mut frame,control,turn,&mut published)?;return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;assert!(producer.terminal_is_empty());drop(producer);retire_published_failure(&mut published,control,turn)?;retire_value(&mut overrun,control,turn)?;Ok(())}).unwrap();bank.finish();assert!(output.is_none());close_source(original);
}

#[test]
fn caller_cancels_at_actual_validation_and_copy_progress(){
 use value::paged_text::{PagedTextStage,PagedTextEncodeProducer};use rig::encoder::{EncodeBank,Stock,projection_work,drive_one,return_loan,close_producer};
 let original=source("x".repeat(8194));for target in [PagedTextStage::Validating,PagedTextStage::Copying]{let stock=Stock::for_outputs::<16384>(&[(8194,projection_work(original.borrow()).unwrap())],65536,64).unwrap();let mut bank=EncodeBank::new(stock,false);let mut output=None;
 let(result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(65536,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,16384>::new(turn.witness(),original.borrow(),&mut output,16384,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;let mut reached=false;
 for _ in 0..100000{let p=frame.producer().unwrap();if p.stage()==target&&p.phase_progress().0==256{reached=true;break}drive_one(&mut frame,control,turn)?;}assert!(reached);let p=frame.producer().unwrap();if target==PagedTextStage::Validating{assert_eq!(p.retained_len(),0);assert_eq!(p.retained_capacity(),0)}else{assert_eq!(p.retained_len(),256);assert!(p.retained_capacity()>=4096)}let expected=p.retained_capacity();assert_eq!(control.owned_bytes(),expected);return_loan(&mut frame,control,turn)?;drop(frame);producer.begin_close();assert_eq!(producer.stage(),PagedTextStage::Closing);close_producer(&mut producer,control,turn)?;assert!(producer.terminal_is_empty());drop(producer);Ok(())}));result.unwrap();let paid=bank.finish();assert_eq!(heap,(paid.submitted.retained_capacity_bytes,paid.submitted.released_bytes));assert_eq!(heap.0,heap.1);assert!(output.is_none());
 }close_source(original);
}

struct RefusingFinish{body:String,calls:std::sync::atomic::AtomicUsize}
impl value::paged_text::TextReadSource for RefusingFinish{
 fn byte_len(&self)->usize{self.body.len()}
 fn byte_at(&self,index:usize)->Option<u8>{self.body.as_bytes().get(index).copied()}
 fn finish_recorded(&self,mut reply:value::paged_text::TextSourceFinishReply<'_,'_>)->value::paged_text::TextSourceFinishComplete{let _=reply.performed(1).expect("original Work receipt must accept the source report");self.calls.fetch_add(1,std::sync::atomic::Ordering::SeqCst);let done=reply.complete(Err(ValueError::literal(value::ValueRefusalKind::Canceled,"original producer refused after its full fragment")),1);std::hint::black_box(&done);panic!("original producer final refusal recorded before interruption")}
}
impl RetireOwned for RefusingFinish{
 type RetirementCursor=<(String,usize) as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{(self.body,self.calls.into_inner()).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{<(String,usize) as RetireOwned>::controlled_retirement_supported()}
}

#[test]
fn producer_final_refusal_retains_short_intrinsic_prefix_without_publication(){
 use value::paged_text::{InlineTextBuffer,PagedTextEncodeProducer};use rig::encoder::{EncodeBank,Stock,projection_work,drive_one_result,take_frame_reply,return_loan,close_producer,retire_published_failure};
 let original=source(RefusingFinish{body:"first".to_owned(),calls:std::sync::atomic::AtomicUsize::new(0)});let stock=Stock::for_outputs::<1024>(&[(5,projection_work(original.borrow()).unwrap())],4096,64).unwrap();let mut bank=EncodeBank::new(stock,false);let mut output=None;let mut inline:Option<InlineTextBuffer>=None;
 bank.run(4096,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,1024>::new_inline(turn.witness(),original.borrow(),&mut output,&mut inline,1024,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;let mut interrupted=false;for _ in 0..100{let result=catch_unwind(AssertUnwindSafe(||drive_one_result(&mut frame,control,turn)));if result.is_err(){interrupted=true;break}assert!(result.unwrap()?.1)}assert!(interrupted);assert_eq!(frame.producer().unwrap().retained_len(),5);assert_eq!(frame.producer().unwrap().retained_capacity(),0);assert_eq!(original.borrow().get().calls.load(std::sync::atomic::Ordering::SeqCst),1);assert!(!drive_one_result(&mut frame,control,turn)?.1);assert_eq!(frame.reply().unwrap().as_ref().unwrap_err().cause.kind,value::ValueRefusalKind::Canceled);assert_eq!(original.borrow().get().calls.load(std::sync::atomic::Ordering::SeqCst),1);let mut published=None;take_frame_reply(&mut frame,control,turn,&mut published)?;return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;assert!(producer.terminal_is_empty());drop(producer);retire_published_failure(&mut published,control,turn)?;Ok(())}).unwrap();bank.finish();assert!(output.is_none()&&inline.is_none());close_source(original);
}

#[test]
fn decoder_final_refusal_retains_its_intrinsic_prefix_without_publication(){
 use value::paged_text::{PagedTextDecodeFrame,InlineTextBuffer};
 let original=source(RefusingFinish{body:"first".to_owned(),calls:std::sync::atomic::AtomicUsize::new(0)});let mut physical=OriginalConstructionAuthority::new(Physical{interrupt:false,submitted:Default::default()});let mut work=OriginalWorkAuthority::new(Work);let mut callback=|_|true;let mut control=value::NativeDecodeControl::new(4096,&mut callback);let mut output=None;let mut inline:Option<InlineTextBuffer>=None;let mut frame=PagedTextDecodeFrame::<1024>::new_inline(&mut output,&mut inline).unwrap();
 let mut interrupted=false;for _ in 0..100{let g=grant(frame.demand(original.borrow()).unwrap());let result=catch_unwind(AssertUnwindSafe(||physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap()));if result.is_err(){interrupted=true;break}assert!(result.unwrap().unwrap().unwrap().result.is_ok())}assert!(interrupted);assert_eq!(frame.retained_len(),5);assert_eq!(frame.retained_capacity(),0);assert_eq!(original.borrow().get().calls.load(std::sync::atomic::Ordering::SeqCst),1);
 let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut (),|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();let failure=match turn.result{Err(failure)=>failure,_=>panic!("original final producer refusal was lost")};assert_eq!(failure.child.as_ref().unwrap().kind,value::ValueRefusalKind::Canceled);assert_eq!(original.borrow().get().calls.load(std::sync::atomic::Ordering::SeqCst),1);rig::close_owned_failure(failure);frame.begin_close();rig::close_decode_frame(&mut frame);assert!(frame.terminal_is_empty());drop(frame);assert!(output.is_none()&&inline.is_none());drop(control);assert!(!work.has_live_actions());assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());close_source(original);
}

struct FinishingText{body:String,finishes:std::sync::atomic::AtomicUsize}
impl value::paged_text::TextReadSource for FinishingText{
 fn byte_len(&self)->usize{self.body.len()}
 fn byte_at(&self,index:usize)->Option<u8>{self.body.as_bytes().get(index).copied()}
 fn finish_recorded(&self,mut reply:value::paged_text::TextSourceFinishReply<'_,'_>)->value::paged_text::TextSourceFinishComplete{let _=reply.performed(1).expect("original Work receipt must accept the source report");self.finishes.fetch_add(1,std::sync::atomic::Ordering::SeqCst);reply.complete(Ok(()),1)}
}
impl RetireOwned for FinishingText{type RetirementCursor=<(String,usize) as RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{(self.body,self.finishes.into_inner()).retirement_seed()}fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::RetirementCursor>())}fn controlled_retirement_supported()->bool{<(String,usize) as RetireOwned>::controlled_retirement_supported()}}
struct FinishReceiptPort{submitted:RetainedCloneProgress}
impl OriginalConstructionRecipient for FinishReceiptPort{
 type Context<'turn,'source>=bool where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&bool)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&bool)->Option<value::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn grant<'a,'turn,'source>(&mut self,_:&mut bool,quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{let d=quote.demand();quote.complete(Ok(grant(d)))}
 fn accept<'a,'turn,'source>(&mut self,interrupt:&mut bool,receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let result=self.submitted.checked_add(receipt.progress()).map(|next|self.submitted=next);let done=receipt.complete(result);if *interrupt{*interrupt=false;std::hint::black_box(&done);panic!("original final source receipt interrupted after reply")}done}
 fn submitted<'turn,'source>(&self,_:&bool)->RetainedCloneProgress where 'source:'turn{self.submitted}
}
#[test]
fn original_final_receipt_must_settle_before_paid_publication(){
 use value::paged_text::{PagedTextStage,PagedTextEncodeProducer,PagedTextEncodeOperation};use rig::encoder::{EncodeBank,Stock,projection_work,drive_one,drive_frame,return_loan,close_producer};
 let original=source(FinishingText{body:"ok".to_owned(),finishes:std::sync::atomic::AtomicUsize::new(0)});let stock=Stock::for_outputs::<1024>(&[(2,projection_work(original.borrow()).unwrap())],4096,64).unwrap();let mut bank=EncodeBank::new(stock,false);let arm=bank.post_interrupt();let mut output=None;
 bank.run(4096,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,1024>::new(turn.witness(),original.borrow(),&mut output,1024,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;
 for _ in 0..100{let p=frame.producer().unwrap();if p.stage()==PagedTextStage::Copying&&p.phase_progress()==(2,Some(2)){break}drive_one(&mut frame,control,turn)?;}assert_eq!(frame.producer().unwrap().stage(),PagedTextStage::Copying);assert_eq!(frame.producer().unwrap().phase_progress(),(2,Some(2)));assert_eq!(frame.producer().unwrap().operation()?,PagedTextEncodeOperation::Finish);let demand=frame.demand()?;arm.store(true,std::sync::atomic::Ordering::SeqCst);let interrupted=catch_unwind(AssertUnwindSafe(||drive_one(&mut frame,control,turn)));assert!(interrupted.is_err());assert_eq!(frame.producer().unwrap().stage(),PagedTextStage::Publishing);assert!(frame.reply().is_none());assert_eq!(frame.operation()?,PagedTextEncodeOperation::Finish);assert_eq!(original.borrow().get().finishes.load(std::sync::atomic::Ordering::SeqCst),1);assert_eq!(frame.demand()?,demand);drive_one(&mut frame,control,turn)?;assert_eq!(original.borrow().get().finishes.load(std::sync::atomic::Ordering::SeqCst),1);assert_eq!(frame.producer().unwrap().operation()?,PagedTextEncodeOperation::Publish);assert_eq!(frame.demand()?.0.copy_bytes,2*std::mem::size_of::<Option<PagedText<1024>>>()+std::mem::size_of::<Option<Result<RetainedCloneProgress,ValueError>>>()+std::mem::size_of::<RetainedCloneProgress>()+std::mem::size_of::<Option<Result<semio_framework_value_native_encoding::NativeEncodeProducerOutcome,semio_framework_value_native_encoding::NativeEncodeProducerFailure>>>());drive_frame(&mut frame,control,turn)?;return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;drop(producer);Ok(())}).unwrap();bank.finish();let text=output.take().unwrap();assert_eq!(text.borrow().unwrap().chars().collect::<String>(),"ok");close_text(text);close_source(original);
}

#[test]
fn decoder_final_receipt_must_settle_before_paid_publication(){
 use value::paged_text::{PagedTextStage,PagedTextDecodeFrame};
 let original=source(FinishingText{body:"ok".to_owned(),finishes:std::sync::atomic::AtomicUsize::new(0)});let mut physical=OriginalConstructionAuthority::new(FinishReceiptPort{submitted:Default::default()});let mut work=OriginalWorkAuthority::new(Work);let mut callback=|_|true;let mut control=value::NativeDecodeControl::new(4096,&mut callback);let mut output=None;let mut frame=PagedTextDecodeFrame::<1024>::new(&mut output).unwrap();let mut arm=false;
 for _ in 0..100{if frame.stage()==PagedTextStage::Copying&&frame.phase_progress()==(2,Some(2)){break}let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut arm,|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();assert!(turn.result.is_ok())}
 assert_eq!(frame.stage(),PagedTextStage::Copying);assert_eq!(frame.phase_progress(),(2,Some(2)));arm=true;let g=grant(frame.demand(original.borrow()).unwrap());assert_eq!(g.maximum_copy_bytes,0);let interrupted=catch_unwind(AssertUnwindSafe(||physical.with_loan(&mut arm,|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap()));assert!(interrupted.is_err());assert_eq!(frame.stage(),PagedTextStage::Settling);assert_eq!(original.borrow().get().finishes.load(std::sync::atomic::Ordering::SeqCst),1);assert_eq!(frame.demand(original.borrow()).unwrap().copy_bytes,0);
 let turn=physical.with_loan(&mut arm,|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();assert!(turn.result.is_ok());assert_eq!(original.borrow().get().finishes.load(std::sync::atomic::Ordering::SeqCst),1);assert_eq!(frame.demand(original.borrow()).unwrap().copy_bytes,std::mem::size_of::<PagedText<1024>>());
 let mut completed=false;for _ in 0..100{let g=grant(frame.demand(original.borrow()).unwrap());let turn=physical.with_loan(&mut arm,|loan|work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|Ok::<_,ValueError>(frame.advance(original.borrow(),control,g)))))).unwrap().unwrap().unwrap();match turn.result{Ok(PagedTextReadStep::Complete(_))=>{completed=true;break},Ok(PagedTextReadStep::Progress(_)|PagedTextReadStep::Pending)=>{},_=>panic!("publication refused")}}assert!(completed);rig::close_decode_frame(&mut frame);drop(frame);let text=output.take().unwrap();assert_eq!(text.borrow().unwrap().chars().collect::<String>(),"ok");close_text(text);drop(control);assert!(!work.has_live_actions());assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());close_source(original);
}


struct FinalExtentChange{body:String,reads:std::sync::atomic::AtomicUsize,finishes:std::sync::atomic::AtomicUsize}
impl value::paged_text::TextReadSource for FinalExtentChange{
 fn byte_len(&self)->usize{if self.reads.load(std::sync::atomic::Ordering::SeqCst)>=self.body.len()*2{self.body.len()+1}else{self.body.len()}}
 fn byte_at(&self,index:usize)->Option<u8>{self.reads.fetch_add(1,std::sync::atomic::Ordering::SeqCst);self.body.as_bytes().get(index).copied()}
 fn finish_recorded(&self,mut reply:value::paged_text::TextSourceFinishReply<'_,'_>)->value::paged_text::TextSourceFinishComplete{let _=reply.performed(1).expect("original Work receipt must accept the source report");self.finishes.fetch_add(1,std::sync::atomic::Ordering::SeqCst);reply.complete(Ok(()),1)}
}
impl RetireOwned for FinalExtentChange{
 type RetirementCursor=<(String,usize,usize) as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{(self.body,self.reads.into_inner(),self.finishes.into_inner()).retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{<(String,usize,usize) as RetireOwned>::controlled_retirement_supported()}
}
#[test]
fn original_final_extent_change_is_refused_before_encoder_finish(){
 use value::paged_text::PagedTextEncodeProducer;use rig::encoder::{EncodeBank,Stock,projection_work,drive_one_result,take_frame_reply,return_loan,close_producer,retire_published_failure};
 let original=source(FinalExtentChange{body:"abc".to_owned(),reads:Default::default(),finishes:Default::default()});let stock=Stock::for_outputs::<1024>(&[(3,projection_work(original.borrow()).unwrap())],4096,64).unwrap();let mut bank=EncodeBank::new(stock,false);let mut output=None;
 bank.run(4096,64,|turn,control|{let mut producer=PagedTextEncodeProducer::<_,1024>::new(turn.witness(),original.borrow(),&mut output,1024,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;let mut refused=false;for _ in 0..1000{if !drive_one_result(&mut frame,control,turn)?.1{refused=true;break}}assert!(refused);assert_eq!(frame.producer().unwrap().retained_len(),3);assert_eq!(original.borrow().get().finishes.load(std::sync::atomic::Ordering::SeqCst),0);let allocated=frame.producer().unwrap().retained_capacity();let before=control.owned_bytes();assert_eq!(before,allocated);let mut published=None;take_frame_reply(&mut frame,control,turn,&mut published)?;return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;assert!(producer.terminal_is_empty());drop(producer);retire_published_failure(&mut published,control,turn)?;Ok(())}).unwrap();let paid=bank.finish();assert_eq!(paid.submitted.retained_capacity_bytes,paid.submitted.released_bytes);assert!(output.is_none());close_source(original);
}

#[test]
fn original_final_extent_change_is_refused_before_decoder_finish(){
 let original=source(FinalExtentChange{body:"abc".to_owned(),reads:Default::default(),finishes:Default::default()});let mut physical=OriginalConstructionAuthority::new(Physical{interrupt:false,submitted:Default::default()});let mut work=OriginalWorkAuthority::new(Work);let mut allow=|_|true;let mut control=value::NativeDecodeControl::new(4096,&mut allow);let mut output=None;let mut frame=value::paged_text::PagedTextDecodeFrame::<1024>::new(&mut output).unwrap();let mut refused=false;
 for _ in 0..1000{let turn=decode_turn(&mut frame,&original,&mut control,&mut physical,&mut work);match turn.result{Err(failure)=>{close_owned_failure(failure);refused=true;break},Ok(PagedTextReadStep::Progress(_)|PagedTextReadStep::Pending)=>{},_=>panic!("changed final extent was published")}}assert!(refused);assert_eq!(frame.retained_len(),3);assert_eq!(original.borrow().get().finishes.load(std::sync::atomic::Ordering::SeqCst),0);let allocated=frame.retained_capacity();let(mut born,mut released)=(0,0);let net=rig::close_decode_frame_observed(&mut frame,|progress,_|{born+=progress.retained_capacity_bytes;released+=progress.released_bytes;});assert_eq!(net,allocated);assert_eq!(released,allocated+born);drop(frame);assert!(output.is_none());drop(control);assert!(physical.into_recipient().is_ok());assert!(work.into_recipient().is_ok());close_source(original);
}
