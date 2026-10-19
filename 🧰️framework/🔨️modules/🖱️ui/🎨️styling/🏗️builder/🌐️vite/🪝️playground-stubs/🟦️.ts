import { relative, isAbsolute, resolve, join, dirname } from "node:path";
import type { OwnedBuildPlugin } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { readFileSync, existsSync } from "node:fs";
import { validateJsonSchemaSubset } from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import providerSchema from "../🧬️schema/🔣️.json" with { type: "json" };

/** @emoji 🧭️ Vite's URL prefix for prebundled chunks under `cacheDir`: root-relative inside `root`, `/@fs/` outside. https://vite.dev/config/shared-options.html#cachedir */
export function playgroundOptimizedDepUrlPrefix(root: string, cacheDir: string): string {
  const path = relative(root, cacheDir).replaceAll("\\", "/");
  return path.startsWith("..") || isAbsolute(path) ? `/@fs/${resolve(cacheDir).replaceAll("\\", "/").replace(/^\/+/, "")}/deps/` : `/${path}/deps/`;
}

/** @emoji 🔗️ True when a percent-encoded request targets this server's Vite prebundled chunks. */
export function isPlaygroundOptimizedDepUrl(url: string, prefix: string): boolean {
  try { return decodeURI(url).includes(prefix); } catch { return false; }
}

/** @emoji 🧱️ Stubs vitest and testing-library when test regions enter the browser graph. */
export function playgroundVitestDevStubPlugin(): OwnedBuildPlugin {
  const vitestStubId = "\0playground-vitest-dev-stub";
  const testingLibraryStubId = "\0playground-testing-library-dev-stub";
  return {
    name: "playground-vitest-dev-stub",
    enforce: "pre",
    resolveId(id) {
      if (id === "vitest" || id.startsWith("vitest/") || id.startsWith("@vitest/")) return vitestStubId;
      if (id === "@testing-library/react" || id.startsWith("@testing-library/")) return testingLibraryStubId;
      return undefined;
    },
    load(id) {
      if (id === vitestStubId) {
        return "export default {}; export const describe = () => {}; export const it = () => {}; export const expect = () => ({ toBe: () => {}, toEqual: () => {} }); export const vi = { fn: () => {}, mock: () => {}, spyOn: () => {} };";
      }
      if (id === testingLibraryStubId) {
        return "export default {}; export const render = () => ({}); export const screen = {}; export const fireEvent = {}; export const waitFor = async (fn) => fn();";
      }
    },
  };
}

const PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID = "\0playground-playwright-dev-stub";

export const PLAYGROUND_WASM_STUB_PREFIX = "\0playground-wasm-stub/";

/** 🗂️ Vite's URL form for an absolute filesystem path outside the project root. */
const FS_URL_PREFIX = "/@fs/";

export function playgroundWasmStubKey(cleanId: string): string {
  return cleanId.replace(/\//g, "__");
}

function playgroundWasmStubKeyDecode(key: string): string {
  return key.replace(/__/g, "/");
}

const PLAYGROUND_WASM_JS_STUB = `const wasmMissing = () => { throw new Error("wasm pkg not built — run the matching wasm build target"); };
const wasmJson = () => "{}";
const dagLodScaleJson = () => ${JSON.stringify(
  JSON.stringify([
    { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; fill only.", maxZoom: 0.4 },
    { id: "overview", name: "Overview", description: "Node icons only.", maxZoom: 0.6 },
    { id: "compact", name: "Compact", description: "Horizontal abbreviations.", maxZoom: 0.8 },
    { id: "normal", name: "Normal", description: "Vertical names with sections; channel abbreviations on ports.", maxZoom: 1.5 },
    { id: "detail", name: "Detail", description: "Channel names on ports, port handles, and control text.", maxZoom: 2.75 },
    { id: "micro", name: "Micro", description: "Full channel names on ports and maximum node fidelity.", maxZoom: Number.MAX_VALUE },
  ]),
)};
export default async function initWasm() {}
export const initSync = () => {};
export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadSnapshotJson() {} snapshotJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}
export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }
export class EditorSession { syncFromSceneJson() {} syncFromScenePack() {} setText() {} text() { return ''; } caret() { return 0; } anchor() { return 0; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScrollScreen() {} insertText() {} backspace() {} deleteForward() {} selectAll() {} replaceSelection() {} selectionText() { return ''; } hoverTokenRangeJson() { return 'null'; } setHoverRange() {} cameraJson() { return '{}'; } }
export class DagSession { lodScaleJson() { return dagLodScaleJson(); } }
export class BoardSession { lodScaleJson() { return dagLodScaleJson(); } }
export class WriterSession {}
export class ImperativeSession {}
export class SequenceSession {}
export class RasterSession {}
export class MapSession {}
export class Puzzle3dPrecomputeSession {}
export class TrinitySession {}
export class JackLspSession {}
export const render_drawing_scene = wasmMissing;
export const export_drawing_svg = wasmMissing;
export const export_drawing_pdf = wasmMissing;
export const dispose_drawing = () => {};
export const trace_drawing_bitmap = wasmMissing;
export const boolean_drawing_segments = wasmMissing;
export const tessellate = async () => JSON.stringify({ positions: [], normals: [], index: [], edges: [], points: [], faceGroups: [] });
export const dispose = () => {};
export const evaluate = wasmMissing;
export const ruleQueryJson = wasmJson;
export const boardComputeEdgeBezier = wasmJson;
export const boardHandlePositionCircle = wasmJson;
export const boardHandlePositionRectangle = wasmJson;
export const boardRedrawLayoutSnapshotJson = wasmJson;
`;

function workspaceWasmPkgResolveCandidates(repoRoot: string, pkgName: string, subpath: string | undefined): string[] {
  const pkgRoot = resolve(repoRoot, "node_modules", pkgName);
  const candidates: string[] = [];
  let manifest: { exports?: Record<string, string | { import?: string; default?: string }>; module?: string; main?: string } | undefined;
  try {
    manifest = JSON.parse(readFileSync(resolve(pkgRoot, "package.json"), "utf8"));
  } catch {}
  const pushExportTarget = (key: string) => {
    const exp = manifest?.exports?.[key];
    const target = typeof exp === "string" ? exp : (exp?.import ?? exp?.default);
    if (target) candidates.push(resolve(pkgRoot, target));
  };
  if (subpath) {
    candidates.push(resolve(pkgRoot, subpath));
    if (subpath.startsWith("pkg/")) {
      candidates.push(resolve(pkgRoot, "rs", subpath));
      candidates.push(resolve(pkgRoot, subpath.slice("pkg/".length)));
    }
    pushExportTarget(`./${subpath}`);
    pushExportTarget(subpath);
  } else {
    pushExportTarget(".");
    if (manifest?.module) candidates.push(resolve(pkgRoot, manifest.module));
    if (manifest?.main) candidates.push(resolve(pkgRoot, manifest.main));
  }
  return candidates;
}

/** 🌊️ Carries closed Source or selected-publication paths as DATA without opening a compiler or store. */
export type FlowWasmDeliveryV1=Readonly<{version:1;kind:"source";browser:string;host:string}>|Readonly<{version:1;kind:"publication";root:string}>;

/** 🧱️ Resolves the explicit Flow facet before generic missing-package stubs. */
export function playgroundFlowWasmDevStubPlugin(repoRoot: string,delivery:FlowWasmDeliveryV1): OwnedBuildPlugin {
  if(validateJsonSchemaSubset(providerSchema.definitions.FlowWasmDeliveryV1,delivery).length)throw Error("Closed explicit Flow delivery required");
  for(const path of delivery.kind==="source"?[delivery.browser,delivery.host]:[delivery.root])if(!isAbsolute(path)||resolve(path)!==path)throw Error("Canonical bounded Flow delivery paths required");
  return {
    name: "playground-flow-wasm-dev-stub",
    enforce: "pre",
    resolveId(id, importer) {
      if (!importer || id.startsWith(PLAYGROUND_WASM_STUB_PREFIX)) return undefined;
      const cleanId = id.split("?", 1)[0] ?? id;
      const flowName=cleanId==="@semio-tech/flow-core"?"flow_core.js":cleanId.startsWith("@semio-tech/flow-core/")?cleanId.slice("@semio-tech/flow-core/".length).replace(/^pkg\//u,""):/(?:^|\/)🕸️bindings\/flow_core\.js$/u.test(cleanId)?"flow_core.js":undefined;
      if(flowName!==undefined){
        if(["🌐️flow-browser.js","🌐️browser/🟨️.js"].includes(flowName))return delivery.kind==="source"?delivery.browser:join(delivery.root,"🌐️browser/🟨️.js");
        if(["🖥️flow-host.js","🖥️host/🟨️.js"].includes(flowName))return delivery.kind==="source"?delivery.host:join(delivery.root,"🖥️host/🟨️.js");
        if(flowName!=="flow_core.js")throw Error("Closed Flow package subpath required");
        return delivery.kind==="source"?`${PLAYGROUND_WASM_STUB_PREFIX}${playgroundWasmStubKey(cleanId)}`:join(delivery.root,"flow_core.js");
      }
      const fsId = cleanId.startsWith(FS_URL_PREFIX) ? cleanId.slice(FS_URL_PREFIX.length - 1) : cleanId;
      const isWasmPkg = cleanId.includes("/pkg/") || cleanId.endsWith(".wasm") || cleanId === "@semio-tech/flow-core" || cleanId === "@semio-tech/flow-core/pkg/flow_core.js" || cleanId === "@semio-tech/flow-core/flow_core.js";
      if (!isWasmPkg) return undefined;
      if (fsId.startsWith(".")) {
        if (existsSync(resolve(dirname(importer), fsId))) return undefined;
        return `${PLAYGROUND_WASM_STUB_PREFIX}${playgroundWasmStubKey(cleanId)}`;
      }
      const workspacePkg = fsId.match(/^(@semio-tech\/[^/]+)(?:\/(.+))?$/);
      const candidates: string[] = [];
      if (workspacePkg) {
        const [, pkgName, subpath] = workspacePkg;
        if(pkgName===undefined)throw Error("Matched workspace package name required");
        candidates.push(...workspaceWasmPkgResolveCandidates(repoRoot, pkgName, subpath));
      } else {
        candidates.push(resolve(repoRoot, fsId));
      }
      const hit = candidates.find((abs) => existsSync(abs));
      if (hit) return hit;
      return `${PLAYGROUND_WASM_STUB_PREFIX}${playgroundWasmStubKey(cleanId)}`;
    },
    load(id) {
      if (!id.startsWith(PLAYGROUND_WASM_STUB_PREFIX)) return undefined;
      const cleanId = playgroundWasmStubKeyDecode(id.slice(PLAYGROUND_WASM_STUB_PREFIX.length).split("?", 1)[0] ?? "");
      if (cleanId.endsWith(".wasm")) return `export default "";`;
      return PLAYGROUND_WASM_JS_STUB;
    },
  };
}

/** @emoji 🧱️ Stubs Playwright when test-only regions are pulled into the browser graph. */
export function playgroundPlaywrightDevStubPlugin(): OwnedBuildPlugin {
  return {
    name: "playground-playwright-dev-stub",
    enforce: "pre",
    resolveId(id) {
      if (id === "@playwright/test" || id === "playwright" || id === "playwright-core" || id === "chromium-bidi") {
        return PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID;
      }
      return undefined;
    },
    load(id) {
      if (id !== PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID) return;
      return "export default {}; export const test = () => {}; export const expect = () => ({ toBe: () => {}, toEqual: () => {} });";
    },
  };
}

/** @emoji 🔄️ Full-reload connected clients when a stale optimized-dep chunk returns 504. */
export function playgroundStaleOptimizeDepPlugin(): OwnedBuildPlugin {
  return {
    name: "playground-stale-optimize-dep",
    configureServer(server) {
      const prefix = playgroundOptimizedDepUrlPrefix(server.config.root, server.config.cacheDir);
      server.middlewares.use((req, res, next) => {
        if (!isPlaygroundOptimizedDepUrl(req.url ?? "", prefix)) {
          next();
          return;
        }
        res.on("finish", () => {
          if (res.statusCode === 504) {
            server.ws.send({ type: "full-reload", path: "*" });
          }
        });
        next();
      });
    },
  };
}
