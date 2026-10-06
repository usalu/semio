//! 🔢️ Role-aware Binary64Transport for the authored Block3d JSON fields.
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Block3d JSON binary64 transport differs")}
fn member<'a>(value:&'a mut DslValue,key:&str)->Option<&'a mut DslValue>{match value{DslValue::Object(entries)=>entries.iter_mut().find(|(name,_)|name==key).map(|(_,value)|value),_=>None}}
fn scalar(value:&mut DslValue,decode:bool)->Result<(),ValueError>{
 if decode{match value{
 DslValue::Object(entries) if entries.len()==1&&entries[0].0=="bits"=>{let DslValue::String(word)=&entries[0].1 else{return Err(invalid())};if word.len()!=16||!word.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b)){return Err(invalid())}let bits=u64::from_str_radix(word,16).map_err(|_|invalid())?;*value=DslValue::float(f64::from_bits(bits));Ok(())},
 DslValue::Number(Number::Float(value)) if !value.is_finite()=>Err(invalid()),
 DslValue::Number(_)=>Ok(()),
 _=>Err(invalid())
 }}else{let DslValue::Number(Number::Float(number))=value else{return Err(invalid())};let bits=number.to_bits();*value=DslValue::Object(vec![("bits".into(),DslValue::String(format!("{bits:016x}")))]);Ok(())}
}
fn fields(value:&mut DslValue,vector:&[&str],number:&str,decode:bool)->Result<(),ValueError>{for name in vector{if let Some(value)=member(value,name){let DslValue::Array(values)=value else{return Err(invalid())};for value in values{scalar(value,decode)?}}}if let Some(value)=member(value,number){scalar(value,decode)?}Ok(())}
/// 🚚️ Convert only the fourteen declared geometry roles before the owner codec runs.
pub(crate) fn convert(value:&mut DslValue,decode:bool)->Result<(),ValueError>{
 if let Some(vortices)=member(value,"vortices"){let DslValue::Array(vortices)=vortices else{return Err(invalid())};for vortex in vortices{fields(vortex,&["position","direction"],"radius",decode)?}}
 if let Some(camera)=member(value,"camera3d"){fields(camera,&["position","target"],"zoom",decode)?}
 Ok(())
}
