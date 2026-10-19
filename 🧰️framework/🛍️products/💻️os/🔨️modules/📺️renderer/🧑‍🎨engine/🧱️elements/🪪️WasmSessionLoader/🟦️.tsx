// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🪪️WasmSessionLoader/component.tsx
/** 🕸️ Compiler-checked shared surface constructors and app-owned session factory scopes.
 * Board sessions are supplied by the owning product composition, never by the shared surface module.
 * Demand scheduling and Flow scene capability detection are shared by surface hosts.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { parseHubProgramIdV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";
import { createWasmModuleLoader, type AppSurfaceSessionFactory, type Board2dWasmSession, type WasmCanvasSession } from "@semio-tech/framework/canvas/react-renderer";
import { createContext } from "react";
import { installPageRuntimeAssets } from "@semio-tech/framework/canvas/runtime-assets";
import { openNodeGraphSessionV1, type NodeGraphSessionV1 } from "../🕸️NodeGraph/📦️packages/🦀️rust/🟦️.ts";
import type { FlowBrowserRuntimeV1, FlowWasmSession } from "./🌊️flow-session/🟦️.ts";
export type { FlowTask, FlowTaskEvent, FlowWasmSession } from "./🌊️flow-session/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️wasm-session-loader


//#region 🪝️WasmBindingsPort
/** 🗺️ The shared surface wasm module as the renderer uses it: its initializer and the session classes it constructs. */
export type SurfaceBindingsV1 = Readonly<{
  default(): Promise<unknown>;
  RasterSession: new () => RasterWasmSession;
  MapSession: new () => MapWasmSession;
  TerrainSession: new () => TerrainWasmSession;
}>;

/** 🌊️ The flow wasm module and its browser runtime factory. */
export type FlowBindingsV1 = Readonly<{
  core: Readonly<{ default(): Promise<unknown> }>;
  browser: Readonly<{ createFlowBrowserRuntime(options: Readonly<{ source: unknown; bindings?: unknown }>): Promise<FlowBrowserRuntimeV1> }>;
}>;

/** ✏️ The editor wasm module as the renderer uses it. */
export type EditorBindingsV1 = Readonly<{
  default(): Promise<unknown>;
  EditorSession: new () => EditorWasmSession;
}>;

/** 🪝️ The wasm engine packages of the page, loaded on demand by the composition root that owns them: the renderer names no wasm package, it asks the installed port. */
export type WasmBindingsV1 = Readonly<{
  surface(): Promise<SurfaceBindingsV1>;
  flow(): Promise<FlowBindingsV1>;
  editor(): Promise<EditorBindingsV1>;
  nodeGraph(): Promise<unknown>;
}>;

const installedWasmBindings: WasmBindingsV1[] = [];

/** 🪝️ Installs the wasm bindings the loaders use and returns the function that retires exactly this installation; every shell of a page installs the page's bindings and the latest installation answers until it retires. */
export function registerWasmBindingsV1(bindings: WasmBindingsV1): () => void {
  installedWasmBindings.push(bindings);
  return () => {
    const index = installedWasmBindings.lastIndexOf(bindings);
    if (index >= 0) installedWasmBindings.splice(index, 1);
  };
}

function requireWasmBindingsV1(): WasmBindingsV1 {
  const bindings = installedWasmBindings.at(-1);
  if (bindings === undefined) throw new Error("wasm-bindings-absent: the composition root installed no wasm bindings");
  return bindings;
}
//#endregion 🪝️WasmBindingsPort

//#region 🗺️SharedSurfaceSessionLoader
type SurfaceSessionModule = SurfaceBindingsV1;

const loadSurfaceSessionModule = createWasmModuleLoader<SurfaceSessionModule>(async () => {
  const module = await requireWasmBindingsV1().surface();
  await module.default();
  await installPageRuntimeAssets(module);
  return module;
});

/** 🗺️ Initializes the actual shared surface module before invoking a compiler-checked constructor. */
async function createSurfaceSession<T>(construct: (module: SurfaceSessionModule) => T): Promise<T> {
  return construct(await loadSurfaceSessionModule());
}
//#endregion 🗺️SharedSurfaceSessionLoader


//#region GraphSession
export async function createGraphSession(): Promise<NodeGraphSessionV1> {
  return openNodeGraphSessionV1(() => requireWasmBindingsV1().nodeGraph());
}
//#endregion GraphSession

//#region FlowSession
let flowSessionPromise: Promise<FlowBrowserRuntimeV1> | null = null;

export async function createFlowSession(): Promise<FlowWasmSession> {
  if (!flowSessionPromise) {
    flowSessionPromise = requireWasmBindingsV1().flow().then(async ({ core, browser }) => {
      const exports = await core.default();
      await installPageRuntimeAssets(core);
      // 🖼️ `core` (not `exports`) carries the wasm-bindgen wrappers, the only ones that can take an
      // `HtmlCanvasElement` — that is how a flow surface reaches its own WebGPU presenter.
      return browser.createFlowBrowserRuntime({ source: exports, bindings: core });
    });
  }
  const runtime = await flowSessionPromise;
  return runtime.openSession();
}
//#endregion FlowSession

//#region EditorSession
export type EditorWasmSession = WasmCanvasSession & {
  synchronizeScene(bytes: Uint8Array, format: 0 | 1, maximumInputBytes: number, maximumOwnedBytes: number, progress: (completed: number, total: number, ownedBytes: number) => boolean): void;
  setText(text: string): void;
  text(): string;
  caret(): number;
  anchor(): number;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number, buttons: number): void;
  pointerUpScreen(sx: number, sy: number, buttons: number): void;
  pointerCancelScreen(): void;
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

type EditorSessionModule = EditorBindingsV1;
const loadEditorSessionModule = createWasmModuleLoader<EditorSessionModule>(async () => {
  const module = await requireWasmBindingsV1().editor();
  await module.default();
  await installPageRuntimeAssets(module);
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
  renderFrame(): boolean;
  renderProgressJson(): string;
  cancelRender(): void;
  setCamera(x: number, y: number, zoom: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  pointerCancelScreen(): void;
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

/** 🪪️ Joins one exact app-owned constructor to the current shell instance without constructing it. A hub document's
 * program (`<plugin>@<bundle sha256>`, {@link parseHubProgramIdV1}) runs its plugin's own app, so it joins that plugin's
 * constructor: the registrations are keyed by plugin id, and a hub-opened 2d puzzle threw "no registered board session
 * factory" (ticket 26/09/23 S18 §14c). The scoped factory keeps the program's own id. */
export function resolveAppSurfaceSessionFactory(registrations: readonly AppSurfaceSessionFactory[], identity: { readonly pluginId: string; readonly appId: string; readonly instanceId: number } | null): ScopedBoardSessionFactory | null {
  if (!identity) return null;
  const pluginId = parseHubProgramIdV1(identity.pluginId)?.pluginId ?? identity.pluginId;
  const matches = registrations.filter((registration) => registration.kind === "board-2d" && registration.pluginId === pluginId && registration.appId === identity.appId);
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
