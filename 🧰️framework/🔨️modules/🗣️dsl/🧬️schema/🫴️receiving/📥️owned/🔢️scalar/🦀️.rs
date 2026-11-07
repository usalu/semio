//! 🔢️ Known scalar creation and existing-owner moves report their different truthful payload effects.
use super::*;
#[doc(hidden)]
pub mod scalar_sealed{pub trait Sealed{}}
/// ✅️ These first-party leaves cannot hide arbitrary Default or conversion callbacks inside one work unit.
pub trait OwnedScalar:scalar_sealed::Sealed+RetireOwned+Sized{fn creation_bytes()->usize;fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>;}
/// 📬️ A scalar output stays resident through paid handoff or its actual close frontier.
pub struct ScalarReceiving<T:OwnedScalar>{output:OwnedSlot<T>,used:bool,failed:bool}
impl<T:OwnedScalar>Default for ScalarReceiving<T>{fn default()->Self{Self{output:Default::default(),used:false,failed:false}}}
impl<T:OwnedScalar>OwnedFieldReceiving<T>for ScalarReceiving<T>{
 fn demands(&self,_:&Option<FieldValue>,_:usize)->Result<RetirementDemand,ValueError>{Ok(if self.used||self.failed{Default::default()}else{RetirementDemand{copy_bytes:T::creation_bytes(),depth:1,..Default::default()}})}
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{if self.failed{return RetainedWorkOutcome{result:Err(invalid().into()),work:0}}if self.used{return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let d=self.demands(source,body).unwrap();if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if let Err(e)=native.checkpoint(){return RetainedWorkOutcome{result:Err(e.into()),work:0}}match T::take_source(source){Ok(value)=>{*self.output.value=Some(value);self.used=true;RetainedWorkOutcome{result:Ok(receipt(d)),work:1}},Err(e)=>{self.failed=true;RetainedWorkOutcome{result:Err(e.with_retained_progress(receipt(metadata::<()>())).into()),work:1}}}}
 fn is_ready(&self)->bool{self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<T>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{move_into(&mut self.output.value,output,g)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.output.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{self.output.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.output.terminal_is_empty()}
}
macro_rules! integer{($variant:ident;$($ty:ty),+)=>{$(impl scalar_sealed::Sealed for $ty{}impl OwnedScalar for $ty{fn creation_bytes()->usize{std::mem::size_of::<Self>()}fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{let Some(FieldValue::$variant(value))=source.as_ref()else{return Err(invalid())};let output=Self::try_from(*value).map_err(|_|invalid())?;source.take();Ok(output)}}impl DslOwnedReceivingField for $ty{type Cursor=ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{ScalarReceiving::default()}})+};}
integer!(Int;i8,i16,i32,i64,isize);
integer!(UInt;u16,u32,u64,usize);
impl scalar_sealed::Sealed for u8{}impl OwnedScalar for u8{fn creation_bytes()->usize{std::mem::size_of::<Self>()}fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{let Some(FieldValue::UInt(value))=source.as_ref()else{return Err(invalid())};let output=Self::try_from(*value).map_err(|_|invalid())?;source.take();Ok(output)}}
impl DslOwnedReceivingField for u8{type Cursor=ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{ScalarReceiving::default()}}
macro_rules! copied{($ty:ty,$variant:ident)=>{impl scalar_sealed::Sealed for $ty{}impl OwnedScalar for $ty{fn creation_bytes()->usize{std::mem::size_of::<Self>()}fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{let Some(FieldValue::$variant(value))=source.as_ref()else{return Err(invalid())};let output=*value;source.take();Ok(output)}}impl DslOwnedReceivingField for $ty{type Cursor=ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{ScalarReceiving::default()}}};}
copied!(bool,Bool);copied!(f64,Float);
impl scalar_sealed::Sealed for f32{}impl OwnedScalar for f32{fn creation_bytes()->usize{std::mem::size_of::<Self>()}fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{let Some(FieldValue::Float(value))=source.as_ref()else{return Err(invalid())};let bits=value.to_bits();let output=if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err(invalid())}Self::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff))}else{*value as Self};source.take();Ok(output)}}
impl DslOwnedReceivingField for f32{type Cursor=ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{ScalarReceiving::default()}}
macro_rules! moved{($ty:ty,$variant:ident)=>{impl scalar_sealed::Sealed for $ty{}impl OwnedScalar for $ty{fn creation_bytes()->usize{0}fn take_source(source:&mut Option<FieldValue>)->Result<Self,ValueError>{if !matches!(source,Some(FieldValue::$variant(_))){return Err(invalid())}let Some(FieldValue::$variant(value))=source.take()else{unreachable!()};Ok(value)}}impl DslOwnedReceivingField for $ty{type Cursor=ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{ScalarReceiving::default()}}};}
moved!(crate::RecordBytes,Bytes64);moved!(String,Text);moved!(semio_framework_value::DslValue,Value);moved!(crate::WireValue,Wire);moved!(crate::ExprValue,Expr);moved!(RecordValue,Record);

/// 🔁️ A newtype keeps its inner scalar's work accounting and only wraps the already-validated value.
#[doc(hidden)]
pub fn map_outcome<A,B>(outcome:RetainedWorkOutcome<A,crate::receiving::NativeSnapshotFailure>,wrap:fn(A)->B)->RetainedWorkOutcome<B,crate::receiving::NativeSnapshotFailure>{RetainedWorkOutcome{result:outcome.result.map(wrap),work:outcome.work}}
/// 🧬️ One declaration gives a tuple newtype over a first-party scalar both receiving families; see [`OwnedScalar`].
#[macro_export]
macro_rules! dsl_scalar_newtype_receiving{
 ($name:ty,$inner:ty)=>{
  impl $crate::receiving::owned::scalar_sealed::Sealed for $name{}
  impl $crate::receiving::owned::OwnedScalar for $name{
   fn creation_bytes()->usize{<$inner as $crate::receiving::owned::OwnedScalar>::creation_bytes()}
   fn take_source(source:&mut Option<$crate::FieldValue>)->Result<Self,$crate::receiving::owned::ValueError>{<$inner as $crate::receiving::owned::OwnedScalar>::take_source(source).map(Self)}
  }
  impl $crate::receiving::owned::DslOwnedReceivingField for $name{type Cursor=$crate::receiving::owned::ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl $crate::DslReceivingField for $name{fn receive_value(value:&$crate::FieldValue,owner:&mut $crate::receiving::NativeSnapshotDecodeOwner<'_,'_>)->$crate::receiving::owned::RetainedWorkOutcome<Self,$crate::receiving::NativeSnapshotFailure>{$crate::receiving::owned::map_outcome(<$inner as $crate::DslReceivingField>::receive_value(value,owner),Self)}}
 };
}
