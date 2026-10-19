//! 📦️ Document archive commands: archive load, merge, poll, cancel, acknowledge and the identity and archive reads of one plugin exchange. See [`super`].

use super::*;

/// 📥️ Starts loading a document archive.
pub(super) async fn arm_load_document_archive<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::LoadDocumentArchive { seq, archive } = command else { unreachable!("arm_load_document_archive is dispatched only for its own command") };
    let admitted = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.begin_document_archive_load(seq, archive)
    });
    match admitted.await {
        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🧩️ Starts merging a document archive.
pub(super) async fn arm_merge_document_archive<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::MergeDocumentArchive { seq, archive } = command else { unreachable!("arm_merge_document_archive is dispatched only for its own command") };
    let admitted = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.begin_document_archive_merge(seq, archive)
    });
    match admitted.await {
        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 📊️ Polls the progress of an archive load.
pub(super) async fn arm_poll_document_archive_load<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx: &mut semio_framework_job::StepContext<'_>,
    frames: &mut Vec<protocol::AppFrame>,
    flags: &mut ExchangeFlags,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::PollDocumentArchiveLoad { seq, operation } = command else { unreachable!("arm_poll_document_archive_load is dispatched only for its own command") };
    let status = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        drive_self_waking_ready(instance.app.poll_document_archive_load(operation, identity, cx))
    });
    match status.await {
        Ok(status) => {
            flags.mutated |= status.state == protocol::DocumentArchiveLoadState::Ready;
            frames.push(protocol::AppFrame::DocumentArchiveLoad { in_reply_to: seq, status });
        }
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🛑️ Cancels an archive load.
pub(super) async fn arm_cancel_document_archive_load<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::CancelDocumentArchiveLoad { seq, operation } = command else { unreachable!("arm_cancel_document_archive_load is dispatched only for its own command") };
    let cancelled = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.cancel_document_archive_load(operation)
    });
    match cancelled.await {
        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// ✅️ Acknowledges a finished archive load.
pub(super) async fn arm_acknowledge_document_archive_load<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation } = command else { unreachable!("arm_acknowledge_document_archive_load is dispatched only for its own command") };
    let acknowledged = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.acknowledge_document_archive_load(operation)
    });
    match acknowledged.await {
        Ok(()) => frames.push(protocol::AppFrame::Done { in_reply_to: seq }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🪪️ Reads the document identity.
pub(super) async fn arm_read_document_identity<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ReadDocumentIdentity { seq } = command else { unreachable!("arm_read_document_identity is dispatched only for its own command") };
    let read = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(protocol::AppDocumentIdentity { app_instance_id: instance_id, parent_document_id: instance.app.document_identity() })
    });
    match read.await {
        Ok(identity) => frames.push(protocol::AppFrame::DocumentIdentity { in_reply_to: seq, identity }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🗄️ Reads the document archive.
pub(super) async fn arm_read_document_archive<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ReadDocumentArchive { seq } = command else { unreachable!("arm_read_document_archive is dispatched only for its own command") };
    let read = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.document_archive())
    });
    match read.await {
        Ok(archive) => frames.push(protocol::AppFrame::DocumentArchive { in_reply_to: seq, archive }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}
