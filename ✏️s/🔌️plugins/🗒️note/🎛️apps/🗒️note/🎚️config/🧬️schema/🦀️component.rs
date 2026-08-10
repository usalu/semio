//! 🧬️ schema leaf
use crate::artifacts::note::NoteCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.note.note.config")]
pub struct NoteConfig {
    #[state(local_ui)] pub selected_block_ids: Vec<String>,
    #[state(local_ui)] pub hovered_block_id: Option<String>,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera: NoteCamera,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
}

