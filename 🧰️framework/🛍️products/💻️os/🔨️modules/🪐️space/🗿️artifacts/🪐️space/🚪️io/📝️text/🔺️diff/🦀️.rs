//! 🚪️ Artifact diff representation.

#[allow(unused_imports)]
mod diff_codec {
use crate::*;
use protocol::{DiffText,DiffBinary};
use io::sqlite::snapshot::{register_sqlite_snapshot,SQLITE_SNAPSHOT_DIALECT};
use serde::{Deserialize, Serialize};

semio_framework_os_kernel::diff_text!(crate::SpaceDiff);
}
pub use diff_codec::*;
