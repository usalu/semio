//! 🛬️ Text record receiving borrows actual ordinal fields under the same required native construction port.
use super::{FieldValue,RecordValue};
use semio_framework_value_native_owned::native_decoding::owned::tree::{NativeValueSource,NativeTreeKind,NativeScalarQuery,NativeSourceScalar};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value::RetainedAllocationEffect;
use semio_framework_value::{Number, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::owned::{NativeRecordKey,NativeSchemaKeySource,NativeSourceKeyReply,NativeSourceKeyReplyComplete,NativeSourceKeyValue};
fn scalar<S:NativeValueSource+?Sized>(source:&S,query:NativeScalarQuery)->Result<NativeSourceScalar,ValueError>{match query{
 NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(source.tree_kind())),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(source.is_null())),NativeScalarQuery::Boolean=>source.boolean().map(NativeSourceScalar::Boolean),NativeScalarQuery::Number=>source.number().map(NativeSourceScalar::Number),NativeScalarQuery::EnumOrdinal=>source.enum_ordinal().map(NativeSourceScalar::EnumOrdinal),NativeScalarQuery::RecordLength=>source.record_len().map(NativeSourceScalar::Length),NativeScalarQuery::SequenceLength=>source.sequence_len().map(NativeSourceScalar::Length),
}}
macro_rules! scalar_methods{()=>{
 fn query_scalar<'reply>(&self,query:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'reply,NativeSourceScalar,ValueError>,ValueError>{
  if effect.maximum_work_units()==0{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query lacks its original work grant"))))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query exceeded its original work authority"))))}
  Ok(reply.complete(scalar(self,query)))
 }
 fn recover_scalar<'reply>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'reply,NativeSourceScalar,ValueError>,ValueError>{Ok(reply.pending())}
};}
macro_rules! schema_key_methods{($read:expr)=>{
 fn query_schema_key<'reply,'source>(&'source self,query:NativeSchemaKeySource,effect:&mut RetainedAllocationEffect<'_>,reply:NativeSourceKeyReply<'reply,'source>)->Result<NativeSourceKeyReplyComplete<'reply>,ValueError>{
  if let Some(inner)=($read)(self){return inner.query_schema_key(query,effect,reply)}
  if effect.remaining_work_units()?<1{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema key query lacks original Work"))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema key query exceeded original Work"))))}
  Ok(reply.complete(schema_key(self,query)))
 }
};}
fn schema_key<S:NativeValueSource+?Sized>(source:&S,query:NativeSchemaKeySource)->Result<NativeSourceKeyValue<'_>,ValueError>{
 use NativeSchemaKeySource as Q;
 match query{
  Q::Text=>source.text().map(|text|NativeSourceKeyValue::Name(text)),
  Q::EnumOrdinal=>source.enum_ordinal().map(NativeSourceKeyValue::Ordinal),
  Q::VariantTag=>source.variant_tag().map(NativeSourceKeyValue::Name).ok_or_else(invalid),
  Q::Record(index)=>match source.record_key(index)?{NativeRecordKey::Name(name)=>Ok(NativeSourceKeyValue::Name(name)),NativeRecordKey::Ordinal(id)=>Ok(NativeSourceKeyValue::Ordinal(u32::from(id)))},
  Q::RecordName(index)=>source.record_name(index).map(NativeSourceKeyValue::Name),
  Q::RecordText(index)=>source.record_child(index)?.text().map(|text|NativeSourceKeyValue::Name(text)),
 }
}
fn no_inner(_:&RecordValue)->Option<&dyn NativeValueSource>{None}
fn literal_inner(value:&FieldValue)->Option<&dyn NativeValueSource>{match unblocked(value){FieldValue::Value(inner)=>Some(inner),_=>None}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native text record member has a different original type or rank")}


impl NativeValueSource for RecordValue{
 scalar_methods!();schema_key_methods!(no_inner);
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Object)}
 fn record_len(&self)->Result<usize,ValueError>{Ok(self.fields.len())}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{self.fields.get_index(index).map(|(id,_)|NativeRecordKey::Ordinal(*id)).ok_or_else(invalid)}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{self.fields.get_index(index).map(|(_,value)|value as &dyn NativeValueSource).ok_or_else(invalid)}
}
fn unblocked(value:&FieldValue)->&FieldValue{if let FieldValue::Block(inner)=value{inner}else{value}}
impl NativeValueSource for FieldValue{
 scalar_methods!();schema_key_methods!(literal_inner);
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{if self.is_null(){return Some(NativeTreeKind::Null)}match unblocked(self){Self::Bool(_)=>Some(NativeTreeKind::Bool),Self::Int(_)|Self::UInt(_)|Self::Float(_)=>Some(NativeTreeKind::Number),Self::Text(_)=>Some(NativeTreeKind::Text),Self::List(_)|Self::Tuple(_)|Self::Statements(_)|Self::Bytes64(_)=>Some(NativeTreeKind::Array),Self::Record(_)|Self::Map(_)=>Some(NativeTreeKind::Object),Self::Value(value)=>value.tree_kind(),_=>None}}
 fn is_null(&self)->bool{matches!(unblocked(self),Self::Statements(values)if values.is_empty())||matches!(unblocked(self),Self::Absent)||matches!(unblocked(self),Self::Value(value)if value.is_null())}
 fn boolean(&self)->Result<bool,ValueError>{match unblocked(self){Self::Bool(value)=>Ok(*value),Self::Value(value)=>value.boolean(),_=>Err(invalid())}}
 fn number(&self)->Result<Number,ValueError>{match unblocked(self){Self::Int(value)=>Ok(Number::Int(*value)),Self::UInt(value)=>Ok(Number::UInt(*value)),Self::Float(value)=>Ok(Number::Float(*value)),Self::Value(value)=>value.number(),_=>Err(invalid())}}
 fn text(&self)->Result<&String,ValueError>{match unblocked(self){Self::Text(value)=>Ok(value),Self::Value(value)=>value.text(),_=>Err(invalid())}}
 fn variant_tag(&self)->Option<&str>{match unblocked(self){Self::Statements(values)if values.len()==1=>Some(&values[0].0),_=>None}}
 fn variant_payload(&self)->Result<&dyn NativeValueSource,ValueError>{match unblocked(self){Self::Statements(values)if values.len()==1=>Ok(&values[0].1),_=>Err(invalid())}}
 fn enum_ordinal(&self)->Result<u32,ValueError>{match unblocked(self){Self::Enum(value)=>Ok(*value),_=>Err(invalid())}}
 fn record_len(&self)->Result<usize,ValueError>{match unblocked(self){Self::Record(value)=>value.record_len(),Self::Map(values)=>Ok(values.len()),Self::Value(value)=>value.record_len(),_=>Err(invalid())}}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{match unblocked(self){Self::Record(value)=>value.record_key(index),Self::Map(values)=>values.get(index).map(|(key,_)|NativeRecordKey::Name(key)).ok_or_else(invalid),Self::Value(value)=>value.record_key(index),_=>Err(invalid())}}
 fn record_entry(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match unblocked(self){Self::Map(values)=>values.get(index).map(|entry|entry as &dyn NativeValueSource).ok_or_else(invalid),Self::Value(value)=>value.record_entry(index),_=>Err(invalid())}}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match unblocked(self){Self::Record(value)=>value.record_child(index),Self::Map(values)=>values.get(index).map(|(_,value)|value as &dyn NativeValueSource).ok_or_else(invalid),Self::Value(value)=>value.record_child(index),_=>Err(invalid())}}
 fn sequence_len(&self)->Result<usize,ValueError>{match unblocked(self){Self::List(values)|Self::Tuple(values)=>Ok(values.len()),Self::Statements(values)=>Ok(values.len()),Self::Bytes64(values)=>Ok(values.len()),Self::Value(value)=>value.sequence_len(),_=>Err(invalid())}}
 fn sequence_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{match unblocked(self){Self::List(values)|Self::Tuple(values)=>values.get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid),Self::Statements(values)=>values.get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid),Self::Bytes64(values)=>values.retained_bytes().get(index).map(|value|value as &dyn NativeValueSource).ok_or_else(invalid),Self::Value(value)=>value.sequence_child(index),_=>Err(invalid())}}
}
