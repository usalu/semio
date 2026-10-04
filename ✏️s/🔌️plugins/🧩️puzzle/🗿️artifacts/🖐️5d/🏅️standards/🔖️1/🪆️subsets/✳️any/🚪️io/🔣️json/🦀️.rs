//! 🖐️ Declared Puzzle5d JSON preserves literal binary64 words at each owned field.
use semio_framework_value::DslValue;
fn member<'a>(v:&'a mut DslValue,key:&str)->Result<Option<&'a mut DslValue>,String>{match v{semio_framework_value::DslValue::Object(entries)=>Ok(entries.iter_mut().find(|(name,_)|name==key).map(|(_,v)|v)),_=>Err("Puzzle5d JSON requires an object".into())}}
fn field(v:&mut DslValue,key:&str,f:fn(&mut DslValue,bool)->Result<(),String>,decode:bool)->Result<(),String>{if let Some(v)=member(v,key)?{if !matches!(v,DslValue::Null){f(v,decode)?}}Ok(())}
fn rows(v:&mut DslValue,f:fn(&mut DslValue,bool)->Result<(),String>,decode:bool)->Result<(),String>{match v{semio_framework_value::DslValue::Array(values)=>{for v in values{f(v,decode)?}Ok(())},_=>Err("Puzzle5d JSON requires an array".into())}}
fn word(v:&mut DslValue,decode:bool)->Result<(),String>{
 if decode{let n=match v{semio_framework_value::DslValue::Number(n)=>{let n=n.as_f64();if !n.is_finite(){return Err("Puzzle5d JSON numeric input must be finite".into())}n},semio_framework_value::DslValue::Object(entries)=>{if entries.len()!=1||entries[0].0!="bits"{return Err("Puzzle5d JSON IEEE object requires only bits".into())}let raw=entries[0].1.as_str().ok_or("Puzzle5d JSON bits require text")?;if raw.len()!=16||!raw.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)){return Err("Puzzle5d JSON bits require sixteen lowercase hexadecimal digits".into())}f64::from_bits(u64::from_str_radix(raw,16).map_err(|error|error.to_string())?)},_=>return Err("Puzzle5d JSON binary64 input differs".into())};*v=semio_framework_value::DslValue::float(n)}
 else{let n=v.as_f64().ok_or("Puzzle5d JSON native binary64 is absent")?;*v=semio_framework_value::DslValue::object([("bits".into(),semio_framework_value::DslValue::String(format!("{:016x}",n.to_bits())))])}
 Ok(())
}
fn vector(v:&mut DslValue,decode:bool)->Result<(),String>{rows(v,word,decode)}
fn scale(v:&mut DslValue,decode:bool)->Result<(),String>{if matches!(v,DslValue::Array(_)){vector(v,decode)}else{word(v,decode)}}
fn part2d(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"x",word,decode)?;field(v,"y",word,decode)?;field(v,"radius",word,decode)?;field(v,"width",word,decode)?;field(v,"height",word,decode)}
fn part3d(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"origin",vector,decode)?;field(v,"orientation",vector,decode)?;field(v,"scale",scale,decode)}
fn grip2d(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"angle",word,decode)?;field(v,"radius",word,decode)}
fn grip3d(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"position",vector,decode)?;field(v,"direction",vector,decode)?;field(v,"radius",word,decode)}
fn grip(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"2d",grip2d,decode)?;field(v,"3d",grip3d,decode)}
fn grips(v:&mut DslValue,decode:bool)->Result<(),String>{rows(v,grip,decode)}
fn part(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"2d",part2d,decode)?;field(v,"3d",part3d,decode)?;field(v,"grips",grips,decode)}
fn parts(v:&mut DslValue,decode:bool)->Result<(),String>{rows(v,part,decode)}
fn fastener(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"gap",word,decode)?;field(v,"shift",word,decode)?;field(v,"rise",word,decode)?;field(v,"rotation",word,decode)?;field(v,"turn",word,decode)?;field(v,"tilt",word,decode)?;field(v,"x",word,decode)?;field(v,"y",word,decode)}
fn fasteners(v:&mut DslValue,decode:bool)->Result<(),String>{rows(v,fastener,decode)}
fn target(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"origin",vector,decode)?;field(v,"orientation",vector,decode)?;field(v,"scale",scale,decode)}
fn targets(v:&mut DslValue,decode:bool)->Result<(),String>{rows(v,target,decode)}
fn template(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"point",vector,decode)?;field(v,"direction",vector,decode)?;field(v,"t",word,decode)?;field(v,"radius",word,decode)}
fn templates(v:&mut DslValue,decode:bool)->Result<(),String>{rows(v,template,decode)}
fn kind(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"grips",templates,decode)}
fn kinds(v:&mut DslValue,decode:bool)->Result<(),String>{rows(v,kind,decode)}
fn extras(v:&mut DslValue,decode:bool)->Result<(),String>{field(v,"parts",kinds,decode)}
pub(crate) fn convert(mut v:DslValue,decode:bool)->Result<DslValue,String>{field(&mut v,"parts",parts,decode)?;field(&mut v,"fasteners",fasteners,decode)?;field(&mut v,"targetVolumes",targets,decode)?;field(&mut v,"kindCatalogsExtra",extras,decode)?;Ok(v)}

