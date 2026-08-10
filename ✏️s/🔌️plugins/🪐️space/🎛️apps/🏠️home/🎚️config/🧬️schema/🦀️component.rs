//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.home.config")]
pub struct HomeConfig {
    #[state(local_ui)] pub active_panel_tab: String,
    #[state(local_ui)] pub locale: String,
}

