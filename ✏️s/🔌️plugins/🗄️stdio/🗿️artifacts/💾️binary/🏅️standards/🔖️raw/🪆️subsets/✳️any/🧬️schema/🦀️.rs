//! 🧬️ BinaryArtifact schema — full artifact state.

use crate::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.binary` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary")]
pub struct BinaryArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub bytes: Vec<u8>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for BinaryArtifact {
    fn default() -> Self {
        Self::from_snapshot(BinarySnapshot::default())
    }
}

impl BinaryArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> BinarySnapshot {
        BinarySnapshot { schema: self.schema.clone(), bytes: self.bytes.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: BinarySnapshot) -> Self {
        Self { schema: snapshot.schema, bytes: snapshot.bytes }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: BinarySnapshot) {
        self.schema = snapshot.schema;
        self.bytes = snapshot.bytes;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️DocumentHelpers
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — mirrors `png`'s own `blank_png_snapshot`/`demo_png_snapshot` placement beside the
/// artifact struct (binary has no format codec of its own to sit beside — the hex `ArtifactDsl`/
/// `ArtifactPack` impls already live in `📸️snapshot/🦀️.rs`, untouched by this move).
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot::default()
}

/// 📄️ The demo `stdio.binary` document -- `bytes = b"hello"`, matching the companion real-format
/// fixture asset (`📚️examples/🎬️demo/🖼️assets/💾️.bin`, which is literally the raw bytes
/// `hello`). The single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` in `💡️inferences/🦀️.rs`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: b"hello".to_vec() }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.binary`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn binary_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.binary",
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
