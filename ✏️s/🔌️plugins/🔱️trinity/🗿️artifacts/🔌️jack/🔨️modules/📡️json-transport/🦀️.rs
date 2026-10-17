//! 📡️ Jack JSON transport: the closed exact binary64 words of the camera fields on the parent object, decoded from a native number or a closed word.

use semio_framework_value::DslValue;
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v1::subsets::any::schema::graph::Camera;
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
pub fn convert(mut value: DslValue, decode: bool) -> Result<DslValue, ValueError> {
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
pub fn decode_camera(value:&DslValue,c:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Camera,ValueError>{
 c.scoped_stage(|c|{c.begin_stage(3)?;let fields=value.object_controlled(c)?;if fields.len()!=3||fields.iter().any(|(key,_)|!["x","y","zoom"].contains(&key.as_str())){return Err(invalid("Jack camera requires exactly its three axes"))}c.charge(size_of::<Camera>())?;
 let mut axis=|key:&str|{let value=DslValue::field_controlled(fields,key,c)?.ok_or_else(||invalid("Jack camera axis missing"))?;let number=if let Some(number)=value.as_f64(){number}else{let fields=value.as_object().ok_or_else(||invalid("Jack camera word required"))?;if fields.len()!=1||fields[0].0!="bits"{return Err(invalid("Jack camera word fields"))}let bits=fields[0].1.as_str().ok_or_else(||invalid("Jack camera bits text required"))?;if bits.len()!=16||!bits.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)){return Err(invalid("Jack camera lowercase hex64 required"))}f64::from_bits(u64::from_str_radix(bits,16).map_err(|_|invalid("Jack camera bits invalid"))?)};c.step()?;Ok(number)};
 Ok(Camera{x:axis("x")?,y:axis("y")?,zoom:axis("zoom")?})})
}
