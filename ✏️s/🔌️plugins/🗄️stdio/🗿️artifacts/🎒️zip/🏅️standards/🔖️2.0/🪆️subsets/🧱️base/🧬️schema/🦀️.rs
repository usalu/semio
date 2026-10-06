//! 🧬️ ZipArtifact schema — full artifact state.

use crate::schema::snapshot::ZipEntry;
use crate::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region Artifact
/// 🧬️ Full `stdio.zip` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip")]
pub struct ZipArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub entries: Vec<ZipEntry>,
    #[state(artifact)]
    #[value(default)]
    pub comment: String,
    #[state(artifact)]
    #[value(default = "default_true")]
    pub comment_utf8: bool,
}

fn default_true() -> bool {
    true
}
//#endregion Artifact

//#region Conversions
impl Default for ZipArtifact {
    fn default() -> Self {
        Self::from_snapshot(ZipSnapshot::default())
    }
}

impl ZipArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> ZipSnapshot {
        ZipSnapshot { schema: self.schema.clone(), entries: self.entries.clone(), comment: self.comment.clone(), comment_utf8: self.comment_utf8 }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: ZipSnapshot) -> Self {
        Self { schema: snapshot.schema, entries: snapshot.entries, comment: snapshot.comment, comment_utf8: snapshot.comment_utf8 }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: ZipSnapshot) {
        self.schema = snapshot.schema;
        self.entries = snapshot.entries;
        self.comment = snapshot.comment;
        self.comment_utf8 = snapshot.comment_utf8;
    }
}
//#endregion Conversions

//#region 🔖️DocumentHelpers
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — mirrors `png`'s own `blank_png_snapshot`/`demo_png_snapshot` placement beside the
/// artifact struct.
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_zip_snapshot() -> ZipSnapshot {
    ZipSnapshot::default()
}

/// 📦️ Demo logical ZIP document with two decompressed semantic members and an archive comment.
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `💡️inferences/🦀️.rs`) and for this artifact's own `protocol_walk_law` (walked against
/// authored `📸️snapshot/💾️binary/📡️.protocol.semio` SPK frame independently of native ZIP directory layout).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_zip_snapshot() -> ZipSnapshot {
    ZipSnapshot {
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        // 🔤️ Canonical (`name`-ascending) member order — the order this artifact's own writer emits
        // and its reader hands back (`🚪️io`'s `encode_zip`/`decode_zip`), so the demo is a fixpoint
        // of its own codec rather than a snapshot no round trip could reproduce.
        entries: vec![
            ZipEntry { name: "data/poem.txt".into(), data: b"deflate this small poem, it should compress reasonably well well well".to_vec(), ..Default::default() },
            ZipEntry { name: "readme.txt".into(), data: b"hello from stdio.zip".to_vec(), ..Default::default() },
        ],
        comment: "demo archive comment".into(),
        comment_utf8: true,
    }
}
//#endregion 🔖️DocumentHelpers

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.zip`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn zip_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.zip",
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
//#endregion Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
