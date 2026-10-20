/** 🟢️ The part4 phase of interactivityLiveReconcileFailures. */
import type { buildReconcileState } from "../🔶️reconcile/🟦️.ts";

/** 🔵 Runs the effects and registrations of interactivityLiveReconcileFailures. */
export function buildPart4({ rendererRetainedStart, rendererRetained, schema, reactor, schemaSource, failures, shellDocumentStart, shellDocuments, shellRefresh, commandDocumentStart, commandExchangeStart, commandDocuments, commandExchange, rendererSequenceStart, rendererSequence, windowMeasureStart, reactorSource, windowMeasure, patchesSource, reconcileSource }: Pick<ReturnType<typeof buildReconcileState>, "rendererRetainedStart" | "rendererRetained" | "schema" | "reactor" | "failures" | "shellDocumentStart" | "shellDocuments" | "shellRefresh" | "commandDocumentStart" | "commandExchangeStart" | "commandDocuments" | "commandExchange" | "rendererSequenceStart" | "rendererSequence" | "windowMeasureStart" | "windowMeasure"> & { readonly schemaSource: string; readonly reactorSource: string; readonly patchesSource: string; readonly reconcileSource: string }) {
  if (
    rendererRetainedStart < 0 ||
    !rendererRetained.includes("struct RetainedSurfaceRegistry") ||
    !rendererRetained.includes("slots: [Option<RetainedSurfaceSlot>; RETAINED_SURFACE_CAPACITY]") ||
    !rendererRetained.includes("generation: u64") ||
    !rendererRetained.includes("surface: SurfaceId") ||
    !rendererRetained.includes("struct PendingSurfaceRejectionRegistry") ||
    !rendererRetained.includes("slots: [Option<PendingSurfaceRejection>; RETAINED_SURFACE_CAPACITY]") ||
    !rendererRetained.includes("close_cursor: usize") ||
    !rendererRetained.includes("terminal_is_empty") ||
    rendererRetained.includes("retained: HashMap") ||
    rendererRetained.includes("pending_rejections: HashMap") ||
    !rendererRetained.includes("if closing.close_step() && closing.terminal_is_empty()") ||
    !rendererRetained.includes("if builder.close_step() && builder.terminal_is_empty()") ||
    !rendererRetained.includes("exchange_closing: Option<UiDocumentLease>") ||
    !schema.includes("pub enum UiTurnPatchTransfer<T>") ||
    !schema.includes("pub fn try_transfer_one<T>(&mut self, transfer: impl FnOnce(UiPatch) -> Result<T, UiPatch>)") ||
    !reactor.includes("result.ui_patches.try_transfer_one(|patch| self.apply_ui_patch(instance, patch))") ||
    !reactor.includes("fn apply_ui_patch(&mut self, instance: u32, patch: KernelUiPatch) -> Result<(), KernelUiPatch>") ||
    !schemaSource.includes("refused_turn_patch_transfer_restores_the_exact_retirement_owner")
  ) failures.push("P5b renderer retained/rejection authority or document closer is dynamic, unqualified, or erased before terminality");
  if (
    shellDocumentStart < 0 ||
    !shellDocuments.includes("SHELL_DOCUMENT_RETIREMENT_CAPACITY: usize = UI_DOCUMENT_LEASE_SLOTS * UI_DOCUMENT_LEASE_ALIASES as usize") ||
    !shellDocuments.includes("slots: [Option<ShellDocumentRetirementSlot>; SHELL_DOCUMENT_RETIREMENT_CAPACITY]") ||
    !shellDocuments.includes("epochs: [u64; SHELL_DOCUMENT_RETIREMENT_CAPACITY]") ||
    !shellDocuments.includes("cursor: usize") ||
    !shellDocuments.includes("fn try_admit(&mut self, document: UiDocumentLease) -> Result<(), UiDocumentLease>") ||
    !shellDocuments.includes("let terminal = slot.document.close_step() && slot.document.terminal_is_empty()") ||
    !shellDocuments.includes("terminal.document.terminal_is_empty()") ||
    !shellRefresh.includes("Self::retain_document_map_for_close(&mut self.closing_documents, std::mem::take(&mut self.window_ui))") ||
    !shellRefresh.includes("self.window_ui = documents") ||
    !shellRefresh.includes("self.panel_documents = documents") ||
    !shellRefresh.includes("self.spawned_ui = Some(document)") ||
    !shellRefresh.includes("return Err(refused)") ||
    !shellRefresh.includes("self.closing_documents.close_one()") ||
    shellRefresh.includes("rejected.close_step()") ||
    shellRefresh.includes("closing_documents.swap_remove(0)")
  ) failures.push("P5b Shell document replacement does not retain qualified exact owners through terminal close and saturation refusal");
  if (
    commandDocumentStart < 0 ||
    commandExchangeStart < 0 ||
    !commandDocuments.includes("COMMAND_DOCUMENT_RETIREMENT_CAPACITY: usize = UI_DOCUMENT_LEASE_SLOTS * ui_contract::UI_DOCUMENT_LEASE_ALIASES as usize") ||
    !commandDocuments.includes("slots: [Option<CommandDocumentRetirementState>; COMMAND_DOCUMENT_RETIREMENT_CAPACITY]") ||
    !commandDocuments.includes("Reserved {") ||
    !commandDocuments.includes("Pending {") ||
    !commandDocuments.includes("Closing {") ||
    !commandDocuments.includes("surface: SurfaceId") ||
    !commandDocuments.includes("document: UiDocumentLease") ||
    !commandDocuments.includes("fn try_reserve_page(&mut self, batch: u64, generation: u64)") ||
    !commandDocuments.includes("let terminal = document.close_step() && document.terminal_is_empty()") ||
    !commandDocuments.includes("CommandDocumentRetirementState::Closing") ||
    !commandExchange.includes("self.command_document_closes.try_reserve_page(key, generation)") ||
    !commandExchange.includes("let outcome = match self.run_turn(actor, instance, events).await") ||
    !commandExchange.includes("self.command_document_closes.admit(&mut destinations, surface)") ||
    !commandExchange.includes("self.command_document_closes.release(&mut destinations)") ||
    !commandExchange.includes("self.command_document_closes.begin_close_batch(key, generation)") ||
    !commandExchange.includes("self.command_document_closes.publish_batch(key, generation, &mut combined.surfaces)") ||
    commandExchange.indexOf("try_reserve_page(key, generation)") > commandExchange.indexOf("self.run_turn(actor, instance, events).await") ||
    commandExchange.includes("rejected.document.close_step()")
  ) failures.push("P5b command-batch aggregation does not pre-reserve qualified fixed document destinations and terminal-gate ninth-owner retirement");
  if (
    rendererSequenceStart < 0 ||
    !rendererSequence.includes("self.committed.checked_add(1).filter(|generation| *generation != 0)") ||
    !rendererSequence.includes("self.exhausted = generation == u64::MAX") ||
    !rendererSequence.includes("struct RendererSequenceReservation") ||
    !rendererSequence.includes("authority.rollback(self.generation)") ||
    rendererSequence.includes("SEQ.fetch_add") ||
    rendererSequence.includes("wrapping_add(1)")
  ) failures.push("P5b renderer sequence is not checked, nonzero, transactional, and permanently exhausted after one maximum");
  if (
    windowMeasureStart < 0 ||
    !reactor.includes("pub struct WindowMeasurePartition<'a>") ||
    !reactor.includes("UiFixedList<&'a WindowMeasure, WINDOW_MEASURE_PARTITION_CAPACITY>") ||
    !reactor.includes("Result<WindowMeasurePartition<'a>, &'a WindowMeasure>") ||
    !reactorSource.includes("partition_window_measures_max_plus_one_returns_exact_borrowed_owner") ||
    !windowMeasure.includes("WindowMeasureRenderFrame") ||
    !windowMeasure.includes("UiFixedList::<WindowMeasureRenderFrame<'_>, WINDOW_MEASURE_TRAVERSAL_CAPACITY>") ||
    !windowMeasure.includes("render_window_measure_select") ||
    !windowMeasure.includes("render_window_measure_slider") ||
    !windowMeasure.includes("render_window_measure_toggle") ||
    !windowMeasure.includes("window_measure_actions.try_upsert") ||
    windowMeasure.includes("self.render_window_measure(") ||
    windowMeasure.includes("on_change.clone()") ||
    windowMeasure.includes("items.iter().map(") ||
    windowMeasure.includes(".collect::<Vec")
  ) failures.push("P5b WindowMeasure rail is not fixed borrowed iterative traversal with fixed retained UI-native action bindings");
  for (const fixture of [
    "renderer_sequence_maximum_is_once_then_permanently_exhausted_and_rollback_is_transactional",
    "renderer_fixed_registries_return_exact_max_plus_one_surface_and_patch_owners",
    "shell_ninth_document_and_nonterminal_first_close_remain_in_qualified_retirement",
    "shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation",
    "command_batch_ninth_document_is_retained_after_nonterminal_close_and_exactly_returned_or_retired",
    "command_document_page_saturation_refuses_before_turn_owner_production",
    "stale_cancel_and_fault_document_builds_retain_closer_until_terminal",
    "document_builder_close_persists_until_terminal_after_one_step",
    "ordinary_lease_drop_releases_then_global_closer_reaches_terminal",
  ]) if (!reactorSource.includes(`fn ${fixture}`) && !schemaSource.includes(`fn ${fixture}`)) failures.push(`P5b integrated fixture missing ${fixture}`);
  for (const fixture of [
    "mounted_path_advances_one_reconcile_opportunity_per_grant",
    "cap_plus_one_returns_the_exact_tree_owner",
    "stale_generation_fault_is_publicly_retrievable",
    "resize_storm_coalesces_to_one_deferred_surface_owner",
    "effects_publish_in_admission_order_even_when_later_tree_finishes_first",
    "actor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot",
    "mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner",
    "close_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish",
    "terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed",
    "terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion",
    "terminal_full_plus_matching_rejected_advances_capacity_before_conversion",
    "terminal_full_plus_matching_surface_advances_capacity_before_conversion",
    "generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation",
    "terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation",
  ]) if (!patchesSource.includes(`fn ${fixture}`)) failures.push(`live reconcile fixture missing ${fixture}`);
  for (const fixture of ["identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation", "stale_cancel_and_drop_handoff_preserve_public_terminal_ownership", "semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged", "semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion", "retained_map_page_advances_each_key_once_without_rewalking_prior_entries", "allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation", "persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement", "public_drop_handback_is_lossless_at_terminal_cap_and_plus_one"]) if (!reconcileSource.includes(`fn ${fixture}`)) failures.push(`live reconcile fixture missing ${fixture}`);
}
