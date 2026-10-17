//! 📇️ Raster diff schema descriptor: the handcrafted facet leaves (Rust, TypeScript, GraphQL, JSON Schema, Protobuf) of the diff schema, embedded once.

/// 🔺️ The diff facet leaves of `s.raster.raster`.
pub const DIFF_FACET: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {
    rust: concat!(include_str!("../🦀️.rs"), include_str!("../🧱️delta-model/🦀️.rs"), include_str!("../🌲️layer-tree-apply/🦀️.rs"), include_str!("../🧮️inverse-and-absorb/🦀️.rs"), include_str!("../🖌️repaint-canvas/🦀️.rs")),
    typescript: include_str!("../🟦️.ts"),
    graphql: include_str!("../🔗️.graphql"),
    json_schema: include_str!("../🔣️.json"),
    proto: include_str!("../🛰️.proto"),
};
