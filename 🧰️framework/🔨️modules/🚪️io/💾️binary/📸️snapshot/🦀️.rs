//! 📸️ Product-neutral snapshot representation contracts.
use semio_framework_value::{DslValue,ToValue,FromValue};
use semio_framework_pack_error::PackError;
use crate::record::{EncodeOptions as PackEncodeOptions,DecodeOptions as PackDecodeOptions};
use semio_framework_dsl_record::{RecordSpecProducer,BorrowedRecordSpecProducer};
/// 🧬️ Native snapshot registration depends only on authentic immutable Pack metadata.
pub trait ArtifactPackMetadata:Sized{
    const EXTENSION:&'static str;
    fn native_record_spec_producer()->Option<BorrowedRecordSpecProducer>;
    fn native_pack_schema_hash()->[u8;32];
}
impl<T:ArtifactPack+super::ArtifactDsl> ArtifactPackMetadata for T{
    const EXTENSION:&'static str=<T as super::ArtifactDsl>::EXTENSION;
    fn native_record_spec_producer()->Option<BorrowedRecordSpecProducer>{<T as ArtifactPack>::borrowed_record_spec_producer()}
    fn native_pack_schema_hash()->[u8;32]{<T as ArtifactPack>::record_spec().map_or([0;32],|spec|crate::record::schema_hash(&spec))}
}

/// 📦️ A snapshot binary representation over first-party options, errors and authored record metadata.
pub trait ArtifactPack: Sized {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError>;
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError>;

    /// 🎒️ `encode_pack_with` at default options — infallible in practice (mirrors
    /// `ArtifactDsl::print_dsl`'s infallible signature); panics only on a `PackLimits` overflow.
    fn encode_pack(&self) -> Vec<u8> {
        self.encode_pack_with(&PackEncodeOptions::default()).expect("default-options pack encode is infallible")
    }

    /// 📕️ `decode_pack_with` at default (Standard) verification.
    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        Self::decode_pack_with(bytes, &PackDecodeOptions::default())
    }

    /// 🧬️ The authored structural field specification, absent for a schema-less value.
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        None
    }

    /// 🏭️ Exposes only an authentically authored native metadata constructor.
    fn record_spec_producer() -> Option<RecordSpecProducer> { None }

    /// 🫳️ Exposes an immutable authored declaration without creating schema owners.
    fn borrowed_record_spec_producer() -> Option<BorrowedRecordSpecProducer> { None }
}


impl ArtifactPack for semio_framework_value::DslValue {
 fn encode_pack_with(&self,options:&PackEncodeOptions)->Result<Vec<u8>,PackError>{
  let spec=semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Lines,vec![semio_framework_dsl_record::FieldSpec::new(1,"value",semio_framework_dsl_record::Shape::Value)]);
  let mut fields=semio_framework_dsl_record::RecordFields::default();fields.insert(1,semio_framework_dsl_record::FieldValue::Value(self.clone()));
  crate::record::encode_document(&spec,&semio_framework_dsl_record::RecordValue{fields},options).map_err(PackError::from)
 }
 fn decode_pack_with(bytes:&[u8],options:&PackDecodeOptions)->Result<Self,PackError>{
  let spec=semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Lines,vec![semio_framework_dsl_record::FieldSpec::new(1,"value",semio_framework_dsl_record::Shape::Value)]);
  let(record,_)=crate::record::decode_document(bytes,&spec,options).map_err(PackError::from)?;
  Ok(match record.get(1){Some(semio_framework_dsl_record::FieldValue::Value(value))=>value.clone(),_=>Self::Null})
 }
}

impl ArtifactPack for semio_framework_replication_interaction::InteractionState {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let value = <Self as ToValue>::to_value(self);
        <DslValue as ArtifactPack>::encode_pack_with(&value, options)
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let value = <DslValue as ArtifactPack>::decode_pack_with(bytes, options)?;
        <Self as FromValue>::from_value(value).map_err(PackError::from)
    }
}
