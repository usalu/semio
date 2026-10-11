//! 🫳️ Bake two adjacent normal-blend siblings into one undoable pixel layer.
use crate::editor::raster::config::{RasterConfig,RasterConfigMutation};
use crate::standards::v1::subsets::any::schema::{find_layer,locate_layer,layer_node_id,layer_name,layer_visible,layer_blend_mode};
use crate::{RasterLayerNode,RasterMutation,RasterSnapshot};
use crate::standards::v1::subsets::any::io::RasterStackPreparation;
use semio_framework_pixels::png_encoding::EncodedPngImage;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_value_derive::{FromValue,ToValue};
use super::flatten_layers::{baked_layer,asset_keys_except};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword="merge-down")]
#[value(rename_all="camelCase")]
pub struct MergeDown {pub layer_id:String}
pub(crate) fn plan<'a>(document:&'a RasterSnapshot,layer_id:&str)->Result<(Option<String>,usize,&'a [RasterLayerNode]),Fault> {
    if layer_id.trim().is_empty()||layer_id.chars().count()>128 {return Err(Fault::from("raster.merge-target-invalid"));}
    let (parent,index)=locate_layer(&document.layers,layer_id).ok_or_else(||Fault::from("raster.merge-target-missing"))?;
    if index==0 {return Err(Fault::from("raster.merge-no-lower-layer"));}
    let siblings=match parent.as_deref().and_then(|id|find_layer(&document.layers,id)) {Some(RasterLayerNode::Group {children,..})=>children.as_slice(),_=>document.layers.as_slice()};
    let pair=&siblings[index-1..=index];
    if pair.iter().any(|layer|!layer_visible(layer)||layer_blend_mode(layer)!="normal"||matches!(layer,RasterLayerNode::Adjustment {..})) {return Err(Fault::from("raster.merge-backdrop-dependent"));}
    for layer in pair {crate::standards::v1::subsets::any::schema::require_layer_edit(&document.layers,layer_node_id(layer),true).map_err(Fault::from)?;}
    Ok((parent,index-1,pair))
}
pub(crate) fn prepare(command:&MergeDown,document:&RasterSnapshot)->Result<RasterStackPreparation,Fault> {
    let (_,_,layers)=plan(document,&command.layer_id)?;
    RasterStackPreparation::from_layers(document,layers).map_err(Fault::from)
}
pub(crate) fn publish(image:EncodedPngImage,origin:[f64;2],command:&MergeDown,document:&RasterSnapshot)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {
    use crate::mutations::{add_layer_asset,create_layer,delete_layer,remove_layer_asset};
    let (parent_id,index,layers)=plan(document,&command.layer_id)?;
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
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
