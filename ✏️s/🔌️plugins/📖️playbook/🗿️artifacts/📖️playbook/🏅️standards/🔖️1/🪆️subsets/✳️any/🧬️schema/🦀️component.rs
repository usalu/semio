//! 🧬️ Playbook artifact schema — every field of the artifact with its state class.

use crate::artifacts::playbook::{PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full playbook artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub id: String,
    #[state(persistent)]
    pub version: String,
    #[state(persistent)]
    pub title: Option<String>,
    #[state(persistent)]
    pub steps: Vec<PlaybookStep>,
    #[state(shared_ui)]
    pub selected_ids: Vec<String>,
    #[state(local_ui)]
    pub locale: String,
    #[state(local_ui)]
    pub contributions_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PlaybookArtifact {
    fn default() -> Self {
        Self {
            schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
            id: "playbook".into(),
            version: "1".into(),
            title: None,
            steps: vec![crate::artifacts::playbook::PlaybookStep {
                id: "s".into(),
                title: "Steps".into(),
                description: None,
                blocks: Vec::new(),
            }],
            selected_ids: Vec::new(),
            locale: "en-US".into(),
            contributions_json: "[]".into(),
        }
    }
}

impl PlaybookArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::playbook::PlaybookSnapshot {
        crate::artifacts::playbook::PlaybookSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            version: self.version.clone(),
            title: self.title.clone(),
            steps: self.steps.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::playbook::PlaybookSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            version: snapshot.version,
            title: snapshot.title,
            steps: snapshot.steps,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::playbook::PlaybookSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.version = snapshot.version;
        self.title = snapshot.title;
        self.steps = snapshot.steps;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.playbook.playbook` — fifteen handcrafted schema leaves.
pub fn playbook_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.playbook.playbook",
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
    }
}
//#endregion 🔖️Descriptor
