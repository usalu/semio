//! 🔌️ Declared Jack JSON camera fields own closed exact binary64 words; decoding also admits a plain number, as the artifact
//! schema's `Binary64Transport` does.
use semio_framework_value::DslValue;
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v1::subsets::any::schema::json_transport::convert;
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
struct JsonView<'a>(&'a crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot);
impl semio_framework_value::ToValue for JsonView<'_>{
 fn to_value(&self)->DslValue{convert(semio_framework_value::ToValue::to_value(self.0),false).expect("declared native parent JSON")}
 fn to_value_controlled(&self,c:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
  let mut root=semio_framework_value::ToValue::to_value_controlled(self.0,c)?.guard_encoded();
  let DslValue::Object(fields)=root.get_mut()else{return Err(invalid("Jack parent object required"))};
  let Some((_,DslValue::Object(axes)))=fields.iter_mut().find(|(key,_)|key=="camera")else{return Err(invalid("Jack parent camera required"))};
  for(_,axis)in axes{let bits=axis.as_f64().ok_or_else(||invalid("Jack camera scalar required"))?.to_bits();let mut bytes=[0u8;16];for(index,byte)in bytes.iter_mut().enumerate(){*byte=b"0123456789abcdef"[((bits>>((15-index)*4))&15)as usize];}let text=std::str::from_utf8(&bytes).map_err(|_|invalid("Jack camera hex word invalid"))?;let child=semio_framework_value::ToValue::to_value_controlled(&text,c)?;let mut word=DslValue::object_encoding_controlled(1,c)?;DslValue::push_encoding_controlled(word.get_mut(),"bits",child,c)?;*axis=DslValue::Object(word.take());}
  Ok(root.take())
 }
}
struct JsonOwned(crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot);
impl semio_framework_value::FromValue for JsonOwned{
 fn from_value(value:DslValue)->Result<Self,ValueError>{semio_framework_value::FromValue::from_value(convert(value,true)?).map(Self)}
 fn from_value_controlled(value:&DslValue,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let fields=value.as_object().ok_or_else(||invalid("Jack JSON object required"))?;if fields.iter().any(|(key,_)|!["schema","name","manifestId","manifest","camera","content","rootNodeId","query"].contains(&key.as_str())){return Err(invalid("Jack JSON parent has unknown fields"))}for key in["schema","name","manifest","camera","content","query"]{if !fields.iter().any(|(name,_)|name==key){return Err(invalid("Jack JSON parent required field missing"))}}<crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot as semio_framework_value::FromValue>::from_value_controlled(value,c).map(Self)}
 fn retire_decoded(self){crate::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(self.0).expect("original JSON parent retirement")}
}

/// 📤️ Publishes exact native parent JSON under the original cumulative encoder.
pub fn encode_controlled(snapshot:&crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot,c:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(&JsonView(snapshot),c)}
/// 📥️ Receives exact native parent JSON under the original cumulative decoder.
pub fn decode_controlled(text:&str,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot,ValueError>{semio_framework_pack_json::from_json_str_controlled::<JsonOwned>(text,semio_framework_pack_json::JsonMemberPolicy::Reject,c).map(|value|value.0)}
