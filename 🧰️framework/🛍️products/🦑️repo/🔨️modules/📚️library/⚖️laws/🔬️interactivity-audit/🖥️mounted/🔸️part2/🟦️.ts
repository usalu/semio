/** 🔸️ The part2 phase of interactivityLiveReconcileFailures. */
import type { buildReconcileState } from "../🔶️reconcile/🟦️.ts";

/** 🔹 Runs the effects and registrations of interactivityLiveReconcileFailures. */
export function buildPart2({ schema, reactor, schemaSource, failures, reconcile, turnPatchBridgeStart, turnPatchBridge }: Pick<ReturnType<typeof buildReconcileState>, "schema" | "reactor" | "failures" | "reconcile" | "turnPatchBridgeStart" | "turnPatchBridge"> & { readonly schemaSource: string }) {
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
}
