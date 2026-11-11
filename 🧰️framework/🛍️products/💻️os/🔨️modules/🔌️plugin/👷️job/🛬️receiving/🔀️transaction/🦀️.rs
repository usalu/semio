//! 🔀️ Transaction preparation takes the original decoded command fields through retained typed receiving.
use semio_os_kernel as store;
use semio_plugin_decl_emission::ChildEmit;
use semio_framework_job::{OriginalCallerSourceLoan,OriginalConstructionAdmission,OriginalConstructionDemand,OriginalConstructionFailure,OriginalConstructionStatus,OriginalSourceBinding,OriginalTurnFailure,StepContext,admit_original_construction};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{NativeAllocationIdentity,RetainedCloneClose}};
use super::original_pack_receiving::OriginalPackReceiving;
use super::original_steps::{begin,fixed,forgotten,funds,moved,receipt,retired};
use std::mem::{ManuallyDrop,size_of};
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{transaction_id:String,mutation_id:String,payload:Vec<u8>,operations:Vec<Vec<u8>>,child_bytes:Vec<u8>,origin_bytes:Vec<u8>,children:Option<Vec<ChildEmit>>,origin:Option<store::MutationOrigin>}
/// 📨️ All typed fields and unchanged parent wire allocations have one independent original owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct OriginalTransactionPrepareParts{pub transaction_id:String,pub mutation_id:String,pub operations:Vec<Vec<u8>>,pub children:Vec<ChildEmit>,pub origin:store::MutationOrigin}
/// 🛡️ A preparing command retains its exact decoder, caller loan and native child/origin parsers until handoff or cancellation.
pub struct OriginalTransactionPrepareSource{binding:OriginalSourceBinding,decoder:(NativeAllocationIdentity,u64),body:usize,single:bool,phase:u8,admitted:bool,fields:ManuallyDrop<Option<Fields>>,children:ManuallyDrop<Option<Box<OriginalPackReceiving<Vec<ChildEmit>>>>>,origin:ManuallyDrop<Option<Box<OriginalPackReceiving<store::MutationOrigin>>>>,frame:OriginalConstructionAdmission,failure:ManuallyDrop<Option<OriginalConstructionFailure>>,loan:ManuallyDrop<Option<OriginalCallerSourceLoan>>,returning:bool,retirement:RetainedCloneClose,closing:bool}
impl OriginalTransactionPrepareSource{
 /// 🧮️ Quotes the actual boxed preparing frame and its captured fields the caller must fund.
 pub fn admission_demands()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),..fixed::<(Self,Fields)>()}}
 /// 🎟️ The native owner captures original fields only after the complete actual frame is funded.
 pub fn admit(command:&mut store::AppCommand,cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?;
  if output.is_some(){return if admission.terminal_is_empty(){Ok(true)}else{admission.reset_step(cx)}}
  if failure.is_some(){return Err(invalid("original transaction preparation retained a failed construction").into())}
  let store::AppCommand::TransactionPrepare{txn_id,mutation_id,payload,prepared_ops,origin,prepared_child_ops,..}=command else{return Err(invalid("original transaction preparation requires its own decoded command").into())};
  let single=prepared_ops.is_empty()&&prepared_child_ops.is_empty();if txn_id.is_empty()||txn_id.len()>128||origin.is_empty()||single&&(payload.is_empty()||mutation_id.is_empty())||!single&&(!payload.is_empty()||!mutation_id.is_empty()){return Err(invalid("original transaction preparation requires an exact identity, explicit provenance and one unambiguous operation source").into())}
  let binding=OriginalSourceBinding::borrow(cx);let body=cx.retained_grant().maximum_copy_bytes;let d=Self::admission_demands();
  let status=admit_original_construction(cx,output,admission,construction(d),1,|native,_|{let result=(||{let decoder=(native.ledger_identity(),native.retirement_recipient_identity().ok_or_else(||invalid("original transaction receiving recipient is absent"))?);Ok((Box::new(Self{binding,decoder,body,single,phase:0,admitted:false,fields:ManuallyDrop::new(Some(Fields{transaction_id:std::mem::take(txn_id),mutation_id:std::mem::take(mutation_id),payload:std::mem::take(payload),operations:std::mem::take(prepared_ops),child_bytes:std::mem::take(prepared_child_ops),origin_bytes:std::mem::take(origin),children:None,origin:None})),children:ManuallyDrop::new(None),origin:ManuallyDrop::new(None),frame:Default::default(),failure:ManuallyDrop::new(None),loan:ManuallyDrop::new(None),returning:false,retirement:Default::default(),closing:false}),receipt(d)))})();semio_framework_value::RetainedWorkOutcome{work:u128::from(result.is_ok()),result}})?;
  match status{OriginalConstructionStatus::Ready=>admission.reset_step(cx),OriginalConstructionStatus::Pending=>Ok(false),OriginalConstructionStatus::Failed=>{if !admission.take_failure_into_step(failure,cx)?{return Ok(false)}Err(invalid("original transaction preparation construction failed; its causes remain retained").into())}}
 }
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{if !self.binding.matches(cx)||cx.native_receiving_identity()!=Some(self.decoder){return Err(invalid("original transaction source changed its caller or decoder").into())}if !self.returning{if let Some(loan)=self.loan.as_ref(){loan.validate_context(cx)?}}Ok(())}
 pub fn transaction_id(&self)->Option<&str>{self.fields.as_ref().map(|fields|fields.transaction_id.as_str())}
 /// 🪜️ Each Pack parser and typed receiver closes before the independent preparing input becomes ready.
 pub fn advance(&mut self,limits:store::PackLimits,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.closing{return Err(invalid("closing transaction source cannot receive").into())}if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original transaction preparation was cancelled").into())}if cx.should_yield(){return Ok(false)}if self.phase==8{return Ok(true)}if self.loan.is_none()&&self.phase<7{cx.original_source_parent()?.admit_loan(&mut self.loan,cx)?;return Ok(false)}match self.phase{
  0=>{if !self.single{if moved(cx,fixed::<u8>(),||{})?{self.phase=1}return Ok(false)}let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;if fields.operations.capacity()==0{let d=RetirementDemand{capacity_bytes:size_of::<Vec<u8>>(),..fixed::<Vec<Vec<u8>>>()};if funds(cx,d){fields.operations.try_reserve_exact(1).map_err(|_|invalid("original owner-operation row could not be reserved"))?;let mut p=receipt(d);p.retained_capacity_bytes=fields.operations.capacity()*size_of::<Vec<u8>>();cx.consume_retained(p)?;cx.consume_fuel(1)}return Ok(false)}let d=fixed::<(Vec<u8>,u8)>();if moved(cx,d,||{fields.operations.push(std::mem::take(&mut fields.payload));})?{self.phase=1}},
  1=>{let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;if self.children.is_none()&&fields.child_bytes.is_empty(){let d=fixed::<Option<Vec<ChildEmit>>>();if moved(cx,d,||{fields.children=Some(Vec::new());})?{self.phase=3}return Ok(false)}if !self.admitted{self.admitted=OriginalPackReceiving::admit(&mut fields.child_bytes,limits,cx,&mut self.frame,&mut self.failure,&mut self.children)?;return Ok(false)}let parser=self.children.as_mut().ok_or_else(||invalid("transaction child parser is absent"))?;if parser.advance(cx)?{self.phase=2}},
  2=>{let parser=self.children.as_mut().ok_or_else(||invalid("transaction child parser is absent"))?;let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;if fields.children.is_none(){parser.take(cx,&mut fields.children)?;return Ok(false)}if !parser.close_step(cx)?{return Ok(false)}if forgotten(cx,OriginalPackReceiving::<Vec<ChildEmit>>::release_demands(),||{self.children.take();})?{self.phase=3;self.admitted=false}},
  3=>{let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;if !self.admitted{self.admitted=OriginalPackReceiving::admit(&mut fields.origin_bytes,limits,cx,&mut self.frame,&mut self.failure,&mut self.origin)?;return Ok(false)}if self.origin.as_mut().ok_or_else(||invalid("transaction origin parser is absent"))?.advance(cx)?{self.phase=4}},
  4=>{let parser=self.origin.as_mut().ok_or_else(||invalid("transaction origin parser is absent"))?;let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;if fields.origin.is_none(){parser.take(cx,&mut fields.origin)?;return Ok(false)}if !parser.close_step(cx)?{return Ok(false)}if forgotten(cx,OriginalPackReceiving::<store::MutationOrigin>::release_demands(),||{self.origin.take();})?{self.phase=5}},
  5=>{let fields=self.fields.as_mut().ok_or_else(||invalid("transaction receiving fields are absent"))?;let value=if fields.payload.capacity()!=0{&mut fields.payload}else if fields.child_bytes.capacity()!=0{&mut fields.child_bytes}else if fields.origin_bytes.capacity()!=0{&mut fields.origin_bytes}else{if moved(cx,fixed::<u8>(),||{})?{self.phase=6}return Ok(false)};if !value.is_empty(){return Err(invalid("original transaction source retained an unconsumed payload").into())}let d=RetirementDemand{release_bytes:value.capacity(),..fixed::<Vec<u8>>()};forgotten(cx,d,||{*value=Vec::new();})?;},
  6=>{if let Some(loan)=self.loan.as_mut(){self.returning=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();self.phase=7;}}},
  7=>{if moved(cx,fixed::<u8>(),||{})?{self.phase=8}},_=>return Err(invalid("transaction receiving phase is invalid").into()),
 }Ok(self.phase==8)}
 /// 📤️ The original transaction identity and every parent wire allocation move without reconstruction.
 pub fn take(&mut self,cx:&mut StepContext<'_>,output:&mut Option<OriginalTransactionPrepareParts>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.phase!=8||self.closing||self.fields.is_none()||output.is_some(){return Err(invalid("original transaction input is not ready for unique handoff").into())}let d=fixed::<(Fields,OriginalTransactionPrepareParts)>();moved(cx,d,||{let fields=self.fields.take().unwrap();*output=Some(OriginalTransactionPrepareParts{transaction_id:fields.transaction_id,mutation_id:fields.mutation_id,operations:fields.operations,children:fields.children.unwrap(),origin:fields.origin.unwrap()});})}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;self.closing=true;let body=self.body;
  if !self.retirement.is_empty(){let d=self.retirement.next_owner_demands::<()>(false,&None,body)?;retired(cx,d,|effect|self.retirement.step_granted(effect.grant().maximum_copy_bytes,effect))?;return Ok(false)}
  if let Some(parser)=self.children.as_mut(){if !parser.close_step(cx)?{return Ok(false)}forgotten(cx,OriginalPackReceiving::<Vec<ChildEmit>>::release_demands(),||{self.children.take();})?;return Ok(false)}
  if let Some(parser)=self.origin.as_mut(){if !parser.close_step(cx)?{return Ok(false)}forgotten(cx,OriginalPackReceiving::<store::MutationOrigin>::release_demands(),||{self.origin.take();})?;return Ok(false)}
  if self.fields.is_some(){let d=self.retirement.next_owner_demands::<Fields>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.fields,effect))?;return Ok(false)}
  if self.failure.is_some(){let d=self.retirement.next_owner_demands::<OriginalConstructionFailure>(true,&None,body)?;retired(cx,d,|effect|begin(&mut self.retirement,&mut self.failure,effect))?;return Ok(false)}
  if !self.frame.terminal_is_empty(){self.frame.close_step(cx)?;return Ok(false)}
  if let Some(loan)=self.loan.as_mut(){self.returning=true;loan.close_step(cx.original_source_parent()?,cx)?;if loan.terminal_is_empty(){self.loan.take();}return Ok(false)}Ok(self.terminal_is_empty())
 }
 pub fn terminal_is_empty(&self)->bool{self.fields.is_none()&&self.children.is_none()&&self.origin.is_none()&&self.failure.is_none()&&self.frame.terminal_is_empty()&&self.loan.is_none()&&self.retirement.is_empty()}
 pub fn release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()}}
}
impl Drop for OriginalTransactionPrepareSource{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original transaction preparing input requires complete funded closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
