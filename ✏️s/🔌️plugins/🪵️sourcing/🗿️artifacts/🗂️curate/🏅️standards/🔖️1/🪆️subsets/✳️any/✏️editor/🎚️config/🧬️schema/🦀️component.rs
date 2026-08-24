//! 🧬️ schema leaf
use crate::artifacts::curate::Filters;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curate.config")]
pub struct SourcingCurateConfig {
    #[state(config)]
    pub filters: Filters,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub contributions_json: String,
}

//#region 🔖️AppSchemaDescriptor
/// 📎 The curate app's config + presence schema facets — the open replacement for this app's entry
/// in framework schema's closed `register_all_app_schema_descriptors()` catalog (see
/// `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`, `s.sourcing.curate`). Returned, not
/// self-registered; `SourcingCurateApp::app_schema` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) hands it to `register_document_app` — app-scope
/// config/presence schema is the one registration `ArtifactDeclaration` deliberately has no field
/// for (see that struct's own doc). `🪵️sourcing/🦀️component.rs` no longer needs `.setup()` for this.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.sourcing.curate",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️AppSchemaDescriptor
