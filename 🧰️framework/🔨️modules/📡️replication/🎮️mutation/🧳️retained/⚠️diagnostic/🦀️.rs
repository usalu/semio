//! ⚠️ Semantic refusal rows are constructed from fixed diagnostic declarations and genuine target sources.
use super::{RetainedFieldPreparation,retained_mutation_demand_fits,settle,settle_exit};
use super::super::{MutationMessage,MutationMessageLedgerRetirement,MUTATION_MESSAGE_ENTRY_BYTES};
use semio_framework_diagnostic::{Severity,FaultCode};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementOutcome,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneBinding,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{size_of,ManuallyDrop};

/// 🧾️ Fixed diagnostic identity precedes inverse creation and any unpublished projection write.
#[derive(Clone,Copy)]
pub struct RetainedDiagnosticSpec{pub level:Severity,pub code:&'static str,pub message:&'static str}

/// 🏷️ Diagnostic addresses borrow original fields or finite declared literal targets.
#[derive(Clone,Copy)]
pub enum RetainedDiagnosticTarget<'a>{Original(RetainedCloneRef<'a,String>),Literal(&'static str),Unsigned(RetainedCloneRef<'a,u32>)}

/// 🧳️ Every original target, literal allocation and final row keeps typed closeable custody.
pub struct RetainedDiagnosticPreparation<const N:usize>{
 targets:[RetainedFieldPreparation<String>;N],target_kinds:[u8;N],integer_bindings:[Option<RetainedCloneBinding>;N],
 address:ManuallyDrop<Option<Vec<String>>>,
 code:ManuallyDrop<Option<String>>,
 text:ManuallyDrop<Option<String>>,
 rows:ManuallyDrop<Option<Vec<MutationMessage>>>,
 ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,
 retirement:RetainedCloneClose,
 literal:ManuallyDrop<Option<String>>,literal_offset:usize,literal_source:Option<&'static str>,
 spec:Option<RetainedDiagnosticSpec>,
 phase:usize,close_target:usize,close_binding:usize,target_close_started:bool,
 closing:bool,
 transferred:bool,
}

impl<const N:usize> Default for RetainedDiagnosticPreparation<N>{fn default()->Self{Self{targets:std::array::from_fn(|_|Default::default()),target_kinds:[0;N],integer_bindings:std::array::from_fn(|_|None),address:ManuallyDrop::new(None),code:ManuallyDrop::new(None),text:ManuallyDrop::new(None),rows:ManuallyDrop::new(None),ledger:ManuallyDrop::new(None),retirement:Default::default(),literal:ManuallyDrop::new(None),literal_offset:0,literal_source:None,spec:None,phase:0,close_target:0,close_binding:0,target_close_started:false,closing:false,transferred:false}}}

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
 pub fn demand(&self,target:[RetainedCloneRef<'_,String>;N],body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.target_demand(target.map(RetainedDiagnosticTarget::Original),body,maximum_work_units)}
 /// 🎯️ Declared literal targets use the same original address and diagnostic row custody.
 pub fn target_demand(&self,target:[RetainedDiagnosticTarget<'_>;N],body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||self.transferred{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic not available for preparation"));}
  let spec=self.spec.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic declaration missing"))?;
  if self.address.is_none()&&self.phase==0{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<String>>(),capacity_bytes:N.checked_mul(size_of::<String>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"semantic target layout overflow"))?,depth:1,..Default::default()});}
  if self.phase<N{
   let kind=match target[self.phase]{RetainedDiagnosticTarget::Original(_)=>1,RetainedDiagnosticTarget::Literal(_)=>2,RetainedDiagnosticTarget::Unsigned(_)=>3};if self.target_kinds[self.phase]!=0&&self.target_kinds[self.phase]!=kind{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic target authority kind changed"));}
   if let RetainedDiagnosticTarget::Unsigned(source)=target[self.phase]{
    source.check_binding(&self.integer_bindings[self.phase])?;
    if self.integer_bindings[self.phase].is_none(){return Ok(RetirementDemand{copy_bytes:source.binding_copy_bytes()+1,depth:1,..Default::default()});}
    return Ok(unsigned_demand(*source.get(),self.literal.is_some()));
   }
   if let RetainedDiagnosticTarget::Literal(literal)=target[self.phase]{
    if self.literal_source.is_some_and(|source|!std::ptr::eq(source,literal)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic literal target authority changed"));}
    if literal.len()>MUTATION_MESSAGE_ENTRY_BYTES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"semantic literal target exceeds row bound"));}
    let fixed=size_of::<String>()*2+size_of::<usize>()+size_of::<Option<&'static str>>()+size_of::<u8>();return Ok(if self.literal.is_none(){RetirementDemand{copy_bytes:fixed,capacity_bytes:literal.len(),depth:1,..Default::default()}}else{let work=usize::try_from(maximum_work_units.saturating_sub(1)).unwrap_or(usize::MAX);let count=literal.len().saturating_sub(self.literal_offset).min(body).min(2048).min(work);let mut end=self.literal_offset+count;while end>self.literal_offset&&!literal.is_char_boundary(end){end-=1;}RetirementDemand{copy_bytes:fixed+end-self.literal_offset,depth:1,..Default::default()}});
   }
   if self.literal_source.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic literal target cannot switch to an original"));}
   let RetainedDiagnosticTarget::Original(source)=target[self.phase]else{unreachable!()};let field=&self.targets[self.phase];
   if field.value().is_some(){let mut demand=field.exchange_demand()?;demand.copy_bytes+=size_of::<String>()+size_of::<u8>();return Ok(demand);}
   let mut d=field.demand(source,body,maximum_work_units.saturating_sub(1))?;d.copy_bytes+=1;return Ok(d);
  }
  if self.phase==N{return Ok(diagnostic_text_demand(spec.code,self.literal_offset,self.code.is_some(),body,maximum_work_units));}
  if self.phase==N+1{return Ok(diagnostic_text_demand(spec.message,self.literal_offset,self.text.is_some(),body,maximum_work_units));}
  if self.phase==N+2{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>(),capacity_bytes:size_of::<MutationMessage>(),depth:1,..Default::default()});}
  if self.phase==N+3{return Ok(RetirementDemand{copy_bytes:size_of::<MutationMessage>()+size_of::<MutationMessageLedgerRetirement>(),depth:1,..Default::default()});}
  Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>(),depth:1,..Default::default()})
 }
 /// ▶️ Builds only the admitted next original child and retains custody before validating actual receipts.
 pub fn advance_work_demand(&self,target:[RetainedCloneRef<'_,String>;N],body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.advance_targets_work_demand(target.map(RetainedDiagnosticTarget::Original),body,maximum_work_units)}
 /// ⛽️ Quotes the selected original target, bounded text segment or owned row handoff.
 pub fn advance_targets_work_demand(&self,target:[RetainedDiagnosticTarget<'_>;N],body:usize,maximum_work_units:u64)->Result<u64,ValueError>{let demand=self.target_demand(target,body,maximum_work_units)?;if maximum_work_units==0{return Ok(0)}let work=if self.address.is_none()&&self.phase==0{1}else if self.phase<N{match target[self.phase]{RetainedDiagnosticTarget::Original(source)=>{let field=&self.targets[self.phase];if field.value().is_some(){2}else{let child=field.advance_work_demand(source,body,maximum_work_units.saturating_sub(1))?;if child==0{return Ok(0)}child.checked_add(1).ok_or_else(||ValueError::invariant("diagnostic target work overflow"))?}},RetainedDiagnosticTarget::Literal(literal)=>{if self.literal.is_none()||self.literal_offset==literal.len(){2}else{let bytes=demand.copy_bytes-(size_of::<String>()*2+size_of::<usize>()+size_of::<Option<&'static str>>()+size_of::<u8>());if bytes==0{return Ok(0)}1+bytes as u64}},RetainedDiagnosticTarget::Unsigned(source)=>{if self.integer_bindings[self.phase].is_none()||self.literal.is_some(){2}else{let digits=if *source.get()==0{1}else{source.get().ilog10() as u64+1};2+2*digits}}}}else if self.phase==N{if self.code.is_some(){let bytes=demand.copy_bytes-size_of::<String>();if bytes==0&&self.literal_offset<self.spec.expect("started diagnostic").code.len(){return Ok(0)}1+bytes as u64}else{1}}else if self.phase==N+1{if self.text.is_some(){let bytes=demand.copy_bytes-size_of::<String>();if bytes==0&&self.literal_offset<self.spec.expect("started diagnostic").message.len(){return Ok(0)}1+bytes as u64}else{1}}else{u64::from(self.phase<=N+3)};Ok(if work<=maximum_work_units{work}else{0})}
 pub fn advance(&mut self,target:[RetainedCloneRef<'_,String>;N],body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementOutcome{self.advance_targets(target.map(RetainedDiagnosticTarget::Original),body,maximum_work_units,effect)}
 /// 🏹️ Literal and original address fragments share one bounded publication owner.
 pub fn advance_targets(&mut self,target:[RetainedDiagnosticTarget<'_>;N],body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementOutcome{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
  let selected=self.advance_targets_work_demand(target,body,maximum_work_units)?;if selected==0{return Ok(if self.phase>=N+4{RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())})}let demand=self.target_demand(target,body,maximum_work_units)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};work=1;
  if self.address.is_none()&&self.phase==0{let mut address=Vec::new();address.try_reserve_exact(N).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"diagnostic target allocation refused"))?;progress.retained_capacity_bytes=address.capacity()*size_of::<String>();*self.address=Some(address);return self.receipt(progress,grant,false).map_err(Into::into);}
  if self.phase<N{
   self.target_kinds[self.phase]=match target[self.phase]{RetainedDiagnosticTarget::Original(_)=>1,RetainedDiagnosticTarget::Literal(_)=>2,RetainedDiagnosticTarget::Unsigned(_)=>3};
   if let RetainedDiagnosticTarget::Unsigned(source)=target[self.phase]{
    if self.integer_bindings[self.phase].is_none(){let turn=source.bind(&mut self.integer_bindings[self.phase],&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant}));work+=turn.work;let mut p=turn.result.map_err(|error|{let mut p=error.retained_progress();p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(1);semio_framework_value::OriginalRetirementFailure::from(error.with_retained_progress(p))})?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original unsigned target binding omitted its capture").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}))?;p.copied_bytes+=1;return self.receipt(p,grant,false).map_err(Into::into);}
    if self.literal.is_none(){let(digits,start)=unsigned_digits(*source.get());work+=(10-start) as u128;let text=std::str::from_utf8(&digits[start..]).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"native unsigned decimal digits are invalid"))?;let mut value=String::new();work+=1;value.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"diagnostic numeric target allocation refused").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}))?;value.push_str(text);work+=text.len() as u128;progress.retained_capacity_bytes=value.capacity();*self.literal=Some(value);return self.receipt(progress,grant,false).map_err(Into::into);}
    work+=1;self.address.as_mut().expect("admitted numeric target backing").push(self.literal.take().expect("original paid decimal target"));self.phase+=1;return self.receipt(progress,grant,false).map_err(Into::into);
   }
   if let RetainedDiagnosticTarget::Literal(literal)=target[self.phase]{
    if self.literal.is_none(){let mut value=String::new();work+=1;value.try_reserve_exact(literal.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"semantic literal target reservation failed").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}))?;progress.retained_capacity_bytes=value.capacity();*self.literal=Some(value);self.literal_source=Some(literal);return self.receipt(progress,grant,false).map_err(Into::into);}
    let fixed=size_of::<String>()*2+size_of::<usize>()+size_of::<Option<&'static str>>()+size_of::<u8>();let count=demand.copy_bytes-fixed;
    if self.literal_offset<literal.len(){if count==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let end=self.literal_offset+count;self.literal.as_mut().unwrap().push_str(&literal[self.literal_offset..end]);work+=count as u128;self.literal_offset=end;}else{work+=1;self.address.as_mut().unwrap().push(self.literal.take().unwrap());self.literal_offset=0;self.literal_source=None;self.phase+=1;}return self.receipt(progress,grant,false).map_err(Into::into);
   }
   let RetainedDiagnosticTarget::Original(source)=target[self.phase]else{unreachable!()};let field=&mut self.targets[self.phase];
   if field.value().is_some(){work+=1;let(value,mut receipt)=field.take(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<String>()-1,..grant})?;self.address.as_mut().expect("admitted semantic target backing").push(value.expect("admitted original semantic target"));receipt.copied_bytes+=size_of::<String>()+1;self.phase+=1;return self.receipt(receipt,grant,false).map_err(Into::into);}
   let turn=field.advance(source,body,maximum_work_units-1,&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant}));work+=turn.work;let step=turn.result.map_err(|error|{let mut p=error.retained_progress();p.copied_bytes=p.copied_bytes.saturating_add(1);error.with_retained_progress(p)})?;let mut p=step.progress();p.copied_bytes+=1;p.copied_items=p.copied_items.max(1);return Ok(RetainedCloneStep::Progress(p));
  }
  let spec=self.spec.expect("started diagnostic");
  if self.phase==N{if self.code.is_none(){let mut code=String::new();code.try_reserve_exact(spec.code.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"diagnostic code allocation refused"))?;progress.retained_capacity_bytes=code.capacity();*self.code=Some(code);}else if self.literal_offset<spec.code.len(){let count=demand.copy_bytes-size_of::<String>();if count==0{work=0;return Ok(RetainedCloneStep::Progress(Default::default()))}let end=self.literal_offset+count;self.code.as_mut().unwrap().push_str(&spec.code[self.literal_offset..end]);work+=count as u128;self.literal_offset=end;}else{self.literal_offset=0;self.phase+=1;}return self.receipt(progress,grant,false).map_err(Into::into);}
  if self.phase==N+1{if self.text.is_none(){let mut text=String::new();text.try_reserve_exact(spec.message.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"diagnostic message allocation refused"))?;progress.retained_capacity_bytes=text.capacity();*self.text=Some(text);}else if self.literal_offset<spec.message.len(){let count=demand.copy_bytes-size_of::<String>();if count==0{work=0;return Ok(RetainedCloneStep::Progress(Default::default()))}let end=self.literal_offset+count;self.text.as_mut().unwrap().push_str(&spec.message[self.literal_offset..end]);work+=count as u128;self.literal_offset=end;}else{self.literal_offset=0;self.phase+=1;}return self.receipt(progress,grant,false).map_err(Into::into);}
  if self.phase==N+2{let mut rows=Vec::new();rows.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"diagnostic row allocation refused"))?;progress.retained_capacity_bytes=rows.capacity()*size_of::<MutationMessage>();*self.rows=Some(rows);self.phase+=1;return self.receipt(progress,grant,false).map_err(Into::into);}
  if self.phase==N+3{
   self.rows.as_mut().expect("admitted diagnostic row slot").push(MutationMessage{level:spec.level,code:FaultCode(self.code.take().expect("owned diagnostic code")),message:self.text.take().expect("owned diagnostic text"),target:self.address.take().expect("owned original target"),op_index:None});
   *self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),self.rows.take().expect("original message rows")));self.phase+=1;return self.receipt(progress,grant,true).map_err(Into::into);
  }
  work=0;Ok(RetainedCloneStep::Complete(Default::default()))
 })();settle(effect,base,result,work)}
 /// 📤️ Transfers original semantic diagnostics once after the whole constructor receipt is admitted.
 pub fn take_work_demand(&self)->Result<u64,ValueError>{if self.phase!=N+4||self.closing||self.transferred{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostics not ready for original transfer"))}Ok(1)}
 pub fn take(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<Option<(Vec<MutationMessage>,RetainedCloneProgress)>,semio_framework_value::OriginalRetirementFailure>{let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
  self.take_work_demand()?;let result=self.ledger.as_mut().expect("original diagnostic ledger").take_messages_controlled(grant)?;if result.is_some(){work=1;self.transferred=true;}Ok(result)
 })();semio_framework_value::RetainedWorkOutcome{result,work}}
 /// 🛑️ Starts irreversible cleanup of every prepared literal and original target alias.
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Quotes one typed target, partial text or original ledger child without an ordinary drop.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.close_target<N{let target=&self.targets[self.close_target];if target.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<usize>()+size_of::<bool>(),depth:1,..Default::default()})}if !self.target_close_started{return Ok(RetirementDemand{copy_bytes:3*size_of::<bool>(),depth:1,..Default::default()})}return target.close_demand(body);}
  if self.close_binding<N{let binding=&self.integer_bindings[self.close_binding];if binding.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<usize>(),depth:1,..Default::default()})}return RetainedCloneBinding::close_demand(binding,body);}
  if !self.retirement.is_empty(){return Ok(RetirementDemand{copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(body)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?});}
  if self.literal.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,body);}
  if self.address.is_some(){return Ok(self.retirement.next_owner_demands::<Vec<String>>(true,&None,body)?);}
  if self.code.is_some()||self.text.is_some(){return Ok(self.retirement.next_owner_demands::<String>(true,&None,body)?);}
  if self.rows.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<MutationMessageLedgerRetirement>()+size_of::<Vec<MutationMessage>>(),depth:1,..Default::default()});}
  if let Some(ledger)=self.ledger.as_ref(){return Ok(RetirementDemand{copy_bytes:ledger.next_copy_byte_demand(),capacity_bytes:ledger.next_capacity_byte_demand(body)?,release_bytes:ledger.next_release_byte_demand()?,depth:ledger.next_depth_demand()?});}
  Ok(Default::default())
 }
 /// 🪜️ Closes partial construction and original diagnostic custody on their exact admitted frontier.
 /// ⛽️ Quotes each inspected fixed target slot and the selected original child.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic closure not started"))}self.close_demand(body)?;if self.close_target<N{let target=&self.targets[self.close_target];return if target.terminal_is_empty()||!self.target_close_started{Ok(1)}else{target.close_work_demand(body)}}if self.close_binding<N{let binding=&self.integer_bindings[self.close_binding];return if binding.is_none(){Ok(1)}else{RetainedCloneBinding::close_work_demand(binding,body)}}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<String>(false,&None,body)}if self.literal.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}if self.address.is_some(){return self.retirement.next_owner_work::<Vec<String>>(true,&None,body)}if self.code.is_some()||self.text.is_some(){return self.retirement.next_owner_work::<String>(true,&None,body)}Ok(u64::from(self.rows.is_some()||self.ledger.is_some()))}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic closure not started").into());}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.close_target<N{let target=&mut self.targets[self.close_target];if target.terminal_is_empty(){work=1;self.close_target+=1;self.target_close_started=false;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}if !self.target_close_started{work=1;target.begin_close();self.target_close_started=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}let turn=RetirementPollExit::settled(target.close_step(effect))?;work=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  if self.close_binding<N{let binding=&mut self.integer_bindings[self.close_binding];if binding.is_none(){work=1;self.close_binding+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(binding,effect))?;work=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  if !self.retirement.is_empty(){let turn=RetirementPollExit::settled(self.retirement.step_granted(effect))?;work+=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()));}
  macro_rules! begin{($slot:ident)=>{{let turn=self.retirement.begin_granted(&mut self.$slot,effect);work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(step.progress()));}}};}
  begin!(literal);begin!(address);begin!(code);begin!(text);
  if let Some(rows)=self.rows.take(){work+=1;*self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),rows));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  if let Some(ledger)=self.ledger.as_mut(){let turn=ledger.close_step(grant);work+=u128::from(ledger.last_turn_work());let step=turn?;if ledger.terminal_is_empty(){*self.ledger=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())});}
  Ok(RetainedCloneStep::Complete(Default::default()))
 })();settle_exit(effect,base,result,work)}
 /// 🫙️ No diagnostic row, partial allocation or original target alias remains.
 pub fn terminal_is_empty(&self)->bool{(if self.closing{self.close_target==N&&self.close_binding==N}else{self.targets.iter().all(RetainedFieldPreparation::terminal_is_empty)&&self.integer_bindings.iter().all(Option::is_none)})&&self.literal.is_none()&&self.address.is_none()&&self.code.is_none()&&self.text.is_none()&&self.rows.is_none()&&self.ledger.is_none()&&self.retirement.is_empty()}
 fn receipt(&self,progress:RetainedCloneProgress,grant:RetainedCloneGrant,complete:bool)->Result<RetainedCloneStep,ValueError>{if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"semantic diagnostic actual construction exceeded original admission").with_retained_progress(progress));}Ok(if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})}
}
impl<const N:usize> Drop for RetainedDiagnosticPreparation<N>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"semantic diagnostic requires complete original typed closure");if empty{unsafe{ManuallyDrop::drop(&mut self.literal);ManuallyDrop::drop(&mut self.address);ManuallyDrop::drop(&mut self.code);ManuallyDrop::drop(&mut self.text);ManuallyDrop::drop(&mut self.rows);ManuallyDrop::drop(&mut self.ledger);}}}}

fn unsigned_digits(mut value:u32)->([u8;10],usize){let mut digits=[0u8;10];let mut start=10;loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{return(digits,start);}}}
fn diagnostic_text_demand(text:&str,offset:usize,prepared:bool,body:usize,maximum_work_units:u64)->RetirementDemand{let fixed=size_of::<String>();if !prepared{return RetirementDemand{copy_bytes:fixed,capacity_bytes:text.len(),depth:1,..Default::default()}}let work=usize::try_from(maximum_work_units.saturating_sub(1)).unwrap_or(usize::MAX);let count=text.len().saturating_sub(offset).min(body).min(2048).min(work);let mut end=offset+count;while end>offset&&!text.is_char_boundary(end){end-=1;}RetirementDemand{copy_bytes:fixed+end-offset,depth:1,..Default::default()}}
fn unsigned_demand(value:u32,prepared:bool)->RetirementDemand{let length=if value==0{1}else{value.ilog10()as usize+1};RetirementDemand{copy_bytes:2*size_of::<String>()+2*size_of::<Option<String>>()+2*size_of::<usize>()+1+if prepared{0}else{20+2*size_of::<u32>()+length},capacity_bytes:if prepared{0}else{length},depth:1,..Default::default()}}
