//! 🖊️ Controlled physical whitespace-separated stroke dash samples.
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Invalid bounded stroke dash text")}
pub fn decode_dash_text(source:&str,control:&mut NativeDecodeControl<'_>)->Result<Option<Vec<f64>>,ValueError>{
 control.checkpoint()?;if source.len()>128||!source.bytes().all(|byte|byte.is_ascii_digit()||matches!(byte,b'.'|b' '|b'\t'|b'\r'|b'\n')){return Err(invalid());}let mut output=control.allocate_vec(source.split_ascii_whitespace().count())?;for token in source.split_ascii_whitespace(){control.step()?;let sample=token.parse::<f64>().map_err(|_|invalid())?;if !sample.is_finite()||sample<0.0{return Err(invalid());}output.push(sample);}Ok(output.iter().any(|value|*value>0.0).then_some(output))
}
struct Text{bytes:[u8;128],length:usize}
impl std::fmt::Write for Text{fn write_str(&mut self,value:&str)->std::fmt::Result{let end=self.length.checked_add(value.len()).filter(|end|*end<=128).ok_or(std::fmt::Error)?;self.bytes[self.length..end].copy_from_slice(value.as_bytes());self.length=end;Ok(())}}
pub fn encode_dash_text(samples:Option<&[f64]>,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{
 use std::fmt::Write;control.checkpoint()?;let mut text=Text{bytes:[0;128],length:0};if let Some(samples)=samples{for(index,sample)in samples.iter().enumerate(){control.step()?;if !sample.is_finite()||*sample<0.0{return Err(invalid());}if index>0{text.write_str(" ").map_err(|_|invalid())?;}write!(text,"{}",if *sample==0.0{0.0}else{*sample}).map_err(|_|invalid())?;}}control.copy_text(std::str::from_utf8(&text.bytes[..text.length]).map_err(|_|invalid())?)
}
