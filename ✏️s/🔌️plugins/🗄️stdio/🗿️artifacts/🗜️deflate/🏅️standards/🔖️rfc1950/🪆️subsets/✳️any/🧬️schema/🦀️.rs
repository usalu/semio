//! 🧬️ DeflateArtifact schema — full artifact state.

use crate::schema::snapshot::DeflateLevelHint;
use crate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.deflate` artifact state — mirrors `DeflateSnapshot`'s typed RFC1950 fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate")]
pub struct DeflateArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub compression_method: u8,
    #[state(artifact)]
    #[value(default)]
    pub window_bits: u8,
    #[state(artifact)]
    #[value(default)]
    pub compression_level_hint: DeflateLevelHint,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<u32>,
    #[state(artifact)]
    #[value(default)]
    pub payload: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DeflateArtifact {
    fn default() -> Self {
        Self::from_snapshot(DeflateSnapshot::default())
    }
}

impl DeflateArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> DeflateSnapshot {
        DeflateSnapshot { schema: self.schema.clone(), compression_method: self.compression_method, window_bits: self.window_bits, compression_level_hint: self.compression_level_hint, dict_id: self.dict_id, payload: self.payload.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: DeflateSnapshot) -> Self {
        Self { schema: snapshot.schema, compression_method: snapshot.compression_method, window_bits: snapshot.window_bits, compression_level_hint: snapshot.compression_level_hint, dict_id: snapshot.dict_id, payload: snapshot.payload }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: DeflateSnapshot) {
        self.schema = snapshot.schema;
        self.compression_method = snapshot.compression_method;
        self.window_bits = snapshot.window_bits;
        self.compression_level_hint = snapshot.compression_level_hint;
        self.dict_id = snapshot.dict_id;
        self.payload = snapshot.payload;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️DocumentHelpers
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — mirrors `png`'s own `blank_png_snapshot`/`demo_png_snapshot` placement beside the
/// artifact struct.
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_deflate_snapshot() -> DeflateSnapshot {
    DeflateSnapshot::default()
}

/// 📄️ The demo `stdio.deflate` document — a genuine, non-empty RFC1950 container: a real
/// preset-dictionary id (exercises the FDICT-gated `dict_id` field) plus repetitive text payload
/// (round-trips through this artifact's own `deflate_raw`/`inflate_raw` in `🚪️io/🦀️.rs`).
/// Single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🗜️example.zz`/
/// `🎒️.pack.semio` (all three are literally this snapshot's `print_dsl`/
/// `encode_deflate_snapshot`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `💡️inferences/🦀️.rs`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_deflate_snapshot() -> DeflateSnapshot {
    DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        compression_method: 8,
        window_bits: 7,
        compression_level_hint: DeflateLevelHint::Default,
        dict_id: Some(0x1234_5678),
        payload: b"the quick brown fox jumps over the lazy dog".to_vec(),
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.deflate`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deflate_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.deflate",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
