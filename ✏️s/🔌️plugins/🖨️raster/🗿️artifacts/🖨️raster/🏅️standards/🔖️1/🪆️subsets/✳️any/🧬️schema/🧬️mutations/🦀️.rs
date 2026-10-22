//! 🧬️ Closed raster mutation vocabulary: layer structure, properties, assets, pixels and masks.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_s_artifact_raster_raster_diff::RasterDiff;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Leaves
use semio_s_artifact_raster_raster_mutations_pixels::add_layer_asset;
use semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_adjustment_kind;
use semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_blend_mode;
use semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_opacity;
use semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_visible;
use semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_locked;
use semio_s_artifact_raster_raster_mutations_layers::create_layer;
use semio_s_artifact_raster_raster_mutations_layers::delete_layer;
use semio_s_artifact_raster_raster_mutations_layers::move_layer;
use semio_s_artifact_raster_raster_mutations_pixels::remove_layer_asset;
use semio_s_artifact_raster_raster_mutations_layers::rename_layer;
use semio_s_artifact_raster_raster_mutations_layers::reorder_layers;
use semio_s_artifact_raster_raster_mutations_layers::resize_layer;
use semio_s_artifact_raster_raster_mutations_pixels::change_layer_pixels;
use semio_s_artifact_raster_raster_mutations_pixels::change_layer_mask;
use semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_transform;
use semio_s_artifact_raster_raster_mutations_layer_properties::change_layer_adjustment_parameter;
use semio_s_artifact_raster_raster_mutations_pixels::paint_stroke;
use semio_s_artifact_raster_raster_mutations_pixels::fill_region;
use semio_s_artifact_raster_raster_mutations_pixels::apply_filter;
use semio_s_artifact_raster_raster_mutations_pixels::transform_image;
use semio_s_artifact_raster_raster_mutations_pixels::fill_selection;
use semio_s_artifact_raster_raster_mutations_pixels::write_pixel_region;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the raster document, derived per
/// `📓️derivation-rules.md` from `RasterLayerNode`'s recursive tree shape and the `assets` root
/// collection.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RasterSnapshot, diff = RasterDiff, schema = "raster.raster", retire_cold = retire_raster_mutation, reference = RasterSnapshot, reference_diff = RasterDiff)]
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
        semio_s_artifact_raster_raster_model::retire_raster_layer(*value.layer);
    }
}

/// ⚡️ Convenience wrapper mirroring `apply_raster_mutation` — forwards to the derive's real
/// `Mutation::inverse`.
pub fn inverse_raster_mutation(snapshot: &RasterSnapshot, mutation: &RasterMutation) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok({
    protocol::Mutation::inverse(mutation, snapshot)?

    })
}

pub type RasterEnvelope = store::os_store::store_schema::ArtifactEnvelope<RasterSnapshot, RasterMutation>;
pub type RasterStore = store::ArtifactStore<RasterSnapshot, RasterMutation>;
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🎭️change-layer-mask/🧪️tests/🦀️.rs"]
mod mask_tests;
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge

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

//#endregion 🔖️Kinds

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog_tests;
//#endregion 🧪️KindsCatalog

//#region 🪢️TaxonomyMounts
#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;
//#endregion 🪢️TaxonomyMounts

#[path = "🌉️codec-bridge/🦀️.rs"]
mod codec_bridge;
pub use codec_bridge::*;
