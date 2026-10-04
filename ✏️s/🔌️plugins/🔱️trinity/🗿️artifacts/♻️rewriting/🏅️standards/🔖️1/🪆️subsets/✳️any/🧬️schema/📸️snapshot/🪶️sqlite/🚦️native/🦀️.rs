//! 🛬️ direct Rewriting input stage guards the actual decoded record and preserves cumulative admission.
use crate::RewritingSnapshot;
use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind,native_decoding::NativeDecodeProgress};
use semio_framework_dsl_record::native_encoding::EncodedRecord;
fn text_error(error:semio_framework_diagnostic::TextError)->ValueError{ValueError::new(error.kind,error.message)}
pub(super) fn decode(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<RewritingSnapshot,ValueError>{
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
 let length=match payload{store::os_io::IoPayload::Binary(bytes)=>bytes.len(),store::os_io::IoPayload::Text(text)=>text.len()};
 if length>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Rewriting native input exceeds file byte limit"))}
 control.allocation_stage(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let mut progress=|state:NativeDecodeProgress|checkpoint(state.completed,state.total);let mut native=NativeDecodeControl::new(remaining,&mut progress);
  let result=(||{
   let spec=RewritingSnapshot::__dsl_spec_producer().decode(&mut native)?;
   let record=match payload{
    store::os_io::IoPayload::Binary(bytes)=>{
     let body=store::semio_format::unwrap_binary_controlled(bytes,"trinity.rewriting",store::semio_format::Component::Pack,1,&mut native).map_err(store::semio_format::SemioError::into_value_error)?;
     store::pack_rt::decode_document_controlled(body,&spec,&store::PackDecodeOptions::default(),&mut native).map_err(store::PackRefusal::into_value_error)?.0
    },
    store::os_io::IoPayload::Text(text)=>{
     let body=store::semio_format::split_text_preamble_controlled(text,"trinity.rewriting",store::semio_format::Component::Dsl,1,&mut native).map_err(store::semio_format::SemioError::into_value_error)?;
     semio_framework_dsl_record::parse_exact_controlled(body,&spec,&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits{max_bytes:limits.max_file_bytes,..semio_framework_diagnostic::Limits::default()},mode:semio_framework_dsl_record::SourceMode::Document},&mut native).map_err(text_error)?
    },
   };
   let record=EncodedRecord::from_record(record);RewritingSnapshot::__dsl_from_record_controlled(record.as_record(),&mut native)
  })();(result,native.owned_bytes())
 })?
}
