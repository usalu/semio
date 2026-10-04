//! 🔧️ Raster artifact — OpText/OpBinary codecs for `RasterMutation`. Mutation apply/inverse live in
//! `🧬️mutations`; this facet only handcrafts the op wire forms. The old hand-written `RasterMutation`
//! derived `dsl::DslEnum` directly; now that the dispatch enum derives `dsl::Mutations` (one unnamed
//! field per variant), that path is gone, so this leaf mirrors `din16798`'s bridge pattern: a private
//! `RasterMutationDsl` enum flattens every real variant into its own keyworded record, converted at
//! the `OpText`/`OpBinary` boundary only — `RasterMutation` itself is untouched.

use crate::mutations::{apply_filter,transform_image,fill_selection,fill_region,paint_stroke,change_layer_transform,change_layer_locked,change_layer_adjustment_parameter, change_layer_mask, change_layer_pixels, add_layer_asset, change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, create_layer, delete_layer, move_layer, remove_layer_asset, rename_layer, reorder_layers, resize_layer};
pub use crate::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterEnvelope, RasterMutation, RasterStore};
use crate::{RasterImageAsset, RasterLayerNode};
use protocol::OpText;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `RasterMutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
enum RasterMutationDsl {
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
        asset: RasterImageAsset,
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

impl protocol::OpBinary for RasterMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn raster_mutation_to_dsl(mutation: &RasterMutation) -> RasterMutationDsl {
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
        RasterMutation::AddLayerAsset(payload) => RasterMutationDsl::AddLayerAsset { asset_id: payload.asset_id.clone(), asset: payload.asset.clone() },
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

fn raster_mutation_from_dsl(mutation: RasterMutationDsl) -> RasterMutation {
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
        RasterMutationDsl::AddLayerAsset { asset_id, asset } => RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
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
impl protocol::OpBinary for RasterMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        raster_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(raster_mutation_from_dsl(RasterMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText
