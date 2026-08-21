//! 📸️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4). `checkpoint()`/`restore()` —
//! there is no more `InstanceGuard` to heal after a trap; a panic now aborts the whole actor and
//! the host restores it from the last checkpoint here, which is what makes checkpoint cadence
//! correctness-critical (design-abi.md §4).
//!
//! ⚠️ Scope note (reported honestly): this wave ships the pack ENVELOPE — `instances` (id +
//! app_id + document/config/draft packs via the SAME `plugin_document_pack`/`plugin_load_document_
//! pack` round trip `AppCommand::LoadDocument`/`ReadDocument` already use), `timers` (id list from
//! `⚛️reactor`'s pending `SetTimer` bookkeeping), `pending_requests` (from `RequestRegistry::
//! pending_ids`, per design-abi.md §4: async tasks are never serialised, only marked
//! re-run-on-restore), and `task_restarts` (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: one
//! `{instance, command}` pair per LIVE `AsyncTask` that was built with `.restartable(command)` —
//! the SAME "never serialise the task itself, only mark it re-run-on-restore" contract, applied to
//! `Emit.tasks` now that it exists). `view_state`/`ephemeral` per instance are NOT captured yet —
//! `AppInstance` doesn't expose a public read for either today, and adding that read is `app`
//! module surface (design-abi.md §4 says `app` "stays"); flagged as a `lease-request` in the
//! report rather than reached into silently.

use crate::plugin_runtime;
use semio_framework::Fault;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct InstanceCheckpoint {
    id: u32,
    app_id: String,
    document_pack: Vec<u8>,
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): one `AsyncTask::restart`
/// survivor — `command` is the SAME OpBinary-encoded bytes `AsyncTask::restartable(command)` was
/// built with, re-dispatched against `instance` (via `TaskResolution::Command`'s exact resume
/// path — Elm's Msg-from-Cmd, re-entering `ArtifactApp::handle` against the JUST-restored state)
/// the first `poll` after `restore`.
#[derive(Clone, Serialize, Deserialize)]
pub struct TaskRestart {
    pub instance: u32,
    pub command: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct CheckpointPack {
    instances: Vec<InstanceCheckpoint>,
    timers: Vec<u64>,
    pending_requests: Vec<u64>,
    #[serde(default)]
    task_restarts: Vec<TaskRestart>,
}

impl CheckpointPack {
    /// 🪪️ `(id, app_id)` pairs restored — `⚛️reactor::restore_now` reseeds `OPEN_INSTANCES` from
    /// this so a later `checkpoint_now` round-trips correctly.
    pub async fn instances(&self) -> Vec<(u32, String)> {
        self.instances.iter().map(|instance| (instance.id, instance.app_id.clone())).collect()
    }

    pub async fn timers(&self) -> &[u64] {
        &self.timers
    }

    pub async fn pending_requests(&self) -> &[u64] {
        &self.pending_requests
    }

    pub async fn task_restarts(&self) -> &[TaskRestart] {
        &self.task_restarts
    }
}

/// 📸️ Builds the checkpoint pack for every currently-open instance in this actor. `document_pack`
/// is `store::encode_document_pack_bytes(files.pack, files.spr)` — the SAME wire codec
/// `AppCommand::LoadDocument`/`ReadDocument` already use for a whole document as one binary blob;
/// `files.ops` (a derived text mirror, never authoritative) is not carried.
pub async fn checkpoint(instance_ids: &[(u32, String)], timers: Vec<u64>, pending_requests: Vec<u64>, task_restarts: Vec<TaskRestart>) -> Result<Vec<u8>, Fault> {
    let mut instances = Vec::with_capacity(instance_ids.len());
    for (id, app_id) in instance_ids {
        let files = plugin_runtime::plugin_document_pack(*id).await.unwrap_or_default();
        let document_pack = store::encode_document_pack_bytes(&files.pack, &files.spr).await;
        instances.push(InstanceCheckpoint { id: *id, app_id: app_id.clone(), document_pack });
    }
    let pack = CheckpointPack { instances, timers, pending_requests, task_restarts };
    serde_json::to_vec(&pack).map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.checkpoint.encode"), error.to_string()))
}

/// 📸️ Restores every instance recorded in `state`, re-creating each and reloading its document
/// pack — `⚛️reactor::poll`'s caller is responsible for re-arming `timers`/treating
/// `pending_requests` as stale (design-abi.md §4).
pub async fn restore(state: &[u8]) -> Result<CheckpointPack, Fault> {
    let pack: CheckpointPack = serde_json::from_slice(state).map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.checkpoint.decode"), error.to_string()))?;
    for instance in &pack.instances {
        let new_id = plugin_runtime::plugin_create_app(&instance.app_id).await?;
        if !instance.document_pack.is_empty() {
            let (doc_pack, spr) =
                store::decode_document_pack_bytes(&instance.document_pack).await.map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.checkpoint.decode-document"), format!("{error:?}")))?;
            let files = store::ArtifactPackFiles { pack: doc_pack, spr, ops: String::new() };
            plugin_runtime::plugin_load_document_pack(new_id, &files).await?;
        }
    }
    Ok(pack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn checkpoint_of_no_instances_round_trips_through_json() {
        let bytes = checkpoint(&[], vec![1, 2], vec![7], Vec::new()).await.expect("an empty instance list must still encode");
        let pack: CheckpointPack = serde_json::from_slice(&bytes).expect("checkpoint bytes must be valid CheckpointPack json");
        assert!(pack.instances.is_empty());
        assert_eq!(pack.timers, vec![1, 2]);
        assert_eq!(pack.pending_requests, vec![7]);
        assert!(pack.task_restarts.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn task_restarts_round_trip_through_json_and_are_exposed_by_the_accessor() {
        let restarts = vec![TaskRestart { instance: 5, command: vec![1, 2, 3] }, TaskRestart { instance: 6, command: vec![4] }];
        let bytes = checkpoint(&[], Vec::new(), Vec::new(), restarts.clone()).await.expect("must encode");
        let pack = restore(&bytes).await.expect("must decode back");
        assert_eq!(pack.task_restarts().await.len(), 2);
        assert_eq!(pack.task_restarts().await[0].instance, 5);
        assert_eq!(pack.task_restarts().await[0].command, vec![1, 2, 3]);
        assert_eq!(pack.task_restarts().await[1].instance, 6);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_checkpoint_pack_encoded_before_task_restarts_existed_still_decodes() {
        // 🧬️ `#[serde(default)]` on `task_restarts` — an older pack (or a hand-built JSON blob
        // missing the field entirely) must not fail to restore just because this wave added a
        // field to the envelope (greenfield repo, no migration script, but a checkpoint taken
        // moments before an actor upgrade is a real same-process scenario, not legacy support).
        let legacy_json = r#"{"instances":[],"timers":[],"pending_requests":[]}"#;
        let pack: CheckpointPack = serde_json::from_str(legacy_json).expect("a pack missing task_restarts must still decode");
        assert!(pack.task_restarts().await.is_empty());
    }
}
