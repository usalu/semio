//! 🔣️ Declared Semio graph JSON binds only actual geometry words and complete intrinsic fields.
use semio_framework_value::{DslValue,FromValue,ToValue};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn fields(value:&mut DslValue,required:&[&str],optional:&[&str])->Result<(),ValueError>{
 let DslValue::Object(entries)=value else{return Err(invalid("Semio graph declared object required"))};
 if required.iter().any(|key|!entries.iter().any(|(name,_)|name==key))||entries.iter().any(|(name,_)|!required.contains(&name.as_str())&&!optional.contains(&name.as_str())){return Err(invalid("Semio graph declared fields invalid"))}Ok(())
}
fn member<'a>(value:&'a mut DslValue,key:&str)->Result<&'a mut DslValue,ValueError>{let DslValue::Object(entries)=value else{return Err(invalid("Semio graph object required"))};entries.iter_mut().find(|(name,_)|name==key).map(|(_,value)|value).ok_or_else(||invalid("Semio graph role missing"))}
fn word(value:&mut DslValue,decode:bool)->Result<(),ValueError>{if decode{
 fields(value,&["bits"],&[])?;let bits=member(value,"bits")?.as_str().ok_or_else(||invalid("Semio graph hex64 text required"))?;
 if bits.len()!=16||!bits.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)){return Err(invalid("Semio graph lowercase hex64 required"))}
 *value=DslValue::float(f64::from_bits(u64::from_str_radix(bits,16).map_err(|_|invalid("Semio graph word invalid"))?));
 }else{let bits=value.as_f64().ok_or_else(||invalid("Semio graph binary64 owner required"))?.to_bits();*value=DslValue::object([("bits".into(),DslValue::String(format!("{bits:016x}")))]);}Ok(())
}
pub fn convert(mut value:DslValue,decode:bool)->Result<DslValue,ValueError>{
 fields(&mut value,&["schema","nodes","edges"],&[])?;
 let DslValue::Array(nodes)=member(&mut value,"nodes")? else{return Err(invalid("Semio graph nodes required"))};
 for node in nodes{
  fields(node,&["id","kind","label","position","width","height","ports","properties"],&[])?;
  let point=member(node,"position")?;fields(point,&["x","y"],&[])?;word(member(point,"x")?,decode)?;word(member(point,"y")?,decode)?;word(member(node,"width")?,decode)?;word(member(node,"height")?,decode)?;
 }
 let DslValue::Array(edges)=member(&mut value,"edges")? else{return Err(invalid("Semio graph edges required"))};for edge in edges{fields(edge,&["id","source","target","kind","label","properties"],&["sourcePort","targetPort"])?;}Ok(value)
}
pub fn encode(snapshot:&super::SemioGraphSnapshot)->Result<String,ValueError>{let value=convert(snapshot.to_value(),false)?;Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))}
pub fn decode(text:&str)->Result<super::SemioGraphSnapshot,ValueError>{let parsed=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|invalid(&error.to_string()))?;super::SemioGraphSnapshot::from_value(convert(semio_framework_pack_json::to_dsl_value(&parsed),true)?)}
