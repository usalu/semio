//! 🧬️ SemioSnapshot — the envelope union over all 13 domain subsets — every semio artifact round-trips through this shape.
//! W2b closer: the 13 imports below now resolve to each subset's REAL, W2a/W2b-completed
//! snapshot type (brep/mesh/model/object/cad/drawing landed in W2a; document/image/video/audio/
//! animation/presentation/workflow landed in W2b) — this file's own shape (an untagged-by-us
//! `SemioSubsetSnapshot` enum + the thin `SemioSnapshot{schema, subset}` wrapper) needed no
//! structural change from the W1b scaffold to pick that up, since only the referenced types'
//! internals grew, not their names/paths.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::SemioWorkflowSnapshot;

/// 🌐️ The envelope union of all 13 semio subset snapshot types (master plan: "SemioSnapshot =
/// tagged union of the 13"). Wrapped by `SemioSnapshot` below (a struct, not the enum directly —
/// keeps `#[derive(ArtifactSchema)]` on a proven struct shape; see the W1b manifest for why).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "subset", rename_all = "camelCase")]
pub enum SemioSubsetSnapshot {
    Brep(SemioBrepSnapshot),
    Mesh(SemioMeshSnapshot),
    Model(SemioModelSnapshot),
    Object(SemioObjectSnapshot),
    Document(SemioDocumentSnapshot),
    Cad(SemioCadSnapshot),
    Drawing(SemioDrawingSnapshot),
    Image(SemioImageSnapshot),
    Video(SemioVideoSnapshot),
    Audio(SemioAudioSnapshot),
    Animation(SemioAnimationSnapshot),
    Presentation(SemioPresentationSnapshot),
    Workflow(SemioWorkflowSnapshot),
}

impl Default for SemioSubsetSnapshot {
    fn default() -> Self { SemioSubsetSnapshot::Brep(SemioBrepSnapshot::default()) }
}

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIO_DOCUMENT_SCHEMA: &str = "stdio.semio";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio")]
pub struct SemioSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub subset: SemioSubsetSnapshot,
}

impl Default for SemioSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
            subset: Default::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🚧 scaffolded by W1b: JSON-pack round trip (honest, genuinely working — not a per-format
/// binary codec, since this subset's snapshot is a NEUTRAL semio type, not an on-disk file
/// format). Wrapped in the same `store::semio_format` envelope every stdio artifact uses.
impl store::ArtifactDsl for SemioSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIO_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_pack_round_trips() {
        let snap = SemioSnapshot::default();
        let bytes = <SemioSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = SemioSnapshot::default();
        let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }
}
//#endregion 🔖️Tests
