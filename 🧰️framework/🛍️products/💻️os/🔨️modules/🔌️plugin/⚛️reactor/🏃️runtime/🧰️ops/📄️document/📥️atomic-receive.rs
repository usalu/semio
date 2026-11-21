//! 🌀️ Schedules paid atomic document work under the same live actor binding as ordinary ingress.

use super::*;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetirementDemand, RetirementPollExit, RetainedCloneProgress, RetainedCloneStep, retained_clone::retained_clone_grant_admits};
use std::mem::{size_of, size_of_val};
const ATOMIC_DOCUMENT_EFFECTS_PER_TURN: usize = 64;

pub fn prepare_atomic_document_turn<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, event: Option<&semio_framework::kernel::Event>, cx: &mut StepContext<'_>,
) -> Result<(bool, bool), OriginalTurnFailure> {
    use semio_framework::kernel::Event;
    let offer_outbound = !matches!(event, Some(Event::Message { source: MessageEndpoint::Shell { .. }, .. }));
    let prior_port_retired = match event {
        Some(Event::InstanceClose(request)) => {
            {
                let lifetimes = runtime.guest_lifetimes.try_borrow().map_err(|_| plugin_internal_fault("lifecycle authority busy"))?;
                let slot = lifetimes.get(request.lifetime.instance_id).ok_or_else(|| plugin_internal_fault("close lifetime absent"))?;
                slot.cell.validate_close(*request).map_err(plugin_internal_fault)?;
            }
            retire_atomic_document_backbone(runtime, request.lifetime.instance_id, cx)?
        }
        Some(Event::Message { source: MessageEndpoint::Shell { instance }, payload }) =>
            prepare_atomic_document_detach(runtime, &instance.0, payload, cx)?,
        _ => true,
    };
    Ok((prior_port_retired, offer_outbound))
}

pub fn retains_atomic_document_backbone<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, uri: &str, payload: Vec<u8>,
) -> Result<(), OriginalTurnFailure> {
    let (instance, generation, owner) = live_document_backbone_for_uri(runtime, uri)?
        .ok_or_else(|| plugin_internal_fault("original atomic frame has no live document binding"))?;
    if !runtime.guest_lifetimes.try_borrow().map_err(|_| plugin_internal_fault("lifecycle authority busy"))?
        .get(instance).is_some_and(|slot| slot.cell.is_live()) {
        return Err(plugin_internal_fault("original atomic backbone instance is closing").into());
    }
    let mut bindings = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
    let binding = bindings.get_mut(instance).ok_or_else(|| plugin_internal_fault("original atomic binding is retired"))?;
    if binding.generation != generation || binding.uri.as_deref() != Some(uri) ||
        !binding.channel.as_ref().is_some_and(|current| current.admits_same_channel(&owner)) ||
        binding.atomic.input_pending() {
        return Err(plugin_internal_fault("original atomic document port still owns a prior delivery").into());
    }
    binding.atomic.original = Some(payload);
    Ok(())
}

pub async fn advance_atomic_document_backbone<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, offer_outbound: bool, cx: &mut StepContext<'_>,
) -> Result<(bool, Option<DocumentBackboneTurnOutputV1>), OriginalTurnFailure> {
    let selected = with_instances_mut(runtime, |list| {
        let mut bindings = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
        let start = runtime.document_backbone_turn_cursor.get();
        for offset in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS {
            let index = (start + offset) % PLUGIN_RUNTIME_INSTANCE_SLOTS;
            let Some((instance_id, binding)) = bindings.entry_at(index) else { continue; };
            if binding.channel.is_none() || binding.uri.is_none() { continue; }
            let mut instance = find_instance(list, instance_id)?;
            let inbound = binding.atomic.input_pending();
            let outbound = binding.atomic.outbound.is_some() || instance.app.original_atomic_outbound_pending();
            if !inbound && !outbound { continue; }
            let binding = bindings.get_mut(instance_id).ok_or_else(|| plugin_internal_fault("original atomic binding is retired"))?;
            let mut source = [0; 32];
            source[..8].copy_from_slice(&binding.generation.to_be_bytes());
            source[8..12].copy_from_slice(&instance_id.to_be_bytes());
            runtime.document_backbone_turn_cursor.set((index + 1) % PLUGIN_RUNTIME_INSTANCE_SLOTS);
            let outcome = if inbound {
                ::semio_framework_async::poll::resolve_ready(instance.app.advance_original_atomic_backbone(
                    &mut binding.atomic.inbox, &mut binding.atomic.original, source, cx,
                ))
            } else if offer_outbound {
                instance.app.advance_original_atomic_outbound(&mut binding.atomic.outbound, source, cx).map(|_| None)
            } else {
                Ok(None)
            };
            return Ok(Some((instance_id, outcome)));
        }
        Ok(None)
    }).await?;
    let Some((instance_id, reports)) = selected else { return Ok((false, None)); };
    let Some(report) = reports? else { return Ok((true, None)); };
    let mut frames = Vec::new();
    frames.push(protocol::encode_app_frame(&protocol::AppFrame::MergeReport { in_reply_to: None, report: encode_wire_serialized(&report) }).await);
    let conflicts = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(::semio_framework_async::poll::resolve_ready(instance.app.open_conflicts()))
    }).await?;
    frames.push(protocol::encode_app_frame(&protocol::AppFrame::Conflicts { in_reply_to: None, conflicts: encode_wire_serialized(&conflicts) }).await);
    Ok((true, Some(DocumentBackboneTurnOutputV1 { instance_id, document_changed: true, frames, effects: Vec::new() })))
}

pub fn retire_atomic_document_backbone<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, instance: u32, cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let mut bindings = runtime.document_backbones.try_borrow_mut().map_err(|_| plugin_internal_fault("document backbone binding authority is busy"))?;
    let Some(binding) = bindings.get_mut(instance) else { return Ok(true); };
    if binding.atomic.terminal() { return Ok(true); }
    if binding.atomic.inbox.ready().is_some() { return Ok(false); }
    cx.with_original_retirement(|cx| {
        for _ in 0..ATOMIC_DOCUMENT_EFFECTS_PER_TURN {
            if binding.atomic.terminal() || cx.retirement_should_yield() { break; }
            let grant = cx.retained_grant();
            let Some((poll, performed)) = cx.with_original_effect(grant, |_, effect| {
                if let Some(original) = binding.atomic.original.as_ref() {
                    let demand = RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>(), release_bytes: original.capacity(), depth: 1, ..Default::default() };
                    let result = (|| {
                        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                        binding.atomic.original = None;
                        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }))
                    })();
                    return RetirementPollExit::settle(effect, result);
                }
                if let Some(delivery) = binding.atomic.outbound.as_mut() {
                    let result = if delivery.terminal_is_empty() {
                        let demand = RetirementDemand { copy_bytes: size_of_val(&binding.atomic.outbound), depth: 1, ..Default::default() };
                        (|| {
                            if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(RetainedCloneStep::Progress(Default::default())); }
                            binding.atomic.outbound = None;
                            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
                        })()
                    } else {
                        delivery.close(effect).map(RetainedCloneStep::Progress).map_err(Into::into)
                    };
                    return RetirementPollExit::settle(effect, result);
                }
                binding.atomic.inbox.close(effect)
            })? else { break; };
            cx.consume_work(performed);
            let receipt = match RetirementPollExit::resume(poll) {
                Ok(step) => step.progress(),
                Err(RetirementPollExit::Pending(_)) => Default::default(),
                Err(RetirementPollExit::Failed(failure)) => return Err(failure.into_value_error().into()),
            };
            if receipt == Default::default() { break; }
            cx.consume_retained(receipt)?;
        }
        Ok(binding.atomic.terminal())
    })
}
