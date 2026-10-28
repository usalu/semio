//! 📇️ One owned IDML schema descriptor binds all four language facets.

pub fn schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.idml",
        artifact: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../🦀️.rs"),
            typescript: include_str!("../🟦️.ts"),
            graphql: include_str!("../🔗️.graphql"),
            json_schema: include_str!("../🔣️.json"),
            proto: include_str!("../🛰️.proto"),
        },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../📸️snapshot/🦀️.rs"),
            typescript: include_str!("../📸️snapshot/🟦️.ts"),
            graphql: include_str!("../📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("../📸️snapshot/🔣️.json"),
            proto: include_str!("../📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../🔺️diff/🦀️.rs"),
            typescript: include_str!("../🔺️diff/🟦️.ts"),
            graphql: include_str!("../🔺️diff/🔗️.graphql"),
            json_schema: include_str!("../🔺️diff/🔣️.json"),
            proto: include_str!("../🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("../🧬️mutations/🦀️.rs"),
            typescript: include_str!("../🧬️mutations/🟦️.ts"),
            graphql: include_str!("../🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("../🧬️mutations/🔣️.json"),
            proto: include_str!("../🧬️mutations/🛰️.proto"),
        },
    }
}
