//! 🎛️ Controlled inspector text admission into typed field color and dash samples.
/// ⌨️ Decode inspector input according to its field, preserving numeric-looking text.
pub fn parse_layer_field_input(field:&str,source:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{
 use semio_framework_value::{DslValue,Number,ValueRefusalKind};control.checkpoint()?;
 let parsed=match semio_framework_pack_json::from_json_str_controlled::<DslValue>(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control){Ok(value)=>Some(value),Err(error)if error.kind==ValueRefusalKind::InvalidValue=>None,Err(error)=>return Err(error)};
 if matches!(field,"fillColor"|"strokeColor"|"strokeDash"){
  let text=parsed.as_ref().and_then(DslValue::as_str).unwrap_or(source);
  if field=="strokeDash"{let samples=crate::standards::v1::subsets::any::io::text::dash::decode_dash_text(text,control)?;return match samples{None=>Ok(DslValue::Null),Some(samples)=>{let mut output=control.allocate_vec(samples.len())?;for sample in samples{control.step()?;output.push(DslValue::Number(Number::Float(sample)));}Ok(DslValue::Array(output))}};}
  let color=crate::standards::v1::subsets::any::io::text::color::decode_color_text(text,1.0,control)?;let mut output=control.allocate_vec(4)?;for component in color{control.step()?;output.push(DslValue::Number(Number::Float(component)));}return Ok(DslValue::Array(output));
 }
 if matches!(field,"imageKey"|"textContent"|"name"|"blendMode"|"fillRule"|"strokeCap"|"strokeJoin"|"booleanOperation"){if let Some(DslValue::String(text))=parsed{return Ok(DslValue::String(text));}return control.copy_text(source).map(DslValue::String);}
 match parsed{Some(value)=>Ok(value),None=>control.copy_text(source).map(DslValue::String)}
}

