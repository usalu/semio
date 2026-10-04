//! 🔣️ Declared Remodeling JSON preserves every literal owned scalar word.
use semio_framework_value::DslValue;
use semio_framework_value::{FromValue, Number, ToValue};

fn member<'a>(value:&'a mut DslValue,key:&str)->Result<Option<&'a mut DslValue>,String>{match value{semio_framework_value::DslValue::Object(entries)=>Ok(entries.iter_mut().find(|(name,_)|name==key).map(|(_,value)|value)),_=>Err("Remodeling JSON requires an object".into())}}
fn field(value:&mut DslValue,key:&str,convert:fn(&mut DslValue,bool)->Result<(),String>,decode:bool)->Result<(),String>{if let Some(value)=member(value,key)?{if !matches!(value,DslValue::Null){convert(value,decode)?}}Ok(())}
fn rows(value:&mut DslValue,convert:fn(&mut DslValue,bool)->Result<(),String>,decode:bool)->Result<(),String>{match value{semio_framework_value::DslValue::Array(rows)=>{for row in rows{convert(row,decode)?}Ok(())},_=>Err("Remodeling JSON requires an array".into())}}
fn word(value:&DslValue,digits:usize)->Result<u64,String>{let semio_framework_value::DslValue::Object(fields)=value else{return Err("Remodeling JSON IEEE word requires an object".into())};if fields.len()!=1||fields[0].0!="bits"{return Err("Remodeling JSON IEEE word requires only bits".into())}let raw=fields[0].1.as_str().ok_or("Remodeling JSON IEEE bits require text")?;if raw.len()!=digits||!raw.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)){return Err(format!("Remodeling JSON IEEE word requires {digits} lowercase hexadecimal digits"))}u64::from_str_radix(raw,16).map_err(|error|error.to_string())}
fn float64(value:&mut DslValue,decode:bool)->Result<(),String>{
 if decode{let number=match value{semio_framework_value::DslValue::Number(value)=>{let number=value.as_f64();if !number.is_finite(){return Err("Remodeling JSON numeric binary64 must be finite".into())}number},_=>f64::from_bits(word(value,16)?)};*value=semio_framework_value::DslValue::float(number)}
 else{let number=value.as_f64().ok_or("Remodeling JSON native binary64 is absent")?;*value=semio_framework_value::DslValue::object([("bits".into(),semio_framework_value::DslValue::String(format!("{:016x}",number.to_bits())))])}
 Ok(())
}
fn float32(value:&mut DslValue,decode:bool)->Result<(),String>{
 if decode{let number=match value{semio_framework_value::DslValue::Number(value)=>{let wide=value.as_f64();let number=wide as f32;if !wide.is_finite()||!number.is_finite(){return Err("Remodeling JSON numeric binary32 must be finite and representable".into())}number},_=>f32::from_bits(word(value,8)? as u32)};*value=number.to_value()}
 else{let number=f32::from_value(std::mem::replace(value,semio_framework_value::DslValue::Null)).map_err(|error|error.to_string())?;*value=semio_framework_value::DslValue::object([("bits".into(),semio_framework_value::DslValue::String(format!("{:08x}",number.to_bits())))])}
 Ok(())
}
fn unsigned64(value:&mut DslValue,decode:bool)->Result<(),String>{
 if decode{let number=match value{
 semio_framework_value::DslValue::String(raw)=>{if raw.is_empty()||raw.len()>20||!raw.bytes().all(|byte|byte.is_ascii_digit())||(raw.len()>1&&raw.starts_with('0')){return Err("Remodeling JSON unsigned64 requires canonical decimal text".into())}raw.parse::<u64>().map_err(|error|error.to_string())?},
 semio_framework_value::DslValue::Number(Number::UInt(number)) if *number<=9007199254740991=>*number,
 semio_framework_value::DslValue::Number(Number::Int(number)) if *number>=0&&*number<=9007199254740991=>*number as u64,
 semio_framework_value::DslValue::Number(Number::Float(number)) if number.is_finite()&&*number>=0.0&&*number<=9007199254740991.0&&number.fract()==0.0=>*number as u64,
 _=>return Err("Remodeling JSON unsigned64 numeric input must be a safe unsigned integer".into())
 };*value=semio_framework_value::DslValue::uint(number)}
 else{*value=semio_framework_value::DslValue::String(value.as_u64().ok_or("Remodeling JSON native unsigned64 is absent")?.to_string())}
 Ok(())
}
fn signed64(value:&mut DslValue,decode:bool)->Result<(),String>{
 if decode{let number=match value{
 semio_framework_value::DslValue::String(raw)=>{let number=raw.parse::<i64>().map_err(|error|error.to_string())?;if number.to_string()!=*raw{return Err("Remodeling JSON signed64 requires canonical decimal text".into())}number},
 semio_framework_value::DslValue::Number(Number::Int(number)) if (-9007199254740991..=9007199254740991).contains(number)=>*number,
 semio_framework_value::DslValue::Number(Number::UInt(number)) if *number<=9007199254740991=>*number as i64,
 semio_framework_value::DslValue::Number(Number::Float(number)) if number.is_finite()&&number.abs()<=9007199254740991.0&&number.fract()==0.0=>*number as i64,
 _=>return Err("Remodeling JSON signed64 numeric input must be a safe integer".into())
 };*value=semio_framework_value::DslValue::int(number)}
 else{*value=semio_framework_value::DslValue::String(value.as_i64().ok_or("Remodeling JSON native signed64 is absent")?.to_string())}
 Ok(())
}
fn vector32(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,float32,decode)}
fn vector64(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,float64,decode)}
fn buffer(value:&mut DslValue,decode:bool)->Result<(),String>{let kind=member(value,"kind")?.and_then(|value|value.as_str()).ok_or("Remodeling JSON buffer kind is absent")?;match kind{"inline"=>field(value,"values",vector32,decode),"content"=>field(value,"chunkCount",unsigned64,decode),_=>Err("Remodeling JSON buffer kind differs".into())}}
fn stream(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"syncOffsetMs",float64,decode)?;field(value,"fpsHint",float64,decode)?;field(value,"source",video,decode)?;field(value,"frames",frames,decode)}
fn video(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"durationMs",float64,decode)}
fn frame(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"timestampMs",float64,decode)}
fn frames(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,frame,decode)}
fn streams(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,stream,decode)}
fn camera(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"fx",float64,decode)?;field(value,"fy",float64,decode)?;field(value,"cx",float64,decode)?;field(value,"cy",float64,decode)?;field(value,"skew",float64,decode)?;field(value,"distortion",vector32,decode)?;field(value,"rmsReprojectionPx",float32,decode)}
fn cameras(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,camera,decode)}
fn rig(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"rotationWxyz",vector32,decode)?;field(value,"translationM",vector32,decode)}
fn rigs(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,rig,decode)}
fn calibration(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"cameras",cameras,decode)?;field(value,"rig",rigs,decode)}
fn observation(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"pixel",vector32,decode)}
fn observations(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,observation,decode)}
fn gcp(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"worldPosition",vector64,decode)?;field(value,"observations",observations,decode)}
fn gcps(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,gcp,decode)}
fn ingest(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"minSharpness",float32,decode)}
fn feature(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"edgeThreshold",float32,decode)}
fn matching(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"ratioTest",float32,decode)}
fn sfm(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"ransacThresholdPx",float32,decode)?;field(value,"huberDeltaPx",float32,decode)}
fn dense_params(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"confidenceThreshold",float32,decode)}
fn mesh_params(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"tsdfVoxelSizeMm",float32,decode)?;field(value,"tsdfTruncationMm",float32,decode)}
fn motion(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"minTrackQuality",float32,decode)}
fn geo_params(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"originLon",float64,decode)?;field(value,"originLat",float64,decode)?;field(value,"originAlt",float64,decode)?;field(value,"gsdM",float32,decode)?;field(value,"dsmCellM",float32,decode)?;field(value,"dtmFilterRadiusM",float32,decode)}
fn params(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"ingest",ingest,decode)?;field(value,"feature",feature,decode)?;field(value,"matching",matching,decode)?;field(value,"sfm",sfm,decode)?;field(value,"dense",dense_params,decode)?;field(value,"mesh",mesh_params,decode)?;field(value,"motion",motion,decode)?;field(value,"geo",geo_params,decode)}
fn sparse(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"points",buffer,decode)}
fn dense(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"positions",buffer,decode)?;field(value,"confidence",buffer,decode)}
fn watertight(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"eulerCharacteristic",signed64,decode)?;field(value,"genus",signed64,decode)?;field(value,"signedVolume",float64,decode)}
fn mesh(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"watertight",watertight,decode)}
fn pose(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"rotationWxyz",vector32,decode)?;field(value,"translation",vector32,decode)}
fn poses(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,pose,decode)}
fn trajectory(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"poses",poses,decode)}
fn track(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"meanSpeedMS",float32,decode)}
fn tracks(value:&mut DslValue,decode:bool)->Result<(),String>{rows(value,track,decode)}
fn qc(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"reprojectionRmsPx",float64,decode)?;field(value,"gcpCheckpointRmse",float64,decode)?;field(value,"watertight",watertight,decode)?;field(value,"meanTrackLength",float32,decode)?;field(value,"registeredFrameRatio",float32,decode)?;field(value,"denseCoverageRatio",float32,decode)}
fn results(value:&mut DslValue,decode:bool)->Result<(),String>{field(value,"sparse",sparse,decode)?;field(value,"dense",dense,decode)?;field(value,"mesh",mesh,decode)?;field(value,"trajectory",trajectory,decode)?;field(value,"tracks",tracks,decode)?;field(value,"qc",qc,decode)}

pub(crate) fn convert(mut value:DslValue,decode:bool)->Result<DslValue,String>{field(&mut value,"streams",streams,decode)?;field(&mut value,"calibration",calibration,decode)?;field(&mut value,"params",params,decode)?;field(&mut value,"gcps",gcps,decode)?;field(&mut value,"results",results,decode)?;Ok(value)}

