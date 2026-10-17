//! 🌉️ Raster mutation codec bridge: cold retirement of bridged operations and the kebab-case kind vocabulary.

use super::*;

/// 🧹️ Cold-retires a batch of operations nobody will apply — a `create-layer` inverse can carry a
/// whole subtree whose adjustment layers own populated `params` maps.
pub fn retire_bridge_mutations(mutations: Vec<RasterMutation>) {
    for mutation in mutations {
        protocol::Mutation::retire_cold(mutation);
    }
}

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `RasterMutation` variant, in declaration order — the vocabulary
/// the `raster-1-any` catalog (`../../🔣️oracle.json`) declares and the
/// `🖨️mutate-raster-1` exhaustive case measures itself against. 
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against the enum,
/// since the framework never parses Rust.
pub const KINDS: &[&str] =
    &["create-layer", "delete-layer", "reorder-layers", "rename-layer", "change-layer-visible", "change-layer-locked", "change-layer-opacity", "change-layer-blend-mode", "move-layer", "resize-layer", "change-layer-adjustment-kind", "add-layer-asset", "remove-layer-asset", "change-layer-pixels", "change-layer-mask", "change-layer-transform", "change-layer-adjustment-parameter", "paint-stroke", "fill-region", "apply-filter", "transform-image", "fill-selection", "write-pixel-region"];
