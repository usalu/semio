//! 🖼️ 🖼️ Raster play app commands command — `duplicate-layer`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::create_layer;
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::{clone_layer, find_layer, locate_layer, require_layer_edit};
use crate::RasterSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub layer_id: String,
}

/// 🧭️ Keeps the source coordinate frame and admits insertion only into an editable parent.
pub fn plan(document:&RasterSnapshot,id:&str)->Result<(Option<String>,usize),Fault> {
    let (parent,index)=locate_layer(&document.layers,id).ok_or_else(||Fault::from("raster-layer-not-found"))?;
    if let Some(parent)=&parent {require_layer_edit(&document.layers,parent,false).map_err(Fault::from)?;}
    Ok((parent,index+1))
}

/// 📑️ Creates a sibling copy and requests framework selection after publication.
pub fn handle(payload: &DuplicateLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document=doc.snapshot;
    let (parent_id,index)=plan(document,&payload.layer_id)?;
    let source=find_layer(&document.layers,&payload.layer_id).ok_or_else(||Fault::from("raster-layer-not-found"))?;
    let layer=clone_layer(source);
    let selection=crate::editor::raster::layer_selection::select_layer_effect(crate::standards::v1::subsets::any::schema::layer_node_id(&layer))?;
    Ok(Emit {artifact_mutations:vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer {parent_id,index,layer:Box::new(layer)})],effects:vec![selection],..Default::default()})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
