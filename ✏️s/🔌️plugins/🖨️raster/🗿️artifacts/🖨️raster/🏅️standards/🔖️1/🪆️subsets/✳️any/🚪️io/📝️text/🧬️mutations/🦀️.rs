//! 🔧️ Raster artifact — OpText/OpBinary codecs for `RasterMutation`. Mutation apply/inverse live in
//! `🧬️mutations`; this facet only handcrafts the op wire forms. The old hand-written `RasterMutation`
//! derived `dsl::DslEnum` directly; now that the dispatch enum derives `dsl::Mutations` (one unnamed
//! field per variant), that path is gone, so this leaf mirrors `din16798`'s bridge pattern: a private
//! `RasterMutationDsl` enum flattens every real variant into its own keyworded record, converted at
//! the `OpText`/`OpBinary` boundary only — `RasterMutation` itself is untouched.

use crate::mutations::{apply_filter,transform_image,fill_selection,fill_region,paint_stroke,change_layer_transform,change_layer_locked,change_layer_adjustment_parameter, change_layer_mask, change_layer_pixels, add_layer_asset, change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, create_layer, delete_layer, move_layer, remove_layer_asset, rename_layer, reorder_layers, resize_layer};
pub use crate::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterEnvelope, RasterMutation, RasterStore};
use crate::{SemioImageSnapshot, RasterLayerNode};
use protocol::OpText;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_dsl_record_derive::DslScalar)]
enum RasterImageColorspaceDsl { Rgb, Rgba, Grayscale, GrayscaleAlpha, Indexed }

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
struct RasterImageFrameDsl { delay_ms: u32, rgba8: Vec<u8> }

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
struct RasterImageMetadataDsl { key: String, value: String }

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct RasterImageDsl {
    schema: String,
    width: u32,
    height: u32,
    colorspace: RasterImageColorspaceDsl,
    bit_depth: u8,
    #[dsl(table)]
    frames: Vec<RasterImageFrameDsl>,
    icc: Option<Vec<u8>>,
    #[dsl(table)]
    metadata: Vec<RasterImageMetadataDsl>,
}

impl From<&SemioImageSnapshot> for RasterImageDsl {
    fn from(image: &SemioImageSnapshot) -> Self {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioColorspace;
        Self {
            schema: image.schema.clone(), width: image.width, height: image.height, bit_depth: image.bit_depth,
            colorspace: match image.colorspace { SemioColorspace::Rgb => RasterImageColorspaceDsl::Rgb, SemioColorspace::Rgba => RasterImageColorspaceDsl::Rgba, SemioColorspace::Grayscale => RasterImageColorspaceDsl::Grayscale, SemioColorspace::GrayscaleAlpha => RasterImageColorspaceDsl::GrayscaleAlpha, SemioColorspace::Indexed => RasterImageColorspaceDsl::Indexed },
            frames: image.frames.iter().map(|frame| RasterImageFrameDsl { delay_ms: frame.delay_ms, rgba8: frame.rgba8.clone() }).collect(),
            icc: image.icc.clone(), metadata: image.metadata.iter().map(|entry| RasterImageMetadataDsl { key: entry.key.clone(), value: entry.value.clone() }).collect(),
        }
    }
}

impl From<RasterImageDsl> for SemioImageSnapshot {
    fn from(image: RasterImageDsl) -> Self {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry};
        Self {
            schema: image.schema, width: image.width, height: image.height, bit_depth: image.bit_depth,
            colorspace: match image.colorspace { RasterImageColorspaceDsl::Rgb => SemioColorspace::Rgb, RasterImageColorspaceDsl::Rgba => SemioColorspace::Rgba, RasterImageColorspaceDsl::Grayscale => SemioColorspace::Grayscale, RasterImageColorspaceDsl::GrayscaleAlpha => SemioColorspace::GrayscaleAlpha, RasterImageColorspaceDsl::Indexed => SemioColorspace::Indexed },
            frames: image.frames.into_iter().map(|frame| SemioImageFrame { delay_ms: frame.delay_ms, rgba8: frame.rgba8 }).collect(),
            icc: image.icc, metadata: image.metadata.into_iter().map(|entry| SemioImageMetadataEntry { key: entry.key, value: entry.value }).collect(),
        }
    }
}

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `RasterMutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub(crate) enum RasterMutationDsl {
    CreateLayer {
        #[dsl(key = "parent")]
        parent_id: Option<String>,
        index: usize,
        #[dsl(statements)]
        layer: Box<RasterLayerNode>,
    },
    DeleteLayer {
        #[dsl(key = "id")]
        layer_id: String,
    },
    ReorderLayers {
        #[dsl(key = "id")]
        layer_id: String,
        #[dsl(key = "parent")]
        parent_id: Option<String>,
        index: usize,
    },
    RenameLayer {
        #[dsl(key = "id")]
        layer_id: String,
        new_name: String,
    },
    ChangeLayerLocked {
        #[dsl(key = "id")]
        layer_id:String,
        expected:bool,
        locked:bool,
    },
    ChangeLayerVisible {
        #[dsl(key = "id")]
        layer_id: String,
        new_visible: bool,
    },
    ChangeLayerOpacity {
        #[dsl(key = "id")]
        layer_id: String,
        new_opacity: f32,
    },
    ChangeLayerBlendMode {
        #[dsl(key = "id")]
        layer_id: String,
        new_blend_mode: String,
    },
    MoveLayer {
        #[dsl(key = "id")]
        layer_id: String,
        new_x: f64,
        new_y: f64,
    },
    ResizeLayer {
        #[dsl(key = "id")]
        layer_id: String,
        new_width: u32,
        new_height: u32,
    },
    ChangeLayerAdjustmentKind {
        #[dsl(key = "id")]
        layer_id: String,
        new_adjustment_kind: String,
    },
    AddLayerAsset {
        #[dsl(key = "id")]
        asset_id: String,
        #[dsl(block)]
        asset: RasterImageDsl,
    },
    ChangeLayerAdjustmentParameter {
        #[dsl(key = "id")]
        layer_id:String,
        parameter:String,
        #[dsl(key = "expected")]
        expected:Option<crate::RasterAdjustmentNumber>,
        #[dsl(key = "value")]
        value:Option<crate::RasterAdjustmentNumber>,
    },
    ChangeLayerTransform {
        #[dsl(key="id")]
        layer_id:String,
        #[dsl(block)]
        expected:crate::RasterTransform,
        #[dsl(block)]
        transform:crate::RasterTransform,
    },
    ChangeLayerMask {
        #[dsl(key = "id")]
        layer_id: String,
        #[dsl(block)]
        expected: Option<crate::RasterLayerMask>,
        #[dsl(block)]
        mask: Option<crate::RasterLayerMask>,
    },
    ChangeLayerPixels {
        #[dsl(key = "id")]
        layer_id: String,
        expected_image_key: Option<String>,
        #[dsl(block)]
        content: crate::RasterPixelContent,
        #[dsl(block)]
        transform: Option<crate::RasterTransform>,
    },
    RemoveLayerAsset {
        #[dsl(key = "id")]
        asset_id: String,
    },
    PaintStroke {
        #[dsl(key = "id")]
        layer_id: String,
        target: String,
        tool: String,
        size: f64,
        hardness: f64,
        opacity: f64,
        color: Vec<f64>,
        xs: Vec<f64>,
        ys: Vec<f64>,
        selection: Option<String>,
    },
    FillRegion {
        #[dsl(key = "id")]
        layer_id: String,
        target: String,
        x: u32,
        y: u32,
        tolerance: u32,
        color: Vec<f64>,
        selection: Option<String>,
    },
    ApplyFilter {
        #[dsl(key = "id")]
        layer_id: String,
        filter: String,
        amount: f64,
        selection: Option<String>,
    },
    TransformImage {
        #[dsl(key = "id")]
        layer_id: String,
        operation: String,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        bilinear: bool,
    },
    FillSelection {
        #[dsl(key = "id")]
        layer_id: String,
        target: String,
        color: Vec<f64>,
        selection: Option<String>,
    },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for RasterMutationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}


//#endregion 🔖️HandcraftedOpCodecs

pub(crate) fn raster_mutation_to_dsl(mutation: &RasterMutation) -> RasterMutationDsl {
    match mutation {
        RasterMutation::CreateLayer(payload) => RasterMutationDsl::CreateLayer { parent_id: payload.parent_id.clone(), index: payload.index, layer: payload.layer.clone() },
        RasterMutation::DeleteLayer(payload) => RasterMutationDsl::DeleteLayer { layer_id: payload.layer_id.clone() },
        RasterMutation::ReorderLayers(payload) => RasterMutationDsl::ReorderLayers { layer_id: payload.layer_id.clone(), parent_id: payload.parent_id.clone(), index: payload.index },
        RasterMutation::RenameLayer(payload) => RasterMutationDsl::RenameLayer { layer_id: payload.layer_id.clone(), new_name: payload.new_name.clone() },
        RasterMutation::ChangeLayerLocked(payload)=>RasterMutationDsl::ChangeLayerLocked {layer_id:payload.layer_id.clone(),expected:payload.expected,locked:payload.locked},
        RasterMutation::ChangeLayerVisible(payload) => RasterMutationDsl::ChangeLayerVisible { layer_id: payload.layer_id.clone(), new_visible: payload.new_visible },
        RasterMutation::ChangeLayerOpacity(payload) => RasterMutationDsl::ChangeLayerOpacity { layer_id: payload.layer_id.clone(), new_opacity: payload.new_opacity },
        RasterMutation::ChangeLayerBlendMode(payload) => RasterMutationDsl::ChangeLayerBlendMode { layer_id: payload.layer_id.clone(), new_blend_mode: payload.new_blend_mode.clone() },
        RasterMutation::MoveLayer(payload) => RasterMutationDsl::MoveLayer { layer_id: payload.layer_id.clone(), new_x: payload.new_x, new_y: payload.new_y },
        RasterMutation::ResizeLayer(payload) => RasterMutationDsl::ResizeLayer { layer_id: payload.layer_id.clone(), new_width: payload.new_width, new_height: payload.new_height },
        RasterMutation::ChangeLayerAdjustmentKind(payload) => RasterMutationDsl::ChangeLayerAdjustmentKind { layer_id: payload.layer_id.clone(), new_adjustment_kind: payload.new_adjustment_kind.clone() },
        RasterMutation::AddLayerAsset(payload) => RasterMutationDsl::AddLayerAsset { asset_id: payload.asset_id.clone(), asset: RasterImageDsl::from(&payload.asset) },
        RasterMutation::ChangeLayerAdjustmentParameter(payload) => RasterMutationDsl::ChangeLayerAdjustmentParameter {layer_id:payload.layer_id.clone(),parameter:payload.parameter.clone(),expected:payload.expected,value:payload.value},
        RasterMutation::ChangeLayerTransform(payload)=>RasterMutationDsl::ChangeLayerTransform {layer_id:payload.layer_id.clone(),expected:payload.expected.clone(),transform:payload.transform.clone()},
        RasterMutation::ChangeLayerMask(payload) => RasterMutationDsl::ChangeLayerMask { layer_id: payload.layer_id.clone(), expected: payload.expected.clone(), mask: payload.mask.clone() },
        RasterMutation::ChangeLayerPixels(payload) => RasterMutationDsl::ChangeLayerPixels { layer_id: payload.layer_id.clone(), expected_image_key: payload.expected_image_key.clone(), content: payload.content.clone(), transform: payload.transform.clone() },
        RasterMutation::RemoveLayerAsset(payload) => RasterMutationDsl::RemoveLayerAsset { asset_id: payload.asset_id.clone() },
        RasterMutation::PaintStroke(payload) => RasterMutationDsl::PaintStroke {
            layer_id: payload.layer_id.clone(),
            target: payload.target.clone(),
            tool: payload.tool.clone(),
            size: payload.brush.size,
            hardness: payload.brush.hardness,
            opacity: payload.brush.opacity,
            color: payload.brush.color.clone(),
            xs: payload.points.iter().map(|point| point.x).collect(),
            ys: payload.points.iter().map(|point| point.y).collect(),
            selection: payload.selection.as_ref().map(semio_framework_pack_json::to_json_string),
        },
        RasterMutation::FillRegion(payload) => RasterMutationDsl::FillRegion {
            layer_id: payload.layer_id.clone(),
            target: payload.target.clone(),
            x: payload.seed.x,
            y: payload.seed.y,
            tolerance: payload.tolerance,
            color: payload.color.clone(),
            selection: payload.selection.as_ref().map(semio_framework_pack_json::to_json_string),
        },
        RasterMutation::ApplyFilter(payload) => RasterMutationDsl::ApplyFilter {
            layer_id: payload.layer_id.clone(),
            filter: payload.filter.clone(),
            amount: payload.amount,
            selection: payload.selection.as_ref().map(semio_framework_pack_json::to_json_string),
        },
        RasterMutation::TransformImage(payload) => RasterMutationDsl::TransformImage {
            layer_id: payload.layer_id.clone(),
            operation: payload.operation.clone(),
            x: payload.x,
            y: payload.y,
            width: payload.width,
            height: payload.height,
            bilinear: payload.bilinear,
        },
        RasterMutation::FillSelection(payload) => RasterMutationDsl::FillSelection {
            layer_id: payload.layer_id.clone(),
            target: payload.target.clone(),
            color: payload.color.clone(),
            selection: payload.selection.as_ref().map(semio_framework_pack_json::to_json_string),
        },
    }
}

pub(crate) fn raster_mutation_from_dsl(mutation: RasterMutationDsl) -> RasterMutation {
    match mutation {
        RasterMutationDsl::CreateLayer { parent_id, index, layer } => RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id, index, layer }),
        RasterMutationDsl::DeleteLayer { layer_id } => RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id }),
        RasterMutationDsl::ReorderLayers { layer_id, parent_id, index } => RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id, parent_id, index }),
        RasterMutationDsl::RenameLayer { layer_id, new_name } => RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id, new_name }),
        RasterMutationDsl::ChangeLayerLocked {layer_id,expected,locked}=>RasterMutation::ChangeLayerLocked(change_layer_locked::ChangeLayerLocked {layer_id,expected,locked}),
        RasterMutationDsl::ChangeLayerVisible { layer_id, new_visible } => RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id, new_visible }),
        RasterMutationDsl::ChangeLayerOpacity { layer_id, new_opacity } => RasterMutation::ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity { layer_id, new_opacity }),
        RasterMutationDsl::ChangeLayerBlendMode { layer_id, new_blend_mode } => RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::mutation::ChangeLayerBlendMode { layer_id, new_blend_mode }),
        RasterMutationDsl::MoveLayer { layer_id, new_x, new_y } => RasterMutation::MoveLayer(move_layer::mutation::MoveLayer { layer_id, new_x, new_y }),
        RasterMutationDsl::ResizeLayer { layer_id, new_width, new_height } => RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id, new_width, new_height }),
        RasterMutationDsl::ChangeLayerAdjustmentKind { layer_id, new_adjustment_kind } => RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id, new_adjustment_kind }),
        RasterMutationDsl::AddLayerAsset { asset_id, asset } => RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id, asset: asset.into() }),
        RasterMutationDsl::ChangeLayerAdjustmentParameter {layer_id,parameter,expected,value} => RasterMutation::ChangeLayerAdjustmentParameter(change_layer_adjustment_parameter::ChangeLayerAdjustmentParameter {layer_id,parameter,expected,value}),
        RasterMutationDsl::ChangeLayerTransform {layer_id,expected,transform}=>RasterMutation::ChangeLayerTransform(change_layer_transform::ChangeLayerTransform {layer_id,expected,transform}),
        RasterMutationDsl::ChangeLayerMask { layer_id, expected, mask } => RasterMutation::ChangeLayerMask(change_layer_mask::ChangeLayerMask { layer_id, expected, mask }),
        RasterMutationDsl::ChangeLayerPixels { layer_id, expected_image_key, content, transform } => RasterMutation::ChangeLayerPixels(change_layer_pixels::ChangeLayerPixels { layer_id, expected_image_key, content, transform }),
        RasterMutationDsl::RemoveLayerAsset { asset_id } => RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id }),
        RasterMutationDsl::PaintStroke { layer_id, target, tool, size, hardness, opacity, color, xs, ys, selection } => RasterMutation::PaintStroke(paint_stroke::PaintStroke {
            layer_id,
            target,
            tool,
            brush: paint_stroke::RasterBrush { size, hardness, opacity, color },
            points: xs.into_iter().zip(ys).map(|(x, y)| paint_stroke::RasterStrokePoint { x, y }).collect(),
            selection: selection.and_then(|spans| semio_framework_pack_json::from_json_str(&spans, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()),
        }),
        RasterMutationDsl::FillRegion { layer_id, target, x, y, tolerance, color, selection } => RasterMutation::FillRegion(fill_region::FillRegion {
            layer_id,
            target,
            seed: fill_region::RasterSeed { x, y },
            tolerance,
            color,
            selection: selection.and_then(|spans| semio_framework_pack_json::from_json_str(&spans, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()),
        }),
        RasterMutationDsl::ApplyFilter { layer_id, filter, amount, selection } => RasterMutation::ApplyFilter(apply_filter::ApplyFilter {
            layer_id,
            filter,
            amount,
            selection: selection.and_then(|spans| semio_framework_pack_json::from_json_str(&spans, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()),
        }),
        RasterMutationDsl::TransformImage { layer_id, operation, x, y, width, height, bilinear } => RasterMutation::TransformImage(transform_image::TransformImage { layer_id, operation, x, y, width, height, bilinear }),
        RasterMutationDsl::FillSelection { layer_id, target, color, selection } => RasterMutation::FillSelection(fill_selection::FillSelection {
            layer_id,
            target,
            color,
            selection: selection.and_then(|spans| semio_framework_pack_json::from_json_str(&spans, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()),
        }),
    }
}

impl OpText for RasterMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(raster_mutation_from_dsl(<RasterMutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <RasterMutationDsl as OpText>::print_op(&raster_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `RasterMutationDsl` already derives `OpBinary` via
/// `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.

//#endregion 🔖️OpText

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::RasterDiff;
use crate::RasterSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::add_layer_asset;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_adjustment_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_blend_mode;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_opacity;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_visible;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_locked;
use crate::standards::v1::subsets::any::schema::mutations::create_layer;
use crate::standards::v1::subsets::any::schema::mutations::delete_layer;
use crate::standards::v1::subsets::any::schema::mutations::move_layer;
use crate::standards::v1::subsets::any::schema::mutations::remove_layer_asset;
use crate::standards::v1::subsets::any::schema::mutations::rename_layer;
use crate::standards::v1::subsets::any::schema::mutations::reorder_layers;
use crate::standards::v1::subsets::any::schema::mutations::resize_layer;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_pixels;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_mask;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_transform;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_adjustment_parameter;
use crate::standards::v1::subsets::any::schema::mutations::paint_stroke;
use crate::standards::v1::subsets::any::schema::mutations::fill_region;
use crate::standards::v1::subsets::any::schema::mutations::apply_filter;
use crate::standards::v1::subsets::any::schema::mutations::transform_image;
use crate::standards::v1::subsets::any::schema::mutations::fill_selection;

/// 🧩️ Decodes one committed `📸️snapshot/⬅️before/🔣️.json` document together with the
/// `🦠️mutation/🔣️.json` payload beside it — the same bytes the leaf's own fixture test
/// reads — into real typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(RasterSnapshot, RasterMutation), String> {
    let snapshot: RasterSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("the committed raster snapshot JSON does not decode: {error}"))?;
    // 🧹️ A decoded before-document owns two fixed-capacity maps (`assets`, every adjustment's
    // `params`) whose `Drop` fails closed, so the snapshot cannot simply fall off this frame when
    // the mutation beside it does not decode — every committed before-document of this artifact is
    // populated.
    match semio_framework_pack_json::from_json_str::<RasterMutation>(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(mutation) => Ok((snapshot, mutation)),
        Err(error) => {
            retire_bridge_snapshot(snapshot);
            Err(format!("the committed raster mutation JSON does not decode: {error}"))
        }
    }
}

/// 🧹️ The bridge's own retirement seam for a displaced document — the artifact's real
/// `retire_raster_snapshot`, named once here so every bridge frame retires the same way.
pub(crate) fn retire_bridge_snapshot(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod mutations_wire_codec {
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::RasterDiff;
use crate::RasterSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::add_layer_asset;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_adjustment_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_blend_mode;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_opacity;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_visible;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_locked;
use crate::standards::v1::subsets::any::schema::mutations::create_layer;
use crate::standards::v1::subsets::any::schema::mutations::delete_layer;
use crate::standards::v1::subsets::any::schema::mutations::move_layer;
use crate::standards::v1::subsets::any::schema::mutations::remove_layer_asset;
use crate::standards::v1::subsets::any::schema::mutations::rename_layer;
use crate::standards::v1::subsets::any::schema::mutations::reorder_layers;
use crate::standards::v1::subsets::any::schema::mutations::resize_layer;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_pixels;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_mask;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_transform;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_adjustment_parameter;
use crate::standards::v1::subsets::any::schema::mutations::paint_stroke;
use crate::standards::v1::subsets::any::schema::mutations::fill_region;
use crate::standards::v1::subsets::any::schema::mutations::apply_filter;
use crate::standards::v1::subsets::any::schema::mutations::transform_image;
use crate::standards::v1::subsets::any::schema::mutations::fill_selection;

/// 📤️ The bridge's answer shape: the resulting document beside the codes it raised, so a caller
/// that cannot name `protocol::MutationOutcome` can still tell an application from a refusal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn bridge_render(snapshot: &RasterSnapshot, messages: Vec<String>) -> String {
    let value = semio_framework_pack_json::object([
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(snapshot))),
        ("messages".to_string(), semio_framework_pack_json::Value::Array(messages.into_iter().map(semio_framework_pack_json::Value::from).collect())),
    ]);
    semio_framework_pack_json::to_string(&value)
}
}
pub use mutations_wire_codec::*;

pub fn apply_raster_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let stepped = bridge_step(&snapshot, &mutation);
    retire_bridge_snapshot(snapshot);
    protocol::Mutation::retire_cold(mutation);
    let (applied, messages) = stepped?;
    let rendered = bridge_render(&applied, messages);
    retire_bridge_snapshot(applied);
    Ok(rendered)
}

pub fn undo_raster_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    use protocol::Mutation;
    let (base, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let inverse = <RasterMutation as Mutation<RasterSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let stepped = bridge_step(&base, &mutation);
    Mutation::retire_cold(mutation);
    retire_bridge_snapshot(base);
    let (mut current, mut messages) = match stepped {
        Ok(pair) => pair,
        Err(error) => {
            retire_bridge_mutations(inverse);
            return Err(error);
        }
    };
    let mut pending = inverse.into_iter();
    let mut refusal = None;
    while let Some(undo) = pending.next() {
        let stepped = bridge_step(&current, &undo);
        Mutation::retire_cold(undo);
        match stepped {
            Ok((next, raised)) => {
                retire_bridge_snapshot(std::mem::replace(&mut current, next));
                messages.extend(raised);
            }
            Err(error) => {
                refusal = Some(error);
                break;
            }
        }
    }
    if let Some(error) = refusal {
        retire_bridge_mutations(pending.collect());
        retire_bridge_snapshot(current);
        return Err(error);
    }
    let rendered = bridge_render(&current, messages);
    retire_bridge_snapshot(current);
    Ok(rendered)
}

use crate::standards::v1::subsets::any::schema::mutations::{bridge_step,retire_bridge_mutations};


use crate::RasterSnapshot;
