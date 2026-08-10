//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.block.2d.config")]
pub struct Block2dConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub locale: String,
}

