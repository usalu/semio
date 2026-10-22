//! 🎞️ Media commands: media import, export submission, poll, cancel and fingerprint of one plugin exchange. See [`super`].

use super::*;

/// 🎙️ Imports a media port payload.
pub(super) async fn arm_media_in<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    cx: &mut semio_framework_job::StepContext<'_>,
    frames: &mut Vec<protocol::AppFrame>,
    flags: &mut ExchangeFlags,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::MediaIn { seq, port, descriptor, data } = command else { unreachable!("arm_media_in is dispatched only for its own command") };
    let descriptor_value: DslValue = decode_wire_serialized_or(&descriptor, DslValue::Null).await;
    let descriptor_json = serde_json::to_string(&descriptor_value).unwrap_or_else(|_| "{}".into());
    match plugin_consume_media(runtime, instance_id, seq, &port, &descriptor_json, data, cx).await {
        Ok(None) => {
            flags.mutated = true;
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
        }
        Ok(Some(status)) => frames.push(protocol::AppFrame::DocumentArchiveLoad { in_reply_to: seq, status }),
        Err(fault) => push_app_fault(frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 📤️ Answers a submitted media export handle.
pub(super) async fn arm_submit_media_export(
    frames: &mut Vec<protocol::AppFrame>,
    owner: &mut semio_framework_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::SubmitMediaExport { seq, .. } = command else { unreachable!("arm_submit_media_export is dispatched only for its own command") };
    let handle=owner.take_media_submission().expect("original media submission is complete before command donation");
    frames.push(protocol::AppFrame::MediaExportSubmitted{in_reply_to:seq,handle:media_export_handle_into_wire(handle)});
    Ok(())
}

/// ⏳️ Answers the poll status of a media export.
pub(super) async fn arm_poll_media_export(
    frames: &mut Vec<protocol::AppFrame>,
    owner: &mut semio_framework_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::PollMediaExport { seq, handle } = command else { unreachable!("arm_poll_media_export is dispatched only for its own command") };
    let handle = media_export_handle_from_wire(handle);
    if let Some(status)=owner.take_media_status(){frames.push(protocol::AppFrame::MediaExportStatus{in_reply_to:seq,handle:media_export_handle_into_wire(handle),state:protocol::MediaExportStateWire::Complete,applied_progress:status.total_bytes,checkpoint_available:false,mime_type:status.mime_type,total_bytes:status.total_bytes,detail:String::new()});}else{
    match owner.take_media_poll().expect("original media poll is retained before command donation") {
        ArtifactMediaExportPoll::Running { applied_progress, checkpoint_available } => frames.push(protocol::AppFrame::MediaExportStatus {
            in_reply_to: seq,
            handle: media_export_handle_into_wire(handle),
            state: protocol::MediaExportStateWire::Running,
            applied_progress,
            checkpoint_available,
            mime_type: String::new(),
            total_bytes: 0,
            detail: String::new(),
        }),
        ArtifactMediaExportPoll::Complete(_) => unreachable!("terminal result donates only to its original download registry before decoded command consumption"),
        ArtifactMediaExportPoll::Cancelled => frames.push(protocol::AppFrame::MediaExportStatus {
            in_reply_to: seq,
            handle: media_export_handle_into_wire(handle),
            state: protocol::MediaExportStateWire::Cancelled,
            applied_progress: 0,
            checkpoint_available: false,
            mime_type: String::new(),
            total_bytes: 0,
            detail: String::new(),
        }),
        ArtifactMediaExportPoll::Failed(detail) => frames.push(protocol::AppFrame::MediaExportStatus {
            in_reply_to: seq,
            handle: media_export_handle_into_wire(handle),
            state: protocol::MediaExportStateWire::Failed,
            applied_progress: 0,
            checkpoint_available: false,
            mime_type: String::new(),
            total_bytes: 0,
            detail,
        }),
    }
}
    Ok(())
}

/// ❌️ Cancels a media export.
pub(super) async fn arm_cancel_media_export<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::CancelMediaExport { seq, handle } = command else { unreachable!("arm_cancel_media_export is dispatched only for its own command") };
    let handle = media_export_handle_from_wire(handle);
    match plugin_cancel_media_export(runtime, instance_id, &handle).await {
        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
        Err(error) => push_app_fault(frames, Some(seq), plugin_internal_fault(error.to_string())).await,
    }
    Ok(())
}

/// 🖐️ Answers the fingerprint of a media port.
pub(super) async fn arm_media_fingerprint<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::MediaFingerprint { seq, port } = command else { unreachable!("arm_media_fingerprint is dispatched only for its own command") };
    let fingerprint = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.media_fingerprint(&port)).map_err(|error| plugin_internal_fault(error.to_string()))
    });
    match fingerprint.await {
        Ok(fingerprint) => {
            let value = DslValue::String(fingerprint.0);
            frames.push(protocol::AppFrame::MediaFingerprint { in_reply_to: seq, port, fingerprint: encode_wire_serialized(&value) });
        }
        Err(fault) => push_app_fault(frames, Some(seq), fault).await,
    }
    Ok(())
}
