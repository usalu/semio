//! 🛰️ `stdio.gltf` Protocol Buffers schema exports of the artifact, snapshot, diff and mutation facets, embedded for the artifact schema descriptor.

/// 🛰️ The artifact facet's Protocol Buffers leaf.
pub const ARTIFACT: &str = include_str!("../../../../🛰️.proto");
/// 🛰️ The snapshot facet's Protocol Buffers leaf.
pub const SNAPSHOT: &str = include_str!("../../../../📸️snapshot/🛰️.proto");
/// 🛰️ The diff facet's Protocol Buffers leaf.
pub const DIFF: &str = include_str!("../../../../🔺️diff/🛰️.proto");
/// 🛰️ The mutations facet's Protocol Buffers leaf.
pub const MUTATIONS: &str = include_str!("../../../../🧬️mutations/🛰️.proto");
