//! 📸️ Controlled native snapshot factories preserve the caller’s complete admission.
use semio_framework_dsl_record::{RecordSpecProducer,RecordValue};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
use crate::io_schema::IoPayload;

/// 🎞️ Declares the exact native physical carrier.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeSnapshotEncoding { Binary,Text }

/// 📥️ Borrows the exact admitted native physical source.
#[derive(Clone,Copy,Debug)]
pub enum NativeSnapshotInput<'a> {Binary(&'a [u8]),Text(&'a str)}

/// 🫴️ Requires native construction and publication under the original caller controls.
pub trait ArtifactNativeSnapshot:Sized {
 fn decode_native_snapshot(payload:NativeSnapshotInput<'_>,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>;
 fn encode_native_snapshot(&self,encoding:NativeSnapshotEncoding,control:&mut NativeEncodeControl<'_>)->Result<IoPayload,ValueError>;
}

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

/// 🌱️ Declares the schema-less native value bridge with actual caller admission.
impl ArtifactNativeSnapshot for semio_framework_value::DslValue{
 fn decode_native_snapshot(payload:NativeSnapshotInput<'_>,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
  match payload{NativeSnapshotInput::Text(text)=>semio_framework_pack_json::from_json_str_controlled(text,semio_framework_pack_json::JsonMemberPolicy::Reject,control),NativeSnapshotInput::Binary(bytes)=>{let spec=value_spec(control)?;let(mut record,_)=pack::record::decode_document_controlled(bytes,&spec,&pack::record::DecodeOptions::default(),control).map_err(super::PackRefusal::into_value_error)?;match record.fields.remove(&1){Some(semio_framework_dsl_record::FieldValue::Value(value)) if record.fields.is_empty()=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"native value bridge requires its one declared field"))}}}
 }
 fn encode_native_snapshot(&self,encoding:NativeSnapshotEncoding,control:&mut NativeEncodeControl<'_>)->Result<IoPayload,ValueError>{
  match encoding{NativeSnapshotEncoding::Text=>semio_framework_pack_json::to_json_string_controlled(self,control).map(IoPayload::Text),NativeSnapshotEncoding::Binary=>{let spec=value_spec(control)?;let mut fields=semio_framework_dsl_record::RecordFields::from_empty_slots(control.allocate_vec(1)?);fields.insert(1,semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value_controlled(self,control)?));let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(RecordValue{fields});pack::record::encode_document_controlled(&spec,record.as_record(),&pack::record::EncodeOptions::default(),control).map(IoPayload::Binary).map_err(super::PackRefusal::into_value_error)}}
 }
}
fn value_spec<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,ValueError>{let mut fields=control.allocate_vec(1)?;fields.push(semio_framework_dsl_record::FieldSpec{id:1,key:control.copy_text("value")?,position:None,shape:semio_framework_dsl_record::Shape::Value,optional:false,flatten:false,defines:None,is_call_name:false});Ok(semio_framework_dsl_record::RecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Lines,fields})}
