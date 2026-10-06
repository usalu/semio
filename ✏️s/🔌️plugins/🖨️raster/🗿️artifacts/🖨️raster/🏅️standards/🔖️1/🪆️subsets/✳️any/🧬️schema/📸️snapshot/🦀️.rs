//! 🧬️ Raster snapshot schema — artifact-lane fields only.
//!
//! `ArtifactDsl` and `ArtifactPack` carry the literal typed Raster layer and intrinsic graph. `RasterSnapshot.assets` carries `store::ArtifactChild<SemioImageSnapshot>`
//! handles (composed `s.stdio.semio.image` children, one per asset id — see `🗿️artifacts/🖨️raster/🦀️.rs`'s
//! `🧩️Composition` region) in a `RasterOwnedMap`, keeping the id-keyed addressing every
//! `add-layer-asset`/`remove-layer-asset` mutation assumes. `child_slots()` is EMPTY for `assets`: the
//! `#[child(kind=...)]` derive only recognizes a bare `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field.

use crate::{RasterAssetChild, RasterLayerMask, RasterLayerNode, RasterOwnedMap, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted raster document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.raster.raster")]
pub struct RasterSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    pub layers: Vec<RasterLayerNode>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "RasterOwnedMap::is_empty")]
    pub assets: RasterOwnedMap<RasterAssetChild>,
}

/// 🧹️ Closes a whole snapshot's owned roots — the `assets` map and every adjustment-parameter map
/// hidden in the layer forest. A `RasterSnapshot` is the projection type the framework's replay
/// arithmetic builds and displaces (`MutationDiff::retire_projection`), and a bare drop of a
/// POPULATED one aborts the guest on `RasterOwnedMap`'s fail-closed destructor, which refuses every
/// later dispatch in the whole shell.
pub fn retire_raster_snapshot(snapshot: RasterSnapshot) {
    let RasterSnapshot { schema: _, id: _, title: _, layers, mut assets } = snapshot;
    crate::retire_raster_layers(layers);
    assets.retire();
}

/// 🧹️ [`retire_raster_snapshot`] for the artifact-shaped twin the diff vocabulary carries — the same
/// two owned roots, reached by `RasterDiff::retire_cold` and by every half-built candidate an
/// `apply` abandons on a rejection.
pub fn retire_raster_artifact(artifact: crate::standards::v1::subsets::any::schema::RasterArtifact) {
    let crate::standards::v1::subsets::any::schema::RasterArtifact { schema: _, id: _, title: _, layers, mut assets } = artifact;
    crate::retire_raster_layers(layers);
    assets.retire();
}
//#endregion 🔖️Snapshot

#[path="📦️record/🦀️.rs"]
mod record;
use record::RasterNativeDocument;




//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Defaults
impl Default for RasterSnapshot {
    fn default() -> Self {
        Self { schema: RASTER_DOCUMENT_SCHEMA.into(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() }
    }
}
//#endregion 🔖️Defaults


