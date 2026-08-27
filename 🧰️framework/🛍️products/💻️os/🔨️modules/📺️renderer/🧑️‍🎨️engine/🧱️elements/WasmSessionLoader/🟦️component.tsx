// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/WasmSessionLoader/component.tsx
/** @emoji 🕸️ `WasmSessionLoader` — the lazy wasm-pack module loader table (`createEngineSession`) and
 * per-surface session factories (`createGraphSession`/`createFlowSession`/`createEditorSession`/
 * `createRasterSession`/`createMapSession`/`createTerrainSession`/`createBoard2dSession`), plus the
 * shared render-on-demand `createDemandFrameScheduler` and the `isFlowGraphScene` capability sniff.
 * Each `*Host` component below (NodeGraph/TextEditor/TiledMapHost/Board2dHost/Paint2dHost/
 * WorldTerrainLayer) leases exactly one session kind through this table.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type GraphWasmSession } from "@semio-tech/infinite-canvas-react-renderer";
// #endregion 🔌️Adapters

//#region 🔖️wasm-session-loader

//#region 🔖️EngineSessionLoader
/** 🔌️ One generic lazy-loader table for every `framework/surface/*` engine crate (plus `board-2d`,
 * still app-hosted) — each surface kind maps to its wasm-pack package; a single cached module promise
 * per kind replaces the five near-identical hand-rolled `let xPromise…create X()` blocks this used to be. */
type EngineSessionWasmModule = { readonly default: (input?: unknown) => Promise<unknown> } & Record<string, new () => unknown>;

const ENGINE_SESSION_IMPORTERS: Record<string, () => Promise<EngineSessionWasmModule>> = {
  "board-2d": () => import("@semio-tech/framework-surface-rs"),
};

const engineSessionModulePromises = new Map<string, Promise<EngineSessionWasmModule>>();

/** 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: deliberately NOT wired through the framework core's
 * `createLeasePool` — this cache's `dispose` would be a no-op. A wasm-bindgen ES-module engine is a
 * module-scope singleton in its realm (`node_modules/@semio-tech/flow-core/flow_core.js`'s cached
 * `wasm`/`wasmInstance`, `__wbg_init`/`initSync` both early-return once set); there is no deinit
 * export, and `WebAssembly.Memory` never shrinks. Once instantiated, an engine's compiled code and
 * memory high-water mark are pinned for the document's lifetime — that ceiling is real and cannot be
 * evicted from here. The actual mitigation is prompt `session.free()` on every consuming surface's
 * unmount (see each `*Host` component below) so the wasm-side allocator reuses freed pages instead of
 * growing further; that keeps the ceiling from climbing, even though it can't lower it. True unload
 * requires hosting an engine in a dedicated Worker whose `terminate()` actually releases the isolate —
 * a larger follow-up, out of this slice. */
async function createEngineSession<TSession>(engineKind: keyof typeof ENGINE_SESSION_IMPORTERS, sessionClassName: string): Promise<TSession> {
  let modulePromise = engineSessionModulePromises.get(engineKind);
  if (!modulePromise) {
    modulePromise = ENGINE_SESSION_IMPORTERS[engineKind]().then(async (mod) => {
      await mod.default();
      return mod;
    });
    engineSessionModulePromises.set(engineKind, modulePromise);
  }
  const mod = await modulePromise;
  return new mod[sessionClassName]() as TSession;
}
//#endregion 🔖️EngineSessionLoader

//#region 🗺️SharedSurfaceSessionLoader
type SurfaceSessionModule = typeof import("@semio-tech/framework-surface-rs");
let surfaceSessionModulePromise: Promise<SurfaceSessionModule> | undefined;

/** 🗺️ Initializes the actual shared surface module before invoking a compiler-checked constructor. */
async function createSurfaceSession<T>(construct: (module: SurfaceSessionModule) => T): Promise<T> {
  surfaceSessionModulePromise ??= import("@semio-tech/framework-surface-rs").then(async (module) => {
    await module.default();
    return module;
  });
  return construct(await surfaceSessionModulePromise);
}
//#endregion 🗺️SharedSurfaceSessionLoader

//#region 🔖️DemandFrameScheduler
/** 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: shared render-on-demand scheduler for wasm engine
 * surfaces that used to run an unconditional `requestAnimationFrame` loop forever (flow node-graph,
 * tiled-map, paint-2d, board-2d) — holding the tab at 60fps even fully idle, which also prevents the
 * browser from throttling/backgrounding it. Mirrors the pattern already used correctly elsewhere in
 * this file: `WasmGraphSurface` renders only from event handlers, and the r3f world uses
 * `frameloop="demand"` + explicit `invalidate()` (`♾️infinite/🌍️world/🎨️r3f/…/📦️index.tsx`).
 * `invalidate()` schedules a render on the next frame, then keeps rendering for `trailingWindowMs`
 * after the LAST invalidate — this absorbs wasm-side eased/animated state (e.g. a spring settling)
 * without needing a per-engine "is something still animating" query. `beginContinuous`/`endContinuous`
 * cover genuinely continuous work (an active pointer gesture, a running compute progress indicator)
 * where invalidate-per-event would be too coarse. */
export function createDemandFrameScheduler(render: () => void, opts?: { readonly trailingWindowMs?: number }): { invalidate(): void; beginContinuous(reason: string): void; endContinuous(reason: string): void; dispose(): void } {
  const trailingWindowMs = opts?.trailingWindowMs ?? 250;
  const continuousReasons = new Set<string>();
  let raf = 0;
  let trailingUntil = 0;
  let disposed = false;

  const tick = () => {
    raf = 0;
    if (disposed) return;
    render();
    if (continuousReasons.size > 0 || Date.now() < trailingUntil) {
      raf = requestAnimationFrame(tick);
    }
  };
  const ensureScheduled = () => {
    if (disposed || raf !== 0) return;
    raf = requestAnimationFrame(tick);
  };

  return {
    invalidate() {
      trailingUntil = Date.now() + trailingWindowMs;
      ensureScheduled();
    },
    beginContinuous(reason: string) {
      continuousReasons.add(reason);
      ensureScheduled();
    },
    endContinuous(reason: string) {
      continuousReasons.delete(reason);
      if (continuousReasons.size === 0) trailingUntil = Date.now() + trailingWindowMs;
    },
    dispose() {
      disposed = true;
      continuousReasons.clear();
      if (raf) cancelAnimationFrame(raf);
      raf = 0;
    },
  };
}
//#endregion 🔖️DemandFrameScheduler

//#region GraphSession
export async function createGraphSession(): Promise<GraphWasmSession> {
  return createSurfaceSession((module) => new module.GraphSession());
}
//#endregion GraphSession

//#region FlowSession
export type { FlowTask, FlowTaskEvent } from "@semio-tech/flow-core/🟨️flow-browser.js";
export type FlowWasmSession = import("@semio-tech/flow-core/🟨️flow-browser.js").FlowSession;
type FlowSessionModule = typeof import("@semio-tech/flow-core/🟨️flow-browser.js");

let flowSessionPromise: Promise<FlowSessionModule> | null = null;

export async function createFlowSession(): Promise<FlowWasmSession> {
  if (!flowSessionPromise) {
    flowSessionPromise = Promise.all([
      import("@semio-tech/flow-core"),
      import("@semio-tech/flow-core/🟨️flow-browser.js"),
    ]).then(async ([core, browser]) => {
      const exports = await core.default();
      await browser.default(exports);
      return browser;
    });
  }
  const mod = await flowSessionPromise;
  return new mod.FlowSession();
}
//#endregion FlowSession

//#region EditorSession
export type EditorWasmSession = GraphWasmSession & {
  syncFromSceneJson(json: string): void;
  syncFromScenePack?(bytes: Uint8Array): void;
  setText(text: string): void;
  text(): string;
  caret(): number;
  anchor(): number;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number, buttons: number): void;
  pointerUpScreen(sx: number, sy: number, buttons: number): void;
  wheelScrollScreen(deltaY: number): void;
  insertText(text: string): void;
  backspace(): void;
  deleteForward(): void;
  selectAll(): void;
  replaceSelection(text: string): void;
  selectionText(): string;
  setCanvasThemeJson(json: string): void;
  hoverTokenRangeJson(): string;
  setHoverRange(start: number, end: number): void;
  cameraJson(): string;
  moveLeft(extend: boolean): void;
  moveRight(extend: boolean): void;
  moveUp(extend: boolean): void;
  moveDown(extend: boolean): void;
  moveLineStart(extend: boolean): void;
  moveLineEnd(extend: boolean): void;
  tabInsertText(): string;
  setSelectionRange(anchor: number, caret: number): void;
  selectSpanAt(offset: number): void;
  selectSpanAtScreen(sx: number, sy: number): void;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  caretWorldJson(): string;
  worldToScreenJson(wx: number, wy: number): string;
  setSelectionOccurrencesJson(json: string): void;
  setExtraCaretsJson(json: string): void;
  setCaretVisible(visible: boolean): void;
};

type EditorSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly EditorSession: new () => EditorWasmSession;
};

let editorSessionPromise: Promise<EditorSessionModule> | null = null;

export async function createEditorSession(): Promise<EditorWasmSession> {
  if (!editorSessionPromise) {
    editorSessionPromise = import("@semio-tech/framework-editor-rs").then(async (mod) => {
      await mod.default();
      return mod as EditorSessionModule;
    });
  }
  const mod = await editorSessionPromise;
  return new mod.EditorSession();
}
//#endregion EditorSession

//#region RasterSession
export type RasterWasmSession = {
  gpuReady(): boolean;
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  setCamera(x: number, y: number, zoom: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  syncDocumentJson(json: string): void;
  uploadLayerImage(layerId: string, bytes: Uint8Array): void;
  uploadRasterImageKey(key: string, bytes: Uint8Array): void;
  setActiveUtility(utility: string): void;
  setBrushSize(size: number): void;
  setBrushOpacity(opacity: number): void;
  syncInteraction(selectedIdsJson: string, hoveredId?: string | null): void;
  setCanvasThemeJson(json: string): void;
  cameraJson(): string;
  setViewMode(mode: string, layerId?: string | null): void;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  marqueeHitsJson(queryJson: string): string;
  navigatorFitCameraJson(viewportW: number, viewportH: number): string;
  navigatorViewportOverlayJson(contentCameraJson: string, contentViewportJson: string): string;
  free(): void;
};

export async function createRasterSession(): Promise<RasterWasmSession> {
  return createSurfaceSession((module) => new module.RasterSession());
}
//#endregion RasterSession

//#region MapSession
export type MapWasmSession = {
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  setCamera(x: number, y: number, zoom: number): void;
  cameraJson(): string;
  cameraLimitsJson(): string;
  fitWorldCamera(): void;
  reclampCamera(): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  syncMapJson(json: string): void;
  uploadTile(z: number, x: number, y: number, bytes: Uint8Array): void;
  uploadVectorTile(z: number, x: number, y: number, bytes: Uint8Array): void;
  visibleTilesJson(): string;
  visibleVectorTilesJson(): string;
  setRenderMode(mode: string): void;
  setVectorStyle(style: string): void;
  setLodMode(mode: string): void;
  setLayerVisibilityJson(json: string): void;
  setLayerStrokeScaleJson(json: string): void;
  syncInteraction(granularity: string, selectedIdsJson: string, hoveredId?: string): void;
  featuresInRectJson(x0: number, y0: number, x1: number, y1: number, crossing: boolean): string;
  featuresInPolygonJson(pointsJson: string, crossing: boolean): string;
  hitTestFeatureJson(sx: number, sy: number): string;
  featureScreenJson(kind: string, id: string): string;
  positionScreenJson(id: string): string;
  currentLodJson(): string;
  setMapThemeJson(json: string): void;
  gpuReady(): boolean;
  free(): void;
};

export async function createMapSession(): Promise<MapWasmSession> {
  return createSurfaceSession((module) => new module.MapSession());
}
//#endregion MapSession

//#region TerrainSession
export type TerrainWasmSession = {
  set_project_origin(lon: number, lat: number): void;
  set_exaggeration(exaggeration: number): void;
  visible_terrain_tiles_json(cameraJson: string): string;
  upload_elevation_tile(z: number, x: number, y: number, bytes: Uint8Array): boolean;
  evict_terrain_tile(z: number, x: number, y: number): void;
  terrain_tile_mesh_json(z: number, x: number, y: number): string;
};

export async function createTerrainSession(): Promise<TerrainWasmSession> {
  return createSurfaceSession((module) => new module.TerrainSession());
}
//#endregion TerrainSession

//#region Board2dSession
export type Board2dWasmSession = {
  attach_canvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  parseFixtureJson(json: string): boolean;
  syncDescriptorJson(json: string): void;
  setKindCatalogsJson(json: string): void;
  setCamera(x: number, y: number, zoom: number): void;
  setSelectionIdsJson(json: string): void;
  setCanvasThemeJson(json: string): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  drainEventsJson(): string;
  cameraJson(): string;
  gpuReady(): boolean;
  setHoveredIdSilent?(id?: string | null): void;
  setActiveUtility?(label: string): void;
  setSelectionOptions?(method: string, mode: string, selectNodes: boolean, selectEdges: boolean, selectHandles: boolean): void;
  setGridSnapEnabled?(enabled: boolean): void;
  setGridFactor?(v: number): void;
  setSuggestionOffset?(distance: number): void;
  setBrushKindWeights?(json: string): void;
  setHandleLinkCompatJson?(json: string): void;
  setAutomaticLod?(enabled: boolean): void;
  setForcedDrawLodLabel?(label: string): void;
  setSelectionIdsJsonSilent?(json: string): void;
  setCameraSilent?(x: number, y: number, zoom: number): void;
  pointerLeaveScreen?(alt: boolean): void;
  pickTargetsAtScreenJson?(sx: number, sy: number): string;
  deleteSelection?(): void;
  cancelAreaSelect?(): boolean;
  brushCycleCandidate?(forward: boolean): void;
  setFixtureDropPreviewJson?(json: string): void;
  clearFixtureDropPreview?(): void;
  defersDescriptorSyncFromJs?(): boolean;
  isDraggingAreaSelect?(): boolean;
  /** @emoji 🐢️ Silent cross-pane mirror setters (WS-live-sync round 4) — move nodes/set preselect/set the marquee outline without emitting board events or a fixture reset, so a peer pane can mirror another pane's live gesture without round-tripping through the program. */
  setNodePositionsJson?(json: string): void;
  setPreselectStateJsonSilent?(json: string): void;
  setSelectionScreenPreview?(flatXy: readonly number[]): void;
  clearSelectionScreenPreview?(): void;
  free(): void;
};

export async function createBoard2dSession(): Promise<Board2dWasmSession> {
  return createEngineSession<Board2dWasmSession>("board-2d", "BoardSession");
}
//#endregion Board2dSession

//#region SceneHelpers
export function isFlowGraphScene(capabilitiesJson?: string): boolean {
  if (!capabilitiesJson) return false;
  try {
    const caps = JSON.parse(capabilitiesJson) as { readonly engine?: string; readonly spotlight?: boolean; readonly noteEdit?: boolean };
    return caps.engine === "flow" || caps.spotlight === true || caps.noteEdit === true;
  } catch {
    return false;
  }
}
//#endregion SceneHelpers
//#endregion 🔖️wasm-session-loader
