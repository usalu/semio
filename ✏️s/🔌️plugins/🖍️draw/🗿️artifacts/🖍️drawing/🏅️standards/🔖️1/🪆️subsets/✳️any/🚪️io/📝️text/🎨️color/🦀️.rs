//! 🎨️ Controlled physical hexadecimal RGB spelling and normalized RGBA admission.
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Expected #RGB or #RRGGBB and finite unit alpha")}
pub fn decode_color_text(source:&str,alpha:f64,control:&mut NativeDecodeControl<'_>)->Result<[f64;4],ValueError>{
 control.checkpoint()?;let bytes=source.as_bytes();if !alpha.is_finite()||!(0.0..=1.0).contains(&alpha)||!matches!(bytes.len(),4|7)||bytes[0]!=b'#'{return Err(invalid());}
 let nibble=|byte:u8|match byte{b'0'..=b'9'=>Some(byte-b'0'),b'a'..=b'f'=>Some(byte-b'a'+10),b'A'..=b'F'=>Some(byte-b'A'+10),_=>None};
 let mut color=[0.0,0.0,0.0,alpha];for index in 0..3{control.step()?;let offset=if bytes.len()==4{index+1}else{index*2+1};let high=nibble(bytes[offset]).ok_or_else(invalid)?;let low=if bytes.len()==4{high}else{nibble(bytes[offset+1]).ok_or_else(invalid)?};color[index]=f64::from(high*16+low)/255.0;}Ok(color)
}
pub fn encode_color_text(color:[f64;4],control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{
 control.checkpoint()?;if color.iter().any(|value|!value.is_finite()){return Err(invalid());}let mut bytes=[b'#',0,0,0,0,0,0];let digits=b"0123456789abcdef";for index in 0..3{control.step()?;let value=(color[index].clamp(0.0,1.0)*255.0).round()as u8;bytes[index*2+1]=digits[usize::from(value>>4)];bytes[index*2+2]=digits[usize::from(value&15)];}control.copy_text(std::str::from_utf8(&bytes).map_err(|_|invalid())?)
}
