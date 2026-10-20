/** 🔶️ The reconcile phase of interactivityLiveReconcileFailures. */
import { interactivityProductionSource } from "../../🗄️database/🟦️.ts";

/** 🔷 Builds reconcile, patches, reactor, value of interactivityLiveReconcileFailures. */
export function buildReconcileState({ reconcileSource, patchesSource, reactorSource, valueSource, schemaSource }: { readonly reconcileSource: string; readonly patchesSource: string; readonly reactorSource: string; readonly valueSource: string; readonly schemaSource: string }) {
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
  return { schema, reactor, failures, reconcile, turnPatchBridgeStart, turnPatchBridge, patches, driveStart, drive, trackerDefinitions, rendererRetainedStart, rendererRetained, shellDocumentStart, shellDocuments, shellRefresh, commandDocumentStart, commandExchangeStart, commandDocuments, commandExchange, rendererSequenceStart, rendererSequence, windowMeasureStart, windowMeasure };
}
