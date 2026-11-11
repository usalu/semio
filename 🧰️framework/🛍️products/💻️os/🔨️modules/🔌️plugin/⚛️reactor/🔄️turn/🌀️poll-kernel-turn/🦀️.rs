//! 🌀️ Poll kernel turn of the reactor turn: the one reactor turn that polls the kernel, routes events, drives retirement, reconciliation and publication and fills the turn patch. See [`super`].

use super::*;

/// 🌀️ Runs one reactor turn that polls the kernel: admits the retained input, drives the retirement ladders, serves the one selected event, ingresses commands and cold pairs, renders dirty surfaces, reconciles, runs the executors, settles the more-work verdict and publishes the turn result. Each phase is a function of its own child module.
///
/// 👥️ M2: once per poll — expire ages-out peer marks, then flush drains every key touched since
/// the last flush into one coalesced `PresenceUpdate` each (free burst coalescing: a hover storm
/// between polls still costs exactly one update per `(surface, node_key)`).
#[expect(clippy::result_large_err, reason = "Patch reservation and publication callbacks return the original fixed surface or patch owner on refusal; these transfers must not allocate an error wrapper.")]
pub(super) async fn poll_kernel_turn<PA: semio_plugin_reactor_app::PluginApp, T, Prepared>(
    runtime: &semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    events: &mut Option<Event>,
    command_page: &mut Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<&semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
    identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx:&mut semio_framework_job::StepContext<'_>,
) -> Result<T, semio_framework_job::OriginalTurnFailure> {
    admit_turn_entry(budget, cx)?;
    let original_progress=cx.retained_progress();let original_work=cx.original_work_units_spent();
    let reserved_progress=cx.reserved_return_progress();let reserved_work=cx.reserved_return_work();
    let retryable_lifecycle = command_page.is_none() && cold_pair_page.is_none() && events.iter().all(|event| matches!(event, Event::InstanceOpen { .. } | Event::InstanceClose(_) | Event::InstanceLifecycleAck(_)));
    let mut dirty = DirtyPollOwners::new();
    let focus = admit_lifecycle_focus(runtime, events)?;
    let close_instances: Vec<u32> = events.iter().filter_map(|event| if let Event::InstanceClose(request) = event { Some(request.lifetime.instance_id) } else { None }).collect();
    let mut effects: Vec<Effect> = Vec::new();
    trace_turn_phase_retention("lifecycle");
    let close_cleanup_work = drive_turn_retirement(runtime, cx)?;
    semio_plugin_runtime_instances::inst_opening::plugin_resume_actor_openings(runtime,identity,cx).await?;
    PATCHES.with(|patches| redirty_acknowledged_deferred_surfaces(patches, &mut dirty,cx))?;
    redirty_arena_starved_surfaces(&mut dirty)?;
    let selected_event = select_turn_event(runtime, events, cx)?;
    dispatch_turn_event(runtime, selected_event, &mut dirty, &mut effects, identity, cx).await?;
    let extension_retirement_work = drive_extension_turn(runtime, &mut effects, cx).await?;
    let close_cleanup_work = close_cleanup_work || extension_retirement_work;
    let cold_pair_ingress = drive_cold_pair_ingress(runtime, cold_pair_page, identity, cx).await;
    let cold_pair_loading = matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_));
    let document_load_work = step_restored_document_loads(runtime, &mut effects, identity, cx).await || COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow().is_some());
    let command_ingress = drive_command_ingress(runtime, command_page, close_instances, &mut effects, identity, cx).await?;
    advance_original_receiving(runtime, &mut dirty, &mut effects, identity, cx).await?;
    step_pending_patch_close(cx)?;

    trace_turn_phase_retention("ingress");
    let (continuation, typed_operation_scan) = semio_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(runtime, semio_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::turn(budget,cx), identity,cx).await?;
    let typed_operation_contended = typed_operation_scan.contended;
    if let Some((instance, output)) = continuation {
        route_exchange_output(instance, output, &mut effects,cx)?;
    }

    trace_turn_phase_retention("continuation");
    let result_page_work=semio_plugin_reactor_lifecycle::result_publication::plugin_drive_result_pages(runtime,&mut effects,budget.max_effects as usize,cx)?;
    let pending_effects_runnable = semio_plugin_runtime_ops::ops_typed_operations::plugin_drive_pending_effects(runtime, &mut effects, cx)?;
    let displaced_surface_work=semio_plugin_runtime_ops::ops_surface_render::plugin_drive_displaced_surfaces(runtime,cx)?;
    let now_ms = u64::try_from(semio_plugin_actor_host::now_ms().await).unwrap_or(0);

    trace_turn_phase_retention("presence-clock");
    render_dirty_surfaces(runtime, dirty, now_ms, &mut effects, cx).await?;
    let reconcile_work = drive_turn_reconcile(&mut effects, cx)?;
    let executors = drive_turn_executors(&mut effects);
    let resumes_remain = drain_task_resumes(runtime, &mut effects, 64, identity, cx);
    effects.extend(semio_plugin_runtime_ops::ops_document::plugin_drain_document_backbones(runtime)?);
    let more_work = settle_turn_work(runtime, TurnWorkInputs { events_pending: events.is_some(), executors, close_cleanup_work, typed_operation_runnable: typed_operation_scan.runnable || result_page_work || displaced_surface_work, typed_operation_contended, pending_effects_runnable, reconcile_work, resumes_remain, document_load_work }, &effects)?;

    let lifecycle_receipt = focus.map(|instance| runtime.guest_lifetimes.borrow_mut().prepare_turn(instance, &ReactorClosePeers)).transpose()?.flatten();
    let (ui_patches, ui_patch_receipt) = if cold_pair_loading { (semio_framework::kernel::UiTurnPatches::default(), None) } else { fill_turn_patch_page(runtime, turn_patch_budget_bytes(budget))? };
    let presence = PRESENCE.with(|hub| {
        let mut hub = hub.borrow_mut();
        hub.expire(now_ms);
        hub.flush()
    });
    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };

    let fuel_used=u64::try_from(cx.original_work_units_spent().checked_sub(original_work).ok_or_else(||reactor_close_fault("original kernel work observer moved backward"))?).map_err(|_|reactor_close_fault("original turn work exceeds fixed receipt extent"))?;
    let reserved_work=cx.reserved_return_work().checked_sub(reserved_work).ok_or_else(||reactor_close_fault("reserved kernel work observer moved backward"))?;
    let retained_receipt=kernel_work_receipt(budget.retained,original_turn_progress(original_progress,cx.retained_progress())?,fuel_used,original_turn_progress(reserved_progress,cx.reserved_return_progress())?,reserved_work)?;
    let result = semio_framework::kernel::TurnResult { retained_receipt, ui_patches, effects, presence, next_wake: ARMED_TIMERS.with(|timers| timers.borrow().first()), status, fuel_used, command_ingress, cold_pair_ingress, lifecycle_receipt, ui_patch_receipt };
    publish_turn_result(runtime, result, focus, retryable_lifecycle, prepare, publish)
}

/// 🧮️ Keeps the nested input receipt fresh-conserved; fixed Core receipts carry the authenticated aggregate.
fn kernel_work_receipt(input:semio_framework::kernel::RetainedTurnInput,aggregate:semio_framework_value::RetainedCloneProgress,work:u64,reserved:semio_framework_value::RetainedCloneProgress,reserved_work:u64)->Result<semio_framework::kernel::RetainedTurnReceipt,semio_framework_job::OriginalTurnFailure>{
    let fresh=original_turn_progress(reserved,aggregate)?;
    let work=work.checked_sub(reserved_work).ok_or_else(||reactor_close_fault("reserved kernel work exceeds its observed aggregate"))?;
    Ok(input.return_original(fresh,work)?)
}

#[cfg(test)]
mod receipt_tests{
 use super::*;
 #[test]
 fn kernel_receipt_keeps_fresh_conservation_and_reserved_aggregate_work(){
  let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
  for case in corpus["cases"].as_array().unwrap(){
   let input=serde_json::from_value(serde_json::json!({"operation":1,"generation":1,"epoch":1,"work":{"clockBinding":1,"absoluteDeadlineUs":1,"fuel":case["fuel"]},"grant":{"maximumItems":case["grant"],"maximumCopyBytes":0,"maximumCapacityBytes":0,"maximumReleaseBytes":0,"maximumDepth":1}})).unwrap();
   let aggregate=semio_framework_value::RetainedCloneProgress{copied_items:case["aggregate"].as_u64().unwrap()as usize,..Default::default()};
   let reserved=semio_framework_value::RetainedCloneProgress{copied_items:case["reserved"].as_u64().unwrap()as usize,..Default::default()};
   let result=kernel_work_receipt(input,aggregate,case["work"].as_u64().unwrap(),reserved,case["reservedWork"].as_u64().unwrap());
   assert_eq!(result.is_ok(),case["accepted"].as_bool().unwrap());
   if let Ok(receipt)=result{receipt.validate_for(input).unwrap();assert_eq!(receipt.spent.copied_items,case["fresh"].as_u64().unwrap()as usize);assert_eq!(receipt.work_units_spent,case["freshWork"].as_u64().unwrap());}
  }
 }
}
