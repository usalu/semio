//! 🎞️ Controlled timeline, channel, keyframe and typed animation sample construction.
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use crate::schema::snapshot::{SemioAnimationSnapshot, AnimTargetProperty, AnimInterpolation, AnimValue, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use semio_framework_value::{native_decoding::NativeDecodeControl,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
/// 🛬️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<SemioAnimationSnapshot,ValueError>{
 crate::io::sqlite::snapshot::admit_layout(control.limits())?;
 native::with_body(payload,STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))
}
fn interpolation(tag:u8)->Result<AnimInterpolation,ValueError>{Ok(match tag{0=>AnimInterpolation::Linear,1=>AnimInterpolation::Step,2=>AnimInterpolation::CubicSpline,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native interpolation"))})}
fn f64(reader:&mut store::ByteReader<'_>)->Result<f64,ValueError>{reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))}

/// 🔷️ Selects each binary animation sample variant in its retained slot before copying its weights.
fn value_binary_into(output:&mut AnimValue,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{
 0=>{native::entities(entities,1,limits)?;*output=AnimValue::Scalar{value:f64(reader)?};},1=>{native::entities(entities,1,limits)?;*output=AnimValue::Vec3{value:SemioPoint3{x:f64(reader)?,y:f64(reader)?,z:f64(reader)?}};},2=>{native::entities(entities,1,limits)?;*output=AnimValue::Quat{value:SemioQuaternion{x:f64(reader)?,y:f64(reader)?,z:f64(reader)?,w:f64(reader)?}};},
 3=>{let count=native::length(reader)?;native::entities(entities,count,limits)?;*output=AnimValue::Weights{values:Vec::new()};let AnimValue::Weights{values}=&mut *output else{unreachable!()};*values=control.allocate_vec::<f64>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{values.push(f64(reader)?);control.step()?;}Ok::<_,ValueError>(())})?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native animation value"))}
 Ok(())
}
/// 🧩️ Fills the retained animation root from borrowed binary octets, inserting every timeline, channel and keyframe before its fields.
pub fn binary_into(output:&mut SemioAnimationSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio animation native format"))}native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.timelines,&mut reader,control,limits,&mut entities,Default::default,|timeline,reader,control,entities|{
  match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>{},1=>{timeline.name=Some(String::new());native::reader_text_into(timeline.name.as_mut().unwrap(),reader,control)?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native timeline name presence"))}
  native::binary_list_into(&mut timeline.channels,reader,control,limits,entities,Default::default,|channel,reader,control,entities|{
   native::reader_text_into(&mut channel.target.node,reader,control)?;match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>channel.target.property=AnimTargetProperty::Translation,1=>channel.target.property=AnimTargetProperty::Rotation,2=>channel.target.property=AnimTargetProperty::Scale,3=>channel.target.property=AnimTargetProperty::Weights,4=>{channel.target.property=AnimTargetProperty::Custom{name:String::new()};let AnimTargetProperty::Custom{name}=&mut channel.target.property else{unreachable!()};native::reader_text_into(name,reader,control)?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native target property"))}
   channel.interpolation=interpolation(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)?;
   native::binary_list_into(&mut channel.keyframes,reader,control,limits,entities,Default::default,|keyframe,reader,control,entities|{keyframe.t=f64(reader)?;value_binary_into(&mut keyframe.value,reader,control,limits,entities)})
  })
 })?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio animation native trailing bytes"))}Ok(())
}

/// 💠️ Selects each text animation sample variant in its retained slot before copying its weights.
fn value_text_into(output:&mut AnimValue,value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize)->Result<(),ValueError>{
 let(tag,body)=value.split_once(':').ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native animation value requires colon"))?;match tag{
 "S"=>{native::entities(entities,1,limits)?;*output=AnimValue::Scalar{value:native::float(body,control)?};},"V"=>{native::entities(entities,1,limits)?;let[x,y,z]=native::record(body,control)?;*output=AnimValue::Vec3{value:SemioPoint3{x:native::float(x,control)?,y:native::float(y,control)?,z:native::float(z,control)?}};},"Q"=>{native::entities(entities,1,limits)?;let[x,y,z,w]=native::record(body,control)?;*output=AnimValue::Quat{value:SemioQuaternion{x:native::float(x,control)?,y:native::float(y,control)?,z:native::float(z,control)?,w:native::float(w,control)?}};},
 "W"=>{let mut items=native::Items::new(body)?;let count=items.count(control,limits.max_rows)?;native::entities(entities,count,limits)?;*output=AnimValue::Weights{values:Vec::new()};let AnimValue::Weights{values}=&mut *output else{unreachable!()};*values=control.allocate_vec::<f64>(count)?;control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{values.push(native::float(item,control)?);control.step()?;}Ok::<_,ValueError>(())})?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native animation value"))}
 Ok(())
}
/// 📝️ Fills the retained animation root from the borrowed document body, inserting every timeline, channel and keyframe before its fields.
pub fn document_into(output:&mut SemioAnimationSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","timelines"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio animation native schema missing"))?,control)?;
 native::text_list_into(&mut output.timelines,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|timeline,item,control,entities|{
  let[name,channel_fields]=native::record(item,control)?;if name!="[0]"{let[tag,name]=native::record(name,control)?;if tag!="1"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native timeline name presence"))}timeline.name=Some(String::new());native::hex_text_into(timeline.name.as_mut().unwrap(),name,control)?;}
  native::text_list_into(&mut timeline.channels,channel_fields,control,limits,entities,Default::default,|channel,item,control,entities|{
   let[target,interpolation_fields,keyframe_fields]=native::record(item,control)?;let[node,property]=native::record(target,control)?;native::hex_text_into(&mut channel.target.node,node,control)?;match property{"t"=>channel.target.property=AnimTargetProperty::Translation,"r"=>channel.target.property=AnimTargetProperty::Rotation,"s"=>channel.target.property=AnimTargetProperty::Scale,"w"=>channel.target.property=AnimTargetProperty::Weights,value=>{let name=value.strip_prefix("c:").ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native target property"))?;channel.target.property=AnimTargetProperty::Custom{name:String::new()};let AnimTargetProperty::Custom{name:slot}=&mut channel.target.property else{unreachable!()};native::hex_text_into(slot,name,control)?;}}
   let tag=match interpolation_fields{"l"=>0,"s"=>1,"c"=>2,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native interpolation"))};channel.interpolation=interpolation(tag)?;
   native::text_list_into(&mut channel.keyframes,keyframe_fields,control,limits,entities,Default::default,|keyframe,item,control,entities|{let[t,value]=native::record(item,control)?;keyframe.t=native::float(t,control)?;value_text_into(&mut keyframe.value,value,control,limits,entities)})
  })
 })
}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioAnimationSnapshot{SemioAnimationSnapshot{schema:String::new(),timelines:Vec::new()}}
