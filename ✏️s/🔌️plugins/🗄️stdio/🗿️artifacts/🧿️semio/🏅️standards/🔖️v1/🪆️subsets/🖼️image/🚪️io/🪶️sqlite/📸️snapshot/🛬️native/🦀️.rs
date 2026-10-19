//! 🛬️ Typed image native construction with cumulative caller admission.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::schema::snapshot::{SemioImageSnapshot, SemioImageFrame, SemioImageMetadataEntry, SemioColorspace, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::{self as native,workspace};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};

fn length(reader:&mut store::ByteReader<'_>)->Result<usize,ValueError>{usize::try_from(reader.read_varint_u64().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"image native length exceeds address space"))}


fn rows(count:usize,limits:SqliteDatabaseLimits)->Result<(),ValueError>{if count>limits.max_rows{Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"image native entities exceed caller row limit"))}else{Ok(())}}

/// 🧵️ Separates complete SQL cells from the same caller's actual native allocation backing.
pub(crate)fn decode(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<SemioImageSnapshot,ValueError>{
 crate::io::sqlite::snapshot::admit_layout(control.limits())?;
 native::with_body(payload,STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA,control,native_control,|body,decoder,limits|workspace::decode(decoder,empty,|output,decoder|match body{native::Body::Binary(body)=>binary_into(output,body,decoder,limits),native::Body::Text(body)=>document_into(output,body,decoder,limits)}))
}

/// 🧩️ Fills the retained image root from borrowed binary octets, inserting every frame and metadata entry before its octets.
pub fn binary_into(output:&mut SemioImageSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported image native format"));}
 native::reader_text_into(&mut output.schema,&mut reader,control)?;output.width=reader.read_u32_le().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?;output.height=reader.read_u32_le().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?;output.colorspace=crate::io::binary::snapshot::colorspace_from_tag(reader.read_u8().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;output.bit_depth=reader.read_u8().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?;
 match reader.read_u8().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?{0=>{},1=>{output.icc=Some(Vec::new());native::reader_bytes_into(output.icc.as_mut().unwrap(),&mut reader,control)?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid image native ICC presence"))}
 let frame_count=length(&mut reader)?;rows(frame_count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"image native entity overflow"))?,limits)?;output.frames=control.allocate_vec::<SemioImageFrame>(frame_count)?;control.begin_stage(frame_count)?;
 for _ in 0..frame_count{output.frames.push(SemioImageFrame::default());let frame=output.frames.last_mut().unwrap();frame.delay_ms=reader.read_u32_le().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?;native::reader_bytes_into(&mut frame.rgba8,&mut reader,control)?;control.step()?;}control.checkpoint()?;
 let metadata_count=length(&mut reader)?;rows(metadata_count.checked_add(frame_count).and_then(|v|v.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"image native entity overflow"))?,limits)?;output.metadata=control.allocate_vec::<SemioImageMetadataEntry>(metadata_count)?;control.begin_stage(metadata_count)?;
 for _ in 0..metadata_count{output.metadata.push(SemioImageMetadataEntry::default());let entry=output.metadata.last_mut().unwrap();native::reader_text_into(&mut entry.key,&mut reader,control)?;native::reader_text_into(&mut entry.value,&mut reader,control)?;control.step()?;}control.checkpoint()?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"image native trailing bytes"));}Ok(())
}

#[derive(Clone)]
pub(super)struct Items<'a>{remaining:&'a str}
impl<'a> Items<'a>{
 pub(super)fn new(value:&'a str)->Result<Self,ValueError>{let value=value.trim();Ok(Self{remaining:value.strip_prefix('[').and_then(|s|s.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"image native list requires brackets"))?})}
 pub(super)fn next(&mut self,control:&mut NativeDecodeControl<'_>)->Result<Option<&'a str>,ValueError>{
 if self.remaining.is_empty(){return Ok(None)}let mut depth=0usize;let mut end=self.remaining.len();
 for(index,byte)in self.remaining.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b'['=>{depth+=1;if depth>2{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"image native list depth exceeds field shape"));}},b']'=>{depth=depth.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"image native unmatched bracket"))?;},b',' if depth==0=>{end=index;break;},_=>{}}}
 if depth!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"image native unmatched bracket"));}let value=self.remaining[..end].trim();self.remaining=if end<self.remaining.len(){&self.remaining[end+1..]}else{""};if value.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"image native empty list element"));}Ok(Some(value))
 }
 pub(super)fn count(&self,control:&mut NativeDecodeControl<'_>,maximum:usize)->Result<usize,ValueError>{let mut items=self.clone();let mut count=0usize;while items.next(control)?.is_some(){count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"image native list count overflow"))?;if count>maximum{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"image native entities exceed caller row limit"));}}Ok(count)}
}
pub(super)fn pair<'a>(value:&'a str,control:&mut NativeDecodeControl<'_>)->Result<(&'a str,&'a str),ValueError>{let inner=value.trim().strip_prefix('[').and_then(|s|s.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"image native pair requires brackets"))?;let mut delimiter=None;for(index,byte)in inner.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b','=>{if delimiter.replace(index).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"image native pair has extra fields"));}},b'['|b']'=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"image native pair contains nested fields")),_=>{}}}let index=delimiter.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"image native pair is missing a field"))?;Ok((inner[..index].trim(),inner[index+1..].trim()))}


/// 📝️ Fills the retained image root from the borrowed document body, inserting every frame and metadata entry before its octets.
pub fn document_into(output:&mut SemioImageSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut fields:[Option<&str>;8]=[None;8];for line in body.lines(){control.step()?;let line=line.trim();if line.is_empty(){continue}let(name,value)=line.split_once('=').ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"image native line requires named field"))?;let index=match name{"schema"=>0,"width"=>1,"height"=>2,"colorspace"=>3,"bitDepth"=>4,"icc"=>5,"frames"=>6,"metadata"=>7,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown image native field"))};if fields[index].replace(value).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate image native field"));}}
 native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"image native schema field is missing"))?,control)?;output.width=semio_s_artifact_stdio_semio_base::io::text::codec::parse_u32(fields[1].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"image native width field is missing"))?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;output.height=semio_s_artifact_stdio_semio_base::io::text::codec::parse_u32(fields[2].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"image native height field is missing"))?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;output.colorspace=match fields[3]{Some("r")=>SemioColorspace::Rgb,Some("a")=>SemioColorspace::Rgba,Some("g")=>SemioColorspace::Grayscale,Some("y")=>SemioColorspace::GrayscaleAlpha,Some("i")=>SemioColorspace::Indexed,Some(_)=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid image native colorspace")),None=>SemioColorspace::default()};output.bit_depth=match fields[4]{Some(value)=>crate::io::text::snapshot::parse_u8(value).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?,None=>0};
 match fields[5]{None|Some("[0]")=>{},Some(value)=>{let(tag,value)=pair(value,control)?;if tag!="1"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid image native ICC presence"));}output.icc=Some(Vec::new());native::hex_bytes_into(output.icc.as_mut().unwrap(),value,control)?;}}
 let mut frame_items=Items::new(fields[6].unwrap_or("[]"))?;let frame_count=frame_items.count(control,limits.max_rows)?;rows(frame_count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"image native entity overflow"))?,limits)?;output.frames=control.allocate_vec::<SemioImageFrame>(frame_count)?;control.begin_stage(frame_count)?;
 while let Some(value)=frame_items.next(control)?{let(delay,rgba)=pair(value,control)?;output.frames.push(SemioImageFrame::default());let frame=output.frames.last_mut().unwrap();frame.delay_ms=semio_s_artifact_stdio_semio_base::io::text::codec::parse_u32(delay).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;native::hex_bytes_into(&mut frame.rgba8,rgba,control)?;control.step()?;}control.checkpoint()?;
 let mut metadata_items=Items::new(fields[7].unwrap_or("[]"))?;let metadata_count=metadata_items.count(control,limits.max_rows)?;rows(frame_count.checked_add(metadata_count).and_then(|v|v.checked_add(1)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"image native entity overflow"))?,limits)?;output.metadata=control.allocate_vec::<SemioImageMetadataEntry>(metadata_count)?;control.begin_stage(metadata_count)?;
 while let Some(value)=metadata_items.next(control)?{let(key,value)=pair(value,control)?;output.metadata.push(SemioImageMetadataEntry::default());let entry=output.metadata.last_mut().unwrap();native::hex_text_into(&mut entry.key,key,control)?;native::hex_text_into(&mut entry.value,value,control)?;control.step()?;}control.checkpoint()
}
/// 🕳️ The empty typed root every native prefix starts from.
pub fn empty()->SemioImageSnapshot{SemioImageSnapshot{schema:String::new(),width:0,height:0,colorspace:Default::default(),bit_depth:0,frames:Vec::new(),icc:None,metadata:Vec::new()}}
