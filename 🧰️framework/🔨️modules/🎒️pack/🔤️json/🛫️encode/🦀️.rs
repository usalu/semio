//! 🧭️ Canonical typed JSON encoding owns borrowed text, scalar and retained tree frontiers.
#[path = "🔤️text/🦀️.rs"]
mod native_text;
pub use native_text::{ArtifactCanonicalJsonText, ArtifactCanonicalJsonTextCursor, ArtifactCanonicalJsonTextStep, canonical_escape, canonical_escaped_byte};
#[path = "🔣️scalar/🦀️.rs"]
mod native_scalar;
pub use native_scalar::{ArtifactCanonicalJsonNode, ArtifactCanonicalJsonScalarBytes};
#[path = "🧭️tree/🦀️.rs"]
mod native_tree;
pub use native_tree::{retained_alias, retained_alias_demand, retained_alias_work_demand, ArtifactCanonicalDecimalI64, ArtifactCanonicalDecimalU64, ArtifactCanonicalHexWordArray, ArtifactCanonicalHexWordF32, ArtifactCanonicalHexWordList, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, ArtifactCanonicalJsonTreeStep, ArtifactCanonicalJsonTreeWireCursor, ArtifactCanonicalJsonTreeWireStep};

#[cfg(feature = "testkit")]
pub mod testkit { pub use super::native_tree::testkit as tree; }

#[cfg(test)]
extern crate self as semio_framework_pack_json;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
