//! 🧬️ TiffArtifact schema — full artifact state (mirrors `TiffSnapshot` field-for-field; see
//! `png_artifact_schema_descriptor`/`PngArtifact` for the established repo pattern this follows).

use crate::schema::snapshot::{TiffIfd};
use crate::TiffSnapshot;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tiff")]
pub struct TiffArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub ifds: Vec<TiffIfd>,
}

impl Default for TiffArtifact {
    fn default() -> Self {
        Self::from_snapshot(TiffSnapshot::default())
    }
}

impl TiffArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> TiffSnapshot {
        TiffSnapshot { schema: self.schema.clone(), ifds: self.ifds.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: TiffSnapshot) -> Self {
        Self { schema: snapshot.schema, ifds: snapshot.ifds }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: TiffSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn tiff_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.tiff",
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
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `blank_tiff_snapshot`/`demo_tiff_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `TiffEngine` (zero construction sites) and the dead
// `register`/`register_pilot_languages`/`register_artifact_inferences` cluster (superseded by
// `declaration()` in the artifact root, zero real callers) deleted outright; the real codec
// (`encode_tiff`/`encode_tiff_packbits`/`decode_tiff` + every pure format algorithm) and
// `io_registry` moved to `../🚪️io`; tests moved beside what they now test.
/// 🆕️ One owned white RGB pixel.
pub fn blank_tiff_snapshot() -> TiffSnapshot {
    use crate::schema::snapshot::*;
    TiffSnapshot { schema: crate::STDIO_TIFF_DOCUMENT_SCHEMA.into(), ifds: vec![TiffIfd {
        entries: vec![TiffTag{tag:256,values:TiffValues::Long(vec![1])},TiffTag{tag:257,values:TiffValues::Long(vec![1])},TiffTag{tag:258,values:TiffValues::Short(vec![8,8,8])},TiffTag{tag:262,values:TiffValues::Short(vec![2])},TiffTag{tag:277,values:TiffValues::Short(vec![3])}],
        blocks: vec![TiffSampleBlock{x:0,y:0,width:1,height:1,channels:3,samples:vec![TiffWord64{lo:255,hi:0};3]}]
    }] }
}
/// 📄️ Literal logical checkerboard and owned artist text.
pub fn demo_tiff_snapshot() -> TiffSnapshot {
    use crate::schema::snapshot::*;
    let mut snapshot=blank_tiff_snapshot();let page=&mut snapshot.ifds[0];
    page.entries[0].values=TiffValues::Long(vec![3]);page.entries[1].values=TiffValues::Long(vec![2]);
    page.entries.push(TiffTag{tag:315,values:TiffValues::Ascii(vec!["stdio.tiff demo".into()])});
    page.blocks[0]=TiffSampleBlock{x:0,y:0,width:3,height:2,channels:3,samples:vec![255,0,0,0,37,0,255,74,0,0,0,53,255,37,53,0,74,53].into_iter().map(TiffWord64::from_word).collect()};
    snapshot
}
