//! 🛬️ Text record receiving borrows actual ordinal fields under the same required native construction port.
use super::{FieldValue,RecordValue};
use semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource;
use semio_framework_value::{Number, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::{native_decoding::owned::NativeRecordKey, native_decoding::owned::NativeRecordSource};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native text record member has a different original type or rank")}
impl NativeRecordSource for RecordValue{
 fn field_count(&self)->Result<usize,ValueError>{Ok(self.fields.len())}
 fn field_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{self.fields.get_index(index).map(|(id,_)|NativeRecordKey::Ordinal(*id)).ok_or_else(invalid)}
 fn text_field(&self,index:usize)->Result<&String,ValueError>{match self.fields.get_index(index){Some((_,FieldValue::Text(value)))=>Ok(value),_=>Err(invalid())}}
 fn number_field(&self,index:usize)->Result<Number,ValueError>{match self.fields.get_index(index){Some((_,FieldValue::Int(value)))=>Ok(Number::Int(*value)),Some((_,FieldValue::UInt(value)))=>Ok(Number::UInt(*value)),Some((_,FieldValue::Float(value)))=>Ok(Number::Float(*value)),_=>Err(invalid())}}
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

impl NativeValueSource for RecordValue{
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn record_len(&self)->Result<usize,ValueError>{Ok(self.fields.len())}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{self.fields.get_index(index).map(|(id,_)|NativeRecordKey::Ordinal(*id)).ok_or_else(invalid)}
 fn record_child(&self,index:usize)->Result<&dyn NativeValueSource,ValueError>{self.fields.get_index(index).map(|(_,value)|value as &dyn NativeValueSource).ok_or_else(invalid)}
}
fn unblocked(value:&FieldValue)->&FieldValue{if let FieldValue::Block(inner)=value{inner}else{value}}
impl NativeValueSource for FieldValue{
 fn as_source(&self)->&dyn NativeValueSource{self}
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
