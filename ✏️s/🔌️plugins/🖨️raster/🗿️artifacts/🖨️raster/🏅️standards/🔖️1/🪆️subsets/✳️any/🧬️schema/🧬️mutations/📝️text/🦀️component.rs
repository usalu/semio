//! 🔧️ Raster artifact — OpText/OpBinary codecs for `RasterMutation`. Mutation apply/inverse live in
//! `🧬️mutations`; this facet only handcrafts the op wire forms. The old hand-written `RasterMutation`
//! derived `dsl::DslEnum` directly; now that the dispatch enum derives `dsl::Mutations` (one unnamed
//! field per variant), that path is gone, so this leaf mirrors `din16798`'s bridge pattern: a private
//! `RasterMutationDsl` enum flattens every real variant into its own keyworded record, converted at
//! the `OpText`/`OpBinary` boundary only — `RasterMutation` itself is untouched.

use crate::artifacts::raster::mutations::{
    add_layer_asset, change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, create_layer, delete_layer, move_layer, remove_layer_asset, rename_layer, reorder_layers, resize_layer,
};
pub use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterEnvelope, RasterMutation, RasterStore};
use crate::artifacts::raster::{RasterImageAsset, RasterLayerNode};
use protocol::OpText;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `RasterMutation` — every real variant flattened into its own
/// keyworded record, converted at the `store::OpText` boundary only.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
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
    RemoveLayerAsset {
        #[dsl(key = "id")]
        asset_id: String,
    },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for RasterMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for RasterMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn raster_mutation_to_dsl(mutation: &RasterMutation) -> RasterMutationDsl {
    match mutation {
        RasterMutation::CreateLayer(payload) => RasterMutationDsl::CreateLayer { parent_id: payload.parent_id.clone(), index: payload.index, layer: payload.layer.clone() },
        RasterMutation::DeleteLayer(payload) => RasterMutationDsl::DeleteLayer { layer_id: payload.layer_id.clone() },
        RasterMutation::ReorderLayers(payload) => RasterMutationDsl::ReorderLayers { layer_id: payload.layer_id.clone(), parent_id: payload.parent_id.clone(), index: payload.index },
        RasterMutation::RenameLayer(payload) => RasterMutationDsl::RenameLayer { layer_id: payload.layer_id.clone(), new_name: payload.new_name.clone() },
        RasterMutation::ChangeLayerVisible(payload) => RasterMutationDsl::ChangeLayerVisible { layer_id: payload.layer_id.clone(), new_visible: payload.new_visible },
        RasterMutation::ChangeLayerOpacity(payload) => RasterMutationDsl::ChangeLayerOpacity { layer_id: payload.layer_id.clone(), new_opacity: payload.new_opacity },
        RasterMutation::ChangeLayerBlendMode(payload) => RasterMutationDsl::ChangeLayerBlendMode { layer_id: payload.layer_id.clone(), new_blend_mode: payload.new_blend_mode.clone() },
        RasterMutation::MoveLayer(payload) => RasterMutationDsl::MoveLayer { layer_id: payload.layer_id.clone(), new_x: payload.new_x, new_y: payload.new_y },
        RasterMutation::ResizeLayer(payload) => RasterMutationDsl::ResizeLayer { layer_id: payload.layer_id.clone(), new_width: payload.new_width, new_height: payload.new_height },
        RasterMutation::ChangeLayerAdjustmentKind(payload) => RasterMutationDsl::ChangeLayerAdjustmentKind { layer_id: payload.layer_id.clone(), new_adjustment_kind: payload.new_adjustment_kind.clone() },
        RasterMutation::AddLayerAsset(payload) => RasterMutationDsl::AddLayerAsset { asset_id: payload.asset_id.clone(), asset: payload.asset.clone() },
        RasterMutation::RemoveLayerAsset(payload) => RasterMutationDsl::RemoveLayerAsset { asset_id: payload.asset_id.clone() },
    }
}

fn raster_mutation_from_dsl(mutation: RasterMutationDsl) -> RasterMutation {
    match mutation {
        RasterMutationDsl::CreateLayer { parent_id, index, layer } => RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id, index, layer }),
        RasterMutationDsl::DeleteLayer { layer_id } => RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id }),
        RasterMutationDsl::ReorderLayers { layer_id, parent_id, index } => RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id, parent_id, index }),
        RasterMutationDsl::RenameLayer { layer_id, new_name } => RasterMutation::RenameLayer(rename_layer::mutation::RenameLayer { layer_id, new_name }),
        RasterMutationDsl::ChangeLayerVisible { layer_id, new_visible } => RasterMutation::ChangeLayerVisible(change_layer_visible::mutation::ChangeLayerVisible { layer_id, new_visible }),
        RasterMutationDsl::ChangeLayerOpacity { layer_id, new_opacity } => RasterMutation::ChangeLayerOpacity(change_layer_opacity::mutation::ChangeLayerOpacity { layer_id, new_opacity }),
        RasterMutationDsl::ChangeLayerBlendMode { layer_id, new_blend_mode } => RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::mutation::ChangeLayerBlendMode { layer_id, new_blend_mode }),
        RasterMutationDsl::MoveLayer { layer_id, new_x, new_y } => RasterMutation::MoveLayer(move_layer::mutation::MoveLayer { layer_id, new_x, new_y }),
        RasterMutationDsl::ResizeLayer { layer_id, new_width, new_height } => RasterMutation::ResizeLayer(resize_layer::mutation::ResizeLayer { layer_id, new_width, new_height }),
        RasterMutationDsl::ChangeLayerAdjustmentKind { layer_id, new_adjustment_kind } => RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id, new_adjustment_kind }),
        RasterMutationDsl::AddLayerAsset { asset_id, asset } => RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
        RasterMutationDsl::RemoveLayerAsset { asset_id } => RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id }),
    }
}

impl OpText for RasterMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
