//! 🧬️ Raster snapshot schema — artifact-lane fields only.
//!
//! `ArtifactDsl` and `ArtifactPack` are the derived text and pack of `RasterPackRecord`. `RasterSnapshot.assets` carries `store::ArtifactChild<SemioImageSnapshot>`
//! handles (composed `s.stdio.semio.image` children, one per asset id — see `🗿️artifacts/🖨️raster/🦀️.rs`'s
//! `🧩️Composition` region) in a `RasterOwnedMap`, keeping the id-keyed addressing every
//! `add-layer-asset`/`remove-layer-asset` mutation assumes. `child_slots()` is EMPTY for `assets`: the
//! `#[child(kind=...)]` derive only recognizes a bare `ArtifactChild<T>`/`Vec<ArtifactChild<T>>` field.

use crate::{RasterAssetChild, RasterLayerMask, RasterLayerNode, RasterOwnedMap, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted raster document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
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

//#region 🔖️CodecPrimitives

use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;

//#endregion 🔖️CodecPrimitives

//#region 🔖️PackRecord
/// 🔁️ One asset entry of the pack: its map key, the composed child handle, and the child's own
/// canonical `ArtifactPack` bytes — `store::ArtifactChild`'s `local_owner` is serialization-skipped by
/// design, and a raster document packed and reopened without them came back with pixel-less handles
/// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). Text and pack both encode this record, so
/// both carry the child; `content` is empty when the handle carries no materialization (a
/// wire-decoded child the host has not resolved, or a committed example whose media is planted by
/// `🎮️commands/🎬️set-active-example`).
#[derive(dsl::DslRecord)]
struct RasterAssetPackRow {
    key: String,
    child: RasterAssetChild,
    #[dsl(base64)]
    content: Vec<u8>,
}

/// 📦️ Derived pack record of a `RasterSnapshot`. Each layer travels as its first-party JSON text:
/// adjustment parameters are free-form `DslValue`s whose key order is significant, while pack
/// canonicalises map keys into sorted order.
#[derive(dsl::DslRecord)]
#[dsl(extension = "raster")]
struct RasterPackRecord {
    schema: String,
    id: String,
    title: Option<String>,
    layers: Vec<String>,
    assets: Vec<RasterAssetPackRow>,
}

impl RasterPackRecord {
    fn from_snapshot(snapshot: &RasterSnapshot) -> Self {
        let assets = snapshot
            .assets
            .iter()
            .map(|(key, child)| RasterAssetPackRow {
                key: key.clone(),
                child: store::ArtifactChild::new(child.child_id.clone(), child.target.clone()),
                content: child.local_owner::<SemioImageSnapshot>().map(|image| <SemioImageSnapshot as store::ArtifactPack>::encode_pack(image.as_ref())).unwrap_or_default(),
            })
            .collect();
        Self { schema: snapshot.schema.clone(), id: snapshot.id.clone(), title: snapshot.title.clone(), layers: snapshot.layers.iter().map(dsl::os_pack::json::to_json_string).collect(), assets }
    }

    fn into_snapshot(self) -> Result<RasterSnapshot, String> {
        let mut children = Vec::with_capacity(self.assets.len());
        for row in self.assets {
            let child = if row.content.is_empty() { row.child } else { row.child.with_local_owner(std::sync::Arc::new(<SemioImageSnapshot as store::ArtifactPack>::decode_pack(&row.content).map_err(|error| error.to_string())?)) };
            children.push((row.key, child));
        }
        let mut layers = Vec::with_capacity(self.layers.len());
        for text in &self.layers {
            match dsl::os_pack::json::from_json_str::<RasterLayerNode>(text) {
                Ok(layer) => layers.push(layer),
                Err(error) => {
                    crate::retire_raster_layers(layers);
                    return Err(error.to_string());
                }
            }
        }
        let mut assets = RasterOwnedMap::new();
        for (key, child) in children {
            if let Err(rejected) = assets.insert(key, child) {
                assets.retire();
                crate::retire_raster_layers(layers);
                return Err(rejected.reason.to_string());
            }
        }
        Ok(RasterSnapshot { schema: self.schema, id: self.id, title: self.title, layers, assets })
    }
}

/// 🖨️ The derived text body: the same `RasterPackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &RasterSnapshot) -> String {
    dsl::print(&RasterPackRecord::from_snapshot(snapshot).__dsl_to_record(), &RasterPackRecord::__dsl_spec(), dsl::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `RasterPackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<RasterSnapshot, store::TextError> {
    let record = dsl::parse(body, &RasterPackRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
    RasterPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ `ArtifactDsl` and `ArtifactPack` are the derived text and pack of `RasterPackRecord`.
impl store::ArtifactDsl for RasterSnapshot {
    const EXTENSION: &'static str = "raster";
    fn envelope_id() -> &'static str {
        "raster.raster"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
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
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&RasterPackRecord::__dsl_spec(), &RasterPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &RasterPackRecord::__dsl_spec(), options)?;
        RasterPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(store::PackError::Schema)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(RasterPackRecord::__dsl_spec())
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

