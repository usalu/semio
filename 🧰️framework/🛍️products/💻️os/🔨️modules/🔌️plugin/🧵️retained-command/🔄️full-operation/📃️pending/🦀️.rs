//! 📃️ Pending page custody freezes its accepted intent and exact token until funded handoff or physical closure.
use super::result_page_wire::{PublicationWire,write_publication_raw_into};
use semio_framework_actor::typed_operation_page::{TypedOperationResultToken,TypedOperationResultLane,TypedOperationResultPage,TYPED_OPERATION_RESULT_PAGE_BYTES,backing::OriginalResultPageBacking};
use semio_framework_pack_json::JsonBorrowedSinkCursor;
use semio_framework_value::{NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::NativeDecodeControl;
use semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl;
use semio_framework_value::native_encoding::{NativeEncodeProgress,NativeEncodeAllocationPort};
trait PageEncodingReceiver{fn encode<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(usize,&mut dyn FnMut(NativeEncodeProgress)->bool,&mut NativeEncodeAllocationPort<'_>)->Result<T,E>)->Result<T,E>;}
impl PageEncodingReceiver for NativeDecodeControl<'_>{fn encode<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(usize,&mut dyn FnMut(NativeEncodeProgress)->bool,&mut NativeEncodeAllocationPort<'_>)->Result<T,E>)->Result<T,E>{self.with_encoding_receiver(operation)}}
impl PageEncodingReceiver for NativeRetirementDecodeControl<'_, '_>{fn encode<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(usize,&mut dyn FnMut(NativeEncodeProgress)->bool,&mut NativeEncodeAllocationPort<'_>)->Result<T,E>)->Result<T,E>{self.with_encoding_receiver(operation)}}
use semio_framework::kernel::UiDirtyScope;
use std::mem::size_of;
/// 🪪️ Already accepted semantic results retain only scalar intent, never borrowed producer pointers.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PublicationPageIntent{
 LaneReceipt{lane:TypedOperationResultLane,generation_before:u64,generation_after:u64},
 Accepted{lane:TypedOperationResultLane,count:u64},
 Ui,Download,Raw{lane:TypedOperationResultLane,reason:PublicationRawReason},
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PublicationRawReason{Complete,Cancelled,Fault,Empty,AdmissionCancelled}
/// 🫳️ The parent borrows its unchanged retained producer only during one framing action.
pub enum PublicationPageSource<'a>{None,Ui(&'a UiDirtyScope),Download{filename:&'a str,mime:&'a str,encoding:Option<&'a str>,bytes:u64},Raw{reason:PublicationRawReason,bytes:&'a[u8]}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PublicationPagePlan{pub token:TypedOperationResultToken,pub intent:PublicationPageIntent}
/// 🧾️ Complete refusal keeps both primary and forwarding ownership evidence without hiding the accepted prefix.
#[derive(semio_framework_value::RetireOwned)]
pub struct PublicationPageWriteFailure{pub primary:Option<ValueError>,pub forwarding:Option<ValueError>,pub context:Option<ValueError>}
pub struct PublicationPageWrite{pub written_bytes:usize,pub progress:RetainedCloneProgress,pub work:u64,pub result:Result<bool,PublicationPageWriteFailure>}
/// 🪙️ Backing, traversal and accepted intent remain in one explicit retained owner across every denied phase.
pub struct PendingPublicationPage{
 plan:Option<PublicationPagePlan>,backing:OriginalResultPageBacking,cursor:JsonBorrowedSinkCursor,
 written:usize,framed:bool,refused:bool,
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"pending original page lost its accepted intent or producer selector")}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn metadata()->usize{size_of::<usize>()+2*size_of::<bool>()}
impl PublicationPageIntent{
 pub fn lane(self)->TypedOperationResultLane{match self{Self::LaneReceipt{lane,..}|Self::Accepted{lane,..}|Self::Raw{lane,..}=>lane,Self::Ui=>TypedOperationResultLane::Ui,Self::Download=>TypedOperationResultLane::Download}}
 fn wire<'a>(self,source:&'a PublicationPageSource<'a>)->Result<PublicationWire<'a>,ValueError>{match(self,source){
  (Self::LaneReceipt{generation_before,generation_after,..},PublicationPageSource::None)=>Ok(PublicationWire::LaneReceipt{generation_before,generation_after}),
  (Self::Accepted{count,..},PublicationPageSource::None)=>Ok(PublicationWire::Accepted(count)),
  (Self::Ui,PublicationPageSource::Ui(scope))=>Ok(PublicationWire::Ui(scope)),
  (Self::Download,PublicationPageSource::Download{filename,mime,encoding,bytes})=>Ok(PublicationWire::Download{filename,mime,encoding:*encoding,bytes:*bytes}),
  _=>Err(invalid()),
 }}
}
impl PendingPublicationPage{
 pub fn capture_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),depth:1,..Default::default()}}
 /// 🌱️ Captures the final accepted token after the producer has committed its semantic generation.
 pub fn capture(plan:PublicationPagePlan,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{
  let d=Self::capture_demands();if !fits(d,grant){return RetainedWorkOutcome{work:0,result:Ok(None)}}
  let cursor=match JsonBorrowedSinkCursor::empty(3){Ok(cursor)=>cursor,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error)}};
  RetainedWorkOutcome{work:1,result:Ok(Some((Self{plan:Some(plan),backing:OriginalResultPageBacking::empty(),cursor,written:0,framed:false,refused:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 }
 pub fn plan(&self)->Option<PublicationPagePlan>{self.plan}
 pub fn written_bytes(&self)->usize{self.written}
 pub fn is_framed(&self)->bool{self.framed}
 pub fn write_refused(&self)->bool{self.refused}
 pub fn ready_to_handoff(&self)->bool{self.framed&&!self.refused&&self.cursor.terminal_is_empty()}
 pub fn terminal_is_empty(&self)->bool{self.plan.is_none()&&self.backing.terminal_is_empty()&&self.cursor.terminal_is_empty()}
 pub fn backing_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.plan.is_none(){return Err(invalid())}
  if self.backing.terminal_is_empty(){Ok(OriginalResultPageBacking::birth_demands())}else{self.backing.initialization_demands(body)}
 }
 pub fn advance_backing(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  if self.plan.is_none(){return RetainedWorkOutcome{work:0,result:Err(invalid())}}
  if self.backing.terminal_is_empty(){self.backing.birth_step(grant)}else{self.backing.initialize_step(grant)}
 }
 pub fn backing_ready(&self)->bool{self.backing.is_ready()}
 pub fn write_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let plan=self.plan.ok_or_else(invalid)?;if !self.backing.is_ready()||self.framed||self.refused{return Err(invalid())}
  let mut d=match plan.intent{
   PublicationPageIntent::Raw{..}=>RetirementDemand{copy_bytes:64,depth:1,..Default::default()},
   _=>self.cursor.next_demands(TYPED_OPERATION_RESULT_PAGE_BYTES-self.written,body.saturating_sub(metadata()))?,
  };d.copy_bytes+=metadata();Ok(d)
 }
 /// ✍️ Bridges only the actual original receiving authority; copied-items and CPU work remain independent.
 pub fn write_step(&mut self,source:&PublicationPageSource<'_>,grant:RetainedCloneGrant,maximum_work:u64,decode:&mut NativeDecodeControl<'_>)->PublicationPageWrite{self.write_encoded(source,grant,maximum_work,decode)}
 /// ♻️ Accepted custody borrows the same original receiver's retirement authorization after cancellation.
 pub fn write_retirement_step(&mut self,source:&PublicationPageSource<'_>,grant:RetainedCloneGrant,maximum_work:u64,decode:&mut NativeRetirementDecodeControl<'_, '_>)->PublicationPageWrite{self.write_encoded(source,grant,maximum_work,decode)}
 fn write_encoded<R:PageEncodingReceiver>(&mut self,source:&PublicationPageSource<'_>,grant:RetainedCloneGrant,maximum_work:u64,decode:&mut R)->PublicationPageWrite{
  let before=self.written;let was_framed=self.framed;let mut progress=RetainedCloneProgress::default();let mut work=0;let mut primary=None;
  if maximum_work==0||grant.maximum_items==0{return PublicationPageWrite{written_bytes:0,progress,work,result:Ok(false)}}
  let demand=match self.write_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return PublicationPageWrite{written_bytes:0,progress,work:1,result:Err(PublicationPageWriteFailure{primary:Some(error),forwarding:None,context:None})}};
  if maximum_work==0||!fits(demand,grant){return PublicationPageWrite{written_bytes:0,progress,work,result:Ok(false)}}
  if self.written==TYPED_OPERATION_RESULT_PAGE_BYTES&&self.cursor.pending_bytes()!=0{self.refused=true;return PublicationPageWrite{written_bytes:0,progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()},work:1,result:Err(PublicationPageWriteFailure{primary:Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"encoded result exceeds fixed original page backing")),forwarding:None,context:None})}}
  let intent=self.plan.unwrap().intent;
  let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-metadata(),..grant};
  let bridge=decode.encode(|remaining,observer,allocation|{
   let mut control=NativeEncodeControl::new_forwarded(remaining,observer,allocation);
   match(intent,source){
    (PublicationPageIntent::Raw{reason,..},PublicationPageSource::Raw{reason:actual,bytes})if reason==*actual=>{
     work=1;control.checkpoint()?;
     if bytes.len()>TYPED_OPERATION_RESULT_PAGE_BYTES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"raw result exceeds fixed page backing"))}
     let count=bytes.len().checked_sub(self.written).ok_or_else(invalid)?.min(64).min(child.maximum_copy_bytes);
     let output=&mut self.backing.initialized_bytes().ok_or_else(invalid)?[self.written..self.written+count];
     let write=write_publication_raw_into(&bytes[self.written..self.written+count],output,&mut control);
     self.written+=write.written;let framed=write.result.is_ok()&&self.written==bytes.len();let changed=self.written!=before||self.framed!=framed;work=write.work.max(u64::from(changed));self.framed=framed;progress=RetainedCloneProgress{copied_items:usize::from(changed),copied_bytes:write.written+usize::from(self.written!=before)*size_of::<usize>()+usize::from(self.framed!=was_framed)*size_of::<bool>(),..Default::default()};primary=write.result.err();
    },
    (PublicationPageIntent::Raw{..},_)=>{work=1;primary=Some(invalid());},
    _=>{
     work=1;let wire=match intent.wire(source){Ok(wire)=>wire,Err(error)=>{primary=Some(error);return Ok(())}};let output=&mut self.backing.initialized_bytes().ok_or_else(invalid)?[self.written..];
     let turn=self.cursor.step(&wire,output,child,maximum_work,&mut control);work=turn.work;
     match turn.result{Ok(step)=>{progress=step.progress;if step.written_bytes!=0{self.written+=step.written_bytes;}if step.complete{self.framed=true;}},Err(error)=>{progress=error.retained_progress();self.written=self.cursor.written_bytes();primary=Some(error);}}
     progress.copied_bytes+=usize::from(self.written!=before)*size_of::<usize>()+usize::from(self.framed!=was_framed)*size_of::<bool>();
    }
   }
   Ok::<(),ValueError>(())
  });
  let forwarding=bridge.err();
  if primary.is_some()||forwarding.is_some(){self.refused=true;progress.copied_items=1;progress.copied_bytes+=size_of::<bool>();if work==0{work=1;}}
  PublicationPageWrite{written_bytes:self.written-before,progress,work,result:if primary.is_some()||forwarding.is_some(){Err(PublicationPageWriteFailure{primary,forwarding,context:None})}else{Ok(self.framed)}}
 }
 /// ♻️ A paid original retirement turn copies raw fixed framing without constructing another encoding authority.
 pub fn write_raw_step(&mut self,source:&PublicationPageSource<'_>,grant:RetainedCloneGrant)->PublicationPageWrite{
  let d=match self.write_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return PublicationPageWrite{written_bytes:0,progress:Default::default(),work:0,result:Err(PublicationPageWriteFailure{primary:Some(error),forwarding:None,context:None})}};
  if !fits(d,grant){return PublicationPageWrite{written_bytes:0,progress:Default::default(),work:0,result:Ok(false)}}
  let checked=(||{let PublicationPageIntent::Raw{reason,..}=self.plan.ok_or_else(invalid)?.intent else{return Err(invalid())};let PublicationPageSource::Raw{reason:actual,bytes}=source else{return Err(invalid())};if reason!=*actual||bytes.len()>TYPED_OPERATION_RESULT_PAGE_BYTES{return Err(invalid())}Ok(*bytes)})();
  let bytes=match checked{Ok(bytes)=>bytes,Err(error)=>{self.refused=true;return PublicationPageWrite{written_bytes:0,progress:RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()},work:1,result:Err(PublicationPageWriteFailure{primary:Some(error),forwarding:None,context:None})}}};
  let count=bytes.len().saturating_sub(self.written).min(64).min(grant.maximum_copy_bytes-metadata());
  let before=self.framed;self.backing.initialized_bytes().expect("quoted initialized raw backing")[self.written..self.written+count].copy_from_slice(&bytes[self.written..self.written+count]);self.written+=count;self.framed=self.written==bytes.len();
  PublicationPageWrite{written_bytes:count,progress:RetainedCloneProgress{copied_items:1,copied_bytes:count+usize::from(count!=0)*size_of::<usize>()+usize::from(before!=self.framed)*size_of::<bool>(),..Default::default()},work:1,result:Ok(self.framed)}
 }
 pub fn traversal_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.framed{return Err(invalid())}self.cursor.close_demands(body)}
 pub fn close_traversal(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{if !self.framed{return RetainedWorkOutcome{work:0,result:Err(invalid())}}self.cursor.close_step(grant)}
 pub fn handoff_demands(&self)->Result<RetirementDemand,ValueError>{
  if !self.ready_to_handoff(){return Err(invalid())}
  let mut d=self.backing.page_demands()?;d.copy_bytes+=size_of::<Option<PublicationPagePlan>>()+metadata();Ok(d)
 }
 /// 📤️ Uses the frozen token rather than any later document generation or cancellation state.
 pub fn take_page(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(TypedOperationResultPage,RetainedCloneProgress)>>{
  let d=match self.handoff_demands(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error)}};if !fits(d,grant){return RetainedWorkOutcome{work:0,result:Ok(None)}}
  let plan=self.plan.unwrap();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<PublicationPagePlan>>()-metadata(),..grant};
  let turn=self.backing.take_page(plan.token,plan.intent.lane(),self.written,child);
  let result=turn.result.map(|page|page.map(|(page,mut p)|{self.plan=None;self.written=0;self.framed=false;self.refused=false;p.copied_bytes+=size_of::<Option<PublicationPagePlan>>()+metadata();(page,p)}));
  RetainedWorkOutcome{work:turn.work,result}
 }
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.cursor.terminal_is_empty(){return self.cursor.close_demands(body)}
  if !self.backing.terminal_is_empty(){return self.backing.close_demands(body)}
  Ok(if self.plan.is_some(){RetirementDemand{copy_bytes:size_of::<Option<PublicationPagePlan>>()+metadata(),depth:1,..Default::default()}}else{Default::default()})
 }
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
  if !self.cursor.terminal_is_empty(){return self.cursor.close_step(grant)}
  if !self.backing.terminal_is_empty(){return self.backing.close_step(grant)}
  let d=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error)}};
  if self.plan.is_none(){return RetainedWorkOutcome{work:0,result:Ok(RetainedCloneStep::Complete(Default::default()))}}
  if !fits(d,grant){return RetainedWorkOutcome{work:0,result:Ok(RetainedCloneStep::Progress(Default::default()))}}
  self.plan=None;self.written=0;self.framed=false;self.refused=false;RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 }
}
impl Drop for PendingPublicationPage{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"pending result intent and partial page require funded handoff or closure")}}

