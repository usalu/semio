//! 📎️ Drawing application schema descriptor; app config is empty and Canvas windows own view state.

/// 📎️ Publishes the empty app-config facet and the non-empty shared-live presence facet.
pub fn app_schema_descriptor() -> framework_schema::AppSchemaDescriptor {
    framework_schema::AppSchemaDescriptor {
        id: "s.draw.drawing",
        config: framework_schema::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" },
        presence: framework_schema::FacetLeaves {
            rust: include_str!("../👥️presence/🧬️schema/🦀️.rs"),
            typescript: include_str!("../👥️presence/🧬️schema/🟦️.ts"),
            graphql: include_str!("../👥️presence/🧬️schema/🔗️.graphql"),
            json_schema: include_str!("../👥️presence/🧬️schema/🔣️.json"),
            proto: include_str!("../👥️presence/🧬️schema/🛰️.proto"),
        },
    }
}
