/** 📖️ The production source reads and the boundary definitions the mounted frame transaction checks scan. */
import { interactivityProductionSource } from "../../../🗄️database/🟦️.ts";
import { INTERACTIVITY_AUDIT_FILE_MARKER } from "../../../🗂️files/🟦️.ts";
import { toolJobRustBlock } from "../../../../🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { interactivityCheckedRustBlockScope } from "../../🧭️checked-rust-block-scope/🟦️.ts";

/** 🧱️ The first item declared at `marker`, from its header through its matching closing brace, with literals and comments kept for needle checks; empty when absent. */
export function rustItem(source: string, marker: string): string {
  const start = source.indexOf(marker);
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  return block === undefined ? "" : source.slice(start, block.end);
}

/** 🗂️ The whole text of the first source file that holds `marker`, cut at the file markers of the joined tree; empty when absent. */
export function fileText(source: string, marker: string): string {
  const at = source.indexOf(marker);
  if (at < 0) return "";
  const start = source.lastIndexOf(INTERACTIVITY_AUDIT_FILE_MARKER, at);
  const end = source.indexOf(INTERACTIVITY_AUDIT_FILE_MARKER, at);
  return source.slice(start < 0 ? 0 : start, end < 0 ? undefined : end);
}

/** ✂️ Splits every block opened at `marker` out of `source`: what is left for the field-page law and what was cut for the bounded-document law. */
export function splitRustBranches(source: string, marker: string): { readonly kept: string; readonly branches: string } {
  let kept = source;
  const branches: string[] = [];
  for (let at = kept.indexOf(marker); at >= 0; at = kept.indexOf(marker)) {
    const block = toolJobRustBlock(kept, kept.indexOf("{", at));
    if (block === undefined) break;
    branches.push(kept.slice(at, block.end));
    kept = kept.slice(0, at) + kept.slice(block.end);
  }
  return { kept, branches: branches.join("\n") };
}

/** 🧱️ The items declared at each marker, joined in marker order. */
export function rustItems(source: string, ...markers: readonly string[]): string {
  return markers.map((marker) => rustItem(source, marker)).join("\n");
}

/** 📚 Reads the production text of every checked owner source. */
export function readProductionSources({ glueSource, frameJobSource, hostSource, snapshotSource, uiRuntimeGlueSource, shellSource, engineCanvasSource, world3dSource, preparedSource, gpuSource, drawSource, interpreterSource, uiEngineSource, paintSource, sceneSlotsSource, scenesSource, servicesSource }: {
  readonly glueSource: string;
  readonly frameJobSource: string;
  readonly hostSource: string;
  readonly snapshotSource: string;
  readonly uiRuntimeGlueSource: string;
  readonly shellSource: string;
  readonly engineCanvasSource: string;
  readonly world3dSource: string;
  readonly preparedSource: string;
  readonly gpuSource: string;
  readonly drawSource: string;
  readonly interpreterSource: string;
  readonly uiEngineSource: string;
  readonly paintSource: string;
  readonly sceneSlotsSource: string;
  readonly scenesSource: string;
  readonly servicesSource: string;
}) {
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
  return { glue, shell, engineCanvas, world3d, prepared, gpu, draw, uiEngine, paint, sceneSlots, interpreter, scenes, services, frameJob, host, snapshot, uiRuntimeGlue };
}

/** 🛠 Defines the failure list, the scope helpers and the maintenance and transaction boundaries. */
export function defineMaintenanceBoundaries({ glue, glueSource }: {
  readonly glue: string;
  readonly glueSource: string;
}) {
  const failures: string[] = [];
  const scope = (source: string, marker: string, name: string) => interactivityCheckedRustBlockScope(source, marker, name, failures);
  const scopes = (...parts: (string | undefined)[]) => parts.every((part) => part !== undefined) ? parts.join("\n") : undefined;
  const transaction = glue.slice(glue.indexOf("pub struct FrameTransaction {"), glue.indexOf("pub async fn boot_runtime("));
  const frameCursors = rustItems(glue, "pub struct FrameBuildCursor", "pub struct FrameFinishCursor");
  const buildBoundary = glue.slice(glue.indexOf("fn frame_before_input_step"), glue.indexOf("fn frame_after_input_step"));
  const finishBoundary = glue.slice(glue.indexOf("fn frame_after_input_step"), glue.indexOf("//#region 🔖️OsHostDecomposition — SemioApp deletion"));
  const deferredBoundary = glue.slice(glue.indexOf("fn start_frame_deferred"), glue.indexOf("fn apply_step", glue.indexOf("fn start_frame_deferred")));
  const maintenanceOwnerBoundary = rustItems(glue, "pub struct FrameDeferredCursor", "impl FrameDeferredCursor", "pub struct FrameMaintenanceOwner", "impl FrameMaintenanceOwner", "pub struct FrameExecutionOwnerCell<T>", "impl<T> FrameExecutionOwnerCell<T>", "pub struct FrameMaintenanceRefusal", "impl FrameMaintenanceRefusal", "pub struct FrameMaintenanceAuthority", "impl FrameMaintenanceAuthority");
  const maintenanceNativeOwnerBoundary = rustItems(glueSource, "pub struct FrameDeferredCursor", "impl FrameDeferredCursor");
  const maintenanceCloseBoundary = scope(maintenanceNativeOwnerBoundary, "fn begin_close(&mut self)", "native maintenance close");
  const maintenanceRegistryBoundary = rustItems(glue, "pub struct FrameExecutionRegistry<T>", "impl<T> FrameExecutionRegistry<T>");
  const maintenanceExecutionBoundary = [maintenanceRegistryBoundary, rustItems(glue, "pub struct FrameMaintenanceExecutionEnvelope<R>", "impl<R> FrameMaintenanceExecutionEnvelope<R>", "impl<R> Drop for FrameMaintenanceExecutionEnvelope<R>", "pub struct FrameMaintenanceExecutionGuard<R>", "impl<R> FrameMaintenanceExecutionGuard<R>", "impl<R> Drop for FrameMaintenanceExecutionGuard<R>"), glue.slice(glue.indexOf("pub frame_maintenance_executions:"), glue.indexOf("pub frame_deferred_executions:"))].join("\n");
  const maintenanceSpawnBoundary = glue.slice(glue.indexOf("fn try_spawn_frame_maintenance_reserved"), glue.indexOf("#[cfg(target_arch = \"wasm32\")]", glue.indexOf("fn try_spawn_frame_maintenance_reserved")));
  const maintenanceTerminalStart = glue.indexOf("fn frame_maintenance_terminal_fault");
  const maintenanceTerminalBoundary = glue.slice(maintenanceTerminalStart, glue.indexOf("pub const WORLD3D_DEADLINE_CAPACITY", maintenanceTerminalStart < 0 ? 0 : maintenanceTerminalStart));
  const maintenanceWorkerTerminalStart = maintenanceSpawnBoundary.indexOf("if let Some(fault) = frame_maintenance_terminal_fault");
  const maintenanceWorkerTerminalBoundary = maintenanceSpawnBoundary.slice(maintenanceWorkerTerminalStart, maintenanceSpawnBoundary.indexOf("            } else {", maintenanceWorkerTerminalStart));
  return { scopes, scope, failures, transaction, frameCursors, buildBoundary, finishBoundary, deferredBoundary, maintenanceOwnerBoundary, maintenanceNativeOwnerBoundary, maintenanceCloseBoundary, maintenanceExecutionBoundary, maintenanceRegistryBoundary, maintenanceSpawnBoundary, maintenanceTerminalBoundary, maintenanceWorkerTerminalBoundary };
}

/** 🪟 Defines the chrome, engine, world, atlas and shell child boundaries. */
export function defineChromeAndEngineBoundaries({ scopes, scope, shell, engineCanvas, world3d, prepared, gpu, draw, interpreterSource, uiEngine }: {
  readonly scopes: (...parts: (string | undefined)[]) => string | undefined;
  readonly scope: (source: string, marker: string, name: string) => string | undefined;
  readonly shell: string;
  readonly engineCanvas: string;
  readonly world3d: string;
  readonly prepared: string;
  readonly gpu: string;
  readonly draw: string;
  readonly interpreterSource: string;
  readonly uiEngine: string;
}) {
  const chromeBoundary = scopes(
    scope(shell, "pub struct ShellChromeFrameCursor", "Shell chrome cursor"),
    scope(shell, "pub enum ShellChromeFramePhase", "Shell chrome phase"),
    scope(shell, "pub fn render_chrome_step", "Shell chrome step"),
  );
  const engineBoundary = fileText(engineCanvas, "pub const ENGINE_CANVAS_FRAME_PACKET_CAPACITY");
  const worldBoundary = fileText(world3d, "const WORLD3D_FRAME_RESOURCE_CAPACITY");
  const atlasPermits = fileText(prepared, "pub const PREPARED_ATLAS_PAGE_BYTES");
  const atlasPages = fileText(prepared, "impl Drop for PreparedAtlasPages");
  const atlasBoundary = `${atlasPermits.slice(atlasPermits.indexOf("pub const PREPARED_ATLAS_PAGE_BYTES"))}\n${atlasPages.slice(0, atlasPages.indexOf("impl PreparedRasterPages"))}`;
  const atlasGpuBoundary = `${rustItem(gpu, "struct PreparedAtlasUploadCursor")}\n${fileText(gpu, "cursor.page.checked_add(1)")}`;
  const atlasDrawBoundary = fileText(draw, "pub fn upload_glyph_atlas_page");
  const chromeMainBoundary = scope(shell, "pub fn render_main_window_step", "Shell main window");
  const chromePanelBoundary = scope(shell, "pub fn render_panel_step", "Shell panel");
  const chromeNavbarBoundary = scope(shell, "pub fn render_navbar_step", "Shell navbar");
  const chromeTutorialBoundary = scope(shell, "pub fn render_tutorial_bar_step", "Shell tutorial");
  const chromeFooterBoundary = scope(shell, "pub fn render_footer_step", "Shell footer");
  const chromeOverlayStepBoundary = scope(shell, "pub fn render_overlay_step", "Shell overlay");
  const chromeContextMenuBoundary = scope(shell, "pub fn render_context_menu_levels_step", "Shell context menu");
  const chromeTooltipBoundary = scope(shell, "pub fn render_chrome_tooltip_step", "Shell tooltip");
  const chromeDialogBoundary = scope(shell, "pub fn render_chrome_dialog_step", "Shell dialog");
  const chromeTourBoundary = scope(shell, "pub fn render_chrome_tour_step", "Shell tour");
  const chromeOverlayBoundary = scopes(chromeOverlayStepBoundary, chromeContextMenuBoundary, chromeTooltipBoundary, chromeDialogBoundary, chromeTourBoundary);
  const shellChildren = scopes(chromeMainBoundary, chromePanelBoundary, chromeNavbarBoundary, chromeTutorialBoundary, chromeFooterBoundary, chromeOverlayBoundary);
  const documentBoundary = interactivityProductionSource(interpreterSource.slice(interpreterSource.indexOf("pub struct UiDocumentFrameCursor"), interpreterSource.indexOf("pub fn render_ui_document(")));
  const uiFrameBoundary = `${fileText(uiEngine, "const RETAINED_PAINT_DEPTH_CREDITS")}\n${fileText(uiEngine, "pub fn frame_into_step")}`;
  return { chromeBoundary, shellChildren, chromeDialogBoundary, chromeTourBoundary, chromeTutorialBoundary, chromeFooterBoundary, chromeOverlayBoundary, chromeNavbarBoundary, documentBoundary, uiFrameBoundary, atlasBoundary, atlasGpuBoundary, atlasDrawBoundary, engineBoundary, worldBoundary };
}

/** 🎨 Defines the paint node, interactive sync, scene, image, find, maintenance and chrome text boundaries. */
export function definePaintSceneAndTextBoundaries({ scopes, scope, paint, sceneSlots, interpreter, scenes, shell }: {
  readonly scopes: (...parts: (string | undefined)[]) => string | undefined;
  readonly scope: (source: string, marker: string, name: string) => string | undefined;
  readonly paint: string;
  readonly sceneSlots: string;
  readonly interpreter: string;
  readonly scenes: string;
  readonly shell: string;
}) {
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
    ...["text", "stack", "separator", "button", "input", "select", "toggle", "key_value", "slider", "number_stepper", "ring", "progress", "icon_select", "field", "section", "group", "image", "component_scene", "external_slot"].map((kind) => scope(paint, `fn paint_${kind}_node_step(`, `paint ${kind} node step`)),
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
  const sceneHostBoundary = rustItem(interpreter, "impl semio_ui_wgpu_hosting::scene_slots::SceneHost for FrameworkSceneHost");
  const retainedImageBoundary = rustItem(interpreter, "pub fn render_ui_image_step");
  const retainedSceneBoundary = rustItem(scenes, "pub fn render_component_scene_step");
  const findStart = shell.indexOf("pub struct ShellFindItems {");
  const findEnd = shell.indexOf("collector.try_push_at(active.generation, item)", findStart);
  const findBoundary = shell.slice(findStart, findEnd < 0 ? undefined : findEnd + "collector.try_push_at(active.generation, item)".length);
  const maintenanceBoundary = shell;
  return { findBoundary, maintenanceBoundary, interactiveSyncBoundary, paintNodeBoundary, sceneNodeBoundary, sceneHostBoundary, retainedSceneBoundary, retainedImageBoundary };
}

/** 🔖 Defines the chrome group, preference and fixed file page boundaries and the require helpers. */
export function defineGroupPreferenceAndRequireHelpers({ scopes, scope, shell, services, failures }: {
  readonly scopes: (...parts: (string | undefined)[]) => string | undefined;
  readonly scope: (source: string, marker: string, name: string) => string | undefined;
  readonly shell: string;
  readonly services: string;
  readonly failures: string[];
}) {
  const chromeTextBoundary = scopes(
    scope(shell, "pub fn chrome_text_step", "Shell chrome text step"),
    scope(shell, "pub fn chrome_text_complete_step", "Shell chrome text completion"),
  );
  const chromeGroupWidthBoundary = scope(shell, "pub fn retained_chrome_group_item_width", "Shell chrome group width");
  const chromeGroupStepBoundary = scopes(
    scope(shell, "pub enum RetainedChromeGroupStep", "Shell chrome group phase"),
    scope(shell, "pub fn render_retained_chrome_group_item_step(", "Shell chrome group step"),
    scope(shell, "fn render_retained_chrome_group_item_step_with_trailing", "Shell chrome group trailing step"),
  );
  const chromeGroupBoundary = scopes(chromeGroupWidthBoundary, chromeGroupStepBoundary);
  const prefsBoundary = shell.slice(shell.indexOf("fn native_pref_field_path"), shell.indexOf("//#endregion 🗄️PrefsStore"));
  const nativePrefsBoundary = prefsBoundary.slice(0, prefsBoundary.indexOf("fn prefs_get("));
  const fixedFilePageBoundary = services.slice(services.indexOf("//#region 📄️FixedFilePage"), services.indexOf("//#endregion 📄️FixedFilePage"));
  const requireAll = (source: string | undefined, needles: readonly string[], label: string) => {
    if (source === undefined) return;
    const present = (needle: string) => source.includes(needle) || (needle.startsWith("pub(crate) ") && source.includes(`pub ${needle.slice("pub(crate) ".length)}`));
    if (needles.some((needle) => !present(needle))) failures.push(label);
  };
  const requireExactlyOne = (source: string | undefined, needle: string, label: string) => {
    if (source === undefined) return;
    if (source.split(needle).length - 1 !== 1) failures.push(label);
  };
  const { kept: nativeFieldPageBoundary, branches: nativeConfigDocumentBoundary } = splitRustBranches(nativePrefsBoundary, "if key == OS_SHELL_CONFIG_STORAGE_KEY {");
  return { requireAll, prefsBoundary, fixedFilePageBoundary, nativePrefsBoundary, nativeFieldPageBoundary, nativeConfigDocumentBoundary, chromeTextBoundary, chromeGroupBoundary, chromeGroupStepBoundary };
}
