//! 🖼️ Canvas dimension edits stay inside the Drawing owner and preserve every unrelated original leaf.
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Drawing canvas dimensions or admitted source are invalid")}
/// 📐️ Changes only the two scalar fields of an already admitted owned Drawing payload.
pub fn set_size(content:&mut DslValue,width:f64,height:f64)->Result<(),ValueError>{
 if !width.is_finite()||!height.is_finite()||width<0.0||height<0.0{return Err(invalid())}if content.get("schema").and_then(DslValue::as_str)!=Some("stdio.semio.drawing"){return Err(invalid())}let DslValue::Object(root)=content else{return Err(invalid())};let Some((_,DslValue::Object(canvas)))=root.iter_mut().find(|(key,_)|key=="canvas")else{return Err(invalid())};let w=canvas.iter().position(|(key,_)|key=="width").ok_or_else(invalid)?;let h=canvas.iter().position(|(key,_)|key=="height").ok_or_else(invalid)?;canvas[w].1=DslValue::Number(Number::Float(width));canvas[h].1=DslValue::Number(Number::Float(height));Ok(())
}
