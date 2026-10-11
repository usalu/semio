//! 🛬️ Adjacent GLTF input admission awaits the owning capability baseline.
use super::*;
use semio_framework_dsl_record::native_encoding::EncodedRecord;
use semio_framework_value::NativeDecodeControl;
use semio_framework_dsl_record::RecordSpecProducer;

/// 📥️ Decodes through a supplied actual schema and typed owner constructor with cumulative settlement.
pub(super) fn decode(payload:&store::io::IoPayload,spec:RecordSpecProducer,reconstruct:impl FnOnce(&semio_framework_dsl_record::RecordValue,&mut NativeDecodeControl<'_>)->Result<GltfSnapshot,ValueError>,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<GltfSnapshot,ValueError>{
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;let length=match payload{store::io::IoPayload::Binary(bytes)=>bytes.len(),store::io::IoPayload::Text(text)=>text.len()};if length>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"GLTF native input exceeds file byte limit"))}
 let snapshot=control.allocation_stage_native(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let native_before=native_control.owned_bytes();
    let Some(allowance)=native_before.checked_add(remaining) else{return (Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow")),0)};let result=native_control.scoped_maximum(allowance,|native| {native.scoped_observer(&mut |event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total),|native|{

  let result=(||->Result<GltfSnapshot,ValueError>{let spec=spec.decode(native)?;let record=match payload{
   store::io::IoPayload::Binary(bytes)=>{let body=store::semio_format::unwrap_binary_controlled(bytes,"stdio.gltf",store::semio_format::Component::Pack,1,native)?;store::pack_rt::decode_document_controlled(body,&spec,&store::PackDecodeOptions::default(),native).map_err(|error|pack_refusal(error,native))?.0},
   store::io::IoPayload::Text(text)=>{let body=store::semio_format::split_text_preamble_controlled(text,"stdio.gltf",store::semio_format::Component::Dsl,1,native).map_err(store::semio_format::SemioError::into_value_error)?;semio_framework_dsl_record::parse_exact_controlled(body,&spec,&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document},native).map_err(|error|text_refusal(error,native))?}
  };let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(record);reconstruct(record.as_record(),native)})();result
    })});
    (result,native_control.owned_bytes().saturating_sub(native_before))
 })??;
 let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(snapshot,GltfSnapshot::retire_sqlite_snapshot);owner.as_mut().to_sqlite_database(control)?;Ok(owner.take())
}

struct SpanContext{bytes:[u8;96],length:usize}
impl std::fmt::Write for SpanContext{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.length.checked_add(text.len()).filter(|end|*end<=self.bytes.len()).ok_or(std::fmt::Error)?;self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())}}
fn text_refusal(error:semio_framework_diagnostic::TextError,native:&mut NativeDecodeControl<'_>)->ValueError{
 let result=(||->Result<String,ValueError>{let mut span=SpanContext{bytes:[0;96],length:0};std::fmt::write(&mut span,format_args!(" at {}:{} (length {})",error.span.line,error.span.column,error.span.length)).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"GLTF parser span formatting failed"))?;let expected=error.expected.as_deref().unwrap_or("");let expected_prefix=if error.expected.is_some(){b"; expected ".as_slice()}else{b"".as_slice()};let count=error.message.len().checked_add(span.length).and_then(|count|count.checked_add(expected_prefix.len())).and_then(|count|count.checked_add(expected.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"GLTF parser context byte count overflow"))?;let mut bytes=native.allocate_vec(count)?;bytes.extend_from_slice(error.message.as_bytes());bytes.extend_from_slice(&span.bytes[..span.length]);bytes.extend_from_slice(expected_prefix);bytes.extend_from_slice(expected.as_bytes());String::from_utf8(bytes).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"GLTF parser context is not UTF8"))})();match result{Ok(message)=>ValueError::new(error.kind,message),Err(error)=>error}
}
fn pack_refusal(error:store::PackRefusal,native:&mut NativeDecodeControl<'_>)->ValueError{match error{store::PackRefusal::TextRefusal(error)=>text_refusal(error,native),other=>other.into_value_error()}}
