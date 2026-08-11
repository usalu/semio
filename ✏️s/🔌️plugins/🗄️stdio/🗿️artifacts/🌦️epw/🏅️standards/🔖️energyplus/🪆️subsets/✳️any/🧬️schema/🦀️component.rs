//! 🧬️ EpwArtifact schema — full artifact state, mirrors `EpwSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{EpwSnapshot, EpwLocation};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.epw")]
pub struct EpwArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub location: EpwLocation,
    #[state(persistent)]
    #[serde(default)]
    pub raw_lines: Vec<String>,
}

impl Default for EpwArtifact {
    fn default() -> Self { Self::from_snapshot(EpwSnapshot::default()) }
}

impl EpwArtifact {
    pub fn to_snapshot(&self) -> EpwSnapshot {
        EpwSnapshot {
            schema: self.schema.clone(),
            location: self.location.clone(),
            raw_lines: self.raw_lines.clone(),
        }
    }
    pub fn from_snapshot(snapshot: EpwSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            location: snapshot.location,
            raw_lines: snapshot.raw_lines,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: EpwSnapshot) {
        self.schema = snapshot.schema;
        self.location = snapshot.location;
        self.raw_lines = snapshot.raw_lines;
    }
}

pub fn epw_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.epw",
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
