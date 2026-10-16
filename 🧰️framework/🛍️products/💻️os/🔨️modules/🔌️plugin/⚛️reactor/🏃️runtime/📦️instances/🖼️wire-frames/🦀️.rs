//! 🖼️ Fault frames, merge-conflict frames and the instance registry accessors of the runtime.

use super::*;
pub(super) async fn push_app_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, fault: Fault) {
    frames.push(protocol::AppFrame::Error { in_reply_to, fault: encode_wire_serialized(&fault), report: Vec::new() });
}
pub(super) async fn push_dispatch_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: u64, fault: Fault, report: &protocol::DispatchReport) {
    frames.push(protocol::AppFrame::Error { in_reply_to: Some(in_reply_to), fault: encode_wire_serialized(&fault), report: encode_wire_serialized(report) });
}
pub(super) async fn push_os_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, code: &str, message: String) {
    push_app_fault(frames, in_reply_to, Fault::new(FaultOrigin::Os, FaultCode::new(code), message)).await;
}
pub(super) async fn decode_wire_serialized_or<T: FromValue>(bytes: &[u8], default: T) -> T {
    decode_wire_serialized(bytes).await.unwrap_or(default)
}
/// ⚖️ Executes the channel's local policy command and returns its canonical acknowledgment frame.
/// 🗃️ sdk-dedyn (O1/§1.5): generic `<PA: PluginApp>` replaces `&mut dyn PluginApp`.
pub(super) async fn set_merge_policy_frames<PA: PluginApp>(app: &mut PA, seq: u64, policy: u8) -> Result<Vec<protocol::AppFrame>, Fault> {
    let policy = protocol::MergePolicy::from_u8(policy).ok_or_else(|| Fault::new(FaultOrigin::Os, FaultCode::new("merge.invalid-policy"), format!("unknown merge policy ordinal {policy}")))?;
    app.set_merge_policy(policy).await;
    Ok(vec![protocol::AppFrame::Done { in_reply_to: seq }])
}
/// ⚔️ Executes the channel's conflict-resolution command and returns the authoritative report and open projection.
pub(super) async fn resolve_conflict_frames<PA: PluginApp>(app: &mut PA, seq: u64, conflict_id: String, resolution: u8) -> Result<Vec<protocol::AppFrame>, Fault> {
    let resolution = match resolution {
        0 => protocol::ConflictResolution::Accept,
        1 => protocol::ConflictResolution::Discard,
        _ => return Err(Fault::new(FaultOrigin::Os, FaultCode::new("merge.invalid-resolution"), format!("unknown conflict resolution ordinal {resolution}"))),
    };
    let report = app.resolve_conflict(&conflict_id, resolution).await?;
    let conflicts = app.open_conflicts().await;
    Ok(vec![protocol::AppFrame::MergeReport { in_reply_to: Some(seq), report: encode_wire_serialized(&report) }, protocol::AppFrame::Conflicts { in_reply_to: Some(seq), conflicts: encode_wire_serialized(&conflicts) }])
}
/// ⚔️ Executes the channel's open-conflict read command and returns its canonical projection frame.
pub(super) async fn read_conflicts_frames<PA: PluginApp>(app: &PA, seq: u64) -> Vec<protocol::AppFrame> {
    vec![protocol::AppFrame::Conflicts { in_reply_to: Some(seq), conflicts: encode_wire_serialized(&app.open_conflicts().await) }]
}
/// 🪪️ Reads the actor retained by exact instance-open admission.
pub(crate) async fn instance_actor<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<String, Fault> {
    runtime.instance_actors.try_borrow().map_err(|_| plugin_internal_fault("runtime actor authority is busy"))?.get(instance_id).map(RuntimeActorAuthority::to_string).ok_or_else(|| plugin_internal_fault("instance has no admitted actor authority"))
}
/// 🧬️ The local async mutex keeps the instance in-place across a suspending app call. Dropping an
/// in-flight future releases the RAII guard and wakes the next caller without losing the instance.
pub(super) async fn with_instances_mut<PA: PluginApp, R, F: FnOnce(&mut RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>) -> Result<R, Fault>>(runtime: &PluginRuntime<PA>, f: F) -> Result<R, Fault> {
    let mut instances = runtime.instances.try_borrow_mut().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
    f(&mut instances)
}
/// 🎟️ Keeps an action source's complete original failure under the actual runtime registry loan.
pub(super) async fn with_instances_turn_mut<PA:PluginApp,R>(runtime:&PluginRuntime<PA>,f:impl FnOnce(&mut RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<R,semio_framework_job::OriginalTurnFailure>{let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority is busy"))?;f(&mut instances)}
/// 🪢️ Returns complete lifecycle causes through the same original registry authority.
pub(super) async fn with_instances_original_mut<PA:PluginApp,R>(runtime:&PluginRuntime<PA>,f:impl FnOnce(&mut RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>)->Result<R,crate::PluginLifecycleFailure>)->Result<R,crate::PluginLifecycleFailure>{let mut instances=runtime.instances.try_borrow_mut().map_err(|_|plugin_internal_fault("runtime instance authority is busy"))?;f(&mut instances)}
/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: registers `instance` directly into `INSTANCES`,
/// bypassing `plugin_create_app_with_id`'s `PLUGIN`/`PluginProgram` requirement — a real
/// `AsyncTask` resume test only needs A `dyn PluginApp` at a known id, not a fully assembled
/// plugin bundle. `#[cfg(test)]`-gated, never part of the real API surface.
#[cfg(test)]
pub(super) async fn test_push_instance<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance: AppInstance<PA>) {
    with_instances_mut(runtime, |list| list.insert(instance.id, std::sync::Arc::new(RuntimeAppCell::new(instance, runtime.mounted_policy))).map_err(|_| plugin_internal_fault("fixed runtime instance authority is saturated or collided")))
        .await
        .expect("test_push_instance must not fail");
}
/// 🪪️ Admits the actor that an instance pushed straight into a test runtime would otherwise lack: the exact authority instance-open admission retains for every later exchange, resume and restore.
#[cfg(test)]
pub(super) fn test_admit_instance_actor<PA: PluginApp>(runtime: &PluginRuntime<PA>, id: u32, actor: &str) {
    let authority = RuntimeActorAuthority::new(actor).expect("test actor authority");
    runtime.instance_actors.try_borrow_mut().expect("test actor authority is free").insert_admitted(id, authority);
}
