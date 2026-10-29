//! 🧩️ Fixed native record members borrow genuine typed children across canonical, text and authored-value inputs.
use crate::{DslValue,Number,ValueError,ValueRefusalKind,retirement::RetireOwned,shared_issuance::SealedShared};
#[path="🧳️receiver/🦀️.rs"]mod receiver;
pub use receiver::NativeFlatRecordReceiver;
/// 🪪️ Original record identity is a borrowed canonical name or an actual declared text field ordinal.
#[derive(Clone,Copy)]pub enum NativeRecordKey<'a>{Name(&'a str),Ordinal(u16)}
impl NativeRecordKey<'_>{pub fn comparison_bytes(self)->usize{match self{Self::Name(name)=>name.len(),Self::Ordinal(_)=>std::mem::size_of::<u16>()}}}
/// 📋️ One native field declares both canonical and text identities without retaining a generated mirror.
#[derive(Clone,Copy)]pub struct NativeRecordField{pub name:&'static str,pub ordinal:u16}
impl NativeRecordField{pub fn matches(self,key:NativeRecordKey<'_>)->bool{match key{NativeRecordKey::Name(name)=>self.name==name,NativeRecordKey::Ordinal(ordinal)=>self.ordinal==ordinal}}}
/// 👓️ All children belong to this actual input owner; no owned proxy or field-value conversion is permitted.
pub trait NativeRecordSource:Sync+'static{
 fn field_count(&self)->Result<usize,ValueError>;
 fn field_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>;
 fn text_field(&self,index:usize)->Result<&String,ValueError>;
 fn number_field(&self,index:usize)->Result<Number,ValueError>;
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native record field has a different original type or rank")}
impl NativeRecordSource for DslValue{
 fn field_count(&self)->Result<usize,ValueError>{match self{Self::Object(entries)=>Ok(entries.len()),_=>Err(invalid())}}
 fn field_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|(key,_)|NativeRecordKey::Name(key)).ok_or_else(invalid),_=>Err(invalid())}}
 fn text_field(&self,index:usize)->Result<&String,ValueError>{match self{Self::Object(entries)=>match entries.get(index){Some((_,Self::String(value)))=>Ok(value),_=>Err(invalid())},_=>Err(invalid())}}
 fn number_field(&self,index:usize)->Result<Number,ValueError>{match self{Self::Object(entries)=>match entries.get(index){Some((_,Self::Number(value)))=>Ok(*value),_=>Err(invalid())},_=>Err(invalid())}}
}
/// 🏗️ Flat domain records declare typed validation and an infallible final ownership assembly.
pub trait NativeFlatRecord<const TEXTS:usize,const NUMBERS:usize>:RetireOwned+Send+'static{
 const TEXT_FIELDS:[NativeRecordField;TEXTS];
 const NUMBER_FIELDS:[NativeRecordField;NUMBERS];
 fn validation_demand(texts:[&str;TEXTS],numbers:[Number;NUMBERS])->crate::RetirementDemand;
 fn validate_parts(texts:[&str;TEXTS],numbers:[Number;NUMBERS])->Result<(),ValueError>;
 fn from_parts(texts:[SealedShared<String>;TEXTS],numbers:[Number;NUMBERS])->Self;
}
/// 🧱️ A declared flat record uses the same mandatory receiving owner for every supported genuine input.
#[macro_export]
macro_rules! native_flat_record_receive {
 ($row:ty,$texts:expr,$numbers:expr)=>{
  impl<I:$crate::native_decoding::owned::NativeRecordSource> $crate::native_decoding::NativeReceive<I> for $row {
   type Receiver=$crate::native_decoding::owned::NativeFlatRecordReceiver<I,Self,$texts,$numbers>;
   fn receive_birth_demand()->$crate::RetirementDemand{Self::Receiver::birth_demand()}
   fn begin_receive(grant:$crate::retained_clone::RetainedCloneGrant)->Result<(Self::Receiver,$crate::retained_clone::RetainedCloneProgress),$crate::ValueError>{Self::Receiver::begin(grant)}
  }
 };
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
