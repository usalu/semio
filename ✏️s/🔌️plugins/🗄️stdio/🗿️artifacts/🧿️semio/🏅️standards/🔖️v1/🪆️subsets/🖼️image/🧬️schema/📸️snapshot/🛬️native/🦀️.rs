//! 🛬️ Typed image native construction with cumulative caller admission.
use super::{SemioImageSnapshot,SemioImageFrame,SemioImageMetadataEntry,SemioColorspace,STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_framework_value::native_decoding::{NativeDecodeControl,NativeDecodeProgress};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SqliteDatabaseLimits};

fn length(reader:&mut store::ByteReader<'_>)->Result<usize,String>{usize::try_from(reader.read_varint_u64().map_err(|e|e.to_string())?).map_err(|_|"image native length exceeds address space".into())}
fn octets<'a>(reader:&mut store::ByteReader<'a>)->Result<&'a[u8],String>{let count=length(reader)?;reader.read_bytes(count).map_err(|e|e.to_string())}
fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<String,String>{let value=control.borrow_text(octets(reader)?)?;control.copy_text(value)}
fn rows(count:usize,limits:SqliteDatabaseLimits)->Result<(),String>{if count>limits.max_rows{Err("image native entities exceed caller row limit".into())}else{Ok(())}}

/// 🖼️ Reads each named image field without delegating ownership to an uncontrolled codec.
pub(super) fn decode(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<SemioImageSnapshot,String>{
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
 let length=match payload{store::os_io::IoPayload::Binary(value)=>value.len(),store::os_io::IoPayload::Text(value)=>value.len()};if length>limits.max_file_bytes{return Err("image native input exceeds file byte limit".into());}
 let mut progress=|event:NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::DecodeNative,event.completed,event.total).is_ok();let mut native=NativeDecodeControl::new(limits.max_value_bytes,&mut progress);
 let snapshot=match payload{
 store::os_io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1,&mut native).map_err(|e|e.to_string())?;binary(body,&mut native,limits)?},
 store::os_io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA,store::semio_format::Component::Dsl,1,&mut native).map_err(|e|e.to_string())?;document(body,&mut native,limits)?}
 };native.checkpoint()?;Ok(snapshot)
}

pub(crate) fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioImageSnapshot,String>{
 let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|e|e.to_string())?!=1{return Err("unsupported image native format".into());}
 let schema=text(&mut reader,control)?;let width=reader.read_u32_le().map_err(|e|e.to_string())?;let height=reader.read_u32_le().map_err(|e|e.to_string())?;let colorspace=super::colorspace_from_tag(reader.read_u8().map_err(|e|e.to_string())?)?;let bit_depth=reader.read_u8().map_err(|e|e.to_string())?;
 let icc=match reader.read_u8().map_err(|e|e.to_string())?{0=>None,1=>Some(control.copy_bytes(octets(&mut reader)?)?),_=>return Err("invalid image native ICC presence".into())};
 let frame_count=length(&mut reader)?;rows(frame_count.checked_add(1).ok_or("image native entity overflow")?,limits)?;let mut frames=control.allocate_vec::<SemioImageFrame>(frame_count)?;control.begin_stage(frame_count)?;
 for _ in 0..frame_count{let delay_ms=reader.read_u32_le().map_err(|e|e.to_string())?;let rgba8=control.copy_bytes(octets(&mut reader)?)?;frames.push(SemioImageFrame{delay_ms,rgba8});control.step()?;}control.checkpoint()?;
 let metadata_count=length(&mut reader)?;rows(metadata_count.checked_add(frame_count).and_then(|v|v.checked_add(1)).ok_or("image native entity overflow")?,limits)?;let mut metadata=control.allocate_vec::<SemioImageMetadataEntry>(metadata_count)?;control.begin_stage(metadata_count)?;
 for _ in 0..metadata_count{let key=text(&mut reader,control)?;let value=text(&mut reader,control)?;metadata.push(SemioImageMetadataEntry{key,value});control.step()?;}control.checkpoint()?;
 if reader.remaining()!=0{return Err("image native trailing bytes".into());}Ok(SemioImageSnapshot{schema,width,height,colorspace,bit_depth,icc,frames,metadata})
}

#[derive(Clone)]
struct Items<'a>{remaining:&'a str}
impl<'a> Items<'a>{
 fn new(value:&'a str)->Result<Self,String>{let value=value.trim();Ok(Self{remaining:value.strip_prefix('[').and_then(|s|s.strip_suffix(']')).ok_or("image native list requires brackets")?})}
 fn next(&mut self,control:&mut NativeDecodeControl<'_>)->Result<Option<&'a str>,String>{
 if self.remaining.is_empty(){return Ok(None)}let mut depth=0usize;let mut end=self.remaining.len();
 for(index,byte)in self.remaining.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b'['=>{depth+=1;if depth>2{return Err("image native list depth exceeds field shape".into());}},b']'=>{depth=depth.checked_sub(1).ok_or("image native unmatched bracket")?;},b',' if depth==0=>{end=index;break;},_=>{}}}
 if depth!=0{return Err("image native unmatched bracket".into());}let value=self.remaining[..end].trim();self.remaining=if end<self.remaining.len(){&self.remaining[end+1..]}else{""};if value.is_empty(){return Err("image native empty list element".into());}Ok(Some(value))
 }
 fn count(&self,control:&mut NativeDecodeControl<'_>,maximum:usize)->Result<usize,String>{let mut items=self.clone();let mut count=0usize;while items.next(control)?.is_some(){count=count.checked_add(1).ok_or("image native list count overflow")?;if count>maximum{return Err("image native entities exceed caller row limit".into());}}Ok(count)}
}
fn pair<'a>(value:&'a str,control:&mut NativeDecodeControl<'_>)->Result<(&'a str,&'a str),String>{let inner=value.trim().strip_prefix('[').and_then(|s|s.strip_suffix(']')).ok_or("image native pair requires brackets")?;let mut delimiter=None;for(index,byte)in inner.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b','=>{if delimiter.replace(index).is_some(){return Err("image native pair has extra fields".into());}},b'['|b']'=>return Err("image native pair contains nested fields".into()),_=>{}}}let index=delimiter.ok_or("image native pair is missing a field")?;Ok((inner[..index].trim(),inner[index+1..].trim()))}
fn hex(value:&str,control:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,String>{
 if !value.len().is_multiple_of(2){return Err("image native odd hex length".into());}
 control.scoped_stage(|control|{let mut out=control.allocate_vec::<u8>(value.len()/2)?;control.begin_stage(value.len()/2)?;for chunk in value.as_bytes().chunks_exact(2){let digit=|v:u8|match v{b'0'..=b'9'=>Ok(v-b'0'),b'a'..=b'f'=>Ok(v-b'a'+10),b'A'..=b'F'=>Ok(v-b'A'+10),_=>Err("invalid image native hexadecimal")};out.push((digit(chunk[0])?<<4)|digit(chunk[1])?);control.step()?;}control.checkpoint()?;Ok(out)})
}
fn hex_text(value:&str,control:&mut NativeDecodeControl<'_>)->Result<String,String>{let bytes=hex(value,control)?;control.borrow_text(&bytes)?;String::from_utf8(bytes).map_err(|_|"invalid image native UTF-8".into())}
pub(crate) fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<SemioImageSnapshot,String>{
 let mut fields:[Option<&str>;8]=[None;8];for line in body.lines(){control.step()?;let line=line.trim();if line.is_empty(){continue}let(name,value)=line.split_once('=').ok_or("image native line requires named field")?;let index=match name{"schema"=>0,"width"=>1,"height"=>2,"colorspace"=>3,"bitDepth"=>4,"icc"=>5,"frames"=>6,"metadata"=>7,_=>return Err("unknown image native field".into())};if fields[index].replace(value).is_some(){return Err("duplicate image native field".into());}}
 let schema=hex_text(fields[0].ok_or("image native schema field is missing")?,control)?;let width=super::parse_u32(fields[1].ok_or("image native width field is missing")?)?;let height=super::parse_u32(fields[2].ok_or("image native height field is missing")?)?;let colorspace=match fields[3]{Some("r")=>SemioColorspace::Rgb,Some("a")=>SemioColorspace::Rgba,Some("g")=>SemioColorspace::Grayscale,Some("y")=>SemioColorspace::GrayscaleAlpha,Some("i")=>SemioColorspace::Indexed,Some(_)=>return Err("invalid image native colorspace".into()),None=>SemioColorspace::default()};let bit_depth=match fields[4]{Some(value)=>super::parse_u8(value)?,None=>0};
 let icc=match fields[5]{None|Some("[0]")=>None,Some(value)=>{let(tag,value)=pair(value,control)?;if tag!="1"{return Err("invalid image native ICC presence".into());}Some(hex(value,control)?)}};
 let mut frame_items=Items::new(fields[6].unwrap_or("[]"))?;let frame_count=frame_items.count(control,limits.max_rows)?;rows(frame_count.checked_add(1).ok_or("image native entity overflow")?,limits)?;let mut frames=control.allocate_vec::<SemioImageFrame>(frame_count)?;control.begin_stage(frame_count)?;
 while let Some(value)=frame_items.next(control)?{let(delay,rgba)=pair(value,control)?;let delay_ms=super::parse_u32(delay)?;let rgba8=hex(rgba,control)?;frames.push(SemioImageFrame{delay_ms,rgba8});control.step()?;}control.checkpoint()?;
 let mut metadata_items=Items::new(fields[7].unwrap_or("[]"))?;let metadata_count=metadata_items.count(control,limits.max_rows)?;rows(frame_count.checked_add(metadata_count).and_then(|v|v.checked_add(1)).ok_or("image native entity overflow")?,limits)?;let mut metadata=control.allocate_vec::<SemioImageMetadataEntry>(metadata_count)?;control.begin_stage(metadata_count)?;
 while let Some(value)=metadata_items.next(control)?{let(key,value)=pair(value,control)?;let key=hex_text(key,control)?;let value=hex_text(value,control)?;metadata.push(SemioImageMetadataEntry{key,value});control.step()?;}control.checkpoint()?;Ok(SemioImageSnapshot{schema,width,height,colorspace,bit_depth,icc,frames,metadata})
}
