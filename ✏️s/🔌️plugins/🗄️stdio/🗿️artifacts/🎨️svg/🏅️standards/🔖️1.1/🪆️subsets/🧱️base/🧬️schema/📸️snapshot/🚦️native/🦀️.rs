//! 🚦️ Actual SVG fields retain the existing structured native state protocol.
use super::SvgSnapshot;
use semio_framework_os_kernel as store;
use store::{io_schema::IoPayload,sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase}};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind,native_decoding::NativeDecodeProgress,native_encoding::NativeEncodeProgress};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlNativeInput,XmlNativeEmission,read_xml_native_snapshot_fields,emit_xml_native_snapshot_fields,sqlite::{XmlDocumentView,retire_xml_document}};
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
fn retire(snapshot:SvgSnapshot){retire_xml_document(snapshot.doc);}
fn prefix(encoding:SnapshotEncoding)->Result<usize,ValueError>{store::semio_format::declared_envelope_prefix_len("stdio.svg",match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl},1)}
pub(super) fn decode(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<SvgSnapshot,ValueError>{
 let limits=control.limits();let length=match payload{IoPayload::Binary(bytes)=>bytes.len(),IoPayload::Text(text)=>text.len()};
 if length>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"SVG native input exceeds file ceiling"))}
 control.allocation_stage(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let mut callback=|event:NativeDecodeProgress|checkpoint(event.completed,event.total);let mut native=NativeDecodeControl::new(remaining,&mut callback);
  let result=(||{
   let(bytes,binary)=match payload{
    IoPayload::Binary(bytes)=>(store::semio_format::unwrap_binary_controlled(bytes,"stdio.svg",store::semio_format::Component::Pack,1,&mut native).map_err(store::semio_format::SemioError::into_value_error)?,true),
    IoPayload::Text(text)=>(store::semio_format::split_text_preamble_controlled(text,"stdio.svg",store::semio_format::Component::Dsl,1,&mut native).map_err(store::semio_format::SemioError::into_value_error)?.as_bytes(),false)};
   native.begin_stage(bytes.len())?;let mut position=0;let mut rows=0;
   let(schema,doc)=read_xml_native_snapshot_fields(XmlNativeInput{bytes,position:&mut position,rows:&mut rows,limits,binary,control:&mut native})?;
   let snapshot=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(SvgSnapshot{schema,doc},retire);native.checkpoint()?;Ok(snapshot.take())
  })();(result,native.owned_bytes())
 })?
}
fn count(value:&SvgSnapshot,encoding:SnapshotEncoding,native:&mut NativeEncodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<usize,ValueError>{
 let mut count=0;let mut rows=0;native.begin_stage(0)?;
 emit_xml_native_snapshot_fields(&value.schema,XmlDocumentView::from(&value.doc),XmlNativeEmission{control:native,output:None,count:&mut count,rows:&mut rows,limits,encoding})?;
 let bytes=count.checked_add(prefix(encoding)?).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"SVG native output ceiling overflow"))?;
 if bytes>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"SVG native output exceeds file ceiling"))}native.checkpoint()?;Ok(count)
}
pub(super) fn preflight(value:&SvgSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let limits=control.limits();control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{
  let mut callback=|event:NativeEncodeProgress|checkpoint(event.completed,event.total);let mut native=NativeEncodeControl::new(remaining,&mut callback);
  let result=count(value,encoding,&mut native,limits).map(|_|());(result,native.owned_bytes())
 })?
}
pub(super) fn encode(value:&SvgSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<IoPayload,ValueError>{
 let limits=control.limits();control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{
  let mut callback=|event:NativeEncodeProgress|checkpoint(event.completed,event.total);let mut native=NativeEncodeControl::new(remaining,&mut callback);
  let result=(||{
   let size=count(value,encoding,&mut native,limits)?;let mut bytes=native.allocate_vec(size)?;let mut count=0;let mut rows=0;native.begin_stage(size)?;
   emit_xml_native_snapshot_fields(&value.schema,XmlDocumentView::from(&value.doc),XmlNativeEmission{control:&mut native,output:Some(&mut bytes),count:&mut count,rows:&mut rows,limits,encoding})?;
   if count!=size||bytes.len()!=size{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"SVG native measured fields differ from emitted size"))}
   let payload=match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(store::semio_format::wrap_binary_controlled("stdio.svg",store::semio_format::Component::Pack,1,&bytes,&mut native)?),
    SnapshotEncoding::Text=>{let text=String::from_utf8(bytes).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"SVG native emission is not UTF8"))?;IoPayload::Text(store::semio_format::wrap_text_controlled("stdio.svg",store::semio_format::Component::Dsl,1,&text,&mut native)?)}};
   native.checkpoint()?;Ok(payload)
  })();(result,native.owned_bytes())
 })?
}
