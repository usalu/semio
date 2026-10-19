//! 🗃️ Puzzle3d SQLite snapshot laws — native capability, typed and play codecs, all 23 domain tables, independent edits and the populated public app forms.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_model::io::sqlite::snapshot::*;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_s_artifact_puzzle_3d_model::io::sqlite::snapshot::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use store::{
    sqlite_snapshot::{
        artifact::{Cell, Projection, Reconstruction},
        SnapshotEncoding, SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SqliteValue,
    },
    os_store::serialization::ArtifactSqliteSnapshotHost,
};
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;

#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
pub mod tests;
