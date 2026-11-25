//! 🪡️ Field plumbing the record machine runs without knowing the receive trait: scalar reads and the erased field plan.
use crate::{Number,NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,OriginalReceivingPrimary,RetainedWorkOutcome,retirement::RetireOwned,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits}};
use crate::native_decoding::NativeReceivingEffect;
use crate::OriginalRetirementFailure;
use std::mem::{ManuallyDrop,size_of};
pub use semio_framework_value_native_receiving_contract::{native_work_limit,work_sum,selected_native_work};
pub use semio_framework_value_native_schema_engine::source::{NativeTreeKind,NativeValueSource,NativeScalarQuery,NativeSourceScalar,NativeScalarQueryProducer,NativeScalarQueryOperation,invalid,fixed,receipt,nested,child_grant,add_receipt,NativeScalarQueryFrame};
use semio_framework_value_native_receiver_protocol::{NativeReceiver,NativeAdvanceSelection,OriginalEffectReply,OriginalEffectReplyComplete};

pub trait NativeScalarValue<I:NativeValueSource+?Sized>:RetireOwned{fn read(original:&I)->Result<Self,ValueError> where Self:Sized;fn read_copy_bytes(original:&I)->Result<usize,ValueError>;}
pub struct NativeScalarReceiver<I:NativeValueSource+?Sized,T:NativeScalarValue<I>>{binding:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<T>>,retirement:RetainedCloneClose,closing:bool,taken:bool,input:std::marker::PhantomData<fn(&I)>}
impl<I:NativeValueSource+?Sized,T:NativeScalarValue<I>> NativeScalarReceiver<I,T>{pub fn new()->Self{Self{binding:None,output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,taken:false,input:std::marker::PhantomData}}}
impl<I:NativeValueSource+?Sized,T:NativeScalarValue<I>> NativeReceiver<I,T> for NativeScalarReceiver<I,T>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,_:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.taken||self.output.is_some(){return Err(invalid());}if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()));}Ok(fixed(T::read_copy_bytes(input.get())?))}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,_body:usize,_maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{if self.closing||self.taken||self.output.is_some(){return Err(invalid())}Ok(NativeAdvanceSelection::Local{work:if input.check_binding(&self.binding)?{1}else{input.binding_work_demand(&self.binding)?}})}
 fn advance_nested(&mut self,_:RetainedCloneRef<'_,I>,_:&mut NativeDecodeControl<'_>,_:semio_framework_value_control_contract::OriginalSourceTurnWitness<'_>,_:usize,_:u64,_:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid().into()),work:0})}
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let result:Result<RetainedCloneStep,OriginalReceivingPrimary>=(||{let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)}
   if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native scalar action exceeded its original work").into())}*self.output=Some(T::read(input.get())?);Ok(RetainedCloneStep::Complete(receipt(d)))})();
  Ok(reply.complete(result))
 }
 fn advance_pending(&self)->bool{false}
 semio_framework_value_native_receiver_protocol::scalar_output_engine!();
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
 semio_framework_value_native_receiver_protocol::scalar_close_engine!();
 fn native_terminal_release(&self)->Option<semio_framework_value_native_receiver_protocol::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{semio_framework_value_native_receiver_protocol::RetainedCursorRelease::native(|value:&Self|value.terminal_is_empty())})}
  fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<I:NativeValueSource+?Sized,T:NativeScalarValue<I>> Drop for NativeScalarReceiver<I,T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native scalar receiving requires funded closure");}}
macro_rules! scalar {($t:ty,$input:ident,$value:expr)=>{impl<I:NativeValueSource+?Sized> NativeScalarValue<I> for $t{fn read($input:&I)->Result<Self,ValueError>{$value}fn read_copy_bytes(_: &I)->Result<usize,ValueError>{Ok(size_of::<Option<Self>>())}}};}
scalar!(bool,input,input.boolean());scalar!((),input,if input.is_null(){Ok(())}else{Err(invalid())});
scalar!(f64,input,{let n=input.number()?.as_f64();if n.is_finite(){Ok(n)}else{Err(invalid())}});
scalar!(f32,input,{let n=input.number()?.as_f64();if n.is_finite()&&n.abs()<=f32::MAX as f64{Ok(n as f32)}else{Err(invalid())}});
macro_rules! unsigned {($($t:ty),*)=>{$(scalar!($t,input,match input.number()?{Number::UInt(n)=><$t>::try_from(n).map_err(|_|invalid()),Number::Int(n)=><$t>::try_from(n).map_err(|_|invalid()),Number::Float(n)if n.is_finite()&&n.fract()==0.0&&n>=0.0&&(n as u128)<=<$t>::MAX as u128&&n<18446744073709551616.0=>Ok(n as $t),_=>Err(invalid())});)*};}
unsigned!(u8,u16,u32,u64,usize);
macro_rules! signed {($($t:ty),*)=>{$(scalar!($t,input,match input.number()?{Number::UInt(n)=><$t>::try_from(n).map_err(|_|invalid()),Number::Int(n)=><$t>::try_from(n).map_err(|_|invalid()),Number::Float(n)if n.is_finite()&&n.fract()==0.0&&n>=<$t>::MIN as f64&&n<=<$t>::MAX as f64&&n<9223372036854775808.0=>Ok(n as $t),_=>Err(invalid())});)*};}
signed!(i8,i16,i32,i64,isize);
macro_rules! tuple_scalar{($n:literal;$($t:ident:$k:literal),+)=>{impl<I:NativeValueSource+?Sized,$($t:NativeScalarValue<dyn NativeValueSource>),+> NativeScalarValue<I> for ($($t,)+){fn read(input:&I)->Result<Self,ValueError>{if input.sequence_len()?!=$n{return Err(invalid())}Ok(($($t::read(input.sequence_child($k)?)?,)+))}fn read_copy_bytes(_:&I)->Result<usize,ValueError>{Ok(size_of::<Option<Self>>())}}};}
/// 🍃️ Scalar tuples read as one leaf: the whole sequence of scalars is read in one inline step.
tuple_scalar!(2;A:0,B:1);tuple_scalar!(3;A:0,B:1,C:2);tuple_scalar!(4;A:0,B:1,C:2,D:3);
/// 🧭️ Erases one field slot so the struct-independent plan phases are compiled once instead of once per record.
pub trait NativeFieldPlanSlot{
 fn plan_ready(&self)->bool;
 fn plan_demands(&self,child:Option<crate::retained_clone::RetainedCloneRef<'_,dyn NativeValueSource>>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>;
 fn plan_take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn plan_take_work_demand(&self)->Result<u64,ValueError>;
 fn plan_terminal_is_empty(&self)->bool;
 fn plan_is_closing(&self)->bool;
 fn plan_begin_close_demand(&self)->Result<RetirementDemand,ValueError>;
 fn plan_begin_close_work_demand(&self)->Result<u64,ValueError>;
 fn plan_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn plan_close_work_demand(&self,body:usize)->Result<u64,ValueError>;
}
impl<T:semio_framework_value_native_default::NativeDefault,F:semio_framework_value_native_receiver_protocol::SlotFrame> NativeFieldPlanSlot for semio_framework_value_native_default::NativeDefaultSlot<T,F>{
 fn plan_ready(&self)->bool{self.ready()}
 fn plan_demands(&self,_:Option<crate::retained_clone::RetainedCloneRef<'_,dyn NativeValueSource>>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{self.demands(body,maximum)}
 fn plan_take_demand(&self)->Result<RetirementDemand,ValueError>{self.take_demand()}
 fn plan_take_work_demand(&self)->Result<u64,ValueError>{self.take_work_demand()}
 fn plan_terminal_is_empty(&self)->bool{self.terminal_is_empty()}
 fn plan_is_closing(&self)->bool{self.is_closing()}
 fn plan_begin_close_demand(&self)->Result<RetirementDemand,ValueError>{self.begin_close_demand()}
 fn plan_begin_close_work_demand(&self)->Result<u64,ValueError>{self.begin_close_work_demand()}
 fn plan_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demands(body)}
 fn plan_close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_work_demand(body)}
}
/// 🧭️ Plans every field stage except the projection of a present field (stage 0), which needs the typed input.
pub fn plan_field_stage(stage:u8,present:bool,slot:&dyn NativeFieldPlanSlot,source:&Option<crate::retained_clone::RetainedOwnedProjection<dyn NativeValueSource>>,child:Option<crate::retained_clone::RetainedCloneRef<'_,dyn NativeValueSource>>,projection:&crate::retained_clone::RetainedOwnedProjectionAdmission<dyn NativeValueSource>,body:usize,maximum:u64,option_bytes:usize,field_demand:fn(usize)->Result<RetirementDemand,ValueError>)->Result<(RetirementDemand,u64,u8),ValueError>{
 let one=||->Result<(RetirementDemand,u64,u8),ValueError>{Ok((fixed(size_of::<u8>()),1,13))};
 match stage{
  0=>if present{Err(invalid())}else{one()},
  8=>if present{Ok((projection.take_demand()?,projection.take_work_demand()?,24))}else{Err(invalid())},
  1=>if slot.plan_ready(){one()}else{Ok((slot.plan_demands(child,body,maximum)?,1,14))},
  2=>{let mut d=slot.plan_take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(option_bytes+size_of::<u8>()).ok_or_else(invalid)?;Ok((d,work_sum(slot.plan_take_work_demand()?,2)?,13))},
  3=>{let mut d=field_demand(0)?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<bool>()+size_of::<u8>()).ok_or_else(invalid)?;Ok((d,work_sum(1,2)?,13))},
  4=>if slot.plan_terminal_is_empty()||slot.plan_is_closing(){one()}else{Ok((slot.plan_begin_close_demand()?,slot.plan_begin_close_work_demand()?,21))},
  7=>if slot.plan_terminal_is_empty(){one()}else{Ok((slot.plan_close_demands(body)?,slot.plan_close_work_demand(body)?,15))},
  5=>if present{match source.as_ref(){Some(source)=>if source.terminal_is_empty(){Ok((fixed(size_of::<Option<crate::retained_clone::RetainedOwnedProjection<dyn NativeValueSource>>>()),1,13))}else{Ok((source.close_demand(body)?,source.close_work_demand(body)?,16))},None=>one()}}else{one()},
  6=>Ok((fixed(size_of::<usize>()+size_of::<u8>()),2,13)),
  _=>Err(invalid())}
}

/// 🍃️ Plans a scalar leaf field: stage 0 reads the one original scalar straight from its parent into the record's pending value (action 25) and stage 3 places it; a leaf owns no slot, projection alias or close frontier.
pub fn plan_leaf_stage(stage:u8,option_bytes:usize,field_demand:fn(usize)->Result<RetirementDemand,ValueError>)->Result<(RetirementDemand,u64,u8),ValueError>{
 match stage{
  0=>Ok((fixed(option_bytes+size_of::<u8>()),1,25)),
  3=>{let mut d=field_demand(0)?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<bool>()+size_of::<u8>()).ok_or_else(invalid)?;Ok((d,work_sum(1,2)?,13))},
  _=>Err(invalid())}
}
pub mod variant{pub use crate::selector::{NativeVariantSelector,NativeVariantSpec,NativeVariantSelection};}
