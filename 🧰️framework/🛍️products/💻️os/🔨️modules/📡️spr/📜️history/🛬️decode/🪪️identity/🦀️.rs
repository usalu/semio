//! 🪪️ Original History identifiers retain archive ranges or prior typed edit bytes until their funded native String recipient receives custody.
use super::HistoryEdit;
use ::protocol::{codec::RetainedWireFieldDecode,dictionary::RetainedSourceDictionary};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use std::mem::ManuallyDrop;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedHistoryIdDecodeStep{Blocked,Progress,Ready,Closed}
#[derive(Clone,Copy)]
enum OriginalIdInput{Range(usize,usize),Uuid,PriorEdit(usize)}
pub struct RetainedHistoryIdDecode{source_identity:(usize,usize),input:OriginalIdInput,consumed_end:usize,field:ManuallyDrop<Option<RetainedWireFieldDecode>>,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,closing:bool,closed:bool}
impl RetainedHistoryIdDecode{
 fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
 fn scalar(source:&[u8],position:&mut usize,end:usize)->Result<usize,ValueError>{let mut value=0u64;for index in 0..10{let byte=*source.get(*position).filter(|_|*position<end).ok_or_else(||Self::fault("original History identity scalar is truncated"))?;*position+=1;if index==9&&byte>1{return Err(Self::fault("original History identity scalar overflows"))}value|=u64::from(byte&127)<<(index*7);if byte&128==0{return usize::try_from(value).map_err(|_|Self::fault("original History identity exceeds platform extent"))}}Err(Self::fault("original History identity scalar is not bounded"))}
 /// 🎟️ Binds the exact original archive and borrowed prior typed payload without materializing an identifier.
 pub fn admit(source:&[u8],original_range:(usize,usize),dictionary:&RetainedSourceDictionary,original_prior_edits:&[HistoryEdit],grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{
  if grant.maximum_items==0||grant.maximum_depth<2{return Ok(None)}
  source.get(original_range.0..original_range.1).ok_or_else(||Self::fault("original History identity escaped its record"))?;
  let mut position=original_range.0;let tag=*source.get(position).filter(|_|position<original_range.1).ok_or_else(||Self::fault("original History identity tag is absent"))?;position+=1;
  let(input,field)=match tag{
   0=>{let length=Self::scalar(source,&mut position,original_range.1)?;let end=position.checked_add(length).filter(|end|*end<=original_range.1).ok_or_else(||Self::fault("original History identity bytes are truncated"))?;let field=RetainedWireFieldDecode::new(&source[position..end],true);let input=OriginalIdInput::Range(position,end);position=end;(input,field)},
   1|2=>{let index=Self::scalar(source,&mut position,original_range.1)?;let(start,end)=dictionary.resolve_range(source,index)?;let prefix=&source[start..end];if tag==1{(OriginalIdInput::Range(start,end),RetainedWireFieldDecode::new(prefix,true))}else{let uuid_end=position.checked_add(16).filter(|end|*end<=original_range.1).ok_or_else(||Self::fault("original History identity UUID is truncated"))?;let field=RetainedWireFieldDecode::new_prefixed_uuid(source,(start,end),(position,uuid_end))?;position=uuid_end;(OriginalIdInput::Uuid,field)}},
   3=>{let ordinal=Self::scalar(source,&mut position,original_range.1)?;let previous=original_prior_edits.get(ordinal).ok_or_else(||Self::fault("original History identity prior edit is absent"))?;(OriginalIdInput::PriorEdit(ordinal),RetainedWireFieldDecode::new(previous.id.as_bytes(),true))},
   _=>return Err(Self::fault("original History identity tag is unsupported")),
  };
  Ok(Some(Self{source_identity:(source.as_ptr()as usize,source.len()),input,consumed_end:position,field:ManuallyDrop::new(Some(field)),receipt:Some((grant,RetainedCloneProgress{copied_items:1,..Default::default()})),closing:false,closed:false}))
 }
 fn validate_source(&self,source:&[u8])->Result<(),ValueError>{if self.source_identity!=(source.as_ptr()as usize,source.len()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original History identity archive changed"))}Ok(())}
 fn input<'a>(input:OriginalIdInput,source:&'a[u8],original_prior_edits:&'a[HistoryEdit])->Result<&'a[u8],ValueError>{match input{OriginalIdInput::Range(start,end)=>source.get(start..end).ok_or_else(||Self::fault("original History identity range changed")),OriginalIdInput::Uuid=>Ok(source),OriginalIdInput::PriorEdit(ordinal)=>original_prior_edits.get(ordinal).map(|edit|edit.id.as_bytes()).ok_or_else(||Self::fault("original History identity prior edit disappeared"))}}
 fn record(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.receipt=Some((grant,progress));if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original History identity receipt exceeded its original grant").with_retained_progress(progress))}Ok(())}
 fn covers(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_depth>=demand.depth&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes}
 pub fn consumed_end(&self)->usize{self.consumed_end}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 pub fn is_ready(&self)->bool{!self.closing&&!self.closed&&self.receipt.is_none()&&self.field.as_ref().is_some_and(RetainedWireFieldDecode::is_ready)}
 /// 📐️ Quotes the exact next physical field piece and its original parent depth without mutating either owner.
 pub fn demands(&self,source:&[u8],original_prior_edits:&[HistoryEdit],maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
  self.validate_source(source)?;if self.closed||self.receipt.is_some(){return Ok(Default::default())}
  if let Some(field)=self.field.as_ref(){let input=Self::input(self.input,source,original_prior_edits)?;let mut demand=if self.closing{field.close_demands()}else{field.demands(input,maximum_copy_bytes)?};demand.depth=demand.depth.checked_add(1).ok_or_else(||Self::fault("original History identity nested depth overflow"))?;Ok(demand)}else{Ok(RetirementDemand{depth:1,..Default::default()})}
 }
 /// 🧵️ Writes only the next funded original bytes, then removes the already paid empty child under separate authority.
 pub fn step(&mut self,source:&[u8],original_prior_edits:&[HistoryEdit],grant:RetainedCloneGrant)->Result<RetainedHistoryIdDecodeStep,ValueError>{
  self.validate_source(source)?;if self.closed||self.closing||self.receipt.is_some(){return Ok(RetainedHistoryIdDecodeStep::Blocked)}let demand=self.demands(source,original_prior_edits,grant.maximum_copy_bytes)?;if !Self::covers(grant,demand){return Ok(RetainedHistoryIdDecodeStep::Blocked)}
  if self.field.as_ref().is_some_and(RetainedWireFieldDecode::terminal_is_empty){drop(self.field.take());self.closed=true;self.record(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(RetainedHistoryIdDecodeStep::Closed)}
  if self.is_ready(){return Ok(RetainedHistoryIdDecodeStep::Ready)}
  let input=Self::input(self.input,source,original_prior_edits)?;let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let result=self.field.as_mut().ok_or_else(||Self::fault("original History identity field is absent"))?.advance(input,child_grant);if let Some((_,progress))=self.field.as_mut().unwrap().take_receipt(){self.record(grant,progress)?}result?;Ok(if self.receipt.is_some(){RetainedHistoryIdDecodeStep::Progress}else{RetainedHistoryIdDecodeStep::Blocked})
 }
 /// 📤️ Moves the exact native String backing into the caller's real empty original recipient.
 pub fn take_ready_into(&mut self,source:&[u8],original_prior_edits:&[HistoryEdit],recipient:&mut Option<String>,grant:RetainedCloneGrant)->Result<RetainedHistoryIdDecodeStep,ValueError>{
  if !self.is_ready()||recipient.is_some(){return Ok(RetainedHistoryIdDecodeStep::Blocked)}let demand=self.demands(source,original_prior_edits,grant.maximum_copy_bytes)?;if !Self::covers(grant,demand){return Ok(RetainedHistoryIdDecodeStep::Blocked)}let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let result=self.field.as_mut().unwrap().take_text_into(recipient,child_grant);if let Some((_,progress))=self.field.as_mut().unwrap().take_receipt(){self.record(grant,progress)?}result?;Ok(if self.receipt.is_some(){RetainedHistoryIdDecodeStep::Progress}else{RetainedHistoryIdDecodeStep::Blocked})
 }
 pub fn cancel(&mut self){self.closing=true;if let Some(field)=self.field.as_mut(){field.cancel()}}
 /// ♻️ Pays the genuine nested native backing release before the separate parent terminal metadata receipt.
 pub fn close_step(&mut self,source:&[u8],original_prior_edits:&[HistoryEdit],grant:RetainedCloneGrant)->Result<RetainedHistoryIdDecodeStep,ValueError>{
  self.validate_source(source)?;if self.closed{return Ok(RetainedHistoryIdDecodeStep::Closed)}if !self.closing||self.receipt.is_some(){return Ok(RetainedHistoryIdDecodeStep::Blocked)}let demand=self.demands(source,original_prior_edits,grant.maximum_copy_bytes)?;if !Self::covers(grant,demand){return Ok(RetainedHistoryIdDecodeStep::Blocked)}
  if self.field.as_ref().is_some_and(RetainedWireFieldDecode::terminal_is_empty){drop(self.field.take());self.closed=true;self.record(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(RetainedHistoryIdDecodeStep::Closed)}
  let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let result=self.field.as_mut().ok_or_else(||Self::fault("original History identity field disappeared before paid close"))?.close_step(child_grant);if let Some((_,progress))=self.field.as_mut().unwrap().take_receipt(){self.record(grant,progress)?}result?;Ok(if self.receipt.is_some(){RetainedHistoryIdDecodeStep::Progress}else{RetainedHistoryIdDecodeStep::Blocked})
 }
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.field.is_none()&&self.receipt.is_none()}
}
impl Drop for RetainedHistoryIdDecode{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original History identity abandoned native field custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.field)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
