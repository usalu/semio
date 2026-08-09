//! 🧬️ Imperative artifact schema — every field with its state class.

use crate::artifacts::imperative::Path;
use neural_engine::Value;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full imperative artifact state across persistent, shared-ui, local-ui and effect classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.imperative")]
pub struct ImperativeArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub path: Path,
    #[state(persistent)]
    #[serde(default)]
    pub seed: BTreeMap<String, Value>,
    #[state(shared_ui)]
    #[serde(default)]
    pub selected_step_ids: Vec<String>,
    #[state(local_ui)]
    pub locale: String,
    #[state(local_ui)]
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
    #[state(effect)]
    #[serde(default)]
    pub run_output_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for ImperativeArtifact {
    fn default() -> Self {
        Self {
            schema: "imperative.document".into(),
            path: Path::new(),
            seed: BTreeMap::new(),
            selected_step_ids: Vec::new(),
            locale: "en-US".into(),
            contributions_json: default_contributions_json(),
            run_output_json: String::new(),
        }
    }
}

impl ImperativeArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::imperative::ImperativeSnapshot {
        crate::artifacts::imperative::ImperativeSnapshot {
            schema: self.schema.clone(),
            path: self.path.clone(),
            seed: self.seed.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::imperative::ImperativeSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            path: snapshot.path,
            seed: snapshot.seed,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::imperative::ImperativeSnapshot) {
        self.schema = snapshot.schema;
        self.path = snapshot.path;
        self.seed = snapshot.seed;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.imperative.imperative` — fifteen handcrafted schema leaves.
pub fn imperative_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.imperative.imperative",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
