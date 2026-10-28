//! 📥️ Owned typed binding advances one admitted source frontier and retains every unfinished destination.
use crate::{FieldValue,RecordValue};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind, retirement::{RetireOwned, controlled::ControlledRetirement}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::mem::ManuallyDrop;

/// 🧩️ Every owned field supplies an allocation-free receiver constructor.
pub trait DslOwnedReceivingField:RetireOwned+Sized{
 type Cursor:OwnedFieldReceiving<Self>+Default;
}
/// 📑️ Generated records bind their original owned field slots incrementally.
pub trait DslOwnedReceivingRecord:DslOwnedReceivingField{}
/// 🏷️ A borrowed declaration selects its generated receiver without allocating metadata.
pub trait DslOwnedReceivingVariants:RetireOwned+Sized{
 type Cursor:OwnedFieldReceiving<Self>;
 fn receiving_cursor(keyword:&str)->Result<Self::Cursor,ValueError>;
}
/// ⏯️ The source and partial destination survive denied, canceled and failed turns until explicit closure.
pub trait OwnedFieldReceiving<T:RetireOwned>:Send+'static{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>;
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn is_ready(&self)->bool;
 fn take_into(&mut self,output:&mut Option<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// 📏️ A fixed metadata move never spends its payload length as a new allocation.
pub fn metadata<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
/// 🎟️ All five original currencies must admit the next frontier before its callback or mutation.
pub fn funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
/// 🧾️ Reports the actual admitted fixed move and its separately measured storage effects.
pub fn receipt(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes}}
/// 🪜️ A child keeps its physical depth inside the same original structural ceiling.
pub fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"owned receiving child depth overflow"))?;Ok(demand)}
/// 📉️ A nested cursor borrows the remaining depth without increasing another original currency.
pub fn child_grant(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
/// 🚫️ Shape refusal leaves the owned source at its current receiving entrance.
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"owned receiving source differs from its declared field")}
/// 🧯️ A receiving invariant never substitutes a semantic field conversion.
pub fn invariant()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"owned receiving source or destination is already occupied")}

/// 🛂️ Closed authored record identities leave all rejected fields in their exact original source.
pub fn declared_record(record:&RecordValue,declared:&[(u16,bool)],owner:&mut super::NativeSnapshotDecodeOwner<'_,'_>)->Result<(),ValueError>{let mut shape=DeclaredRecordShape::default();while !shape.is_ready(){let grant=owner.remaining_grant();if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"declared record shape requires original work authority"))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"declared record shape requires original depth authority"))}match shape.step(record,declared,owner.native(),grant){Ok(progress)=>owner.record_progress(progress)?,Err(error)=>{owner.record_progress(error.retained_progress())?;return Err(error)}}}Ok(())}

/// 📹️ One funded declaration inspection retains the whole original record until shape admission completes.
#[derive(Default)]
pub struct DeclaredRecordShape{declaration:usize,field:usize,ready:bool,refused:bool}
impl DeclaredRecordShape{
 /// 👓️ Reports completed admission without traversing the original source.
 pub fn is_ready(&self)->bool{self.ready}
 /// 📌️ Caps source count before constant-time slot inspection under one original metadata turn.
 pub fn step(&mut self,record:&RecordValue,declared:&[(u16,bool)],native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.ready{return Ok(Default::default())}let demand=metadata::<()>();if !funds(grant,demand){return Ok(Default::default())}native.checkpoint()?;if self.refused{return Err(invalid())}let progress=receipt(demand);let result=(||{if record.fields.len()>declared.len(){return Err(invalid())}if let Some(&(id,required))=declared.get(self.declaration){let value=record.fields.at(self.field);if value.is_some_and(|(field,_)|field<id){return Err(invalid())}let present=value.filter(|(field,_)|*field==id);if required&&present.is_none_or(|(_,value)|matches!(value,FieldValue::Absent)){return Err(invalid())}if present.is_some(){self.field+=1}self.declaration+=1}if self.declaration==declared.len(){if self.field!=record.fields.len(){return Err(invalid())}self.ready=true}Ok(progress)})();if result.is_err(){self.refused=true}result.map_err(|error|error.with_retained_progress(progress))
 }
}

/// 🎒️ A retained root closes physically before its slot can disappear.
pub struct OwnedSlot<T:RetireOwned>{pub value:ManuallyDrop<Option<T>>,retiring:ManuallyDrop<Option<ControlledRetirement<T>>>}
impl<T:RetireOwned> Default for OwnedSlot<T>{fn default()->Self{Self{value:ManuallyDrop::new(None),retiring:ManuallyDrop::new(None)}}}
impl<T:RetireOwned> OwnedSlot<T>{
 /// 🕳️ A scalar transfer empties this root without touching another allocation.
 pub fn terminal_is_empty(&self)->bool{self.value.is_none()&&self.retiring.is_none()}
 /// 🔎️ Quotes the retained root's next original retirement frontier.
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.retiring.as_ref(){Some(owner)=>owner.selected_demand(body),None if self.value.is_some()=>Ok(metadata::<T>()),None=>Ok(Default::default())}}
 /// ♻️ Denied closure preserves both root and cursor; admitted closure reports every physical effect.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}let demand=self.close_demands(grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(Default::default())}if let Some(owner)=self.retiring.as_mut(){let step=owner.step(grant)?;if owner.terminal_is_empty(){self.retiring.take();}return Ok(match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress})}let value=self.value.take().unwrap();match ControlledRetirement::new(value){Ok(owner)=>*self.retiring=Some(owner),Err((error,value))=>{*self.value=Some(value);return Err(error)}}Ok(receipt(demand))}
}
impl<T:RetireOwned> Drop for OwnedSlot<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"owned receiving slot requires paid terminal closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.retiring)}}}}
/// 🪙️ Decode-time source cleanup admits its exact metadata births on the same original native ledger.
pub fn close_source<T:RetireOwned>(owner:&mut OwnedSlot<T>,native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let demand=owner.close_demands(grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(Default::default())}native.charge(demand.capacity_bytes)?;owner.close_step(grant)}

/// ✅️ Scalar implementations validate before taking the original source.
pub trait OwnedScalar:RetireOwned+Sized{fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>;}
/// 📬️ Fixed scalar metadata retains its original text, bytes and semantic identity.
pub struct ScalarReceiving<T:OwnedScalar>{output:OwnedSlot<T>,used:bool}
impl<T:OwnedScalar> Default for ScalarReceiving<T>{fn default()->Self{Self{output:Default::default(),used:false}}}
impl<T:OwnedScalar> OwnedFieldReceiving<T> for ScalarReceiving<T>{
 fn demands(&self,_:&Option<FieldValue>,_:usize)->Result<RetirementDemand,ValueError>{Ok(metadata::<FieldValue>())}
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.used{return Err(invariant())}let demand=self.demands(source,body)?;if !funds(grant,demand){return Ok(Default::default())}native.checkpoint()?;*self.output.value=Some(T::take_source(source)?);self.used=true;Ok(receipt(demand))}
 fn is_ready(&self)->bool{self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if output.is_some()||!self.is_ready(){return Err(invariant())}let demand=metadata::<T>();if !funds(grant,demand){return Ok(Default::default())}*output=self.output.value.take();Ok(receipt(demand))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.output.close_demands(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.output.close_step(grant)}
 fn terminal_is_empty(&self)->bool{self.output.terminal_is_empty()}
}
macro_rules! integer{($variant:ident;$($ty:ty),+)=>{$(impl OwnedScalar for $ty{fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{let Some(FieldValue::$variant(value))=source.as_ref()else{return Err(invalid())};let output=Self::try_from(*value).map_err(|_|invalid())?;source.take();Ok(output)}}impl DslOwnedReceivingField for $ty{type Cursor=ScalarReceiving<Self>;})+};}
integer!(Int;i8,i16,i32,i64,isize);
integer!(UInt;u16,u32,u64,usize);
impl OwnedScalar for u8{fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{let Some(FieldValue::UInt(value))=source.as_ref()else{return Err(invalid())};let output=Self::try_from(*value).map_err(|_|invalid())?;source.take();Ok(output)}}
impl DslOwnedReceivingField for u8{type Cursor=ScalarReceiving<Self>;}
macro_rules! copied{($ty:ty,$variant:ident)=>{impl OwnedScalar for $ty{fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{let Some(FieldValue::$variant(value))=source.as_ref()else{return Err(invalid())};let output=*value;source.take();Ok(output)}}impl DslOwnedReceivingField for $ty{type Cursor=ScalarReceiving<Self>;}};}
copied!(bool,Bool);copied!(f64,Float);
impl OwnedScalar for f32{fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{let Some(FieldValue::Float(value))=source.as_ref()else{return Err(invalid())};let bits=value.to_bits();let output=if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err(invalid())}Self::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff))}else{*value as Self};source.take();Ok(output)}}
impl DslOwnedReceivingField for f32{type Cursor=ScalarReceiving<Self>;}
macro_rules! moved{($ty:ty,$variant:ident)=>{impl OwnedScalar for $ty{fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{if !matches!(source,Some(FieldValue::$variant(_))){return Err(invalid())}let Some(FieldValue::$variant(value))=source.take()else{unreachable!()};Ok(value)}}impl DslOwnedReceivingField for $ty{type Cursor=ScalarReceiving<Self>;}};}
moved!(crate::RecordBytes,Bytes64);moved!(String,Text);moved!(semio_framework_value::DslValue,Value);moved!(crate::WireValue,Wire);moved!(crate::ExprValue,Expr);moved!(RecordValue,Record);

/// 🪆️ One paid cursor box bounds recursive receiving without an unbounded native stack owner.
pub fn child_demand<C>()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<C>(),depth:2,..metadata::<FieldValue>()}}
/// 🐣️ The original native allocation ledger admits a child before its cursor storage is born.
pub fn admit_child<C:Default>(native:&mut NativeDecodeControl<'_>)->Result<Box<C>,ValueError>{native.charge(size_of::<C>())?;Ok(Box::new(C::default()))}
/// 📤️ Empty child cursor storage has its own paid physical release.
pub fn release_child<C>(child:&mut Option<Box<C>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if child.is_none(){return Ok(Default::default())}let demand=RetirementDemand{release_bytes:size_of::<C>(),depth:2,..Default::default()};if !funds(grant,demand){return Ok(Default::default())}child.take();Ok(receipt(demand))}

/// 🧮️ Typed vector backing and every child have separate original admitted turns.
pub struct VectorReceiving<T:DslOwnedReceivingField>{source:OwnedSlot<FieldValue>,current:OwnedSlot<FieldValue>,child_output:OwnedSlot<T>,output:OwnedSlot<Vec<T>>,child:Option<Box<T::Cursor>>,index:usize,length:usize,initialized:bool,complete:bool,closing:bool}
impl<T:DslOwnedReceivingField> Default for VectorReceiving<T>{fn default()->Self{Self{source:Default::default(),current:Default::default(),child_output:Default::default(),output:Default::default(),child:None,index:0,length:0,initialized:false,complete:false,closing:false}}}
impl<T:DslOwnedReceivingField> DslOwnedReceivingField for Vec<T>{type Cursor=VectorReceiving<T>;}
impl<T:DslOwnedReceivingField> OwnedFieldReceiving<Vec<T>> for VectorReceiving<T>{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.initialized{let value=source.as_ref().ok_or_else(invalid)?;let length=match value{FieldValue::List(values)|FieldValue::Tuple(values)=>values.len(),FieldValue::Bytes64(values)=>values.len(),_=>return Err(invalid())};return Ok(RetirementDemand{capacity_bytes:length.checked_mul(size_of::<T>()).ok_or_else(invariant)?,..metadata::<FieldValue>()})}
  if let Some(child)=self.child.as_ref(){if self.child_output.value.is_some(){return Ok(RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:2,..metadata::<T>()})}if child.is_ready(){return nested(metadata::<T>())}return child.demands(&self.current.value,body).and_then(nested)}
  if self.index<self.length{return Ok(child_demand::<T::Cursor>())}if !self.source.terminal_is_empty(){return self.source.close_demands(body)}Ok(metadata::<bool>())
 }
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.closing{return Err(invariant())}if self.complete{return Ok(Default::default())}let demand=self.demands(source,body)?;if !funds(grant,demand){return Ok(Default::default())}
  if let Some(child)=self.child.as_mut(){if self.child_output.value.is_none()&&!child.is_ready(){return child.step(&mut self.current.value,native,body,child_grant(grant))}}native.checkpoint()?;
  if !self.initialized{
   let length=match source.as_ref().unwrap(){FieldValue::List(values)|FieldValue::Tuple(values)=>values.len(),FieldValue::Bytes64(values)=>values.len(),_=>return Err(invalid())};native.charge(demand.capacity_bytes)?;let mut output=Vec::new();output.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original typed vector backing admission failed"))?;let actual=output.capacity().checked_mul(size_of::<T>()).ok_or_else(invariant)?;if actual!=demand.capacity_bytes{return Err(invariant())}*self.output.value=Some(output);*self.source.value=source.take();self.length=length;self.initialized=true;return Ok(receipt(demand))
  }
  if self.child.is_some(){
   if self.child_output.value.is_some(){if !self.child.as_ref().unwrap().terminal_is_empty(){return Err(invariant())}let output=self.output.value.as_mut().unwrap();if output.len()==output.capacity(){return Err(invariant())}output.push(self.child_output.value.take().unwrap());self.child.take();self.index+=1;return Ok(receipt(demand))}
   return self.child.as_mut().unwrap().take_into(&mut self.child_output.value,child_grant(grant))
  }
  if self.index<self.length{let child=admit_child::<T::Cursor>(native)?;let value=match self.source.value.as_mut().unwrap(){FieldValue::List(values)|FieldValue::Tuple(values)=>std::mem::replace(&mut values[self.index],FieldValue::Absent),FieldValue::Bytes64(values)=>FieldValue::UInt(u64::from(values.retained_bytes()[self.index])),_=>return Err(invariant())};*self.current.value=Some(value);self.child=Some(child);return Ok(receipt(demand))}
  if !self.source.terminal_is_empty(){return close_source(&mut self.source,native,grant)}self.complete=true;Ok(receipt(demand))
 }
 fn is_ready(&self)->bool{!self.closing&&self.complete&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<Vec<T>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if output.is_some()||!self.is_ready(){return Err(invariant())}let demand=metadata::<Vec<T>>();if !funds(grant,demand){return Ok(Default::default())}*output=self.output.value.take();Ok(receipt(demand))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:2,..Default::default()})}else{child.close_demands(body).and_then(nested)}}if !self.current.terminal_is_empty(){return self.current.close_demands(body)}if !self.child_output.terminal_is_empty(){return self.child_output.close_demands(body)}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.source.close_demands(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}let demand=self.close_demands(grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(Default::default())}self.closing=true;if let Some(child)=self.child.as_mut(){return if child.terminal_is_empty(){release_child(&mut self.child,grant)}else{child.close_step(child_grant(grant))}}if !self.current.terminal_is_empty(){return self.current.close_step(grant)}if !self.child_output.terminal_is_empty(){return self.child_output.close_step(grant)}if !self.output.terminal_is_empty(){return self.output.close_step(grant)}self.source.close_step(grant)}
 fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.current.terminal_is_empty()&&self.child_output.terminal_is_empty()&&self.output.terminal_is_empty()&&self.source.terminal_is_empty()}
}

/// 🎁️ Nullable and boxed fields share one retained child without sharing a destination allocation.
pub trait OwnedWrapping<T:RetireOwned>:Send+'static{type Output:RetireOwned;const OPTIONAL:bool;fn empty()->Option<Self::Output>;fn demand()->RetirementDemand;fn wrap(value:T)->Self::Output;}
/// ❓️ A nullable scalar distinguishes original absence from a pending child.
pub enum OptionalWrapping{}
impl<T:RetireOwned> OwnedWrapping<T> for OptionalWrapping{type Output=Option<T>;const OPTIONAL:bool=true;fn empty()->Option<Self::Output>{Some(None)}fn demand()->RetirementDemand{metadata::<Option<T>>()}fn wrap(value:T)->Self::Output{Some(value)}}
/// 📦️ A required box admits its exact typed backing after its child completes.
pub enum BoxWrapping{}
impl<T:RetireOwned> OwnedWrapping<T> for BoxWrapping{type Output=Box<T>;const OPTIONAL:bool=false;fn empty()->Option<Self::Output>{None}fn demand()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<T>(),..metadata::<T>()}}fn wrap(value:T)->Self::Output{Box::new(value)}}
/// 📩️ Partial child input and output remain distinct original roots throughout wrapping and cleanup.
pub struct WrappedReceiving<T:DslOwnedReceivingField,W:OwnedWrapping<T>>{source:OwnedSlot<FieldValue>,current:OwnedSlot<FieldValue>,child_output:OwnedSlot<T>,output:OwnedSlot<W::Output>,child:Option<Box<T::Cursor>>,initialized:bool,selected:bool,complete:bool,closing:bool,kind:std::marker::PhantomData<W>}
impl<T:DslOwnedReceivingField,W:OwnedWrapping<T>> Default for WrappedReceiving<T,W>{fn default()->Self{Self{source:Default::default(),current:Default::default(),child_output:Default::default(),output:Default::default(),child:None,initialized:false,selected:false,complete:false,closing:false,kind:Default::default()}}}
impl<T:DslOwnedReceivingField> DslOwnedReceivingField for Option<T>{type Cursor=WrappedReceiving<T,OptionalWrapping>;}
impl<T:DslOwnedReceivingField> DslOwnedReceivingField for Box<T>{type Cursor=WrappedReceiving<T,BoxWrapping>;}
impl<T:DslOwnedReceivingField,W:OwnedWrapping<T>> WrappedReceiving<T,W>{
 fn optional_record(&self)->Result<&RecordValue,ValueError>{let Some(FieldValue::Block(inner))=self.source.value.as_ref()else{return Err(invariant())};let FieldValue::Record(record)=inner.as_ref()else{return Err(invalid())};if record.fields.len()>1||record.fields.keys().any(|id|*id!=0){return Err(invalid())}Ok(record)}
}
impl<T:DslOwnedReceivingField,W:OwnedWrapping<T>> OwnedFieldReceiving<W::Output> for WrappedReceiving<T,W>{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.initialized{if W::OPTIONAL&&matches!(source,Some(FieldValue::Absent|FieldValue::Block(_))){return Ok(metadata::<FieldValue>())}if source.is_none(){return Err(invalid())}return Ok(child_demand::<T::Cursor>())}
  if !self.selected{return Ok(if self.optional_record()?.get(0).is_none_or(|value|matches!(value,FieldValue::Absent)){metadata::<W::Output>()}else{child_demand::<T::Cursor>()})}
  if let Some(child)=self.child.as_ref(){if self.output.value.is_some(){return Ok(RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:2,..Default::default()})}if self.child_output.value.is_some(){return Ok(W::demand())}if child.is_ready(){return nested(metadata::<T>())}return child.demands(&self.current.value,body).and_then(nested)}
  if !self.source.terminal_is_empty(){return self.source.close_demands(body)}Ok(metadata::<bool>())
 }
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.closing{return Err(invariant())}if self.complete{return Ok(Default::default())}let demand=self.demands(source,body)?;if !funds(grant,demand){return Ok(Default::default())}if let Some(child)=self.child.as_mut(){if self.output.value.is_none()&&self.child_output.value.is_none()&&!child.is_ready(){return child.step(&mut self.current.value,native,body,child_grant(grant))}}native.checkpoint()?;
  if !self.initialized{
   if W::OPTIONAL&&matches!(source,Some(FieldValue::Absent)){source.take();*self.output.value=W::empty();self.initialized=true;self.selected=true;self.complete=true;return Ok(receipt(demand))}
   if W::OPTIONAL&&matches!(source,Some(FieldValue::Block(_))){let Some(FieldValue::Block(inner))=source.as_ref()else{unreachable!()};let FieldValue::Record(record)=inner.as_ref()else{return Err(invalid())};if record.fields.len()>1||record.fields.keys().any(|id|*id!=0){return Err(invalid())}*self.source.value=source.take();self.initialized=true;return Ok(receipt(demand))}
   let child=admit_child::<T::Cursor>(native)?;*self.current.value=source.take();self.child=Some(child);self.initialized=true;self.selected=true;return Ok(receipt(demand))
  }
  if !self.selected{if self.optional_record()?.get(0).is_none_or(|value|matches!(value,FieldValue::Absent)){*self.output.value=W::empty();self.selected=true;return Ok(receipt(demand))}let child=admit_child::<T::Cursor>(native)?;let Some(FieldValue::Block(inner))=self.source.value.as_mut()else{return Err(invariant())};let FieldValue::Record(record)=inner.as_mut()else{return Err(invariant())};*self.current.value=Some(std::mem::replace(record.fields.get_mut(&0).unwrap(),FieldValue::Absent));self.child=Some(child);self.selected=true;return Ok(receipt(demand))}
  if self.child.is_some(){if self.output.value.is_some(){if !self.child.as_ref().unwrap().terminal_is_empty(){return Err(invariant())}return release_child(&mut self.child,grant)}if self.child_output.value.is_some(){native.charge(demand.capacity_bytes)?;*self.output.value=Some(W::wrap(self.child_output.value.take().unwrap()));return Ok(receipt(demand))}return self.child.as_mut().unwrap().take_into(&mut self.child_output.value,child_grant(grant))}
  if !self.source.terminal_is_empty(){return close_source(&mut self.source,native,grant)}self.complete=true;Ok(receipt(demand))
 }
 fn is_ready(&self)->bool{!self.closing&&self.complete&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<W::Output>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if output.is_some()||!self.is_ready(){return Err(invariant())}let demand=metadata::<W::Output>();if !funds(grant,demand){return Ok(Default::default())}*output=self.output.value.take();Ok(receipt(demand))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:2,..Default::default()})}else{child.close_demands(body).and_then(nested)}}if !self.current.terminal_is_empty(){return self.current.close_demands(body)}if !self.child_output.terminal_is_empty(){return self.child_output.close_demands(body)}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.source.close_demands(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}let demand=self.close_demands(grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(Default::default())}self.closing=true;if let Some(child)=self.child.as_mut(){return if child.terminal_is_empty(){release_child(&mut self.child,grant)}else{child.close_step(child_grant(grant))}}if !self.current.terminal_is_empty(){return self.current.close_step(grant)}if !self.child_output.terminal_is_empty(){return self.child_output.close_step(grant)}if !self.output.terminal_is_empty(){return self.output.close_step(grant)}self.source.close_step(grant)}
 fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.current.terminal_is_empty()&&self.child_output.terminal_is_empty()&&self.output.terminal_is_empty()&&self.source.terminal_is_empty()}
}

/// 🗺️ A declared tuple variant maps its completed typed record under a separate metadata turn.
pub struct MappedReceiving<T:DslOwnedReceivingField,U:RetireOwned>{current:OwnedSlot<FieldValue>,child_output:OwnedSlot<T>,output:OwnedSlot<U>,child:Option<Box<T::Cursor>>,map:fn(T)->U,initialized:bool,complete:bool,closing:bool}
impl<T:DslOwnedReceivingField,U:RetireOwned> MappedReceiving<T,U>{
 /// 🌿️ Captures the static variant constructor without allocating an owned variant roster.
 pub fn new(map:fn(T)->U)->Self{Self{current:Default::default(),child_output:Default::default(),output:Default::default(),child:None,map,initialized:false,complete:false,closing:false}}
}
impl<T:DslOwnedReceivingField,U:RetireOwned> OwnedFieldReceiving<U> for MappedReceiving<T,U>{
 fn demands(&self,_:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if !self.initialized{return Ok(child_demand::<T::Cursor>())}if self.output.value.is_some(){return Ok(RetirementDemand{release_bytes:self.child.as_ref().map_or(0,|_|size_of::<T::Cursor>()),depth:1,..Default::default()})}if self.child_output.value.is_some(){return Ok(metadata::<U>())}let child=self.child.as_ref().ok_or_else(invariant)?;if child.is_ready(){nested(metadata::<T>())}else{child.demands(&self.current.value,body).and_then(nested)}}
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.closing{return Err(invariant())}if self.complete{return Ok(Default::default())}let demand=self.demands(source,body)?;if !funds(grant,demand){return Ok(Default::default())}if let Some(child)=self.child.as_mut(){if self.output.value.is_none()&&self.child_output.value.is_none()&&!child.is_ready(){return child.step(&mut self.current.value,native,body,child_grant(grant))}}native.checkpoint()?;if !self.initialized{if source.is_none(){return Err(invalid())}let child=admit_child::<T::Cursor>(native)?;*self.current.value=source.take();self.child=Some(child);self.initialized=true;return Ok(receipt(demand))}if self.output.value.is_some(){if let Some(child)=self.child.as_ref(){if !child.terminal_is_empty(){return Err(invariant())}}let progress=release_child(&mut self.child,grant)?;self.complete=true;return Ok(progress)}if self.child_output.value.is_some(){*self.output.value=Some((self.map)(self.child_output.value.take().unwrap()));return Ok(receipt(demand))}self.child.as_mut().unwrap().take_into(&mut self.child_output.value,child_grant(grant))}
 fn is_ready(&self)->bool{!self.closing&&self.complete&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<U>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if output.is_some()||!self.is_ready(){return Err(invariant())}let demand=metadata::<U>();if !funds(grant,demand){return Ok(Default::default())}*output=self.output.value.take();Ok(receipt(demand))}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:2,..Default::default()})}else{child.close_demands(body).and_then(nested)}}if !self.current.terminal_is_empty(){return self.current.close_demands(body)}if !self.child_output.terminal_is_empty(){return self.child_output.close_demands(body)}self.output.close_demands(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}let demand=self.close_demands(grant.maximum_copy_bytes)?;if !funds(grant,demand){return Ok(Default::default())}self.closing=true;if let Some(child)=self.child.as_mut(){return if child.terminal_is_empty(){release_child(&mut self.child,grant)}else{child.close_step(child_grant(grant))}}if !self.current.terminal_is_empty(){return self.current.close_step(grant)}if !self.child_output.terminal_is_empty(){return self.child_output.close_step(grant)}self.output.close_step(grant)}
 fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.current.terminal_is_empty()&&self.child_output.terminal_is_empty()&&self.output.terminal_is_empty()}
}

#[path="🌳️intrinsic/🦀️.rs"]mod intrinsic;
pub use intrinsic::{NativeIntrinsicRecord,NativeIntrinsicReceiving};
