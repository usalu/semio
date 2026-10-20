/** 🟠️ The reserve phase of interactivityLiveReconcileFailures. */
import type { buildReconcileState } from "../🔶️reconcile/🟦️.ts";

/** 🟡 Builds reserveAt, cursorAt, reconcileJobImpl, reconcileTakeReady of interactivityLiveReconcileFailures. */
export function buildReserveState({ schemaSource, failures, valueSource, reconcile, patches, reactor, driveStart, drive, trackerDefinitions }: { readonly schemaSource: string; readonly valueSource: string } & Pick<ReturnType<typeof buildReconcileState>, "failures" | "reconcile" | "patches" | "reactor" | "driveStart" | "drive" | "trackerDefinitions">) {
  for (const fixture of [
    "document_lease_publishes_complete_generation_and_closes_incrementally",
    "document_builder_returns_exact_max_plus_one_node_owner",
    "retained_patch_applies_one_census_node_op_and_validation_owner_per_opportunity",
    "retained_patch_max_plus_one_returns_exact_state_and_patch",
    "retained_patch_cancel_stale_and_deadline_preserve_owner",
    "retained_patch_remove_advances_one_node_or_child_per_opportunity",
    "retained_patch_duplicate_and_deep_validation_are_cursorized",
    "patch_handback_arena_max_plus_one_refuses_without_reusing_a_live_generation",
    "abandoned_patch_owner_moves_to_incremental_handback_and_reopens_capacity",
    "ui_turn_patches_max_plus_one_returns_the_exact_patch_owner",
    "ui_turn_patches_fixed_serde_visitor_rejects_plus_one",
    "ui_turn_patches_close_retires_one_op_or_patch_owner_per_step",
    "ui_turn_patch_retirement_max_plus_one_refuses_before_owner_transfer",
    "ui_turn_patch_retirement_rejects_stale_epoch_release_and_closes_one_owner_per_step",
    "ui_turn_patch_transport_round_trip_is_single_claim_and_preserves_populated_owner",
    "ui_turn_patch_transport_rejects_truncated_stale_and_cancelled_tokens",
    "ui_turn_patch_transport_max_plus_one_returns_exact_owner_and_session_close_is_incremental",
  ]) if (!schemaSource.includes(`fn ${fixture}`)) failures.push(`fixed document lease fixture missing ${fixture}`);
  for (const fixture of [
    "fixed_page_max_plus_one_returns_the_exact_untransferred_owner",
    "ascending_map_duplicate_refusal_preserves_key_and_value",
    "credited_alias_keeps_pages_live_after_original_handle_is_lost",
    "poisoned_arena_lock_recovers_without_losing_fixed_authority",
    "arena_initialization_is_a_fixed_control_and_page_taxonomy",
  ]) if (!valueSource.includes(`fn ${fixture}`)) failures.push(`fixed UiValue arena fixture missing ${fixture}`);
  for (const cap of [
    "SURFACE_RECONCILE_ADMISSION_SLOTS: usize = 64",
    "SURFACE_RECONCILE_PAGE_BYTES: usize = 32 * 1_024",
    "max_nodes: SURFACE_RECONCILE_FIXED_NODES",
    "max_items: 4_097",
    "max_bytes: SURFACE_RECONCILE_SURFACE_BYTES",
    "max_identifier_bytes: 256",
    "SURFACE_RECONCILE_SURFACE_BYTES: usize = 8 * 1_024 * 1_024",
    "SURFACE_RECONCILE_AGGREGATE_BYTES: usize = SURFACE_RECONCILE_SURFACE_BYTES * 4",
    "SURFACE_RECONCILE_AGGREGATE_ITEMS: usize = 131_076",
  ]) if (!reconcile.includes(cap)) failures.push(`live reconcile fixed credit changed or disappeared: ${cap}`);
  const reserveAt = reconcile.indexOf("let credit = if surface_bytes <= limits.max_identifier_bytes { reserve_surface_reconcile(limits) }");
  const cursorAt = reconcile.indexOf("let cursor = SurfaceReconcileCursor::new_with_limits(tree, &current, limits)");
  if (reserveAt < 0 || cursorAt < 0 || reserveAt > cursorAt || !reconcile.includes("slots: [SurfaceReconcileAdmissionSlot; SURFACE_RECONCILE_ADMISSION_SLOTS]") || !reconcile.includes("next_items > SURFACE_RECONCILE_AGGREGATE_ITEMS") || !reconcile.includes("next_bytes > SURFACE_RECONCILE_AGGREGATE_BYTES")) failures.push("live reconcile does not reserve fixed item/byte/generation credit before by-value tree cursor construction");
  if (reconcile.includes("pub fn reconcile(&mut self") || patches.includes(".reconcile(") || patches.includes("snapshot().revision") || patches.includes("tree.clone()") || reactor.includes("patches.diff(")) failures.push("mounted reactor reconcile reintroduced a synchronous, snapshot-clone, or cloned-tree compatibility path");
  if (driveStart < 0 || (drive.match(/cursor\.step\(current\)/g) ?? []).length !== 1 || drive.includes("while ") || drive.includes("for ") || !drive.includes("cx.generation().0 != state.generation") || !drive.includes("cx.is_cancelled()") || !drive.includes("cx.should_yield()") || !drive.includes("cx.consume_fuel(1)") || drive.indexOf("cx.should_yield()") > drive.indexOf("cursor.step(current)")) failures.push("live reconcile grant does not advance exactly one fresh fuel/deadline/cancel/generation cursor opportunity");
  const reconcileJobImpl = reconcile.indexOf("impl SurfaceReconcileJob {");
  const reconcileTakeReady = reconcile.indexOf("pub fn take_ready", reconcileJobImpl);
  const takeReady = reconcile.slice(reconcileTakeReady, reconcile.indexOf("pub fn into_terminal", reconcileTakeReady));
  if (!reconcile.includes("RetireCursor") || !reconcile.includes("RetirePrevious") || !reconcile.includes("state.phase = SurfaceReconcileJobPhase::Ready") || !reconcile.includes("split_surface_reconcile(credit)") || !reconcile.includes("reconciler.persistent_credit = Some(candidate_credit)") || !reconcile.includes("credit: Some(patch_credit)") || takeReady.includes("release_surface_reconcile(credit)")) failures.push("live reconcile persistent credit does not split and transfer through candidate/publication owners after exact previous-owner retirement");
  const semanticAt = reconcile.indexOf("struct SurfaceSemanticCensusCursor");
  const cloneAt = reconcile.indexOf("self.seen.try_insert((parent, node.key.clone()))", semanticAt);
  const semanticRegion = reconcile.slice(semanticAt, reconcile.indexOf("pub(crate) struct SurfaceReconcileCursor", semanticAt));
  for (const cursor of ["field: u8", "container: u8", "entry: usize", "binding: usize", "action: u8", "data_attribute: u8", "string_byte: usize", "depth: usize", "value_stack: Box<[Option<SurfaceSemanticValueFrame>]>", "value_stack.resize_with(SURFACE_RECONCILE_VALUE_DEPTH, || None)", "value_stack: value_stack.into_boxed_slice()"]) if (!reconcile.includes(cursor)) failures.push(`live reconcile semantic census cursor missing ${cursor}`);
  if (semanticAt < 0 || cloneAt < 0 || semanticAt > cloneAt || reconcile.includes("tree_node_semantic_usage") || reconcile.includes("fn include_ui_value") || semanticRegion.includes("for value in values") || semanticRegion.includes("while let Some") || semanticRegion.includes("cursor.by_ref().collect") || !reconcile.includes("SURFACE_RECONCILE_VALUE_DEPTH: usize = 64") || !reconcile.includes("self.semantic_census.as_mut()") || !reconcile.includes("SurfaceSemanticCensusStep::Complete") || reconcile.includes("Vec::with_capacity(limits.max_nodes)") || reconcile.includes("Vec::with_capacity(limits.max_items)")) failures.push("live reconcile semantic census is recursive, whole-container, decorative, dynamically pre-grown, or occurs after candidate mutation");
  if (!reconcile.includes("struct SurfaceSemanticMapPage") || !reconcile.includes("cursor: ui_contract::UiMapCursor") || !semanticRegion.includes("page.cursor.advance()") || semanticRegion.includes("get_index(") || semanticRegion.includes("BTreeMap") || semanticRegion.includes("values.range(") || semanticRegion.includes("page.values.iter().nth(") || semanticRegion.includes("unsafe") || semanticRegion.includes("transmute") || semanticRegion.includes("*const") || semanticRegion.includes("*mut")) failures.push("live reconcile map census does not retain a sound credited page cursor or reintroduces borrowed/raw/restart traversal");
  for (const owner of [
    "traversal: SurfaceFixedVec<PresentationFrame",
    "flat: SurfaceFixedVec<FlatPresentedNode",
    "postorder: SurfaceFixedVec<usize",
    "seen: SurfaceLinearSet<",
    "ids: SurfaceFixedVec<ui_contract::UiNodeId",
    "new_retained: SurfaceLinearMap<",
    "new_key_index: SurfaceLinearMap<",
    "removal: SurfaceFixedVec<RemovalFrame",
    "    ops: ui_contract::UiPatchOps,\n    pending_op: Option<ui_contract::UiPatchOp>",
  ]) if (!reconcile.includes(owner)) failures.push(`live reconcile fixed owner disappeared: ${owner}`);
  if (reconcile.includes("fn admit_vec_backing") || reconcile.includes("fn admit_hash_map_backing") || reconcile.includes("fn admit_hash_set_backing") || !reconcile.includes("struct RecordDiffCursor") || !reconcile.includes("fn diff_record_field") || reconcile.includes("fn diff_record(") || !reconcile.includes("struct SurfaceTreeRetireCursor") || reconcile.includes("retire_forest: Vec")) failures.push("live reconcile cursor/diff/retire backing is eager, fictional, hash-layout-estimated, or dynamically unowned");
  if (!reconcile.includes("pub struct SurfaceReconcileReservation") || !patches.includes("reservation: SurfaceReconcileReservation") || !patches.includes("SurfaceReconcileReservation::try_new(generation)") || !patches.includes("SurfaceReconcileJob::try_new_reserved") || !reconcile.includes("persistent_credit: Option<SurfaceReconcileCredit>")) failures.push("live reconcile producer/current/candidate ownership does not retain exact pre-materialization and persistent aggregate credit");
  if (!reconcile.includes("pub struct SurfaceReconcileRejected") || !reconcile.includes("pub struct SurfaceReconcileTerminal") || !reconcile.includes("pub fn take_sources(&mut self)") || !reconcile.includes("pub fn resume(mut self") || !reconcile.includes("pub fn close_step(&mut self)") || !reconcile.includes("impl Drop for SurfaceReconcileJob") || !reconcile.includes("impl Drop for SurfaceReconcileRejected") || !reconcile.includes("impl Drop for SurfaceReconcileTerminal") || !reconcile.includes("pub fn take_surface_reconcile_terminal")) failures.push("live reconcile exact rejected/terminal owners lack public take-resume-close and Drop handback");
  if (
    !patches.includes("slots: Box<[Option<SurfaceSlot>]>") ||
    !patches.includes("rejected: Box<[Option<RejectedSlot>]>") ||
    !patches.includes("terminals: Box<[Option<TerminalSlot>]>") ||
    !patches.includes("producer_terminals: Box<[Option<MountedTreeTerminal>]>") ||
    !patches.includes("rejected_reserved: Box<[Option<u64>]>") ||
    !patches.includes("deferred: Box<[Option<ui_contract::SurfaceId>]>") ||
    !patches.includes("unadmitted: Box<[Option<UnadmittedSlot>]>") ||
    !patches.includes("closing_instances: Box<[Option<ClosingInstance>]>") ||
    !patches.includes("ready: Box<[Option<ReadySlot>]>") ||
    !patches.includes("slots: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS)") ||
    !patches.includes("unadmitted: fixed_slots(SURFACE_RECONCILE_ADMISSION_SLOTS + 1)") ||
    !patches.includes("ready: fixed_slots(READY_PATCH_CAPACITY)") ||
    !patches.includes("fn fixed_slots<T>(capacity: usize) -> Box<[Option<T>]>") ||
    !patches.includes(".take(capacity).collect::<Vec<_>>().into_boxed_slice()") ||
    patches.includes("HashMap") ||
    patches.includes("pub fn begin(&self, surface: String, tree: ComponentTree)") ||
    !patches.includes("pub fn commit_source(mut self, root: TreeNode) -> Result<(), TreeNode>") ||
    !patches.includes("pub fn reserve_mounted") ||
    !patches.includes("pub fn drive_one(&self)") ||
    !patches.includes("pub fn begin_close_instance") ||
    !patches.includes("pub fn close_step(&self)") ||
    !patches.includes("pub fn take_terminal") ||
    !patches.includes("pub fn take_rejected")
  ) failures.push("mounted PatchTracker is not the fixed admitted public lifecycle authority");
  const commitSource = patches.slice(patches.indexOf("pub fn commit_source"), patches.indexOf("#[cfg(test)]", patches.indexOf("pub fn commit_source")));
  if ((commitSource.match(/return Err\(root\)/g) ?? []).length !== 6 || commitSource.includes("drop(root)")) failures.push("mounted source rejection does not return the exact root on every preflight or transfer refusal");
  if (!patches.includes("ready: Box<[Option<ReadySlot>]>") || !patches.includes("pending < ready_generation") || !patches.includes("acknowledged_revision") || !patches.includes("closing_instances: Box<[Option<ClosingInstance>]>") || !patches.includes("surface_instance(slot.surface.as_ref()) == Some(closing.instance)") || !patches.includes("slot.instance == Some(closing.instance)") || !patches.includes("state.ready.iter().all(Option::is_none)") || !patches.includes("state.deferred.iter().all(Option::is_none)") || !patches.includes("state.unadmitted.iter().all(Option::is_none)")) failures.push("mounted reconcile fixed publication/ACK/close order or terminal emptiness omits a retained owner class");
  const unadmittedDefinition = patches.slice(patches.indexOf("struct UnadmittedSlot"), patches.indexOf("pub struct MountedReconcileGrant"));
  if (!patches.includes("MountedReconcileGrant") || !patches.includes("MountedReconcileOwner::Live { reconciler, reservation }") || !patches.includes("rejected_index: usize") || unadmittedDefinition.includes("tree:" ) || unadmittedDefinition.includes("reservation:") || patches.includes("tree: Option<ComponentTree>") || patches.includes("pub fn take_unadmitted")) failures.push("mounted producer can retain or publicly escape a whole unadmitted source tree instead of a reservation-only marker plus exact grant owner");
  const closeStart = patches.indexOf("pub fn close_step(&self)");
  const closeEnd = patches.indexOf("pub fn take_terminal", closeStart);
  const close = patches.slice(closeStart, closeEnd);
  const producingTerminal = close.indexOf("if terminal_target.is_none()");
  for (const blocked of ["state.unadmitted.iter().position", "state.rejected.iter().position", "state.slots.iter().position"]) if (producingTerminal < 0 || producingTerminal > close.indexOf(blocked)) failures.push(`mounted saturation close checks blocked ${blocked} before a capacity-producing terminal`);
  if (!patches.includes("fn next_generation(state: &PatchTrackerState) -> Option<u64>") || !patches.includes("state.next_generation.checked_add(1)") || !patches.includes("fn commit_generation(state: &mut PatchTrackerState, generation: u64)") || !patches.includes("state.generation_exhausted = generation == u64::MAX") || patches.includes("unwrap_or(u64::MAX)") || patches.includes("or(Some(u64::MAX))")) failures.push("mounted generation authority saturates, wraps, or reissues exhausted maximum");
  const mountedStart = patches.indexOf("pub fn reserve_mounted");
  const mounted = patches.slice(mountedStart, patches.indexOf("pub fn drive_one", mountedStart));
  if (mounted.indexOf("SurfaceReconcileReservation::try_new(generation)") < 0 || mounted.indexOf("state.rejected_reserved[rejected_index] = Some(generation)") < 0 || mounted.indexOf("commit_generation(&mut state, generation)") < mounted.indexOf("SurfaceReconcileReservation::try_new(generation)")) failures.push("mounted generation commits before exact aggregate/public-terminal reservation succeeds");
  const rejectedStart = patches.indexOf("pub fn mark_rejected");
  const rejectedEnd = patches.indexOf("pub fn mark_published_ack", rejectedStart);
  const rejected = patches.slice(rejectedStart, rejectedEnd);
  if (rejected.indexOf("state.terminals.iter().position(Option::is_none)") < 0 || rejected.indexOf("state.terminals.iter().position(Option::is_none)") > rejected.indexOf("next_generation(&state)") || rejected.indexOf("SurfaceReconcileTerminal::try_from_reconciler") > rejected.indexOf("commit_generation(&mut state, generation)")) failures.push("rejection consumes a generation or mutates its owner before exact terminal capacity succeeds");
  if (!reconcile.includes("SURFACE_RECONCILE_HANDBACK_SLOTS: usize = SURFACE_RECONCILE_ADMISSION_SLOTS * 6") || !reconcile.includes("slots: [SurfaceReconcileHandbackSlot; SURFACE_RECONCILE_HANDBACK_SLOTS]") || !reconcile.includes("pub struct SurfaceReconcileHandbackKey") || !reconcile.includes("registry.slots.get_mut(key.slot)") || !reconcile.includes("pub fn close_surface_reconcile_handback_one()") || reconcile.includes("handback_next: Option<Box") || reconcile.includes("static SURFACE_RECONCILE_TERMINALS") || reconcile.includes("else { drop(state); }") || reconcile.includes("pub fn from_reconciler") || reconcile.includes("pub fn from_patch")) failures.push("public reconciliation handback is dynamic, linear, uncredited, or lacks fixed scheduled one-owner close");
  const ackEvent = reactor.slice(reactor.indexOf("Event::PatchAck"), reactor.indexOf("Event::PatchRejected"));
  const pendingAuthority = reactor.slice(reactor.indexOf("struct PendingPatchAuthority"), reactor.indexOf("thread_local!", reactor.indexOf("struct PendingPatchAuthority")));
  if (!reconcile.includes("pub struct SurfaceReconcileReadyPatch") || !reconcile.includes("pub struct SurfaceReconcilePublishedPatch") || !reconcile.includes("pub struct SurfaceReconcilePublishedAck") || !patches.includes("authority: SurfaceReconcileReadyPatch") || !reactor.includes("slots: [Option<PendingPatchSlot>; PENDING_PATCH_CAPACITY]") || reactor.includes("RefCell<Vec<UiPatch>>") || !reactor.includes("PendingPatchOwner::Published") || !reactor.includes("apply_published_ack") || !ackEvent.includes("pending.borrow_mut().apply_published_ack") || ackEvent.indexOf("apply_published_ack") > ackEvent.indexOf("mark_published_ack") || !reactor.includes("ack.into_published()") || pendingAuthority.includes('expect("matching published patch authority")') || pendingAuthority.includes("unreachable!(\"published ACK lookup") || !reactor.includes("close_instance_one") || !reactor.includes("close_surface_reconcile_handback_one")) failures.push("published owner is not consumed nonpanically before generation-qualified tracker ACK or restored exactly on stale/duplicate/ABA refusal");
  const reconcilerDrop = reconcile.slice(reconcile.indexOf("impl Drop for SurfaceReconciler"), reconcile.indexOf("impl<K, V> SurfaceLinearMap", reconcile.indexOf("impl Drop for SurfaceReconciler")));
  const readyDrop = reconcile.slice(reconcile.indexOf("impl Drop for SurfaceReconcileReadyPatch"), reconcile.indexOf("pub struct SurfaceReconcilePublishedPatch"));
  if (!reconcilerDrop.includes("handback_surface_reconcile(state)") || reconcilerDrop.includes("self.retained.clear()") || !readyDrop.includes("handback_surface_reconcile") || readyDrop.includes("release_surface_reconcile(credit)") || !reconcile.includes("SURFACE_RECONCILE_TREE_RETIRE_DEPTH: usize = 4_097") || !reconcile.includes("if self.depth == self.frames.len()")) failures.push("populated ordinary Drop or deep-tree retirement can bulk destroy ownership, release credit, or overflow its fixed frames");
  const mountedRenderAt = reactor.indexOf("match PATCHES.with(|patches| patches.reserve_mounted(surface))");
  const mountedRender = reactor.slice(mountedRenderAt, reactor.indexOf("match crate::plugin_runtime::plugin_take_presence", mountedRenderAt)).replace(/\s+/g, " ");
  const renderFault = "Err(fault) => { grant.cancel(); effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) }); }";
  if ((reactor.match(/patches\.drive_one\(\)/g) ?? []).length !== 1 || mountedRenderAt < 0 || !mountedRender.includes("let _ = grant.commit_source(tree.root)") || !mountedRender.includes(renderFault) || !mountedRender.includes("let _ = patches.defer(surface)") || !reactor.includes("patches.begin_close_instance(*numeric_instance)") || !reactor.includes("patches.close_step()") || !reactor.includes("reconcile_work")) failures.push("production reactor does not mount one pre-reserved reconcile opportunity with lossless render-fault cancellation, deferred-storm and instance-close rearm");
  if (
    !reactor.includes("fn advance_command_cursor") ||
    !reactor.includes("cursor.page_index.checked_add(1)") ||
    reactor.includes("cursor.page_index.saturating_add(1)") ||
    reactor.includes("terminal.page_index = terminal.page_index.saturating_add(1)") ||
    !reactor.includes("static COMMAND_INGRESS: RefCell<[Option<CommandIngressOwner>; 2]>") ||
    !reactor.includes("(ingress[0].take(), 0)") ||
    reactor.includes('assert!(previous.is_none(), "command ingress has one exact owner")') ||
    reactor.includes('expect("host-validated generic pages preserve their fixed shape")') ||
    reactor.includes('expect("fixed timer tail authority")') ||
    reactor.includes('expect("fixed pending-patch close instance")')
  ) failures.push("reactor mounted owner/cursor transitions still saturate or panic instead of returning a fault or preserving the fixed owner");
  if (
    !reactor.includes("struct DirtyPollOwners") ||
    !reactor.includes("surfaces: ui_contract::UiFixedList<(u32, ui_contract::SurfaceId), DIRTY_RENDER_CAPACITY>") ||
    !reactor.includes("intents: ui_contract::UiFixedList<DirtyIntentBatch, DIRTY_INTENT_INSTANCE_CAPACITY>") ||
    !reactor.includes("fn try_surface(&mut self, instance: u32, surface: ui_contract::SurfaceId) -> Result<(), ui_contract::SurfaceId>") ||
    !reactor.includes("fn try_intent(&mut self, instance: u32, intent: ui_contract::UiIntent) -> Result<(), ui_contract::UiIntent>") ||
    reactor.includes("let mut dirty_render: Vec") ||
    reactor.includes("dirty_intents: HashMap<u32, Vec") ||
    reactor.includes("new batch retains one exact intent")
  ) failures.push("P5b reactor poll work owners are not fixed SurfaceId/UiIntent collections with exact nonpanicking refusal");
  if (
    !trackerDefinitions.includes("surface: ui_contract::SurfaceId") ||
    !trackerDefinitions.includes("deferred: Box<[Option<ui_contract::SurfaceId>]>") ||
    !patches.includes("pub fn reserve_mounted(&self, surface: ui_contract::SurfaceId) -> Result<MountedReconcileGrant<'_>, ui_contract::SurfaceId>") ||
    trackerDefinitions.includes("surface: String") ||
    trackerDefinitions.includes("deferred: [Option<String>")
  ) failures.push("P5b tracker still retains heap String identity instead of fixed SurfaceId text ownership");
}
