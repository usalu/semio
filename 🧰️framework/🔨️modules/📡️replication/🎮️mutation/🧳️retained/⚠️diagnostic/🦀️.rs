//! ⚠️ Semantic refusal rows are constructed from fixed diagnostic declarations and genuine target sources.
use super::{RetainedFieldPreparation,retained_mutation_demand_fits};
use super::super::{MutationMessage,MutationMessageLedgerRetirement,MUTATION_MESSAGE_ENTRY_BYTES};
use semio_framework_diagnostic::{Severity,FaultCode};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{size_of,ManuallyDrop};

/// 🧾️ Fixed diagnostic identity precedes inverse creation and any unpublished projection write.
#[derive(Clone,Copy)]
pub struct RetainedDiagnosticSpec{pub level:Severity,pub code:&'static str,pub message:&'static str}

/// 🏷️ Diagnostic addresses borrow original fields or finite declared literal targets.
#[derive(Clone,Copy)]
pub enum RetainedDiagnosticTarget<'a>{Original(RetainedCloneRef<'a,String>),Literal(&'static str)}

/// 🧳️ Every original target, literal allocation and final row keeps typed closeable custody.
pub struct RetainedDiagnosticPreparation<const N:usize>{
 targets:[RetainedFieldPreparation<String>;N],target_kinds:[u8;N],
 address:ManuallyDrop<Option<Vec<String>>>,
 code:ManuallyDrop<Option<String>>,
 text:ManuallyDrop<Option<String>>,
 rows:ManuallyDrop<Option<Vec<MutationMessage>>>,
 ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,
 retirement:RetainedCloneClose,
 literal:ManuallyDrop<Option<String>>,literal_offset:usize,literal_source:Option<&'static str>,
 spec:Option<RetainedDiagnosticSpec>,
 phase:usize,
 closing:bool,
 transferred:bool,
}

impl<const N:usize> Default for RetainedDiagnosticPreparation<N>{fn default()->Self{Self{targets:std::array::from_fn(|_|Default::default()),target_kinds:[0;N],address:ManuallyDrop::new(None),code:ManuallyDrop::new(None),text:ManuallyDrop::new(None),rows:ManuallyDrop::new(None),ledger:ManuallyDrop::new(None),retirement:Default::default(),literal:ManuallyDrop::new(None),literal_offset:0,literal_source:None,spec:None,phase:0,closing:false,transferred:false}}}

impl<const N:usize> RetainedDiagnosticPreparation<N>{
 /// 📐️ Quotes one fixed diagnostic declaration capture before its text or target is inspected.
 pub fn start_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<RetainedDiagnosticSpec>()+size_of::<usize>(),depth:1,..Default::default()}}
 /// 🎟️ Captures only a bounded immutable diagnostic declaration under the original grant.
 pub fn start(&mut self,spec:RetainedDiagnosticSpec,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.spec.is_some()||self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic already started or closing"));}
  let demand=Self::start_demand();if !retained_mutation_demand_fits(demand,grant){return Ok(Default::default());}
  if spec.code.len()>MUTATION_MESSAGE_ENTRY_BYTES||spec.message.len()>MUTATION_MESSAGE_ENTRY_BYTES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"semantic diagnostic declaration exceeds its owned row bound"));}
  self.spec=Some(spec);Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
 }
 /// 📏️ Quotes one target field, literal, backing allocation or original ledger transfer.
 pub fn demand(&self,target:[RetainedCloneRef<'_,String>;N],body:usize)->Result<RetirementDemand,ValueError>{self.target_demand(target.map(RetainedDiagnosticTarget::Original),body)}
 /// 📏️ Declared literal targets use the same original address and diagnostic row custody.
 pub fn target_demand(&self,target:[RetainedDiagnosticTarget<'_>;N],body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.transferred{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic not available for preparation"));}
  let spec=self.spec.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic declaration missing"))?;
  if self.address.is_none()&&self.phase==0{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<String>>(),capacity_bytes:N.checked_mul(size_of::<String>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"semantic target layout overflow"))?,depth:1,..Default::default()});}
  if self.phase<N{
   let kind=match target[self.phase]{RetainedDiagnosticTarget::Original(_)=>1,RetainedDiagnosticTarget::Literal(_)=>2};if self.target_kinds[self.phase]!=0&&self.target_kinds[self.phase]!=kind{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic target authority kind changed"));}
   if let RetainedDiagnosticTarget::Literal(literal)=target[self.phase]{
    if self.literal_source.is_some_and(|source|!std::ptr::eq(source,literal)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic literal target authority changed"));}
    if literal.len()>MUTATION_MESSAGE_ENTRY_BYTES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"semantic literal target exceeds row bound"));}
    let fixed=size_of::<String>()*2+size_of::<usize>()+size_of::<Option<&'static str>>()+size_of::<u8>();return Ok(if self.literal.is_none(){RetirementDemand{copy_bytes:fixed,capacity_bytes:literal.len(),depth:1,..Default::default()}}else{let minimum=literal[self.literal_offset..].chars().next().map_or(0,char::len_utf8);let count=literal.len().saturating_sub(self.literal_offset).min(body.min(4096).max(minimum));let mut end=self.literal_offset+count;while end>self.literal_offset&&!literal.is_char_boundary(end){end-=1;}RetirementDemand{copy_bytes:fixed+end-self.literal_offset,depth:1,..Default::default()}});
   }
   if self.literal_source.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic literal target cannot switch to an original"));}
   let RetainedDiagnosticTarget::Original(source)=target[self.phase]else{unreachable!()};let field=&self.targets[self.phase];
   if field.value().is_some(){let mut demand=field.exchange_demand()?;demand.copy_bytes+=size_of::<String>()+size_of::<u8>();return Ok(demand);}
   let mut d=field.demand(source,body)?;d.copy_bytes+=1;return Ok(d);
  }
  if self.phase==N{return Ok(RetirementDemand{copy_bytes:spec.code.len()+size_of::<String>(),capacity_bytes:spec.code.len(),depth:1,..Default::default()});}
  if self.phase==N+1{return Ok(RetirementDemand{copy_bytes:spec.message.len()+size_of::<String>(),capacity_bytes:spec.message.len(),depth:1,..Default::default()});}
  if self.phase==N+2{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>(),capacity_bytes:size_of::<MutationMessage>(),depth:1,..Default::default()});}
  if self.phase==N+3{return Ok(RetirementDemand{copy_bytes:size_of::<MutationMessage>()+size_of::<MutationMessageLedgerRetirement>(),depth:1,..Default::default()});}
  Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>(),depth:1,..Default::default()})
 }
 /// ▶️ Builds only the admitted next original child and retains custody before validating actual receipts.
 pub fn advance(&mut self,target:[RetainedCloneRef<'_,String>;N],body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.advance_targets(target.map(RetainedDiagnosticTarget::Original),body,grant)}
 /// ▶️ Literal and original address fragments share one bounded publication owner.
 pub fn advance_targets(&mut self,target:[RetainedDiagnosticTarget<'_>;N],body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.target_demand(target,body)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if self.address.is_none()&&self.phase==0{*self.address=Some(Vec::with_capacity(N));progress.retained_capacity_bytes=self.address.as_ref().unwrap().capacity()*size_of::<String>();return self.receipt(progress,grant,false);}
  if self.phase<N{
   self.target_kinds[self.phase]=match target[self.phase]{RetainedDiagnosticTarget::Original(_)=>1,RetainedDiagnosticTarget::Literal(_)=>2};
   if let RetainedDiagnosticTarget::Literal(literal)=target[self.phase]{
    if self.literal.is_none(){let mut value=String::new();value.try_reserve_exact(literal.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"semantic literal target reservation failed"))?;progress.retained_capacity_bytes=value.capacity();*self.literal=Some(value);self.literal_source=Some(literal);return self.receipt(progress,grant,false);}
    let fixed=size_of::<String>()*2+size_of::<usize>()+size_of::<Option<&'static str>>()+size_of::<u8>();let count=demand.copy_bytes-fixed;
    if self.literal_offset<literal.len(){if count==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let end=self.literal_offset+count;self.literal.as_mut().unwrap().push_str(&literal[self.literal_offset..end]);self.literal_offset=end;}else{self.address.as_mut().unwrap().push(self.literal.take().unwrap());self.literal_offset=0;self.literal_source=None;self.phase+=1;}return self.receipt(progress,grant,false);
   }
   let RetainedDiagnosticTarget::Original(source)=target[self.phase]else{unreachable!()};let field=&mut self.targets[self.phase];
   if field.value().is_some(){let(value,mut receipt)=field.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<String>()-1,..grant})?;self.address.as_mut().expect("admitted semantic target backing").push(value.expect("admitted original semantic target"));receipt.copied_bytes+=size_of::<String>()+1;self.phase+=1;return self.receipt(receipt,grant,false);}
   let step=field.advance(source,body,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant})?;let mut p=step.progress();p.copied_bytes+=1;p.copied_items=p.copied_items.max(1);return Ok(RetainedCloneStep::Progress(p));
  }
  let spec=self.spec.expect("started diagnostic");
  if self.phase==N{let mut code=String::with_capacity(spec.code.len());code.push_str(spec.code);*self.code=Some(code);progress.retained_capacity_bytes=self.code.as_ref().unwrap().capacity();self.phase+=1;return self.receipt(progress,grant,false);}
  if self.phase==N+1{let mut text=String::with_capacity(spec.message.len());text.push_str(spec.message);*self.text=Some(text);progress.retained_capacity_bytes=self.text.as_ref().unwrap().capacity();self.phase+=1;return self.receipt(progress,grant,false);}
  if self.phase==N+2{*self.rows=Some(Vec::with_capacity(1));progress.retained_capacity_bytes=self.rows.as_ref().unwrap().capacity()*size_of::<MutationMessage>();self.phase+=1;return self.receipt(progress,grant,false);}
  if self.phase==N+3{
   self.rows.as_mut().expect("admitted diagnostic row slot").push(MutationMessage{level:spec.level,code:FaultCode(self.code.take().expect("owned diagnostic code")),message:self.text.take().expect("owned diagnostic text"),target:self.address.take().expect("owned original target"),op_index:None});
   *self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),self.rows.take().expect("original message rows")));self.phase+=1;return self.receipt(progress,grant,true);
  }
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 /// 📤️ Transfers original semantic diagnostics once after the whole constructor receipt is admitted.
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<MutationMessage>,RetainedCloneProgress)>,ValueError>{
  if self.phase!=N+4||self.closing||self.transferred{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostics not ready for original transfer"));}
  let result=self.ledger.as_mut().expect("original diagnostic ledger").take_messages_controlled(grant)?;if result.is_some(){self.transferred=true;}Ok(result)
 }
 /// 🛑️ Starts irreversible cleanup of every prepared literal and original target alias.
 pub fn begin_close(&mut self){self.closing=true;for target in &mut self.targets{target.begin_close();}}
 /// ♻️ Quotes one typed target, partial text or original ledger child without an ordinary drop.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(target)=self.targets.iter().find(|target|!target.terminal_is_empty()){return target.close_demand(body);}
  if !self.retirement.is_empty(){return Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?});}
  if self.literal.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body);}
  if self.address.is_some(){return Ok(self.retirement.next_owner_demands::<Vec<String>>(true,&None,body)?);}
  if self.code.is_some()||self.text.is_some(){return Ok(self.retirement.next_owner_demands::<String>(true,&None,body)?);}
  if self.rows.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<MutationMessageLedgerRetirement>()+size_of::<Vec<MutationMessage>>(),depth:1,..Default::default()});}
  if let Some(ledger)=self.ledger.as_ref(){return Ok(RetirementDemand{copy_bytes:ledger.next_copy_byte_demand(),capacity_bytes:ledger.next_capacity_byte_demand(body)?,release_bytes:ledger.next_release_byte_demand()?,depth:ledger.next_depth_demand()?});}
  Ok(Default::default())
 }
 /// 🪜️ Closes partial construction and original diagnostic custody on their exact admitted frontier.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic closure not started"));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(target)=self.targets.iter_mut().find(|target|!target.terminal_is_empty()){return target.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
  if let Some(step)=self.retirement.begin_granted(&mut self.literal,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(step)=self.retirement.begin_granted(&mut self.address,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(step)=self.retirement.begin_granted(&mut self.code,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(step)=self.retirement.begin_granted(&mut self.text,grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
  if let Some(rows)=self.rows.take(){*self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),rows));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  if let Some(ledger)=self.ledger.as_mut(){let step=ledger.close_step(grant)?;if ledger.terminal_is_empty(){*self.ledger=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())});}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 /// 🫙️ No diagnostic row, partial allocation or original target alias remains.
 pub fn terminal_is_empty(&self)->bool{self.targets.iter().all(RetainedFieldPreparation::terminal_is_empty)&&self.literal.is_none()&&self.address.is_none()&&self.code.is_none()&&self.text.is_none()&&self.rows.is_none()&&self.ledger.is_none()&&self.retirement.is_empty()}
 fn receipt(&self,progress:RetainedCloneProgress,grant:RetainedCloneGrant,complete:bool)->Result<RetainedCloneStep,ValueError>{if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic actual construction exceeded original admission").with_retained_progress(progress));}Ok(if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})}
}
impl<const N:usize> Drop for RetainedDiagnosticPreparation<N>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"semantic diagnostic requires complete original typed closure");if empty{unsafe{ManuallyDrop::drop(&mut self.literal);ManuallyDrop::drop(&mut self.address);ManuallyDrop::drop(&mut self.code);ManuallyDrop::drop(&mut self.text);ManuallyDrop::drop(&mut self.rows);ManuallyDrop::drop(&mut self.ledger);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;