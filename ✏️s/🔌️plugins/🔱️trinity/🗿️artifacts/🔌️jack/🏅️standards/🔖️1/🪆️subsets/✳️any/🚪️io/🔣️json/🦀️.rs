//! 🔌️ Declared Jack JSON camera fields own closed exact binary64 words; decoding also admits a plain number, as the artifact
//! schema's `Binary64Transport` does.
use semio_framework_value::DslValue;
use semio_framework_value::{ValueError, ValueRefusalKind};
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
pub(crate) fn convert(mut value: DslValue, decode: bool) -> Result<DslValue, ValueError> {
    let DslValue::Object(fields) = &mut value else { return Err(invalid("Jack JSON object required")) };
    if decode {
        if fields.iter().any(|(key, _)| !["schema", "name", "manifestId", "manifest", "camera", "content", "rootNodeId", "query"].contains(&key.as_str())) { return Err(invalid("Jack JSON parent has unknown fields")); }
        for key in ["schema", "name", "manifest", "camera", "content", "query"] { if !fields.iter().any(|(name, _)| name == key) { return Err(invalid(format!("Jack JSON parent requires {key}"))); } }
    }
    if let Some((_, camera)) = fields.iter_mut().find(|(key, _)| key == "camera") {
        let DslValue::Object(axes) = camera else { return Err(invalid("Jack JSON camera object required")) };
        for key in ["x", "y", "zoom"] {
            if let Some((_, axis)) = axes.iter_mut().find(|(name, _)| name == key) {
                if decode && axis.as_f64().is_some() {
                    continue;
                }
                if decode {
                    let DslValue::Object(word) = axis else { return Err(invalid("Jack JSON camera requires a closed word or a plain number")) };
                    if word.len() != 1 || word[0].0 != "bits" {
                        return Err(invalid("Jack JSON camera word fields"));
                    }
                    let bits = word[0].1.as_str().ok_or_else(|| invalid("Jack JSON camera bits require text"))?;
                    if bits.len() != 16 || !bits.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
                        return Err(invalid("Jack JSON camera bits require lowercase hex64"));
                    }
                    *axis = DslValue::float(f64::from_bits(u64::from_str_radix(bits, 16).map_err(|error| invalid(error.to_string()))?));
                } else {
                    let bits = axis.as_f64().ok_or_else(|| invalid("Jack owned camera scalar required"))?.to_bits();
                    *axis = DslValue::object([("bits".into(), DslValue::String(format!("{bits:016x}")))]);
                }
            }
        }
    }
    Ok(value)
}

/// 🎛️ Borrows a native or closed binary64 camera under the original caller's decode control.
pub(crate) fn decode_camera(value:&DslValue,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<crate::Camera,ValueError>{
 c.scoped_stage(|c|{c.begin_stage(3)?;let fields=value.object_controlled(c)?;if fields.len()!=3||fields.iter().any(|(key,_)|!["x","y","zoom"].contains(&key.as_str())){return Err(invalid("Jack camera requires exactly its three axes"))}c.charge(size_of::<crate::Camera>())?;
 let mut axis=|key:&str|{let value=DslValue::field_controlled(fields,key,c)?.ok_or_else(||invalid("Jack camera axis missing"))?;let number=if let Some(number)=value.as_f64(){number}else{let fields=value.as_object().ok_or_else(||invalid("Jack camera word required"))?;if fields.len()!=1||fields[0].0!="bits"{return Err(invalid("Jack camera word fields"))}let bits=fields[0].1.as_str().ok_or_else(||invalid("Jack camera bits text required"))?;if bits.len()!=16||!bits.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)){return Err(invalid("Jack camera lowercase hex64 required"))}f64::from_bits(u64::from_str_radix(bits,16).map_err(|_|invalid("Jack camera bits invalid"))?)};c.step()?;Ok(number)};
 Ok(crate::Camera{x:axis("x")?,y:axis("y")?,zoom:axis("zoom")?})})
}
struct JsonView<'a>(&'a crate::JackSnapshot);
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
struct JsonOwned(crate::JackSnapshot);
impl semio_framework_value::FromValue for JsonOwned{
 fn from_value(value:DslValue)->Result<Self,ValueError>{semio_framework_value::FromValue::from_value(convert(value,true)?).map(Self)}
 fn from_value_controlled(value:&DslValue,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{let fields=value.as_object().ok_or_else(||invalid("Jack JSON object required"))?;if fields.iter().any(|(key,_)|!["schema","name","manifestId","manifest","camera","content","rootNodeId","query"].contains(&key.as_str())){return Err(invalid("Jack JSON parent has unknown fields"))}for key in["schema","name","manifest","camera","content","query"]{if !fields.iter().any(|(name,_)|name==key){return Err(invalid("Jack JSON parent required field missing"))}}<crate::JackSnapshot as semio_framework_value::FromValue>::from_value_controlled(value,c).map(Self)}
 fn retire_decoded(self){crate::retire_owned_to_terminal(self.0).expect("original JSON parent retirement")}
}

/// 📤️ Publishes exact native parent JSON under the original cumulative encoder.
pub fn encode_controlled(snapshot:&crate::JackSnapshot,c:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<String,ValueError>{semio_framework_pack_json::to_json_string_controlled(&JsonView(snapshot),c)}
/// 📥️ Receives exact native parent JSON under the original cumulative decoder.
pub fn decode_controlled(text:&str,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<crate::JackSnapshot,ValueError>{semio_framework_pack_json::from_json_str_controlled::<JsonOwned>(text,semio_framework_pack_json::JsonMemberPolicy::Reject,c).map(|value|value.0)}
