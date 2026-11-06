//! 🎟️ One original source prepays fixed return capture and receiving without a context-free callback.
use crate::{StepContext,OriginalTurnFailure,OriginalSourceBinding};
use crate::original_job_continuation::{OriginalReturnClaimCredit,OriginalReturnWitness,OriginalReturnPhase,claim_original_sized_return,receive_original_return_witness};
use semio_framework_value_retained_clone::{ValueError,ValueRefusalKind};

pub use semio_framework_job_return_budget::OriginalSizedClaimReserve;
/// 🪙️ See `README.md`.
pub struct OriginalCallerReturnPlace{credit:Option<OriginalReturnClaimCredit>,witness:Option<OriginalReturnWitness>,identity:Option<OriginalSourceBinding>,destination:Option<(usize,usize)>}
impl OriginalCallerReturnPlace{
 /// 🧮️ See `README.md`.
 pub fn core_wire_sized_claim_reserve(extent:usize)->Result<OriginalSizedClaimReserve,ValueError>{OriginalSizedClaimReserve::for_target(extent,crate::original_job_continuation::source::original_return_claim_copy_bytes(),std::mem::size_of::<Option<OriginalSourceBinding>>(),std::mem::size_of::<Option<(usize,usize)>>())}

pub const fn empty()->Self{Self{credit:None,witness:None,identity:None,destination:None}}
 pub fn terminal_is_empty(&self)->bool{self.credit.is_none()&&self.witness.is_none()&&self.identity.is_none()&&self.destination.is_none()}
 pub fn is_admitted(&self)->bool{self.credit.is_some()}
 /// 🔎️ See `README.md`.
 pub(crate) fn qualify_preissued_budget(&self,budget:&crate::original_job_continuation::source::OriginalReturnBudget,destination:usize,extent:usize)->Result<(),ValueError>{if self.destination!=Some((destination,extent)){return Err(ValueError::invariant("exposure return destination changed"))}budget.qualify_preissued_destination(self.credit.as_ref().ok_or_else(||ValueError::invariant("exposure credit absent"))?,self.witness.as_ref().ok_or_else(||ValueError::invariant("exposure witness absent"))?,destination,extent)}
 pub fn qualify_sized(&self,cx:&StepContext<'_>,destination:usize,copy_bytes:usize)->Result<(),OriginalTurnFailure>{let credit=self.credit.as_ref().ok_or_else(||ValueError::invariant("return credit absent"))?;crate::original_job_continuation::source::qualify_original_return_source(credit,cx,self.identity.ok_or_else(||ValueError::invariant("identity absent"))?)?;if self.destination!=Some((destination,copy_bytes)){return Err(ValueError::invariant("sized return destination changed").into())}Ok(())}
 pub fn admit(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let destination=self as *mut Self as usize;self.admit_sized(cx,destination,0)}
 pub fn admit_sized(&mut self,cx:&mut StepContext<'_>,destination:usize,copy_bytes:usize)->Result<bool,OriginalTurnFailure>{self.admit_sized_inner(cx,destination,copy_bytes,None)}
 /// ⏱️ See `README.md`.
 pub fn admit_sized_with_epoch(&mut self,cx:&mut StepContext<'_>,destination:usize,copy_bytes:usize,watermark:&mut u64)->Result<bool,OriginalTurnFailure>{let epoch=cx.original_source_epoch().filter(|epoch|*epoch>0&&*epoch>=*watermark).ok_or_else(||ValueError::invariant("return admission changed its Source epoch"))?;self.admit_sized_inner(cx,destination,copy_bytes,Some((watermark,epoch)))}
 fn admit_sized_inner(&mut self,cx:&mut StepContext<'_>,destination:usize,copy_bytes:usize,watermark:Option<(&mut u64,u64)>)->Result<bool,OriginalTurnFailure>{
  if destination==0{return Err(ValueError::invariant("sized return requires its exact nonzero preborn destination and extent").into())}
  if let Some(credit)=self.credit.as_ref(){crate::original_job_continuation::source::qualify_original_return_source(credit,cx,self.identity.ok_or_else(||ValueError::invariant("identity absent"))?)?;if self.destination!=Some((destination,copy_bytes)){return Err(ValueError::invariant("sized return destination changed").into())}return Ok(credit.has_unused_capture()&&self.witness.is_none())}
  if !cx.admit_original_return_table()?{return Ok(false)}
  let identity=OriginalSourceBinding::borrow(cx);let result=claim_original_sized_return(cx,&mut self.credit,destination,copy_bytes,std::mem::size_of::<Option<OriginalSourceBinding>>()+std::mem::size_of::<Option<(usize,usize)>>() + if watermark.is_some(){std::mem::size_of::<u64>()}else{0});
  if self.credit.is_some(){self.identity=Some(identity);self.destination=Some((destination,copy_bytes));if let Some((watermark,epoch))=watermark{*watermark=epoch}}result
 }
 /// 📮️ See `README.md`.
 pub fn record_return(&mut self)->Result<(),ValueError>{let destination=self as *mut Self as usize;self.record_sized_return(destination,0)}
 pub fn record_sized_return(&mut self,destination:usize,copy_bytes:usize)->Result<(),ValueError>{
  if self.destination!=Some((destination,copy_bytes)){return Err(ValueError::invariant("sized return changed its preborn destination or extent"))}
  if self.witness.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original return place already captured its return"))}
  let credit=self.credit.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original return place has no held source credit"))?;
  self.witness=Some(credit.issue_return(OriginalReturnPhase::Capture,copy_bytes)?);Ok(())
 }
pub fn capture_is_preissued(&self)->bool{self.witness.as_ref().is_some_and(OriginalReturnWitness::is_capture)}
pub const fn preissue_demand()->(usize,usize,u64){(2,std::mem::size_of::<Option<OriginalReturnWitness>>()+1,2)}
 /// 📮️ See `README.md`.
 pub fn preissue_capture(&mut self,cx:&mut StepContext<'_>,destination:usize,copy_bytes:usize)->Result<bool,OriginalTurnFailure>{
  self.qualify_sized(cx,destination,copy_bytes)?;let(items,bytes,work)=Self::preissue_demand();let progress=crate::RetainedCloneProgress{copied_items:items,copied_bytes:bytes,..Default::default()};
  if cx.retained_grant().maximum_depth==0||!progress.fits(cx.retained_grant())||cx.fuel_remaining()<work{return Ok(false)}cx.retained_progress().checked_add(progress)?;self.record_sized_return(destination,copy_bytes)?;cx.consume_fuel(work);cx.consume_retained(progress)?;self.qualify_sized(cx,destination,copy_bytes)?;Ok(true)
 }
 pub const fn cancel_preissue_demand()->(usize,usize,u64){(4,std::mem::size_of::<OriginalReturnPhase>()+std::mem::size_of::<usize>()+2,4)}
 pub fn cancel_preissued_capture(&mut self,cx:&mut StepContext<'_>,destination:usize,copy_bytes:usize)->Result<bool,OriginalTurnFailure>{
  self.qualify_sized(cx,destination,copy_bytes)?;let(items,bytes,work)=Self::cancel_preissue_demand();let p=crate::RetainedCloneProgress{copied_items:items,copied_bytes:bytes,..Default::default()};if cx.retained_grant().maximum_depth==0||!p.fits(cx.retained_grant())||cx.fuel_remaining()<work{return Ok(false)}cx.retained_progress().checked_add(p)?;
  self.credit.as_mut().ok_or_else(||ValueError::invariant("credit absent"))?.cancel_preissued(self.witness.as_mut().ok_or_else(||ValueError::invariant("witness absent"))?)?;cx.consume_fuel(work);cx.consume_retained(p)?;self.qualify_sized(cx,destination,copy_bytes)?;Ok(true)
 }
 pub fn settle_return(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let Some(credit)=self.credit.as_ref()else{return Ok(true)};crate::original_job_continuation::source::qualify_original_return_source(credit,cx,self.identity.ok_or_else(||ValueError::invariant("identity absent"))?)?;
  let captured=self.witness.is_some();let copy_bytes=self.destination.ok_or_else(||ValueError::invariant("destination absent"))?.1;
  if !captured{let credit=self.credit.as_mut().ok_or_else(||ValueError::invariant("credit absent"))?;self.witness=Some(if credit.has_unused_capture(){credit.issue_success_return(copy_bytes)?}else{credit.issue_return(OriginalReturnPhase::Receive,copy_bytes)?})}
  receive_original_return_witness(cx,self.credit.as_mut().ok_or_else(||ValueError::invariant("credit absent"))?,&mut self.witness)?;
  if self.credit.as_ref().ok_or_else(||ValueError::invariant("credit absent"))?.terminal_is_empty(){self.credit.take();self.identity=None;self.destination=None;return Ok(true)}
  Ok(false)
 }
}
impl Drop for OriginalCallerReturnPlace{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original return place requires same-source settlement of held currencies")}}
