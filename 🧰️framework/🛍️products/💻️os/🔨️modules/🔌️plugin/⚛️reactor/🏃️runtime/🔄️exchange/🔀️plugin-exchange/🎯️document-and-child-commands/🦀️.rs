//! 🎯️ Document and child commands: local interaction queries, context menus, envelope ingestion, composed children and the history read of one plugin exchange. See [`super`].

use super::*;

/// 🔎️ Reads, acknowledges or cancels a local interaction query and answers its receipt or rejection.
pub(super) async fn arm_local_interaction_query<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::LocalInteractionQuery { seq, command } = command else { unreachable!("arm_local_interaction_query is dispatched only for its own command") };
    frames.try_reserve_exact(2).map_err(|_| plugin_internal_fault("local interaction receipt and rejection slots could not be admitted"))?;
    let reply = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        Ok(match command {
            protocol::LocalInteractionQueryCommand::Read { request_id } => match runtime.local_interaction_query_generation.next() {
                Some(generation) => instance.app.begin_local_interaction_query(request_id, generation),
                None => Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::GenerationExhausted }),
            },
            protocol::LocalInteractionQueryCommand::Acknowledge { token } => {
                instance.app.acknowledge_local_interaction_query(&token);
                None
            }
            protocol::LocalInteractionQueryCommand::Cancel { token } => {
                instance.app.cancel_local_interaction_query(&token);
                None
            }
        })
    })
    .await?;
    if let Some(reply) = reply {
        frames.push(protocol::AppFrame::LocalInteractionQuery { reply });
    }
    frames.push(protocol::AppFrame::Done { in_reply_to: seq });
    Ok(())
}

/// 📋️ Decodes a context menu request and answers its items.
pub(super) async fn arm_context_menu<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ContextMenu { seq, request } = command else { unreachable!("arm_context_menu is dispatched only for its own command") };
    match decode_wire_serialized::<ContextMenuWireRequest>(&request).await.and_then(ContextMenuWireRequest::into_parts) {
        Ok((request, view_state)) => {
            let outcome = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                Ok(::semio_framework_async::poll::resolve_ready(instance.app.context_menu(&request, &view_state)))
            });
            match outcome.await {
                Ok(items) => frames.push(protocol::AppFrame::ContextMenu { in_reply_to: seq, items: encode_wire_serialized(&items) }),
                Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
            }
        }
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 📨️ Ingests remote envelopes, reports the merges and publishes the open conflicts.
pub(super) async fn arm_apply_envelopes<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx: &mut semio_framework_job::StepContext<'_>,
    frames: &mut Vec<protocol::AppFrame>,
    flags: &mut ExchangeFlags,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ApplyEnvelopes { seq, envelopes } = command else { unreachable!("arm_apply_envelopes is dispatched only for its own command") };
    let encoded = protocol::encode_envelopes(&envelopes);
    match plugin_ingest_operations(runtime, instance_id, &encoded, identity, cx).await {
        Ok(reports) => {
            flags.mutated = true;
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
            for report in &reports {
                frames.push(protocol::AppFrame::MergeReport { in_reply_to: None, report: encode_wire_serialized(report) });
            }
            if !reports.is_empty() {
                let open: Vec<protocol::Conflict> = with_instances_mut(runtime, |list| {
                    let instance = find_instance(list, instance_id)?;
                    Ok(::semio_framework_async::poll::resolve_ready(instance.app.open_conflicts()))
                })
                .await?;
                frames.push(protocol::AppFrame::Conflicts { in_reply_to: None, conflicts: encode_wire_serialized(&open) });
            }
        }
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🧸️ Loads the composed children of a parent document.
pub(super) async fn arm_load_children<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    frames: &mut Vec<protocol::AppFrame>,
    flags: &mut ExchangeFlags,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::LoadChildren { seq, entries } = command else { unreachable!("arm_load_children is dispatched only for its own command") };
    let loaded = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        for entry in &entries {
            if !entry.owner.is_empty() {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), format!("child pack {}/{} belongs to member {}: a member's own children load with the document archive", entry.slot, entry.child_id, entry.owner)));
            }
            let dialect = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&entry.dialect).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), error))?;
            ::semio_framework_async::poll::resolve_ready(instance.app.load_child_pack(&entry.slot, &entry.child_id, dialect, &entry.envelope_pack, identity))?;
        }
        Ok(())
    });
    match loaded.await {
        Ok(()) => {
            flags.mutated = true;
            frames.push(protocol::AppFrame::Done { in_reply_to: seq });
        }
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 👶️ Reads the composed children of a parent document.
pub(super) async fn arm_read_children<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ReadChildren { seq } = command else { unreachable!("arm_read_children is dispatched only for its own command") };
    let read = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.child_packs())
    });
    match read.await {
        Ok(entries) => frames.push(protocol::AppFrame::Children { in_reply_to: seq, entries }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 🧬️ Reads the head of every composed child.
pub(super) async fn arm_read_child_heads<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ReadChildHeads { seq } = command else { unreachable!("arm_read_child_heads is dispatched only for its own command") };
    let read = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.child_head_packs())
    });
    match read.await {
        Ok(entries) => frames.push(protocol::AppFrame::ChildHeads { in_reply_to: seq, entries }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}

/// 📜️ Reads the retained history of the document.
pub(super) async fn arm_read_history<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    frames: &mut Vec<protocol::AppFrame>,
    command: protocol::AppCommand,
) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let protocol::AppCommand::ReadHistory { seq } = command else { unreachable!("arm_read_history is dispatched only for its own command") };
    let snapshot = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.history_snapshot())
    });
    match snapshot.await {
        Ok(history_patch) => frames.push(protocol::AppFrame::HistorySnapshot { in_reply_to: seq, history_patch: encode_wire_serialized(&history_patch) }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
    Ok(())
}
