//! 🧬️ SemioImageArtifact schema — full artifact state, mirrors `SemioImageSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image")]
pub struct SemioImageArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[value(default)]
    pub colorspace: SemioColorspace,
    #[state(artifact)]
    #[value(default)]
    pub bit_depth: u8,
    #[state(artifact)]
    #[value(default)]
    pub frames: Vec<SemioImageFrame>,
    #[state(artifact)]
    #[value(default)]
    pub icc: Option<Vec<u8>>,
    #[state(artifact)]
    #[value(default)]
    pub metadata: Vec<SemioImageMetadataEntry>,
}

impl Default for SemioImageArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioImageSnapshot::default())
    }
}

impl SemioImageArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioImageSnapshot {
        SemioImageSnapshot { schema: self.schema.clone(), width: self.width, height: self.height, colorspace: self.colorspace, bit_depth: self.bit_depth, frames: self.frames.clone(), icc: self.icc.clone(), metadata: self.metadata.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioImageSnapshot) -> Self {
        Self { schema: snapshot.schema, width: snapshot.width, height: snapshot.height, colorspace: snapshot.colorspace, bit_depth: snapshot.bit_depth, frames: snapshot.frames, icc: snapshot.icc, metadata: snapshot.metadata }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioImageSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.colorspace = snapshot.colorspace;
        self.bit_depth = snapshot.bit_depth;
        self.frames = snapshot.frames;
        self.icc = snapshot.icc;
        self.metadata = snapshot.metadata;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_image_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.image",
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
