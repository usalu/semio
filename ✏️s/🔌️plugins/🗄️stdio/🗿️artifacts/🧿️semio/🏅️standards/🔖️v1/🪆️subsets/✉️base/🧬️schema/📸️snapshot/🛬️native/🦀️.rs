//! 🛬️ Borrowed Semio native primitives admit explicit owner fields before materialization.
use semio_framework_value::native_decoding::{NativeDecodeControl,NativeDecodeProgress};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SqliteDatabaseLimits};

pub(crate) fn decode<T:semio_framework_value::retirement::RetireOwned>(payload:&store::os_io::IoPayload,id:&str,control:&mut SqliteSnapshotControl<'_>,binary:impl FnOnce(&[u8],&mut NativeDecodeControl<'_>,SqliteDatabaseLimits)->Result<T,String>,document:impl FnOnce(&str,&mut NativeDecodeControl<'_>,SqliteDatabaseLimits)->Result<T,String>)->Result<T,String>{
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
 let size=match payload{store::os_io::IoPayload::Binary(v)=>v.len(),store::os_io::IoPayload::Text(v)=>v.len()};if size>limits.max_file_bytes{return Err("Semio native input exceeds file limit".into())}
 let mut callback=|event:NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::DecodeNative,event.completed,event.total).is_ok();let mut native=NativeDecodeControl::new(limits.max_value_bytes,&mut callback);
 let result=match payload{
 store::os_io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,id,store::semio_format::Component::Pack,1,&mut native).map_err(|e|e.to_string())?;binary(body,&mut native,limits)?},
 store::os_io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,id,store::semio_format::Component::Dsl,1,&mut native).map_err(|e|e.to_string())?;document(body,&mut native,limits)?}
 };let result=Owned::new(result);native.checkpoint()?;Ok(result.take())
}
pub(crate) fn length(reader:&mut store::ByteReader<'_>)->Result<usize,String>{usize::try_from(reader.read_varint_u64().map_err(|e|e.to_string())?).map_err(|_|"Semio native length exceeds address space".into())}
pub(crate) fn bytes<'a>(reader:&mut store::ByteReader<'a>)->Result<&'a[u8],String>{let length=length(reader)?;reader.read_bytes(length).map_err(|e|e.to_string())}
pub(crate) fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<String,String>{let text=control.borrow_text(bytes(reader)?)?;control.copy_text(text)}
pub(crate) fn entities(total:&mut usize,count:usize,limits:SqliteDatabaseLimits)->Result<(),String>{*total=total.checked_add(count).filter(|count|*count<=limits.max_rows).ok_or("Semio native entities exceed caller row limit")?;Ok(())}
pub(crate) fn fields<'a,const N:usize>(body:&'a str,names:[&str;N],control:&mut NativeDecodeControl<'_>)->Result<[Option<&'a str>;N],String>{
 control.scoped_stage(|control|{control.begin_stage(body.len())?;for _ in body.bytes(){control.step()?;}let mut fields=[None;N];for line in body.lines(){let line=line.trim();if line.is_empty(){continue}let(name,value)=line.split_once('=').ok_or("Semio native field requires equals")?;let index=names.iter().position(|expected|*expected==name).ok_or("unknown Semio native field")?;if fields[index].replace(value).is_some(){return Err("duplicate Semio native field".into())}}Ok(fields)})
}
#[derive(Clone)]
pub(crate) struct Items<'a>{remaining:&'a str}
impl<'a> Items<'a>{
 pub(crate) fn new(value:&'a str)->Result<Self,String>{let value=value.trim();Ok(Self{remaining:value.strip_prefix('[').and_then(|v|v.strip_suffix(']')).ok_or("Semio native list requires brackets")?})}
 pub(crate) fn next(&mut self,control:&mut NativeDecodeControl<'_>)->Result<Option<&'a str>,String>{
 if self.remaining.is_empty(){return Ok(None)}let mut depth=0usize;let mut end=self.remaining.len();
 for(index,byte)in self.remaining.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b'['=>{depth+=1;if depth>64{return Err("Semio native list depth exceeds limit".into())}},b']'=>{depth=depth.checked_sub(1).ok_or("Semio native unmatched bracket")?},b',' if depth==0=>{end=index;break},_=>{}}}
 if depth!=0{return Err("Semio native unmatched bracket".into())}let item=self.remaining[..end].trim();self.remaining=if end<self.remaining.len(){let rest=&self.remaining[end+1..];if rest.trim().is_empty(){return Err("Semio native trailing list separator".into())}rest}else{""};if item.is_empty(){return Err("Semio native empty list item".into())}Ok(Some(item))
 }
 pub(crate) fn count(&self,control:&mut NativeDecodeControl<'_>,maximum:usize)->Result<usize,String>{let mut items=self.clone();let mut count=0usize;while items.next(control)?.is_some(){count=count.checked_add(1).filter(|count|*count<=maximum).ok_or("Semio native list exceeds entity limit")?;}Ok(count)}
}
pub(crate) fn record<'a,const N:usize>(value:&'a str,control:&mut NativeDecodeControl<'_>)->Result<[&'a str;N],String>{
 let value=value.trim();let inner=value.strip_prefix('[').and_then(|v|v.strip_suffix(']')).ok_or("Semio native record requires brackets")?;let mut fields=["";N];if N==0{return if inner.is_empty(){Ok(fields)}else{Err("Semio native record has extra fields".into())}}
 let mut depth=0usize;let mut field=0usize;let mut start=0;
 for(index,byte)in inner.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b'['=>{depth+=1;if depth>64{return Err("Semio native record exceeds nesting limit".into())}},b']'=>{depth=depth.checked_sub(1).ok_or("Semio native unmatched bracket")?},b',' if depth==0=>{if field>=N-1{return Err("Semio native record has extra fields".into())}fields[field]=inner[start..index].trim();field+=1;start=index+1},_=>{}}}
 if depth!=0||field!=N-1{return Err("Semio native record field shape differs".into())}fields[field]=inner[start..].trim();Ok(fields)
}
pub(crate) fn hex(value:&str,control:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,String>{
 if !value.len().is_multiple_of(2){return Err("Semio native odd hex length".into())}
 control.scoped_stage(|control|{let mut bytes=control.allocate_vec::<u8>(value.len()/2)?;control.begin_stage(value.len()/2)?;for pair in value.as_bytes().chunks_exact(2){let digit=|value|match value{b'0'..=b'9'=>Ok(value-b'0'),b'a'..=b'f'=>Ok(value-b'a'+10),b'A'..=b'F'=>Ok(value-b'A'+10),_=>Err("invalid Semio native hex digit")};bytes.push((digit(pair[0])?<<4)|digit(pair[1])?);control.step()?;}control.checkpoint()?;Ok(bytes)})
}
pub(crate) fn hex_text(value:&str,control:&mut NativeDecodeControl<'_>)->Result<String,String>{let bytes=hex(value,control)?;control.borrow_text(&bytes)?;String::from_utf8(bytes).map_err(|_|"Semio native text is not UTF-8".into())}

pub(crate) fn float(value:&str,control:&mut NativeDecodeControl<'_>)->Result<f64,String>{
 control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}if let Some(word)=value.strip_prefix("nan64_"){if word.len()!=16||!word.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err("invalid Semio native NaN word".into())}let bits=u64::from_str_radix(word,16).map_err(|_|"invalid Semio native NaN word")?;if bits&0x7ff0000000000000!=0x7ff0000000000000||bits&0xfffffffffffff==0{return Err("Semio native NaN word has non-NaN class".into())}return Ok(f64::from_bits(bits))}value.parse::<f64>().map_err(|_|"invalid Semio native floating number".into())})
}

pub(crate) struct Owned<T:semio_framework_value::retirement::RetireOwned>{value:Option<T>}
impl<T:semio_framework_value::retirement::RetireOwned> Owned<T>{
 pub(crate) fn new(value:T)->Self{Self{value:Some(value)}}
 pub(crate) fn get_mut(&mut self)->&mut T{self.value.as_mut().expect("live Semio native owner")}
 pub(crate) fn take(mut self)->T{self.value.take().expect("live Semio native owner")}
}
impl<T:semio_framework_value::retirement::RetireOwned> Drop for Owned<T>{
 fn drop(&mut self){if let Some(value)=self.value.take(){let mut cursor=semio_framework_value::retirement::owned_retirement(value);while !cursor.terminal_is_empty(){cursor.close_step(256,65536).expect("valid Semio native retirement cursor");}}}
}

pub(crate) fn binary_list<T:semio_framework_value::retirement::RetireOwned>(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize,mut read:impl FnMut(&mut store::ByteReader<'_>,&mut NativeDecodeControl<'_>,&mut usize)->Result<T,String>)->Result<Vec<T>,String>{
 let count=length(reader)?;self::entities(entities,count,limits)?;let mut items=Owned::new(control.allocate_vec::<T>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{items.get_mut().push(read(reader,control,entities)?);control.step()?;}Ok::<_,String>(())})?;Ok(items.take())
}
pub(crate) fn text_list<T:semio_framework_value::retirement::RetireOwned>(value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize,mut read:impl FnMut(&str,&mut NativeDecodeControl<'_>,&mut usize)->Result<T,String>)->Result<Vec<T>,String>{
 let mut source=Items::new(value)?;let count=source.count(control,limits.max_rows)?;self::entities(entities,count,limits)?;let mut items=Owned::new(control.allocate_vec::<T>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=source.next(control)?{items.get_mut().push(read(value,control,entities)?);control.step()?;}Ok::<_,String>(())})?;Ok(items.take())
}
pub(crate) fn float32(value:&str,control:&mut NativeDecodeControl<'_>)->Result<f32,String>{control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}if let Some(word)=value.strip_prefix("nan32_"){if word.len()!=8||!word.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err("invalid Semio native binary32 NaN word".into())}let bits=u32::from_str_radix(word,16).map_err(|_|"invalid Semio native binary32 NaN word")?;if bits&0x7f800000!=0x7f800000||bits&0x7fffff==0{return Err("Semio native binary32 word has non-NaN class".into())}return Ok(f32::from_bits(bits))}value.parse::<f32>().map_err(|_|"invalid Semio native binary32 number".into())})}

pub(super) fn decode_snapshot(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<super::SemioSnapshot,String>{decode(payload,super::STDIO_SEMIO_DOCUMENT_SCHEMA,control,binary_snapshot,text_snapshot)}
fn subset_limits(mut limits:SqliteDatabaseLimits)->Result<SqliteDatabaseLimits,String>{limits.max_rows=limits.max_rows.checked_sub(1).ok_or("Semio envelope document exceeds caller row limit")?;Ok(limits)}
fn binary_snapshot(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<super::SemioSnapshot,String>{
 use super::SemioSubsetSnapshot as S;use crate::standards::v1::subsets as owners;
 let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|e|e.to_string())?!=1{return Err("unsupported Semio envelope native format".into())}let ordinal=reader.read_u8().map_err(|e|e.to_string())?;let limits=subset_limits(limits)?;let schema=text(&mut reader,control)?;let payload=reader.read_bytes(reader.remaining()).map_err(|e|e.to_string())?;
 let id=match ordinal{0=>"stdio.semio.brep",1=>"stdio.semio.mesh",2=>"stdio.semio.model",3=>"stdio.semio.value",4=>"s.stdio.semio.document",5=>"stdio.semio.cad",6=>"stdio.semio.drawing",7=>"s.stdio.semio.image",8=>"stdio.semio.video",9=>"stdio.semio.audio",10=>"s.stdio.semio.animation",11=>"s.stdio.semio.presentation",12=>"stdio.semio.flow",13=>"s.stdio.semio.text",14=>"s.stdio.semio.table",15=>"s.stdio.semio.graph",16=>"stdio.semio.object",17=>"stdio.semio.kit",_=>return Err("unknown Semio native envelope subset ordinal".into())};
 let body=store::semio_format::unwrap_binary_controlled(payload,id,store::semio_format::Component::Pack,1,control).map_err(|e|e.to_string())?;
 let subset=match ordinal{
 0=>S::Brep(owners::brep::schema::snapshot::native_decoding::binary(body,control,limits)?),
 1=>S::Mesh(owners::mesh::schema::snapshot::native_decoding::binary(body,control,limits)?),
 2=>S::Model(owners::model::schema::snapshot::native_decoding::binary(body,control,limits)?),
 3=>{let body=control.borrow_text(body)?;S::Value(owners::value::schema::snapshot::native_decoding::document(body,control,limits)?)},
 4=>S::Document(owners::document::schema::snapshot::native_decoding::binary(body,control,limits)?),
 5=>S::Cad(owners::cad::schema::snapshot::native_decoding::binary(body,control,limits)?),
 6=>S::Drawing(owners::drawing::schema::snapshot::native_decoding::binary(body,control,limits)?),
 7=>S::Image(owners::image::schema::snapshot::native_decoding::binary(body,control,limits)?),
 8=>S::Video(owners::video::schema::snapshot::native_decoding::binary(body,control,limits)?),
 9=>S::Audio(owners::audio::schema::snapshot::native_decoding::binary(body,control,limits)?),
 10=>S::Animation(owners::animation::schema::snapshot::native_decoding::binary(body,control,limits)?),
 11=>S::Presentation(owners::presentation::schema::snapshot::native_decoding::binary(body,control,limits)?),
 12=>S::Flow(owners::flow::schema::snapshot::native_decoding::binary(body,control,limits)?),
 13=>S::Text(owners::text::schema::snapshot::native_decoding::binary(body,control,limits)?),
 14=>S::Table(owners::table::schema::snapshot::native_decoding::binary(body,control,limits)?),
 15=>S::Graph(owners::graph::schema::snapshot::native_decoding::binary(body,control,limits)?),
 16=>S::Object(owners::object::schema::snapshot::native_decoding::binary(body,control,limits)?),
 17=>S::Kit(owners::kit::schema::snapshot::native_decoding::binary(body,control,limits)?),
 _=>return Err("unknown Semio native envelope subset ordinal".into())
 };Ok(super::SemioSnapshot{schema,subset})
}
fn text_snapshot(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<super::SemioSnapshot,String>{
 use super::SemioSubsetSnapshot as S;use crate::standards::v1::subsets as owners;
 control.scoped_stage(|control|{control.begin_stage(body.len())?;for _ in body.bytes(){control.step()?}Ok::<_,String>(())})?;
 let mut fields=body.splitn(3,'\n');let tag=fields.next().ok_or("missing Semio envelope subset")?.trim().strip_prefix("subset=").ok_or("Semio envelope requires subset header")?;let schema=fields.next().ok_or("missing Semio envelope schema")?.trim().strip_prefix("schema=").ok_or("Semio envelope requires schema header")?;let body=fields.next().unwrap_or("");let limits=subset_limits(limits)?;let schema=hex_text(schema,control)?;
 let subset=match tag{
 "brep"=>S::Brep(owners::brep::schema::snapshot::native_decoding::document(body,control,limits)?),
 "mesh"=>S::Mesh(owners::mesh::schema::snapshot::native_decoding::document(body,control,limits)?),
 "model"=>S::Model(owners::model::schema::snapshot::native_decoding::document(body,control,limits)?),
 "value"=>S::Value(owners::value::schema::snapshot::native_decoding::document(body,control,limits)?),
 "document"=>S::Document(owners::document::schema::snapshot::native_decoding::document(body,control,limits)?),
 "cad"=>S::Cad(owners::cad::schema::snapshot::native_decoding::document(body,control,limits)?),
 "drawing"=>S::Drawing(owners::drawing::schema::snapshot::native_decoding::document(body,control,limits)?),
 "image"=>S::Image(owners::image::schema::snapshot::native_decoding::document(body,control,limits)?),
 "video"=>S::Video(owners::video::schema::snapshot::native_decoding::document(body,control,limits)?),
 "audio"=>S::Audio(owners::audio::schema::snapshot::native_decoding::document(body,control,limits)?),
 "animation"=>S::Animation(owners::animation::schema::snapshot::native_decoding::document(body,control,limits)?),
 "presentation"=>S::Presentation(owners::presentation::schema::snapshot::native_decoding::document(body,control,limits)?),
 "flow"=>S::Flow(owners::flow::schema::snapshot::native_decoding::document(body,control,limits)?),
 "text"=>S::Text(owners::text::schema::snapshot::native_decoding::document(body,control,limits)?),
 "table"=>S::Table(owners::table::schema::snapshot::native_decoding::document(body,control,limits)?),
 "graph"=>S::Graph(owners::graph::schema::snapshot::native_decoding::document(body,control,limits)?),
 "object"=>S::Object(owners::object::schema::snapshot::native_decoding::document(body,control,limits)?),
 "kit"=>S::Kit(owners::kit::schema::snapshot::native_decoding::document(body,control,limits)?),
 _=>return Err("unknown Semio native envelope subset tag".into())
 };Ok(super::SemioSnapshot{schema,subset})
}
