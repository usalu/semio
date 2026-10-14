//! 📸️ Controlled native snapshot factories preserve the caller’s complete admission.
use semio_framework_dsl_record::{RecordSpecProducer,RecordValue};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
use crate::io_schema::IoPayload;

use pack::snapshot::{ArtifactNativeSnapshot,NativeSnapshotInput,NativeSnapshotEncoding};
/// 🫴️ Requires original native receiving custody instead of a stateless whole-pack decoder.
pub trait ArtifactPackReceiving:super::ArtifactPack+semio_framework_value::retirement::RetireOwned {
 fn receive_pack(bytes:&[u8],owner:&mut super::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>;
}

/// 🛬️ Borrows the envelope and binds every parsed field through the same decoder.
pub fn decode_native_snapshot_record<T>(payload:NativeSnapshotInput<'_>,envelope_id:&str,spec:RecordSpecProducer,construct:impl FnOnce(&RecordValue,&mut NativeDecodeControl<'_>)->Result<T,ValueError>,control:&mut NativeDecodeControl<'_>)->Result<T,ValueError>{
 control.checkpoint()?;
 let spec=spec.decode(control)?;
 let record=match payload {
  NativeSnapshotInput::Binary(bytes)=>{let body=super::semio_format::unwrap_binary_controlled(bytes,envelope_id,super::semio_format::Component::Pack,1,control)?;pack::record::decode_document_controlled(body,&spec,&pack::record::DecodeOptions::default(),control).map_err(super::PackRefusal::into_value_error)?.0}
  NativeSnapshotInput::Text(text)=>{let body=super::semio_format::split_text_preamble_controlled(text,envelope_id,super::semio_format::Component::Dsl,1,control).map_err(super::semio_format::SemioError::into_value_error)?;semio_framework_dsl_record::parse_exact_controlled(body,&spec,&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits{max_bytes:control.maximum_bytes(),..Default::default()},mode:semio_framework_dsl_record::SourceMode::Document},control).map_err(|error|ValueError::new(error.kind,error.message))?}
 };
 let result=construct(&record,control)?;
 control.checkpoint()?;
 Ok(result)
}

/// 🛫️ Admits the spec, projected fields, physical body and envelope before publication.
pub fn encode_native_snapshot_record(encoding:NativeSnapshotEncoding,envelope_id:&str,spec:RecordSpecProducer,construct:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>,control:&mut NativeEncodeControl<'_>)->Result<IoPayload,ValueError>{
 control.checkpoint()?;
 let component=match encoding{NativeSnapshotEncoding::Binary=>super::semio_format::Component::Pack,NativeSnapshotEncoding::Text=>super::semio_format::Component::Dsl};
 let body_limit=control.maximum_bytes().checked_sub(super::semio_format::declared_envelope_prefix_len(envelope_id,component,1)?).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"native ownership cannot contain its declared envelope"))?;
 let spec=spec.encode(control)?;
 let record=construct(control)?;
 let result=match encoding{
  NativeSnapshotEncoding::Binary=>{let mut options=pack::record::EncodeOptions::default();options.limits.max_file_len=body_limit as u64;let body=pack::record::encode_document_controlled(&spec,&record,&options,control).map_err(super::PackRefusal::into_value_error)?;IoPayload::Binary(super::semio_format::wrap_binary_controlled(envelope_id,component,1,&body,control)?)}
  NativeSnapshotEncoding::Text=>{let body=semio_framework_dsl_record::print_controlled(&record,&spec,semio_framework_dsl_record::JoinMode::Document,body_limit,control).map_err(|error|ValueError::new(error.kind,error.message))?;IoPayload::Text(super::semio_format::wrap_text_controlled(envelope_id,component,1,&body,control)?)}
 };
 control.checkpoint()?;
 Ok(result)
}

