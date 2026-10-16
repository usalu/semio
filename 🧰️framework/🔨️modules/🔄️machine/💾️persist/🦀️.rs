//! 💾️ Versioned persistence — stable string state ids, fingerprint-checked restore.
//!
//! Never serializes JS promises, futures, callbacks, timer handles, or actor
//! references — only the logical configuration, history and context survive.

use super::kernel::{Snapshot, Status};
use crate::{Configuration, Machine, NodeId};
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};

//#region 🔖️Persist

/// 💾️ A machine's logical state, addressed by stable string ids so adding/renumbering
/// states never invalidates a previously persisted snapshot.
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PersistedSnapshot {
    pub version: u32,
    pub fingerprint: u64,
    pub states: Vec<String>,
    pub history: Vec<(String, Vec<String>)>,
    pub done: bool,
}

/// 💾️ Why [`restore`] could not rebuild a [`Snapshot`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreError {
    /// No [`Migration`] bridges the persisted fingerprint to the current machine.
    FingerprintMismatch,
    /// A persisted stable id no longer exists in the current machine.
    UnknownStableId(String),
}

/// 💾️ Migrates a [`PersistedSnapshot`] captured under an older machine fingerprint.
#[dyn_enum]
pub trait Migration {
    /// The fingerprint this migration accepts as input.
    fn source_fingerprint(&self) -> u64;
    /// Produces a [`PersistedSnapshot`] valid under a newer fingerprint.
    fn migrate(&self, snapshot: PersistedSnapshot) -> PersistedSnapshot;
}

// 🧬️ `restore`/`step` take a caller-supplied, potentially heterogeneous list of migrations — per
// R11, an open-shaped parameter like this becomes generic (`Mg: Migration`) rather than
// `dyn Migration`; when a real caller someday needs to mix several concrete migration types in one
// call, ITS crate declares its own closed `dyn_enum_close!` enum over just the types it needs and
// passes that as `Mg`. Today nothing outside tests implements `Migration`, so production callers
// that pass an empty slice use this zero-variant enum — the same uninhabited shape proven by
// `semio-framework-dispatch-macros`' own `NoWidgets`/`NoCounters` tests.
dyn_enum_close! {
    pub enum NoMigrations: Migration {}
}

/// 💾️ Captures a running [`Snapshot`] as a portable, stable-id-addressed value.
pub fn persist<M: Machine>(snapshot: &Snapshot<M>) -> PersistedSnapshot {
    let def = M::definition();
    let states = snapshot.configuration.iter_ones().map(|id| def.nodes[id.0 as usize].stable_id.to_string()).collect();
    let history = snapshot.history_entries().iter().map(|(owner, ids)| (def.nodes[owner.0 as usize].stable_id.to_string(), ids.iter().map(|id| def.nodes[id.0 as usize].stable_id.to_string()).collect())).collect();
    PersistedSnapshot { version: 1, fingerprint: def.fingerprint, states, history, done: matches!(snapshot.status, Status::Done(_)) }
}

fn stable_id_to_node(def_nodes: &[super::kernel::NodeDef], stable_id: &str) -> Result<NodeId, RestoreError> {
    def_nodes.iter().position(|n| n.stable_id == stable_id).map(|idx| NodeId(idx as u16)).ok_or_else(|| RestoreError::UnknownStableId(stable_id.to_string()))
}

/// 💾️ Rebuilds a [`Snapshot`] from a [`PersistedSnapshot`], applying `migrations` in
/// sequence until the fingerprint matches the current machine, then re-resolving
/// stable ids back to dense [`NodeId`]s. `context` is supplied by the caller since
/// the consumer's `Context` may itself need domain-specific deserialization.
pub fn restore<M: Machine, Mg: Migration>(persisted: &PersistedSnapshot, context: M::Context, migrations: &[&Mg]) -> Result<Snapshot<M>, RestoreError> {
    let def = M::definition();
    let mut current = persisted.clone();
    while current.fingerprint != def.fingerprint {
        match migrations.iter().find(|m| m.source_fingerprint() == current.fingerprint) {
            Some(m) => current = m.migrate(current),
            None => return Err(RestoreError::FingerprintMismatch),
        }
    }
    let mut configuration = M::Config::default();
    for stable_id in &current.states {
        configuration.set(stable_id_to_node(def.nodes, stable_id)?);
    }
    let mut history = Vec::new();
    for (owner_id, ids) in &current.history {
        let owner = stable_id_to_node(def.nodes, owner_id)?;
        let mut resolved = Vec::new();
        for id in ids {
            resolved.push(stable_id_to_node(def.nodes, id)?);
        }
        history.push((owner, resolved));
    }
    let status = Status::Running;
    Ok(Snapshot::from_parts(configuration, context, status, history))
}

//#endregion 🔖️Persist

//#region 🧪️Tests

#[cfg(test)]
include!("../🧪️tests/🔬️persist-unit/🦀️.rs");

//#endregion 🧪️Tests
