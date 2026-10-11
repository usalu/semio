//! 🎬️ 🖼️ Raster play app commands command — `set-active-example`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::raster_example_document;
use crate::{RasterLayerNode, RasterOwnedMap, RasterSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🧹️ Empties one owned map entry by entry and releases every page backing, so the map reaches
/// `Drop` in the only state its guard admits. `ArtifactChild`/`DslValue` entries carry no drop guard
/// of their own.
fn drain_owned_map<V>(map: &mut RasterOwnedMap<V>) {
    while map.take_last_entry().is_some() {}
    while let Some(page) = map.take_empty_page_backing() {
        page.release();
    }
}

/// 🎞️ Media a committed example declares but its `.dsl.semio` carrier cannot carry — the same split
/// `📸️remodel`'s `example_media_operations` uses for synthetic-orbit frame PNGs.
pub(crate) fn example_media(example_id: &str) -> Vec<(String, crate::SemioImageSnapshot)> {
    if example_id != crate::examples::art_raster_demo::ID {
        return Vec::new();
    }
    vec![("semio-emblem".to_owned(), crate::standards::v1::subsets::any::io::semio_image_snapshot_from_raster_asset(&crate::examples::art_raster_demo::emblem_image_asset()).expect("curated image decodes"))]
}

/// 🌱️ The registered example as a complete document, its declared media already planted in the asset pool.
pub(crate) fn example_document(example_id: &str) -> Option<RasterSnapshot> {
    let mut document = raster_example_document(example_id)?;
    for (asset_id, image) in example_media(example_id) {
        if !document.assets.contains_key(&asset_id) {
            let handle = crate::mint_raster_image_child(&asset_id, &image);
            if document.assets.insert(asset_id, handle).is_err() {
                dismantle(document);
                return None;
            }
        }
    }
    Some(document)
}

/// 🔍️ Whether the open pool already resolves every pixel the example's media declares.
fn example_media_present(example_id: &str, current: &RasterSnapshot) -> bool {
    example_media(example_id).iter().all(|(asset_id, _)| crate::raster_asset(&current.assets, asset_id).is_some())
}

/// 🧹️ Dismantles a document that is not handed on: the asset pool is drained and the layer forest released before the drop.
fn dismantle(mut document: RasterSnapshot) {
    drain_owned_map(&mut document.assets);
    release_layer_forest(std::mem::take(&mut document.layers));
}

fn release_layer_forest(layers: Vec<RasterLayerNode>) {
    for layer in layers {
        match layer {
            RasterLayerNode::Adjustment { mut params, .. } => drain_owned_map(&mut params),
            RasterLayerNode::Group { children, .. } => release_layer_forest(children),
            RasterLayerNode::Pixel { .. } => {}
        }
    }
}

/// 🎬️ Loads one registered example as the open document through the load effect — no mutation rows, no history row. An
/// id this subset does not register is a no-op rather than a fault — the navbar switcher is free-text on the wire, and an
/// unknown id must not destroy the open document. The shell replays `setActiveExample` on every boot, so a document that
/// already carries the example's forest and media is left alone.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    for layer in &doc.snapshot.layers {crate::standards::v1::subsets::any::schema::require_layer_edit(&doc.snapshot.layers,crate::standards::v1::subsets::any::schema::layer_node_id(layer),true).map_err(Fault::from)?;}
    let Some(example) = example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };
    if doc.snapshot.layers == example.layers && example_media_present(&payload.example_id, doc.snapshot) {
        dismantle(example);
        return Ok(Emit::default());
    }
    let effect = crate::editor::raster::raster_reset_document_effect(&example);
    dismantle(example);
    Ok(Emit { effects: vec![effect], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
