//! 🧬️ Closed raster mutation vocabulary: layer structure, properties, assets, pixels and masks.

use crate::diff::RasterDiff;
use crate::RasterSnapshot;

//#region 🔖️Leaves
use super::add_layer_asset;
use super::change_layer_adjustment_kind;
use super::change_layer_blend_mode;
use super::change_layer_opacity;
use super::change_layer_visible;
use super::change_layer_locked;
use super::create_layer;
use super::delete_layer;
use super::move_layer;
use super::remove_layer_asset;
use super::rename_layer;
use super::reorder_layers;
use super::resize_layer;
use super::change_layer_pixels;
use super::change_layer_mask;
use super::change_layer_transform;
use super::change_layer_adjustment_parameter;
use super::paint_stroke;
use super::fill_region;
use super::apply_filter;
use super::transform_image;
use super::fill_selection;
use super::write_pixel_region;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the raster document, derived per
/// `📓️derivation-rules.md` from `RasterLayerNode`'s recursive tree shape and the `assets` root
/// collection.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RasterSnapshot, diff = RasterDiff, schema = "raster.raster", retire_cold = retire_raster_mutation)]
pub enum RasterMutation {
    CreateLayer(create_layer::CreateLayer),
    DeleteLayer(delete_layer::DeleteLayer),
    ReorderLayers(reorder_layers::ReorderLayers),
    RenameLayer(rename_layer::RenameLayer),
    ChangeLayerVisible(change_layer_visible::ChangeLayerVisible),
    ChangeLayerLocked(change_layer_locked::ChangeLayerLocked),
    ChangeLayerOpacity(change_layer_opacity::ChangeLayerOpacity),
    ChangeLayerBlendMode(change_layer_blend_mode::ChangeLayerBlendMode),
    MoveLayer(move_layer::MoveLayer),
    ResizeLayer(resize_layer::ResizeLayer),
    ChangeLayerAdjustmentKind(change_layer_adjustment_kind::ChangeLayerAdjustmentKind),
    AddLayerAsset(add_layer_asset::AddLayerAsset),
    RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset),
    ChangeLayerPixels(change_layer_pixels::ChangeLayerPixels),
    ChangeLayerMask(change_layer_mask::ChangeLayerMask),
    ChangeLayerTransform(change_layer_transform::ChangeLayerTransform),
    ChangeLayerAdjustmentParameter(change_layer_adjustment_parameter::ChangeLayerAdjustmentParameter),
    PaintStroke(paint_stroke::PaintStroke),
    FillRegion(fill_region::FillRegion),
    ApplyFilter(apply_filter::ApplyFilter),
    TransformImage(transform_image::TransformImage),
    FillSelection(fill_selection::FillSelection),
    WritePixelRegion(write_pixel_region::WritePixelRegion),
}

/// 🧯️ Cold disposal of a scratch mutation nobody will apply again — the store retires decoded
/// arrivals, replay clones and rebased inverses through `Mutation::retire_cold`, and `CreateLayer`
/// is the one leaf that owns a layer subtree (so, through an `Adjustment`, a fail-closed
/// `RasterOwnedMap`). Every other leaf carries plain scalars and needs nothing.
pub fn retire_raster_mutation(mutation: RasterMutation) {
    if let RasterMutation::CreateLayer(value) = mutation {
        crate::retire_raster_layer(*value.layer);
    }
}

/// ⚡️ Convenience wrapper mirroring `apply_raster_mutation` — forwards to the derive's real
/// `Mutation::inverse`.
pub fn inverse_raster_mutation(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
    protocol::Mutation::inverse(mutation, snapshot)?

    })
}

pub type RasterEnvelope = store::ArtifactEnvelope<RasterSnapshot, RasterMutation>;
pub type RasterStore = store::ArtifactStore<RasterSnapshot, RasterMutation>;
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🎭️change-layer-mask/🧪️tests/🦀️.rs"]
mod mask_tests;
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge




/// 🧹️ Cold-retires a batch of operations nobody will apply — a `create-layer` inverse can carry a
/// whole subtree whose adjustment layers own populated `params` maps.
pub(crate) fn retire_bridge_mutations(mutations: Vec<RasterMutation>) {
    for mutation in mutations {
        protocol::Mutation::retire_cold(mutation);
    }
}




/// 🌉️ Applies one committed mutation payload to one committed before-document and answers
/// `{"snapshot": …, "messages": [ … ]}`.
///
/// The bridge exists because the generated Rust test host links only `semio-repo-test-host` and,
/// behind its `sut` feature, this crate — `dsl`, `protocol` and `store` are private
/// extern-crate aliases (`🦀️.rs`) and cannot be named from a case adapter. Same shape and same
/// reason as `🗄️stdio`'s `decode_semio_mesh_mutation_json`/`apply_semio_mesh_mutation` pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9


/// ↩️ Applies one committed mutation payload and then EVERY step of its own computed inverse,
/// answering in the same shape — the metamorphic half of what `🖨️mutate-raster-1` compares against its
/// Python second implementation. The inverse is computed against the PRE-mutation document, which is
/// the only state that carries what a delete removed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9



//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `RasterMutation` variant, in declaration order — the vocabulary
/// the `raster-1-any` catalog (`../../🔣️oracle.json`) declares and the
/// `🖨️mutate-raster-1` exhaustive case measures itself against. 
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against the enum,
/// since the framework never parses Rust.
pub const KINDS: &[&str] =
    &["create-layer", "delete-layer", "reorder-layers", "rename-layer", "change-layer-visible", "change-layer-locked", "change-layer-opacity", "change-layer-blend-mode", "move-layer", "resize-layer", "change-layer-adjustment-kind", "add-layer-asset", "remove-layer-asset", "change-layer-pixels", "change-layer-mask", "change-layer-transform", "change-layer-adjustment-parameter", "paint-stroke", "fill-region", "apply-filter", "transform-image", "fill-selection", "write-pixel-region"];
//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
//#endregion 🧪️KindsCatalog
