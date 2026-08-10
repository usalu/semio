//! 🧬️ GifArtifact schema — full artifact state.

// 🔀️ S-6: `crate::artifacts::gif::schema` now shims to 89a (canonical) -- 87a's own schema uses
// its own standard-local snapshot type directly rather than the shared root re-export.
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::{GifSnapshot, RasterImage};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif")]
pub struct GifArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub image: RasterImage,
}

impl Default for GifArtifact {
    fn default() -> Self { Self::from_snapshot(GifSnapshot::default()) }
}

impl GifArtifact {
    pub fn to_snapshot(&self) -> GifSnapshot {
        GifSnapshot { schema: self.schema.clone(), image: self.image.clone() }
    }
    pub fn from_snapshot(snapshot: GifSnapshot) -> Self {
        Self { schema: snapshot.schema, image: snapshot.image }
    }
    pub fn set_snapshot(&mut self, snapshot: GifSnapshot) {
        self.schema = snapshot.schema;
        self.image = snapshot.image;
    }
}

pub fn gif_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.gif",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
