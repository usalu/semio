// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/WasmSessionLoader/component.tsx
/** 🕸️ Compiler-checked shared surface constructors and app-owned session factory scopes.
 * Board sessions are supplied by the owning product composition, never by the shared surface module.
 * Demand scheduling and Flow scene capability detection are shared by surface hosts.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type GraphWasmSession } from "@semio-tech/infinite-canvas-react-renderer";
import { createContext } from "react";
// #endregion 🔌️Adapters

//#region 🔖️wasm-session-loader


//#region 🗺️SharedSurfaceSessionLoader
type SurfaceSessionModule = typeof import("@semio-tech/framework-surface-rs");

/** 📦️ Deduplicates in-flight module initialization and retires only the exact failed attempt. */
export function createWasmModuleLoader<T>(load: () => Promise<T>): () => Promise<T> {
  let pending: Promise<T> | undefined;
  return () => {
    if (pending) return pending;
    const attempt = Promise.resolve().then(load);
    pending = attempt;
    void attempt.catch(() => { if (pending === attempt) pending = undefined; });
    return attempt;
  };
}

const loadSurfaceSessionModule = createWasmModuleLoader<SurfaceSessionModule>(async () => {
  const module = await import("@semio-tech/framework-surface-rs");
  await module.default();
  return module;
});

/** 🗺️ Initializes the actual shared surface module before invoking a compiler-checked constructor. */
async function createSurfaceSession<T>(construct: (module: SurfaceSessionModule) => T): Promise<T> {
  return construct(await loadSurfaceSessionModule());
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

type EditorSessionModule = typeof import("@semio-tech/framework-editor-rs");
const loadEditorSessionModule = createWasmModuleLoader<EditorSessionModule>(async () => {
  const module = await import("@semio-tech/framework-editor-rs");
  await module.default();
  return module;
});

export async function createEditorSession(): Promise<EditorWasmSession> {
  const mod = await loadEditorSessionModule();
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
  hasTile(z: number, x: number, y: number): boolean;
  hasVectorTile(z: number, x: number, y: number): boolean;
  visibleTilesJson(): string;
  visibleVectorTilesJson(): string;
  visibleTilesRevision(): number;
  visibleVectorTilesRevision(): number;
  prefetchTilesJson(): string;
  prefetchVectorTilesJson(): string;
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

export type AppSurfaceSessionFactory = {
  readonly kind: "board-2d";
  readonly pluginId: string;
  readonly appId: string;
  readonly create: () => Promise<Board2dWasmSession>;
};

export type ScopedBoardSessionFactory = {
  readonly pluginId: string;
  readonly appId: string;
  readonly instanceId: number;
  readonly create: () => Promise<Board2dWasmSession>;
  readonly scope: BoardPeerScope;
};

export type Board2dPeer = { readonly session: Board2dWasmSession; readonly onPeerGestureEnded: (flushed: boolean) => void };
export type BoardPeerScope = {
  readonly peers: Map<string, Map<string, Board2dPeer>>;
  readonly gestures: Map<string, { readonly surfaceId: string; readonly peer: Board2dPeer }>;
};

export function createBoardPeerScope(): BoardPeerScope {
  return { peers: new Map(), gestures: new Map() };
}

export const BoardSessionFactoryContext = createContext<ScopedBoardSessionFactory | null>(null);

/** 🪪️ Joins one exact app-owned constructor to the current shell instance without constructing it. */
export function resolveAppSurfaceSessionFactory(registrations: readonly AppSurfaceSessionFactory[], identity: { readonly pluginId: string; readonly appId: string; readonly instanceId: number } | null): ScopedBoardSessionFactory | null {
  if (!identity) return null;
  const matches = registrations.filter((registration) => registration.kind === "board-2d" && registration.pluginId === identity.pluginId && registration.appId === identity.appId);
  if (matches.length > 1) throw new Error(`Duplicate board session factory for ${identity.pluginId}/${identity.appId}`);
  const registration = matches[0];
  return registration ? { pluginId: identity.pluginId, appId: identity.appId, instanceId: identity.instanceId, create: registration.create, scope: createBoardPeerScope() } : null;
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
