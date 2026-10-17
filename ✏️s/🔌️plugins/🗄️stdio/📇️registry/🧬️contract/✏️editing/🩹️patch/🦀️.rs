//! 🩹️ Path-scoped native snapshot edits: ONE RFC 6901 pointer operation per mutation, its exact inverse, and the input
//! schema that types its value by the snapshot sub-schema the pointer addresses, so a history edit renders the control the
//! snapshot schema declares for that field (design §20.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
//!
//! @see ../🟦️.ts — the TypeScript twin of preparation, application, inversion and location.
//! @see ../../../🧬️schema/🔣️.json — `$defs/SnapshotPatch`, the wire schema of [`SnapshotPatch`].

use semio_s_artifact_stdio_contract_editing::{
    array_index, decode_pointer, snapshot_from_edit_source, snapshot_schema_descriptor_for_dialect, validate_snapshot_schema_for_dialect, validate_value, values_equivalent, SnapshotEditError, SnapshotEditEvent, SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES,
    SNAPSHOT_SCHEMA_VALIDATORS,
};
use semio_framework_os_kernel as kernel;
use semio_framework_value_derive as value_derive;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use kernel::OpBinary;
use semio_framework_value::ToValue;
use semio_framework_value::ValueEdit;
use semio_framework_value::ValueShape;
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub const SNAPSHOT_PATCH_MAX_BYTES: usize = 1_048_576;
pub const SNAPSHOT_PATCH_MAX_SEGMENTS: usize = 128;
/// 🧩️ The most parts one exact inverse may take (`x-semio-inverse-rows.bounded` of every `patch-snapshot` leaf): a value from a
/// 16 MiB source projects to at most ~64 MiB of canonical JSON (octets as decimal arrays), i.e. ~64 parts of the patch budget.
pub const SNAPSHOT_PATCH_MAX_INVERSE_PARTS: usize = 128;
const SNAPSHOT_PATCH_MAX_INDEX: u64 = 9_007_199_254_740_991;
const SNAPSHOT_SCHEMA_REF_HOPS: usize = 32;

fn is_false(value: &bool) -> bool {
    !*value
}

#[path = "📜️patch-wire/🦀️.rs"]
mod patch_wire;
pub use patch_wire::*;

#[path = "📦️codec/🫳️borrowed/🦀️.rs"]
mod borrowed_operation_source;
pub use borrowed_operation_source::SnapshotPatchReadCursor;

#[path = "🪜️patch-steps/🦀️.rs"]
mod patch_steps;
use patch_steps::*;

#[path = "🧭️patch-edits/🦀️.rs"]
mod patch_edits;
pub use patch_edits::*;

#[path = "🧩️inverse-parts/🦀️.rs"]
mod inverse_parts;
use inverse_parts::*;

#[path = "🧬️input-schema/🦀️.rs"]
mod input_schema;
pub use input_schema::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
