//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RasterDiffText = String;



use crate::standards::v1::subsets::any::schema::diff::*;
use crate::standards::v1::subsets::any::io::text::mutations::RasterImageDsl;
use crate::{RasterLayerNode,RasterLayerPatch,RasterOwnedMap};
use semio_framework_dsl_record::{RecordValue,RecordSpec};
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterAssetDeltaDsl {key:String,#[dsl(block)] image:Option<RasterImageDsl>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterAssetsDeltaDsl {#[dsl(table)] entries:Vec<RasterAssetDeltaDsl>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterLayerInsertionDsl {parent_id:Option<String>,index:usize,#[dsl(block)] layer:RasterNativeDocument}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterLayerPatchEntryDsl {id:String,#[dsl(block)] patch:RasterLayerPatch}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterLayerMoveDsl {id:String,parent_id:Option<String>,index:usize}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterLayersDeltaDsl {#[dsl(table)] added:Vec<RasterLayerInsertionDsl>,removed:Vec<String>,#[dsl(table)] patched:Vec<RasterLayerPatchEntryDsl>,#[dsl(table)] moved:Vec<RasterLayerMoveDsl>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterDiffDsl {schema:Option<String>,id:Option<String>,title:Option<Option<String>>,#[dsl(block)] layers:Option<RasterLayersDeltaDsl>,#[dsl(block)] assets:Option<RasterAssetsDeltaDsl>}
use crate::RasterSnapshot;
use crate::standards::v1::subsets::any::io::text::snapshot::record::RasterNativeDocument;
fn document_to_dsl(snapshot:RasterSnapshot)->RasterNativeDocument {
 let document=RasterNativeDocument::ordinary(&snapshot);
 crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
 document
}
fn layer_to_dsl(layer:&RasterLayerNode)->RasterNativeDocument {document_to_dsl(RasterSnapshot{schema:String::new(),id:String::new(),title:None,layers:vec![layer.clone()],assets:RasterOwnedMap::new()})}
fn layer_from_dsl(document:RasterNativeDocument)->Result<RasterLayerNode,semio_framework_diagnostic::TextError> {
 let mut snapshot=document.ordinary_snapshot().map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))?;
 if snapshot.layers.len()!=1||!snapshot.assets.is_empty(){crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"a layer diff requires one literal layer",semio_framework_diagnostic::TextSpan::at(1,1)));}
 let layer=snapshot.layers.pop().expect("validated one literal layer");
 crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
 Ok(layer)
}
fn admit_diff_record(wire:RasterDiffDsl)->Result<RasterDiff,semio_framework_diagnostic::TextError> {
 let layers=wire.layers.map(|v|Ok(RasterLayersDelta{added:v.added.into_iter().map(|a|Ok(RasterLayerInsertion{parent_id:a.parent_id,index:a.index,layer:layer_from_dsl(a.layer)?})).collect::<Result<Vec<_>,semio_framework_diagnostic::TextError>>()?,removed:v.removed,patched:v.patched.into_iter().map(|p|RasterLayerPatchEntry{id:p.id,patch:p.patch}).collect(),moved:v.moved.into_iter().map(|m|RasterLayerMove{id:m.id,parent_id:m.parent_id,index:m.index}).collect()})).transpose()?;
 Ok(RasterDiff{schema:wire.schema,id:wire.id,title:wire.title,layers,assets:wire.assets.map(|v|RasterAssetsDelta{entries:v.entries.into_iter().map(|a|(a.key,a.image.map(Into::into))).collect()})})
}
/// 📖️ Physical record layout shared by text and binary diff codecs.
pub(crate) fn raster_diff_record_spec()->RecordSpec {RasterDiffDsl::__dsl_spec()}
/// 📝️ Projects a semantic delta at its physical boundary and retires copied owners.
pub(crate) fn raster_diff_to_record(diff:&RasterDiff)->RecordValue {
 let wire=RasterDiffDsl{schema:diff.schema.clone(),id:diff.id.clone(),title:diff.title.clone(),
  layers:diff.layers.as_ref().map(|v|RasterLayersDeltaDsl{added:v.added.iter().map(|a|RasterLayerInsertionDsl{parent_id:a.parent_id.clone(),index:a.index,layer:layer_to_dsl(&a.layer)}).collect(),removed:v.removed.clone(),patched:v.patched.iter().map(|p|RasterLayerPatchEntryDsl{id:p.id.clone(),patch:p.patch.clone()}).collect(),moved:v.moved.iter().map(|m|RasterLayerMoveDsl{id:m.id.clone(),parent_id:m.parent_id.clone(),index:m.index}).collect()}),
  assets:diff.assets.as_ref().map(|v|RasterAssetsDeltaDsl{entries:v.entries.iter().map(|(key,image)|RasterAssetDeltaDsl{key:key.clone(),image:image.as_ref().map(RasterImageDsl::from)}).collect()})};
 let record=wire.__dsl_to_record();
 record
}
/// 📖️ Reconstructs semantic delta owners from the physical record.
pub(crate) fn raster_diff_from_record(record:&RecordValue)->Result<RasterDiff,semio_framework_diagnostic::TextError> {RasterDiffDsl::__dsl_from_record(record).and_then(admit_diff_record)}
impl protocol::DiffText for RasterDiff {
 fn print_diff(&self)->String {semio_framework_dsl_record::print(&raster_diff_to_record(self),&raster_diff_record_spec(),semio_framework_dsl_record::JoinMode::Inline)}
 fn parse_diff(line:&str)->Result<Self,semio_framework_diagnostic::TextError> {
  let record=semio_framework_dsl_record::parse_exact(line,&raster_diff_record_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Inline})?;
  raster_diff_from_record(&record)
 }
}
