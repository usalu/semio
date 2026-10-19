//! 🏁️ Exchange finish: the effect pull, the typed operation drain, the ephemeral snapshot and the presence outcome that seal one plugin exchange. See [`super`].

use super::*;

/// 🏁️ Pulls the pending effects of a mutating exchange, drives the typed operation drain, folds the ephemeral snapshot and the presence outcome into the reply frames and seals the exchange output.
#[allow(clippy::too_many_arguments, reason = "one exchange context plus the reply state the finish seals")]
pub(super) async fn finish_exchange<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx: &mut semio_framework_job::StepContext<'_>,
    mut frames: Vec<protocol::AppFrame>,
    mut effect_bytes: Vec<Vec<u8>>,
    mut event_bytes: Vec<Vec<u8>>,
    retry_command: Option<(u64, PluginCommandIngress)>,
    command_terminal_fault: Option<Vec<u8>>,
    presence_pending: Option<u64>,
    presence_terminal: Option<u64>,
    presence_terminal_fault: Option<Vec<u8>>,
    flags: ExchangeFlags,
) -> Result<PluginExchangeOutput, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    if flags.mutated {
        let effects = with_instances_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            let view = instance.surface_contexts.view().cloned();
            ::semio_framework_async::poll::resolve_ready(instance.app.pending_effects(view.as_ref(), cx))
        })
        .await
        ?;
        for effect in effects.iter() {
            effect_bytes.push(encode_wire_serialized(effect));
        }
    }
    let (mut typed_output, mut typed_leftover) = with_instances_original_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let cell=instance.cell;let mut pages=match cell.result_pages.try_lock(){Ok(pages)=>pages,Err(std::sync::TryLockError::WouldBlock)=>return Ok((PluginExchangeOutput::default(),None)),Err(std::sync::TryLockError::Poisoned(_))=>return Err(plugin_internal_fault("original result receiving authority is poisoned"))};
        advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, &mut pages, identity,cx)
    })
    .await?;
    if let Some(operation) = flags.started_typed_operation {
        let mut pumps = 0;
        while pumps < TYPED_OPERATION_REPLY_LEFTOVER_PUMPS && typed_leftover.as_ref().is_none_or(|leftover| leftover.operation != operation) && typed_output.typed_operation_results.is_empty() && !typed_output.ownership_receipt_pending() {
            let (more, leftover) = with_instances_original_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                if !instance.app.has_runnable_typed_operations() {
                    return Ok((None, None));
                }
                let cell=instance.cell;let mut pages=match cell.result_pages.try_lock(){Ok(pages)=>pages,Err(std::sync::TryLockError::WouldBlock)=>return Ok((None,None)),Err(std::sync::TryLockError::Poisoned(_))=>return Err(plugin_internal_fault("original result receiving authority is poisoned"))};
                advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, &mut pages, identity,cx).map(|(output, leftover)| (Some(output), leftover))
            })
            .await?;
            let Some(more) = more else { break };
            typed_output.original_receiving=more.original_receiving;
            typed_output.constructor_receipt=more.constructor_receipt;
            typed_output.preparation_receipt=more.preparation_receipt;
            typed_output.retirement_receipt=more.retirement_receipt;
            typed_output.history_command_receipt=more.history_command_receipt;
            typed_output.tool_run_receipt=more.tool_run_receipt;
            typed_output.frames.extend(more.frames);
            typed_output.effects.extend(more.effects);
            typed_output.events.extend(more.events);
            typed_output.typed_operation_pages+=more.typed_operation_pages;
            if leftover.is_some() {
                typed_leftover = leftover;
            }
            pumps += 1;
        }
    }
    effect_bytes.extend(typed_output.effects);
    event_bytes.extend(typed_output.events);
    if let Some(leftover) = typed_leftover.as_ref() {
        let merged = merge_typed_operation_leftover_into_reply(&mut frames, leftover);
        debug_runtime_line(format_args!("[TRACE] typed-operation inline leftover operation={} merged-into-reply={merged}", leftover.operation));
    }
    let ephemeral = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(::semio_framework_async::poll::resolve_ready(instance.app.ephemeral_snapshot()))
    });
    if let Ok(EphemeralSnapshot { presence, presence_generation, transient_generation, interaction, tool_run, history_edit }) = ephemeral.await {
        let tool_run = tool_run.map_or_else(Vec::new, |tool_run| {
            let mut bytes = Vec::new();
            protocol::encode_presence_tool_run(&tool_run, &mut bytes);
            bytes
        });
        let history_edit = history_edit.map_or_else(Vec::new, |history_edit| {
            let mut bytes = Vec::new();
            protocol::encode_presence_history_edit(&history_edit, &mut bytes);
            bytes
        });
        frames.push(protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit });
    }
    if let Some(outcome) = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        Ok(instance.app.take_presence_outcome())
    })
    .await?
    {
        if let Some(fault) = outcome.fault {
            push_app_fault(&mut frames, Some(outcome.seq), fault).await;
        }
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    frame_bytes.extend(typed_output.frames);
    Ok(PluginExchangeOutput {
        original_receiving: typed_output.original_receiving,
        constructor_receipt: typed_output.constructor_receipt,
        preparation_receipt: typed_output.preparation_receipt,
        retirement_receipt: typed_output.retirement_receipt,
        history_command_receipt: typed_output.history_command_receipt,
        tool_run_receipt: typed_output.tool_run_receipt,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command,
        command_terminal_fault,
        presence_pending,
        presence_terminal,
        presence_terminal_fault,
        typed_operation_pages: typed_output.typed_operation_pages,
    })
}
