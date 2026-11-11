//! 🏁️ Exchange finish: the effect pull, the typed operation drain, the ephemeral snapshot and the presence outcome that seal one plugin exchange. See [`super`].

use super::*;

/// 🧾️ Records one pull of the pending effects exactly once: `None` is the retained "not admitted yet" answer and changes nothing, and a re-entry after the effects were recorded never pushes them again.
pub(super) fn record_pending_effects(state: &mut semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState, pulled: Option<Vec<semio_framework::kernel::Effect>>) -> bool {
    if state.effects_done { return true }
    let Some(effects) = pulled else { return false };
    for effect in &effects { state.effects.push(encode_wire_serialized(effect)); }
    state.pending_effects = Some(effects);
    state.effects_done = true;
    true
}

/// 🏁️ Pulls the pending effects of a mutating exchange, drives the typed operation drain, folds the ephemeral snapshot and the presence outcome into the reply frames and seals the exchange output.
#[allow(clippy::too_many_arguments, reason = "one exchange context plus the reply state the finish seals")]
pub(super) async fn finish_exchange<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx: &mut semio_framework_job::StepContext<'_>,
    state:&mut semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState,
    command_terminal_fault: Option<Vec<u8>>,
    presence_pending: Option<u64>,
    presence_terminal: Option<u64>,
    presence_terminal_fault: Option<Vec<u8>>,
    flags: ExchangeFlags,
) -> Result<PluginExchangeOutput, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    if flags.mutated {state.mutated = true;}
    if state.mutated && !state.effects_done {
        let pulled = with_instances_original_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            let view = instance.surface_contexts.view().cloned();
            let pulled = ::semio_framework_async::poll::resolve_ready(instance.app.pending_effects(view.as_ref(), cx))?;
            if pulled.is_none() { instance.cell.pending_effects.retain() } else { instance.cell.pending_effects.publish() }
            Ok(pulled)
        })
        .await?;
        if !record_pending_effects(state, pulled) { return Ok(PluginExchangeOutput::default()) }
    }
    let (typed_output,typed_leftover)= with_instances_original_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let cell=instance.cell;let mut pages=match cell.result_pages.try_lock(){Ok(pages)=>pages,Err(std::sync::TryLockError::WouldBlock)=>return Ok((PluginExchangeOutput::default(),None)),Err(std::sync::TryLockError::Poisoned(_))=>return Err(plugin_internal_fault("original result receiving authority is poisoned").into())};
        advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, &mut pages, identity,cx)
    })
    .await?;
    state.typed_output=Some(typed_output);state.typed_leftover=typed_leftover;
    if let Some(operation) = flags.started_typed_operation {
        let mut pumps = 0;
        while pumps < TYPED_OPERATION_REPLY_LEFTOVER_PUMPS && state.typed_leftover.as_ref().is_none_or(|leftover| leftover.operation != operation) && state.typed_output.as_mut().expect("retained actual typed output").typed_operation_pages==0 && !state.typed_output.as_mut().expect("retained actual typed output").ownership_receipt_pending() {
            let (more, leftover) = with_instances_original_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                if !instance.app.has_runnable_typed_operations() {
                    return Ok((None, None));
                }
                let cell=instance.cell;let mut pages=match cell.result_pages.try_lock(){Ok(pages)=>pages,Err(std::sync::TryLockError::WouldBlock)=>return Ok((None,None)),Err(std::sync::TryLockError::Poisoned(_))=>return Err(plugin_internal_fault("original result receiving authority is poisoned").into())};
                advance_typed_operation_output_with_leftover(&mut instance.app, instance_id, &mut pages, identity,cx).map(|(output, leftover)| (Some(output), leftover))
            })
            .await?;
            let Some(more) = more else { break };
            let typed_output=state.typed_output.as_mut().expect("retained actual typed output");
            typed_output.original_receiving=more.original_receiving;
            typed_output.constructor_receipt=more.constructor_receipt;
            typed_output.preparation_receipt=more.preparation_receipt;
            typed_output.retirement_receipt=more.retirement_receipt;
            typed_output.history_command_receipt=more.history_command_receipt;
            typed_output.tool_run_receipt=more.tool_run_receipt;
            state.typed_output.as_mut().expect("retained actual typed output").frames.extend(more.frames);
            state.typed_output.as_mut().expect("retained actual typed output").effects.extend(more.effects);
            state.typed_output.as_mut().expect("retained actual typed output").events.extend(more.events);
            state.typed_output.as_mut().expect("retained actual typed output").typed_operation_pages+=more.typed_operation_pages;
            if leftover.is_some() {
                state.typed_leftover = leftover;
            }
            pumps += 1;
        }
    }
    state.effects.append(&mut state.typed_output.as_mut().expect("retained actual typed output").effects);
    state.events.append(&mut state.typed_output.as_mut().expect("retained actual typed output").events);
    if let Some(leftover) = state.typed_leftover.as_ref() {
        let merged = merge_typed_operation_leftover_into_reply(&mut state.frames, leftover);
        debug_runtime_line(format_args!("[TRACE] typed-operation inline leftover operation={} merged-into-reply={merged}", leftover.operation));
    }
    let ephemeral = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(::semio_framework_async::poll::resolve_ready(instance.app.ephemeral_snapshot()))
    });
    let EphemeralSnapshot { presence, presence_generation, transient_generation, interaction, tool_run, history_edit }=ephemeral.await?;
    {
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
        state.frames.push(protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit });
    }
    if let Some(outcome) = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        Ok(instance.app.take_presence_outcome())
    })
    .await?
    {
        if let Some(fault) = outcome.fault {
            push_app_fault(&mut state.frames, Some(outcome.seq), fault).await;
        }
    }
    for frame in state.frames.iter(){state.frame_bytes.push(protocol::encode_app_frame(frame).await);}
    state.frame_bytes.append(&mut state.typed_output.as_mut().expect("retained actual typed output").frames);
    let typed_output=state.typed_output.take().expect("all fallible finish steps kept their actual output");
    Ok(PluginExchangeOutput {
        original_receiving: typed_output.original_receiving,
        constructor_receipt: typed_output.constructor_receipt,
        preparation_receipt: typed_output.preparation_receipt,
        retirement_receipt: typed_output.retirement_receipt,
        history_command_receipt: typed_output.history_command_receipt,
        tool_run_receipt: typed_output.tool_run_receipt,
        frames:std::mem::take(&mut state.frame_bytes),
        effects:std::mem::take(&mut state.effects),
        events:std::mem::take(&mut state.events),
        command_terminal_fault,
        presence_pending,
        presence_terminal,
        presence_terminal_fault,
        typed_operation_pages: typed_output.typed_operation_pages,
    })
}

#[cfg(test)]
mod reentry_laws {
    use super::*;
    use semio_framework_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState;

    fn notify(message: &str) -> semio_framework::kernel::Effect { semio_framework::kernel::Effect::Notify { message: message.to_string() } }

    /// 🔁️ Re-entering the finish after a `None` pull neither records nor duplicates anything, and re-entering after the recorded pull never pushes its effects twice.
    #[test]
    fn re_entry_after_a_none_pull_records_the_effects_exactly_once() {
        let mut state = OriginalCommandExchangeState::empty();
        assert!(!record_pending_effects(&mut state, None));
        assert!(state.effects.is_empty() && state.pending_effects.is_none() && !state.effects_done);
        assert!(!record_pending_effects(&mut state, None));
        assert!(state.effects.is_empty());
        assert!(record_pending_effects(&mut state, Some(vec![notify("a"), notify("b")])));
        assert_eq!(state.effects.len(), 2);
        assert!(state.effects_done && state.pending_effects.as_ref().is_some_and(|effects| effects.len() == 2));
        assert!(record_pending_effects(&mut state, Some(vec![notify("c")])));
        assert!(record_pending_effects(&mut state, None));
        assert_eq!(state.effects.len(), 2);
        assert_eq!(state.pending_effects.as_ref().map(Vec::len), Some(2));
        state.effects.clear();
        state.pending_effects = None;
    }
}
