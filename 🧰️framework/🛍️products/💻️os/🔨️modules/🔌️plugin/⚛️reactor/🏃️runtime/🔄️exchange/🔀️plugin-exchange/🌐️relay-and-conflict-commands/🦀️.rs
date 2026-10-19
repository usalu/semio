//! 🌐️ Relay and conflict commands: artifact open, default app, merge policy and conflict resolution of one plugin exchange. See [`super`].

use super::*;

/// 🚪️ Relays an artifact open request.
pub(super) async fn arm_open_artifact(
    frames: &mut Vec<protocol::AppFrame>,
    effect_bytes: &mut Vec<Vec<u8>>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } = command else { unreachable!("arm_open_artifact is dispatched only for its own command") };
    match relay_open_artifact(artifact_ref, role, plugin_id, app_id).await {
        Ok(effect) => {
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
            effect_bytes.push(encode_wire_serialized(&effect));
        }
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 📌️ Relays a default app assignment.
pub(super) async fn arm_set_default_app(
    frames: &mut Vec<protocol::AppFrame>,
    effect_bytes: &mut Vec<Vec<u8>>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } = command else { unreachable!("arm_set_default_app is dispatched only for its own command") };
    match relay_set_default_app(artifact_kind, standard, subset, role, plugin_id, app_id).await {
        Ok(effect) => {
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
            effect_bytes.push(encode_wire_serialized(&effect));
        }
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🧹️ Relays a default app removal.
pub(super) async fn arm_clear_default_app(
    frames: &mut Vec<protocol::AppFrame>,
    effect_bytes: &mut Vec<Vec<u8>>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } = command else { unreachable!("arm_clear_default_app is dispatched only for its own command") };
    match relay_clear_default_app(artifact_kind, standard, subset, role).await {
        Ok(effect) => {
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
            effect_bytes.push(encode_wire_serialized(&effect));
        }
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// ⚖️ Sets the merge policy.
pub(super) async fn arm_set_merge_policy<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::SetMergePolicy { seq, policy } = command else { unreachable!("arm_set_merge_policy is dispatched only for its own command") };
    let outcome = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(set_merge_policy_frames(&mut instance.app, seq, policy))
    });
    match outcome.await {
        Ok(outcome) => frames.extend(outcome),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🤝️ Resolves one open conflict.
pub(super) async fn arm_resolve_conflict<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ResolveConflict { seq, conflict_id, resolution } = command else { unreachable!("arm_resolve_conflict is dispatched only for its own command") };
    let outcome = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(resolve_conflict_frames(&mut instance.app, seq, conflict_id, resolution))
    });
    match outcome.await {
        Ok(outcome) => frames.extend(outcome),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// ⚔️ Reads the open conflicts.
pub(super) async fn arm_read_conflicts<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ReadConflicts { seq } = command else { unreachable!("arm_read_conflicts is dispatched only for its own command") };
    let outcome = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(::semio_framework_async::poll::resolve_ready(read_conflicts_frames(&instance.app, seq)))
    });
    match outcome.await {
        Ok(outcome) => frames.extend(outcome),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}
