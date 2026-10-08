//! 🧬️ PNG artifact schema over precise owned samples.

use crate::PngSnapshot;
use framework_schema::ArtifactSchema;

#[path="🔏️canonical/🦀️.rs"]
mod canonical;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png")]
pub struct PngArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub image: crate::schema::snapshot::PngImage,
}

impl Default for PngArtifact {
    fn default() -> Self {
        Self::from_snapshot(PngSnapshot::default())
    }
}

impl PngArtifact {
    pub fn to_snapshot(&self) -> PngSnapshot {
        PngSnapshot { schema: self.schema.clone(), image: self.image.clone() }
    }

    pub fn from_snapshot(snapshot: PngSnapshot) -> Self {
        Self { schema: snapshot.schema, image: snapshot.image }
    }

    pub fn set_snapshot(&mut self, snapshot: PngSnapshot) {
        self.schema = snapshot.schema;
        self.image = snapshot.image;
    }
}

pub fn blank_png_snapshot() -> PngSnapshot {
    PngSnapshot::default()
}

pub fn demo_png_snapshot() -> PngSnapshot {
    PngSnapshot { schema: crate::STDIO_PNG_DOCUMENT_SCHEMA.into(), image: crate::schema::snapshot::PngImage { width:4,height:2,samples:vec![255,0,0,255,0,255,0,255,0,0,255,255,255,255,0,255,0,255,255,255,255,0,255,255,255,255,255,255,128,128,128,255],..Default::default() } }
}

pub fn png_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.png",
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
