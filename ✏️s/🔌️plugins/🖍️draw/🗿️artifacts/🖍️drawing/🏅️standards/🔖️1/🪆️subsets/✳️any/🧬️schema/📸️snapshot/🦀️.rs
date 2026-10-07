//! 🧬️ Drawing snapshot schema — artifact-lane fields only.

use crate::{DrawingArtboard, DrawingImageAsset, DrawingLayerNode, DRAWING_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted drawing document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[value(retire_with = "crate::standards::v1::subsets::any::schema::snapshot::retire_decoded_drawing_snapshot")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "drawing.drawing", layout = "lines")]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingSnapshot {
    #[state(artifact)]
    pub schema: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[state(artifact)]
    pub id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub title: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    #[state(artifact)]
    #[dsl(statements, block)]
    pub layers: semio_framework_value::list::PagedList<DrawingLayerNode, {usize::MAX}>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "semio_framework_value::paged::PagedMap::is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "semio_framework_value::paged::PagedMap::is_empty"))]
    pub assets: semio_framework_value::paged::PagedMap<DrawingImageAsset, {usize::MAX}>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub artboard: Option<DrawingArtboard>,
}
// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` impls for `DrawingSnapshot` relocated to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}/🦀️.rs` (design.md §1 CORRECTION: the native codec
// is one bidirectional thing per type and sits unsplit under `🚪️io`; this file keeps types + pure
// transforms only, per design.md rule 3).

impl Default for DrawingSnapshot {
    fn default() -> Self {
        Self { schema: DRAWING_DOCUMENT_SCHEMA.into(), id: Default::default(), title: None, layers: semio_framework_value::list::PagedList::new(), assets: semio_framework_value::paged::PagedMap::default(), artboard: Some(DrawingArtboard { width: 1024.0, height: 1024.0 }) }
    }
}

/// ♻️ Retires every decoded domain field through its incremental owned-value cursor.
pub fn retire_decoded_drawing_snapshot(value: DrawingSnapshot) {
    let mut cursor = semio_framework_value::retirement::owned_retirement(value);
    loop {
        match cursor.close_step(256, 65536).expect("Draw owner retirement") {
            semio_framework_value::SnapshotRetirementStep::Complete => break,
            semio_framework_value::SnapshotRetirementStep::Pending { .. } => {},
            semio_framework_value::SnapshotRetirementStep::Blocked => panic!("Draw owned retirement was blocked"),
        }
    }
    assert!(cursor.terminal_is_empty());
}
//#endregion 🔖️Snapshot


