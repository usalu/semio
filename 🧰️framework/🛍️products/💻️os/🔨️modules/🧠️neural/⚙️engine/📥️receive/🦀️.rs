//! 📥️ Native receiving of the untagged neural vocabulary: an object is a dictionary, any other scalar is an atom, with no intermediate wire owner.
use super::*;
use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneBinding,RetainedCloneRef,retained_clone_grant_admits}};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeReceive,NativeReceiver,owned::{NativeDefault,NativeMappedReceiver,NativeReceiveMap,tree::{NativeReceiveSlot,NativeScalarReceiver,NativeScalarValue,NativeValueSource,add_receipt,child_grant,fixed,invalid,nested,receipt}}};
use std::mem::size_of;

/// ⚛️ A scalar source selects its atom kind by its actual original type; integers beyond `i64` are refused.
impl<I:NativeValueSource+?Sized> NativeScalarValue<I> for Atom{
 fn read(input:&I)->Result<Self,ValueError>{
  if input.is_null(){return Ok(Self::Null)}
  if let Ok(value)=input.boolean(){return Ok(Self::Boolean(value))}
  if let Ok(number)=input.number(){return match number{Number::Int(value)=>Ok(Self::Integer(value)),Number::UInt(value)=>i64::try_from(value).map(Self::Integer).map_err(|_|invalid()),Number::Float(value)=>Ok(Self::Decimal(value))}}
  input.text().map(|text|Self::String(text.clone()))
 }
 fn read_copy_bytes(input:&I)->Result<usize,ValueError>{Ok(size_of::<Self>()+input.text().map_or(0,|text|text.len()))}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for Atom{
 type Receiver=NativeScalarReceiver<I,Self>;
 fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth()}
 fn begin_receive(grant:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(grant)}
}

/// 📚️ A dictionary adopts its completed ordered map of values without rebuilding it.
impl NativeReceiveMap<OrderedMap<Value>> for Dictionary{fn from_native(pairs:OrderedMap<Value>)->Self{Self{pairs}}}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for Dictionary{
 type Receiver=NativeMappedReceiver<I,OrderedMap<Value>,Self>;
 fn receive_birth_demand()->RetirementDemand{Self::Receiver::birth().expect("dictionary receiving has one fixed original map ancestor")}
 fn begin_receive(grant:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{Self::Receiver::begin(grant)}
}
impl NativeDefault for Dictionary{
 fn native_default_demand()->RetirementDemand{fixed(2*size_of::<Self>())}
 fn native_default(grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{let demand=Self::native_default_demand();if !retained_clone_grant_admits(grant,demand){return Err(invalid())}Ok((Self::new(),receipt(demand)))}
}

/// 💎️ The untagged value chooses its dictionary or atom child from the original source shape before any child work starts.
pub struct NativeValueReceiver<I:NativeValueSource+?Sized>{binding:Option<RetainedCloneBinding>,atom:NativeReceiveSlot<I,Atom>,dictionary:NativeReceiveSlot<I,Dictionary>,phase:u8,closing:bool}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for Value{
 type Receiver=NativeValueReceiver<I>;
 fn receive_birth_demand()->RetirementDemand{fixed(size_of::<Self::Receiver>())}
 fn begin_receive(grant:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let demand=<Self as NativeReceive<I>>::receive_birth_demand();if !retained_clone_grant_admits(grant,demand){return Err(invalid())}Ok((NativeValueReceiver{binding:None,atom:Default::default(),dictionary:Default::default(),phase:0,closing:false},receipt(demand)))}
}
impl<I:NativeValueSource+?Sized> NativeReceiver<I,Value> for NativeValueReceiver<I>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||self.phase>=5{return Err(invalid())}
  if !input.check_binding(&self.binding)?{return Ok(fixed(input.binding_copy_bytes()))}
  let inner=body.saturating_sub(size_of::<u8>());
  match self.phase{0=>Ok(fixed(size_of::<u8>())),1=>nested(self.atom.demands(input,inner)?,size_of::<u8>()),2=>nested(self.dictionary.demands(input,inner)?,size_of::<u8>()),_=>Err(invalid())}
 }
 fn advance(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.demands(input,body)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  control.checkpoint()?;
  if let Some(progress)=input.bind(&mut self.binding,grant)?{return Ok(RetainedCloneStep::Progress(progress))}
  if self.phase==0{self.phase=if input.get().record_len().is_ok(){2}else{1};return Ok(RetainedCloneStep::Progress(receipt(demand)))}
  let inner=body.saturating_sub(size_of::<u8>());let child=child_grant(grant,size_of::<u8>());
  let step=if self.phase==1{let step=self.atom.advance(input,control,inner,child)?;if self.atom.ready(){self.phase=3}step}else{let step=self.dictionary.advance(input,control,inner,child)?;if self.dictionary.ready(){self.phase=4}step};
  Ok(RetainedCloneStep::Progress(add_receipt(step.progress(),size_of::<u8>())?))
 }
 fn output_ready(&self)->bool{!self.closing&&matches!(self.phase,3|4)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}Ok(fixed(2*size_of::<Value>()+size_of::<u8>()))}
 fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Value,RetainedCloneProgress)>,ValueError>{
  let demand=self.take_demand()?;if !retained_clone_grant_admits(grant,demand){return Ok(None)}
  let value=if self.phase==3{Value::Atom(self.atom.take().ok_or_else(invalid)?)}else{Value::Dictionary(self.dictionary.take().ok_or_else(invalid)?)};
  self.phase=5;Ok(Some((value,receipt(demand))))
 }
 fn begin_close(&mut self){self.closing=true;self.atom.begin_close();self.dictionary.begin_close();}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.atom.terminal_is_empty(){return self.atom.close_demands(body)}
  if !self.dictionary.terminal_is_empty(){return self.dictionary.close_demands(body)}
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.atom.terminal_is_empty(){return self.atom.close_step(grant)}
  if !self.dictionary.terminal_is_empty(){return self.dictionary.close_step(grant)}
  RetainedCloneBinding::close_one(&mut self.binding,grant)
 }
 fn terminal_is_empty(&self)->bool{self.atom.terminal_is_empty()&&self.dictionary.terminal_is_empty()&&self.binding.is_none()}
}
impl<I:NativeValueSource+?Sized> Drop for NativeValueReceiver<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native value receiving requires funded closure")}}
