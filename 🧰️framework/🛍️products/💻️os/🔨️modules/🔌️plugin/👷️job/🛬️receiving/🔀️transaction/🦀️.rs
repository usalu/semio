//! 🔀️ Transaction preparation takes the original decoded command fields through retained typed receiving.
use semio_os_kernel as store;
use semio_plugin_decl_emission::ChildEmit;
use semio_framework_job::{OriginalCallerSourceLoan,OriginalConstructionAdmission,OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{NativeAllocationIdentity,RetainedCloneClose}};
use super::original_pack_receiving::OriginalPackReceiving;
use crate::original_steps::{settled_grant,widened,adopted,begun,built,built_with,construction,fixed,receipt,retired,retired_poll,unbuilt,unit};
use std::mem::{ManuallyDrop,size_of};
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{transaction_id:String,mutation_id:String,payload:Vec<u8>,operations:Vec<Vec<u8>>,child_bytes:Vec<u8>,origin_bytes:Vec<u8>,children:Option<Vec<ChildEmit>>,origin:Option<store::MutationOrigin>}
/// 📨️ All typed fields and unchanged parent wire allocations have one independent original owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct OriginalTransactionPrepareParts{pub transaction_id:String,pub mutation_id:String,pub operations:Vec<Vec<u8>>,pub children:Vec<ChildEmit>,pub origin:store::MutationOrigin}
/// 🛡️ A preparing command retains its exact decoder, caller loan and native child/origin parsers until handoff or cancellation.
pub struct OriginalTransactionPrepareSource{binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize,single:bool,phase:u8,fields:ManuallyDrop<Option<Fields>>,children:ManuallyDrop<Option<Box<OriginalPackReceiving<Vec<ChildEmit>>>>>,origin:ManuallyDrop<Option<Box<OriginalPackReceiving<store::MutationOrigin>>>>,loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,returning:bool,retirement:RetainedCloneClose,construction:OriginalConstructionAdmission,unit:Option<()>,closing:bool}
impl OriginalTransactionPrepareSource{
 /// 🧮️ Quotes the actual boxed preparing frame and its captured fields the caller must fund.
 pub fn admission_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<(Self,Fields)>()}}
 /// 🎟️ The native owner captures original fields only after the complete actual frame is funded.
 pub fn admit(command:&mut store::AppCommand,cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  let store::AppCommand::TransactionPrepare{txn_id,mutation_id,payload,prepared_ops,origin,prepared_child_ops,..}=command else{return Err(invalid("original transaction preparation requires its own decoded command").into())};
  let single=prepared_ops.is_empty()&&prepared_child_ops.is_empty();if output.is_none()&&(txn_id.is_empty()||txn_id.len()>128||origin.is_empty()||single&&(payload.is_empty()||mutation_id.is_empty())||!single&&(!payload.is_empty()||!mutation_id.is_empty())){return Err(invalid("original transaction preparation requires an exact identity, explicit provenance and one unambiguous operation source").into())}
  let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let d=Self::admission_demands();
  let settled=built_with(cx,admission,output,construction(d),|control|Ok((Box::new(Self{binding,decoder:(control.ledger_identity(),control.retirement_recipient_identity().ok_or_else(||invalid("original transaction receiving recipient is absent"))?),body,single,phase:0,fields:ManuallyDrop::new(Some(Fields{transaction_id:std::mem::take(txn_id),mutation_id:std::mem::take(mutation_id),payload:std::mem::take(payload),operations:std::mem::take(prepared_ops),child_bytes:std::mem::take(prepared_child_ops),origin_bytes:std::mem::take(origin),children:None,origin:None})),children:ManuallyDrop::new(None),origin:ManuallyDrop::new(None),loan:ManuallyDrop::new(None),returning:false,retirement:Default::default(),construction:Default::default(),unit:None,closing:false}),receipt(d))))?;Ok(adopted(settled,admission,output,|owner|&mut owner.construction))
 }
 fn next_phase(&mut self,phase:u8,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{unit(cx,&mut self.construction,&mut self.unit,fixed::<u8>(),||{self.phase=phase;Ok(())})?;Ok(())}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid("original transaction source changed its caller or decoder").into())}if !self.returning{if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)?}}Ok(())}
 pub fn transaction_id(&self)->Option<&str>{self.fields.as_ref().map(|fields|fields.transaction_id.as_str())}
 /// 🪜️ Each Pack parser and typed receiver closes before the independent preparing input becomes ready.
 pub fn advance(&mut self,limits:store::PackLimits,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing{return Err(invalid("closing transaction source cannot receive").into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original transaction preparation was cancelled").into())}if cx.should_yield(){return Ok(false)}if self.phase==8{return Ok(true)}if self.loan.is_none()&&self.phase<7{cx.original_source_parent()?.admit_loan(&mut self.loan,cx)?;return Ok(false)}match self.phase{
  0=>{if !self.single{self.next_phase(1,cx)?;return Ok(false)}let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;if fields.operations.capacity()==0{let d=RetirementDemand{capacity_bytes:size_of::<Vec<u8>>(),..fixed::<Vec<Vec<u8>>>()};if built(cx,&mut self.construction,&mut self.unit,construction(d),||{fields.operations.try_reserve_exact(1).map_err(|_|invalid("original owner-operation row could not be reserved"))?;let mut p=receipt(d);p.retained_capacity_bytes=fields.operations.capacity()*size_of::<Vec<u8>>();Ok(((),p))})?{self.unit.take();}return Ok(false)}unit(cx,&mut self.construction,&mut self.unit,fixed::<(Vec<u8>,u8)>(),||{fields.operations.push(std::mem::take(&mut fields.payload));self.phase=1;Ok(())})?;},
  1=>{let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;if self.children.is_none()&&fields.child_bytes.is_empty(){let d=fixed::<Option<Vec<ChildEmit>>>();built(cx,&mut self.construction,&mut fields.children,construction(d),||Ok((Vec::new(),receipt(d))))?;if fields.children.is_some(){self.phase=3}return Ok(false)}if self.children.is_none(){OriginalPackReceiving::admit(&mut fields.child_bytes,limits,cx,&mut self.construction,&mut self.children)?;return Ok(false)}let parser=self.children.as_mut().ok_or_else(||invalid("transaction child parser is absent"))?;if parser.advance(cx)?{self.phase=2}},
  2=>{let parser=self.children.as_mut().ok_or_else(||invalid("transaction child parser is absent"))?;if self.fields.as_ref().ok_or_else(||invalid("transaction receiving fields are absent"))?.children.is_none(){parser.take(cx,&mut self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?.children)?;return Ok(false)}if !parser.close_step(cx)?{return Ok(false)}let d=OriginalPackReceiving::<Vec<ChildEmit>>::release_demands();retired(cx,d,|_|{self.children.take();self.phase=3;Ok(RetainedCloneStep::Complete(receipt(d)))})?;},
  3=>{let fields=self.fields.as_mut().unwrap();if self.origin.is_none(){OriginalPackReceiving::admit(&mut fields.origin_bytes,limits,cx,&mut self.construction,&mut self.origin)?;return Ok(false)}if self.origin.as_mut().unwrap().advance(cx)?{self.phase=4}},
  4=>{let parser=self.origin.as_mut().ok_or_else(||invalid("transaction origin parser is absent"))?;if self.fields.as_ref().ok_or_else(||invalid("transaction receiving fields are absent"))?.origin.is_none(){parser.take(cx,&mut self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?.origin)?;return Ok(false)}if !parser.close_step(cx)?{return Ok(false)}let d=OriginalPackReceiving::<store::MutationOrigin>::release_demands();retired(cx,d,|_|{self.origin.take();self.phase=5;Ok(RetainedCloneStep::Complete(receipt(d)))})?;},
  5=>{let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;let value=if fields.payload.capacity()!=0{&mut fields.payload}else if fields.child_bytes.capacity()!=0{&mut fields.child_bytes}else if fields.origin_bytes.capacity()!=0{&mut fields.origin_bytes}else{self.next_phase(6,cx)?;return Ok(false)};if !value.is_empty(){return Err(invalid("original transaction source retained an unconsumed payload").into())}let d=RetirementDemand{release_bytes:value.capacity(),..fixed::<Vec<u8>>()};retired(cx,d,|_|{*value=Vec::new();Ok(RetainedCloneStep::Complete(receipt(d)))})?;},
  6=>{if let Some(loan)=self.loan.as_mut(){self.returning=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();self.phase=7;}}},
  7=>{unit(cx,&mut self.construction,&mut self.unit,fixed::<u8>(),||{self.phase=8;Ok(())})?;},_=>return Err(invalid("transaction receiving phase is invalid").into()),
 }Ok(self.phase==8)}
 /// 📤️ The original transaction identity and every parent wire allocation move without reconstruction.
 pub fn take(&mut self,cx:&mut StepContext<'_>,output:&mut Option<OriginalTransactionPrepareParts>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if output.is_none()&&(self.phase!=8||self.closing||self.fields.is_none()){return Err(invalid("original transaction input is not ready for unique handoff").into())}let d=Self::take_demands();built(cx,&mut self.construction,output,construction(d),||{let fields=self.fields.take().unwrap();Ok((OriginalTransactionPrepareParts{transaction_id:fields.transaction_id,mutation_id:fields.mutation_id,operations:fields.operations,children:fields.children.unwrap(),origin:fields.origin.unwrap()},receipt(d)))})}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;self.closing=true;
  if let Some(parser)=self.children.as_mut(){if !parser.close_step(cx)?{return Ok(false)}let d=OriginalPackReceiving::<Vec<ChildEmit>>::release_demands();retired(cx,d,|_|{self.children.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;return Ok(false)}
  if let Some(parser)=self.origin.as_mut(){if !parser.close_step(cx)?{return Ok(false)}let d=OriginalPackReceiving::<store::MutationOrigin>::release_demands();retired(cx,d,|_|{self.origin.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;return Ok(false)}
  if self.fields.is_some()||!self.retirement.is_empty(){let d=self.retirement.next_owner_demands::<Fields>(self.fields.is_some(),&None,self.body)?;if self.retirement.is_empty(){begun(cx,d,|effect|self.retirement.begin_granted(&mut self.fields,effect))?;}else{let body=self.body;retired_poll(cx,d,|effect|self.retirement.step_granted(body,effect))?;}return Ok(false)}
  if let Some(loan)=self.loan.as_mut(){self.returning=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();}return Ok(false)}if !unbuilt(cx,&mut self.construction)?{return Ok(false)}Ok(self.terminal_is_empty())
 }
 fn take_demands()->RetirementDemand{fixed::<(Fields,OriginalTransactionPrepareParts)>()}
 /// 🎫️ The grant of the unique handoff of the prepared parts.
 pub fn take_grant()->Result<RetainedCloneGrant,ValueError>{settled_grant::<OriginalTransactionPrepareParts>(Self::take_demands())}
 /// 🎫️ The grant of the next advance, the nested receiver's own grant while one runs; `fuel` is the caller's remaining work.
 pub fn advance_grant(&self,fuel:u64)->Result<RetainedCloneGrant,ValueError>{
  let demand=self.advance_demands(fuel)?;let child=||self.children.as_ref().ok_or_else(||invalid("transaction child parser is absent"));let origin=||self.origin.as_ref().ok_or_else(||invalid("transaction origin parser is absent"));let fields=||self.fields.as_ref().ok_or_else(||invalid("transaction receiving fields are absent"));
  match self.phase{
   1 if self.loan.is_some()&&self.children.is_none()&&fields()?.child_bytes.is_empty()=>settled_grant::<Vec<ChildEmit>>(demand),
   1 if self.loan.is_some()&&self.children.is_none()=>settled_grant::<Box<OriginalPackReceiving<Vec<ChildEmit>>>>(demand),
   1 if self.loan.is_some()&&self.children.is_some()=>child()?.advance_grant(fuel),
   2 if self.children.is_some()&&fields()?.children.is_none()=>OriginalPackReceiving::<Vec<ChildEmit>>::take_grant(),
   3 if self.loan.is_some()&&self.origin.is_some()=>origin()?.advance_grant(fuel),
   3 if self.loan.is_some()&&self.origin.is_none()=>settled_grant::<Box<OriginalPackReceiving<store::MutationOrigin>>>(demand),
   4 if self.origin.is_some()&&fields()?.origin.is_none()=>OriginalPackReceiving::<store::MutationOrigin>::take_grant(),
   _=>settled_grant::<()>(demand),
  }
 }
 /// 🎫️ The grant of the next close step.
 pub fn close_grant(&self)->Result<RetainedCloneGrant,ValueError>{settled_grant::<()>(self.close_demands()?)}
 fn advance_demands(&self,fuel:u64)->Result<RetirementDemand,ValueError>{
  let absent=||invalid("transaction receiving fields are absent");let parser=||invalid("transaction parser is absent");
  if self.loan.is_none()&&self.phase<7{return Ok(OriginalCallerSourceLoan::admission_demand())}
  let fields=||self.fields.as_ref().ok_or_else(absent);
  Ok(match self.phase{
   0=>if !self.single{fixed::<u8>()}else if fields()?.operations.capacity()==0{RetirementDemand{capacity_bytes:size_of::<Vec<u8>>(),..fixed::<Vec<Vec<u8>>>()}}else{fixed::<(Vec<u8>,u8)>()},
   1=>if self.children.is_none()&&fields()?.child_bytes.is_empty(){fixed::<Option<Vec<ChildEmit>>>()}else if self.children.is_none(){RetirementDemand{capacity_bytes:size_of::<OriginalPackReceiving<Vec<ChildEmit>>>(),..fixed::<OriginalPackReceiving<Vec<ChildEmit>>>()}}else{self.children.as_ref().ok_or_else(parser)?.advance_demands(fuel)?},
   2=>{let child=self.children.as_ref().ok_or_else(parser)?;if fields()?.children.is_none(){OriginalPackReceiving::<Vec<ChildEmit>>::take_demands()}else{widened(child.close_demands()?,OriginalPackReceiving::<Vec<ChildEmit>>::release_demands())}}
   3=>match self.origin.as_ref(){None=>RetirementDemand{capacity_bytes:size_of::<OriginalPackReceiving<store::MutationOrigin>>(),..fixed::<OriginalPackReceiving<store::MutationOrigin>>()},Some(origin)=>origin.advance_demands(fuel)?},
   4=>{let origin=self.origin.as_ref().ok_or_else(parser)?;if fields()?.origin.is_none(){OriginalPackReceiving::<store::MutationOrigin>::take_demands()}else{widened(origin.close_demands()?,OriginalPackReceiving::<store::MutationOrigin>::release_demands())}}
   5=>{let fields=fields()?;let capacity=[&fields.payload,&fields.child_bytes,&fields.origin_bytes].into_iter().map(Vec::capacity).find(|capacity|*capacity!=0);match capacity{Some(capacity)=>RetirementDemand{release_bytes:capacity,..fixed::<Vec<u8>>()},None=>fixed::<u8>()}}
   6=>widened(self.loan.as_ref().map_or(Default::default(),|loan|loan.retirement_demands()),fixed::<u8>()),
   _=>fixed::<u8>(),
  })
 }
 fn close_demands(&self)->Result<RetirementDemand,ValueError>{
  if let Some(parser)=self.children.as_ref(){return Ok(widened(parser.close_demands()?,OriginalPackReceiving::<Vec<ChildEmit>>::release_demands()))}
  if let Some(parser)=self.origin.as_ref(){return Ok(widened(parser.close_demands()?,OriginalPackReceiving::<store::MutationOrigin>::release_demands()))}
  if self.fields.is_some()||!self.retirement.is_empty(){return self.retirement.next_owner_demands::<Fields>(self.fields.is_some(),&None,self.body)}
  Ok(widened(self.loan.as_ref().map_or(Default::default(),|loan|loan.retirement_demands()),fixed::<u8>()))
 }
 pub fn terminal_is_empty(&self)->bool{self.fields.is_none()&&self.children.is_none()&&self.origin.is_none()&&self.loan.is_none()&&self.retirement.is_empty()&&self.construction.terminal_is_empty()}
 pub fn release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalTransactionPrepareSource{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original transaction preparing input requires complete funded closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
