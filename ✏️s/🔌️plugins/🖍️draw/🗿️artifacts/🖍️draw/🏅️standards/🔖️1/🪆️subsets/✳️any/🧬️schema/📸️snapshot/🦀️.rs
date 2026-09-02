//! 🧬️ Draw snapshot schema — artifact-lane fields only.

use crate::artifacts::draw::{DrawArtboard, DrawImageAsset, DrawLayerNode, DRAW_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted draw document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "draw.draw", layout = "lines")]
#[artifact_schema(id = "s.draw.draw")]
pub struct DrawSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub title: Option<String>,
    #[state(artifact)]
    #[dsl(statements, block)]
    pub layers: Vec<DrawLayerNode>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    pub assets: BTreeMap<String, DrawImageAsset>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub artboard: Option<DrawArtboard>,
}
// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` impls for `DrawSnapshot` relocated to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}/🦀️.rs` (design.md §1 CORRECTION: the native codec
// is one bidirectional thing per type and sits unsplit under `🚪️io`; this file keeps types + pure
// transforms only, per design.md rule 3).

impl Default for DrawSnapshot {
    fn default() -> Self {
        Self { schema: DRAW_DOCUMENT_SCHEMA.into(), id: String::new(), title: None, layers: Vec::new(), assets: BTreeMap::new(), artboard: Some(DrawArtboard { width: 1024.0, height: 1024.0 }) }
    }
}
//#endregion 🔖️Snapshot
