//! ✍️ Original typing edits move canonical net vectors at one funded resident exchange into the actual receiving slot.
use std::mem::size_of;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure};
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind,SharedUtf8};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::retained_clone::original_slot::RetainedOriginalSlotKey;
use semio_framework_tool_machine::{TypingLedger,TypingState,ToolStep,TYPING_IDLE_MS};
use semio_framework_replication::{ActorId,HybridLogicalTimestamp,TransactionRef};
#[derive(RetireOwned)]
pub struct OriginalTypingEditRequest<M>{pub window:String,pub tool:String,pub actor:ActorId,pub buffer:String,pub leaves:Vec<M>,pub clock:HybridLogicalTimestamp}
#[derive(RetireOwned)]
pub enum OriginalTypingFold<M>{Split,ReplaceIncoming,Net(Vec<M>)}
#[derive(RetireOwned)]
pub struct OriginalTypingEditResult<M>{pub previous:Option<ToolStep<M>>,pub open:bool}
#[derive(RetireOwned)]
struct Metadata{window:String,tool:Option<String>,actor:Option<ActorId>,buffer:Option<String>}
#[derive(RetireOwned)]
struct PreviousMetadata{states:Vec<String>,tool:String,actor:ActorId,buffer:String}
#[derive(RetireOwned)]
struct Owners<M>{request:Option<OriginalTypingEditRequest<M>>,decision:Option<OriginalTypingFold<M>>,key:Option<SharedUtf8>,slot_key:Option<SharedUtf8>,net:Option<Vec<M>>,input_residue:Option<Vec<M>>,states:Vec<String>,buffers:[Vec<u8>;2],transaction:Option<TransactionRef>,old_net:Option<Vec<M>>,metadata:Option<Metadata>,previous_metadata:Option<PreviousMetadata>,empty_net:Option<Vec<M>>}
pub struct OriginalTypingEdit<M:RetireOwned+Send+Sync+'static>{binding:Option<OriginalSourceBinding>,decoder:(u64,u64),ledger:usize,revision:u64,key:Option<RetainedOriginalSlotKey>,phase:u8,length:usize,reuse:bool,needs_fold:bool,closing:bool,received:bool,owners:ControlledRetirement<Owners<M>>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original typing edit changed its owned source")}
fn movement(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn funds(cx:&StepContext<'_>,d:RetirementDemand)->bool{!cx.should_yield()&&cx.retained_grant().maximum_items!=0&&cx.retained_work_can_enter(d)}
fn paid(cx:&mut StepContext<'_>,p:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);Ok(())}
fn text_demand(output:&Vec<u8>,length:usize)->RetirementDemand{if output.capacity()<length{RetirementDemand{capacity_bytes:length,..movement(2*size_of::<Vec<u8>>())}}else if output.len()<length{movement(1)}else{Default::default()}}
fn text(cx:&mut StepContext<'_>,output:&mut Vec<u8>,source:&[u8])->Result<bool,OriginalTurnFailure>{
 if output.capacity()<source.len(){let d=text_demand(output,source.len());if !funds(cx,d){return Ok(false)}output.try_reserve_exact(source.len()).map_err(|_|invalid())?;paid(cx,RetainedCloneProgress{retained_capacity_bytes:output.capacity(),..progress(d)})?;return Ok(false)}
 if output.len()<source.len(){let n=(source.len()-output.len()).min(cx.retained_grant().maximum_copy_bytes);if n==0||!funds(cx,movement(n)){return Ok(false)}let start=output.len();output.extend_from_slice(&source[start..start+n]);paid(cx,progress(movement(n)))?;return Ok(false)}Ok(true)
}
impl<M:RetireOwned+Send+Sync+'static> OriginalTypingEdit<M>{
 pub fn admission_demands(_ledger:&TypingLedger<M>,_request:&OriginalTypingEditRequest<M>)->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..movement(2*size_of::<Self>()+2*size_of::<OriginalTypingEditRequest<M>>()+size_of::<bool>())}}
 pub fn admit(ledger:&mut TypingLedger<M>,request:&mut Option<OriginalTypingEditRequest<M>>,output:&mut Option<Box<Self>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let _=cx.original_source_parent()?;let decoder=cx.native_receiving_identity().ok_or_else(invalid)?;
  if output.is_some()||request.is_none(){return Err(invalid().into())}
  let input=request.as_ref().unwrap();if ledger.original_write_pending()||ledger.retained_read_pending(&input.window){return Ok(false)}
  if !Owners::<M>::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original typing edit has no controlled retirement").into())}
  let d=Self::admission_demands(ledger,input);if !funds(cx,d){return Ok(false)}
  let previous=ledger.open(&input.window);if previous.is_some_and(|row|row.states.as_slice()!=["root","typing"]||row.transaction.tool!=row.tool){return Err(invalid().into())}
  let needs_fold=previous.is_some_and(|row|row.tool==input.tool&&row.buffer==input.buffer&&row.deadline_ms>input.clock.physical_ms);
  let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(std::alloc::Layout::new::<Self>())}.cast::<Self>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"original typing edit allocation refused"))?;
  let owner=Self{binding:Some(OriginalSourceBinding::borrow(cx)),decoder,ledger:ledger as*mut _ as usize,revision:ledger.source_revision(),key:None,phase:0,length:0,reuse:false,needs_fold,closing:false,received:false,owners:ControlledRetirement::retain(Owners{request:request.take(),decision:None,key:None,slot_key:None,net:None,input_residue:None,states:Vec::new(),buffers:std::array::from_fn(|_|Vec::new()),transaction:None,old_net:None,metadata:None,previous_metadata:None,empty_net:None})};
  *output=Some(unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())});ledger.begin_original_write();paid(cx,progress(d))?;Ok(true)
 }
 fn validate(&self,ledger:&TypingLedger<M>,cx:&StepContext<'_>)->Result<(),ValueError>{if self.ledger!=ledger as*const _ as usize||!self.binding.is_some_and(|b|b.matches(cx))||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid())}Ok(())}
 fn validate_source(&self,ledger:&TypingLedger<M>,cx:&StepContext<'_>)->Result<(),ValueError>{self.validate(ledger,cx)?;if self.revision!=ledger.source_revision(){return Err(invalid())}Ok(())}
 pub fn ready(&self)->bool{self.phase==4&&!self.received&&!self.closing}
 pub fn matches_context(&self,ledger:&TypingLedger<M>,cx:&StepContext<'_>)->bool{self.validate_source(ledger,cx).is_ok()}
 pub fn fold_inputs<'a>(&'a self,ledger:&'a TypingLedger<M>,cx:&StepContext<'_>)->Result<Option<(&'a [M],&'a [M])>,OriginalTurnFailure>{self.validate_source(ledger,cx)?;if !self.needs_fold||self.phase!=0{return Ok(None)}let owners=self.owners.original().ok_or_else(invalid)?;let request=owners.request.as_ref().ok_or_else(invalid)?;let previous=ledger.open(&request.window).ok_or_else(invalid)?;Ok(Some((&previous.net,&request.leaves)))}
 pub fn accept_fold(&mut self,decision:&mut Option<OriginalTypingFold<M>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if !self.binding.is_some_and(|b|b.matches(cx))||cx.native_receiving_identity()!=Some(self.decoder)||!self.needs_fold||self.phase!=0||decision.is_none(){return Err(invalid().into())}
  let d=movement(2*size_of::<OriginalTypingFold<M>>()+size_of::<u8>());if !funds(cx,d){return Ok(false)}
  let owners=self.owners.original_mut().ok_or_else(invalid)?;if owners.decision.is_some(){return Err(invalid().into())}owners.decision=decision.take();self.needs_fold=false;paid(cx,progress(d))?;Ok(true)
 }
 pub fn demands(&self,ledger:&TypingLedger<M>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.received{return self.close_demands(body)}
  let owners=self.owners.original().ok_or_else(invalid)?;
  Ok(match self.phase{
   0=>movement(2*size_of::<Vec<M>>()+2*size_of::<OriginalTypingFold<M>>()+3*size_of::<usize>()),
   1=>if owners.key.is_none(){let mut d=SharedUtf8::admission_demand();d.copy_bytes+=2*size_of::<String>()+size_of::<Option<SharedUtf8>>();d}else if owners.slot_key.is_none(){let mut d=SharedUtf8::lease_demand()?;d.copy_bytes+=size_of::<Option<SharedUtf8>>();d}else{let mut d=ledger.original_window_demands(owners.key.as_ref().unwrap().as_str())?;d.copy_bytes+=size_of::<Option<RetainedOriginalSlotKey>>()+size_of::<u64>()+size_of::<u8>();d},
   2=>if !self.reuse&&self.length!=0{if owners.states.capacity()<2{RetirementDemand{capacity_bytes:2*size_of::<String>(),..movement(2*size_of::<Vec<String>>())}}else if owners.states.len()<2{let word=if owners.states.is_empty(){b"root".as_slice()}else{b"typing".as_slice()};let d=text_demand(&owners.buffers[0],word.len());if d!=RetirementDemand::default(){d}else{movement(size_of::<String>())}}else{movement(size_of::<u8>())}}else{movement(size_of::<u8>())},
   3=>if !self.reuse&&self.length!=0&&owners.transaction.is_none(){let request=owners.request.as_ref().unwrap();let d=text_demand(&owners.buffers[1],request.tool.len());if d!=RetirementDemand::default(){d}else{TransactionRef::original_mint_demands(&request.actor,&request.clock,&request.tool)?}}else{movement(size_of::<u8>())},
   4=>Default::default(),
   _=>return Err(invalid())
  })
 }
 pub fn advance(&mut self,ledger:&mut TypingLedger<M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_source(ledger,cx)?;if self.received{return Err(invalid().into())}if self.closing||cx.is_cancelled(){self.close_step(ledger,cx)?;return Ok(false)}if self.needs_fold{return Ok(false)}if self.phase==4{return Ok(true)}
  let d=self.demands(ledger,cx.retained_grant().maximum_copy_bytes)?;if !funds(cx,d){return Ok(false)}
  let owners=self.owners.original_mut().ok_or_else(invalid)?;
  if self.phase==0{let request=owners.request.as_mut().unwrap();self.reuse=match owners.decision.as_ref(){Some(OriginalTypingFold::Split)|None=>false,_=>true};let net=match owners.decision.take(){Some(OriginalTypingFold::Net(net))=>net,Some(OriginalTypingFold::ReplaceIncoming)|Some(OriginalTypingFold::Split)|None=>std::mem::take(&mut request.leaves)};owners.net=Some(net);self.length=owners.net.as_ref().unwrap().len();self.phase=1;paid(cx,progress(d))?;return Ok(false)}
  if self.phase==1{
   if owners.key.is_none(){let overhead=2*size_of::<String>()+size_of::<Option<SharedUtf8>>();let request=owners.request.as_mut().unwrap();let window=std::mem::take(&mut request.window);match SharedUtf8::admit(window,RetainedCloneGrant{maximum_copy_bytes:cx.retained_grant().maximum_copy_bytes-overhead,..cx.retained_grant()}){Ok((key,mut p))=>{owners.key=Some(key);p.copied_bytes+=overhead;paid(cx,p)?},Err((e,window))=>{request.window=window;return Err(e.into())}}return Ok(false)}
   if owners.slot_key.is_none(){let overhead=size_of::<Option<SharedUtf8>>();let(key,mut p)=owners.key.as_ref().unwrap().admit_clone(RetainedCloneGrant{maximum_copy_bytes:cx.retained_grant().maximum_copy_bytes-overhead,..cx.retained_grant()})?;owners.slot_key=Some(key);p.copied_bytes+=overhead;paid(cx,p)?;return Ok(false)}
   let extra=size_of::<Option<RetainedOriginalSlotKey>>()+size_of::<u64>()+size_of::<u8>();let(key,mut p)=ledger.prepare_original_window(&mut owners.slot_key,RetainedCloneGrant{maximum_copy_bytes:cx.retained_grant().maximum_copy_bytes-extra,..cx.retained_grant()})?;if p==RetainedCloneProgress::default(){return Ok(false)}if let Some(key)=key{self.key=Some(key);self.phase=2}self.revision=ledger.source_revision();p.copied_bytes+=extra;paid(cx,p)?;return Ok(false)
  }
  if self.phase==2{
   if !self.reuse&&self.length!=0&&owners.states.len()<2{if owners.states.capacity()<2{owners.states.try_reserve_exact(2).map_err(|_|invalid())?;paid(cx,RetainedCloneProgress{retained_capacity_bytes:owners.states.capacity()*size_of::<String>(),..progress(d)})?;return Ok(false)}let word=if owners.states.is_empty(){b"root".as_slice()}else{b"typing".as_slice()};if !text(cx,&mut owners.buffers[0],word)?{return Ok(false)}owners.states.push(String::from_utf8(std::mem::take(&mut owners.buffers[0])).map_err(|_|invalid())?);paid(cx,progress(d))?;return Ok(false)}self.phase=3;paid(cx,progress(d))?;return Ok(false)
  }
  if self.phase==3{
   if !self.reuse&&self.length!=0&&owners.transaction.is_none(){let request=owners.request.as_ref().unwrap();if !text(cx,&mut owners.buffers[1],request.tool.as_bytes())?{return Ok(false)}let tool=String::from_utf8(std::mem::take(&mut owners.buffers[1])).map_err(|_|invalid())?;match TransactionRef::mint_original(&request.actor,&request.clock,tool,cx.retained_grant()){Ok((tx,p))=>{owners.transaction=Some(tx);paid(cx,p)?},Err((e,tool))=>{owners.buffers[1]=tool.into_bytes();paid(cx,e.retained_progress())?;return Err(e.into())}}return Ok(false)}self.phase=4;paid(cx,progress(d))?;return Ok(false)
  }
  Err(invalid().into())
 }
 pub fn take_result_demands(&self,ledger:&TypingLedger<M>)->Result<RetirementDemand,ValueError>{let mut d=ledger.original_edit_transform_demands::<OriginalTypingEditResult<M>>();d.copy_bytes+=2*size_of::<OriginalTypingEditRequest<M>>()+size_of::<Metadata>()+size_of::<PreviousMetadata>()+3*size_of::<Option<Vec<M>>>()+3*size_of::<bool>();Ok(d)}
 pub fn take_result(&mut self,ledger:&mut TypingLedger<M>,output:&mut Option<OriginalTypingEditResult<M>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate_source(ledger,cx)?;if self.phase!=4||output.is_some()||self.received{return Err(invalid().into())}if cx.is_cancelled(){return Ok(false)}
  let base=ledger.original_edit_transform_demands::<OriginalTypingEditResult<M>>();let d=self.take_result_demands(ledger)?;let extra=d.copy_bytes-base.copy_bytes;
  if !funds(cx,d){return Ok(false)}
  let owners=self.owners.original_mut().ok_or_else(invalid)?;let window=owners.key.as_ref().unwrap().as_str();let reuse=self.reuse;let length=self.length;
  let received=ledger.transform_original_edit(window,self.key.unwrap(),RetainedCloneGrant{maximum_copy_bytes:cx.retained_grant().maximum_copy_bytes-extra,..cx.retained_grant()},|previous|{
   let OriginalTypingEditRequest{window,tool,actor,buffer,leaves,clock}=owners.request.take().unwrap();owners.input_residue=Some(leaves);owners.metadata=Some(Metadata{window,tool:Some(tool),actor:Some(actor),buffer:Some(buffer)});
   let(state,previous)=if reuse{let mut row=previous.unwrap();owners.old_net=Some(std::mem::replace(&mut row.net,owners.net.take().unwrap()));row.deadline_ms=clock.physical_ms.saturating_add(TYPING_IDLE_MS);(Some(row),None)}else{
    let previous=previous.map(|row|{let TypingState{states,tool,actor,buffer,deadline_ms:_,transaction,net}=row;owners.previous_metadata=Some(PreviousMetadata{states,tool,actor,buffer});if net.is_empty(){owners.empty_net=Some(net);ToolStep::Empty(transaction)}else{ToolStep::Committed(transaction,net)}});
    let state=if length==0{None}else{let meta=owners.metadata.as_mut().unwrap();Some(TypingState{states:std::mem::take(&mut owners.states),tool:meta.tool.take().unwrap(),actor:meta.actor.take().unwrap(),buffer:meta.buffer.take().unwrap(),deadline_ms:clock.physical_ms.saturating_add(TYPING_IDLE_MS),transaction:owners.transaction.take().unwrap(),net:owners.net.take().unwrap()})};(state,previous)
   };(state,OriginalTypingEditResult{previous,open:reuse||length!=0})
  })?;
  let Some((result,mut p))=received else{return Ok(false)};*output=Some(result);self.revision=ledger.source_revision();self.received=true;p.copied_bytes+=extra;paid(cx,p)?;Ok(true)
 }
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.owners.terminal_is_empty(){Ok(movement(size_of::<Option<OriginalSourceBinding>>()+2*size_of::<bool>()))}else{self.owners.next_demand(body)}}
 pub fn close_step(&mut self,ledger:&mut TypingLedger<M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.terminal_is_empty(){return Ok(true)}self.validate(ledger,cx)?;if cx.retirement_should_yield(){return Ok(false)}let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;if self.owners.terminal_is_empty(){let g=cx.retained_grant();if g.maximum_items==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(false)}ledger.end_original_write();self.binding=None;self.closing=true;paid(cx,progress(d))?;return Ok(true)}semio_framework_job::close_original_retained_step(cx,d,|g|self.owners.step(g))?;self.closing=true;Ok(false)}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()&&self.owners.terminal_is_empty()}
 pub fn shell_close_demands(&self)->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),..movement(size_of::<Option<Box<Self>>>())}}
}
impl<M:RetireOwned+Send+Sync+'static> Drop for OriginalTypingEdit<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original typing edit requires paid source closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
