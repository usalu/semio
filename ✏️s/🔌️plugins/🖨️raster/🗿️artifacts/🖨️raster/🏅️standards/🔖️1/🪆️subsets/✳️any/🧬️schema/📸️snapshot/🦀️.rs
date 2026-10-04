//! 🧬️ Raster snapshot schema — artifact-lane fields only.
//!
//! `ArtifactDsl` and `ArtifactPack` carry the literal typed Raster layer and intrinsic graph. `RasterSnapshot.assets` carries `store::ArtifactChild<SemioImageSnapshot>`
//! handles (composed `s.stdio.semio.image` children, one per asset id — see `🗿️artifacts/🖨️raster/🦀️.rs`'s
//! `🧩️Composition` region) in a `RasterOwnedMap`, keeping the id-keyed addressing every
//! `add-layer-asset`/`remove-layer-asset` mutation assumes. `child_slots()` is EMPTY for `assets`: the
//! `#[child(kind=...)]` derive only recognizes a bare `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field.

use crate::{RasterAssetChild, RasterLayerMask, RasterLayerNode, RasterOwnedMap, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;

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

/// 🖨️ Prints the owner's literal typed forest and intrinsic records.
pub(crate) fn print_pack_record_text(snapshot:&RasterSnapshot)->String{
 semio_framework_dsl_record::print(&RasterNativeDocument::ordinary(snapshot).__dsl_to_record(),&RasterNativeDocument::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document)
}
/// 📖️ Reconstructs the literal native forest without an embedded layer or child container.
pub(crate) fn parse_pack_record_text(body:&str)->Result<RasterSnapshot,semio_framework_diagnostic::TextError>{
 let record=semio_framework_dsl_record::parse(body,&RasterNativeDocument::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;
 RasterNativeDocument::__dsl_from_record(&record)?.ordinary_snapshot().map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))
}

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Native text and pack encode the same literal typed Raster graph.
impl store::ArtifactDsl for RasterSnapshot {
    const EXTENSION: &'static str = "raster";
    fn envelope_id() -> &'static str {
        "raster.raster"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for RasterSnapshot {
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&RasterNativeDocument::__dsl_spec(), &RasterNativeDocument::ordinary(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &RasterNativeDocument::__dsl_spec(), options)?;
        RasterNativeDocument::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.ordinary_snapshot().map_err(store::PackError::from)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(RasterNativeDocument::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Defaults
impl Default for RasterSnapshot {
    fn default() -> Self {
        Self { schema: RASTER_DOCUMENT_SCHEMA.into(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() }
    }
}
//#endregion 🔖️Defaults

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;
