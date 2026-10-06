//! 📝️ Text representation codec surface for `stdio.tiff` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type TiffSnapshotText = String;
//#endregion 🚚️Carrier


use crate::standards::v6_0::subsets::document::schema::snapshot::*;
use semio_framework_value::{FromValue,ToValue,ValueError,ValueRefusalKind};
use semio_framework_dsl_record::{FieldSpec,FieldValue,RecordLayout,RecordSpec,RecordSpecProducer,RecordValue,Shape};

const FIELDS:[(u16,&str,Shape);3]=[(1,"schema",Shape::Text),(2,"byteOrder",Shape::Value),(3,"ifds",Shape::Value)];
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
/// \U0001f9ec\U0000fe0f Declares the complete authored TIFF snapshot independently of its natural image file.
pub fn spec()->RecordSpec{RecordSpec::new(None,RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape)|FieldSpec::new(id,key,shape)).collect())}
fn spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<RecordSpec,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<FieldSpec>(FIELDS.len())?;for(id,key,shape)in FIELDS{fields.push(semio_framework_dsl_record::producer::field(id,key,shape,control)?);control.step()?;}semio_framework_dsl_record::producer::record(None,RecordLayout::Lines,fields,control)})
}
/// \U0001f3ed\U0000fe0f Produces the same literal three-field declaration under both ownership controllers.
pub fn spec_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}
/// \U0001f4e4\U0000fe0f Projects schema, byte order and every typed directory field into the declared native record.
pub fn to_record(snapshot:&TiffSnapshot)->RecordValue{to_record_controlled(snapshot,&mut semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut |_|true)).expect("complete TIFF native projection")}
fn project<T:ToValue>(value:&T,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control).map(FieldValue::Value)})}
/// \U0001f6a6\U0000fe0f Pays for every actual field and keeps nested projection stages independent.
pub fn to_record_controlled(snapshot:&TiffSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(3)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(3,control)?;
 record.insert(1,FieldValue::Text(control.copy_text(&snapshot.schema)?))?;control.step()?;
 record.insert(2,project(&snapshot.byte_order,control)?)?;control.step()?;
 record.insert(3,project(&snapshot.ifds,control)?)?;control.step()?;Ok(record.take())})
}
fn value(record:&RecordValue,id:u16)->Result<&semio_framework_value::DslValue,ValueError>{match record.get(id){Some(FieldValue::Value(value))=>Ok(value),_=>Err(invalid("TIFF native typed field has a different shape"))}}
/// \U0001f4e5\U0000fe0f Restores exactly the complete declared owner, including absent storage word metadata.
pub fn from_record(record:RecordValue)->Result<TiffSnapshot,semio_framework_diagnostic::TextError>{from_record_controlled(&record,&mut semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut |_|true)).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))}
/// \U0001f6a6\U0000fe0f Copies the actual typed owner directly from the borrowed record under one allocation ledger.
pub fn from_record_controlled(record:&RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<TiffSnapshot,ValueError>{
 if record.fields.len()!=3||record.fields.keys().any(|id|!(1..=3).contains(id)){return Err(invalid("TIFF native owner requires exactly its three declared root fields"))}
 control.scoped_stage(|control|{control.begin_stage(3)?;let schema=match record.get(1){Some(FieldValue::Text(value))=>control.copy_text(value)?,_=>return Err(invalid("TIFF native schema is not text"))};control.step()?;
 let byte_order=control.scoped_stage(|control|TiffByteOrder::from_value_controlled(value(record,2)?,control))?;control.step()?;
 let ifds=control.scoped_stage(|control|Vec::<TiffIfd>::from_value_controlled(value(record,3)?,control))?;control.step()?;Ok(TiffSnapshot{schema,byte_order,ifds})})
}

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v6_0::subsets::document::schema::snapshot::*;
use crate::STDIO_TIFF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for TiffSnapshot {
 const EXTENSION:&'static str="tiff";
 fn envelope_id()->&'static str{"stdio.tiff"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
  let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
  if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"TIFF owned Text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}
  crate::standards::v6_0::subsets::document::io::text::snapshot::from_record(semio_framework_dsl_record::parse_exact(body,&crate::standards::v6_0::subsets::document::io::text::snapshot::spec(),&semio_framework_dsl_record::ParseOptions::default())?)
 }
 fn print_dsl(&self)->String{
  let body=semio_framework_dsl_record::print(&crate::standards::v6_0::subsets::document::io::text::snapshot::to_record(self),&crate::standards::v6_0::subsets::document::io::text::snapshot::spec(),semio_framework_dsl_record::JoinMode::Document);
  let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared TIFF envelope");store::semio_format::wrap_text(&envelope,&body)
 }
}
}
pub use snapshot_codec::*;
