//! 🧬️ Drawing snapshot schema — artifact-lane fields only.

use crate::{DrawingArtboard, DrawingImageAsset, DrawingLayerNode, DRAWING_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

#[path = "🔎️lookup/🦀️.rs"]
pub mod lookup;

//#region 🔖️Snapshot
/// 📸️ Persisted drawing document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::NativeReceive)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[value(retire_with = "crate::snapshot::retire_decoded_drawing_snapshot")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(id = "drawing.drawing", layout = "lines")]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingSnapshot {
    #[state(artifact)]
    pub schema: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[state(artifact)]
    pub id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub title: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    #[state(artifact)]
    #[dsl(statements, block)]
    pub layers: semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "semio_framework_value::paged::PagedMap::is_empty")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "semio_framework_value::paged::PagedMap::is_empty"))]
    pub assets: semio_framework_value::paged::PagedMap<DrawingImageAsset, {usize::MAX}>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    #[dsl(block)]
    pub artboard: Option<DrawingArtboard>,
}
// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` impls for `DrawingSnapshot` relocated to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}/🦀️.rs` (design.md §1 CORRECTION: the native codec
// is one bidirectional thing per type and sits unsplit under `🚪️io`; this file keeps types + pure
// transforms only, per design.md rule 3).

impl Default for DrawingSnapshot {
    fn default() -> Self {
        Self { schema: DRAWING_DOCUMENT_SCHEMA.into(), id: Default::default(), title: None, layers: semio_framework_value_persistent::list::PagedList::new(), assets: semio_framework_value::paged::PagedMap::default(), artboard: Some(DrawingArtboard { width: 1024.0, height: 1024.0 }) }
    }
}

/// ♻️ Releases a cold decoded document without recursively dropping nested layer groups.
pub fn retire_decoded_drawing_snapshot(value: DrawingSnapshot) {
    use semio_framework_value::FromValue;
    let DrawingSnapshot { schema, id, title, layers, assets, artboard } = value;
    let mut forest = vec![layers];
    while let Some(level) = forest.last_mut() {
        match level.pop() {
            Some(DrawingLayerNode::Group(mut group)) => {
                forest.push(std::mem::take(&mut group.children));
                DrawingLayerNode::Group(group).retire_decoded();
            }
            Some(node) => node.retire_decoded(),
            None => { forest.pop(); }
        }
    }
    schema.retire_decoded();
    id.retire_decoded();
    title.retire_decoded();
    assets.retire_decoded();
    artboard.retire_decoded();
}
//#endregion 🔖️Snapshot

