//! 🫳️ Bake two adjacent normal-blend siblings into one undoable pixel layer.
use crate::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_model::{layer_node_id, layer_name};
use semio_s_artifact_raster_raster_model::RasterLayerNode;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_io::RasterStackPreparation;
use semio_framework_pixels::png_encoding::EncodedPngImage;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_value_derive::{FromValue,ToValue};
use super::flatten_layers::{baked_layer,asset_keys_except};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword="merge-down")]
#[value(rename_all="camelCase")]
pub struct MergeDown {pub layer_id:String}
pub fn prepare(command:&MergeDown,document:&RasterSnapshot)->Result<RasterStackPreparation,Fault> {
    let (_,_,layers)=crate::merge_down_plan(document,&command.layer_id)?;
    RasterStackPreparation::from_layers(document,layers).map_err(Fault::from)
}
pub fn publish(image:EncodedPngImage,origin:[f64;2],command:&MergeDown,document:&RasterSnapshot)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {
    use semio_s_artifact_raster_raster_mutations_pixels::{add_layer_asset, remove_layer_asset};
    use semio_s_artifact_raster_raster_mutations_layers::{create_layer, delete_layer};
    let (parent_id,index,layers)=crate::merge_down_plan(document,&command.layer_id)?;
    let (mut layer,key,asset)=baked_layer(image,origin,layer_name(&layers[1]),"merge");
    if let RasterLayerNode::Pixel {id,..}=&mut layer {*id=command.layer_id.clone();}
    let ids:std::collections::BTreeSet<_>=layers.iter().map(layer_node_id).collect();
    let previous=asset_keys_except(layers,&Default::default());
    let retained=asset_keys_except(&document.layers,&ids);
    let mut mutations=Vec::new();
    mutations.extend(layers.iter().map(|layer|RasterMutation::DeleteLayer(delete_layer::DeleteLayer {layer_id:layer_node_id(layer).to_owned()})));
    mutations.extend(previous.difference(&retained).filter(|prior|*prior!=&key&&document.assets.contains_key(prior)).map(|asset_id|RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset {asset_id:asset_id.clone()})));
    mutations.push(RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset {asset_id:key,asset}));
    mutations.push(RasterMutation::CreateLayer(create_layer::CreateLayer {parent_id,index,layer:Box::new(layer)}));
    Ok(Emit::mutations(mutations))
}
pub fn handle(_payload:&MergeDown,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {Err(Fault::from("raster.merge-requires-retained-work"))}
