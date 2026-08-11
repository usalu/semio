//! 🧬️ TsvArtifact schema — full artifact state, mirrors `TsvSnapshot` field for field.

use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tsv")]
pub struct TsvArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub records: Vec<Vec<String>>,
    #[state(persistent)]
    #[serde(default)]
    pub trailing_newline: bool,
    #[state(persistent)]
    #[serde(default)]
    pub line_ending: LineEnding,
}

impl Default for TsvArtifact {
    fn default() -> Self { Self::from_snapshot(TsvSnapshot::default()) }
}

impl TsvArtifact {
    pub fn to_snapshot(&self) -> TsvSnapshot {
        TsvSnapshot {
            schema: self.schema.clone(),
            records: self.records.clone(),
            trailing_newline: self.trailing_newline,
            line_ending: self.line_ending,
        }
    }
    pub fn from_snapshot(snapshot: TsvSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            records: snapshot.records,
            trailing_newline: snapshot.trailing_newline,
            line_ending: snapshot.line_ending,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: TsvSnapshot) {
        self.schema = snapshot.schema;
        self.records = snapshot.records;
        self.trailing_newline = snapshot.trailing_newline;
        self.line_ending = snapshot.line_ending;
    }
}

pub fn tsv_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.tsv",
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
