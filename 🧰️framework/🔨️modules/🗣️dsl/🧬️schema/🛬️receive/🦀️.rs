//! 🛬️ Text record receiving borrows actual ordinal fields under the same required native construction port.
use super::{FieldValue,RecordValue};
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
