/** 🖥️ Interactivity audit: mounted reconcile, layout and frame transaction sources; surface lifetime, prepared render and surface lane sources live in the child `🖼️mounted-surface`.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts
 */
import { interactivityProductionSource } from "../🗄️database/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { toolJobRustBlock } from "../../🎯️tool-job-coverage/📋️inventory/🟦️.ts";

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



//#region 🧵️P5cMountedLayoutText
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
  requireAll(scheduler, ["session.poll() == semio_framework_job::WorkerJobPoll::CheckedOut", "session.pump_one(pool, worker_lane(lane))", "cx.cancel_token()", "session.close_step(1", "job.close_one()", "job.identity() != identity", "MountedWorkerJobSession::try_new"], "P5c engine does not retain exact mounted take/resume/close authority on the shared pool");
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


//#endregion 🧵️P5cMountedLayoutText

//#region 🔄️P5aMountedFrameTransaction
/** 🧭️ Extracts one uniquely declared Rust item through the shared literal/comment-aware block scanner. */
export function interactivityCheckedRustBlockScope(source: string, marker: string, name: string, failures: string[]): string | undefined {
  const escaped = marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const declarations = [...source.matchAll(new RegExp(`^[\\t ]*${escaped}`, "gmu"))];
  const stale = `P5a policy boundary is stale: ${name}`;
  if (declarations.length !== 1) {
    if (!failures.includes(stale)) failures.push(stale);
    return undefined;
  }
  const start = source.indexOf(marker, declarations[0]!.index);
  const headerEnd = start + marker.length;
  const open = source.indexOf("{", headerEnd);
  const semicolon = source.indexOf(";", headerEnd);
  const block = open < 0 || (semicolon >= 0 && semicolon < open) ? undefined : toolJobRustBlock(source, open);
  if (!block) {
    if (!failures.includes(stale)) failures.push(stale);
    return undefined;
  }
  return policyStripRustCommentsAndStrings(source.slice(start, block.end));
}

export function interactivityMountedFrameTransactionFailures(
  glueSource: string,
  frameJobSource: string,
  hostSource: string,
  snapshotSource: string,
  uiRuntimeGlueSource: string,
  shellSource: string,
  engineCanvasSource: string,
  world3dSource: string,
  preparedSource: string,
  gpuSource: string,
  drawSource: string,
  interpreterSource: string,
  uiEngineSource: string,
  paintSource: string,
  sceneSlotsSource: string,
  scenesSource: string,
  servicesSource: string,
): string[] {
  const glue = interactivityProductionSource(glueSource);
  const frameJob = interactivityProductionSource(frameJobSource);
  const host = interactivityProductionSource(hostSource);
  const snapshot = interactivityProductionSource(snapshotSource);
  const uiRuntimeGlue = interactivityProductionSource(uiRuntimeGlueSource);
  const shell = interactivityProductionSource(shellSource);
  const engineCanvas = interactivityProductionSource(engineCanvasSource);
  const world3d = interactivityProductionSource(world3dSource);
  const prepared = interactivityProductionSource(preparedSource);
  const gpu = interactivityProductionSource(gpuSource);
  const draw = interactivityProductionSource(drawSource);
  const interpreter = interactivityProductionSource(interpreterSource);
  const uiEngine = interactivityProductionSource(uiEngineSource);
  const paint = interactivityProductionSource(paintSource);
  const sceneSlots = interactivityProductionSource(sceneSlotsSource);
  const scenes = interactivityProductionSource(scenesSource);
  const services = interactivityProductionSource(servicesSource);
  const failures: string[] = [];
  const scope = (source: string, marker: string, name: string) => interactivityCheckedRustBlockScope(source, marker, name, failures);
  const scopes = (...parts: (string | undefined)[]) => parts.every((part) => part !== undefined) ? parts.join("\n") : undefined;
  const transaction = glue.slice(glue.indexOf("pub(crate) struct FrameTransaction"), glue.indexOf("pub(crate) struct AppFramePresentation"));
  const frameCursors = glue.slice(glue.indexOf("struct FrameBuildCursor"), glue.indexOf("struct FrameWheelCursor"));
  const buildBoundary = glue.slice(glue.indexOf("fn frame_before_input_step"), glue.indexOf("fn frame_after_input_step"));
  const finishBoundary = glue.slice(glue.indexOf("fn frame_after_input_step"), glue.indexOf("//#region 🔖️OsHostDecomposition — SemioApp deletion"));
  const deferredBoundary = glue.slice(glue.indexOf("fn start_frame_deferred"), glue.indexOf("fn apply_step", glue.indexOf("fn start_frame_deferred")));
  const maintenanceOwnerBoundary = glue.slice(glue.indexOf("struct FrameDeferredCursor"), glue.indexOf("enum RuntimeApply"));
  const maintenanceNativeOwnerBoundary = glueSource.slice(glueSource.indexOf("struct FrameDeferredCursor"), glueSource.indexOf("enum RuntimeApply"));
  const maintenanceCloseBoundary = scope(maintenanceNativeOwnerBoundary, "fn begin_close(&mut self)", "native maintenance close");
  const maintenanceExecutionBoundary = glue.slice(glue.indexOf("struct FrameMaintenanceExecutionRegistry"), glue.indexOf("impl<R: MailboxRuntime> RuntimeMailbox<R> {"));
  const maintenanceRegistryBoundary = glue.slice(glue.indexOf("struct FrameMaintenanceExecutionRegistry"), glue.indexOf("struct FrameMaintenanceAuthority"));
  const maintenanceSpawnBoundary = glue.slice(glue.indexOf("fn try_spawn_frame_maintenance_reserved"), glue.indexOf("#[cfg(target_arch = \"wasm32\")]", glue.indexOf("fn try_spawn_frame_maintenance_reserved")));
  const maintenanceTerminalBoundary = glue.slice(glue.indexOf("fn frame_maintenance_terminal_fault"), glue.indexOf("enum RuntimeApply"));
  const maintenanceWorkerTerminalStart = maintenanceSpawnBoundary.indexOf("if let Some(fault) = frame_maintenance_terminal_fault");
  const maintenanceWorkerTerminalBoundary = maintenanceSpawnBoundary.slice(maintenanceWorkerTerminalStart, maintenanceSpawnBoundary.indexOf("            } else {", maintenanceWorkerTerminalStart));
  const chromeBoundary = scopes(
    scope(shell, "pub(crate) struct ShellChromeFrameCursor", "Shell chrome cursor"),
    scope(shell, "enum ShellChromeFramePhase", "Shell chrome phase"),
    scope(shell, "pub(crate) fn render_chrome_step", "Shell chrome step"),
  );
  const engineBoundary = engineCanvas.slice(engineCanvas.indexOf("const ENGINE_CANVAS_FRAME_PACKET_CAPACITY"), engineCanvas.indexOf("struct EngineGpuSurface"));
  const worldBoundary = world3d.slice(world3d.indexOf("const WORLD3D_FRAME_RESOURCE_CAPACITY"), world3d.indexOf("//#endregion 📦️PreparedWorldResources"));
  const atlasBoundary = prepared.slice(prepared.indexOf("pub const PREPARED_ATLAS_PAGE_BYTES"), prepared.indexOf("impl PreparedRasterPages"));
  const atlasGpuBoundary = gpu.slice(gpu.indexOf("struct PreparedAtlasUploadCursor"), gpu.indexOf("pub fn finish_prepared"));
  const atlasDrawBoundary = draw.slice(draw.indexOf("upload_glyph_atlas_page"), draw.indexOf("pub fn render_frame"));
  const chromeMainBoundary = scope(shell, "fn render_main_window_step", "Shell main window");
  const chromePanelBoundary = scope(shell, "fn render_panel_step", "Shell panel");
  const chromeNavbarBoundary = scope(shell, "fn render_navbar_step", "Shell navbar");
  const chromeTutorialBoundary = scope(shell, "fn render_tutorial_bar_step", "Shell tutorial");
  const chromeFooterBoundary = scope(shell, "fn render_footer_step", "Shell footer");
  const chromeOverlayStepBoundary = scope(shell, "fn render_overlay_step", "Shell overlay");
  const chromeContextMenuBoundary = scope(shell, "fn render_context_menu_step", "Shell context menu");
  const chromeTooltipBoundary = scope(shell, "fn render_chrome_tooltip_step", "Shell tooltip");
  const chromeDialogBoundary = scope(shell, "fn render_chrome_dialog_step", "Shell dialog");
  const chromeTourBoundary = scope(shell, "fn render_chrome_tour_step", "Shell tour");
  const chromeOverlayBoundary = scopes(chromeOverlayStepBoundary, chromeContextMenuBoundary, chromeTooltipBoundary, chromeDialogBoundary, chromeTourBoundary);
  const shellChildren = scopes(chromeMainBoundary, chromePanelBoundary, chromeNavbarBoundary, chromeTutorialBoundary, chromeFooterBoundary, chromeOverlayBoundary);
  const documentBoundary = interactivityProductionSource(interpreterSource.slice(interpreterSource.indexOf("pub struct UiDocumentFrameCursor"), interpreterSource.indexOf("pub fn render_ui_document(")));
  const uiFrameBoundary = uiEngine.slice(uiEngine.indexOf("const RETAINED_PAINT_DEPTH_CREDITS"), uiEngine.indexOf("pub fn frame<H"));
  const paintNodeBoundary = scopes(
    scope(paint, "pub struct RetainedGlyphCursor", "paint glyph cursor"),
    scope(paint, "pub(crate) struct RetainedNodePaintCursor", "paint node cursor"),
    scope(paint, "impl RetainedNodePaintCursor", "paint node cursor implementation"),
    scope(paint, "fn paint_retained_glyph_step_inner", "paint retained glyph step"),
    scope(paint, "fn retained_fixed_output_budgeted", "paint output credit"),
    scope(paint, "fn retained_text_node_step_weighted", "paint retained text"),
    scope(paint, "fn retained_control_is_bounded", "paint bounded control"),
    scope(paint, "fn retained_tree_node_step", "paint retained tree"),
    scope(paint, "pub(crate) fn paint_node_step(", "paint node step"),
    scope(paint, "pub(crate) fn paint_node_step_with_driver", "paint node driver step"),
  );
  const interactiveSyncBoundary = scopes(
    scope(paint, "struct RetainedSyncTreeFrame", "interactive sync tree frame"),
    scope(paint, "struct RetainedSyncTreeRecord", "interactive sync tree record"),
    scope(paint, "pub(crate) struct RetainedInteractiveSyncCursor", "interactive sync cursor"),
    scope(paint, "impl RetainedInteractiveSyncCursor", "interactive sync cursor implementation"),
    scope(paint, "pub(crate) fn sync_interactive_state_node_step", "interactive sync step"),
  );
  const sceneNodeBoundary = scopes(
    scope(sceneSlots, "pub struct ScenePaintCursor", "scene paint cursor"),
    scope(sceneSlots, "impl ScenePaintCursor", "scene paint cursor implementation"),
    scope(sceneSlots, "pub trait SceneHost", "scene host contract"),
    scope(sceneSlots, "pub(crate) fn scene_slot_for_node", "scene slot node resolver"),
  );
  const sceneHostBoundary = interpreter.slice(interpreter.indexOf("impl ui_wgpu::wgpu::SceneHost for FrameworkSceneHost"), interpreter.indexOf("fn drive_mounted_layout_text_one"));
  const retainedImageBoundary = interpreter.slice(interpreter.indexOf("fn render_ui_image_step"), interpreter.indexOf("fn render_ui_image("));
  const retainedSceneBoundary = scenes.slice(scenes.indexOf("pub fn render_component_scene_step"), scenes.indexOf("pub fn render_component_scene("));
  const findBoundary = shell.slice(shell.indexOf("pub struct ShellFindItems"));
  const maintenanceBoundary = shell.slice(shell.indexOf("struct ShellChromeMaintenance"), shell.indexOf("fn body_rect"));
  const chromeTextBoundary = scopes(
    scope(shell, "fn chrome_text_step", "Shell chrome text step"),
    scope(shell, "fn chrome_text_complete_step", "Shell chrome text completion"),
  );
  const chromeGroupWidthBoundary = scope(shell, "fn retained_chrome_group_item_width", "Shell chrome group width");
  const chromeGroupStepBoundary = scopes(
    scope(shell, "enum RetainedChromeGroupStep", "Shell chrome group phase"),
    scope(shell, "fn render_retained_chrome_group_item_step(", "Shell chrome group step"),
    scope(shell, "fn render_retained_chrome_group_item_step_with_trailing", "Shell chrome group trailing step"),
  );
  const chromeGroupBoundary = scopes(chromeGroupWidthBoundary, chromeGroupStepBoundary);
  const prefsBoundary = shell.slice(shell.indexOf("fn native_pref_field_path"), shell.indexOf("//#endregion 🗄️PrefsStore"));
  const nativePrefsBoundary = prefsBoundary.slice(0, prefsBoundary.indexOf("fn prefs_get("));
  const fixedFilePageBoundary = services.slice(services.indexOf("//#region 📄️FixedFilePage"), services.indexOf("//#endregion 📄️FixedFilePage"));
  const requireAll = (source: string | undefined, needles: readonly string[], label: string) => {
    if (source === undefined) return;
    if (needles.some((needle) => !source.includes(needle))) failures.push(label);
  };
  const requireExactlyOne = (source: string | undefined, needle: string, label: string) => {
    if (source === undefined) return;
    if (source.split(needle).length - 1 !== 1) failures.push(label);
  };
  requireAll(transaction, [
    "DrainProjectionDeltas",
    "RouteIntents",
    "FlushEffects",
    "PresentSurface",
    "ReconcileTree",
    "BuildRenderPackets",
    "PublishSnapshot",
    "context.set_stage(self.stage_label())",
  ], "P5a exact seven-stage mounted transaction is incomplete");
  requireAll(glue, [
    "struct FrameActionOwners",
    "slots: [Option<ActionDescriptor>; WORLD3D_DEADLINE_CAPACITY]",
    "fn try_push(&mut self, action: ActionDescriptor) -> Result<(), ActionDescriptor>",
    "fn pop_front(&mut self) -> Option<ActionDescriptor>",
    "EFFECT_STORM_BUDGET: u32 = 64",
  ], "P5a action/effect admission is not fixed, fallible, and identity preserving");
  requireAll(glueSource, ["frame_action_max_plus_one_returns_the_exact_owner", "assert_eq!(rejected.controller_id.as_ptr(), identity)"], "P5a exact MAX + 1 action owner law is missing");
  requireAll(transaction, [
    "operation: semio_framework_trace::OperationId",
    "generation: semio_framework_trace::Generation",
    "base_witness: Option<RuntimePresentationWitness>",
    "context.operation() != self.operation",
    "context.generation() != self.generation",
    "context.is_cancelled()",
    "context.deadline_exceeded()",
    "base_witness != current_witness",
    "checked_add(1)",
  ], "P5a opportunity freshness, revision, generation, cancellation, deadline, or effect guards are incomplete");
  for (const forbidden of ["VecDeque", "std::vec::IntoIter", "wrapping_add", "saturating_add", ".clear()", ".expect(", ".unwrap(", "loop {", "while "]) if (transaction.includes(forbidden)) failures.push(`P5a mounted transaction contains forbidden ${forbidden}`);
  requireAll(frameCursors, [
    "struct FrameBuildCursor",
    "previous_draw: Option<DrawList>",
    "previous_overlay: Option<DrawList>",
    "struct FrameFinishCursor",
    "icon_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>",
    "glyph_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>",
  ], "P5a mounted build/finish cursors do not retain draw, overlay, and atlas page ownership");
  requireAll(glue, ["struct FrameEnginePackets", "engine_packets: FrameEnginePackets", "fn try_push(&mut self, packet: engine_canvas::EngineCanvasPacket) -> Result<(), engine_canvas::EngineCanvasPacket>"], "P5a mounted packet collector is not fixed and identity-preserving");
  if (frameCursors.includes("engine_packets: Vec<") || frameCursors.includes("Vec<engine_canvas::EngineCanvasPacket>")) failures.push("P5a mounted frame cursor retains a dynamic engine packet workset");
  requireAll(buildBoundary, [
    "FrameBuildPhase::PreparedGpuAbandonment",
    "PreparedGpuPresentCursor::close_abandoned_step()",
    "FrameBuildPhase::PreparedInputAbandonment",
    "PreparedRenderInput::next_abandoned_close_demands(4096)",
    "PreparedRenderInput::close_abandoned_step(grant)",
    "FrameBuildPhase::PreparedJobAbandonment",
    "PreparedRenderJob::next_abandoned_close_demands(4096)",
    "PreparedRenderJob::close_abandoned_step(grant)",
    "FrameBuildPhase::PreparedAbandonment",
    "PreparedRenderReceiver::next_abandoned_close_demands(4096)",
    "PreparedRenderReceiver::close_abandoned_step(grant)",
    "FrameBuildPhase::PacketAbandonment",
    "PreparedRenderPacket::next_abandoned_close_demands(4096)",
    "PreparedRenderPacket::close_abandoned_step(grant)",
    "FrameBuildPhase::RetireDraw",
    "previous.retire_step()",
    "FrameBuildPhase::RetireOverlay",
    "render_chrome_step",
    "resources.append_step(input)",
    "resources.take_packet_step()",
    "PreparedAtlasPages::try_new",
    "pages.push_page(&self.icons.pixels, pages.next_row())",
    "PreparedRenderUpload::IconAtlasPages",
  ], "P5a pre-input boundary does not advance retained draw, chrome, packet, and upload owners one step at a time");
  requireAll(finishBoundary, [
    "FrameFinishPhase::GlyphUpload",
    "pages.push_page(&self.atlas.pixels, pages.next_row())",
    "PreparedRenderUpload::GlyphAtlasPages",
    "FrameFinishPhase::Draw",
    "FrameFinishPhase::Overlay",
    "self.pending_frame_deferred = Some",
  ], "P5a post-input boundary does not page atlas work and retain draw/overlay/deferred transfers");
  for (const forbidden of ["frame_before_input(", "frame_after_input(", "drive_pending_frame_deferred", "take_packets(", "append_to(", "self.draw.clear()", "self.overlay.clear()", "self.icons.pixels.clone()", "self.atlas.pixels.clone()"])
    if (glue.includes(forbidden)) failures.push(`P5a mounted call graph retains opaque whole-work callee ${forbidden}`);
  requireAll(deferredBoundary, ["cursor_value.take_next()", "spawn_frame_deferred_reserved", "try_spawn_frame_maintenance_reserved", "pending_frame_maintenance_refusal", "FrameDeferredCursor::close_step", "runtime.return_interaction(interaction)"], "P5a deferred boundary does not take, refuse, close, and resume exactly one retained work owner");
  for (const forbidden of [".expect(", ".unwrap(", "loop {", "while "])
    if (deferredBoundary.includes(forbidden)) failures.push(`P5a deferred boundary retains panicking or run-to-completion ${forbidden}`);
  requireAll(maintenanceOwnerBoundary, [
    "generation: u64",
    "cancel: semio_framework_async::CancelToken",
    "closing: bool",
    "struct FrameMaintenanceOwner",
    "deadline_ms: Option<u64>",
    "struct FrameMaintenanceOwnerCell<T>",
    "state: std::sync::atomic::AtomicU8",
    "compare_exchange(0, 1",
    "fn try_restore(&self, owner: T) -> Result<(), T>",
    "struct FrameMaintenanceRefusal",
    "struct FrameMaintenanceAuthority",
    "fn is_live(&self, generation: u64) -> bool",
    "fn release(&self, generation: u64) -> bool",
    "self.generation.store(0, Ordering::Release)",
    "compare_exchange(1, 0",
  ], "P5a maintenance authority is not generation/cancel/deadline-qualified with nonblocking exact-owner refusal handback");
  requireAll(maintenanceNativeOwnerBoundary, ["#[cfg(any(not(target_arch = \"wasm32\"), test))]"], "P5a native maintenance close is not available in production");
  requireAll(maintenanceCloseBoundary, ["self.closing = true"], "P5a native maintenance close does not retain its closing state");
  requireAll(maintenanceExecutionBoundary, [
    "struct FrameMaintenanceExecutionRegistry",
    "const QUEUED: u8 = 2",
    "const RUNNING: u8 = 3",
    "const ABANDONED: u8 = 4",
    "fn try_publish(&self, generation: u64",
    "fn try_begin(&self, generation: u64)",
    "fn abandon(&self, generation: u64) -> bool",
    "fn try_take_abandoned(&self)",
    "struct FrameMaintenanceExecutionEnvelope",
    "impl Drop for FrameMaintenanceExecutionEnvelope",
    "if self.armed && self.registry.abandon(self.generation)",
    "RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None }",
    "struct FrameMaintenanceExecutionGuard",
    "impl Drop for FrameMaintenanceExecutionGuard",
    "self.owner_cell.restore_taken(owner)",
    "frame_maintenance_executions: Arc<FrameMaintenanceExecutionRegistry>",
  ], "P5a accepted maintenance execution lacks generation-qualified external Drop handback");
  for (const forbidden of ["Mutex", ".lock()", "std::mem::take", "Vec<", "loop {", "while "])
    if (maintenanceRegistryBoundary.includes(forbidden)) failures.push(`P5a maintenance execution registry retains blocking, dynamic, or recursive recovery ${forbidden}`);
  requireAll(maintenanceSpawnBoundary, [
    "fn try_spawn_frame_maintenance_reserved",
    "frame_maintenance.try_reserve(generation)",
    "renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job)",
    "error.into_job()",
    "FrameMaintenanceRefusal { owner: reclaimed, reservation_live: true }",
    "cursor.cancel.is_cancelled_now()",
    "presentation_authority.witness_for(generation).is_none()",
    "worker_pool.now_ms() > deadline",
    "frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded)",
    "cursor.begin_close()",
    "frame_maintenance.release(generation)",
    "execution.finish()",
  ], "P5a maintenance submission does not preserve exact owners across refusal/cancel/stale/deadline/close");
  requireAll(maintenanceTerminalBoundary, ["if cancelled", "else if stale", "else if deadline_exceeded", "frame maintenance cancelled", "frame maintenance generation became stale", "frame maintenance deadline expired"], "P5a maintenance terminal classifier is incomplete for cancel/stale/deadline");
  requireAll(maintenanceWorkerTerminalBoundary, ["frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded)", "cursor.begin_close()", "interaction.frame_fault = Some(fault.to_string())"], "P5a maintenance terminal outcome does not enter retained close before handback");
  for (const forbidden of ["KernelPoolFuture::spawn", "let _ = renderer_worker_pool().try_submit", "renderer_worker_pool().submit(", "Lane::Interactive", ".lock()", "loop {", "while "])
    if (maintenanceSpawnBoundary.includes(forbidden)) failures.push(`P5a maintenance submission retains discarded, blocking, interactive-lane, or whole-work ${forbidden}`);
  if (maintenanceOwnerBoundary.includes("self.actions = FrameActionOwners::default()")) failures.push("P5a deferred close bulk-drops the populated action owner");
  requireAll(findBoundary, [
    "slots: Box<[Option<ShellFindItem>; SHELL_FIND_ITEM_CAPACITY]>",
    "payload_bytes: usize",
    "generation: u64",
    "fn try_push_at(&mut self, generation: u64, item: ShellFindItem) -> Result<(), ShellFindItem>",
    "fn pop_front(&mut self) -> Option<ShellFindItem>",
    "fn close_step(&mut self) -> bool",
    "std::cell::Cell<Option<ActiveShellFindItems>>",
    "std::ptr::NonNull<ShellFindItems>",
    "pub fn try_push_find_item(item: ShellFindItem) -> Result<(), ShellFindItem>",
    "collector.try_push_at(active.generation, item)",
  ], "P5a find producer is not fixed, generation-qualified, nonblocking, and exact-owner fallible");
  for (const forbidden of ["Arc<Mutex<Vec<ShellFindItem>>>", "Vec<ShellFindItem>", ".lock()", "std::mem::take", "take_find_items", "ACTIVE_FIND_ITEM_SINKS"])
    if (findBoundary.includes(forbidden)) failures.push(`P5a find producer retains blocking, dynamic, or whole-owner ${forbidden}`);
  requireAll(shellSource, ["find_item_max_plus_one_returns_the_exact_owned_item", "stale_find_generation_returns_the_exact_owned_item", "assert_eq!(rejected.id.as_ptr(), identity)"], "P5a find MAX + 1, stale-generation, close, or exact-owner law is missing");
  requireAll(maintenanceBoundary, [
    "load_requested: bool",
    "load_phase: u8",
    "introduction_read: Option<String>",
    "introduction_write: Option<String>",
    "persist_phase: u8",
    "request_chrome_preferences_load",
    "request_introduction_read",
    "request_panel_layout_persist",
    "request_presence_preview",
    "request_chrome_preferences_persist",
    "pub(crate) fn advance_chrome_maintenance_step",
    "read_ui_preferences()",
    "stored_field_get(",
    "stored_field_set(",
    "persist_ui_preferences(",
  ], "P5a Shell persistence/presence work is not a retained one-field/page child");
  requireAll(prefsBoundary, [
    "fn native_pref_field_path",
    "key.len() > SHELL_CHROME_IO_FIELD_BYTES",
    "fn native_pref_read_page",
    "semio_framework_os_services::storage_worker_read_fixed_file_document(&path, OS_SHELL_CONFIG_MAX_BYTES)",
    "semio_framework_os_services::storage_worker_read_fixed_file_page(&path, SHELL_CHROME_IO_FIELD_BYTES)",
    "String::from_utf8(page)",
    "fn native_pref_write_page",
    "semio_framework_os_services::storage_worker_write_fixed_file_page(&path, value.as_bytes(), SHELL_CHROME_IO_FIELD_BYTES)",
    "fn prefs_get(",
    "fn prefs_set(",
  ], "P5a native preference maintenance does not delegate document and field pages to the host-storage service");
  requireAll(fixedFilePageBoundary, [
    "pub const STORAGE_FIXED_FILE_PAGE_BYTES: usize = 16 * 1024",
    "pub fn storage_worker_read_fixed_file_page",
    "maximum_bytes > STORAGE_FIXED_FILE_PAGE_BYTES",
    "logical_bytes > maximum_bytes",
    "let mut page = [0u8; STORAGE_FIXED_FILE_PAGE_BYTES]",
    "file.read_exact(&mut page[..logical_bytes])",
    "if file.read(&mut trailing)? != 0",
    "pub fn storage_worker_write_fixed_file_page",
    "bytes.len() > maximum_bytes",
    "std::fs::create_dir_all(parent)?",
    "file.write_all(bytes)",
  ], "P5a host-storage service does not own exact 16 KiB page admission/read/write and growth rejection");
  if (!servicesSource.includes("fixed_file_page_exact_max_plus_one_matches_system_oracle_and_preserves_last_valid_page")) failures.push("P5a host-storage fixed page exact/max-plus-one/oracle preservation law is missing");
  for (const forbidden of ["PREFS_STORE", "Mutex", ".lock()", "serde_json::from_", "serde_json::to_", "OS_SHELL_CONFIG_STORAGE_KEY", "Vec::with_capacity", "vec![", "loop {", "while ", "std::fs::", "std::io::"])
    if (nativePrefsBoundary.includes(forbidden)) failures.push(`P5a native preference field page retains blocking, dynamic, or whole-config ${forbidden}`);
  for (const forbidden of ["load_ui_prefs_once()", "read_stored_introduction_seen(", "persist_panel_layout_if_changed()", "write_stored_introduction_seen(", "persist_ui_prefs_if_changed()", "publish_presence_heartbeat()"])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a chrome frame still reaches synchronous maintenance ${forbidden}`);
  requireAll(glue, ["FrameDeferredWork::ShellMaintenance", "shell_maintenance: bool", "semio_framework_async::Lane::Io", "PreparedAtlasPages::next_abandoned_close_demands(4096)", "PreparedAtlasPages::close_abandoned_step(grant)"], "P5a Shell I/O or abandoned atlas close is not mounted on the shared retained boundary");
  requireAll(chromeBoundary, [
    "ShellChromeFramePhase::MainWindow",
    "ShellChromeFramePhase::PaneOverlayHits",
    "ShellChromeFramePhase::Panels",
    "ShellChromeFramePhase::AgentChatHeader",
    "ShellChromeFramePhase::Navbar",
    "ShellChromeFramePhase::TutorialBar",
    "ShellChromeFramePhase::Footer",
    "ShellChromeFramePhase::Overlay",
    "ShellChromeFramePhase::IntroductionWrite",
    "introduction_seen_writes.pop()",
  ], "P5a shell chrome boundary is not a retained one-child cursor");
  requireAll(chromeBoundary, [
    "render_main_window_step(&mut cursor.child",
    "render_mobile_panel_step(&mut cursor.child",
    "render_panel_step(&mut cursor.child, anchor",
    "render_agent_chat_header_step(&mut cursor.child",
    "render_navbar_step(&mut cursor.child",
    "render_tutorial_bar_step(&mut cursor.child",
    "render_footer_step(&mut cursor.child",
    "render_overlay_step(&mut cursor.child",
    "cursor.advance(ShellChromeFramePhase::PaneOverlayHits)",
  ], "P5a shell chrome phases do not remain parked on retained child cursors");
  for (const forbidden of ["self.render_main_window(", "self.render_left_panel(", "self.render_right_panel(", "self.render_navbar(", "self.render_tutorial_bar(", "self.render_footer(", "self.render_overlay(", "self.render_tree_drag_overlay(", "render_tutorial_gesture_overlay("])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a shell chrome cursor reaches whole child ${forbidden}`);
  for (const forbidden of ["render_chrome(", "render_chrome_build(", ".clear()", "std::mem::take(&mut self.chrome_build.introduction_seen_writes)", "for app_id in", "while "])
    if (chromeBoundary?.includes(forbidden)) failures.push(`P5a shell chrome cursor retains whole-work ${forbidden}`);
  requireAll(shellChildren, [
    "fn render_main_window_step",
    "render_ui_document_step(",
    "fn render_panel_step",
    "fn render_navbar_step",
    "fn render_tutorial_bar_step",
    "fn render_footer_step",
    "fn render_overlay_step",
    "fn render_context_menu_step",
    "fn render_chrome_tooltip_step",
    "fn render_chrome_dialog_step",
    "fn render_chrome_tour_step",
  ], "P5a live Shell children are not retained node/widget cursors");
  for (const forbidden of ["render_main_window(", "render_left_panel(", "render_right_panel(", "render_navbar(", "render_tutorial_bar(", "render_footer(", "render_overlay(", "render_ui_document(", "render_context_menu(", "render_chrome_tour(", "measure_chrome_group_item(", "render_chrome_group("])
    if (shellChildren?.includes(forbidden)) failures.push(`P5a live Shell child body reaches whole subtree ${forbidden}`);
  requireAll(chromeTextBoundary, [
    "fn chrome_text_step",
    "paint_retained_glyph_step",
    "fn chrome_text_complete_step",
    "RetainedGlyphStep::Pending",
    "RetainedGlyphStep::Complete",
    "RetainedGlyphStep::Fault",
    "cursor.reset()",
  ], "P5a Shell text callee is not a retained per-scalar/glyph authority");
  requireAll(chromeGroupBoundary, [
    "enum RetainedChromeGroupStep",
    "fn retained_chrome_group_item_width",
    "label.len() > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES",
    "fn render_retained_chrome_group_item_step",
    "group_phase: &mut u8",
    "glyph: &mut RetainedGlyphCursor",
    "chrome_text_complete_step",
    "RetainedChromeGroupStep::Pending",
    "RetainedChromeGroupStep::Complete",
    "RetainedChromeGroupStep::Fault",
  ], "P5a Shell chrome-group callee is not a fixed retained output/glyph authority");
  requireAll(shellSource, [
    "#[cfg(test)]\nfn measure_chrome_group_item",
    "#[cfg(test)]\nfn render_chrome_group",
  ], "P5a legacy whole-string chrome-group oracles are not explicitly test-only");
  for (const forbidden of ["fn measure_chrome_group_item", "fn render_chrome_group"])
    if (shell.includes(forbidden)) failures.push(`P5a production Shell retains legacy whole-string chrome-group reachability ${forbidden}`);
  for (const forbidden of ["draw_text(", "chrome_text(", "measure_text(", "for ch in", "for item in", "loop {", "while "])
    if (chromeGroupStepBoundary?.includes(forbidden)) failures.push(`P5a retained Shell chrome group reaches whole work ${forbidden}`);
  requireAll(chromeDialogBoundary, ["chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false", "cursor.scalar += 1"], "P5a live dialog does not park on the retained glyph cursor");
  requireAll(chromeTourBoundary, ["chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false", "cursor.scalar += 1"], "P5a live tour does not park on the retained glyph cursor");
  requireAll(chromeTutorialBoundary, ["fn render_tutorial_bar_step", "chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false"], "P5a mounted tutorial bar does not park dynamic text on the retained glyph cursor");
  requireAll(chromeTutorialBoundary, ["render_retained_chrome_group_item_step", "retained_chrome_group_item_width", "RetainedChromeGroupStep::Pending => return false"], "P5a mounted tutorial groups do not park each output and glyph opportunity");
  requireAll(chromeFooterBoundary, ["render_retained_panel_chrome_item_step", "render_footer_status_step", "Ok(false) => return false"], "P5a mounted footer does not park on retained dynamic utility and sync labels");
  requireAll(chromeOverlayBoundary, ["fn render_overlay_step", "fn render_context_menu_step", "fn render_chrome_tooltip_step", "fn render_chrome_dialog_step", "fn render_chrome_tour_step", "chrome_text_complete_step", "&mut cursor.glyph", "Ok(false) => return false"], "P5a mounted overlay children do not park dynamic text on the retained glyph cursor");
  requireAll(chromeBoundary, ["ShellChromeFramePhase::Error", "chrome_text_complete_step(draw, atlas, error", "&mut cursor.child.glyph", "Ok(false) => return false"], "P5a mounted Shell error path does not park on the retained glyph cursor");
  for (const [boundary, label] of [[chromeTextBoundary, "text callee"], [chromeDialogBoundary, "dialog"], [chromeTourBoundary, "tour"]] as const)
    for (const forbidden of ["draw_text(", "chrome_text(", "for ch in", "for character in", ".chars().for_each", ".chars().count()", "loop {", "while "])
      if (boundary?.includes(forbidden)) failures.push(`P5a Shell ${label} retains whole-string work ${forbidden}`);
  for (const [boundary, label] of [[chromeNavbarBoundary, "navbar"], [chromeTutorialBoundary, "tutorial"], [chromeFooterBoundary, "footer"], [chromeOverlayBoundary, "overlay"]] as const)
    for (const forbidden of ["draw_text(", "chrome_text(", ".chars().count()", "for ch in", "for character in", "loop {", "while "])
      if (boundary?.includes(forbidden)) failures.push(`P5a mounted Shell ${label} reaches whole-string work ${forbidden}`);
  requireAll(shellSource, ["dialog_and_tour_text_advance_one_scalar_and_one_glyph_per_grant", "dialog_and_tour_text_max_plus_one_fails_without_output", "dynamic_chrome_group_retains_every_glyph_and_border_opportunity", "dynamic_chrome_group_max_plus_one_is_identity_preserving"], "P5a Shell multi-megabyte one-scalar, chrome-group, or MAX + 1 law is missing");
  requireAll(documentBoundary, [
    "pub struct UiDocumentFrameCursor",
    "UiDocumentFramePhase::Ingress",
    "UiDocumentFramePhase::Layout",
    "UiDocumentFramePhase::Paint",
    "engine.apply_document_page",
    "drive_mounted_layout_text_one",
    "engine.frame_into_step",
  ], "P5a document consumer does not retain ingress/layout/node progress");
  for (const forbidden of ["engine.frame(", "composite_retained_draw_list(", "dispatch_pointer_events("])
    if (documentBoundary.includes(forbidden)) failures.push(`P5a live document consumer reaches whole work ${forbidden}`);
  requireAll(uiFrameBoundary, [
    "RETAINED_PAINT_DEPTH_CREDITS: usize = 64",
    "visits: [Option<RetainedPaintVisit>; RETAINED_PAINT_DEPTH_CREDITS]",
    "enum RetainedPaintWalkStep",
    "pub fn frame_into_step",
    "sync_interactive_state_node_step",
    "sync_node: Option<",
    "node_sync: RetainedInteractiveSyncCursor",
    "frame.node_sync.close_step()",
    "RetainedInteractiveSyncStep::Pending",
    "RetainedInteractiveSyncStep::Complete",
    "RetainedInteractiveSyncStep::Fault",
    "paint_node_step",
    "paint_node: Option<(",
    "node_paint: RetainedNodePaintCursor",
    "scene_node: Option<(",
    "scene_paint: ScenePaintCursor",
    "host.paint_slot_step",
    "scene_slot_for_node",
    "RetainedPaintPhase::Publish",
  ], "P5a retained UI frame does not advance one fixed visit/node/scene/publication unit");
  for (const forbidden of ["paint_tree(", "sync_interactive_state_node(&mut", "collect_scene_slots(", "window.draw.clear()", "for slot in"])
    if (uiFrameBoundary.includes(forbidden)) failures.push(`P5a live UI frame reaches whole subtree ${forbidden}`);
  requireAll(paint, [
    "RETAINED_SYNC_COLLECTION_ITEMS: usize = 256",
    "RETAINED_SYNC_OUTPUTS: usize = RETAINED_SYNC_COLLECTION_ITEMS * 2",
    "RETAINED_SYNC_DEPTH: usize = 64",
    "RETAINED_SYNC_KEY_BYTES: usize = 256",
  ], "P5a mounted interactive synchronization fixed limits are missing");
  requireAll(interactiveSyncBoundary, [
    "struct RetainedSyncTreeFrame",
    "items_pointer: usize",
    "items_len: usize",
    "tree_frames: [Option<RetainedSyncTreeFrame>; RETAINED_SYNC_DEPTH]",
    "tree_records: [Option<RetainedSyncTreeRecord>; RETAINED_SYNC_OUTPUTS]",
    "pub(crate) fn sync_interactive_state_node_step",
    "RetainedInteractiveSyncPhase::SelectItem",
    "RetainedInteractiveSyncPhase::SelectScan",
    "RetainedInteractiveSyncPhase::SelectWrite",
    "RetainedInteractiveSyncPhase::TreeItem",
    "RetainedInteractiveSyncPhase::TreeApplyScan",
    "RetainedInteractiveSyncPhase::TreeApplyWrite",
    "RetainedInteractiveSyncPhase::TreeClose",
    "pub(crate) fn close_step(&mut self) -> bool",
  ], "P5a mounted interactive synchronization lacks fixed retained item/depth/output ownership");
  for (const forbidden of ["Vec<", ".clone()", ".collect::<Vec", "tree.children(id)", "sync_tree_row_layout", "sync_tree_item_layout", "for index in", "for child in", "while ", "loop {"])
    if (interactiveSyncBoundary?.includes(forbidden)) failures.push(`P5a mounted interactive synchronization retains whole collection/depth work ${forbidden}`);
  requireAll(paint, ["RETAINED_NODE_TEXT_MAX_BYTES: usize = 4 * 1024 * 1024"], "P5a retained node text bound is missing");
  requireAll(paintNodeBoundary, [
    "pub(crate) struct RetainedNodePaintCursor",
    "glyph: RetainedGlyphCursor",
    "phase: u16",
    "item: usize",
    "path: [usize; RETAINED_TREE_DEPTH]",
    "byte: usize",
    "line: usize",
    "pen_x: f32",
    "pub(crate) fn paint_node_step",
    "draw.begin_retained_output(strikes, strikes * size_of::<crate::wgpu::draw::UiInstance>())",
    "draw.finish_retained_output()",
    "value[cursor.byte..].chars().next()",
    "cursor.byte = next_byte",
    "atlas.ensure_glyph(ch, size)",
    "cursor.line.checked_add(1)",
    "match &node.spec.0",
    "UiNode::Select(select)",
    "UiNode::KeyValue(key_value)",
    "UiNode::Tree(tree_node) => retained_tree_node_step",
    "fn retained_tree_node_step",
    "retained_control_is_bounded",
  ], "P5a paint child authority is not retained by byte, glyph, line, output credit, and close state");
  for (const forbidden of ["wrap_text(", ".collect::<Vec", "for (index, line)", "for ch in", "for character in", "paint_stack(tree", "paint_node(tree", "paint_node_self(", "paint_control(", "for child in", "loop {", "while "])
    if (paintNodeBoundary?.includes(forbidden)) failures.push(`P5a paint node recursively reaches child work ${forbidden}`);
  requireAll(paintSource, [
    "retained_text_paint_emits_at_most_one_glyph_per_grant",
    "retained_multi_megabyte_input_advances_one_scalar_per_grant",
    "retained_select_max_plus_one_refuses_before_output_without_moving_tree_owner",
    "retained_select_sync_writes_at_most_one_row_per_grant",
    "retained_select_sync_max_plus_one_fault_closes_exact_cursor_owner",
    "retained_tree_sync_abandonment_releases_one_record_or_depth_owner_per_grant",
    "retained_text_multi_megabyte_max_plus_one_preserves_tree_owner_identity",
    "retained_text_cancel_close_preserves_exact_terminal_witness",
  ], "P5a retained node text/non-text one-scalar, collection MAX + 1, identity, or close law is missing");
  requireAll(glueSource, [
    "frame_deferred_cancel_token_closes_one_exact_owner_per_grant",
    "frame_maintenance_authority_refuses_aba_and_releases_the_exact_generation",
    "frame_maintenance_refusal_cell_recovers_exact_identity_without_blocking",
    "frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant",
    "accepted_frame_maintenance_queue_drop_publishes_exact_generation_owner",
    "interrupted_frame_maintenance_execution_restores_before_incremental_recovery",
  ], "P5a maintenance cancel/ABA/refusal exact-owner laws are missing");
  requireAll(sceneNodeBoundary, [
    "pub struct ScenePaintCursor",
    "node: Option<NodeId>",
    "phase: u16",
    "item: usize",
    "page: usize",
    "byte: usize",
    "pub fn advance_byte(&mut self) -> Result<(), ScenePaintCursorError>",
    "fn paint_slot_step",
    "pub(crate) fn scene_slot_for_node",
    "UiNode::ComponentScene",
    "UiNode::Image",
  ], "P5a scene child authority is not one-node and producer-cursor addressable");
  if (sceneNodeBoundary?.includes("collect_scene_slots(")) failures.push("P5a one-node scene authority reaches whole scene collection");
  requireAll(sceneSlotsSource, ["scene_paint_cursor_rejects_stale_node_without_consuming_owner", "scene_paint_cursor_advances_one_scalar_and_closes_one_bound_owner"], "P5a scene stale-owner, scalar, or close law is missing");
  requireAll(sceneHostBoundary, ["fn paint_slot_step", "cursor.bind(slot.node)", "render_component_scene_step", "render_ui_image_step"], "P5a Framework scene host does not resume exact retained scene/image consumers");
  for (const forbidden of ["render_component_scene(", "render_ui_image(", "fn paint_slot("])
    if (sceneHostBoundary.includes(forbidden)) failures.push(`P5a Framework scene host reaches complete leaf renderer ${forbidden}`);
  requireAll(retainedSceneBoundary, ["cursor.phase()", "cursor.byte()", "cursor.advance_byte()", "try_reserve_retained_items(1)", "cursor.advance_item()", "cursor.finish()"], "P5a component scene consumer is not retained by scalar/item phase");
  requireAll(retainedImageBoundary, ["cursor.phase()", "cursor.byte()", "cursor.advance_byte()", "queue_ui_image_url_fetch", "try_reserve_retained_items(1)", "cursor.advance_item()", "cursor.finish()"], "P5a image consumer is not retained by scalar, asset request, and output item phase");
  if ((retainedSceneBoundary.match(/cursor\.advance_byte\(\)/g)?.length ?? 0) !== 2 || (retainedImageBoundary.match(/cursor\.advance_byte\(\)/g)?.length ?? 0) !== 2)
    failures.push("P5a component scene and image consumers must each retain both scalar byte phases");
  for (const forbidden of ["resolve_ui_image(", "resolve_ui_image_svg(", "queue_canvas_image_upload_sized("])
    if (retainedImageBoundary.includes(forbidden)) failures.push(`P5a retained image consumer reaches complete decode ${forbidden}`);
  requireAll(atlasBoundary, [
    "PREPARED_ATLAS_PAGE_BYTES: usize = 16 * 1024",
    "PREPARED_ATLAS_PAGE_CAPACITY: usize = 2_048",
    "PREPARED_ATLAS_PROCESS_ITEMS: usize = 64",
    "PREPARED_ATLAS_PROCESS_PAGES: usize = 4_096",
    "PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES: usize = 64 * 1024 * 1024",
    "PREPARED_ATLAS_PROCESS_BACKING_BYTES: usize = 96 * 1024 * 1024",
    "static PREPARED_ATLAS_PROCESS_PERMITS: AtomicU64",
    "prepared_atlas_field(current, PREPARED_ATLAS_ITEM_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_PAGE_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_PAYLOAD_SHIFT",
    "prepared_atlas_field(current, PREPARED_ATLAS_BACKING_SHIFT",
    "struct PreparedAtlasPermit",
    "items: usize",
    "pages: usize",
    "payload_bytes: usize",
    "backing_bytes: usize",
    "PREPARED_ATLAS_PROCESS_PERMITS.compare_exchange(current, next",
    "slots: Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>",
    "let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes)",
    "Box::new([const { None }; PREPARED_ATLAS_PAGE_CAPACITY])",
    "Box::new([0; PREPARED_ATLAS_PAGE_BYTES])",
    "pub fn close_step(&mut self) -> bool",
    "slots[index] = None",
    "permit.release_step()",
    "pub fn close_abandoned_step() -> bool",
    "impl Drop for PreparedAtlasPages",
    "PreparedAtlasAbandonment",
  ], "P5a atlas authority lacks nonblocking exact item/page/payload/backing permits or retained abandonment close");
  const atlasCredit = atlasBoundary.indexOf("let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes)");
  const atlasSlots = atlasBoundary.indexOf("Box::new([const { None }; PREPARED_ATLAS_PAGE_CAPACITY])");
  const atlasPage = atlasBoundary.indexOf("Box::new([0; PREPARED_ATLAS_PAGE_BYTES])");
  if (atlasCredit < 0 || atlasSlots < atlasCredit || atlasPage < atlasCredit) failures.push("P5a atlas backing allocates before process credit transfer");
  for (const forbidden of ["PREPARED_ATLAS_PROCESS_LEDGER", "Mutex<usize>", ".lock()", "Vec<PreparedAtlasPage>", "Vec::with_capacity(byte_len)", "Vec::with_capacity(source.len())", ".truncate(", ".fill(None)", "while ", "loop {"])
    if (atlasBoundary.includes(forbidden)) failures.push(`P5a atlas authority retains whole-backing work ${forbidden}`);
  requireAll(preparedSource, [
    "atlas_process_item_max_plus_one_is_nonblocking_and_recovers_every_permit",
    "abandoned_atlas_schedules_the_same_incremental_close_authority",
    "interrupted_atlas_close_rejoins_the_same_abandonment_authority",
    "atlas_allocation_refusal_preserves_the_packed_permit_ledger",
    "atlas_contended_permit_attempts_are_nonblocking_and_poison_free",
  ], "P5a atlas MAX + 1, allocation refusal, contention, abandonment, or interrupted-close law is missing");
  requireAll(atlasGpuBoundary, ["struct PreparedAtlasUploadCursor", "GlyphAtlasPages", "IconAtlasPages", "upload_glyph_atlas_page", "upload_icon_atlas_page", "cursor.page.checked_add(1)"], "P5a GPU atlas upload is not one retained page per grant");
  if ((atlasGpuBoundary.match(/cursor\.page\.checked_add\(1\)/g)?.length ?? 0) !== 2) failures.push("P5a glyph and icon atlas consumers must each advance exactly one page per grant");
  requireAll(atlasDrawBoundary, ["upload_glyph_atlas_page", "upload_icon_atlas_page", "origin: wgpu::Origin3d", "rows_per_image: Some(rows)"], "P5a atlas page upload boundary is incomplete");
  for (const forbidden of ["Vec::with_capacity(self.icons.pixels.len())", "Vec::with_capacity(self.atlas.pixels.len())", "keys().next().cloned()"])
    if (glue.includes(forbidden) || shell.includes(forbidden)) failures.push(`P5a mounted child authority retains uncredited whole owner ${forbidden}`);
  requireAll(engineBoundary, [
    "ENGINE_CANVAS_FRAME_PACKET_CAPACITY: usize = 256",
    "packets: Box<[Option<EngineCanvasPacket>; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]>",
    "take_packet_step",
    "Result<Option<EngineCanvasPacket>, EngineCanvasPacket>",
    "terminal_is_empty",
  ], "P5a EngineCanvas packet producer is not fixed, fallible, and one-owner resumable");
  for (const forbidden of ["packets: Vec<EngineCanvasPacket>", "take_packets(", "std::mem::take(&mut self.packets)"])
    if (engineBoundary.includes(forbidden)) failures.push(`P5a EngineCanvas boundary retains dynamic or bulk packet staging ${forbidden}`);
  requireAll(worldBoundary, [
    "WORLD3D_FRAME_RESOURCE_CAPACITY: usize = 256",
    "uploads: Box<[Option<PreparedRenderUpload>; WORLD3D_FRAME_RESOURCE_CAPACITY]>",
    "evictions: Box<[Option<PreparedRenderEviction>; WORLD3D_FRAME_RESOURCE_CAPACITY]>",
    "pub fn append_step",
    "Result<bool, World3dBuildRejected>",
    "checked_add(input.uploads.len())",
    "pub fn terminal_is_empty",
  ], "P5a World3D resource producer is not fixed, fallible, and one-owner resumable");
  for (const forbidden of ["uploads: Vec<", "evictions: Vec<", "mesh_requests: HashSet", "raster_requests: HashSet", "append_to(", ".append(", ".clear()"])
    if (worldBoundary.includes(forbidden)) failures.push(`P5a World3D boundary retains dynamic or bulk resource staging ${forbidden}`);
  requireAll(transaction, [
    "pub(crate) fn close_step(&mut self) -> bool",
    "pub(crate) fn terminal_is_empty(&self) -> bool",
  ], "P5a retained close and terminal witness are incomplete");
  requireAll(frameJob, [
    "WorkerJobSession::try_new",
    "crate::FrameTransaction::new",
    "try_submit_step(&crate::renderer_worker_pool(), Lane::Interactive)",
    "session.take_rejected()",
    "rejected.resume()",
    "session.take_terminal()",
    "session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)",
    "transaction.close_step() && transaction.terminal_is_empty()",
    "generation_is_fresh(generation, frame_generation)",
  ], "P5a mounted shared-pool take/resume/close/terminal laws are incomplete");
  if ((frameJob.match(/session\.take_rejected\(\)/g)?.length ?? 0) !== 3 || (frameJob.match(/rejected\.resume\(\)/g)?.length ?? 0) !== 3 || (frameJob.match(/session\.take_terminal\(\)/g)?.length ?? 0) !== 2)
    failures.push("P5a every mounted admission/refusal/terminal lane must retain its exact take/resume owner protocol");
  for (const forbidden of ["try_step_on_caller", "WorkerPool::new(", "thread::spawn", "loop {"]) if (frameJob.includes(forbidden)) failures.push(`P5a mounted worker path contains forbidden ${forbidden}`);
  requireAll(host, [
    "fn advance_frame_generation",
    "generation.checked_add(1)",
    "observe_presentation_input_generation(build_generation.0)",
    "AppPresentStep::Complete { generation, cursor, fullscreen, cursor_wake }",
    "if generation.0 != self.frame_generation",
    "self.snapshot_sink.publish",
  ], "P5a host does not fail closed on exhaustion/stale publication or atomically publish the accepted frame");
  requireAll(hostSource, ["mounted_pointer_storm_callback_p99_stays_below_two_milliseconds", "mounted_resize_storm_callback_p99_stays_below_two_milliseconds", "mounted_frame_generation_exhaustion_is_permanent_and_non_wrapping"], "P5a mounted input/resize/latency/exhaustion laws are missing");
  if (host.includes("frame_generation.wrapping_add") || host.includes("snapshot_sink.publish(crate::render_snapshot::RenderSnapshot::new(revision, generation, timestamp_us, cursor")) failures.push("P5a host retains wrapping generation or callback-local snapshot publication");
  requireAll(snapshot, ["pub fn next_revision(&self) -> Option<u64>", "current.checked_add(1)?", "compare_exchange(current, next"], "P5a snapshot revision does not permanently fail closed at exhaustion");
  if (!snapshotSource.includes("revision_exhaustion_is_permanent_and_preserves_last_valid_snapshot")) failures.push("P5a last-valid snapshot exhaustion law is missing");
  if (uiRuntimeGlue.includes("mod transaction") || uiRuntimeGlue.includes("pub use transaction::*")) failures.push("P5a dormant headless transaction is production reachable beside the mounted authority");
  return failures;
}


//#endregion 🔄️P5aMountedFrameTransaction
