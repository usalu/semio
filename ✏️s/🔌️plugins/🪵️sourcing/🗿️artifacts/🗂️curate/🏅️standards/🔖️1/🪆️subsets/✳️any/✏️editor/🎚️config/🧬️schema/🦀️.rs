//! 🧬️ schema leaf
use crate::artifacts::curate::Filters;
use schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
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
/// for (see that struct's own doc). `🪵️sourcing/🦀️.rs` no longer needs `.setup()` for this.
pub fn app_schema_descriptor() -> ::schema::AppSchemaDescriptor {
    ::schema::AppSchemaDescriptor {
        id: "s.sourcing.curate",
        config: ::schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        presence: ::schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
//#endregion 🔖️AppSchemaDescriptor
