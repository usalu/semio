//! 🧬️ Shared Puzzle 5D generator-preference schema leaf.
use ::semio_framework_schema::ArtifactSchema;
use super::Puzzle5dKindWeights;

#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle5d.config")]
pub struct Puzzle5dConfig {
    #[state(config)]
    pub fill_count: u32,
    #[state(config)]
    pub contact_tolerance: f64,
    #[state(config)]
    pub proximity_radius: f64,
    #[state(config)]
    pub chunk_size: f64,
    #[state(config)]
    pub object_kind_weights: Puzzle5dKindWeights,
    #[state(config)]
    pub vortex_kind_weights: Puzzle5dKindWeights,
}

pub fn app_schema_descriptor() -> ::semio_framework_schema_registry::AppSchemaDescriptor {
    ::semio_framework_schema_registry::AppSchemaDescriptor {
        id: "s.puzzle.puzzle5d",
        config: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        presence: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
