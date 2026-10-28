//! 📸️ Product-neutral snapshot physical contracts.
#[path="../📝️text/📸️snapshot/🦀️.rs"]
mod text;
#[path="../💾️binary/📸️snapshot/🦀️.rs"]
mod binary;
pub use text::ArtifactDsl;
pub use binary::{ArtifactPack,ArtifactPackMetadata};

#[path="🫴️native/🦀️.rs"]
mod native;
pub use native::{ArtifactNativeSnapshot,NativeSnapshotInput,NativeSnapshotEncoding};

#[path="../🪶️sqlite-snapshot/📸️snapshot/🦀️.rs"]
mod sqlite;
pub use sqlite::ArtifactSqliteSnapshot;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
