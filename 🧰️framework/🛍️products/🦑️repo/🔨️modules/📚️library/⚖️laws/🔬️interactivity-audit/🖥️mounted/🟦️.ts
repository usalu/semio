import { interactivityProductionSource } from "../🗄️database/🟦️.ts";

export function interactivityLiveReconcileFailures(reconcileSource: string, patchesSource: string, reactorSource: string, valueSource: string, schemaSource = ""): string[] {
  const reconcile = interactivityProductionSource(reconcileSource);
  const patches = interactivityProductionSource(patchesSource);
  const reactor = interactivityProductionSource(reactorSource);
  const value = interactivityProductionSource(valueSource);
  const schema = interactivityProductionSource(schemaSource);
  const driveStart = reconcile.indexOf("    pub fn drive_one(&mut self, cx:");
  const driveEnd = reconcile.indexOf("    pub fn fault(&self)", driveStart);
  const drive = reconcile.slice(driveStart, driveEnd);
  const turnPatchBridgeStart = reactor.indexOf("let owner = std::mem::take(&mut result.ui_patches)");
  const turnPatchBridge = reactor.slice(turnPatchBridgeStart, reactor.indexOf("//#endregion 🔀️BudgetBridge", turnPatchBridgeStart));
  const rendererRetainedStart = reactor.indexOf("struct RetainedDocumentBuild");
  const rendererRetained = reactor.slice(rendererRetainedStart, reactor.indexOf("struct PendingJobProgressPresentation", rendererRetainedStart));
  const commandDocumentStart = reactor.indexOf("const COMMAND_DOCUMENT_RETIREMENT_CAPACITY");
  const commandDocuments = reactor.slice(commandDocumentStart, reactor.indexOf("struct KernelPoolState", commandDocumentStart));
  const commandExchangeStart = reactor.indexOf("async fn exchange_commands(&mut self");
  const commandExchange = reactor.slice(commandExchangeStart, reactor.indexOf("async fn run_turn", commandExchangeStart));
  const shellDocumentStart = reactor.indexOf("const SHELL_DOCUMENT_RETIREMENT_CAPACITY");
  const shellDocuments = reactor.slice(shellDocumentStart, reactor.indexOf("pub struct ShellState", shellDocumentStart));
  const shellRefreshStart = reactor.indexOf("pub async fn refresh_ui(&mut self)");
  const shellRefresh = reactor.slice(shellRefreshStart, reactor.indexOf("fn queue_host_effects", shellRefreshStart));
  const rendererSequenceStart = reactor.indexOf("struct RendererSequenceAuthority");
  const rendererSequence = reactor.slice(rendererSequenceStart, reactor.indexOf("const JOB_PROGRESS_PRESENTATION_CAPACITY", rendererSequenceStart));
  const windowMeasureStart = reactor.indexOf("fn measures_for_kind");
  const windowMeasure = reactor.slice(windowMeasureStart, reactor.indexOf("fn render_window_engagement_rail", windowMeasureStart));
  const trackerDefinitions = patches.slice(patches.indexOf("struct SurfaceSlot"), patches.indexOf("impl PatchTracker"));
  const failures: string[] = [];
  if (
    !value.includes("pub struct UiList") ||
    !value.includes("pub struct UiMap") ||
    !value.includes("pub struct UiText") ||
    !value.includes("pub struct UiFixedList<T, const N: usize") ||
    !value.includes("items: Option<Box<[Option<T>]>>") ||
    !value.includes("let mut items = Vec::with_capacity(N)") ||
    !value.includes("items.resize_with(N, || None)") ||
    !value.includes("pub struct UiFixedMap<V> {") ||
    !value.includes("pub struct UiFixedBytes {") ||
    !value.includes("bytes: Box<[u8]>") ||
    !value.includes("vec![0; UI_FIXED_BYTES].into_boxed_slice()") ||
    !value.includes("List(UiList)") ||
    !value.includes("Map(UiMap)") ||
    !value.includes("struct UiValueArena") ||
    !value.includes("pages: Box<[UiPageSlot]>") ||
    !value.includes("collections: Box<[UiCollectionSlot]>") ||
    !value.includes("free_pages: Box<[usize]>") ||
    !value.includes("free_collections: Box<[usize]>") ||
    !value.includes("retirement: Box<[usize]>") ||
    !value.includes("let mut pages = Vec::with_capacity(UI_VALUE_AGGREGATE_ITEMS)") ||
    !value.includes("pages.resize_with(UI_VALUE_AGGREGATE_ITEMS, UiPageSlot::default)") ||
    !value.includes("let mut collections = Vec::with_capacity(UI_VALUE_ADMISSION_SLOTS)") ||
    !value.includes("collections.resize_with(UI_VALUE_ADMISSION_SLOTS, UiCollectionSlot::default)") ||
    !value.includes("free_page_count: UI_VALUE_AGGREGATE_ITEMS") ||
    !value.includes("free_collection_count: UI_VALUE_ADMISSION_SLOTS") ||
    !value.includes("retirement: vec![UI_VALUE_NONE; UI_VALUE_ADMISSION_SLOTS].into_boxed_slice()") ||
    !value.includes("epoch: u64") ||
    !value.includes("aliases: u64") ||
    !value.includes("pub struct UiListBuilder") ||
    !value.includes("pub struct UiMapBuilder") ||
    !value.includes("pub fn credited_clone") ||
    !value.includes("fn try_push_page(&mut self, handle: UiCollectionHandle, value: UiPageValue) -> Result<(), UiPageValue>") ||
    !value.includes("fn try_clone_handle(&mut self, handle: UiCollectionHandle) -> Option<UiCollectionHandle>") ||
    !value.includes("impl Drop for UiList {") ||
    !value.includes("impl Drop for UiMap {") ||
    !value.includes("pub fn close_ui_value_page_one()") ||
    !value.includes("let bytes = size_of::<UiPageSlot>()") ||
    value.includes("value.semantic_bytes()") ||
    !value.includes("if values.is_empty() { Ok(Self::default()) } else { Err(values) }") ||
    !value.includes("if entries.is_empty() { Ok(Self::default()) } else { Err(entries) }") ||
    !reactor.includes("ui_contract::close_ui_value_page_one()") ||
    value.includes("BTreeMap<String, UiValue>") ||
    value.includes("List(Vec<UiValue>)") ||
    value.includes("Arc<") ||
    value.includes("items: Vec<Option<T>>") ||
    value.includes("items: Box<[Option<T>; N]>") ||
    value.includes("entries: Box<UiFixedList<(UiText, V)>>") ||
    value.includes("bytes: Box<[u8; UI_FIXED_BYTES]>") ||
    value.includes("std::process::abort") ||
    value.includes("fn admit_page(") ||
    value.includes("push_reserved_page_back") ||
    value.includes("impl Clone for UiList") ||
    value.includes("impl Clone for UiMap") ||
    value.includes("#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]\n#[serde(untagged)]\npub enum UiValue") ||
    value.includes("saturating_") ||
    value.includes("wrapping_") ||
    value.includes("serializer.collect_seq") ||
    value.includes("Vec::<UiValue>::deserialize") ||
    value.includes("while let Some(value) = access.next_element::<UiValue>()? { builder.push(value)") ||
    value.includes("entries.sort_by(")
  ) failures.push("UiValue list/map schema is not a generation-qualified fixed arena with credited aliases, retained cursors, fail-closed source refusal, and one-page retirement");
  if (
    !schema.includes("pub struct BuiltChildren") ||
    !schema.includes("pub fn try_with_children") ||
    !schema.includes("fn try_child(") ||
    !schema.includes("pub struct ComponentTreeProducer") ||
    !schema.includes("pub use ui_contract::BuiltNode as TreeNode") ||
    schema.includes("pub struct TreeNode {") ||
    schema.includes("pub children: Vec<BuiltNode>") ||
    schema.includes("pub children: Vec<UiNodeId>") ||
    schema.includes("pub ops: Vec<UiPatchOp>") ||
    schema.includes("pub nodes: HashMap<UiNodeId, UiNodeRecord>") ||
    schema.includes("pub bytes: Vec<u8>") ||
    schema.includes("Option<HashMap<String, String>>") ||
    schema.includes("pub items: Vec<SelectItem>") ||
    schema.includes("pub entries: Vec<KeyValueEntry>") ||
    schema.includes("pub row_actions: Vec<RowAction>")
    || schema.includes("pub struct Label(pub String)")
    || schema.includes("pub struct SurfaceId(pub String)")
    || schema.includes("pub key: String")
    || schema.includes("pub scope: String")
    || schema.includes("pub name: String")
    || schema.includes("pub node_key: String")
    || schema.includes("pub id: String")
    || schema.includes("pub doc_schema: String")
    || schema.includes("pub shortcut: Option<String>")
    || schema.includes("pub bindings: Vec<crate::ActionBinding>")
    || schema.includes("pub children: Vec<")
    || schema.includes("pub nodes: Vec<")
    || schema.includes("pub ops: Vec<")
    || schema.includes('expect("fixed child backing follows its reservation")')
    || schema.includes('expect("🚫️ ImageBuilder<HasAlt>')
    || schema.includes("owner.cursor += 1")
    || schema.includes("retire_scalar += 1")
  ) failures.push("component/accessibility/surface/action/document schema still exposes recursive or dynamically backed mounted owners");
  if (
    !schema.includes("pub const UI_GRID_TRACKS: usize = 32") ||
    !schema.includes("pub type UiGridTracks = crate::UiFixedList<GridTrack, UI_GRID_TRACKS>") ||
    !schema.includes("pub columns: UiGridTracks") ||
    !schema.includes("pub rows: UiGridTracks") ||
    !schema.includes("pub fn try_push_column(&mut self, track: GridTrack) -> Result<(), GridTrack>") ||
    !schema.includes("pub fn try_push_row(&mut self, track: GridTrack) -> Result<(), GridTrack>") ||
    schema.includes("pub columns: Vec<GridTrack>") ||
    schema.includes("pub rows: Vec<GridTrack>") ||
    !schemaSource.includes("fn grid_tracks_max_plus_one_returns_the_exact_track")
  ) failures.push("GridLayout tracks are not fixed exact-owner lists with fallible max-plus-one admission");
  if (
    !schema.includes("pub const TABLE_WINDOW_ROWS: usize = 32") ||
    !schema.includes("pub type TableRows = UiFixedList<TableRow, TABLE_WINDOW_ROWS>") ||
    !schema.includes("pub type TableRowActions = UiFixedList<TableRowAction, TABLE_WINDOW_ROW_ACTIONS>") ||
    !schema.includes("binding: ActionBinding") ||
    !schema.includes("pub fn try_push_row(&mut self, row: TableRow) -> Result<(), TableRow>") ||
    !schema.includes("pub fn try_push_action(&mut self, action: TableRowAction) -> Result<(), TableRowAction>") ||
    !schema.includes("pub fn render_rows(view: TableRowsView) -> UiAssemblyResult<BuiltNode>") ||
    !schema.includes("impl Drop for TableRowsView") ||
    !schema.includes("pub fn close_table_rows_view_one() -> bool") ||
    !reactor.includes("crate::app::close_table_rows_view_one()") ||
    !schemaSource.includes("table_rows_max_plus_one_returns_the_exact_row_owner") ||
    !schemaSource.includes("abandoned_table_rows_retire_one_row_action_or_cell_per_opportunity") ||
    schema.includes("ui_wgpu::wgpu::ActionDescriptor") ||
    schema.includes("pub rows: Vec<TableRow>") ||
    schema.includes("pub actions: Vec<TableRowAction>") ||
    schema.includes("pub fn render_rows(view: &TableRowsView)") ||
    schema.includes("serde_json::to_string(&rows)") ||
    schema.includes("ui_wgpu::wgpu::TableCell")
  ) failures.push("TableWindowKit rows/actions are not fixed fallible UI-native owners with exact abandonment retirement");
  if (
    !schema.includes("pub const UI_COMMAND_VALUE_DEPTH: usize = 64") ||
    !schema.includes("struct UiCommandJsonProducer") ||
    !schema.includes("stack: UiFixedList<UiCommandJsonFrame, UI_COMMAND_VALUE_DEPTH>") ||
    !schema.includes("fn drive_one(&mut self, cancelled: bool, deadline_expired: bool)") ||
    !schema.includes("if cancelled {\n                self.pending = None") ||
    !schema.includes("if deadline_expired {\n                return UiCommandJsonStep::MoreWork") ||
    !schema.includes("UiCommandJsonFrame::List { cursor") ||
    !schema.includes("UiCommandJsonFrame::Map { cursor") ||
    !schema.includes("async fn ui_value_to_json_retained") ||
    !schema.includes("semio_framework_async::yield_once().await") ||
    !schema.includes("async fn merge_ui_values") ||
    !schemaSource.includes("retained_ui_value_bridge_cancel_and_deadline_preserve_the_original_owner") ||
    !schemaSource.includes("retained_ui_value_bridge_rejects_depth_plus_one_without_consuming_the_original") ||
    schema.includes("fn ui_value_to_json(value: &UiValue)") ||
    schema.includes("values.iter().map(|value| ui_value_to_json") ||
    schema.includes("entries.iter().map(|(key, value)|")
  ) failures.push("typed intent command arguments still recurse or lack a bounded retained page cursor and hostile owner fixtures");
  if (
    !schema.includes("pub const UI_DOCUMENT_LEASE_SLOTS: usize = 8") ||
    !schema.includes("pub const UI_DOCUMENT_LEASE_ALIASES: u64 = 8") ||
    !schema.includes("slots: [UiDocumentSlot; UI_DOCUMENT_LEASE_SLOTS]") ||
    !schema.includes("pub struct UiDocumentBuilder") ||
    !schema.includes("pub struct UiDocumentLease") ||
    !schema.includes("pub struct UiDocumentNodePage") ||
    !schema.includes("impl UiDocumentLease {\n    pub fn generation(&self) -> u64") ||
    !schema.includes("pub fn try_alias(&self) -> Result<Self, UiDocumentLeaseError>") ||
    !schema.includes("pub fn read_node_page(&self, index: usize) -> Result<Option<UiDocumentNodePage>, UiDocumentLeaseError>") ||
    !schema.includes("pub fn close_step(&mut self) -> bool") ||
    !schema.includes("pub fn close_ui_document_page_one() -> bool") ||
    schema.includes("slots: Vec<UiDocumentSlot>") ||
    schema.includes("pub struct UiDocumentLease {\n    state: UiSnapshotState") ||
    !reactor.includes("ui_contract::close_ui_document_page_one()")
  ) failures.push("renderer document publication is not a generation-qualified fixed lease with one-node page reads and mounted incremental close");
  if (
    !reconcile.includes("pub struct SurfaceDocumentProducer") ||
    !reconcile.includes("pub enum SurfaceDocumentProducerStep") ||
    !reconcile.includes("pub struct SurfaceDocumentOutcome") ||
    !reconcile.includes("pub fn drive_one(&mut self, current: &SurfaceReconciler, cx: &mut semio_framework_job::StepContext<'_>)") ||
    !reconcile.includes("cx.generation().0 != self.generation") ||
    !reconcile.includes("current.revision != self.revision") ||
    !reconcile.includes("current.retained.get_index(self.next_node)") ||
    !reconcile.includes("builder.try_push(record)") ||
    !reconcile.includes("pub fn take_ready(mut self) -> Result<SurfaceDocumentOutcome, Self>") ||
    reconcile.includes("current.retained.values().cloned().collect")
  ) failures.push("retained reconciler does not expose a stale-safe one-record-per-opportunity fixed document producer/outcome seam");
  if (
    !schema.includes("pub struct UiPatchApplyProducer") ||
    !schema.includes("pub enum UiPatchApplyStep") ||
    !schema.includes("pub struct UiPatchApplyOutcome") ||
    !schema.includes("pub struct UiPatchApplyRejected") ||
    !schema.includes("pub fn try_new(state: crate::UiSnapshotState, patch: crate::UiPatch, limits: UiDocumentLimits, generation: u64)") ||
    !schema.includes("pub fn drive_one(&mut self, generation: u64, cancelled: bool, deadline_expired: bool)") ||
    !schema.includes("fn census_patch_one(&mut self)") ||
    !schema.includes("fn clone_state_one(&mut self)") ||
    !schema.includes("fn apply_op_one(&mut self)") ||
    !schema.includes("fn remove_subtree_one(&mut self)") ||
    !schema.includes("UiPatchApplyPhase::RemoveSubtree") ||
    !schema.includes("fn validate_candidate_one(&mut self)") ||
    !schema.includes("validation_stack: crate::UiFixedList<UiPatchValidateFrame") ||
    !schema.includes("validation_seen: [crate::UiFixedList<crate::UiText") ||
    !schema.includes("pub fn take_ready(mut self) -> Result<UiPatchApplyOutcome, Self>") ||
    !schema.includes("pub fn take_rejected(mut self) -> Result<UiPatchApplyRejected, Self>") ||
    !schema.includes("fn retire_snapshot_one") ||
    !schema.includes("fn retire_patch_one") ||
    !schema.includes("pub const UI_PATCH_APPLY_SLOTS: usize = 8") ||
    !schema.includes("slots: [UiPatchApplySlot; UI_PATCH_APPLY_SLOTS]") ||
    !schema.includes("retirement_handle: Option<UiPatchApplyHandle>") ||
    !schema.includes("impl Drop for UiPatchApplyProducer") ||
    !schema.includes("impl Drop for UiPatchApplyOutcome") ||
    !schema.includes("impl Drop for UiPatchApplyRejected") ||
    !schema.includes("remove_record: Option<crate::UiNodeRecord>") ||
    !schema.includes("pub fn close_ui_patch_owner_one() -> bool") ||
    !reactor.includes("ui_contract::close_ui_patch_owner_one()") ||
    !schemaSource.includes("#[cfg(test)]\npub fn validate_snapshot") ||
    schema.includes("pub fn apply_patch(state: &mut crate::UiSnapshotState")
  ) failures.push("renderer patch application is not a by-value retained generation-qualified transactional producer with exact outcome/rejection retirement");
  if (
    !schema.includes("pub const UI_TURN_PATCHES_MAXIMUM: usize = semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES") ||
    !schema.includes("pub const UI_TURN_PATCH_BUDGET_BYTES: usize = semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES / 4") ||
    !schema.includes("pub fn ui_patch_turn_bytes(patch: &UiPatch) -> usize") ||
    schema.includes("pub const UI_TURN_PATCHES_MAXIMUM: usize = 1") ||
    !schema.includes("pub struct UiTurnPatches") ||
    !schema.includes("entries: [UiTurnPatchEntry; UI_TURN_PATCHES_MAXIMUM]") ||
    !schema.includes("assert!(size_of::<UiTurnPatches>() <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES)") ||
    !schema.includes("pub fn try_push_ui_patch(&mut self, patch: UiPatch) -> Result<(), UiPatch>") ||
    !schema.includes("pub const UI_TURN_PATCH_RETIRE_SLOTS: usize = 64") ||
    !schema.includes("slots: [UiTurnPatchRetireSlot; UI_TURN_PATCH_RETIRE_SLOTS]") ||
    !schema.includes("retirement: Option<UiTurnPatchRetireKey>") ||
    !schema.includes("impl Drop for UiTurnPatches") ||
    !schema.includes("pub fn close_ui_turn_patch_owner_one() -> bool") ||
    !schema.includes("pub const UI_TURN_PATCH_TRANSPORT_SLOTS: usize = 64") ||
    !schema.includes("pub struct UiTurnPatchTransportProducer") ||
    !schema.includes("pub fn drive_one(&mut self, session: u64, cancelled: bool, deadline_expired: bool)") ||
    !schema.includes("if !self.ready || self.transferred") ||
    !schema.includes("pub struct UiTurnPatchTransportLease") ||
    !schema.includes("pub fn try_from_token(token: &[u8], expected_session: u64)") ||
    !schema.includes("pub fn close_ui_turn_patch_transport_one() -> bool") ||
    !schema.includes("impl<'de> Deserialize<'de> for UiTurnPatches") ||
    !schema.includes("struct UiTurnPatchesVisitor") ||
    !schema.includes("pub ui_patches: UiTurnPatches") ||
    schema.includes("pub ui_patches: Vec<UiPatch>") ||
    !reactor.includes("let mut page = semio_framework::kernel::UiTurnPatches::default()") ||
    !reactor.includes("page.try_push_ui_patch(patch)") ||
    !reactor.includes("pub(crate) fn take_turn_patch_page(budget_bytes: usize)") ||
    !reactor.includes("semio_framework::kernel::ui_patch_turn_bytes(&patch)") ||
    !reactor.includes("close_ui_turn_patch_owner_one()") ||
    !reactor.includes("result.ui_patches.iter().map(kernel_ui_patch_to_wit)") ||
    !reactor.includes("Result<semio_framework_actor::TurnResult, semio_framework::Fault>") ||
    !reactor.includes("to_actor_turn_result(mut result: TurnResult") ||
    !reactor.includes("UiTurnPatchTransportProducer::try_new(session, owner)") ||
    !reactor.includes("patch_transport.drive_one(session, false, false)") ||
    !reactor.includes("let ui_patches = patch_transport\n        .take_ready()") ||
    !reactor.includes("semio_framework_async::yield_once().await") ||
    turnPatchBridgeStart < 0 ||
    turnPatchBridge.indexOf("let effects = serde_json::to_vec(&result.effects)") < 0 ||
    turnPatchBridge.indexOf("let effects = serde_json::to_vec(&result.effects)") > turnPatchBridge.indexOf(".take_ready()") ||
    (reactor.match(/UiTurnPatchTransportLease::try_from_token\(&result\.ui_patches/g) ?? []).length < 2 ||
    (reactor.match(/close_ui_turn_patch_transport_session_one\(/g) ?? []).length < 2 ||
    (reactor.match(/semio_framework::kernel::close_ui_turn_patch_transport_one\(\)/g) ?? []).length < 3 ||
    (reactor.match(/to_actor_turn_result\(result, actor\.0, wall_us, memory_bytes\)\.await\.map_err\(\|_\| KernelError::InvalidTransition\)\?/g) ?? []).length < 1 ||
    (reactor.match(/match to_actor_turn_result\(result, actor_id, 0, 0\)\.await/g) ?? []).length < 1 ||
    !reactor.includes("let bridged = to_actor_turn_result_in_place(&mut result, actor_id, 0, 0).await") ||
    !reactor.includes("match bridged") ||
    reactor.includes("serde_json::from_slice(&result.ui_patches)") ||
    reactor.includes("serde_json::to_vec(&result.ui_patches)")
  ) failures.push("TurnResult patch ownership escapes through a growable vector or infallible whole-turn encoder instead of the fixed exact-owner authority");
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
  return failures;
}

export function interactivityMountedLayoutTextFailures(mountedSource: string, engineSource: string, treeSource: string, paintSource: string, eventsSource: string, slotsSource: string, interpreterSource: string, glueSource: string): string[] {
  const mounted = interactivityProductionSource(mountedSource);
  const engine = interactivityProductionSource(engineSource);
  const tree = interactivityProductionSource(treeSource);
  const paint = interactivityProductionSource(paintSource);
  const events = interactivityProductionSource(eventsSource);
  const slots = interactivityProductionSource(slotsSource);
  const interpreter = interactivityProductionSource(interpreterSource);
  const glue = interactivityProductionSource(glueSource);
  const failures: string[] = [];
  const requireAll = (source: string, needles: readonly string[], label: string) => {
    if (needles.some((needle) => !source.includes(needle))) failures.push(label);
  };
  const count = (source: string, needle: string) => source.split(needle).length - 1;
  const operation = (source: string, start: string, end: string) => {
    const from = source.indexOf(start);
    const until = source.indexOf(end, from + start.length);
    return from < 0 ? "" : source.slice(from, until < 0 ? source.length : until);
  };
  const law = (source: string, name: string, evidence: readonly string[]) => {
    const start = source.indexOf(`fn ${name}`);
    const end = source.indexOf("\n    #[test]", start + name.length);
    const body = start < 0 ? "" : source.slice(start, end < 0 ? source.length : end);
    if (evidence.some((needle) => !body.includes(needle))) failures.push(`P5c hostile law ${name} is missing production-path evidence`);
  };

  requireAll(mounted, [
    "LAYOUT_NODE_CREDITS: usize = 4_096",
    "LAYOUT_GLYPH_CREDITS: usize = 16_384",
    "LAYOUT_DEPTH_CREDITS: usize = 64",
    "LAYOUT_ATLAS_PAGE_CREDITS: usize = 4",
    "LAYOUT_ATLAS_PAGE_BYTES: usize = 16 * 1024",
  ], "P5c fixed node/glyph/depth/atlas credits are missing or changed");
  requireAll(mounted, [
    "walk: Box<ui_contract::UiFixedList<WalkFrame",
    "nodes: Box<ui_contract::UiFixedList<LayoutInputNode",
    "glyphs: Box<ui_contract::UiFixedList<RetainedGlyphInput",
    "runs: Box<ui_contract::UiFixedList<RetainedTextRun",
    "lines: Box<ui_contract::UiFixedList<RetainedLine",
    "glyph_previews: Box<ui_contract::UiFixedList<RetainedGlyphPreview",
    "results: Box<ui_contract::UiFixedList<MountedLayoutResult",
    "pages: [Option<Box<[u8; LAYOUT_ATLAS_PAGE_BYTES]>>; LAYOUT_ATLAS_PAGE_CREDITS]",
  ], "P5c production working sets are not the actual fixed admitted owners");
  requireAll(mounted, ["text_byte: usize", "text_glyph_start: usize", "run_cursor: usize", "glyph_cursor: usize", "line_cursor: usize", "measure_cursor: usize", "arrange_cursor: usize", "preview_cursor: usize", "publish_cursor: usize"], "P5c retained node/run/glyph/line/preview/publication cursors are incomplete");
  for (const forbidden of ["nodes: Vec<", "glyphs: Vec<", "runs: Vec<", "lines: Vec<", "glyph_previews: Vec<", "results: Vec<", "walk: Vec<", "pages: Vec<", "HashMap<", "VecDeque", "FontAtlas", ".measure(", "std::mem::forget", "wrapping_add", "loop {", "while !job"]) if (mounted.includes(forbidden)) failures.push(`P5c mounted production source contains forbidden ${forbidden}`);
  if (!mounted.includes("child_scratch: Vec<usize>")) failures.push("P5c mounted production source lacks bounded child-index scratch reuse");
  const admit = operation(mounted, "pub(crate) fn admit_one", "fn admit_node_one");
  const worker = operation(mounted, "fn worker_one", "fn shape_one");
  const shape = operation(mounted, "fn shape_one", "fn measure_one");
  const publish = operation(mounted, "pub(crate) fn publish_one", "fn stage_label");
  const close = operation(mounted, "pub(crate) fn close_one", "pub(crate) fn terminal_is_empty");
  requireAll(admit, ["cx.is_cancelled()", "cx.should_yield()", "cx.consume_fuel(1)", "match self.admission"], "P5c admission is not one cancel/deadline/fuel-bounded cursor opportunity");
  requireAll(worker, ["if self.close_requested || cx.is_cancelled()", "cx.set_stage(self.stage_label())", "match self.stage {", "cx.consume_fuel(1)", "cx.deadline_exceeded()"], "P5c worker turn does not prove pool-thread and bounded before/after checks");
  if (count(worker, "cx.is_cancelled()") < 2) failures.push("P5c opaque text worker call is not cancellation-checked before and after");
  requireAll(shape, ["self.runs.get(self.run_cursor)", "self.text_worker.shape_one(input)", "self.atlas_candidate.retain_one", "self.glyph_cursor += 1"], "P5c text shaping is not one retained run/glyph/atlas unit");
  requireAll(mounted, ["trait OwnedTextWorker: Send", "generation: u64", "revision: u64", "atlas_page: u8", "atlas_offset: u16", "atlas_length: u8"], "P5c external text boundary or generation-tagged atlas output is missing");
  requireAll(publish, ["self.identity() != identity", "self.results.len() != self.nodes.len()", "tree.write_inactive_layout", "tree.commit_inactive_layout(generation)", "self.publication_committed = true"], "P5c publication is not complete, fresh, inactive, and one-swap-only");
  requireAll(close, ["rejected_result.take()", "rejected_glyph.take()", "self.results.pop()", "self.atlas_candidate.close_one()"], "P5c close does not release exactly one retained/rejected owner or atlas page");

  requireAll(engine, [
    "slots: Box<[Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS]>",
    "generations: [u64; UI_LAYOUT_SURFACE_SLOTS]",
    "slots: [Option<SurfaceLaneEntry>; UI_LAYOUT_SURFACE_SLOTS]",
    "window_id: SurfaceId",
    "layout_queues: [SurfaceLaneRing; 3]",
  ], "P5c multi-surface authority or lane queues are not fixed token storage");
  for (const forbidden of ["VecDeque", "HashMap<String, UiWindow>", "wrapping_add", "job.worker_one", "try_step_on_caller", "WorkerPool::new(", "window.layout_session = None; session.close_step"]) if (engine.includes(forbidden)) failures.push(`P5c engine production path contains forbidden ${forbidden}`);
  const scheduler = operation(engine, "pub fn step_layouts", "fn enqueue_layout");
  requireAll(scheduler, ["WorkerJobPoll::CheckedOut", "session.pump_one(pool, worker_lane(lane), cx.retained_grant())", "cx.cancel_token()", "close_layout_worker_session(session", "job.identity() != identity", "MountedWorkerJobSession::try_admit_owned(&mut window.layout_job, &mut window.layout_admission.params, cx)"], "P5c engine does not retain exact mounted take/resume/close authority on the shared pool");
  if (scheduler.includes("MountedWorkerJobSession::try_new")) failures.push("P5c engine admits its mounted layout session through the removed fallible constructor");
  requireAll(engine, ["theme_layout_identity(&self.theme) != theme_layout_identity(&theme)", "window.theme_revision == u64::MAX", "window.viewport_revision.checked_add(1)", "window.layout_generation.checked_add(1)", "window.revision.checked_add(1)"], "P5c equality or non-wrapping theme/tree/viewport generations are incomplete");
  requireAll(engine, ["layout_preview: Option<MountedLayoutResult>", "glyph_preview: Option<RetainedGlyphPreview>", "MountedLayoutJob::take_preview_one", "job.latest_glyph_preview()", "window.layout_preview = Some(preview)", "preview.generation == window.layout_generation", "preview.revision == window.revision", "window.glyph_preview = Some(preview)"], "P5c progressive geometry/text preview retention is missing or stale-unchecked");

  requireAll(tree, ["mounted_layout: [MountedLayoutRecord; 2]", "mounted_layout_active: usize", "mounted_layout_generation: u64", "write_inactive_layout", "self.mounted_layout_active ^= 1", "mounted.generation == self.mounted_layout_generation", "self.mounted_layout_generation = generation"], "P5c tree lacks inactive/active O(1) accepted snapshot authority");
  const retainedPaint = operation(paint, "pub(crate) fn paint_node_step(", "pub(crate) fn paint_tree");
  const sceneSlotNode = operation(slots, "pub(crate) fn scene_slot_for_node", "pub(crate) fn collect_scene_slots");
  if (!retainedPaint.includes("let Some(layout) = tree.accepted_layout(id) else { return RetainedNodePaintStep::Fault }") || !operation(events, "fn hit_test_node", "fn is_plain_stack_container").includes("let layout = tree.accepted_layout(id)?") || !sceneSlotNode.includes("let layout = tree.accepted_layout(id)?")) failures.push("P5c retained paint, hit testing, and scene slots do not consume the same accepted snapshot");

  const driver = operation(interpreter, "fn drive_mounted_layout_text_one", "//#region 📄️RetainedDocumentConsumer");
  requireAll(driver, ["StepBudget::from_duration(1, now, 1000)", "crate::renderer_worker_pool()", "engine.step_layouts(&pool", "pool.pump(now / 1_000)"], "P5c mounted renderer does not drive one shared-pool opportunity");
  const documentRoute = operation(interpreter, "pub(crate) fn render_ui_document_step", "fn render_ui_image_step");
  const viewportArm = documentRoute.indexOf("UiDocumentFramePhase::Viewport");
  const layoutArm = documentRoute.indexOf("UiDocumentFramePhase::Layout");
  const paintArm = documentRoute.indexOf("UiDocumentFramePhase::Paint");
  const driveAt = documentRoute.indexOf("drive_mounted_layout_text_one");
  const frameAt = documentRoute.indexOf("frame_into_step", paintArm);
  if (!(viewportArm >= 0 && layoutArm > viewportArm && paintArm > layoutArm && driveAt > layoutArm && frameAt > paintArm && frameAt > driveAt && documentRoute.includes("engine.set_viewport"))) failures.push("P5c renderer production route does not prioritize fresh input then layout drive before frame");
  const poolGlue = operation(glue, "//#region 🧵️RendererWorkerPool", "//#endregion 🧵️RendererWorkerPool");
  requireAll(poolGlue, ["pub(crate) fn renderer_worker_pool()", "process_worker_pool", "ProcessKind::InteractiveNative"], "P5c renderer is not using the one process WorkerPool on native and Wasm-shaped routes");
  if (poolGlue.includes("WorkerPool::new")) failures.push("P5c renderer introduced a second layout scheduler");

  law(mountedSource, "mounted_layout_actual_glyph_max_plus_one_retains_the_exact_rejected_scalar", ["job.rejected_glyph().map", "Some('x')", "while !job.close_one()", "job.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_actual_node_max_plus_one_retains_the_exact_rejected_tree_owner", ["tree.contains(rejected.id)", "!job.nodes.iter().any", "while !job.close_one()"]);
  law(mountedSource, "mounted_layout_multi_page_unicode_uses_one_glyph_or_atlas_boundary_per_turn", ["job.glyph_cursor - before <= 1", "LAYOUT_ATLAS_PAGE_CREDITS - 1", "preview.generation == 11"]);
  law(mountedSource, "mounted_layout_deep_tree_depth_refusal_retains_walk_authority_for_close", ["job.rejected_walk.is_some()", "retained - after, 1", "job.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_worker_runs_on_shared_user_visible_lane_and_pool_thread", ["session.pump_one(&pool, lane)", "Lane::UserVisible", "owner.worker_thread_observed()", "session.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained", ["after.cancel_after_shape", "StepOutcome::Cancelled", "after.glyph_cursor, 1", "after.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner", ["StepBudget::new(1, 0)", "retained - after_one, 1", "job.terminal_is_empty()"]);
  law(mountedSource, "mounted_layout_publication_rechecks_full_identity_and_repeat_ready_swaps_once", ["LayoutJobStep::Fault(\"layout.stale\")", "tree.accepted_layout_generation(), before", "tree.accepted_layout_generation(), 11"]);
  law(engineSource, "mounted_layout_surface_max_plus_one_returns_exact_owner_without_mutation", ["rejected.id, owner", "ui.windows.ids().cloned().collect"]);
  law(engineSource, "mounted_layout_equal_theme_does_not_invalidate_or_requeue", [
    "assert_eq!(ui.windows.get(\"theme\").map(|window| window.layout_generation), before_generation)",
    "assert_eq!(ui.windows.get(\"theme\").map(|window| window.theme_revision), before_theme_revision)",
    "assert_eq!(ui.layout_queues.iter().map(SurfaceLaneRing::len).sum::<usize>(), before_queue)",
  ]);
  law(engineSource, "mounted_layout_atomic_snapshot_keeps_last_valid_geometry_until_fresh_swap", ["after, old_generation", "old_layout", "swaps, 1", "progressive_glyph_preview"]);
  law(engineSource, "mounted_layout_revision_max_refuses_theme_tree_and_viewport_without_alias", ["theme_revision = u64::MAX", "viewport_revision = u64::MAX", "revision = u64::MAX"]);
  law(engineSource, "mounted_layout_replay_and_resize_supersede_are_deterministic", ["assert_eq!(first, second)", "second.width, 640.0"]);
  law(engineSource, "large_layout_and_shaping_job_admits_one_work_unit_per_slice", ["slices > 10_000", "assert_eq!(widest_slice, (1, 1)", "accepted_layout"]);
  law(engineSource, "interactive_storm_does_not_starve_background_surface_lane", ["slice < LANE_WHEEL.len()", "window_id.as_ref() == \"background\""]);
  return failures;
}
