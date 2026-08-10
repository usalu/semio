//! 🧬️ schema leaf
use crate::artifacts::curate::Filters;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curate.config")]
pub struct SourcingCurateConfig {
    #[state(local_ui)] pub filters: Filters,
    #[state(local_ui)] pub selected_object_id: Option<String>,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String,
}

