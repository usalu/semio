//! 🧬️ CsvArtifact schema — full artifact state.

use crate::artifacts::csv::CsvSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.csv` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv")]
pub struct CsvArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub has_header: bool,
    #[state(persistent)]
    #[serde(default)]
    pub headers: Vec<String>,
    #[state(persistent)]
    #[serde(default)]
    pub rows: Vec<Vec<String>>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for CsvArtifact {
    fn default() -> Self {
        Self::from_snapshot(CsvSnapshot::default())
    }
}

impl CsvArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> CsvSnapshot {
        CsvSnapshot {
            schema: self.schema.clone(),
            has_header: self.has_header,
            headers: self.headers.clone(),
            rows: self.rows.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: CsvSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            has_header: snapshot.has_header,
            headers: snapshot.headers,
            rows: snapshot.rows,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: CsvSnapshot) {
        self.schema = snapshot.schema;
        self.has_header = snapshot.has_header;
        self.headers = snapshot.headers;
        self.rows = snapshot.rows;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.csv`.
pub fn csv_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.csv",
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
//#endregion 🔖️Descriptor
