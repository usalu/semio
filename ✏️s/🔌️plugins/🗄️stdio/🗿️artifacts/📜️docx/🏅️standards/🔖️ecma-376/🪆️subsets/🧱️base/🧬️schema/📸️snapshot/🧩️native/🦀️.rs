//! 📜️ Complete DOCX literal schema, ordered OPC metadata and typed XML fields.
use super::{DocxSnapshot,DocxXmlPart};
use semio_framework_os_kernel::{NativeEncodeControl,NativeDecodeControl,DecodedValue,sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,SqliteDatabaseLimits},io_schema::IoPayload};
use semio_s_artifact_stdio_zip::opc::native::{OpcNativeWriter,OpcNativeReader};
use semio_s_artifact_stdio_xml::schema::snapshot::sqlite::{XmlDocumentView,retire_xml_document};
struct Parts(Vec<DocxXmlPart>);
impl Drop for Parts{fn drop(&mut self){for part in self.0.drain(..){retire_xml_document(part.document);}}}
fn write(snapshot:&DocxSnapshot,writer:&mut OpcNativeWriter<'_, '_, '_>)->Result<(),String>{
 writer.rows(1)?;
 if writer.encoding==SnapshotEncoding::Binary{let token=b"stdio.docx.pack v1";writer.raw(b"\x89SEM\r\n\x1a\n")?;writer.raw(&(token.len()as u32).to_le_bytes())?;writer.raw(token)?;writer.raw(&[1])?;}else{writer.raw(b"semio stdio.docx.dsl v1\n[")?;}
 writer.string(&snapshot.schema)?;writer.delimiter(b",")?;writer.package(&snapshot.opc)?;writer.delimiter(b",")?;
 writer.list(&snapshot.xml_parts,|writer,part|{writer.delimiter(b"[")?;writer.string(&part.path)?;writer.delimiter(b",")?;writer.string(&part.content_type)?;writer.delimiter(b",")?;writer.document(XmlDocumentView::from(&part.document))?;writer.delimiter(b"]")})?;writer.delimiter(b"]")
}
pub(super) fn encode(snapshot:&DocxSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,String>{
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|control.checkpoint(SqliteSnapshotPhase::EncodeNative,event.completed,event.total).is_ok();let mut native=NativeEncodeControl::new(limits.max_value_bytes,&mut callback);native.begin_stage(0)?;
 let mut writer=OpcNativeWriter{control:&mut native,output:None,count:0,rows:0,limits,encoding};write(snapshot,&mut writer)?;let total=writer.count;native.begin_stage(total)?;let mut bytes=native.allocate_vec::<u8>(total)?;
 let mut writer=OpcNativeWriter{control:&mut native,output:Some(&mut bytes),count:0,rows:0,limits,encoding};write(snapshot,&mut writer)?;if writer.count!=total||bytes.len()!=total{return Err("DOCX measured native field size differs".into());}native.checkpoint()?;
 match encoding{SnapshotEncoding::Binary=>Ok(IoPayload::Binary(bytes)),SnapshotEncoding::Text=>Ok(IoPayload::Text(String::from_utf8(bytes).map_err(|_|"DOCX native output is not UTF-8")?))}
}
fn read(reader:&mut OpcNativeReader<'_, '_, '_>)->Result<DocxSnapshot,String>{
 reader.rows(1)?;if reader.binary{if reader.take(1)?!=[1]{return Err("DOCX native revision differs".into());}}else{reader.delimiter(b'[')?;}
 let schema=reader.string()?;reader.delimiter(b',')?;let opc=reader.package()?;reader.delimiter(b',')?;reader.delimiter(b'[')?;
 let count=if reader.binary{usize::try_from(reader.unsigned()?).map_err(|_|"DOCX native part length exceeds platform width")?}else{0};if reader.binary{reader.rows(count)?;}let mut parts=Parts(reader.control.allocate_vec::<DocxXmlPart>(count)?);
 while if reader.binary{parts.0.len()<count}else{reader.bytes.get(reader.position)!=Some(&b']')}{
  if !reader.binary{reader.rows(1)?;reader.reserve(&mut parts.0)?;}if !parts.0.is_empty(){reader.delimiter(b',')?;}reader.delimiter(b'[')?;let path=reader.string()?;reader.delimiter(b',')?;let content_type=reader.string()?;reader.delimiter(b',')?;
  let document=DecodedValue::new(reader.document()?,retire_xml_document);reader.delimiter(b']')?;parts.0.push(DocxXmlPart{path,content_type,document:document.take()});
 }
 reader.delimiter(b']')?;reader.delimiter(b']')?;if reader.position!=reader.bytes.len(){return Err("DOCX native input has trailing fields".into());}reader.control.checkpoint()?;Ok(DocxSnapshot{schema,opc,xml_parts:std::mem::take(&mut parts.0)})
}
fn input(bytes:&[u8],binary:bool,control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,String>{
 let limits=control.limits();if bytes.len()>limits.max_file_bytes{return Err("DOCX native input exceeds caller file ceiling".into());}control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,bytes.len())?;
 let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|control.checkpoint(SqliteSnapshotPhase::DecodeNative,event.completed,event.total).is_ok();let mut native=NativeDecodeControl::new(limits.max_value_bytes,&mut callback);
 let body=if binary{store::semio_format::unwrap_binary_controlled(bytes,"stdio.docx",store::semio_format::Component::Pack,1,&mut native).map_err(|error|error.to_string())?}else{let text=native.borrow_text(bytes)?;store::semio_format::split_text_preamble_controlled(text,"stdio.docx",store::semio_format::Component::Dsl,1,&mut native).map_err(|error|error.to_string())?.as_bytes()};native.begin_stage(body.len())?;read(&mut OpcNativeReader{bytes:body,position:0,rows:0,limits,binary,control:&mut native})
}
pub(super) fn decode(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,String>{match payload{IoPayload::Binary(bytes)=>input(bytes,true,control),IoPayload::Text(text)=>input(text.as_bytes(),false,control)}}
pub(super) fn decode_text(text:&str,control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,String>{input(text.as_bytes(),false,control)}
pub(super) fn decode_binary(bytes:&[u8],control:&mut SqliteSnapshotControl<'_>)->Result<DocxSnapshot,String>{input(bytes,true,control)}
pub(super) fn pack_limits(limits:&store::mounted_pack_rt::PackLimits)->SqliteDatabaseLimits{SqliteDatabaseLimits{max_file_bytes:usize::try_from(limits.max_file_len).unwrap_or(usize::MAX),max_value_bytes:usize::try_from(limits.max_total_alloc).unwrap_or(usize::MAX),max_rows:usize::try_from(limits.max_items).unwrap_or(usize::MAX),..SqliteDatabaseLimits::default()}}
