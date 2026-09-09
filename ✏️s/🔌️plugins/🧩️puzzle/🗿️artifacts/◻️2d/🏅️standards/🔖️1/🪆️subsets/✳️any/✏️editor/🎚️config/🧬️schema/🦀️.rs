//! 🧬️ Shared Puzzle 2D generator-preference schema leaf.
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle2d.config")]
pub struct Puzzle2dConfig {
    #[state(config)]
    pub node_kind_weights: BTreeMap<String, f64>,
    #[state(config)]
    pub handle_kind_weights: BTreeMap<String, f64>,
}

pub fn app_schema_descriptor() -> ::semio_framework_schema::AppSchemaDescriptor {
    ::semio_framework_schema::AppSchemaDescriptor {
        id: "s.puzzle.puzzle2d",
        config: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        presence: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
