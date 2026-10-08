// @vitest-environment node
// #region 🧲️Header
/** 🌐️ Vite plugins serving the asset-owned `/🖼️assets/*` namespace. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralMap } from "@semio-tech/framework";
import { type IncomingMessage, type ServerResponse } from "node:http";
import { copyFileSync, cpSync, createReadStream, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";


import {
  type OwnedBuildConfig,
  type OwnedBuildMiddleware,
  type OwnedBuildPlugin,
} from "../../../🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts";
import { parseTileProxyAssetSpecV1, TILE_PROXY_TRANSPORT_LIMITS_V1, type TileProxyAssetSpecV1 } from "../../../../🖼️assets/🗺️tile-proxy/🟦️.ts";
import type { AssetDeliveryDeclarationV1, AssetDeliveryModeV1, AssetDeliveryProviderV1 } from "../../../../🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
import providerSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import { parseMeshDeliveryCatalog, type MeshDeliveryCatalog } from "../../../../🖼️assets/🥽️mesh/🟦️.ts";
import { assetPathFromRequest, SEMIO_ASSET_DIRECTORY, SEMIO_ASSET_ROUTE } from "../../../../🖼️assets/🔍️resolver/🌐️delivery/🟦️.ts";
import faviconDelivery from "../../🌐️favicon/🔣️.json" with { type: "json" };

// #endregion 🔌️Adapters

export { playgroundIframeEmbedHeadersPlugin } from "../../🌐️iframe/🟦️.ts";

/** 🥽️ Supplies one owner-authored mesh catalog to its build provider. */
export type MeshCollectionAssetSpecV1 = { readonly kind: "mesh-collection"; readonly route: string; readonly catalog: string };

/** 🗂️ Supplies one owner-authored directory to its build provider. */
export type StaticDirectoryAssetSpecV1 = { readonly kind: "static-dir"; readonly route: string; readonly root: string };

function admitBuildAssetSpecV1<K extends keyof typeof providerSchema.definitions>(value: AssetDeliveryDeclarationV1, kind: K): K extends "MeshCollectionAssetSpecV1" ? MeshCollectionAssetSpecV1 : StaticDirectoryAssetSpecV1 {
  const failures = validateJsonSchemaSubset(providerSchema.definitions[kind], value);
  if (failures.length) throw Error("Invalid " + kind + ": " + failures.join("; "));
  return value as K extends "MeshCollectionAssetSpecV1" ? MeshCollectionAssetSpecV1 : StaticDirectoryAssetSpecV1;
}

//#region 🔖️ViteElementsAssets
/** @emoji 📦️ Relative-base Vite build defaults for playground static sites (iframe + subdomain safe). */
export function playgroundStaticSiteBuildOptions(overrides?: OwnedBuildConfig["build"]): NonNullable<OwnedBuildConfig["build"]> {
  return {
    target: "esnext",
    outDir: "dist",
    emptyOutDir: true,
    ...overrides,
  };
}

/** @emoji 🚀️ Production Vite `build` defaults: minify, strip console/debugger, no sourcemaps. */
export function semioViteProductionBuild(overrides?: OwnedBuildConfig["build"]): NonNullable<OwnedBuildConfig["build"]> {
  return {
    target: "es2022",
    sourcemap: false,
    minify: "esbuild",
    cssMinify: true,
    reportCompressedSize: false,
    ...overrides,
    esbuild: {
      drop: ["console", "debugger"],
      legalComments: "none",
      ...(overrides?.esbuild ?? {}),
    },
  };
}

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

const PLAYGROUND_WASM_JS_STUB = `const wasmMissing = () => { throw new Error("wasm pkg not built — run the matching nx wasm target"); };
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
export const boardRedrawHandlesSnapshotJson = wasmJson;
export const boardRedrawLayoutSnapshotJson = wasmJson;
`;

function workspaceWasmPkgResolveCandidates(repoRoot: string, pkgName: string, subpath: string | undefined): string[] {
  const pkgRoot = resolve(repoRoot, "node_modules", pkgName);
  const candidates: string[] = [];
  let manifest: { exports?: Record<string, string | { import?: string; default?: string }>; module?: string; main?: string } | undefined;
  try {
    manifest = JSON.parse(readFileSync(resolve(pkgRoot, "package.json"), "utf8"));
  } catch {
    /* package.json may be absent for a half-linked workspace package */
  }
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

/** @emoji 🧱️ Stubs missing wasm pkg imports until `nx run …:wasm` artifacts exist. */
export function playgroundFlowWasmDevStubPlugin(repoRoot: string): OwnedBuildPlugin {
  return {
    name: "playground-flow-wasm-dev-stub",
    enforce: "pre",
    resolveId(id, importer) {
      if (!importer || id.startsWith(PLAYGROUND_WASM_STUB_PREFIX)) return undefined;
      const cleanId = id.split("?", 1)[0] ?? id;
      // 🗂️ A module the browser requests directly arrives as Vite's own `/@fs/<absolute path>` URL, not
      // as the specifier the importer wrote. Testing that URL for existence always fails, which used to
      // hand back the "wasm pkg not built" stub for a pkg that is sitting right there on disk.
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

function contentTypeForUiAsset(filePath: string): string | undefined {
  if (filePath.endsWith(".woff2")) {
    return "font/woff2";
  }
  if (filePath.endsWith(".svg")) {
    return "image/svg+xml";
  }
  if (filePath.endsWith(".wasm")) {
    return "application/wasm";
  }
  return undefined;
}

function createUiAssetsMiddleware(assetsRoot: string): OwnedBuildMiddleware {
  const assetsRootResolved = resolve(assetsRoot);
  return (req, res, next) => {
    const rel = req.url ? assetPathFromRequest(req.url) : null;
    if (rel === null) {
      next();
      return;
    }
    const filePath = resolve(assetsRootResolved, rel);
    const relToRoot = relative(assetsRootResolved, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    serveFileWithValidatorsV1(req, res, filePath, contentTypeForUiAsset(filePath));
  };
}

//#region 🔖️MeshCollectionAssetPlugin
function readMeshDeliveryCatalog(repoRoot: string, spec: MeshCollectionAssetSpecV1): MeshDeliveryCatalog {
  if (spec.route !== "/mesh") throw new Error(`Unsupported mesh catalog route: ${spec.route}`);
  const read = (path: string): unknown => JSON.parse(readFileSync(resolve(repoRoot, path), "utf8"));
  return parseMeshDeliveryCatalog(read(spec.catalog), read);
}

/** 🌐️ Serves only explicit catalog transport paths, retaining public identity at the caller boundary. */
function createMeshCollectionMiddleware(repoRoot: string, spec: MeshCollectionAssetSpecV1): OwnedBuildMiddleware {
  const route = `${spec.route}/`;
  const catalog = new Map(readMeshDeliveryCatalog(repoRoot, spec).map(entry => [`${route}${entry.path}`, entry]));
  return (req, res, next) => {
    if (!req.url?.startsWith(route)) {
      next();
      return;
    }
    let path: string;
    try {
      path = decodeURIComponent(req.url.split(/[?#]/, 1)[0] ?? "");
    } catch {
      res.statusCode = 400;
      res.end();
      return;
    }
    const entry = catalog.get(path);
    if (!entry) return next();
    const source = resolve(repoRoot, entry.source);
    if (!existsSync(source) || !statSync(source).isFile()) return next();
    serveFileWithValidatorsV1(req, res, source, "model/gltf-binary");
  };
}

/** 📦️ Copies only admitted source entries to their exact handpicked nested delivery paths. */
function copyMeshCollectionGlbs(repoRoot: string, catalog: MeshDeliveryCatalog, dest: string): void {
  mkdirSync(dest, { recursive: true });
  for (const entry of catalog) {
    const source = resolve(repoRoot, entry.source);
    if (!existsSync(source) || !statSync(source).isFile()) throw new Error(`Missing catalog mesh source: ${entry.source}`);
    const destination = resolve(dest, entry.path);
    mkdirSync(dirname(destination), { recursive: true });
    cpSync(source, destination);
  }
}

/** 🧊️ Dev and static delivery share one explicit public-ID/source/output authority. */
export function meshCollectionVitePlugin(repoRoot: string, spec: MeshCollectionAssetSpecV1): OwnedBuildPlugin[] {
  const serveMeshes = createMeshCollectionMiddleware(repoRoot, spec);
  const catalog = readMeshDeliveryCatalog(repoRoot, spec);
  const destName = spec.route.replace(/^\//, "");
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return [
    {
      name: `mesh-collection-serve${spec.route}`,
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveMeshes);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveMeshes);
      },
    },
    {
      name: `mesh-collection-build${spec.route}`,
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        const dest = resolve(outDir, destName);
        mkdirSync(outDir, { recursive: true });
        copyMeshCollectionGlbs(repoRoot, catalog, dest);
      },
    },
  ];
}
//#endregion 🔖️MeshCollectionAssetPlugin

//#region 🔖️HostHtmlPlugin
/** @emoji 🎬️ Inline shell paint before Tailwind finishes compiling the play stylesheet. */
export const PLAYGROUND_PLAY_BOOT_INLINE_STYLE =
  "html{color-scheme:light dark}html,body,#root{height:var(--ui-available-height,100dvh);margin:0}body{background-color:#f7f3e3;color:#001117}html.dark body{background-color:#001117;color:#f7f3e3}html:not([data-semio-styled]) body{visibility:hidden}";

/** @emoji 📐 Sizes the host to the available viewport before first paint.
 *
 * `100vh` is the screen. On Android Firefox the browser navbar is drawn over that screen and a
 * `100vh` page with `overflow: hidden` is clipped underneath it. `visualViewport.height` is the
 * available height; the same choice as {@link availableViewportHeightPx}. Listeners keep the
 * variable live as the toolbar shows and hides. */
export const PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT = `(function(){var last="";function sync(){var vv=window.visualViewport;var h=vv&&vv.height>0?vv.height:window.innerHeight;if(!(h>0)||!isFinite(h))return;var px=Math.round(h)+"px";if(px===last)return;last=px;document.documentElement.style.setProperty("--ui-available-height",px)}sync();var vv=window.visualViewport;if(vv&&vv.addEventListener){vv.addEventListener("resize",sync);vv.addEventListener("scroll",sync)}window.addEventListener("resize",sync);window.addEventListener("orientationchange",sync)})();`;

/**
 * @emoji 🌓️ Synchronous appearance bootstrap for every semio host `🌐️.html` head.
 *
 * Reads the ONE durable document the OS shell actually writes — `localStorage["semio.os.config"]`,
 * whose `preferences["os.config.ui-preferences"]` holds the append-only UI-preference event log — and
 * REPLAYS it, because that lane is event-sourced (`commitUiPreferencesConfigMutation`,
 * `🧑‍🎨engine/🎚️UiPreferences/🟦️.ts`) and the last `setAppearance` is the only appearance there is.
 * It used to read `ui.chrome.appearance`, a key nothing has written since the shell moved to that
 * document: with appearance = dark, `<html>` got no `.dark` class and `<body>` stayed light until
 * React mounted, which is a guaranteed light flash on every reload of a dark session
 * (`📓️react-i18n-a11y-customization-2026-09-13.md` §3.2, measured there and fixed here).
 *
 * 🔁️ Deliberately a hand-replayed literal and not an import: this is an inline `<script>` in a head
 * that runs before any module is fetched, so it can share no code with the engine. What keeps it
 * honest is `🧫️fixtures/🌓️appearance-boot.json`, answered by this script in a DOM-less sandbox and
 * independently by the engine's own projection.
 *
 * @see ../../../../../🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/🟦️.ts
 */
export const PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT = `(function(){var d=document.documentElement,m=window.matchMedia("(prefers-color-scheme: dark)");var a=null;try{var c=JSON.parse(localStorage.getItem("semio.os.config")||"null");var l=c&&c.preferences&&c.preferences["os.config.ui-preferences"];var g=l?JSON.parse(l):null;if(g&&g.version===1&&g.events&&g.events.length){for(var i=0;i<g.events.length;i++){var e=g.events[i];if(e&&e.mutation==="setAppearance")a=e.appearance||null}}}catch(e){}var dark=a==="dark"||(a!=="light"&&m.matches);d.classList.toggle("dark",dark);d.dataset.uiAppearance=dark?"dark":"light";d.style.colorScheme=dark?"dark":"light";if(document.body){document.body.style.colorScheme=dark?"dark":"light";document.body.style.backgroundColor=dark?"#001117":"#f7f3e3";document.body.style.color=dark?"#f7f3e3":"#001117";}})();`;

/** @emoji 👁️ Reveals the play shell after the linked globals stylesheet finishes loading. */
export const PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT = `(function(){function reveal(){document.documentElement.dataset.semioStyled="ready"}var link=document.getElementById("semio-play-styles");if(link){if(link.sheet)reveal();else link.addEventListener("load",reveal,{once:true})}else{reveal()}setTimeout(reveal,8000)})();`;

/**
 * @emoji 🎨️ Synchronous active-theme bootstrap: reapplies the active CUSTOM theme's colors before
 * first paint so a user-authored theme does not flash the semio defaults. Runs after
 * {@link PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT} so its resolved light/dark class wins the appearance
 * choice; this script only overrides colors.
 *
 * Replays the same `semio.os.config` event log the appearance script replays, folding `setTheme` and
 * `setCustomTheme` — the two mutations that actually decide which theme is active — instead of the
 * dead `ui.chrome.theme.snapshot` key. One deliberate consequence: a BUILT-IN theme is not in storage
 * at all (only its id is), so it has nothing to pre-apply and boots on the head's inline defaults, as
 * it did before this fix in every session where the snapshot key was absent, which is all of them.
 */
export const PLAYGROUND_PLAY_BOOT_THEME_SCRIPT = `(function(){try{var c=JSON.parse(localStorage.getItem("semio.os.config")||"null");var l=c&&c.preferences&&c.preferences["os.config.ui-preferences"];var g=l?JSON.parse(l):null;if(!g||g.version!==1||!g.events)return;var id=null,themes={};for(var i=0;i<g.events.length;i++){var e=g.events[i];if(!e)continue;if(e.mutation==="setTheme")id=e.themeId||null;if(e.mutation==="setCustomTheme"){if(e.theme)themes[e.themeId]=e.theme;else delete themes[e.themeId]}}var stored=id?themes[id]:null;var t=stored&&stored.config;if(!t||!t.colors)return;var d=document.documentElement;var dark=d.classList.contains("dark");for(var k in t.colors){d.style.setProperty("--color-"+k.replace(/_/g,"-"),t.colors[k])}if(t.spacing)for(var s in t.spacing){d.style.setProperty("--spacing-"+s.replace(/_/g,"-"),t.spacing[s])}d.dataset.uiTheme=id;var appearance=t.appearances&&t.appearances[dark?"dark":"light"];var chrome=appearance&&appearance.chrome;function resolveSimple(ref){return ref&&ref.token&&t.colors[ref.token]?t.colors[ref.token]:undefined}var base=chrome&&resolveSimple(chrome.base);var fg=chrome&&resolveSimple(chrome.foreground);if(document.body){if(base)document.body.style.backgroundColor=base;if(fg)document.body.style.color=fg}}catch(e){}})();`;

/** @emoji 🧬️ Boot-time head tags every semio host document shares (color-scheme inline style + synchronous
 * appearance/theme scripts) — single source both {@link semioHostHtmlString} and
 * {@link playgroundPlayBootHtmlPlugin} inject from, so the generalized host and playground never drift. */
function semioHostBootHeadTags(): { readonly tag: string; readonly attrs?: Record<string, string>; readonly children?: string; readonly injectTo: "head-prepend" | "head" }[] {
  return [
    { tag: "script", children: PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT, injectTo: "head-prepend" },
    { tag: "style", children: PLAYGROUND_PLAY_BOOT_INLINE_STYLE, injectTo: "head-prepend" },
    { tag: "script", children: PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, injectTo: "head-prepend" },
    { tag: "script", children: PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, injectTo: "head-prepend" },
  ];
}

/** @emoji 🎬️ Vite: inject early appearance + theme + stylesheet link into play `🌐️.html` to avoid unstyled flashes — additive tag injection onto each play's own hand-authored `🌐️.html`, sharing its boot-head fragment ({@link semioHostBootHeadTags}) with {@link semioHostHtmlVitePlugin} instead of duplicating the style/script assembly. */
export function playgroundPlayBootHtmlPlugin(): OwnedBuildPlugin {
  return {
    name: "playground-play-boot-html",
    transformIndexHtml: {
      order: "pre",
      handler() {
        return {
          tags: [
            ...semioHostBootHeadTags(),
            { tag: "link", attrs: { rel: "stylesheet", href: "./🎨️.css", id: "semio-play-styles" }, injectTo: "head" },
            { tag: "script", children: PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, injectTo: "head" },
          ],
        };
      },
    },
  };
}

/** @emoji 🔖️ Canonical semio emblem favicon `<link>` tags for playground and app `🌐️.html` heads. */
export const SEMIO_FAVICON_HEAD_HTML = `<link rel="icon" href="./${faviconDelivery.svg}" type="image/svg+xml" />\n    <link rel="icon" href="./${faviconDelivery.ico}" sizes="any" />`;

/** @emoji 🔖️ Repo-root paths for the round dark emblem SVG and ICO fallback (matches {@link SemioLogo}). */
export function semioFaviconSources(repoRoot: string): { readonly svg: string; readonly ico: string } {
  const logoRoot = resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos");
  return {
    svg: resolve(logoRoot, "🛡️emblem/🌘️dark-round/🖋️vector.svg"),
    ico: resolve(logoRoot, "🌐️favicon/🌘️dark-round/📏️size-32.ico"),
  };
}

const SEMIO_FAVICON_BLEED_RECT = '<rect width="350" height="350" fill="#001117"/>';

/** @emoji 🔖️ Favicon SVG with opaque bleed so ICO rasterization avoids white matte outside the round emblem. */
export function semioFaviconSvgMarkup(svgPath: string): string | undefined {
  if (!existsSync(svgPath)) {
    return undefined;
  }
  const raw = readFileSync(svgPath, "utf8");
  if (raw.includes(SEMIO_FAVICON_BLEED_RECT)) {
    return raw;
  }
  const open = raw.match(/<svg[^>]*>/)?.[0];
  if (!open) {
    return raw;
  }
  return raw.replace(open, `${open}${SEMIO_FAVICON_BLEED_RECT}`);
}

/** @emoji 🔖️ Resolved favicon content for one host: inline SVG markup plus an optional ICO fallback path. */
type FaviconContent = { readonly svgMarkup?: string; readonly icoPath?: string };

const STATIC_SITE_FAVICON_ALIASES = { svg: "favicon.svg", ico: "favicon.ico" } as const;

function createFaviconMiddleware(content: FaviconContent): OwnedBuildMiddleware {
  return (req, res, next) => {
    let url: string;
    try { url = decodeURIComponent(req.url?.split(/[?#]/, 1)[0] ?? ""); } catch { next(); return; }
    if ((url === `/${faviconDelivery.svg}` || url === `/${STATIC_SITE_FAVICON_ALIASES.svg}`) && content.svgMarkup) {
      res.setHeader("Content-Type", "image/svg+xml");
      res.end(content.svgMarkup);
      return;
    }
    if ((url === `/${faviconDelivery.ico}` || url === `/${STATIC_SITE_FAVICON_ALIASES.ico}`) && content.icoPath && existsSync(content.icoPath)) {
      serveFileWithValidatorsV1(req, res, content.icoPath, "image/x-icon");
      return;
    }
    next();
  };
}

/** @emoji 🔖️ Vite: serve and copy the given emblem SVG and bookmark ICO under their exact publication names. */
function faviconVitePlugins(content: FaviconContent): OwnedBuildPlugin[] {
  const serveFavicon = createFaviconMiddleware(content);
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return [
    {
      name: "semio-favicon-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFavicon);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFavicon);
      },
    },
    {
      name: "semio-favicon-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        const dist = outDir;
        mkdirSync(dist, { recursive: true });
        if (content.svgMarkup) {
          writeFileSync(resolve(dist, faviconDelivery.svg), content.svgMarkup);
          writeFileSync(resolve(dist, STATIC_SITE_FAVICON_ALIASES.svg), content.svgMarkup);
        }
        if (content.icoPath && existsSync(content.icoPath)) {
          cpSync(content.icoPath, resolve(dist, faviconDelivery.ico));
          cpSync(content.icoPath, resolve(dist, STATIC_SITE_FAVICON_ALIASES.ico));
        }
      },
    },
  ];
}

/** @emoji 🔖️ Vite: serve and copy semio emblem favicons at `/🛡️favicon.svg` and `/🔖️favicon.ico`. */
export function semioFaviconVitePlugin(repoRoot: string): OwnedBuildPlugin[] {
  const favicons = semioFaviconSources(repoRoot);
  return faviconVitePlugins({ svgMarkup: semioFaviconSvgMarkup(favicons.svg), icoPath: favicons.ico });
}

/** @emoji 🏷️ The host-chrome surface of a shell brand (structural subset of `framework/core/js`'s `ShellBrand`, so this styling layer never imports framework types). */
export type ShellBrandHostChrome = {
  readonly windowTitle: string;
  readonly logoSvg?: string;
  readonly faviconIcoPath?: string;
  /** 🌐️ Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `CNAME` file at the build root. */
  readonly cnameHost?: string;
};

/** @emoji 🚫️ Vite: writes `dist/.nojekyll` on every build (unconditionally — any static host that runs
 * Jekyll, e.g. GitHub Pages, silently drops files/dirs starting with `_` otherwise, breaking Vite's own
 * `__vite-browser-external-*.js` shim chunk) and `dist/CNAME` when a brand declares `cnameHost`. */
export function staticDeployMarkerVitePlugins(cnameHost: string | undefined): OwnedBuildPlugin[] {
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return [
    {
      name: "static-deploy-markers",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        mkdirSync(outDir, { recursive: true });
        writeFileSync(resolve(outDir, ".nojekyll"), "");
        if (cnameHost) writeFileSync(resolve(outDir, "CNAME"), `${cnameHost}\n`);
      },
    },
  ];
}

/** @emoji 🧭️ Rewrites Vite's SPA fallback target `/index.html` onto the constitutional emoji entry path. */
export function rewriteSpaFallbackToEmojiEntry(url: string, entryPath: string): string {
  const [pathOnly, ...rest] = url.split(/(?=[?#])/);
  const base = pathOnly ?? url;
  if (base !== "/index.html") return url;
  return `${entryPath}${rest.join("")}`;
}

function semioEmojiIndexHtmlRootRewrite(entry: string): OwnedBuildMiddleware {
  return (req, _res, next) => {
    const url = req.url ?? "";
    if (url === "/" || url.startsWith("/?")) req.url = `${entry}${url.slice(1)}`;
    next();
  };
}

function semioEmojiIndexHtmlSpaFallbackRewrite(entry: string): OwnedBuildMiddleware {
  return (req, _res, next) => {
    const url = req.url ?? "";
    const nextUrl = rewriteSpaFallbackToEmojiEntry(url, entry);
    if (nextUrl !== url) req.url = nextUrl;
    next();
  };
}

/** @emoji 📄️ Conventional static-host entry filenames emitted beside the constitutional emoji HTML entry. */
export const STATIC_SITE_HTML_ALIASES = ["index.html", "404.html"] as const;

/** @emoji 🌐️ Vite: treat hand-authored `🌐️.html` as the app index (`/` + build input). Vite's default
 * `index.html` name does not match the constitutional emoji entry filename. */
export function semioEmojiIndexHtmlVitePlugin(rootDir: string, fileName = "🌐️.html"): OwnedBuildPlugin {
  const entry = `/${fileName}`;
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return {
    name: "semio-emoji-index-html",
    enforce: "pre",
    config() {
      return {
        build: {
          rollupOptions: {
            input: resolve(rootDir, fileName),
          },
        },
      };
    },
    configResolved(config) {
      outDir = resolve(config.root, config.build.outDir);
      writeOutput = config.build.write !== false;
    },
    closeBundle() {
      if (!writeOutput) return;
      const source = resolve(outDir, fileName);
      if (!existsSync(source)) return;
      const html = readFileSync(source);
      for (const alias of STATIC_SITE_HTML_ALIASES) {
        writeFileSync(resolve(outDir, alias), html);
      }
    },
    configureServer(server) {
      server.middlewares.use(semioEmojiIndexHtmlRootRewrite(entry));
      return () => {
        server.middlewares.use(semioEmojiIndexHtmlSpaFallbackRewrite(entry));
      };
    },
    configurePreviewServer(server) {
      server.middlewares.use(semioEmojiIndexHtmlRootRewrite(entry));
      return () => {
        server.middlewares.use(semioEmojiIndexHtmlSpaFallbackRewrite(entry));
      };
    },
  };
}

/** @emoji 🏷️ Vite: brand-aware host chrome — rewrites the `<title>` to the brand's `windowTitle`, serves/copies the brand mark at `/🛡️favicon.svg` (ICO only when the brand provides one), and writes the static-deploy markers above; no brand ⇒ canonical semio favicons (still with `.nojekyll`). */
export function semioBrandHtmlVitePlugins(repoRoot: string, brand: ShellBrandHostChrome | undefined): OwnedBuildPlugin[] {
  if (!brand) return [...semioFaviconVitePlugin(repoRoot), ...staticDeployMarkerVitePlugins(undefined)];
  return [
    ...faviconVitePlugins({ svgMarkup: brand.logoSvg, icoPath: brand.faviconIcoPath ? resolve(repoRoot, brand.faviconIcoPath) : undefined }),
    ...staticDeployMarkerVitePlugins(brand.cnameHost),
    {
      name: "semio-brand-html",
      transformIndexHtml: {
        order: "pre",
        handler: (html) => html.replace(/<title>[^<]*<\/title>/, `<title>${brand.windowTitle}</title>`),
      },
    },
  ];
}

/** @emoji 🗣️ One text of a host document in one language: a BCP 47 tag and the text in that language. */
export type SemioHostHtmlText = { readonly lang: string; readonly text: string };

/** @emoji 🗣️ A text of a host document: one string in the document's language, or — for a document that assumes no
 * language — the same text once per offered language, in the order the languages are offered. */
export type SemioHostHtmlTexts = string | readonly SemioHostHtmlText[];

/** @emoji 🧬️ Full-document spec for a semio host `🌐️.html`: title, entry module, mount point, and
 * optional CSP + pre-mount loading copy — everything an app needs beyond the shared boot scripts so it
 * stops hand-authoring its own splash screen and `<style>` blocks. A document whose `title` is given per language
 * declares no language of its own: `<html>` carries no `lang`, the title joins the languages, every other text marks
 * each part with its `lang`, and the app sets `lang` once it knows its reader's. `noscript` is what a reader without
 * scripts is told instead of the app. */
export type SemioHostHtmlSpec = {
  readonly title: SemioHostHtmlTexts;
  readonly entry: string;
  readonly rootId?: string;
  readonly bodyClass?: string;
  readonly csp?: string;
  readonly loading?: { readonly title: SemioHostHtmlTexts };
  readonly noscript?: SemioHostHtmlTexts;
  /** 🌐️ Custom domain this app's static build deploys to (e.g. GitHub Pages) — written verbatim into a
   * `CNAME` file at the build root, alongside the always-written `.nojekyll` marker. */
  readonly cnameHost?: string;
};

const SEMIO_HOST_TEXT_SEPARATOR = " · ";

function semioHostEscapedText(text: string): string {
  return text.replace(/&/gu, "&amp;").replace(/</gu, "&lt;").replace(/>/gu, "&gt;").replace(/"/gu, "&quot;");
}

/** @emoji 🏷️ The title of a host document as plain text: the one string, or every language's title joined. */
export function semioHostTitleText(title: SemioHostHtmlTexts): string {
  return typeof title === "string" ? title : title.map((part) => semioHostEscapedText(part.text)).join(SEMIO_HOST_TEXT_SEPARATOR);
}

/** @emoji 🗣️ A text as markup: the one string as it is, or one element per language carrying its `lang`. */
function semioHostTextsHtml(texts: SemioHostHtmlTexts, tag: "span" | "p", attributes = ""): string {
  if (typeof texts === "string") return tag === "span" ? texts : `<${tag}${attributes}>${texts}</${tag}>`;
  return texts.map((part) => `<${tag} lang="${semioHostEscapedText(part.lang)}"${attributes}>${semioHostEscapedText(part.text)}</${tag}>`).join(tag === "span" ? SEMIO_HOST_TEXT_SEPARATOR : "");
}

/** @emoji 🪧️ Pre-mount placeholder markup shown inside `#{rootId}` until the entry module mounts and
 * replaces it — inline-styled so it renders before any external stylesheet loads. */
function semioHostLoadingHtml(loading: SemioHostHtmlSpec["loading"]): string {
  if (!loading) {
    return "";
  }
  return `<div style="display:flex;align-items:center;justify-content:center;height:100%;font:14px system-ui,sans-serif">${semioHostTextsHtml(loading.title, "span")}</div>`;
}

/** @emoji 🚫️ What a reader without scripts gets: the boot style keeps `<body>` hidden until a script reveals it, so each
 * paragraph makes itself visible. */
function semioHostNoscriptHtml(noscript: SemioHostHtmlSpec["noscript"]): string {
  if (!noscript) {
    return "";
  }
  return `\n    <noscript>${semioHostTextsHtml(noscript, "p", ' style="visibility:visible;margin:1em;font:14px system-ui,sans-serif"')}</noscript>`;
}

/** @emoji 📄️ Generates a complete semio host `🌐️.html` document: doctype/head (title, favicon,
 * optional CSP, boot style + appearance/theme scripts) and body (`#{rootId}` mount with pre-mount loading
 * copy, the reveal script, and the entry module script) — the single source of truth
 * {@link semioHostHtmlVitePlugin} renders from, reusable as-is by non-Vite hosts such as a VS Code webview. */
export function semioHostHtmlString(spec: SemioHostHtmlSpec): string {
  const rootId = spec.rootId ?? "root";
  const cspTag = spec.csp ? `<meta http-equiv="Content-Security-Policy" content="${spec.csp}" />\n    ` : "";
  const headTags = semioHostBootHeadTags()
    .map((tag) => (tag.tag === "style" ? `<style>${tag.children}</style>` : `<script>${tag.children}</script>`))
    .join("\n    ");
  return `<!doctype html>
<html${typeof spec.title === "string" ? ' lang="en"' : ""}>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    ${cspTag}<title>${semioHostTitleText(spec.title)}</title>
    ${SEMIO_FAVICON_HEAD_HTML}
    ${headTags}
  </head>
  <body${spec.bodyClass ? ` class="${spec.bodyClass}"` : ""}>
    <div id="${rootId}">${semioHostLoadingHtml(spec.loading)}</div>${semioHostNoscriptHtml(spec.noscript)}
    <script>${PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT}</script>
    <script type="module" src="${spec.entry}"></script>
  </body>
</html>
`;
}

/** @emoji 🎬️ Vite: renders {@link semioHostHtmlString} as the app's `🌐️.html` on every request/build
 * (full-document replace, `order: "pre"` so later plugins such as `@vitejs/plugin-react`'s HMR preamble
 * still layer on top), bundles semio favicon serving ({@link semioFaviconVitePlugin}), and writes the
 * static-deploy markers ({@link staticDeployMarkerVitePlugins} — `.nojekyll` always, `CNAME` when
 * `spec.cnameHost` is set) — one call wires an app's whole boot + deploy surface instead of a
 * hand-authored `🌐️.html` plus a separate build-output step. */
export function semioHostHtmlVitePlugin(repoRoot: string, spec: SemioHostHtmlSpec): OwnedBuildPlugin[] {
  return [
    ...semioFaviconVitePlugin(repoRoot),
    ...staticDeployMarkerVitePlugins(spec.cnameHost),
    {
      name: "semio-host-html",
      transformIndexHtml: {
        order: "pre",
        handler() {
          return semioHostHtmlString(spec);
        },
      },
    },
  ];
}
//#endregion 🔖️HostHtmlPlugin

//#region 🔖️StatusSurfaceHtml
/** @emoji 🎨️ Light/dark background+foreground hex pair mirrored from {@link PLAYGROUND_PLAY_BOOT_INLINE_STYLE}
 * / {@link PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT} — this file has no `../🎨️styling/🔣️.json` import, so these are the
 * canonical values already baked into every other boot surface here, not new ones. */
const SEMIO_STATUS_SURFACE_COLORS = { lightBg: "#f7f3e3", lightFg: "#001117", darkBg: "#001117", darkFg: "#f7f3e3" } as const;

const SEMIO_STATUS_SURFACE_GLYPH: Record<"empty" | "error" | "loading", string> = { empty: "◌️", error: "⚠️", loading: "…" };

function semioStatusSurfaceInlineStyle(): string {
  const c = SEMIO_STATUS_SURFACE_COLORS;
  return `html{color-scheme:light dark}html,body{height:100dvh;margin:0}body{background-color:${c.lightBg};color:${c.lightFg};display:flex;align-items:center;justify-content:center;font-family:system-ui,sans-serif}@media (prefers-color-scheme: dark){body{background-color:${c.darkBg};color:${c.darkFg}}}`;
}

/** @emoji 🚦️ Minimal, standalone status document (empty/error/loading) for host-agnostic contexts that
 * can't run React — e.g. a WebView2 navigation-failure page — fully inline-styled so it renders with zero
 * external CSS/JS dependency, reusing the same light/dark hex values every other boot surface in this
 * file uses. */
export function statusSurfaceHtml(spec: { readonly kind: "empty" | "error" | "loading"; readonly title: string; readonly description?: string }): string {
  const description = spec.description ? `<p style="margin:8px 0 0;font-size:14px;opacity:0.72">${spec.description}</p>` : "";
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>${spec.title}</title>
    <style>${semioStatusSurfaceInlineStyle()}</style>
  </head>
  <body data-status-kind="${spec.kind}">
    <div style="text-align:center;max-width:28rem;padding:0 24px">
      <p style="margin:0 0 8px;font-size:28px" aria-hidden="true">${SEMIO_STATUS_SURFACE_GLYPH[spec.kind]}</p>
      <p style="margin:0;font-size:16px;font-weight:600">${spec.title}</p>
      ${description}
    </div>
  </body>
</html>
`;
}
//#endregion 🔖️StatusSurfaceHtml

//#region 🔖️ServeClose
/** @emoji 🧷️ The part of a dev serve's HTTP server that closing it needs; an HTTP/2 server has no `closeAllConnections`. */
export type ServeCloseHttpServer = {
  close(callback?: (error?: Error) => void): unknown;
  closeAllConnections(): void;
  readonly listening: boolean;
  emit(event: "close"): boolean;
};

/** @emoji 🔎️ Whether a serve's HTTP server can close every connection it holds. */
export function isServeCloseHttpServer(value: object | null): value is ServeCloseHttpServer {
  return value !== null && typeof (value as Partial<ServeCloseHttpServer>).closeAllConnections === "function" && typeof (value as Partial<ServeCloseHttpServer>).close === "function";
}

/** @emoji 🚪️ Makes closing a dev serve mean what Vite asks of it under every runtime: every open connection gone, `close`
 * emitted once, the callback called. Vite destroys the sockets it tracked and then waits for `server.close`; Bun 1.3's
 * `node:http` ignores `socket.destroy()` on a served connection, so its graceful `close` never completes while a browser holds
 * a keep-alive connection. `server.restart()` (every edit of the config entry) then stopped listening and never listened
 * again, shutdown hung, and every resource released on the server's `close` event (source watchers, the stream channel)
 * stayed live — measured: bun 1.3.14 unreachable 10 s after a restart under a connected page, node 24 restarted in 4 ms
 * (ticket 26/09/23 F2, `📓️wp-f2.md`). `closeAllConnections` is the runtime's own "close every connection": on Node it
 * destroys them and `close` proceeds as usual; on Bun it stops the server outright, so the plugin completes the close.
 *
 * @see https://nodejs.org/api/http.html#servercloseallconnections
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/♨️hot-update/🟦️.ts */
export function semioServeCloseVitePlugin() {
  return {
    name: "semio-serve-close",
    apply: "serve" as const,
    configureServer(server: { readonly httpServer: object | null }) {
      const httpServer = server.httpServer;
      if (!isServeCloseHttpServer(httpServer)) return;
      const close = httpServer.close.bind(httpServer);
      httpServer.close = (callback) => {
        const listening = httpServer.listening;
        httpServer.closeAllConnections();
        if (!listening || httpServer.listening) return close(callback);
        process.nextTick(() => {
          httpServer.emit("close");
          callback?.();
        });
        return httpServer;
      };
    },
  };
}
//#endregion 🔖️ServeClose

//#region 🔖️ServeUpgrade
/** @emoji 🧦️ The part of the socket of an HTTP upgrade that ending it softly needs. */
export type ServeUpgradeSocket = {
  destroySoon?: () => void;
  readonly writable: boolean;
  readonly writableFinished: boolean;
  end(): unknown;
  destroy(): unknown;
  once(event: "finish", listener: () => void): unknown;
};

/** @emoji 🧦️ The part of a dev serve's HTTP server that sees every upgrade first. */
export type ServeUpgradeHttpServer = {
  prependListener(event: "upgrade", listener: (request: unknown, socket: ServeUpgradeSocket) => void): unknown;
};

/** @emoji 🔎️ Whether a serve's HTTP server lets a listener see an upgrade before every other one. */
export function isServeUpgradeHttpServer(value: object | null): value is ServeUpgradeHttpServer {
  return value !== null && typeof (value as Partial<ServeUpgradeHttpServer>).prependListener === "function";
}

/** @emoji 🕊️ Ends `socket` once what was written to it is flushed, as Node's `Socket#destroySoon` does: end it while it is
 * writable, then destroy it at once when it is finished, else as soon as it finishes.
 * @see https://github.com/nodejs/node/blob/main/lib/net.js — `Socket.prototype.destroySoon` */
export function destroySoon(socket: ServeUpgradeSocket): void {
  if (socket.writable) socket.end();
  if (socket.writableFinished) socket.destroy();
  else socket.once("finish", () => socket.destroy());
}

/** @emoji 🧦️ Lets every socket an upgrade arrives on be ended softly under every runtime. The dev server's proxy of a
 * WebSocket route ends the browser's socket with `destroySoon` once its upstream answers without upgrading or ends — a
 * backend that restarts or goes away does exactly that — and Bun 1.3's `node:http` hands upgrades a socket without
 * `destroySoon`: the `TypeError` killed the whole dev server whenever its backend went away while a socket was proxied.
 * The listener runs before the proxy's and gives such a socket {@link destroySoon}. */
export function semioServeUpgradeVitePlugin() {
  return {
    name: "semio-serve-upgrade",
    apply: "serve" as const,
    configureServer(server: { readonly httpServer: object | null }) {
      const httpServer = server.httpServer;
      if (!isServeUpgradeHttpServer(httpServer)) return;
      httpServer.prependListener("upgrade", (_request, socket) => {
        if (typeof socket.destroySoon !== "function") socket.destroySoon = () => destroySoon(socket);
      });
    },
  };
}
//#endregion 🔖️ServeUpgrade

/** 🗂️ Canonical repo-relative root of the asset-owned public namespace. */
export const SEMIO_ASSET_ROOT = "🧰️framework/🔨️modules/🖼️assets";

/** @emoji 📂 Resolves and validates the merged Semio asset package root (fonts required). */
export function resolveSemioAssetRoot(repoRoot: string): string {
  const assetsRoot = resolve(repoRoot, SEMIO_ASSET_ROOT);
  const fontDir = resolve(assetsRoot, "🔤️fonts");
  if (!existsSync(assetsRoot) || !statSync(assetsRoot).isDirectory() || !existsSync(fontDir)) {
    throw new Error(`Missing Semio asset root at ${assetsRoot} (expected ${SEMIO_ASSET_ROOT} with 🔤️fonts)`);
  }
  return assetsRoot;
}

function uiAssetsVitePluginsForRoot(assetsRoot: string): OwnedBuildPlugin[] {
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  const serveAssets = createUiAssetsMiddleware(assetsRoot);
  return [
    {
      name: "ui-assets-serve",
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveAssets);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveAssets);
      },
    },
    {
      name: "ui-assets-build",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        if (!existsSync(assetsRoot)) {
          return;
        }
        const dest = resolve(outDir, SEMIO_ASSET_DIRECTORY);
        mkdirSync(outDir, { recursive: true });
        cpSync(assetsRoot, dest, { recursive: true });
      },
    },
  ];
}

/** 🌐️ Serves and copies shared fonts and cursors at `/🖼️assets/*`. */
export function semioAssetsVitePlugin(repoRoot: string): OwnedBuildPlugin[] {
  return uiAssetsVitePluginsForRoot(resolveSemioAssetRoot(repoRoot));
}

/** 🌐️ Serves `/🖼️assets/*` like {@link semioAssetsVitePlugin}, but a build copies only the asset files the written bundle
 * references (the fonts of an imported palette, an icon named in CSS) instead of the whole asset root — for a site that
 * ships a handful of files rather than the library's every font, mesh and image. */
export function semioReferencedAssetsVitePlugin(repoRoot: string): OwnedBuildPlugin[] {
  const assetsRoot = resolveSemioAssetRoot(repoRoot);
  const serveAssets = createUiAssetsMiddleware(assetsRoot);
  const reference = new RegExp(`${SEMIO_ASSET_ROUTE}/[^"'()\\s?#]+`, "gu");
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  const referenced = (directory: string, found: Set<string>): Set<string> => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isDirectory() && entry.name !== SEMIO_ASSET_DIRECTORY) referenced(path, found);
      else if (entry.isFile() && /\.(?:css|html|js|mjs)$/u.test(entry.name)) for (const [match] of readFileSync(path, "utf8").matchAll(reference)) found.add(assetPathFromRequest(match) ?? "");
    }
    return found;
  };
  return [
    {
      name: "semio-referenced-assets",
      configureServer(server) {
        server.middlewares.use(serveAssets);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveAssets);
      },
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput || !existsSync(outDir)) return;
        for (const asset of referenced(outDir, new Set())) {
          if (!asset) continue;
          const target = resolve(outDir, SEMIO_ASSET_DIRECTORY, asset);
          mkdirSync(dirname(target), { recursive: true });
          copyFileSync(resolve(assetsRoot, asset), target);
        }
      },
    },
  ];
}

/** @emoji 🛝️ Playground app kind for Vite play harness config (validated against manifest scan). */
export type PlaygroundRendererPuzzleKind = string;

function namedImportSpecifiersForModule(source: string, moduleId: string): string[] {
  const escaped = moduleId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const re = new RegExp(`import\\s*\\{([^}]+)\\}\\s*from\\s*["']${escaped}["']`, "gs");
  const names: string[] = [];
  let match: RegExpExecArray | null;
  while ((match = re.exec(source))) {
    for (const part of match[1].split(",")) {
      const trimmed = part.trim();
      if (!trimmed) continue;
      const name = trimmed
        .replace(/^type\s+/, "")
        .split(/\s+as\s+/)[0]
        ?.trim();
      if (name) names.push(name);
    }
  }
  return names;
}

/** @emoji 🔁️ Named import specifiers duplicated within the same module import block(s). */
export function duplicateNamedImportsForModule(source: string, moduleId: string): string[] {
  const names = namedImportSpecifiersForModule(source, moduleId);
  const seen = new Set<string>();
  const dupes: string[] = [];
  for (const name of names) {
    if (seen.has(name)) dupes.push(name);
    else seen.add(name);
  }
  return dupes;
}

const PRESENTATION_RENDERER_VITEST_START = "//#region 🧪️Tests";

/** @emoji ✂️ Drops vitest regions from animate present renderer in browser dev. */
export function animatePresentRendererVitestStripPlugin(animatePresentIndexPath: string): OwnedBuildPlugin {
  return {
    name: "animate-present-renderer-vitest-strip",
    enforce: "pre",
    load(id) {
      if (process.env.VITEST) return;
      const filePath = id.split("?")[0];
      if (filePath !== animatePresentIndexPath) return;
      const source = readFileSync(animatePresentIndexPath, "utf8");
      const testsStart = source.indexOf(PRESENTATION_RENDERER_VITEST_START);
      if (testsStart < 0) return source;
      return source.slice(0, testsStart);
    },
  };
}

/** @emoji 🎬️ R3F packages that must resolve once with {@link sceneHostPort} and drei controls. */
export const PLAYGROUND_SCENE_HOST_DEDUPE = ["@react-three/fiber", "@react-three/drei"] as const;

/** @emoji 🎬️ Vite aliases that pin R3F to a single node_modules entry (avoids duplicate Canvas stores). */
export function playgroundSceneHostResolveAliases(repoRoot: string): ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> {
  return [
    { find: /^@react-three\/fiber$/, replacement: resolve(repoRoot, "node_modules/@react-three/fiber/dist/react-three-fiber.esm.js") },
    { find: /^@react-three\/drei$/, replacement: resolve(repoRoot, "node_modules/@react-three/drei/index.js") },
  ];
}

/** @emoji 🎬️ CommonJS leaves an EXCLUDED R3F package still reaches — fiber → `scheduler`; drei → `stats.js`;
 * drei → `tunnel-rat` → (nested) `zustand` → `use-sync-external-store/shim{,/with-selector}.js`, and top-level
 * `zustand/esm/react.mjs` → the same shims. Vite serves an excluded package's files raw and rewrites their bare
 * imports to a prebundled copy only when that dependency IS in the optimizer — so without these entries a fresh
 * serve boots black on `The requested module '…/scheduler/index.js' (or …/with-selector.js) does not provide an
 * export named 'default'` (lanes started before the R3F exclusion landed kept working only because they
 * predated it — measured on :6086, 2026-09-16). Named per file where the package has no ESM entry at all,
 * exactly as Vite's own "exclude an ESM dependency, include its CJS descendants" rule asks; every other
 * package in the two graphs ships an ESM entry (`its-fine`, `suspend-react`, `tunnel-rat`, `zustand`,
 * `react-use-measure`, `stats-gl`, `three-stdlib`, `maath`, …) and is served raw on purpose. */
export const PLAYGROUND_SCENE_HOST_CJS_INCLUDE = ["scheduler", "stats.js", "use-sync-external-store/shim/index.js", "use-sync-external-store/shim/with-selector.js"] as const;

/** @emoji 📦️ ESM packages the EXCLUDED R3F graph reaches through a barrel and that the optimizer must bundle anyway. Every
 * drei control imports `three-stdlib`'s index, and an import from an excluded package is never scanned, so Vite served the
 * whole barrel as separate raw modules on every cold boot: 282 files, 15.7 MB — a third of the shell's boot requests after
 * drei's own barrel was cut (measured on :6580, ticket 26/09/23 F2). Prebundled, it is one module; `three` stays one instance
 * because the optimizer bundles `three` too and shares it between both. */
export const PLAYGROUND_SCENE_HOST_ESM_INCLUDE = ["three-stdlib"] as const;

/** @emoji 🎬️ `optimizeDeps` preset for configs that use {@link playgroundSceneHostResolveAliases}: never prebundle R3F — a `.vite/deps` fiber copy and the aliased ESM entry are two Canvas stores, and drei's `PerspectiveCamera` then throws outside Canvas — but DO prebundle the CJS shims R3F's excluded graph imports ({@link PLAYGROUND_SCENE_HOST_CJS_INCLUDE}). */
export function playgroundSceneHostOptimizeDeps(extra?: Pick<NonNullable<OwnedBuildConfig["optimizeDeps"]>, "include" | "exclude">): NonNullable<OwnedBuildConfig["optimizeDeps"]> {
  const include = ["three", ...PLAYGROUND_SCENE_HOST_CJS_INCLUDE, ...PLAYGROUND_SCENE_HOST_ESM_INCLUDE, ...(extra?.include ?? [])].filter((id) => !PLAYGROUND_SCENE_HOST_DEDUPE.includes(id as (typeof PLAYGROUND_SCENE_HOST_DEDUPE)[number]));
  const exclude = [...PLAYGROUND_SCENE_HOST_DEDUPE, ...(extra?.exclude ?? [])];
  return { include: [...new Set(include)], exclude: [...new Set(exclude)] };
}

//#region 🔖️TileProxyAssetPlugin
/** @emoji 🧩️ Extension implied by a resolved tile URL template's tail (`.png`, `.pbf`, …), `"bin"` if absent. */
function tileProxyExtFromTemplate(template: string): string {
  const clean = template.split(/[?#]/, 1)[0] ?? template;
  const ext = clean.split(".").pop();
  return ext && ext.length <= 4 ? ext : "bin";
}

function contentTypeForTileExt(ext: string): string {
  if (ext === "png") return "image/png";
  if (ext === "pbf" || ext === "mvt") return "application/x-protobuf";
  return "application/octet-stream";
}

const tileProxyTemplateCache = ephemeralMap<string, { readonly template: string; readonly at: number }>("framework.modules.ui.styling.packages.rust.vite.elements.assets.ts.tileProxyTemplateCache");
async function boundedTileProxyBytes(response: Response, maximum: number): Promise<Uint8Array> {
  if (Number(response.headers.get("content-length")) > maximum) { await response.body?.cancel(); throw Error("tile proxy body exceeds its byte budget"); }
  if (!response.body) return new Uint8Array();
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let length = 0;
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      length += value.byteLength;
      if (length > maximum) throw Error("tile proxy body exceeds its byte budget");
      chunks.push(value);
    }
  } finally { await reader.cancel(); }
  const output = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) { output.set(chunk, offset); offset += chunk.byteLength; }
  return output;
}

/** @emoji 🧭️ Resolves a `tile-proxy` spec's `upstream` to a concrete `{z}/{x}/{y}` URL template: used
 * directly when it already contains `{z}`, otherwise treated as a TileJSON endpoint and resolved
 * (cached, 7-day TTL) for the owner-authored request headers. */
async function resolveTileProxyUrlTemplate(upstream: string, userAgent: string, signal: AbortSignal): Promise<string> {
  if (upstream.includes("{z}")) {
    return upstream;
  }
  const now = Date.now();
  const cacheKey = JSON.stringify([upstream, userAgent]);
  const cached = tileProxyTemplateCache.get(cacheKey);
  if (cached && now - cached.at < TILE_PROXY_TRANSPORT_LIMITS_V1.templateTtlMs) {
    return cached.template;
  }
  const res = await fetch(upstream, { headers: { "User-Agent": userAgent }, signal });
  if (!res.ok) {
    throw new Error(`tile proxy upstream TileJSON failed: ${res.status}`);
  }
  const json = JSON.parse(new TextDecoder().decode(await boundedTileProxyBytes(res, TILE_PROXY_TRANSPORT_LIMITS_V1.templateBytes))) as { tiles?: string[] };
  const template = json.tiles?.[0];
  if (typeof template !== "string" || !template.includes("{z}")) {
    throw new Error("tile proxy TileJSON missing tiles URL template");
  }
  tileProxyTemplateCache.set(cacheKey, { template, at: now });
  return template;
}

async function fetchTileProxyTileToCache(cacheRoot: string, upstream: string, z: number, x: number, y: number, userAgent: string, signal: AbortSignal): Promise<{ readonly ok: boolean; readonly ext: string }> {
  const template = await resolveTileProxyUrlTemplate(upstream, userAgent, signal);
  const ext = tileProxyExtFromTemplate(template);
  const filePath = resolve(cacheRoot, `${z}/${x}/${y}.${ext}`);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return { ok: false, ext };
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
  const upstreamRes = await fetch(url, { headers: { "User-Agent": userAgent }, signal });
  if (!upstreamRes.ok) {
    return { ok: false, ext };
  }
  const buf = await boundedTileProxyBytes(upstreamRes, TILE_PROXY_TRANSPORT_LIMITS_V1.tileBytes);
  if (buf.length === 0) {
    return { ok: false, ext };
  }
  signal.throwIfAborted();
  await writeFile(filePath, buf);
  return { ok: true, ext };
}

/** @emoji 🌐️ Connect middleware serving `{route}/{z}/{x}/{y}.{ext}` tiles from `cacheRoot`, fetching
 * (and caching) from the declared upstream on a miss. */
function createTileProxyMiddleware(route: string, cacheRoot: string, upstream: string, mode: AssetDeliveryModeV1, userAgent: string): OwnedBuildMiddleware {
  const prefix = route.endsWith("/") ? route : `${route}/`;
  const pattern = new RegExp(`^${prefix.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}(\\d+)/(\\d+)/(\\d+)\\.(\\w+)(?:\\?.*)?$`);
  return async (req, res, next) => {
    const match = req.url?.match(pattern);
    if (!match) {
      next();
      return;
    }
    const [, zs, xs, ys, ext] = match as unknown as [string, string, string, string, string];
    const z = Number(zs);
    const x = Number(xs);
    const y = Number(ys);
    const filePath = resolve(cacheRoot, `${z}/${x}/${y}.${ext}`);
    const relToRoot = relative(cacheRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
      next();
      return;
    }
    if (existsSync(filePath)) {
      serveFileWithValidatorsV1(req, res, filePath, contentTypeForTileExt(ext));
      return;
    }
    if (mode === "bundle") {
      res.statusCode = 404;
      res.end();
      return;
    }
    const lifetime = new AbortController();
    const retire = () => lifetime.abort();
    req.once("aborted", retire);
    req.socket.once("close", retire);
    res.once("close", retire);
    const expiry = setTimeout(retire, TILE_PROXY_TRANSPORT_LIMITS_V1.requestMs);
    const signal = lifetime.signal;
    try {
      const result = await fetchTileProxyTileToCache(cacheRoot, upstream, z, x, y, userAgent, signal);
      if (!result.ok) {
        res.statusCode = 404;
        res.end();
        return;
      }
      serveFileWithValidatorsV1(req, res, filePath, contentTypeForTileExt(result.ext));
    } catch {
      if (!req.socket.destroyed && !res.destroyed && !res.writableEnded) { res.statusCode = 502; res.end(); }
    } finally {
      clearTimeout(expiry);
      req.removeListener("aborted", retire);
      req.socket.removeListener("close", retire);
      res.removeListener("close", retire);
    }
  };
}

/** @emoji 🌐️ Generic dev/preview/build Vite plugin pair for one `tile-proxy` asset spec — replaces the
 * owner-authored route and request coordinates into the neutral transport. */
export function tileProxyVitePlugin(repoRoot: string, spec: TileProxyAssetSpecV1, mode: AssetDeliveryModeV1 = "fetch"): OwnedBuildPlugin[] {
  const admitted = parseTileProxyAssetSpecV1(spec);
  const cacheRoot = resolve(repoRoot, admitted.cache);
  const serveTiles = createTileProxyMiddleware(admitted.route, cacheRoot, admitted.upstream, mode, admitted.userAgent);
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  const plugins: OwnedBuildPlugin[] = [
    {
      name: `tile-proxy-serve${spec.route}`,
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveTiles);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveTiles);
      },
    },
  ];
  if (mode === "bundle") {
    plugins.push({
      name: `tile-proxy-build${spec.route}`,
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        const dist = outDir;
        mkdirSync(dist, { recursive: true });
        if (existsSync(cacheRoot)) {
          cpSync(cacheRoot, resolve(dist, spec.route.replace(/^\//, "")), { recursive: true });
        }
      },
    });
  }
  return plugins;
}

//#endregion 🔖️TileProxyAssetPlugin

/** @emoji 🦀️ Vite `optimizeDeps.exclude` entries for wasm-bindgen flow modules (must not be prebundled). */
export const FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE = [
  "@semio-tech/flow-module-core",
  "@semio-tech/flow-module-math",
  "@semio-tech/flow-module-text",
  "@semio-tech/flow-module-logic",
  "@semio-tech/flow-module-dictionary",
  "@semio-tech/flow-module-list",
  "@semio-tech/flow-module-draw",
] as const;

/** @emoji 🧭️ Workspace Vite resolve preset: dedupe, fs.allow, optimizeDeps.exclude, scene-host aliases. */
export function createWorkspaceViteResolveConfig(repoRoot: string, extraAliases: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> = []): Pick<OwnedBuildConfig, "resolve" | "server" | "optimizeDeps"> {
  return {
    resolve: {
      alias: [...extraAliases],
      dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
    },
    server: {
      fs: { allow: [repoRoot] },
    },
    optimizeDeps: {
      exclude: [...findWorkspacePackages(repoRoot), ...FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE],
    },
  };
}

//#region 🔖️StaticDirAssetPlugin
export function contentTypeForStaticDirAsset(filePath: string): string | undefined {
  if (filePath.endsWith(".js") || filePath.endsWith(".mjs")) {
    return "text/javascript";
  }
  if (filePath.endsWith(".wasm")) {
    return "application/wasm";
  }
  if (filePath.endsWith(".json")) {
    return "application/json";
  }
  if (filePath.endsWith(".png")) {
    return "image/png";
  }
  if (filePath.endsWith(".jpg") || filePath.endsWith(".jpeg")) {
    return "image/jpeg";
  }
  if (filePath.endsWith(".pdf")) {
    return "application/pdf";
  }
  if (filePath.endsWith(".svg")) {
    return "image/svg+xml";
  }
  return undefined;
}

/** @emoji 🏷️ Serves one file with HTTP validators: a weak `ETag` from its size and nanosecond mtime, `Last-Modified`,
 * `Content-Length` and `Cache-Control: no-cache` (always revalidate — a restage rewrites files in place). A request whose
 * `If-None-Match` names the current tag (weak comparison, RFC 9110 §13.1.2), or — without one — whose `If-Modified-Since`
 * is not older than the file, is answered `304` with no body; `HEAD` gets the headers only.
 *
 * 🐛️ Without validators every reload of `s` re-downloaded every staged descriptor, the 8 MB guest font pack and every
 * component core module in full (measured: 15.7 MB of descriptors + 8.4 MB of fonts per warm reload, ticket 26/09/23 F2);
 * with them an unchanged file costs one `304`. */
export function serveFileWithValidatorsV1(req: IncomingMessage, res: ServerResponse, filePath: string, contentType: string | undefined): void {
  const stats = statSync(filePath, { bigint: true });
  const tag = `W/"${stats.size.toString(16)}-${stats.mtimeNs.toString(16)}"`;
  const modifiedSeconds = Number(stats.mtimeMs / 1000n);
  res.setHeader("ETag", tag);
  res.setHeader("Last-Modified", new Date(modifiedSeconds * 1000).toUTCString());
  res.setHeader("Cache-Control", "no-cache");
  if (contentType) res.setHeader("Content-Type", contentType);
  const matches = req.headers["if-none-match"];
  const since = req.headers["if-modified-since"];
  const opaque = (value: string): string => value.trim().replace(/^W\//u, "");
  const notModified =
    typeof matches === "string"
      ? matches.split(",").some((candidate) => candidate.trim() === "*" || opaque(candidate) === opaque(tag))
      : typeof since === "string" && Number.isFinite(Date.parse(since)) && Math.floor(Date.parse(since) / 1000) >= modifiedSeconds;
  if (notModified) {
    res.statusCode = 304;
    res.end();
    return;
  }
  res.setHeader("Content-Length", stats.size.toString());
  if (req.method === "HEAD") {
    res.end();
    return;
  }
  createReadStream(filePath).pipe(res);
}

/** @emoji 🗂️ Connect middleware: serve one `static-dir` spec's files at `{route}/…`. */
function createStaticDirMiddleware(repoRoot: string, spec: StaticDirectoryAssetSpecV1): OwnedBuildMiddleware {
  const fixtureRoot = resolve(repoRoot, spec.root);
  const route = spec.route.endsWith("/") ? spec.route : `${spec.route}/`;
  return (req, res, next) => {
    const rawUrl = req.url ?? "";
    const pathOnly = rawUrl.split(/[?#]/, 1)[0] ?? "";
    let decodedPath = pathOnly;
    try {
      decodedPath = decodeURIComponent(pathOnly);
    } catch {
      next();
      return;
    }
    if (!decodedPath.startsWith(route)) {
      next();
      return;
    }
    const rel = decodedPath.slice(route.length);
    const filePath = resolve(fixtureRoot, rel);
    const relToRoot = relative(fixtureRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
      next();
      return;
    }
    if (!existsSync(filePath) || !statSync(filePath).isFile()) {
      res.statusCode = 404;
      res.end();
      return;
    }
    serveFileWithValidatorsV1(req, res, filePath, contentTypeForStaticDirAsset(filePath));
  };
}

/** @emoji 🖼️ Generic dev/build Vite plugin pair for one `static-dir` asset spec: serves and copies
 * `spec.root` at `spec.route` — replaces the previous `cadFixtureVitePlugin`/`infiniteFixtureVitePlugin`
 * pair (byte-identical serving logic, now route/root-driven instead of hardcoded per fixture tree). */
export function staticDirVitePlugin(repoRoot: string, spec: StaticDirectoryAssetSpecV1): OwnedBuildPlugin[] {
  const serveFixture = createStaticDirMiddleware(repoRoot, spec);
  const fixtureRoot = resolve(repoRoot, spec.root);
  const destName = spec.route.replace(/^\//, "");
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  return [
    {
      name: `static-dir-serve${spec.route}`,
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveFixture);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveFixture);
      },
    },
    {
      name: `static-dir-build${spec.route}`,
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        // 🖼️ `config.build.outDir` is root-relative unless already absolute — `resolve` handles both, so a
        // brand's custom `outDir` (see `ShellBrand.distDir`) is honored instead of assuming `<root>/dist`.
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        if (!existsSync(fixtureRoot)) {
          return;
        }
        const dest = resolve(outDir, destName);
        mkdirSync(outDir, { recursive: true });
        if (existsSync(dest)) rmSync(dest, { recursive: true, force: true });
        cpSync(fixtureRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🪜️ Serves several `static-dir` roots as ONE route table. Each mount answers 404 for a file its
 * root lacks (never the SPA fallback), so two roots on one route shadow each other — the second becomes
 * unreachable — and a parent route registered before a nested one swallows the nested one's requests.
 * The table therefore refuses a route claimed twice and orders the halves by nesting: serving
 * most-specific route first, build copies parent first (a parent copy replaces its destination).
 * Fixture law: `🧫️fixtures/🗂️static-dir-mounts/🔣️.json`. */
export function staticDirMountVitePlugins(repoRoot: string, specs: readonly StaticDirectoryAssetSpecV1[]): OwnedBuildPlugin[] {
  const routes = specs.map((spec) => spec.route.replace(/\/+$/, ""));
  const claimed = routes.find((route, index) => routes.indexOf(route) !== index);
  if (claimed !== undefined) throw new Error(`Two static-dir roots claim one route: ${claimed}`);
  const depth = (index: number) => routes[index]!.split("/").length;
  const halves = specs.map((spec, index) => ({ index, plugins: staticDirVitePlugin(repoRoot, spec) }));
  return [
    ...[...halves].sort((a, b) => depth(b.index) - depth(a.index)).flatMap((half) => half.plugins.filter((plugin) => plugin.apply !== "build")),
    ...[...halves].sort((a, b) => depth(a.index) - depth(b.index)).flatMap((half) => half.plugins.filter((plugin) => plugin.apply === "build")),
  ];
}

//#endregion 🔖️StaticDirAssetPlugin

/** @emoji 🚫️ Directory names [[findWorkspacePackages]] never descends into: installed dependencies and
 * build output, none of which may contribute a workspace package name. */
const WORKSPACE_PACKAGE_SCAN_SKIP: ReadonlySet<string> = new Set(["node_modules", "dist", "target", "storybook-static", "🗑️generated", "🤖️generated"]);

/** @emoji 🧠️ Memo for [[findWorkspacePackages]], keyed by repo root.
 *
 * 🩸️ The scan costs 14–27 s on this tree and every vite config that calls
 * [[createWorkspaceViteResolveConfig]] runs it at module load, so a process loading several configs paid
 * it several times over. The workspace set cannot change under a running dev server — its vite config is
 * evaluated once at boot — so one walk per process is the whole truth. The declared `workspaces` array in
 * the root `package.json` is NOT a substitute: it is missing `@semio-tech/framework-graph-layout-run-rs`,
 * `@semio-tech/framework-tool-run-rs` and `@semio-tech/print-viz-inference`, which must stay out of
 * `optimizeDeps`. */
const workspacePackagesByRoot = new Map<string, string[]>();

/** @emoji 📦️ Every `@semio-tech/*` package name in the workspace tree, for `optimizeDeps.exclude`.
 *
 * 🩸️ Directory entries are classified from `readdirSync`'s own `Dirent`, which never follows a link.
 * The previous `statSync(full).isDirectory()` did follow, and a hub test leaves a **self-referential**
 * link behind (`🌎️hub/📦️packages/🦀️rust/🗑️generated/test-artifacts/linked-ancestor-publication-owner-*
 * → …/test-artifacts`), so the walk recursed into the same directory forever. Every caller of
 * [[createWorkspaceViteResolveConfig]] evaluates this at config-module load, so a single leftover link
 * hung the `📐️cad` and `🌍️world/🎨️r3f` vitest owners — and any dev server sharing that config — with
 * no error and no timeout. Build output carries no workspace package and is skipped outright. */
export function findWorkspacePackages(repoRoot: string): string[] {
  const memoized = workspacePackagesByRoot.get(repoRoot);
  if (memoized !== undefined) return memoized;
  const packages: string[] = [];
  const scan = (dir: string) => {
    let entries;
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (entry.isSymbolicLink()) continue;
      if (WORKSPACE_PACKAGE_SCAN_SKIP.has(entry.name) || entry.name.startsWith(".")) continue;
      const full = resolve(dir, entry.name);
      if (entry.isDirectory()) {
        scan(full);
        continue;
      }
      if (entry.name !== "package.json" || full === resolve(repoRoot, "package.json")) continue;
      try {
        const pkg = JSON.parse(readFileSync(full, "utf8"));
        if (pkg.name && typeof pkg.name === "string" && pkg.name.startsWith("@semio-tech/")) packages.push(pkg.name);
      } catch {
        /* an unreadable or malformed manifest declares no workspace package */
      }
    }
  };
  scan(repoRoot);
  workspacePackagesByRoot.set(repoRoot, packages);
  return packages;
}

/** 🗺️ Contributes tile transport only when the caller explicitly registers it. */
export const TILE_PROXY_ASSET_PROVIDER_V1: AssetDeliveryProviderV1 = Object.freeze<AssetDeliveryProviderV1>({
  kind: "tile-proxy",
  middleware: (context, declaration) => {
    const spec = parseTileProxyAssetSpecV1(declaration);
    return createTileProxyMiddleware(spec.route, resolve(context.root, spec.cache), spec.upstream, context.mode, spec.userAgent);
  },
  plugins: (context, declaration) => tileProxyVitePlugin(context.root, parseTileProxyAssetSpecV1(declaration), context.mode),
});

/** 🥽️ Contributes catalog transport only when the caller explicitly registers it. */
export const MESH_COLLECTION_ASSET_PROVIDER_V1: AssetDeliveryProviderV1 = Object.freeze<AssetDeliveryProviderV1>({
  kind: "mesh-collection",
  middleware: (context, declaration) => createMeshCollectionMiddleware(context.root, admitBuildAssetSpecV1(declaration, "MeshCollectionAssetSpecV1")),
  plugins: (context, declaration) => meshCollectionVitePlugin(context.root, admitBuildAssetSpecV1(declaration, "MeshCollectionAssetSpecV1")),
});

/** 🗂️ Contributes directory transport only when the caller explicitly registers it. */
export const STATIC_DIRECTORY_ASSET_PROVIDER_V1: AssetDeliveryProviderV1 = Object.freeze<AssetDeliveryProviderV1>({
  kind: "static-dir",
  middleware: (context, declaration) => createStaticDirMiddleware(context.root, admitBuildAssetSpecV1(declaration, "StaticDirectoryAssetSpecV1")),
  plugins: (context, declaration) => staticDirVitePlugin(context.root, admitBuildAssetSpecV1(declaration, "StaticDirectoryAssetSpecV1")),
});

if (import.meta.vitest) {
  const { createAssetHttpServerV1, createAssetBuildPluginsV1, resolveAssetDeliveryModeV1 } = await import("../../../../🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts");
  const { MESH_DELIVERY_CATALOG } = await import("../../../../🖼️assets/🥽️mesh/📇️catalog/🟦️.ts");
  const { registerTests1 } = await import("../../🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts");
  await registerTests1(import.meta.vitest, { MESH_DELIVERY_CATALOG, TILE_PROXY_ASSET_PROVIDER_V1, MESH_COLLECTION_ASSET_PROVIDER_V1, STATIC_DIRECTORY_ASSET_PROVIDER_V1, PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT, PLAYGROUND_PLAY_BOOT_INLINE_STYLE, PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, PLAYGROUND_WASM_STUB_PREFIX, SEMIO_ASSET_ROOT, SEMIO_FAVICON_HEAD_HTML, contentTypeForStaticDirAsset, createWorkspaceViteResolveConfig, findWorkspacePackages, isPlaygroundOptimizedDepUrl, playgroundOptimizedDepUrlPrefix, meshCollectionVitePlugin, createAssetBuildPluginsV1, playgroundFlowWasmDevStubPlugin, playgroundPlayBootHtmlPlugin, playgroundSceneHostOptimizeDeps, playgroundSceneHostResolveAliases, playgroundWasmStubKey, resolveAssetDeliveryModeV1, resolveSemioAssetRoot, rewriteSpaFallbackToEmojiEntry, semioFaviconSources, semioFaviconSvgMarkup, semioFaviconVitePlugin, semioHostHtmlString, semioHostHtmlVitePlugin, createAssetHttpServerV1, staticDirVitePlugin, statusSurfaceHtml, tileProxyVitePlugin }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🔖️ViteElementsAssets
