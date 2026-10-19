/** 📖️ The production source reads and the boundary definitions the mounted frame transaction checks scan. */
import { interactivityProductionSource } from "../../../🗄️database/🟦️.ts";
import { interactivityCheckedRustBlockScope } from "../../🧭️checked-rust-block-scope/🟦️.ts";

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
  const sceneHostBoundary = interpreter.slice(interpreter.indexOf("impl ui_wgpu::wgpu::SceneHost for FrameworkSceneHost"), interpreter.indexOf("fn drive_mounted_layout_text_one"));
  const retainedImageBoundary = interpreter.slice(interpreter.indexOf("fn render_ui_image_step"), interpreter.indexOf("fn render_ui_image("));
  const retainedSceneBoundary = scenes.slice(scenes.indexOf("pub fn render_component_scene_step"), scenes.indexOf("pub fn render_component_scene("));
  const findBoundary = shell.slice(shell.indexOf("pub struct ShellFindItems"));
  const maintenanceBoundary = shell.slice(shell.indexOf("struct ShellChromeMaintenance"), shell.indexOf("fn body_rect"));
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
  return { requireAll, prefsBoundary, fixedFilePageBoundary, nativePrefsBoundary, chromeTextBoundary, chromeGroupBoundary, chromeGroupStepBoundary };
}
