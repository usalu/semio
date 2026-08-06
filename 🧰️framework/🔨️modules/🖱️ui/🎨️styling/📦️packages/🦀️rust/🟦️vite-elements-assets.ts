// #region 🧲️Header
/** @emoji 🌐️ Vite plugin: serve and copy `framework/ui/asset` at `/asset/*` (fonts, cursors, …). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import mdx from "@mdx-js/rollup";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import { createServer, type Server } from "node:http";
import { cpSync, createReadStream, existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Connect, Plugin } from "vite";
import { defineConfig, type UserConfig } from "vite";
import {
  PLAYGROUND_PORTS,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  playgroundDevPortString,
  playgroundPlayViteDefine,
  playgroundPortEnv,
  playgroundTestPort,
  playgroundTestPortString,
  type PlaygroundHostKind,
} from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";
import type { PlaygroundAssetSpec } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";
// #endregion 🔌️Adapters

export type { PlaygroundAssetSpec };

export {
  PLAYGROUND_PORTS,
  allPlaygroundReservedPorts,
  playgroundDevPort,
  playgroundDevPortString,
  playgroundPortEnv,
  playgroundTestPort,
  playgroundTestPortString,
  type PlaygroundHostKind,
};

//#region 🔖️ViteElementsAssets
/** @emoji 📦️ Relative-base Vite build defaults for playground static sites (iframe + subdomain safe). */
export function playgroundStaticSiteBuildOptions(overrides?: UserConfig["build"]): NonNullable<UserConfig["build"]> {
  return {
    target: "esnext",
    outDir: "dist",
    emptyOutDir: true,
    ...overrides,
  };
}

/** @emoji 🚀️ Production Vite `build` defaults: minify, strip console/debugger, no sourcemaps. */
export function semioViteProductionBuild(overrides?: UserConfig["build"]): NonNullable<UserConfig["build"]> {
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

/** @emoji 🔗️ True when a request targets Vite prebundled `node_modules/.vite/deps` chunks. */
export function isPlaygroundOptimizedDepUrl(url: string): boolean {
  return url.includes("/node_modules/.vite/deps/");
}

/** @emoji 🧱️ Stubs vitest and testing-library when test regions enter the browser graph. */
export function playgroundVitestDevStubPlugin(): Plugin {
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

const PLAYGROUND_COMPOSE_SKETCHPAD_STUB_ID = "\0playground-compose-sketchpad-stub";
const PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID = "\0playground-compose-sketchpad-mdx-stub";

const PLAYGROUND_WASM_STUB_PREFIX = "\0playground-wasm-stub/";

function playgroundWasmStubKey(cleanId: string): string {
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
export class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadFixtureJson() {} fixtureJson() { return "{}"; } setCatalogueJson() {} catalogueJson() { return "[]"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } sliderOverlayStateJson() { return '{"sliders":[]}'; } selectionUnionBoundsScreenJson() { return "{}"; } selectionPreviewPointsJson() { return "[]"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return "[]"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return "{}"; } pickTargetsAtScreenJson() { return "[]"; } previewText() { return ""; } preselectWidgetIdsJson() { return "[]"; } previewOffWidgetIds() { return "[]"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return ""; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{"x":0,"y":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}
export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } cameraJson() { return '{"x":0,"y":0,"zoom":1}'; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }
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
export const boardRedrawHandlesFixtureJson = wasmJson;
export const boardRedrawLayoutFixtureJson = wasmJson;
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
export function playgroundFlowWasmDevStubPlugin(repoRoot: string): Plugin {
  return {
    name: "playground-flow-wasm-dev-stub",
    enforce: "pre",
    resolveId(id, importer) {
      if (!importer || id.startsWith(PLAYGROUND_WASM_STUB_PREFIX)) return undefined;
      const cleanId = id.split("?", 1)[0] ?? id;
      const isWasmPkg = cleanId.includes("/pkg/") || cleanId.endsWith(".wasm") || cleanId === "@semio-tech/flow-core" || cleanId === "@semio-tech/flow-core/pkg/flow_core.js" || cleanId === "@semio-tech/flow-core/flow_core.js";
      if (!isWasmPkg) return undefined;
      if (cleanId.startsWith(".")) {
        if (existsSync(resolve(dirname(importer), cleanId))) return undefined;
        return `${PLAYGROUND_WASM_STUB_PREFIX}${playgroundWasmStubKey(cleanId)}`;
      }
      const workspacePkg = cleanId.match(/^(@semio-tech\/[^/]+)(?:\/(.+))?$/);
      const candidates: string[] = [];
      if (workspacePkg) {
        const [, pkgName, subpath] = workspacePkg;
        candidates.push(...workspaceWasmPkgResolveCandidates(repoRoot, pkgName, subpath));
      } else {
        candidates.push(resolve(repoRoot, cleanId));
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

/** @emoji 🧱️ Stubs compose-sketchpad when the monolithic play renderer is bundled outside the s playground. */
export function playgroundComposeSketchpadStubPlugin(repoRoot: string): Plugin {
  const sketchpadRoot = resolve(repoRoot, "compose/client/lib/sketchpad/js");
  const sketchpadIndex = resolve(sketchpadRoot, "index.ts");
  return {
    name: "playground-compose-sketchpad-stub",
    enforce: "pre",
    resolveId(id) {
      const cleanId = id.split("?", 1)[0] ?? id;
      if (cleanId.endsWith(".mdx") && cleanId.includes("sketchpad") && cleanId.includes("/page/")) {
        return PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID;
      }
      if (cleanId.includes("/compose/client/lib/sketchpad/js/page/") && cleanId.endsWith(".mdx")) {
        return PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID;
      }
      if (
        id === "@semio-tech/compose-sketchpad" ||
        id === "@semio-tech/compose-sketchpad/boot" ||
        id.startsWith("@semio-tech/compose-sketchpad/") ||
        id === sketchpadIndex ||
        id === sketchpadRoot ||
        (id.startsWith(`${sketchpadRoot}/`) && !id.endsWith(".mdx"))
      ) {
        return PLAYGROUND_COMPOSE_SKETCHPAD_STUB_ID;
      }
      return undefined;
    },
    load(id) {
      if (id === PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID) {
        return "export default function SketchpadMdxStub() { return null; }";
      }
      if (id !== PLAYGROUND_COMPOSE_SKETCHPAD_STUB_ID) return;
      return `export const COMPOSE_SKETCHPAD_PROGRAM_ID = "compose.sketchpad";
export async function ensureSketchpadPlatform() {
  throw new Error("compose-sketchpad is only available in the s playground");
}
export function buildSketchpadPlatformDefinition() {
  return { id: COMPOSE_SKETCHPAD_PROGRAM_ID, name: "Compose Sketchpad", apiVersion: "1", apps: [], createPlatformApi: () => ({}) };
}`;
    },
  };
}

/** @emoji 📄️ MDX support for sketchpad when manifest declares `sketchpad-mdx`, or a stub elsewhere. */
export function playgroundComposeSketchpadVitePlugins(repoRoot: string, enableSketchpadMdx: boolean): Plugin[] {
  const sketchpadIndex = resolve(repoRoot, "compose/client/lib/sketchpad/js/index.ts");
  const mdxStubPlugins: Plugin[] = [
    {
      name: "playground-compose-sketchpad-mdx-stub-load",
      enforce: "pre",
      resolveId(id) {
        const cleanId = id.split("?", 1)[0] ?? id;
        if (cleanId.endsWith(".mdx") && cleanId.includes("sketchpad") && cleanId.includes("/page/")) {
          return PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID;
        }
        return undefined;
      },
      load(id) {
        if (id === PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID) {
          return "export default function SketchpadMdxStub() { return null; }";
        }
        return undefined;
      },
    },
  ];
  if (enableSketchpadMdx) {
    const sketchpadMdxStubSource = "export default function SketchpadMdxStub() { return null; }";
    return [
      {
        name: "playground-compose-sketchpad-docs-mdx-stub",
        enforce: "pre",
        resolveId(id) {
          const cleanId = id.split("?", 1)[0] ?? id;
          if (cleanId.endsWith(".mdx") && cleanId.includes("/compose/client/lib/sketchpad/")) {
            return PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID;
          }
          return undefined;
        },
        load(id) {
          const cleanId = id.split("?", 1)[0] ?? id;
          if (id === PLAYGROUND_COMPOSE_SKETCHPAD_MDX_STUB_ID) {
            return sketchpadMdxStubSource;
          }
          if (cleanId.endsWith(".mdx") && cleanId.includes("/compose/client/lib/sketchpad/")) {
            return sketchpadMdxStubSource;
          }
          return undefined;
        },
      },
    ];
  }
  return [playgroundComposeSketchpadStubPlugin(repoRoot), ...mdxStubPlugins];
}

/** @emoji 🧱️ Stubs Playwright when test-only regions are pulled into the browser graph. */
export function playgroundPlaywrightDevStubPlugin(): Plugin {
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
export function playgroundStaleOptimizeDepPlugin(): Plugin {
  return {
    name: "playground-stale-optimize-dep",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!isPlaygroundOptimizedDepUrl(req.url ?? "")) {
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

/** @emoji 🖼️ Dev/preview CSP so playgrounds can be iframe-embedded locally. */
export function playgroundIframeEmbedHeadersPlugin(): Plugin {
  const useHeaders: Connect.NextHandleFunction = (_req, res, next) => {
    res.setHeader("Content-Security-Policy", "frame-ancestors *");
    res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
    res.setHeader("Cross-Origin-Embedder-Policy", "credentialless");
    next();
  };
  return {
    name: "playground-iframe-embed-headers",
    configureServer(server) {
      server.middlewares.use(useHeaders);
    },
    configurePreviewServer(server) {
      server.middlewares.use(useHeaders);
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

function createUiAssetsMiddleware(assetsRoot: string): Connect.NextHandleFunction {
  const assetsRootResolved = resolve(assetsRoot);
  return (req, res, next) => {
    if (!req.url?.startsWith("/asset/")) {
      next();
      return;
    }
    const rel = decodeURIComponent(req.url.slice("/asset/".length).split(/[?#]/, 1)[0] ?? "");
    const filePath = resolve(assetsRootResolved, rel);
    const relToRoot = relative(assetsRootResolved, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    const contentType = contentTypeForUiAsset(filePath);
    if (contentType) {
      res.setHeader("Content-Type", contentType);
    }
    createReadStream(filePath).pipe(res);
  };
}

//#region 🔖️MeshCollectionAssetPlugin
/** @emoji 🌐️ Connect middleware: serve a `mesh-collection` spec's GLBs at `{route}/<name>.glb` (first
 * matching root wins; `{route}/<placeholder-basename>` always serves `spec.placeholder`). Generalizes
 * the previous puzzle-3d-only `/mesh/*` middleware — driven entirely by the spec, no app names. */
function createMeshCollectionMiddleware(repoRoot: string, spec: Extract<PlaygroundAssetSpec, { kind: "mesh-collection" }>): Connect.NextHandleFunction {
  const route = spec.route.endsWith("/") ? spec.route : `${spec.route}/`;
  const rootsResolved = spec.roots.map((root) => resolve(repoRoot, root));
  const placeholderResolved = resolve(repoRoot, spec.placeholder);
  const placeholderBasename = placeholderResolved.split(/[\\/]/).pop()!;
  return (req, res, next) => {
    if (!req.url?.startsWith(route)) {
      next();
      return;
    }
    const rawName = decodeURIComponent(req.url.slice(route.length).split(/[?#]/, 1)[0] ?? "");
    if (rawName === placeholderBasename) {
      if (!existsSync(placeholderResolved) || !statSync(placeholderResolved).isFile()) {
        next();
        return;
      }
      res.setHeader("Content-Type", "model/gltf-binary");
      createReadStream(placeholderResolved).pipe(res);
      return;
    }
    for (const meshRootResolved of rootsResolved) {
      const filePath = resolve(meshRootResolved, rawName);
      const relToRoot = relative(meshRootResolved, filePath);
      if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
        continue;
      }
      if (!existsSync(filePath) || !statSync(filePath).isFile()) {
        continue;
      }
      res.setHeader("Content-Type", "model/gltf-binary");
      createReadStream(filePath).pipe(res);
      return;
    }
    next();
  };
}

/** @emoji 📦️ Copies every `.glb` under `roots` into a static `dest` tree (first match per basename wins). */
function copyMeshCollectionGlbs(roots: readonly string[], dest: string): void {
  mkdirSync(dest, { recursive: true });
  const copied = new Set<string>();
  for (const root of roots) {
    if (!existsSync(root)) {
      continue;
    }
    for (const entry of readdirSync(root)) {
      if (!entry.endsWith(".glb") || copied.has(entry)) {
        continue;
      }
      const src = resolve(root, entry);
      if (!statSync(src).isFile()) {
        continue;
      }
      cpSync(src, resolve(dest, entry));
      copied.add(entry);
    }
  }
}

/** @emoji 🧊️ Generic dev/build Vite plugin pair for one `mesh-collection` asset spec: serves and
 * copies every GLB under `spec.roots` at `spec.route`, plus `spec.placeholder` as a fallback.
 * `spec.filterFromExamples` is reserved for a future per-locked-example basename filter (no source
 * data populates it yet, so every declared spec currently copies its full collection — matches the
 * puzzle-3d-only plugin this replaces, whose equivalent filter was already permanently inert). */
export function meshCollectionVitePlugin(repoRoot: string, spec: Extract<PlaygroundAssetSpec, { kind: "mesh-collection" }>): Plugin[] {
  const serveMeshes = createMeshCollectionMiddleware(repoRoot, spec);
  const meshRoots = spec.roots.map((root) => resolve(repoRoot, root));
  const placeholderMesh = resolve(repoRoot, spec.placeholder);
  const placeholderBasename = placeholderMesh.split(/[\\/]/).pop()!;
  const destName = spec.route.replace(/^\//, "");
  let outDir = resolve(process.cwd(), "dist");
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
      },
      closeBundle() {
        const dest = resolve(outDir, destName);
        mkdirSync(outDir, { recursive: true });
        copyMeshCollectionGlbs(meshRoots, dest);
        if (existsSync(placeholderMesh)) {
          cpSync(placeholderMesh, resolve(dest, placeholderBasename));
        }
      },
    },
  ];
}
//#endregion 🔖️MeshCollectionAssetPlugin

//#region 🔖️HostHtmlPlugin
/** @emoji 🎬️ Inline shell paint before Tailwind finishes compiling the play stylesheet. */
export const PLAYGROUND_PLAY_BOOT_INLINE_STYLE =
  "html{color-scheme:light dark}html,body,#root{height:100%;margin:0}body{background-color:#f7f3e3;color:#001117}html.dark body{background-color:#001117;color:#f7f3e3}html:not([data-semio-styled]) body{visibility:hidden}";

/** @emoji 🌓️ Synchronous appearance bootstrap for play `🌐️index.html` heads — prefers persisted `ui.chrome.appearance`, else system. */
export const PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT = `(function(){var d=document.documentElement,m=window.matchMedia("(prefers-color-scheme: dark)");var stored=null;try{stored=localStorage.getItem("ui.chrome.appearance")}catch(e){}var dark=stored==="dark"||(stored!=="light"&&m.matches);d.classList.toggle("dark",dark);d.dataset.uiAppearance=dark?"dark":"light";d.style.colorScheme=dark?"dark":"light";if(document.body){document.body.style.colorScheme=dark?"dark":"light";document.body.style.backgroundColor=dark?"#001117":"#f7f3e3";document.body.style.color=dark?"#f7f3e3":"#001117";}})();`;

/** @emoji 👁️ Reveals the play shell after the linked globals stylesheet finishes loading. */
export const PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT = `(function(){function reveal(){document.documentElement.dataset.semioStyled="ready"}var link=document.getElementById("semio-play-styles");if(link){if(link.sheet)reveal();else link.addEventListener("load",reveal,{once:true})}else{reveal()}setTimeout(reveal,8000)})();`;

/** @emoji 🎨️ Synchronous active-theme bootstrap for play `🌐️index.html` heads: reapplies the persisted `UiTheme` snapshot's colors before first paint so non-semio themes don't flash the semio defaults. Runs after {@link PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT} so its resolved light/dark class wins the appearance choice; this script only overrides colors. */
export const PLAYGROUND_PLAY_BOOT_THEME_SCRIPT = `(function(){try{var raw=localStorage.getItem("ui.chrome.theme.snapshot");if(!raw)return;var t=JSON.parse(raw);if(!t||!t.colors)return;var d=document.documentElement;var dark=d.classList.contains("dark");for(var k in t.colors){d.style.setProperty("--color-"+k.replace(/_/g,"-"),t.colors[k])}if(t.spacing)for(var s in t.spacing){d.style.setProperty("--spacing-"+s.replace(/_/g,"-"),t.spacing[s])}d.dataset.uiTheme=t.id;var appearance=t.appearances&&t.appearances[dark?"dark":"light"];var chrome=appearance&&appearance.chrome;function resolveSimple(ref){return ref&&ref.token&&t.colors[ref.token]?t.colors[ref.token]:undefined}var base=chrome&&resolveSimple(chrome.base);var fg=chrome&&resolveSimple(chrome.foreground);if(document.body){if(base)document.body.style.backgroundColor=base;if(fg)document.body.style.color=fg}}catch(e){}})();`;

/** @emoji 🧬️ Boot-time head tags every semio host document shares (color-scheme inline style + synchronous
 * appearance/theme scripts) — single source both {@link semioHostHtmlString} and
 * {@link playgroundPlayBootHtmlPlugin} inject from, so the generalized host and playground never drift. */
function semioHostBootHeadTags(): { readonly tag: string; readonly attrs?: Record<string, string>; readonly children?: string; readonly injectTo: "head-prepend" | "head" }[] {
  return [
    { tag: "style", children: PLAYGROUND_PLAY_BOOT_INLINE_STYLE, injectTo: "head-prepend" },
    { tag: "script", children: PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, injectTo: "head-prepend" },
    { tag: "script", children: PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, injectTo: "head-prepend" },
  ];
}

/** @emoji 🎬️ Vite: inject early appearance + theme + stylesheet link into play `🌐️index.html` to avoid unstyled flashes — additive tag injection onto each play's own hand-authored `🌐️index.html`, sharing its boot-head fragment ({@link semioHostBootHeadTags}) with {@link semioHostHtmlVitePlugin} instead of duplicating the style/script assembly. */
export function playgroundPlayBootHtmlPlugin(): Plugin {
  return {
    name: "playground-play-boot-html",
    transformIndexHtml: {
      order: "pre",
      handler() {
        return {
          tags: [
            ...semioHostBootHeadTags(),
            { tag: "link", attrs: { rel: "stylesheet", href: "./🎨️globals.css", id: "semio-play-styles" }, injectTo: "head" },
            { tag: "script", children: PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, injectTo: "head" },
          ],
        };
      },
    },
  };
}

/** @emoji 🔖️ Canonical semio emblem favicon `<link>` tags for playground and app `🌐️index.html` heads. */
export const SEMIO_FAVICON_HEAD_HTML = `<link rel="icon" href="./favicon.svg" type="image/svg+xml" />\n    <link rel="icon" href="./🖼️favicon.ico" sizes="any" />`;

/** @emoji 🔖️ Repo-root paths for the round dark emblem SVG and ICO fallback (matches {@link SemioLogo}). */
export function semioFaviconSources(repoRoot: string): { readonly svg: string; readonly ico: string } {
  const logoRoot = resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos");
  return {
    svg: resolve(logoRoot, "🔣️emblem_dark_round.svg"),
    ico: resolve(logoRoot, "🖼️favicon_dark_round_32x32.ico"),
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

function createFaviconMiddleware(content: FaviconContent): Connect.NextHandleFunction {
  return (req, res, next) => {
    const url = req.url?.split(/[?#]/, 1)[0];
    if (url === "/favicon.svg" && content.svgMarkup) {
      res.setHeader("Content-Type", "image/svg+xml");
      res.end(content.svgMarkup);
      return;
    }
    if (url === "/🖼️favicon.ico" && content.icoPath && existsSync(content.icoPath)) {
      res.setHeader("Content-Type", "image/x-icon");
      createReadStream(content.icoPath).pipe(res);
      return;
    }
    next();
  };
}

/** @emoji 🔖️ Vite: serve and copy the given favicon content at `/favicon.svg` and `/🖼️favicon.ico`. */
function faviconVitePlugins(content: FaviconContent): Plugin[] {
  const serveFavicon = createFaviconMiddleware(content);
  let outDir = resolve(process.cwd(), "dist");
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
      },
      closeBundle() {
        const dist = outDir;
        mkdirSync(dist, { recursive: true });
        if (content.svgMarkup) {
          writeFileSync(resolve(dist, "favicon.svg"), content.svgMarkup);
        }
        if (content.icoPath && existsSync(content.icoPath)) {
          cpSync(content.icoPath, resolve(dist, "🖼️favicon.ico"));
        }
      },
    },
  ];
}

/** @emoji 🔖️ Vite: serve and copy semio emblem favicons at `/favicon.svg` and `/🖼️favicon.ico`. */
export function semioFaviconVitePlugin(repoRoot: string): Plugin[] {
  const favicons = semioFaviconSources(repoRoot);
  return faviconVitePlugins({ svgMarkup: semioFaviconSvgMarkup(favicons.svg), icoPath: favicons.ico });
}

/** @emoji 🏷️ The host-chrome surface of a shell brand (structural subset of `framework/core/js`'s `ShellBrand`, so this styling layer never imports framework types). */
export type ShellBrandHostChrome = {
  readonly windowTitle: string;
  readonly logoSvg?: string;
  readonly faviconIcoPath?: string;
  /** 🌐️ Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `🌐️CNAME` file at the build root. */
  readonly cnameHost?: string;
};

/** @emoji 🚫️ Vite: writes `dist/.nojekyll` on every build (unconditionally — any static host that runs
 * Jekyll, e.g. GitHub Pages, silently drops files/dirs starting with `_` otherwise, breaking Vite's own
 * `__vite-browser-external-*.js` shim chunk) and `dist/🌐️CNAME` when a brand declares `cnameHost`. */
function staticDeployMarkerVitePlugins(cnameHost: string | undefined): Plugin[] {
  let outDir = resolve(process.cwd(), "dist");
  return [
    {
      name: "static-deploy-markers",
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
      },
      closeBundle() {
        mkdirSync(outDir, { recursive: true });
        writeFileSync(resolve(outDir, ".nojekyll"), "");
        if (cnameHost) writeFileSync(resolve(outDir, "🌐️CNAME"), `${cnameHost}\n`);
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

function semioEmojiIndexHtmlRootRewrite(entry: string): Connect.NextHandleFunction {
  return (req, _res, next) => {
    const url = req.url ?? "";
    if (url === "/" || url.startsWith("/?")) req.url = `${entry}${url.slice(1)}`;
    next();
  };
}

function semioEmojiIndexHtmlSpaFallbackRewrite(entry: string): Connect.NextHandleFunction {
  return (req, _res, next) => {
    const url = req.url ?? "";
    const nextUrl = rewriteSpaFallbackToEmojiEntry(url, entry);
    if (nextUrl !== url) req.url = nextUrl;
    next();
  };
}

/** @emoji 🌐️ Vite: treat hand-authored `🌐️index.html` as the app index (`/` + build input). Vite's default
 * `index.html` name does not match the constitutional emoji entry filename. */
export function semioEmojiIndexHtmlVitePlugin(rootDir: string, fileName = "🌐️index.html"): Plugin {
  const entry = `/${fileName}`;
  let outDir = "";
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
    },
    closeBundle() {
      const built = resolve(outDir, fileName);
      if (!existsSync(built)) return;
      const html = readFileSync(built, "utf8");
      writeFileSync(resolve(outDir, "index.html"), html);
      writeFileSync(resolve(outDir, "404.html"), html);
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

/** @emoji 🏷️ Vite: brand-aware host chrome — rewrites the `<title>` to the brand's `windowTitle`, serves/copies the brand mark at `/favicon.svg` (ICO only when the brand provides one), and writes the static-deploy markers above; no brand ⇒ canonical semio favicons (still with `.nojekyll`). */
export function semioBrandHtmlVitePlugins(repoRoot: string, brand: ShellBrandHostChrome | undefined): Plugin[] {
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

/** @emoji 🧬️ Full-document spec for a semio host `🌐️index.html`: title, entry module, mount point, and
 * optional CSP + pre-mount loading copy — everything an app needs beyond the shared boot scripts so it
 * stops hand-authoring its own splash screen and `<style>` blocks. */
export type SemioHostHtmlSpec = {
  readonly title: string;
  readonly entry: string;
  readonly rootId?: string;
  readonly bodyClass?: string;
  readonly csp?: string;
  readonly loading?: { readonly title: string };
  /** 🌐️ Custom domain this app's static build deploys to (e.g. GitHub Pages) — written verbatim into a
   * `🌐️CNAME` file at the build root, alongside the always-written `.nojekyll` marker. */
  readonly cnameHost?: string;
};

/** @emoji 🪧️ Pre-mount placeholder markup shown inside `#{rootId}` until the entry module mounts and
 * replaces it — inline-styled so it renders before any external stylesheet loads. */
function semioHostLoadingHtml(loading: SemioHostHtmlSpec["loading"]): string {
  if (!loading) {
    return "";
  }
  return `<div style="display:flex;align-items:center;justify-content:center;height:100%;font:14px system-ui,sans-serif">${loading.title}</div>`;
}

/** @emoji 📄️ Generates a complete semio host `🌐️index.html` document: doctype/head (title, favicon,
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
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    ${cspTag}<title>${spec.title}</title>
    ${SEMIO_FAVICON_HEAD_HTML}
    ${headTags}
  </head>
  <body${spec.bodyClass ? ` class="${spec.bodyClass}"` : ""}>
    <div id="${rootId}">${semioHostLoadingHtml(spec.loading)}</div>
    <script>${PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT}</script>
    <script type="module" src="${spec.entry}"></script>
  </body>
</html>
`;
}

/** @emoji 🎬️ Vite: renders {@link semioHostHtmlString} as the app's `🌐️index.html` on every request/build
 * (full-document replace, `order: "pre"` so later plugins such as `@vitejs/plugin-react`'s HMR preamble
 * still layer on top), bundles semio favicon serving ({@link semioFaviconVitePlugin}), and writes the
 * static-deploy markers ({@link staticDeployMarkerVitePlugins} — `.nojekyll` always, `🌐️CNAME` when
 * `spec.cnameHost` is set) — one call wires an app's whole boot + deploy surface instead of a
 * hand-authored `🌐️index.html` plus a separate build-output step. */
export function semioHostHtmlVitePlugin(repoRoot: string, spec: SemioHostHtmlSpec): Plugin[] {
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
 * / {@link PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT} — this file has no `🔣️tokens.json` import, so these are the
 * canonical values already baked into every other boot surface here, not new ones. */
const SEMIO_STATUS_SURFACE_COLORS = { lightBg: "#f7f3e3", lightFg: "#001117", darkBg: "#001117", darkFg: "#f7f3e3" } as const;

const SEMIO_STATUS_SURFACE_GLYPH: Record<"empty" | "error" | "loading", string> = { empty: "◌️", error: "⚠️", loading: "…" };

function semioStatusSurfaceInlineStyle(): string {
  const c = SEMIO_STATUS_SURFACE_COLORS;
  return `html{color-scheme:light dark}html,body{height:100%;margin:0}body{background-color:${c.lightBg};color:${c.lightFg};display:flex;align-items:center;justify-content:center;font-family:system-ui,sans-serif}@media (prefers-color-scheme: dark){body{background-color:${c.darkBg};color:${c.darkFg}}}`;
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

/** @emoji 🗂️ Canonical repo-relative root of `@semio-tech/assets`, the only tree served at `/asset/*`. */
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

function uiAssetsVitePluginsForRoot(assetsRoot: string): Plugin[] {
  let outDir = resolve(process.cwd(), "dist");
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
      },
      closeBundle() {
        if (!existsSync(assetsRoot)) {
          return;
        }
        const dest = resolve(outDir, "asset");
        mkdirSync(outDir, { recursive: true });
        cpSync(assetsRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🌐️ Vite: serve and copy `@semio-tech/assets` at `/asset/*` for palette fonts and cursors. */
export function semioAssetsVitePlugin(repoRoot: string): Plugin[] {
  return uiAssetsVitePluginsForRoot(resolveSemioAssetRoot(repoRoot));
}

/** @emoji 🌐️ @deprecated Use {@link semioAssetsVitePlugin} — caller-supplied roots caused silent font 404s. */
export function uiAssetsVitePlugin(assetsRoot: string): Plugin[] {
  const fontDir = resolve(assetsRoot, "🔤️fonts");
  if (!existsSync(assetsRoot) || !existsSync(fontDir)) {
    throw new Error(`uiAssetsVitePlugin: invalid asset root ${assetsRoot} (missing 🔤️fonts); use semioAssetsVitePlugin(repoRoot)`);
  }
  return uiAssetsVitePluginsForRoot(assetsRoot);
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
export function animatePresentRendererVitestStripPlugin(animatePresentIndexPath: string): Plugin {
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

/** @deprecated Use {@link animatePresentRendererVitestStripPlugin}. */
export const presentationRendererVitestStripPlugin = animatePresentRendererVitestStripPlugin;

export type PlaygroundPlayViteOptions = {
  readonly playDir: string;
  readonly repoRoot: string;
  /** @emoji 🎯️ When set, `import.meta.env.PLAYGROUND_APP_KIND` gates browser boot in that play's `index.ts`. */
  readonly playEntryKind?: string;
  readonly extraAliases?: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }>;
  readonly extraPlugins?: readonly Plugin[];
  readonly watchIgnored?: readonly string[];
  readonly build?: UserConfig["build"];
  readonly server?: UserConfig["server"];
  readonly optimizeDeps?: UserConfig["optimizeDeps"];
  readonly resolveDedupe?: readonly string[];
};

/** @emoji 🎬️ R3F packages that must resolve once with {@link sceneHostPort} and drei controls. */
export const PLAYGROUND_SCENE_HOST_DEDUPE = ["@react-three/fiber", "@react-three/drei"] as const;

/** @emoji 🎬️ Vite aliases that pin R3F to a single node_modules entry (avoids duplicate Canvas stores). */
export function playgroundSceneHostResolveAliases(repoRoot: string): ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> {
  return [
    { find: /^@react-three\/fiber$/, replacement: resolve(repoRoot, "node_modules/@react-three/fiber/dist/react-three-fiber.esm.js") },
    { find: /^@react-three\/drei$/, replacement: resolve(repoRoot, "node_modules/@react-three/drei/index.js") },
  ];
}

//#region 🔖️MapTileCache
/** @emoji 🗺️ Compliant User-Agent for OSM / MapLibre demotiles in map play. */
export const GIS_MAP_TILE_USER_AGENT = "ComposeGisMapPlay/0.1 (+https://github.com/usalu/semio; dev playground)";

/** @emoji 🗺️ Default dev prefetch bounds (Switzerland) for GIS map play. */
export const GIS_MAP_DEFAULT_PREFETCH_BOUNDS = {
  west: 5.95,
  south: 45.82,
  east: 10.52,
  north: 47.81,
} as const;

export type GisMapPrefetchBounds = {
  readonly west: number;
  readonly south: number;
  readonly east: number;
  readonly north: number;
};

export const GIS_MAP_OSM_TILE_MAX_Z = 19;
/** @emoji 🗺️ OpenFreeMap / OpenMapTiles planet MVT (OSM); matches raster detail up to z14. */
export const GIS_MAP_VECTOR_TILE_MAX_Z = 14;
export const GIS_MAP_OPENFREEMAP_TILEJSON = "https://tiles.openfreemap.org/planet";
/** @emoji 🗺️ Highest zoom prefetched for offline map play (matches `GIS_MAP_LOD_TILE_Z` building band). */
export const GIS_MAP_PREFETCH_RASTER_Z_MAX = 13;

/** @emoji 🗺️ `fetch` loads missing tiles at runtime; `bundle` serves only cached tiles and copies them into `dist` on build. */
export type GisMapTileServeMode = "fetch" | "bundle";

export const GIS_MAP_TILE_SERVE_MODE_ENV = "GIS_MAP_TILE_SERVE_MODE";

export function resolveGisMapTileServeMode(value?: string): GisMapTileServeMode {
  return value === "bundle" ? "bundle" : "fetch";
}

export function mapTileCacheRoots(repoRoot: string): { readonly osm: string; readonly vt: string } {
  return {
    osm: resolve(repoRoot, ".repo-cache", "osm-tiles"),
    vt: resolve(repoRoot, ".repo-cache", "openfreemap-vt"),
  };
}

/** @emoji 🧭️ Web Mercator tile index for a lon/lat at zoom `z`. */
export function lonLatToTileXY(lon: number, lat: number, z: number): { x: number; y: number } {
  const n = 2 ** z;
  const x = Math.floor(((lon + 180) / 360) * n);
  const latRad = (lat * Math.PI) / 180;
  const y = Math.floor(((1 - Math.log(Math.tan(latRad) + 1 / Math.cos(latRad)) / Math.PI) / 2) * n);
  return { x: Math.max(0, Math.min(n - 1, x)), y: Math.max(0, Math.min(n - 1, y)) };
}

/** @emoji 📐️ Inclusive OSM tile index range covering `bounds` at zoom `z`. */
export function tileRangeForBounds(bounds: GisMapPrefetchBounds, z: number): { x0: number; x1: number; y0: number; y1: number } {
  const sw = lonLatToTileXY(bounds.west, bounds.south, z);
  const ne = lonLatToTileXY(bounds.east, bounds.north, z);
  return {
    x0: Math.min(sw.x, ne.x),
    x1: Math.max(sw.x, ne.x),
    y0: Math.min(sw.y, ne.y),
    y1: Math.max(sw.y, ne.y),
  };
}

export type GisMapTileCoord = { readonly z: number; readonly x: number; readonly y: number };

/** @emoji 📋️ Lists every tile in `bounds` for zoom levels `zMin`…`zMax` (inclusive). */
export function listMapTilesForBounds(bounds: GisMapPrefetchBounds, zMin: number, zMax: number): GisMapTileCoord[] {
  const lo = Math.max(0, Math.min(zMin, zMax));
  const hi = Math.max(lo, zMax);
  const out: GisMapTileCoord[] = [];
  for (let z = lo; z <= hi; z++) {
    const { x0, x1, y0, y1 } = tileRangeForBounds(bounds, z);
    for (let x = x0; x <= x1; x++) {
      for (let y = y0; y <= y1; y++) {
        out.push({ z, x, y });
      }
    }
  }
  return out;
}

export type PrefetchMapTilesResult = {
  readonly downloaded: number;
  readonly skipped: number;
  readonly failed: number;
};

export type PrefetchMapTilesOptions = {
  readonly repoRoot: string;
  readonly bounds?: GisMapPrefetchBounds;
  readonly raster?: boolean;
  readonly vector?: boolean;
  readonly zMinRaster?: number;
  readonly zMaxRaster?: number;
  readonly zMinVector?: number;
  readonly zMaxVector?: number;
  readonly concurrency?: number;
  readonly skipExisting?: boolean;
  readonly delayMs?: number;
  readonly log?: (line: string) => void;
};

async function fetchOsmTileToCache(cacheRoot: string, z: number, x: number, y: number): Promise<boolean> {
  const rel = `${z}/${x}/${y}.png`;
  const filePath = resolve(cacheRoot, rel);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return false;
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const upstream = await fetch(`https://tile.openstreetmap.org/${z}/${x}/${y}.png`, {
    headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT },
  });
  if (!upstream.ok) {
    return false;
  }
  await writeFile(filePath, Buffer.from(await upstream.arrayBuffer()));
  return true;
}

let openFreeMapTileTemplate: string | null = null;
let openFreeMapTileTemplateAt = 0;
const OPENFREEMAP_TILE_TEMPLATE_TTL_MS = 7 * 24 * 60 * 60 * 1000;

async function resolveOpenFreeMapTileTemplate(): Promise<string> {
  const now = Date.now();
  if (openFreeMapTileTemplate && now - openFreeMapTileTemplateAt < OPENFREEMAP_TILE_TEMPLATE_TTL_MS) {
    return openFreeMapTileTemplate;
  }
  const res = await fetch(GIS_MAP_OPENFREEMAP_TILEJSON, { headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT } });
  if (!res.ok) {
    throw new Error(`OpenFreeMap TileJSON failed: ${res.status}`);
  }
  const json = (await res.json()) as { tiles?: string[] };
  const template = json.tiles?.[0];
  if (typeof template !== "string" || !template.includes("{z}")) {
    throw new Error("OpenFreeMap TileJSON missing tiles URL template");
  }
  openFreeMapTileTemplate = template;
  openFreeMapTileTemplateAt = now;
  return template;
}

async function fetchVtTileToCache(cacheRoot: string, z: number, x: number, y: number): Promise<boolean> {
  const rel = `${z}/${x}/${y}.pbf`;
  const filePath = resolve(cacheRoot, rel);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return false;
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const template = await resolveOpenFreeMapTileTemplate();
  const url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
  const upstream = await fetch(url, { headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT } });
  if (!upstream.ok) {
    return false;
  }
  const buf = Buffer.from(await upstream.arrayBuffer());
  if (buf.length === 0) {
    return false;
  }
  await writeFile(filePath, buf);
  return true;
}

/** @emoji ⬇️ Prefetch OSM PNG and MapLibre MVT tiles into `.repo-cache` for offline map play. */
export async function prefetchMapTiles(options: PrefetchMapTilesOptions): Promise<PrefetchMapTilesResult> {
  const {
    repoRoot,
    bounds = GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
    raster = true,
    vector = true,
    zMinRaster = 0,
    zMaxRaster = GIS_MAP_PREFETCH_RASTER_Z_MAX,
    zMinVector = 0,
    zMaxVector = GIS_MAP_VECTOR_TILE_MAX_Z,
    concurrency = 4,
    skipExisting = true,
    delayMs = 120,
    log = (line) => console.log(line),
  } = options;
  const { osm, vt } = mapTileCacheRoots(repoRoot);
  const jobs: { kind: "osm" | "vt"; z: number; x: number; y: number }[] = [];
  if (raster) {
    for (const { z, x, y } of listMapTilesForBounds(bounds, zMinRaster, Math.min(zMaxRaster, GIS_MAP_OSM_TILE_MAX_Z))) {
      jobs.push({ kind: "osm", z, x, y });
    }
  }
  if (vector) {
    for (const { z, x, y } of listMapTilesForBounds(bounds, zMinVector, Math.min(zMaxVector, GIS_MAP_VECTOR_TILE_MAX_Z))) {
      jobs.push({ kind: "vt", z, x, y });
    }
  }
  const zoomLabel = `(raster z${zMinRaster}-${zMaxRaster}, vector z${zMinVector}-${zMaxVector})`;
  let skipped = 0;
  const pending = skipExisting
    ? jobs.filter((job) => {
        const cacheRoot = job.kind === "osm" ? osm : vt;
        const ext = job.kind === "osm" ? "png" : "pbf";
        const filePath = resolve(cacheRoot, `${job.z}/${job.x}/${job.y}.${ext}`);
        if (existsSync(filePath)) {
          skipped++;
          return false;
        }
        return true;
      })
    : jobs;
  log(`[gis/2d/play] prefetch ${jobs.length} tiles ${zoomLabel}` + (skipExisting ? ` (${skipped} cached, ${pending.length} to fetch)` : ""));
  if (pending.length === 0) {
    log(`[gis/2d/play] prefetch done: downloaded=0 skipped=${skipped} failed=0`);
    return { downloaded: 0, skipped, failed: 0 };
  }
  let downloaded = 0;
  let failed = 0;
  const sleep = (ms: number) => new Promise<void>((r) => setTimeout(r, ms));
  for (let i = 0; i < pending.length; i += concurrency) {
    const batch = pending.slice(i, i + concurrency);
    await Promise.all(
      batch.map(async (job) => {
        const cacheRoot = job.kind === "osm" ? osm : vt;
        const ok = job.kind === "osm" ? await fetchOsmTileToCache(cacheRoot, job.z, job.x, job.y) : await fetchVtTileToCache(cacheRoot, job.z, job.x, job.y);
        if (ok) {
          downloaded++;
        } else {
          failed++;
        }
      }),
    );
    if (delayMs > 0 && i + concurrency < pending.length) {
      await sleep(delayMs);
    }
  }
  log(`[gis/2d/play] prefetch done: downloaded=${downloaded} skipped=${skipped} failed=${failed}`);
  return { downloaded, skipped, failed };
}
//#endregion 🔖️MapTileCache

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

const tileProxyTemplateCache = new Map<string, { readonly template: string; readonly at: number }>();
const TILE_PROXY_TEMPLATE_TTL_MS = 7 * 24 * 60 * 60 * 1000;

/** @emoji 🧭️ Resolves a `tile-proxy` spec's `upstream` to a concrete `{z}/{x}/{y}` URL template: used
 * directly when it already contains `{z}`, otherwise treated as a TileJSON endpoint and resolved
 * (cached, 7-day TTL) — generalizes the previous OpenFreeMap-only MVT template resolution so any
 * TileJSON-backed upstream (not just OpenFreeMap) works the same way. */
async function resolveTileProxyUrlTemplate(upstream: string): Promise<string> {
  if (upstream.includes("{z}")) {
    return upstream;
  }
  const now = Date.now();
  const cached = tileProxyTemplateCache.get(upstream);
  if (cached && now - cached.at < TILE_PROXY_TEMPLATE_TTL_MS) {
    return cached.template;
  }
  const res = await fetch(upstream, { headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT } });
  if (!res.ok) {
    throw new Error(`tile proxy upstream TileJSON failed: ${res.status}`);
  }
  const json = (await res.json()) as { tiles?: string[] };
  const template = json.tiles?.[0];
  if (typeof template !== "string" || !template.includes("{z}")) {
    throw new Error("tile proxy TileJSON missing tiles URL template");
  }
  tileProxyTemplateCache.set(upstream, { template, at: now });
  return template;
}

async function fetchTileProxyTileToCache(cacheRoot: string, upstream: string, z: number, x: number, y: number): Promise<{ readonly ok: boolean; readonly ext: string }> {
  const template = await resolveTileProxyUrlTemplate(upstream);
  const ext = tileProxyExtFromTemplate(template);
  const filePath = resolve(cacheRoot, `${z}/${x}/${y}.${ext}`);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return { ok: false, ext };
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
  const upstreamRes = await fetch(url, { headers: { "User-Agent": GIS_MAP_TILE_USER_AGENT } });
  if (!upstreamRes.ok) {
    return { ok: false, ext };
  }
  const buf = Buffer.from(await upstreamRes.arrayBuffer());
  if (buf.length === 0) {
    return { ok: false, ext };
  }
  await writeFile(filePath, buf);
  return { ok: true, ext };
}

/** @emoji 🌐️ Connect middleware serving `{route}/{z}/{x}/{y}.{ext}` tiles from `cacheRoot`, fetching
 * (and caching) from `upstream` on a miss — generalizes the previous OSM/OpenFreeMap/Terrarium
 * middlewares into one route-driven implementation. */
function createTileProxyMiddleware(route: string, cacheRoot: string, upstream: string, mode: GisMapTileServeMode): Connect.NextHandleFunction {
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
      res.setHeader("Content-Type", contentTypeForTileExt(ext));
      createReadStream(filePath).pipe(res);
      return;
    }
    if (mode === "bundle") {
      res.statusCode = 404;
      res.end();
      return;
    }
    try {
      const result = await fetchTileProxyTileToCache(cacheRoot, upstream, z, x, y);
      if (!result.ok) {
        res.statusCode = 404;
        res.end();
        return;
      }
      res.setHeader("Content-Type", contentTypeForTileExt(result.ext));
      createReadStream(filePath).pipe(res);
    } catch {
      res.statusCode = 502;
      res.end();
    }
  };
}

/** @emoji 🌐️ Generic dev/preview/build Vite plugin pair for one `tile-proxy` asset spec — replaces the
 * previous `gisMapTilesVitePlugins`/`terrainTilesVitePlugins`/`osmTileProxyVitePlugin`/
 * `mapLibreVectorTileProxyVitePlugin` quartet with a single spec-driven implementation. */
export function tileProxyVitePlugin(repoRoot: string, spec: Extract<PlaygroundAssetSpec, { kind: "tile-proxy" }>, mode: GisMapTileServeMode = "fetch"): Plugin[] {
  const cacheRoot = resolve(repoRoot, ".repo-cache", spec.cache);
  const serveTiles = createTileProxyMiddleware(spec.route, cacheRoot, spec.upstream, mode);
  let outDir = resolve(process.cwd(), "dist");
  const plugins: Plugin[] = [
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
      },
      closeBundle() {
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

/** @emoji 🌐️ Standalone HTTP server for every declared playground asset kind (tile-proxy, mesh-collection,
 * static-dir) — wgpu Trunk proxies and native-bin `SEMIO_ASSET_BASE_URL` hit this instead of Vite. */
export function startAssetServer(repoRoot: string, port: number, specs: readonly PlaygroundAssetSpec[], mode: GisMapTileServeMode = "fetch", host = "127.0.0.1"): Server {
  const seen = new Set<string>();
  const middlewares: Connect.NextHandleFunction[] = [];
  for (const spec of specs) {
    const key = `${spec.kind}:${spec.route}`;
    if (seen.has(key)) continue;
    seen.add(key);
    if (spec.kind === "tile-proxy") {
      middlewares.push(createTileProxyMiddleware(spec.route, resolve(repoRoot, ".repo-cache", spec.cache), spec.upstream, mode));
    } else if (spec.kind === "mesh-collection") {
      middlewares.push(createMeshCollectionMiddleware(repoRoot, spec));
    } else {
      middlewares.push(createStaticDirMiddleware(repoRoot, spec));
    }
  }
  const server = createServer((req, res) => {
    const run = (i: number): void => {
      if (i >= middlewares.length) {
        res.statusCode = 404;
        res.end();
        return;
      }
      middlewares[i]!(req, res, () => run(i + 1));
    };
    run(0);
  });
  server.listen(port, host);
  return server;
}
//#endregion 🔖️TileProxyAssetPlugin

//#region 🔖️PlaygroundAssetVitePlugins
/** @emoji 🚦️ Dispatches every declared `[[package.metadata.semio.assets]]` spec to its generic Vite
 * plugin factory — the single driver a dev `vite.config` calls with a playground's resolved `assets`
 * metadata instead of hand-picking per-app plugin factories. */
export function playgroundAssetVitePlugins(repoRoot: string, specs: readonly PlaygroundAssetSpec[], mode: GisMapTileServeMode = "fetch"): Plugin[] {
  const seen = new Set<string>();
  const plugins: Plugin[] = [];
  for (const spec of specs) {
    const key = `${spec.kind}:${spec.route}`;
    if (seen.has(key)) {
      continue;
    }
    seen.add(key);
    if (spec.kind === "tile-proxy") {
      plugins.push(...tileProxyVitePlugin(repoRoot, spec, mode));
    } else if (spec.kind === "static-dir") {
      plugins.push(...staticDirVitePlugin(repoRoot, spec));
    } else {
      plugins.push(...meshCollectionVitePlugin(repoRoot, spec));
    }
  }
  return plugins;
}
//#endregion 🔖️PlaygroundAssetVitePlugins

/** @emoji 🦀️ Vite `optimizeDeps.exclude` entries for wasm-bindgen flow modules (must not be prebundled). */
export const FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE = [
  "@semio-tech/flow-module-core",
  "@semio-tech/flow-module-math",
  "@semio-tech/flow-module-text",
  "@semio-tech/flow-module-logic",
  "@semio-tech/flow-module-dictionary",
  "@semio-tech/flow-module-list",
  "@semio-tech/flow-module-brep",
  "@semio-tech/flow-module-draw",
  "@semio-tech/flow-module-bim",
] as const;

/** @emoji 🧭️ Workspace Vite resolve preset: dedupe, fs.allow, optimizeDeps.exclude, scene-host aliases. */
export function createWorkspaceViteResolveConfig(repoRoot: string, extraAliases: ReadonlyArray<{ readonly find: string | RegExp; readonly replacement: string }> = []): Pick<UserConfig, "resolve" | "server" | "optimizeDeps"> {
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
function contentTypeForStaticDirAsset(filePath: string): string | undefined {
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

/** @emoji 🗂️ Connect middleware: serve one `static-dir` spec's files at `{route}/…`. */
function createStaticDirMiddleware(repoRoot: string, spec: Extract<PlaygroundAssetSpec, { kind: "static-dir" }>): Connect.NextHandleFunction {
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
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot) || !existsSync(filePath) || !statSync(filePath).isFile()) {
      next();
      return;
    }
    const contentType = contentTypeForStaticDirAsset(filePath);
    if (contentType) {
      res.setHeader("Content-Type", contentType);
    }
    createReadStream(filePath).pipe(res);
  };
}

/** @emoji 🖼️ Generic dev/build Vite plugin pair for one `static-dir` asset spec: serves and copies
 * `spec.root` at `spec.route` — replaces the previous `cadFixtureVitePlugin`/`infiniteFixtureVitePlugin`
 * pair (byte-identical serving logic, now route/root-driven instead of hardcoded per fixture tree). */
export function staticDirVitePlugin(repoRoot: string, spec: Extract<PlaygroundAssetSpec, { kind: "static-dir" }>): Plugin[] {
  const serveFixture = createStaticDirMiddleware(repoRoot, spec);
  const fixtureRoot = resolve(repoRoot, spec.root);
  const destName = spec.route.replace(/^\//, "");
  let outDir = resolve(process.cwd(), "dist");
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
      },
      closeBundle() {
        if (!existsSync(fixtureRoot)) {
          return;
        }
        const dest = resolve(outDir, destName);
        mkdirSync(outDir, { recursive: true });
        cpSync(fixtureRoot, dest, { recursive: true });
      },
    },
  ];
}

/** @emoji 🌐️ Reference-plane fixture trees every `*-play` static bundle serves unconditionally
 * (not app-specific — `cad/fixture` and `infinite/fixture` back shared world reference planes used
 * across playgrounds), kept as a literal baseline rather than per-plugin metadata. */
export const PLAYGROUND_PLAY_STATIC_ASSETS: readonly Extract<PlaygroundAssetSpec, { kind: "static-dir" }>[] = [
  { kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🧫️fixtures" },
  { kind: "static-dir", route: "/infinite-fixture", root: "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures" },
];
//#endregion 🔖️StaticDirAssetPlugin

export function findWorkspacePackages(repoRoot: string): string[] {
  const packages: string[] = [];
  const scan = (dir: string) => {
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      return;
    }
    for (const entry of entries) {
      if (entry === "node_modules" || entry === "dist" || entry === "target" || entry === "storybook-static" || entry.startsWith(".")) continue;
      const full = resolve(dir, entry);
      try {
        const stat = statSync(full);
        if (stat.isDirectory()) {
          scan(full);
        } else if (entry === "package.json" && full !== resolve(repoRoot, "package.json")) {
          const pkg = JSON.parse(readFileSync(full, "utf8"));
          if (pkg.name && typeof pkg.name === "string" && pkg.name.startsWith("@semio-tech/")) {
            packages.push(pkg.name);
          }
        }
      } catch {
        /* ignore statSync or readFileSync errors (e.g. broken symlinks or unreadable files) */
      }
    }
  };
  scan(repoRoot);
  return packages;
}

/** @emoji 📄️ Stubs all compose sketchpad MDX modules for s/os dev graphs (including worker bundles). */
function playgroundSketchpadMdxGlobalStubPlugin(): Plugin {
  const stub = "export default function SketchpadMdxStub() { return null; }";
  const isSketchpadMdx = (id: string): boolean => {
    const cleanId = id.split("?", 1)[0] ?? id;
    return cleanId.endsWith(".mdx") && cleanId.includes("/compose/client/lib/sketchpad/");
  };
  return {
    name: "playground-sketchpad-mdx-global-stub",
    enforce: "pre",
    resolveId(id) {
      if (!isSketchpadMdx(id)) return;
      return `\0playground-sketchpad-mdx:${id.split("?", 1)[0]}`;
    },
    load(id) {
      if (id.startsWith("\0playground-sketchpad-mdx:") || isSketchpadMdx(id)) return stub;
    },
  };
}

/** @emoji 🛝️ `defineConfig` for `@puzzle/*-play` Vite entries with consistent renderer and core aliases. */
export function createPlaygroundPlayViteConfig(options: PlaygroundPlayViteOptions) {
  const { playDir, repoRoot, playEntryKind, extraAliases = [], extraPlugins = [], watchIgnored, build, server, optimizeDeps, resolveDedupe } = options;
  const osHubAliases =
    playEntryKind === "s"
      ? [
          {
            find: "@semio-tech/graph-dsl-core",
            replacement: resolve(repoRoot, "mathematical/graph/dsl/core/js/index.ts"),
          },
        ]
      : [];
  const workspaceResolve = createWorkspaceViteResolveConfig(repoRoot, [...extraAliases, ...osHubAliases]);
  const workerStubPlugins = [playgroundPlaywrightDevStubPlugin(), playgroundVitestDevStubPlugin()];
  return defineConfig({
    root: playDir,
    base: "./",
    publicDir: resolve(playDir, "public"),
    assetsInclude: ["**/*.wasm"],
    worker: {
      format: "es",
      plugins: () => workerStubPlugins,
    },
    define: {
      ...playgroundPlayViteDefine(playEntryKind ? { "import.meta.env.PLAYGROUND_APP_KIND": JSON.stringify(playEntryKind) } : {}),
    },
    plugins: [
      playgroundPlayBootHtmlPlugin(),
      playgroundFlowWasmDevStubPlugin(repoRoot),
      playgroundComposeSketchpadStubPlugin(repoRoot),
      ...semioAssetsVitePlugin(repoRoot),
      ...semioFaviconVitePlugin(repoRoot),
      ...playgroundAssetVitePlugins(repoRoot, PLAYGROUND_PLAY_STATIC_ASSETS),
      tailwindcss(),
      react(),
      playgroundPlaywrightDevStubPlugin(),
      playgroundVitestDevStubPlugin(),
      playgroundIframeEmbedHeadersPlugin(),
      playgroundStaleOptimizeDepPlugin(),
      ...extraPlugins,
    ],
    build: playgroundStaticSiteBuildOptions(build),
    server: {
      ...workspaceResolve.server,
      ...(watchIgnored ? { watch: { ignored: watchIgnored } } : {}),
      ...server,
    },
    resolve: {
      ...workspaceResolve.resolve,
      dedupe: [...(workspaceResolve.resolve?.dedupe ?? []), ...(resolveDedupe ?? [])],
    },
    optimizeDeps: {
      ...workspaceResolve.optimizeDeps,
      ...optimizeDeps,
      exclude: [...(workspaceResolve.optimizeDeps?.exclude ?? []), ...(optimizeDeps?.exclude ?? [])],
    },
  });
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const repoRoot = resolve(fileURLToPath(new URL(".", import.meta.url)), "../../../../../..");

  describe("playgroundFlowWasmDevStubPlugin", () => {
    const importer = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx");
    const plugin = playgroundFlowWasmDevStubPlugin(repoRoot);
    const resolveId = plugin.resolveId as (id: string, importer: string) => string | undefined;

    it("resolves bare @semio-tech/flow-core to the wasm-pack entry, not the stub", () => {
      const resolved = resolveId("@semio-tech/flow-core", importer);
      expect(resolved).toBeDefined();
      expect(resolved).not.toContain("playground-wasm-stub");
      expect(resolved).toMatch(/flow_core\.js$/);
      expect(existsSync(resolved!)).toBe(true);
    });

    it("falls back to stub for an unbuilt @semio-tech wasm package subpath", () => {
      const id = "@semio-tech/__playground_wasm_stub_test_missing__/pkg/entry.js";
      const resolved = resolveId(id, importer);
      expect(resolved).toBe(`${PLAYGROUND_WASM_STUB_PREFIX}${playgroundWasmStubKey(id)}`);
    });
  });

  describe("isPlaygroundOptimizedDepUrl", () => {
    it("matches Vite prebundle chunk URLs", () => {
      expect(isPlaygroundOptimizedDepUrl("/node_modules/.vite/deps/chunk-ABC.js?v=1")).toBe(true);
      expect(isPlaygroundOptimizedDepUrl("/index.ts")).toBe(false);
    });
  });

  describe("playgroundSceneHostResolveAliases", () => {
    it("pins fiber and drei to node_modules entries", () => {
      const aliases = playgroundSceneHostResolveAliases(repoRoot);
      expect(aliases.some((row) => String(row.find).includes("fiber") && row.replacement.endsWith("react-three-fiber.esm.js"))).toBe(true);
      expect(aliases.some((row) => String(row.find).includes("drei") && row.replacement.endsWith("@react-three/drei/index.js"))).toBe(true);
    });
  });

  describe("resolveGisMapTileServeMode", () => {
    it("defaults to fetch", () => {
      expect(resolveGisMapTileServeMode(undefined)).toBe("fetch");
      expect(resolveGisMapTileServeMode("")).toBe("fetch");
      expect(resolveGisMapTileServeMode("online")).toBe("fetch");
    });

    it("selects bundle only for bundle", () => {
      expect(resolveGisMapTileServeMode("bundle")).toBe("bundle");
    });
  });

  describe("tileProxyVitePlugin", () => {
    const osmSpec: Extract<PlaygroundAssetSpec, { kind: "tile-proxy" }> = {
      kind: "tile-proxy",
      route: "/osm",
      upstream: "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
      cache: "osm-tiles",
    };

    it("adds a build copy plugin only for bundle mode", () => {
      const fetchPlugins = tileProxyVitePlugin(repoRoot, osmSpec, "fetch");
      const bundlePlugins = tileProxyVitePlugin(repoRoot, osmSpec, "bundle");
      expect(fetchPlugins.some((plugin) => plugin.name === "tile-proxy-build/osm")).toBe(false);
      expect(bundlePlugins.some((plugin) => plugin.name === "tile-proxy-build/osm")).toBe(true);
    });
  });

  describe("playgroundAssetVitePlugins", () => {
    it("dispatches each asset kind to its generic factory and dedupes by kind+route", () => {
      const specs: PlaygroundAssetSpec[] = [
        { kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🧫️fixtures" },
        { kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🧫️fixtures" },
      ];
      const plugins = playgroundAssetVitePlugins(repoRoot, specs);
      expect(plugins.filter((plugin) => plugin.name === "static-dir-serve/cad-fixture")).toHaveLength(1);
    });
  });

  describe("contentTypeForStaticDirAsset", () => {
    it("assigns module script mime types for wasm plugin artifacts", () => {
      expect(contentTypeForStaticDirAsset("/plugin-modules/sourcing/sourcing_plugin.js")).toBe("text/javascript");
      expect(contentTypeForStaticDirAsset("/plugin-modules/_vendor/@bytecodealliance/preview2-shim/cli.js")).toBe("text/javascript");
      expect(contentTypeForStaticDirAsset("/plugin-modules/puzzle/🕸️puzzle_plugin.wasm")).toBe("application/wasm");
    });
  });

  describe("listMapTilesForBounds", () => {
    it("covers Switzerland at z0 with a single world tile", () => {
      const tiles = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 0, 0);
      expect(tiles).toEqual([{ z: 0, x: 0, y: 0 }]);
    });

    it("returns more tiles at higher zoom", () => {
      // 🇨️🇭️ Switzerland still fits inside a single OSM tile up to z6 (~5.6°/tile > its ~4.6° span), so
      // the comparison needs a zoom gap wide enough to actually straddle a tile boundary.
      const z2 = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 2, 2).length;
      const z8 = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 8, 8).length;
      expect(z8).toBeGreaterThan(z2);
    });
  });

  describe("prefetchMapTiles", () => {
    it("skips tiles already present in cache without fetching", async () => {
      const { osm } = mapTileCacheRoots(repoRoot);
      const tile = { z: 0, x: 0, y: 0 };
      const filePath = resolve(osm, `${tile.z}/${tile.x}/${tile.y}.png`);
      const hadCache = existsSync(filePath);
      if (!hadCache) {
        mkdirSync(resolve(filePath, ".."), { recursive: true });
        writeFileSync(filePath, Buffer.from([0x89, 0x50, 0x4e, 0x47]));
      }
      const lines: string[] = [];
      const result = await prefetchMapTiles({
        repoRoot,
        bounds: GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
        raster: true,
        vector: false,
        zMinRaster: 0,
        zMaxRaster: 0,
        concurrency: 4,
        delayMs: 0,
        log: (line) => lines.push(line),
      });
      expect(result.skipped).toBeGreaterThan(0);
      expect(result.downloaded).toBe(0);
      expect(lines.some((line) => line.includes("cached"))).toBe(true);
      if (!hadCache) {
        const { unlinkSync } = await import("node:fs");
        unlinkSync(filePath);
      }
    });
  });

  describe("resolveSemioAssetRoot", () => {
    it("resolves the merged asset package with fonts", () => {
      const root = resolveSemioAssetRoot(repoRoot);
      expect(root.endsWith(SEMIO_ASSET_ROOT.split("/").pop()!)).toBe(true);
      expect(existsSync(resolve(root, "🔤️fonts/🔤️anta/🔤️latin.woff2"))).toBe(true);
    });

    it("throws when fonts are missing", () => {
      expect(() => resolveSemioAssetRoot(resolve(repoRoot, ".🦑️repo"))).toThrow(/Missing Semio asset root/);
    });
  });

  describe("playgroundPlayBootHtmlPlugin", () => {
    it("registers index html boot injection", () => {
      expect(playgroundPlayBootHtmlPlugin().name).toBe("playground-play-boot-html");
    });

    it("exposes inline appearance and reveal scripts", () => {
      expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).toContain("prefers-color-scheme");
      expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).toContain("ui.chrome.appearance");
      expect(PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT).toContain("semio-play-styles");
      expect(PLAYGROUND_PLAY_BOOT_INLINE_STYLE).toContain("data-semio-styled");
    });

    it("exposes an inline theme bootstrap script reading the persisted theme snapshot", () => {
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("ui.chrome.theme.snapshot");
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("--color-");
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("dataset.uiTheme");
    });

    it("injects the theme script after the appearance script and before the stylesheet link", () => {
      const tags = playgroundPlayBootHtmlPlugin().transformIndexHtml!.handler!({} as never).tags;
      const kinds = tags.map((tag) => (tag.children === PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT ? "appearance" : tag.children === PLAYGROUND_PLAY_BOOT_THEME_SCRIPT ? "theme" : tag.attrs && "href" in tag.attrs ? "stylesheet" : "other"));
      expect(kinds.indexOf("appearance")).toBeLessThan(kinds.indexOf("theme"));
      expect(kinds.indexOf("theme")).toBeLessThan(kinds.indexOf("stylesheet"));
    });
  });

  describe("rewriteSpaFallbackToEmojiEntry", () => {
    const entry = "/🌐️index.html";
    it("rewrites /index.html to the emoji entry", () => {
      expect(rewriteSpaFallbackToEmojiEntry("/index.html", entry)).toBe(entry);
    });
    it("preserves query and hash on /index.html", () => {
      expect(rewriteSpaFallbackToEmojiEntry("/index.html?x=1#frag", entry)).toBe(`${entry}?x=1#frag`);
    });
    it("leaves asset paths unchanged", () => {
      expect(rewriteSpaFallbackToEmojiEntry("/spaces/space-1", entry)).toBe("/spaces/space-1");
    });
  });

  describe("semioHostHtmlString", () => {
    it("renders title, entry module, root mount, favicon links, and boot scripts", () => {
      const html = semioHostHtmlString({ title: "Semio App", entry: "/js/index.tsx" });
      expect(html).toContain("<title>Semio App</title>");
      expect(html).toContain('<script type="module" src="/js/index.tsx"></script>');
      expect(html).toContain('<div id="root">');
      expect(html).toContain(SEMIO_FAVICON_HEAD_HTML);
      expect(html).toContain(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT);
      expect(html).toContain(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT);
      expect(html).toContain(PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT);
    });

    it("honors rootId, bodyClass, csp, and loading overrides", () => {
      const html = semioHostHtmlString({
        title: "Semio App",
        entry: "/js/index.tsx",
        rootId: "semio-root",
        bodyClass: "semio-app-body",
        csp: "default-src 'self'",
        loading: { title: "Loading…" },
      });
      expect(html).toContain('<div id="semio-root">');
      expect(html).toContain('<body class="semio-app-body">');
      expect(html).toContain('<meta http-equiv="Content-Security-Policy" content="default-src \'self\'" />');
      expect(html).toContain("Loading…");
    });
  });

  describe("semioHostHtmlVitePlugin", () => {
    it("bundles favicon serving and static-deploy-marker plugins alongside the host html plugin", () => {
      const plugins = semioHostHtmlVitePlugin(repoRoot, { title: "Semio App", entry: "/js/index.tsx" });
      expect(plugins.map((plugin) => plugin.name)).toEqual(["semio-favicon-serve", "semio-favicon-build", "static-deploy-markers", "semio-host-html"]);
    });

    it("renders the same document semioHostHtmlString produces", () => {
      const spec = { title: "Semio App", entry: "/js/index.tsx" };
      const plugin = semioHostHtmlVitePlugin(repoRoot, spec).find((p) => p.name === "semio-host-html")!;
      const result = (plugin.transformIndexHtml as { handler: () => string }).handler();
      expect(result).toBe(semioHostHtmlString(spec));
    });
  });

  describe("statusSurfaceHtml", () => {
    it("renders title, description, and status kind with no external CSS dependency", () => {
      const html = statusSurfaceHtml({ kind: "error", title: "Something went wrong", description: "Try again later." });
      expect(html).toContain("Something went wrong");
      expect(html).toContain("Try again later.");
      expect(html).toContain('data-status-kind="error"');
      expect(html).not.toContain("<link");
      expect(html).not.toContain('rel="stylesheet"');
    });

    it("omits the description paragraph when none is given", () => {
      const html = statusSurfaceHtml({ kind: "loading", title: "Loading…" });
      expect(html).toContain('data-status-kind="loading"');
      expect(html).not.toContain("<p style=\"margin:8px 0 0");
    });
  });

  describe("semioFaviconVitePlugin", () => {
    it("points at round dark emblem svg and ico under asset/logo", () => {
      const { svg, ico } = semioFaviconSources(repoRoot);
      expect(svg).toBe(resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️emblem_dark_round.svg"));
      expect(ico).toBe(resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos/🖼️favicon_dark_round_32x32.ico"));
      expect(existsSync(svg)).toBe(true);
      expect(existsSync(ico)).toBe(true);
    });

    it("registers serve and build plugins", () => {
      const plugins = semioFaviconVitePlugin(repoRoot);
      expect(plugins.map((plugin) => plugin.name)).toEqual(["semio-favicon-serve", "semio-favicon-build"]);
    });

    it("injects opaque bleed into round dark favicon svg", () => {
      const { svg } = semioFaviconSources(repoRoot);
      const markup = semioFaviconSvgMarkup(svg);
      expect(markup).toContain('<rect width="350" height="350" fill="#001117"/>');
    });
  });

  describe("meshCollectionVitePlugin", () => {
    const puzzle3dMeshSpec: Extract<PlaygroundAssetSpec, { kind: "mesh-collection" }> = {
      kind: "mesh-collection",
      route: "/mesh",
      roots: ["./🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation", "./♻️mit-bestand/🖼️asset/🏚️abbau-aufbau"],
      placeholder: "./🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧊️placeholder.glb",
      filterFromExamples: true,
    };

    it("points at metabolism and abbau-aufbau kit glbs plus shared placeholder", () => {
      expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.roots[0]!, "🧊️capsule_J.glb"))).toBe(true);
      expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.roots[0]!, "🧊️capsule-with-balcony_slash.glb"))).toBe(true);
      expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.roots[1]!, "🧊️hexagonal-cut-concrete-forest-left.glb"))).toBe(true);
      expect(existsSync(resolve(repoRoot, puzzle3dMeshSpec.placeholder))).toBe(true);
    });

    it("registers serve and build plugins named after the route", () => {
      const plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
      expect(plugins.map((plugin) => plugin.name)).toEqual(["mesh-collection-serve/mesh", "mesh-collection-build/mesh"]);
    });

    it("startAssetServer serves 🧊️base.glb as model/gltf-binary", async () => {
      const probe = createServer();
      await new Promise<void>((resolveListen) => probe.listen(0, "127.0.0.1", () => resolveListen()));
      const address = probe.address();
      if (!address || typeof address === "string") throw new Error("expected TCP address");
      const port = address.port;
      await new Promise<void>((resolveClose, reject) => probe.close((err) => (err ? reject(err) : resolveClose())));
      const server = startAssetServer(repoRoot, port, [puzzle3dMeshSpec]);
      try {
        const response = await fetch(`http://127.0.0.1:${port}/mesh/🧊️base.glb`);
        expect(response.status).toBe(200);
        expect(response.headers.get("content-type")).toBe("model/gltf-binary");
        const bytes = new Uint8Array(await response.arrayBuffer());
        expect(String.fromCharCode(bytes[0]!, bytes[1]!, bytes[2]!, bytes[3]!)).toBe("glTF");
      } finally {
        await new Promise<void>((resolveClose, reject) => server.close((err) => (err ? reject(err) : resolveClose())));
      }
    });
  });

  describe("createWorkspaceViteResolveConfig", () => {
    // ⏱️ `findWorkspacePackages` walks the whole repo tree — past the 5s default on this monorepo's size.
    it(
      "pins scene hosts and excludes workspace packages from optimizeDeps",
      () => {
        const config = createWorkspaceViteResolveConfig(repoRoot);
        expect(config.resolve?.dedupe).toContain("react");
        expect(config.resolve?.dedupe).toContain("three");
        expect(config.server?.fs?.allow).toContain(repoRoot);
        expect(config.optimizeDeps?.exclude).toContain("@semio-tech/flow-module-core");
      },
      20000,
    );
  });

  describe("findWorkspacePackages", () => {
    it(
      "discovers workspace packages while skipping hidden dot directories",
      () => {
        const pkgs = findWorkspacePackages(repoRoot);
        expect(pkgs).toContain("@semio-tech/ui-react");
        expect(pkgs.every((p) => p.startsWith("@semio-tech/"))).toBe(true);
      },
      20000,
    );
  });
}
//#endregion 🔖️ViteElementsAssets

