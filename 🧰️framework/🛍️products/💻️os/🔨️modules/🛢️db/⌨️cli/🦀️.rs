//! 🗄️ `db_cli` — the `db` binary: `inspect`/`doc`/`wal-inspect`/`snapshot-inspect`/`verify`/
//! `query`/`replay`/`repair`/`compact`/`health`/`conflict-simulate`/`replica-simulate`/`migrate`/
//! `profile` over a `db::storage::FsStorage`-rooted document store. Hand-rolled arg parsing (no
//! external CLI crate, per repo convention), exit codes `0` (success) / `1` (operation failed) /
//! `2` (usage error), never panics on corrupt input. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_cli` row).
//!
//! 🎯️ Design choice (depends on the `db` facade alone, `db_*` paths nowhere): every subcommand
//! below reaches every primitive it needs purely through `db::<submodule>::…` paths (`db::storage`,
//! `db::wal`, `db::snapshot`, `db::conflict`, `db::cluster`, `db::observe`, `db::document`,
//! `db::actor`, `db::core`, …) — the facade's own re-exports, verified complete against `db/rs/
//! lib.rs`. This crate's `Cargo.toml` accordingly depends on nothing but `db` itself plus two
//! siblings that are NOT `db_*` crates: `protocol` (every frozen `Database`/`ArtifactHandle` entry
//! point is typed against `protocol::ArtifactId`/`MutationEnvelope`/…, which the facade exposes
//! without re-exporting a path to) and `pack` (`SnapshotManager::verify`'s `VerificationLevel` is
//! pack's own type, snapshots being pack files). No `db_storage`/`db_wal`/`db_snapshot`/… path
//! dependency of its own — `wal-inspect`/`snapshot-inspect`/`replay`/`repair` still need
//! lower-level access than the actor-mediated `Database` API exposes (there is no `Database` method
//! that lists WAL segments or snapshot generations), reached the same facade-path way.
//!
//! 🎯️ Every subcommand is real, including `migrate` and `profile`: `migrate` appends a genuine
//! `WAL_MIGRATION` record via `db::wal::ArtifactWal` (force-flushed durably); `profile` submits `N`
//! real commands sequentially through `ArtifactHandle::submit` and reports wall-clock throughput —
//! both self-contained (bootstrap their own document/WAL if it doesn't exist yet), needing nothing
//! from `db_fault_testing` (a separate, non-`db`-facade sibling crate this one deliberately does not
//! depend on, to keep the "facade only" dependency footprint honest). `conflict-simulate` runs the
//! genuine `db::conflict::ConflictDetector` over two hand-built `CommandTouch`es (no storage
//! touched); `replica-simulate` runs the genuine `db::cluster::replicate_document` primitive between
//! two local `FsStorage` roots (no network transport exists yet in this family, which is exactly why
//! this stays a *simulation* rather than a real cluster operation).
#![allow(async_fn_in_trait)]
#![cfg(not(target_arch = "wasm32"))]
extern crate semio_framework_os_kernel as pack;

use std::collections::HashMap;
use std::path::Path;

use db_io_core::{SnapshotStorage as _, WalStorage as _};

#[path = "✂️args-format-and-inspect/🦀️.rs"]
mod args_format_and_inspect;
use args_format_and_inspect::{
    close_document_open_rejected, close_replication_rejected, cmd_doc, cmd_inspect,
    describe_value_bytes, fail, fail_document_open, fail_wal_open, hex32, now_ms, open_database,
    open_fs_storage, parse_args, parse_merge_policy, parse_profile, print_health, strip_flag, usage,
};

#[path = "🔚️command-close/🦀️.rs"]
mod command_close;
#[cfg(test)]
use command_close::{
    cli_command_close_witness, CliCommandCloseExit, CliCommandCloseTerminal, CliCommandCloseWitness,
    CLI_COMMAND_CLOSE_INTERRUPTED,
};
use command_close::{
    MountedSnapshotCommandClose, MountedWalBatchCommandClose, MountedWalRecordCommandClose,
    MountedWalReplayCommandClose,
};

#[path = "📜️wal-and-snapshot-inspect/🦀️.rs"]
mod wal_and_snapshot_inspect;
use wal_and_snapshot_inspect::{
    admit_cli_wal_bytes, cmd_snapshot_inspect, cmd_wal_inspect, wal_cursor_control,
    wal_record_kind_name,
};

#[path = "🩺️verify-query-replay-repair-compact-health/🦀️.rs"]
mod verify_query_replay_repair_compact_health;
#[cfg(test)]
use verify_query_replay_repair_compact_health::verify_document;
use verify_query_replay_repair_compact_health::{
    cmd_compact, cmd_health, cmd_query, cmd_repair, cmd_replay, cmd_verify,
};

#[path = "🚀️simulate-migrate-profile-and-main/🦀️.rs"]
mod simulate_migrate_profile_and_main;
pub use simulate_migrate_profile_and_main::main_impl;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
