//! 🧩️ Space-scoped fixed-chunk artifact CAS and canonical manifest codec.

use super::file_fence::{try_acquire, FileFence};
use super::{ArtifactBlobIntegrity, ArtifactPair, AuthorityError, AuthorityProgress, AuthorityProgressStage, ImmutableArtifactBlobStore, OperationContext, StagedArtifactBlob, AUTHORITY_MAX_PAIR_BYTES};
use directory::os_directory::{ArtifactCheckpoint, ArtifactHash, DocumentScope};
use directory::os_directory::io::binary::artifact_hash::hex_lower;
use semio_framework_hash::Sha256;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[path = "🧬️object-model/🦀️.rs"]
mod object_model;
pub use object_model::*;

#[path = "🚪️storage-port/🦀️.rs"]
mod storage_port;
pub use storage_port::*;

#[path = "🧮️manifest-codec/🦀️.rs"]
mod manifest_codec;
pub use manifest_codec::*;

#[path = "🧠️memory-storage/🦀️.rs"]
mod memory_storage;
pub use memory_storage::*;

#[path = "📁️filesystem-storage/🦀️.rs"]
mod filesystem_storage;
pub use filesystem_storage::*;

#[path = "🗄️sqlite-storage/🦀️.rs"]
mod sqlite_storage;
pub use sqlite_storage::*;

#[path = "🐘️postgres-storage/🦀️.rs"]
mod postgres_storage;
pub use postgres_storage::*;

#[path = "🕸️neo4j-storage/🦀️.rs"]
mod neo4j_storage;
pub use neo4j_storage::*;

#[path = "🔀️store-selector/🦀️.rs"]
mod store_selector;
pub use store_selector::*;

#[path = "🫙️blob-store/🦀️.rs"]
mod blob_store;
pub use blob_store::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
