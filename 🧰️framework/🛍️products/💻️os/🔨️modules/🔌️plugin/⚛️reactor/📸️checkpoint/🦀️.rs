//! 📸️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4). `checkpoint()`/`restore()` —
//! there is no more `InstanceGuard` to heal after a trap; a panic now aborts the whole actor and
//! the host restores it from the last checkpoint here, which is what makes checkpoint cadence
//! correctness-critical (design-abi.md §4).
//!
//! ⚠️ Scope note (reported honestly): this wave ships the pack ENVELOPE — `instances` (id +
//! app_id + the document pack `plugin_document_pack` reads, restored through the stepped document
//! archive load of `📓️api-stepped-document-load.md` §4), `timers` (id list from
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
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, ToValue, Deserialize, FromValue)]
#[serde(deny_unknown_fields)]
struct InstanceCheckpoint {
    id: u32,
    app_id: String,
    actor: String,
    document_pack: Vec<u8>,
}

/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): one `AsyncTask::restart`
/// survivor — `command` is the SAME OpBinary-encoded bytes `AsyncTask::restartable(command)` was
/// built with, re-dispatched against `instance` (via `TaskResolution::Command`'s exact resume
/// path — Elm's Msg-from-Cmd, re-entering `ArtifactApp::handle` against the JUST-restored state)
/// the first `poll` after `restore`.
#[derive(Clone, Serialize, ToValue, Deserialize, FromValue)]
#[serde(deny_unknown_fields)]
pub struct TaskRestart {
    pub instance: u32,
    pub command: Vec<u8>,
}

#[derive(Serialize, ToValue, Deserialize, FromValue)]
#[serde(deny_unknown_fields)]
pub struct CheckpointPack {
    instances: Vec<InstanceCheckpoint>,
    timers: Vec<u64>,
    pending_requests: Vec<u64>,
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
/// is `store::encode_document_pack_bytes(files.pack, files.spr)` — a whole document as one binary
/// blob; `files.ops` (a derived text mirror, never authoritative) is not carried.
pub async fn checkpoint<PA: crate::app::PluginApp>(runtime: &plugin_runtime::PluginRuntime<PA>, instance_ids: &[(u32, String)], timers: Vec<u64>, pending_requests: Vec<u64>, task_restarts: Vec<TaskRestart>) -> Result<Vec<u8>, Fault> {
    let mut instances = Vec::with_capacity(instance_ids.len());
    for (id, app_id) in instance_ids {
        let files = plugin_runtime::plugin_document_pack(runtime, *id).await?;
        let document_pack = store::encode_document_pack_bytes(&files.pack, &files.spr).await;
        let actor = plugin_runtime::instance_actor(runtime, *id).await?;
        instances.push(InstanceCheckpoint { id: *id, app_id: app_id.clone(), actor, document_pack });
    }
    let pack = CheckpointPack { instances, timers, pending_requests, task_restarts };
    Ok(semio_framework_pack_json::to_json_string(&pack).into_bytes())
}

/// 🛬️ The archive operation every restored instance's document load runs under: bit 62 alone, below the cold-pair
/// namespace (bit 63) and above any host-chosen archive load sequence. One id suffices because each restored instance is
/// fresh and owns at most one load.
pub(crate) const RESTORE_DOCUMENT_LOAD_OPERATION: u64 = 1 << 62;

/// 🫴️ Admits the original decoded actor text through the same decoder and caller wallet before its shared frame is born.
pub(crate) fn admit_restored_actor(source:&mut Option<String>,original:&mut semio_framework_os_kernel::io::control::NativeSnapshotDecodeOwner<'_, '_>)->Result<crate::protocol::ActorId,semio_framework_value::ValueError>{
    use semio_framework_value::{SharedUtf8,ValueError,ValueRefusalKind};
    if source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"restored actor requires its original decoded text"))}
    let demand=SharedUtf8::admission_demand();
    let grant=original.remaining_grant();
    if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"restored actor exceeds original item grant"))}
    if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"restored actor exceeds original depth grant"))}
    if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"restored actor exceeds original physical grant"))}
    original.native().checkpoint()?;
    original.native().charge(demand.capacity_bytes)?;
    let text=source.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"restored actor lost original decoded text"))?;
    match SharedUtf8::admit(text,grant){
        Ok((text,progress))=>{original.record_progress(progress)?;Ok(crate::protocol::ActorId(text))}
        Err((error,text))=>{*source=Some(text);Err(error)}
    }
}

/// 📸️ A decoded checkpoint and the instances whose document load it admitted under [`RESTORE_DOCUMENT_LOAD_OPERATION`].
pub struct RestoredCheckpoint {
    pub pack: CheckpointPack,
    pub document_loads: Vec<u32>,
}

/// 📸️ Restores every instance recorded in `state` under its checkpointed id and admits its document pack as a stepped
/// whole-document archive load (`📓️api-stepped-document-load.md` §4), which `⚛️reactor`'s turn drives to `Ready` while
/// the instance answers `document.loading`. `⚛️reactor::poll`'s caller is responsible for re-arming
/// `timers`/treating `pending_requests` as stale (design-abi.md §4).
pub async fn restore<PA: crate::app::PluginApp>(runtime: &plugin_runtime::PluginRuntime<PA>, state: &[u8]) -> Result<RestoredCheckpoint, Fault> {
    let state_text = std::str::from_utf8(state).map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.checkpoint.decode"), error.to_string()))?;
    let pack: CheckpointPack = semio_framework_pack_json::from_json_str(state_text, semio_framework_pack_json::JsonMemberPolicy::Reject)
        .map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.checkpoint.decode"), error.to_string()))?;
    let mut document_loads = Vec::with_capacity(pack.instances.len());
    for instance in &pack.instances {
        let id = plugin_runtime::plugin_create_app_with_id(runtime, instance.id, &instance.app_id, crate::protocol::ActorId(instance.actor.clone())).await?;
        if !instance.document_pack.is_empty() {
            let (parent_pack, parent_spr) =
                store::decode_document_pack_bytes(&instance.document_pack).await.map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("plugin.checkpoint.decode-document"), format!("{error:?}")))?;
            plugin_runtime::plugin_begin_document_archive_load(runtime, id, RESTORE_DOCUMENT_LOAD_OPERATION, store::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).await?;
            document_loads.push(id);
        }
    }
    Ok(RestoredCheckpoint { pack, document_loads })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
