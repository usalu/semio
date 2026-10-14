//! 🔣️ `stdio.dwg` JSON Schema schema exports of the artifact, snapshot, diff and mutation facets, embedded for the artifact schema descriptor.

/// 🔣️ The artifact facet's JSON Schema leaf.
pub const ARTIFACT: &str = include_str!("../../../../🔣️.json");
/// 🔣️ The snapshot facet's JSON Schema leaf.
pub const SNAPSHOT: &str = include_str!("../../../../📸️snapshot/🔣️.json");
/// 🔣️ The diff facet's JSON Schema leaf.
pub const DIFF: &str = include_str!("../../../../🔺️diff/🔣️.json");
/// 🔣️ The mutations facet's JSON Schema leaf.
pub const MUTATIONS: &str = include_str!("../../../../🧬️mutations/🔣️.json");
