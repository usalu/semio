//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.imperative.imperative.config")]
pub struct ImperativeConfig {
    #[state(local_ui)] pub selected_step_ids: Vec<String>,
    #[state(local_ui)] pub run_output_json: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String,
}

