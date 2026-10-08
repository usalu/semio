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
struct RasterLayerModificationDsl {id:String,#[dsl(block)] patch:RasterLayerPatch}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterLayerRemovalDsl {id:String,parent_id:Option<String>,index:usize}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterLayerRelocationDsl {id:String,from_parent_id:Option<String>,from_index:usize,to_parent_id:Option<String>,to_index:usize}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterLayersDeltaDsl {#[dsl(table)] removed:Vec<RasterLayerRemovalDsl>,#[dsl(table)] inserted:Vec<RasterLayerInsertionDsl>,#[dsl(table)] moved:Vec<RasterLayerRelocationDsl>,#[dsl(table)] modified:Vec<RasterLayerModificationDsl>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterPixelRegionDsl {layer_id:String,target:String,x:u32,y:u32,width:u32,height:u32,samples:String}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterDiffDsl {schema:Option<String>,id:Option<String>,title:Option<Option<String>>,#[dsl(block)] layers:Option<RasterLayersDeltaDsl>,#[dsl(block)] assets:Option<RasterAssetsDeltaDsl>,#[dsl(table)] pixels:Vec<RasterPixelRegionDsl>}
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
fn samples_to_hex(samples:&[u8])->String {samples.iter().map(|byte|format!("{byte:02x}")).collect()}
fn samples_from_hex(text:&str)->Result<Vec<u8>,semio_framework_diagnostic::TextError> {
 let invalid=||semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"pixel samples must be an even-length hex string",semio_framework_diagnostic::TextSpan::at(1,1));
 if text.len()%2!=0{return Err(invalid());}
 (0..text.len()/2).map(|at|text.get(at*2..at*2+2).and_then(|pair|u8::from_str_radix(pair,16).ok()).ok_or_else(invalid)).collect()
}
fn admit_diff_record(wire:RasterDiffDsl)->Result<RasterDiff,semio_framework_diagnostic::TextError> {
 let layers=wire.layers.map(|v|Ok(RasterLayersDelta{
  removed:v.removed.into_iter().map(|r|RasterLayerRemoval{id:r.id,parent_id:r.parent_id,index:r.index}).collect(),
  inserted:v.inserted.into_iter().map(|a|Ok(RasterLayerInsertion{parent_id:a.parent_id,index:a.index,layer:layer_from_dsl(a.layer)?})).collect::<Result<Vec<_>,semio_framework_diagnostic::TextError>>()?,
  moved:v.moved.into_iter().map(|m|RasterLayerRelocation{id:m.id,from:RasterLayerAddress{parent_id:m.from_parent_id,index:m.from_index},to:RasterLayerAddress{parent_id:m.to_parent_id,index:m.to_index}}).collect(),
  modified:v.modified.into_iter().map(|p|RasterLayerModification{id:p.id,patch:p.patch}).collect()})).transpose()?;
 let pixels=wire.pixels.into_iter().map(|r|Ok(RasterPixelRegion{layer_id:r.layer_id,target:r.target,x:r.x,y:r.y,width:r.width,height:r.height,samples:samples_from_hex(&r.samples)?})).collect::<Result<Vec<_>,semio_framework_diagnostic::TextError>>()?;
 Ok(RasterDiff{schema:wire.schema,id:wire.id,title:wire.title,layers,assets:wire.assets.map(|v|RasterAssetsDelta{entries:v.entries.into_iter().map(|a|(a.key,a.image.map(Into::into))).collect()}),pixels})
}
/// 📖️ Physical record layout shared by text and binary diff codecs.
pub(crate) fn raster_diff_record_spec()->RecordSpec {RasterDiffDsl::__dsl_spec()}
/// 📝️ Projects a semantic delta at its physical boundary and retires copied owners.
pub(crate) fn raster_diff_to_record(diff:&RasterDiff)->RecordValue {
 let wire=RasterDiffDsl{schema:diff.schema.clone(),id:diff.id.clone(),title:diff.title.clone(),
  layers:diff.layers.as_ref().map(|v|RasterLayersDeltaDsl{
   removed:v.removed.iter().map(|r|RasterLayerRemovalDsl{id:r.id.clone(),parent_id:r.parent_id.clone(),index:r.index}).collect(),
   inserted:v.inserted.iter().map(|a|RasterLayerInsertionDsl{parent_id:a.parent_id.clone(),index:a.index,layer:layer_to_dsl(&a.layer)}).collect(),
   moved:v.moved.iter().map(|m|RasterLayerRelocationDsl{id:m.id.clone(),from_parent_id:m.from.parent_id.clone(),from_index:m.from.index,to_parent_id:m.to.parent_id.clone(),to_index:m.to.index}).collect(),
   modified:v.modified.iter().map(|p|RasterLayerModificationDsl{id:p.id.clone(),patch:p.patch.clone()}).collect()}),
  assets:diff.assets.as_ref().map(|v|RasterAssetsDeltaDsl{entries:v.entries.iter().map(|(key,image)|RasterAssetDeltaDsl{key:key.clone(),image:image.as_ref().map(RasterImageDsl::from)}).collect()}),
  pixels:diff.pixels.iter().map(|r|RasterPixelRegionDsl{layer_id:r.layer_id.clone(),target:r.target.clone(),x:r.x,y:r.y,width:r.width,height:r.height,samples:samples_to_hex(&r.samples)}).collect()};
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
