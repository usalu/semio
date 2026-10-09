//! 🛬️ Borrowed Semio native primitives admit explicit owner fields before materialization.
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::native_decoding::{NativeDecodeControl,NativeDecodeProgress};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SqliteDatabaseLimits};

pub(crate) fn decode<T:semio_framework_value::retirement::RetireOwned>(payload:&store::io::IoPayload,id:&str,control:&mut SqliteSnapshotControl<'_>,binary:impl FnOnce(&[u8],&mut NativeDecodeControl<'_>,SqliteDatabaseLimits)->Result<T,ValueError>,document:impl FnOnce(&str,&mut NativeDecodeControl<'_>,SqliteDatabaseLimits)->Result<T,ValueError>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<T,ValueError>{
 let limits=control.limits();
 let size=match payload{store::io::IoPayload::Binary(v)=>v.len(),store::io::IoPayload::Text(v)=>v.len()};if size>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio native input exceeds file limit"))}
 control.allocation_stage_native(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
 let native_before=native_control.owned_bytes();
    let result=native_control.scoped_maximum(native_before.checked_add(remaining).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow"))?, |native| {native.scoped_observer(&mut |event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total),|native|{

 let result=(||->Result<T,ValueError>{let result=match payload{
 store::io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,id,store::semio_format::Component::Pack,1,native).map_err(store::semio_format::SemioError::into_value_error)?;binary(body,native,limits)?},
 store::io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,id,store::semio_format::Component::Dsl,1,native).map_err(store::semio_format::SemioError::into_value_error)?;document(body,native,limits)?}
 };let result=Owned::new(result);native.checkpoint()?;Ok(result.take())})();
 result
    })});
    (result,native_control.owned_bytes().saturating_sub(native_before))
 })?
}
pub(crate) fn length(reader:&mut store::ByteReader<'_>)->Result<usize,ValueError>{usize::try_from(reader.read_varint_u64().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Semio native length exceeds address space"))}
pub(crate) fn bytes<'a>(reader:&mut store::ByteReader<'a>)->Result<&'a[u8],ValueError>{let length=length(reader)?;reader.read_bytes(length).map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))}
pub(crate) fn text(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<String,ValueError>{let text=control.borrow_text(bytes(reader)?)?;control.copy_text(text)}
pub(crate) fn entities(total:&mut usize,count:usize,limits:SqliteDatabaseLimits)->Result<(),ValueError>{let next=total.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio native entity count overflow"))?;if next>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio native entities exceed caller row limit"))}*total=next;Ok(())}
pub(crate) fn fields<'a,const N:usize>(body:&'a str,names:[&str;N],control:&mut NativeDecodeControl<'_>)->Result<[Option<&'a str>;N],ValueError>{
 control.scoped_stage(|control|{control.begin_stage(body.len())?;for _ in body.bytes(){control.step()?;}let mut fields=[None;N];for line in body.lines(){let line=line.trim();if line.is_empty(){continue}let(name,value)=line.split_once('=').ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native field requires equals"))?;let index=names.iter().position(|expected|*expected==name).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native field"))?;if fields[index].replace(value).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio native field"))}}Ok(fields)})
}
#[derive(Clone)]
pub(crate) struct Items<'a>{remaining:&'a str}
impl<'a> Items<'a>{
 pub(crate) fn new(value:&'a str)->Result<Self,ValueError>{let value=value.trim();Ok(Self{remaining:value.strip_prefix('[').and_then(|v|v.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native list requires brackets"))?})}
 pub(crate) fn next(&mut self,control:&mut NativeDecodeControl<'_>)->Result<Option<&'a str>,ValueError>{
 if self.remaining.is_empty(){return Ok(None)}let mut depth=0usize;let mut end=self.remaining.len();
 for(index,byte)in self.remaining.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b'['=>{depth+=1;if depth>64{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Semio native list depth exceeds limit"))}},b']'=>{depth=depth.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native unmatched bracket"))?},b',' if depth==0=>{end=index;break},_=>{}}}
 if depth!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native unmatched bracket"))}let item=self.remaining[..end].trim();self.remaining=if end<self.remaining.len(){let rest=&self.remaining[end+1..];if rest.trim().is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native trailing list separator"))}rest}else{""};if item.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native empty list item"))}Ok(Some(item))
 }
 pub(crate) fn count(&self,control:&mut NativeDecodeControl<'_>,maximum:usize)->Result<usize,ValueError>{let mut items=self.clone();let mut count=0usize;while items.next(control)?.is_some(){count=count.checked_add(1).filter(|count|*count<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio native list exceeds entity limit"))?;}Ok(count)}
}
pub(crate) fn record<'a,const N:usize>(value:&'a str,control:&mut NativeDecodeControl<'_>)->Result<[&'a str;N],ValueError>{
 let value=value.trim();let inner=value.strip_prefix('[').and_then(|v|v.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native record requires brackets"))?;let mut fields=["";N];if N==0{return if inner.is_empty(){Ok(fields)}else{Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native record has extra fields"))}}
 let mut depth=0usize;let mut field=0usize;let mut start=0;
 for(index,byte)in inner.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b'['=>{depth+=1;if depth>64{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Semio native record exceeds nesting limit"))}},b']'=>{depth=depth.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native unmatched bracket"))?},b',' if depth==0=>{if field>=N-1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native record has extra fields"))}fields[field]=inner[start..index].trim();field+=1;start=index+1},_=>{}}}
 if depth!=0||field!=N-1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native record field shape differs"))}fields[field]=inner[start..].trim();Ok(fields)
}
pub(crate) fn hex(value:&str,control:&mut NativeDecodeControl<'_>)->Result<Vec<u8>,ValueError>{
 if !value.len().is_multiple_of(2){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native odd hex length"))}
 control.scoped_stage(|control|{let mut bytes=control.allocate_vec::<u8>(value.len()/2)?;control.begin_stage(value.len()/2)?;for pair in value.as_bytes().chunks_exact(2){let digit=|value|match value{b'0'..=b'9'=>Ok(value-b'0'),b'a'..=b'f'=>Ok(value-b'a'+10),b'A'..=b'F'=>Ok(value-b'A'+10),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native hex digit"))};bytes.push((digit(pair[0])?<<4)|digit(pair[1])?);control.step()?;}control.checkpoint()?;Ok(bytes)})
}
pub(crate) fn hex_text(value:&str,control:&mut NativeDecodeControl<'_>)->Result<String,ValueError>{let bytes=hex(value,control)?;control.borrow_text(&bytes)?;String::from_utf8(bytes).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Semio native text is not UTF-8"))}

pub(crate) fn float(value:&str,control:&mut NativeDecodeControl<'_>)->Result<f64,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}if let Some(word)=value.strip_prefix("nan64_"){if word.len()!=16||!word.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native NaN word"))}let bits=u64::from_str_radix(word,16).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native NaN word"))?;if bits&0x7ff0000000000000!=0x7ff0000000000000||bits&0xfffffffffffff==0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native NaN word has non-NaN class"))}return Ok(f64::from_bits(bits))}value.parse::<f64>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native floating number"))})
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

pub(crate) fn binary_list<T:semio_framework_value::retirement::RetireOwned>(reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize,mut read:impl FnMut(&mut store::ByteReader<'_>,&mut NativeDecodeControl<'_>,&mut usize)->Result<T,ValueError>)->Result<Vec<T>,ValueError>{
 let count=length(reader)?;self::entities(entities,count,limits)?;let mut items=Owned::new(control.allocate_vec::<T>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{items.get_mut().push(read(reader,control,entities)?);control.step()?;}Ok::<_,ValueError>(())})?;Ok(items.take())
}
pub(crate) fn text_list<T:semio_framework_value::retirement::RetireOwned>(value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize,mut read:impl FnMut(&str,&mut NativeDecodeControl<'_>,&mut usize)->Result<T,ValueError>)->Result<Vec<T>,ValueError>{
 let mut source=Items::new(value)?;let count=source.count(control,limits.max_rows)?;self::entities(entities,count,limits)?;let mut items=Owned::new(control.allocate_vec::<T>(count)?);control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=source.next(control)?{items.get_mut().push(read(value,control,entities)?);control.step()?;}Ok::<_,ValueError>(())})?;Ok(items.take())
}
pub(crate) fn float32(value:&str,control:&mut NativeDecodeControl<'_>)->Result<f32,ValueError>{control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}if let Some(word)=value.strip_prefix("nan32_"){if word.len()!=8||!word.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native binary32 NaN word"))}let bits=u32::from_str_radix(word,16).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native binary32 NaN word"))?;if bits&0x7f800000!=0x7f800000||bits&0x7fffff==0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native binary32 word has non-NaN class"))}return Ok(f32::from_bits(bits))}value.parse::<f32>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native binary32 number"))})}

pub(crate) fn decode_snapshot(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<crate::standards::v1::subsets::base::schema::snapshot::SemioSnapshot,ValueError>{super::semantic::layout(control.limits())?;decode(payload,crate::standards::v1::subsets::base::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA,control,binary_snapshot,text_snapshot)}
fn binary_snapshot(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<crate::standards::v1::subsets::base::schema::snapshot::SemioSnapshot,ValueError>{
 use crate::standards::v1::subsets::base::schema::snapshot::SemioSubsetSnapshot as S;use crate::standards::v1::subsets as owners;
 let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio envelope native format"))}let ordinal=reader.read_u8().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?;let schema=control.borrow_text(bytes(&mut reader)?)?;let limits=super::semantic::subset_limits(limits,schema.len(),super::semantic::tag(ordinal)?)?;let schema=Owned::new(control.copy_text(schema)?);let payload=reader.read_bytes(reader.remaining()).map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?;
 let id=match ordinal{0=>"stdio.semio.brep",1=>"stdio.semio.mesh",2=>"stdio.semio.model",3=>"stdio.semio.value",4=>"s.stdio.semio.document",5=>"stdio.semio.cad",6=>"stdio.semio.drawing",7=>"s.stdio.semio.image",8=>"stdio.semio.video",9=>"stdio.semio.audio",10=>"s.stdio.semio.animation",11=>"s.stdio.semio.presentation",12=>"stdio.semio.flow",13=>"s.stdio.semio.text",14=>"s.stdio.semio.table",15=>"s.stdio.semio.graph",16=>"stdio.semio.object",17=>"stdio.semio.kit",_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native envelope subset ordinal"))};
 let body=store::semio_format::unwrap_binary_controlled(payload,id,store::semio_format::Component::Pack,1,control).map_err(store::semio_format::SemioError::into_value_error)?;
 let subset=Owned::new(match ordinal{
 0=>S::Brep(owners::brep::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 1=>S::Mesh(owners::mesh::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 2=>S::Model(owners::model::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 3=>{let body=control.borrow_text(body)?;S::Value(owners::value::io::sqlite::snapshot::native_decoding::document(body,control,limits)?)},
 4=>S::Document(owners::document::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 5=>S::Cad(owners::cad::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 6=>S::Drawing(owners::drawing::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 7=>S::Image(owners::image::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 8=>S::Video(owners::video::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 9=>S::Audio(owners::audio::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 10=>S::Animation(owners::animation::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 11=>S::Presentation(owners::presentation::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 12=>S::Flow(owners::flow::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 13=>S::Text(owners::text::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 14=>S::Table(owners::table::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 15=>S::Graph(owners::graph::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 16=>S::Object(owners::object::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 17=>S::Kit(owners::kit::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native envelope subset ordinal"))
 });let value=Owned::new(crate::standards::v1::subsets::base::schema::snapshot::SemioSnapshot{schema:schema.take(),subset:subset.take()});control.checkpoint()?;Ok(value.take())
}
fn text_snapshot(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<crate::standards::v1::subsets::base::schema::snapshot::SemioSnapshot,ValueError>{
 use crate::standards::v1::subsets::base::schema::snapshot::SemioSubsetSnapshot as S;use crate::standards::v1::subsets as owners;
 control.scoped_stage(|control|{control.begin_stage(body.len())?;for _ in body.bytes(){control.step()?}Ok::<_,ValueError>(())})?;
 let mut fields=body.splitn(3,'\n');let tag=fields.next().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio envelope subset"))?.trim().strip_prefix("subset=").ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio envelope requires subset header"))?;let schema=fields.next().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio envelope schema"))?.trim().strip_prefix("schema=").ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio envelope requires schema header"))?;let body=fields.next().unwrap_or("");if !schema.len().is_multiple_of(2){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native odd hex length"))}let limits=super::semantic::subset_limits(limits,schema.len()/2,tag)?;let schema=Owned::new(hex_text(schema,control)?);
 let subset=Owned::new(match tag{
 "brep"=>S::Brep(owners::brep::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "mesh"=>S::Mesh(owners::mesh::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "model"=>S::Model(owners::model::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "value"=>S::Value(owners::value::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "document"=>S::Document(owners::document::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "cad"=>S::Cad(owners::cad::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "drawing"=>S::Drawing(owners::drawing::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "image"=>S::Image(owners::image::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "video"=>S::Video(owners::video::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "audio"=>S::Audio(owners::audio::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "animation"=>S::Animation(owners::animation::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "presentation"=>S::Presentation(owners::presentation::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "flow"=>S::Flow(owners::flow::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "text"=>S::Text(owners::text::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "table"=>S::Table(owners::table::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "graph"=>S::Graph(owners::graph::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "object"=>S::Object(owners::object::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "kit"=>S::Kit(owners::kit::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native envelope subset tag"))
 });let value=Owned::new(crate::standards::v1::subsets::base::schema::snapshot::SemioSnapshot{schema:schema.take(),subset:subset.take()});control.checkpoint()?;Ok(value.take())
}

/// 🧷️ Measures exact decoded UTF-8 from borrowed hexadecimal source with fixed scalar storage.
pub(crate)fn hex_text_extent(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{
 if !value.len().is_multiple_of(2){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))}
 control.scoped_stage(|control|{
  let mut scalar=[0u8;4];let mut used=0;let mut needed=0;control.begin_stage(value.len()/2)?;
  for pair in value.as_bytes().chunks_exact(2){
   let digit=|value|match value{b'0'..=b'9'=>Ok(value-b'0'),b'a'..=b'f'=>Ok(value-b'a'+10),b'A'..=b'F'=>Ok(value-b'A'+10),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))};let byte=digit(pair[0])?*16+digit(pair[1])?;
   if used==0{needed=match byte{0..=0x7f=>1,0xc2..=0xdf=>2,0xe0..=0xef=>3,0xf0..=0xf4=>4,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))};}else if byte&0xc0!=0x80{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))}
   scalar[used]=byte;used+=1;if used==needed{std::str::from_utf8(&scalar[..used]).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is not UTF-8"))?;used=0;}control.step()?;
  }
  if used!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))}Ok(value.len()/2)
 })
}
