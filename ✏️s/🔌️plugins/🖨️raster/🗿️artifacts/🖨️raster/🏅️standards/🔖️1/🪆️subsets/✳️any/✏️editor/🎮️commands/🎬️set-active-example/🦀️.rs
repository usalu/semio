//! 🎬️ 🖼️ Raster play app commands command — `set-active-example`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::{add_layer_asset, create_layer, delete_layer, remove_layer_asset};
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::{layer_node_id, raster_example_document};
use crate::{raster_asset, RasterLayerNode, RasterOwnedMap, RasterSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🔁️ The ordered batch of semantic mutations carrying `current` to `next`. Raster deliberately has
/// no whole-document replace verb (`🎮️commands/📃️document/🦀️.rs` records why `setSnapshot` was
/// deleted), so loading an example is spelled as real, undoable operations: every root layer is
/// deleted first (deleting a `Group` cascades its whole subtree), then the asset pool is re-pointed,
/// then the example's own forest is planted at the root in declaration order — `create-layer` carries
/// a whole `RasterLayerNode` including its `children`, so one operation per root layer is one whole
/// subtree. Committed example media the DSL cannot inline is planted through
/// [`example_media_operations`] before the wire handles in `next.assets` are considered.
///
/// 🚚️ `next` is consumed, never cloned: `RasterLayerNode::clone` (and the derived `RasterSnapshot`
/// drop) trap on a populated `RasterOwnedMap` — the demo's brighten adjustment carries `brightness`/
/// `contrast` params — so every layer MOVES into its `create-layer` operation and the asset map is
/// drained page by page, leaving an empty shell that drops cleanly (react boot of ticket
/// 26/09/05/RASTER-PLUGIN-END-TO-END, 2026-09-16).
fn replace_document_operations(current: &RasterSnapshot, mut next: RasterSnapshot, example_id: &str) -> Vec<RasterMutation> {
    let mut operations = Vec::new();
    for layer in &current.layers {
        operations.push(RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: layer_node_id(layer).to_string() }));
    }
    for asset_id in current.assets.keys() {
        operations.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: asset_id.clone() }));
    }
    // 🖼️ UNCONDITIONALLY, not through [`example_media_operations`]: the loop above just emitted a
    // `remove-layer-asset` for EVERY asset the open document carries, so by the time this batch's
    // media lands the pool is empty — and `example_media_operations`'s guard reads `current`, i.e.
    // the pool as it was BEFORE those removals. Re-selecting the demo over a document that already
    // carried the materialized emblem (any edit that makes the layer forests differ takes this path,
    // the early-return in `handle` no longer applies) therefore removed the emblem and planted
    // nothing, leaving a pixel-less handle pool and a BLANK composite.
    operations.extend(example_media(example_id));
    for (asset_id, _) in next.assets.iter() {
        if let Some(asset) = raster_asset(&next.assets, asset_id) {
            operations.push(RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: asset_id.to_owned(), asset }));
        }
    }
    drain_owned_map(&mut next.assets);
    for (index, layer) in std::mem::take(&mut next.layers).into_iter().enumerate() {
        operations.push(RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index, layer: Box::new(layer) }));
    }
    operations
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

/// 🧹️ Dismantles a layer forest that is NOT being planted (the no-op path below): an adjustment's
/// `params` map must be drained before the node drops; groups recurse.
/// 🎞️ Media a committed example declares but its `.dsl.semio` carrier cannot carry — the same split
/// `📸️remodel`'s `example_media_operations` uses for synthetic-orbit frame PNGs.
pub(crate) fn example_media_operations(example_id: &str, current: &RasterSnapshot) -> Vec<RasterMutation> {
    if crate::raster_asset(&current.assets, "semio-emblem").is_some() {
        return Vec::new();
    }
    example_media(example_id)
}

/// 🎞️ The same media, planted UNCONDITIONALLY — what a caller needs when it has already emptied the
/// asset pool itself (see [`replace_document_operations`]). [`example_media_operations`] is this
/// plus the "the open document already resolves these pixels" guard, which only a caller that keeps
/// the pool may ask.
pub(crate) fn example_media(example_id: &str) -> Vec<RasterMutation> {
    if example_id != crate::examples::art_raster_demo::ID {
        return Vec::new();
    }
    vec![RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: "semio-emblem".into(), asset: crate::examples::art_raster_demo::emblem_image_asset() })]
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

/// 🎬️ Loads one registered example over the open document. An id this subset does not register is a
/// no-op rather than a fault — the navbar switcher is free-text on the wire, and an unknown id must
/// not destroy the open document.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let Some(mut example) = raster_example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };
    // 🟰 The shell replays `setActiveExample` on every boot; over a document that already carries
    // the example's forest, re-planting would delete and recreate every layer for no change. Only
    // the layer forest is compared: the store keeps its own `id`/`title`, and the example's asset
    // handles are planted through `example_media_operations` and any materialized child already in
    // `next.assets`. The unused example is dismantled, never dropped populated (see
    // `release_layer_forest`).
    let media = example_media_operations(&payload.example_id, doc.snapshot);
    if doc.snapshot.layers == example.layers && media.is_empty() {
        drain_owned_map(&mut example.assets);
        release_layer_forest(std::mem::take(&mut example.layers));
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(replace_document_operations(doc.snapshot, example, &payload.example_id)))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::standards::v1::subsets::any::schema::empty_raster_snapshot;

    #[test]
    fn demo_example_media_operation_mints_the_emblem() {
        let operations = example_media_operations(crate::examples::art_raster_demo::ID, &empty_raster_snapshot());
        assert_eq!(operations.len(), 1);
        assert!(matches!(&operations[0], RasterMutation::AddLayerAsset(_)));
    }

    /// ⚖️ LAW: the replace batch is self-contained — it removes the open asset pool, so it must plant
    /// the example's own media back in the SAME batch, whatever the open pool held. Reading the
    /// `current` pool for that decision (which `example_media_operations` does, correctly, for the
    /// "nothing to do" check in `handle`) made re-selecting the demo over an already-materialized
    /// document emit `remove-layer-asset semio-emblem` and nothing else: a pixel-less handle pool and
    /// a blank composite (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP).
    #[test]
    fn replacing_a_materialized_document_plants_the_example_media_it_just_removed() {
        let current = crate::standards::v1::subsets::any::schema::semio_example_document();
        assert!(crate::raster_asset(&current.assets, "semio-emblem").is_some(), "the open document starts with real pixels");
        let example = raster_example_document(crate::examples::art_raster_demo::ID).expect("the demo example document");
        let operations = replace_document_operations(&current, example, crate::examples::art_raster_demo::ID);
        let removed = operations.iter().filter(|operation| matches!(operation, RasterMutation::RemoveLayerAsset(remove) if remove.asset_id == "semio-emblem")).count();
        let planted = operations.iter().any(|operation| matches!(operation, RasterMutation::AddLayerAsset(add) if add.asset_id == "semio-emblem" && !add.asset.data.is_empty()));
        assert_eq!(removed, 1, "the batch removes the open pool");
        assert!(planted, "and plants real emblem pixels back in the same batch");
        for operation in operations {
            crate::standards::v1::subsets::any::schema::mutations::retire_raster_mutation(operation);
        }
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(current);
    }
}
//#endregion 🧪️Tests
