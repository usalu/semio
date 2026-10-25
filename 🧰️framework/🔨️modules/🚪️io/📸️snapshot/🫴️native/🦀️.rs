//! 🫴️ Native snapshot decoding and encoding retain the original caller controls.
use semio_framework_value::{NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_io_schema::IoPayload;
use semio_framework_dsl_record::RecordValue;
/// 🎞️ Declares the exact native physical carrier.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeSnapshotEncoding { Binary,Text }

/// 📥️ Borrows the exact admitted native physical source.
#[derive(Clone,Copy,Debug)]
pub enum NativeSnapshotInput<'a> {Binary(&'a [u8]),Text(&'a str)}

/// 📸️ Requires native construction and publication under the original caller controls.
pub trait ArtifactNativeSnapshot:Sized {
 fn decode_native_snapshot(payload:NativeSnapshotInput<'_>,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>;
 fn encode_native_snapshot(&self,encoding:NativeSnapshotEncoding,control:&mut NativeEncodeControl<'_>)->Result<IoPayload,ValueError>;
}

/// 🌱️ Declares the schema-less native value bridge with actual caller admission.
impl ArtifactNativeSnapshot for semio_framework_value::DslValue{
 fn decode_native_snapshot(payload:NativeSnapshotInput<'_>,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
  match payload{NativeSnapshotInput::Text(text)=>semio_framework_pack_json::from_json_str_controlled(text,semio_framework_pack_json::JsonMemberPolicy::Reject,control),NativeSnapshotInput::Binary(bytes)=>{let spec=value_spec(control)?;let(mut record,_)=crate::record::decode_document_controlled(bytes,&spec,&crate::record::DecodeOptions::default(),control).map_err(crate::PackRefusal::into_value_error)?;match record.fields.remove(&1){Some(semio_framework_dsl_record::FieldValue::Value(value)) if record.fields.is_empty()=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"native value bridge requires its one declared field"))}}}
 }
 fn encode_native_snapshot(&self,encoding:NativeSnapshotEncoding,control:&mut NativeEncodeControl<'_>)->Result<IoPayload,ValueError>{
  match encoding{NativeSnapshotEncoding::Text=>semio_framework_pack_json::to_json_string_controlled(self,control).map(IoPayload::Text),NativeSnapshotEncoding::Binary=>{let spec=value_spec(control)?;let mut fields=semio_framework_dsl_record::RecordFields::from_empty_slots(control.allocate_vec(1)?);fields.insert(1,semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value_controlled(self,control)?));let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(RecordValue{fields});crate::record::encode_document_controlled(&spec,record.as_record(),&crate::record::EncodeOptions::default(),control).map(IoPayload::Binary).map_err(crate::PackRefusal::into_value_error)}}
 }
}
fn value_spec<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,ValueError>{let mut fields=control.allocate_vec(1)?;fields.push(semio_framework_dsl_record::FieldSpec{id:1,key:control.copy_text("value")?,position:None,shape:semio_framework_dsl_record::Shape::Value,optional:false,flatten:false,defines:None,is_call_name:false});Ok(semio_framework_dsl_record::RecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Lines,fields})}
