//! 📖️ Immutable native source queries preserve the canonical borrowed key and scalar types.
pub use semio_framework_value_control_contract::{ValueError,ValueRefusalKind};
pub use semio_framework_value_intrinsic::{DslValue,Number};
pub use semio_framework_value_native_decoding::native_decoding::NativeDecodeControl;
use std::mem::size_of;
#[derive(Clone,Copy)]pub enum NativeRecordKey<'a>{Name(&'a str),Ordinal(u16)}
impl NativeRecordKey<'_>{pub fn comparison_bytes(self)->usize{match self{Self::Name(name)=>name.len(),Self::Ordinal(_)=>size_of::<u16>()}}}
#[derive(Clone,Copy)]pub struct NativeRecordField{pub name:&'static str,pub ordinal:u16}
impl NativeRecordField{pub fn matches(self,key:NativeRecordKey<'_>)->bool{match key{NativeRecordKey::Name(name)=>self.name==name,NativeRecordKey::Ordinal(ordinal)=>self.ordinal==ordinal}}}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeTreeKind{Null,Bool,Number,Text,Array,Object}
impl NativeTreeKind{pub const fn as_str(self)->&'static str{match self{Self::Null=>"null",Self::Bool=>"bool",Self::Number=>"number",Self::Text=>"text",Self::Array=>"array",Self::Object=>"object"}}}
/// ⏱️ Identifies one original source accessor and its exact indexed selection.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeSourceAccess{AsSource,TreeKind,EnumOrdinal,VariantTag,VariantPayload,IsNull,Boolean,Number,Text,RecordLen,RecordKey(usize),RecordName(usize),RecordEntry(usize),RecordChild(usize),SequenceLen,SequenceChild(usize)}
pub trait NativeValueSource:Sync+'static{
 /// ⛽️ Quotes the selected original accessor without performing it or scanning the source.
 fn accessor_work_demand(&self,access:NativeSourceAccess)->Result<u64,ValueError>;
 fn as_source(&self)->&dyn NativeValueSource;
 fn tree_kind(&self)->Option<NativeTreeKind>;
 fn enum_ordinal(&self)->Result<u32,ValueError>{Err(invalid())}
 fn variant_tag(&self)->Option<&str>{None}
 fn variant_payload(&self)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn is_null(&self)->bool{false}
 fn boolean(&self)->Result<bool,ValueError>{Err(invalid())}
 fn number(&self)->Result<Number,ValueError>{Err(invalid())}
 fn text(&self)->Result<&String,ValueError>{Err(invalid())}
 fn record_len(&self)->Result<usize,ValueError>{Err(invalid())}
 fn record_key(&self,_index:usize)->Result<NativeRecordKey<'_>,ValueError>{Err(invalid())}
 fn record_name(&self,index:usize)->Result<&str,ValueError>{match self.record_key(index)?{NativeRecordKey::Name(name)=>Ok(name),NativeRecordKey::Ordinal(_)=>Err(invalid())}}
 fn record_entry(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn record_child(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
 fn sequence_len(&self)->Result<usize,ValueError>{Err(invalid())}
 fn sequence_child(&self,_index:usize)->Result<&dyn NativeValueSource,ValueError>{Err(invalid())}
}
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native receiving original type, field or phase is invalid")}
impl NativeValueSource for DslValue{
 fn accessor_work_demand(&self,access:NativeSourceAccess)->Result<u64,ValueError>{Ok(if matches!(access,NativeSourceAccess::RecordName(_)){2}else{1})}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{match self{Self::Null=>Some(NativeTreeKind::Null),Self::Bool(_)=>Some(NativeTreeKind::Bool),Self::Number(_)=>Some(NativeTreeKind::Number),Self::String(_)=>Some(NativeTreeKind::Text),Self::Array(_)=>Some(NativeTreeKind::Array),Self::Object(_)=>Some(NativeTreeKind::Object),_=>None}}
 fn is_null(&self)->bool{matches!(self,Self::Null)}
 fn boolean(&self)->Result<bool,ValueError>{match self{Self::Bool(value)=>Ok(*value),_=>Err(invalid())}}
 fn number(&self)->Result<Number,ValueError>{match self{Self::Number(value)=>Ok(*value),_=>Err(invalid())}}
 fn text(&self)->Result<&String,ValueError>{match self{Self::String(value)=>Ok(value),_=>Err(invalid())}}
 fn record_len(&self)->Result<usize,ValueError>{match self{Self::Object(entries)=>Ok(entries.len()),_=>Err(invalid())}}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|(key,_)|NativeRecordKey::Name(key.as_str())).ok_or_else(invalid),_=>Err(invalid())}}
 fn record_entry(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|entry|entry as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Object(entries)=>entries.get(index).map(|(_,value)|value as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
 fn sequence_len(&self)->Result<usize,ValueError>{match self{Self::Array(entries)=>Ok(entries.len()),_=>Err(invalid())}}
 fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match self{Self::Array(entries)=>entries.get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid),_=>Err(invalid())}}
}
impl NativeValueSource for String{fn accessor_work_demand(&self,access:NativeSourceAccess)->Result<u64,ValueError>{Ok(if matches!(access,NativeSourceAccess::RecordName(_)){2}else{1})}fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Text)}fn text(&self)->Result<&String,ValueError>{Ok(self)}}
impl NativeValueSource for bool{fn accessor_work_demand(&self,access:NativeSourceAccess)->Result<u64,ValueError>{Ok(if matches!(access,NativeSourceAccess::RecordName(_)){2}else{1})}fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Bool)}fn boolean(&self)->Result<bool,ValueError>{Ok(*self)}}
impl NativeValueSource for u8{fn accessor_work_demand(&self,access:NativeSourceAccess)->Result<u64,ValueError>{Ok(if matches!(access,NativeSourceAccess::RecordName(_)){2}else{1})}fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Number)}fn number(&self)->Result<Number,ValueError>{Ok(Number::UInt(u64::from(*self)))}}
impl NativeValueSource for Number{fn accessor_work_demand(&self,access:NativeSourceAccess)->Result<u64,ValueError>{Ok(if matches!(access,NativeSourceAccess::RecordName(_)){2}else{1})}fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Number)}fn number(&self)->Result<Number,ValueError>{Ok(*self)}}
impl<T:NativeValueSource> NativeValueSource for (String,T){fn accessor_work_demand(&self,access:NativeSourceAccess)->Result<u64,ValueError>{Ok(if matches!(access,NativeSourceAccess::RecordName(_)){2}else{1})}fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Array)}fn variant_tag(&self)->Option<&str>{Some(&self.0)}fn variant_payload(&self)->Result<&dyn NativeValueSource,ValueError>{Ok(&self.1)}fn sequence_len(&self)->Result<usize,ValueError>{Ok(2)}fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match index{0=>Ok(&self.0),1=>Ok(&self.1),_=>Err(invalid())}}}
impl<T:NativeValueSource> NativeValueSource for Vec<T>{fn accessor_work_demand(&self,access:NativeSourceAccess)->Result<u64,ValueError>{Ok(if matches!(access,NativeSourceAccess::RecordName(_)){2}else{1})}fn as_source(&self)->&dyn NativeValueSource{self}fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Array)}fn sequence_len(&self)->Result<usize,ValueError>{Ok(self.len())}fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{self.get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid)}}
pub fn receiving_capacity(control:&NativeDecodeControl<'_>,bytes:usize)->Result<(),ValueError>{if control.owned_bytes().checked_add(bytes).is_none_or(|next|next>control.maximum_bytes()){Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decoding ownership exceeds caller limit"))}else{Ok(())}}

