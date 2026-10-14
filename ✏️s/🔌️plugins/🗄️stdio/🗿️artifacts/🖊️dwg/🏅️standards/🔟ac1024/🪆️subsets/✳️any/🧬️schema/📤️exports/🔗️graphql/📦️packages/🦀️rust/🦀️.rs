//! 🔗️ `stdio.dwg` GraphQL schema exports of the artifact, snapshot, diff and mutation facets, embedded for the artifact schema descriptor.

/// 🔗️ The artifact facet's GraphQL leaf.
pub const ARTIFACT: &str = include_str!("../../../../🔗️.graphql");
/// 🔗️ The snapshot facet's GraphQL leaf.
pub const SNAPSHOT: &str = include_str!("../../../../📸️snapshot/🔗️.graphql");
/// 🔗️ The diff facet's GraphQL leaf.
pub const DIFF: &str = include_str!("../../../../🔺️diff/🔗️.graphql");
/// 🔗️ The mutations facet's GraphQL leaf.
pub const MUTATIONS: &str = include_str!("../../../../🧬️mutations/🔗️.graphql");
