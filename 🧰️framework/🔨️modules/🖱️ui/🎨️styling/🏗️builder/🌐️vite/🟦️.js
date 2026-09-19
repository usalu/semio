"use strict";
// #region 🧲️Header
/** 🌐️ Vite plugins serving the asset-owned `/🖼️assets/*` namespace. */
// #endregion 🧲️Header
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.PLAYGROUND_PLAY_STATIC_ASSETS = exports.FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE = exports.GIS_MAP_TILE_SERVE_MODE_ENV = exports.GIS_MAP_PREFETCH_RASTER_Z_MAX = exports.GIS_MAP_OPENFREEMAP_TILEJSON = exports.GIS_MAP_VECTOR_TILE_MAX_Z = exports.GIS_MAP_OSM_TILE_MAX_Z = exports.GIS_MAP_DEFAULT_PREFETCH_BOUNDS = exports.GIS_MAP_TILE_USER_AGENT = exports.PLAYGROUND_SCENE_HOST_CJS_INCLUDE = exports.PLAYGROUND_SCENE_HOST_DEDUPE = exports.presentationRendererVitestStripPlugin = exports.SEMIO_ASSET_ROOT = exports.STATIC_SITE_HTML_ALIASES = exports.SEMIO_FAVICON_HEAD_HTML = exports.PLAYGROUND_PLAY_BOOT_THEME_SCRIPT = exports.PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT = exports.PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT = exports.PLAYGROUND_PLAY_BOOT_INLINE_STYLE = exports.PLAYGROUND_WASM_STUB_PREFIX = exports.playgroundTestPortString = exports.playgroundTestPort = exports.playgroundPortEnv = exports.playgroundDevPortString = exports.playgroundDevPort = exports.allPlaygroundReservedPorts = exports.PLAYGROUND_PORTS = exports.playgroundIframeEmbedHeadersPlugin = void 0;
exports.playgroundStaticSiteBuildOptions = playgroundStaticSiteBuildOptions;
exports.semioViteProductionBuild = semioViteProductionBuild;
exports.playgroundOptimizedDepUrlPrefix = playgroundOptimizedDepUrlPrefix;
exports.isPlaygroundOptimizedDepUrl = isPlaygroundOptimizedDepUrl;
exports.playgroundVitestDevStubPlugin = playgroundVitestDevStubPlugin;
exports.playgroundWasmStubKey = playgroundWasmStubKey;
exports.playgroundFlowWasmDevStubPlugin = playgroundFlowWasmDevStubPlugin;
exports.playgroundPlaywrightDevStubPlugin = playgroundPlaywrightDevStubPlugin;
exports.playgroundStaleOptimizeDepPlugin = playgroundStaleOptimizeDepPlugin;
exports.meshCollectionVitePlugin = meshCollectionVitePlugin;
exports.playgroundPlayBootHtmlPlugin = playgroundPlayBootHtmlPlugin;
exports.semioFaviconSources = semioFaviconSources;
exports.semioFaviconSvgMarkup = semioFaviconSvgMarkup;
exports.semioFaviconVitePlugin = semioFaviconVitePlugin;
exports.staticDeployMarkerVitePlugins = staticDeployMarkerVitePlugins;
exports.rewriteSpaFallbackToEmojiEntry = rewriteSpaFallbackToEmojiEntry;
exports.semioEmojiIndexHtmlVitePlugin = semioEmojiIndexHtmlVitePlugin;
exports.semioBrandHtmlVitePlugins = semioBrandHtmlVitePlugins;
exports.semioHostHtmlString = semioHostHtmlString;
exports.semioHostHtmlVitePlugin = semioHostHtmlVitePlugin;
exports.statusSurfaceHtml = statusSurfaceHtml;
exports.resolveSemioAssetRoot = resolveSemioAssetRoot;
exports.semioAssetsVitePlugin = semioAssetsVitePlugin;
exports.uiAssetsVitePlugin = uiAssetsVitePlugin;
exports.duplicateNamedImportsForModule = duplicateNamedImportsForModule;
exports.animatePresentRendererVitestStripPlugin = animatePresentRendererVitestStripPlugin;
exports.playgroundSceneHostResolveAliases = playgroundSceneHostResolveAliases;
exports.playgroundSceneHostOptimizeDeps = playgroundSceneHostOptimizeDeps;
exports.resolveGisMapTileServeMode = resolveGisMapTileServeMode;
exports.mapTileCacheRoots = mapTileCacheRoots;
exports.lonLatToTileXY = lonLatToTileXY;
exports.tileRangeForBounds = tileRangeForBounds;
exports.listMapTilesForBounds = listMapTilesForBounds;
exports.prefetchMapTiles = prefetchMapTiles;
exports.tileProxyVitePlugin = tileProxyVitePlugin;
exports.startAssetServer = startAssetServer;
exports.playgroundAssetVitePlugins = playgroundAssetVitePlugins;
exports.createWorkspaceViteResolveConfig = createWorkspaceViteResolveConfig;
exports.contentTypeForStaticDirAsset = contentTypeForStaticDirAsset;
exports.staticDirVitePlugin = staticDirVitePlugin;
exports.findWorkspacePackages = findWorkspacePackages;
exports.createPlaygroundPlayViteConfig = createPlaygroundPlayViteConfig;
// #region 🔌️Adapters
var framework_1 = require("@semio-tech/framework");
var node_http_1 = require("node:http");
var node_fs_1 = require("node:fs");
var promises_1 = require("node:fs/promises");
var node_path_1 = require("node:path");
var node_os_1 = require("node:os");
var node_url_1 = require("node:url");
var ____ts_1 = require("../../../\uD83C\uDFAF\uFE0Ftargets/\u269B\uFE0Freact/\uD83D\uDEE0\uFE0Fbuild-tooling/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../../../../../\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83C\uDFAE\uFE0Fplayground/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "PLAYGROUND_PORTS", { enumerable: true, get: function () { return ____ts_2.PLAYGROUND_PORTS; } });
Object.defineProperty(exports, "allPlaygroundReservedPorts", { enumerable: true, get: function () { return ____ts_2.allPlaygroundReservedPorts; } });
Object.defineProperty(exports, "playgroundDevPort", { enumerable: true, get: function () { return ____ts_2.playgroundDevPort; } });
Object.defineProperty(exports, "playgroundDevPortString", { enumerable: true, get: function () { return ____ts_2.playgroundDevPortString; } });
Object.defineProperty(exports, "playgroundPortEnv", { enumerable: true, get: function () { return ____ts_2.playgroundPortEnv; } });
Object.defineProperty(exports, "playgroundTestPort", { enumerable: true, get: function () { return ____ts_2.playgroundTestPort; } });
Object.defineProperty(exports, "playgroundTestPortString", { enumerable: true, get: function () { return ____ts_2.playgroundTestPortString; } });
var ____ts_3 = require("../../../../\uD83D\uDDBC\uFE0Fassets/\uD83E\uDD7D\uFE0Fmesh/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("../../../../\uD83D\uDDBC\uFE0Fassets/\uD83D\uDD0D\uFE0Fresolver/\uD83C\uDF10\uFE0Fdelivery/\uD83D\uDFE6\uFE0F.ts");
var ____json_1 = require("../../\uD83C\uDF10\uFE0Ffavicon/\uD83D\uDD23\uFE0F.json");
var ____ts_5 = require("../../\uD83C\uDF10\uFE0Fiframe/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "playgroundIframeEmbedHeadersPlugin", { enumerable: true, get: function () { return ____ts_5.playgroundIframeEmbedHeadersPlugin; } });
//#region 🔖️ViteElementsAssets
/** @emoji 📦️ Relative-base Vite build defaults for playground static sites (iframe + subdomain safe). */
function playgroundStaticSiteBuildOptions(overrides) {
    return __assign({ target: "esnext", outDir: "dist", emptyOutDir: true }, overrides);
}
/** @emoji 🚀️ Production Vite `build` defaults: minify, strip console/debugger, no sourcemaps. */
function semioViteProductionBuild(overrides) {
    var _a;
    return __assign(__assign({ target: "es2022", sourcemap: false, minify: "esbuild", cssMinify: true, reportCompressedSize: false }, overrides), { esbuild: __assign({ drop: ["console", "debugger"], legalComments: "none" }, ((_a = overrides === null || overrides === void 0 ? void 0 : overrides.esbuild) !== null && _a !== void 0 ? _a : {})) });
}
/** @emoji 🧭️ Vite's URL prefix for prebundled chunks under `cacheDir`: root-relative inside `root`, `/@fs/` outside. https://vite.dev/config/shared-options.html#cachedir */
function playgroundOptimizedDepUrlPrefix(root, cacheDir) {
    var path = (0, node_path_1.relative)(root, cacheDir).replaceAll("\\", "/");
    return path.startsWith("..") || (0, node_path_1.isAbsolute)(path) ? "/@fs/".concat((0, node_path_1.resolve)(cacheDir).replaceAll("\\", "/").replace(/^\/+/, ""), "/deps/") : "/".concat(path, "/deps/");
}
/** @emoji 🔗️ True when a percent-encoded request targets this server's Vite prebundled chunks. */
function isPlaygroundOptimizedDepUrl(url, prefix) {
    try {
        return decodeURI(url).includes(prefix);
    }
    catch (_a) {
        return false;
    }
}
/** @emoji 🧱️ Stubs vitest and testing-library when test regions enter the browser graph. */
function playgroundVitestDevStubPlugin() {
    var vitestStubId = "\0playground-vitest-dev-stub";
    var testingLibraryStubId = "\0playground-testing-library-dev-stub";
    return {
        name: "playground-vitest-dev-stub",
        enforce: "pre",
        resolveId: function (id) {
            if (id === "vitest" || id.startsWith("vitest/") || id.startsWith("@vitest/"))
                return vitestStubId;
            if (id === "@testing-library/react" || id.startsWith("@testing-library/"))
                return testingLibraryStubId;
            return undefined;
        },
        load: function (id) {
            if (id === vitestStubId) {
                return "export default {}; export const describe = () => {}; export const it = () => {}; export const expect = () => ({ toBe: () => {}, toEqual: () => {} }); export const vi = { fn: () => {}, mock: () => {}, spyOn: () => {} };";
            }
            if (id === testingLibraryStubId) {
                return "export default {}; export const render = () => ({}); export const screen = {}; export const fireEvent = {}; export const waitFor = async (fn) => fn();";
            }
        },
    };
}
var PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID = "\0playground-playwright-dev-stub";
exports.PLAYGROUND_WASM_STUB_PREFIX = "\0playground-wasm-stub/";
/** 🗂️ Vite's URL form for an absolute filesystem path outside the project root. */
var FS_URL_PREFIX = "/@fs/";
function playgroundWasmStubKey(cleanId) {
    return cleanId.replace(/\//g, "__");
}
function playgroundWasmStubKeyDecode(key) {
    return key.replace(/__/g, "/");
}
var PLAYGROUND_WASM_JS_STUB = "const wasmMissing = () => { throw new Error(\"wasm pkg not built \u2014 run the matching nx wasm target\"); };\nconst wasmJson = () => \"{}\";\nconst dagLodScaleJson = () => ".concat(JSON.stringify(JSON.stringify([
    { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; fill only.", maxZoom: 0.4 },
    { id: "overview", name: "Overview", description: "Node icons only.", maxZoom: 0.6 },
    { id: "compact", name: "Compact", description: "Horizontal abbreviations.", maxZoom: 0.8 },
    { id: "normal", name: "Normal", description: "Vertical names with sections; channel abbreviations on ports.", maxZoom: 1.5 },
    { id: "detail", name: "Detail", description: "Channel names on ports, port handles, and control text.", maxZoom: 2.75 },
    { id: "micro", name: "Micro", description: "Full channel names on ports and maximum node fidelity.", maxZoom: Number.MAX_VALUE },
])), ";\nexport default async function initWasm() {}\nexport const initSync = () => {};\nexport class FlowSession { lodScaleJson() { return dagLodScaleJson(); } attachCanvas() { return Promise.resolve(); } setSize() {} renderFrame() {} loadFixtureJson() {} fixtureJson() { return \"{}\"; } setCatalogueJson() {} catalogueJson() { return \"[]\"; } setNeuronKindInfosJson() {} setComputingProgress() {} setAutomaticLod() {} setForcedDrawLodLabel() {} setCanvasThemeJson() {} setCamera() {} viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} labelOverlayPaintStateJson() { return '{\"labels\":[]}'; } sliderOverlayStateJson() { return '{\"sliders\":[]}'; } selectionUnionBoundsScreenJson() { return \"{}\"; } selectionPreviewPointsJson() { return \"[]\"; } selectionPreviewCrossing() { return false; } selectedWidgetIds() { return \"[]\"; } hoveredWidgetId() { return undefined; } hoveredChannelJson() { return \"{}\"; } pickTargetsAtScreenJson() { return \"[]\"; } previewText() { return \"\"; } preselectWidgetIdsJson() { return \"[]\"; } previewOffWidgetIds() { return \"[]\"; } alignSelection() {} undo() { return false; } redo() { return false; } selectAll() {} deleteSelection() {} addWidget() { return \"\"; } setGhostWidget() {} clearGhostWidget() {} worldFromScreen() { return '{\"x\":0,\"y\":0}'; } applyEvalOutputsJson() {} setSliderValue() {} setNeuronParams() {} setSelection() {} setPreviewOff() {} syncFromSceneJson() {}}\nexport class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{\"labels\":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }\nexport class EditorSession { syncFromSceneJson() {} syncFromScenePack() {} setText() {} text() { return ''; } caret() { return 0; } anchor() { return 0; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScrollScreen() {} insertText() {} backspace() {} deleteForward() {} selectAll() {} replaceSelection() {} selectionText() { return ''; } hoverTokenRangeJson() { return 'null'; } setHoverRange() {} cameraJson() { return '{}'; } }\nexport class DagSession { lodScaleJson() { return dagLodScaleJson(); } }\nexport class BoardSession { lodScaleJson() { return dagLodScaleJson(); } }\nexport class WriterSession {}\nexport class ImperativeSession {}\nexport class SequenceSession {}\nexport class RasterSession {}\nexport class MapSession {}\nexport class Puzzle3dPrecomputeSession {}\nexport class TrinitySession {}\nexport class JackLspSession {}\nexport const render_drawing_scene = wasmMissing;\nexport const export_drawing_svg = wasmMissing;\nexport const export_drawing_pdf = wasmMissing;\nexport const dispose_drawing = () => {};\nexport const trace_drawing_bitmap = wasmMissing;\nexport const boolean_drawing_segments = wasmMissing;\nexport const tessellate = async () => JSON.stringify({ positions: [], normals: [], index: [], edges: [], points: [], faceGroups: [] });\nexport const dispose = () => {};\nexport const evaluate = wasmMissing;\nexport const ruleQueryJson = wasmJson;\nexport const boardComputeEdgeBezier = wasmJson;\nexport const boardHandlePositionCircle = wasmJson;\nexport const boardHandlePositionRectangle = wasmJson;\nexport const boardRedrawHandlesFixtureJson = wasmJson;\nexport const boardRedrawLayoutFixtureJson = wasmJson;\n");
function workspaceWasmPkgResolveCandidates(repoRoot, pkgName, subpath) {
    var pkgRoot = (0, node_path_1.resolve)(repoRoot, "node_modules", pkgName);
    var candidates = [];
    var manifest;
    try {
        manifest = JSON.parse((0, node_fs_1.readFileSync)((0, node_path_1.resolve)(pkgRoot, "package.json"), "utf8"));
    }
    catch (_a) {
        /* package.json may be absent for a half-linked workspace package */
    }
    var pushExportTarget = function (key) {
        var _a, _b;
        var exp = (_a = manifest === null || manifest === void 0 ? void 0 : manifest.exports) === null || _a === void 0 ? void 0 : _a[key];
        var target = typeof exp === "string" ? exp : ((_b = exp === null || exp === void 0 ? void 0 : exp.import) !== null && _b !== void 0 ? _b : exp === null || exp === void 0 ? void 0 : exp.default);
        if (target)
            candidates.push((0, node_path_1.resolve)(pkgRoot, target));
    };
    if (subpath) {
        candidates.push((0, node_path_1.resolve)(pkgRoot, subpath));
        if (subpath.startsWith("pkg/")) {
            candidates.push((0, node_path_1.resolve)(pkgRoot, "rs", subpath));
            candidates.push((0, node_path_1.resolve)(pkgRoot, subpath.slice("pkg/".length)));
        }
        pushExportTarget("./".concat(subpath));
        pushExportTarget(subpath);
    }
    else {
        pushExportTarget(".");
        if (manifest === null || manifest === void 0 ? void 0 : manifest.module)
            candidates.push((0, node_path_1.resolve)(pkgRoot, manifest.module));
        if (manifest === null || manifest === void 0 ? void 0 : manifest.main)
            candidates.push((0, node_path_1.resolve)(pkgRoot, manifest.main));
    }
    return candidates;
}
/** @emoji 🧱️ Stubs missing wasm pkg imports until `nx run …:wasm` artifacts exist. */
function playgroundFlowWasmDevStubPlugin(repoRoot) {
    return {
        name: "playground-flow-wasm-dev-stub",
        enforce: "pre",
        resolveId: function (id, importer) {
            var _a;
            if (!importer || id.startsWith(exports.PLAYGROUND_WASM_STUB_PREFIX))
                return undefined;
            var cleanId = (_a = id.split("?", 1)[0]) !== null && _a !== void 0 ? _a : id;
            // 🗂️ A module the browser requests directly arrives as Vite's own `/@fs/<absolute path>` URL, not
            // as the specifier the importer wrote. Testing that URL for existence always fails, which used to
            // hand back the "wasm pkg not built" stub for a pkg that is sitting right there on disk.
            var fsId = cleanId.startsWith(FS_URL_PREFIX) ? cleanId.slice(FS_URL_PREFIX.length - 1) : cleanId;
            var isWasmPkg = cleanId.includes("/pkg/") || cleanId.endsWith(".wasm") || cleanId === "@semio-tech/flow-core" || cleanId === "@semio-tech/flow-core/pkg/flow_core.js" || cleanId === "@semio-tech/flow-core/flow_core.js";
            if (!isWasmPkg)
                return undefined;
            if (fsId.startsWith(".")) {
                if ((0, node_fs_1.existsSync)((0, node_path_1.resolve)((0, node_path_1.dirname)(importer), fsId)))
                    return undefined;
                return "".concat(exports.PLAYGROUND_WASM_STUB_PREFIX).concat(playgroundWasmStubKey(cleanId));
            }
            var workspacePkg = fsId.match(/^(@semio-tech\/[^/]+)(?:\/(.+))?$/);
            var candidates = [];
            if (workspacePkg) {
                var pkgName = workspacePkg[1], subpath = workspacePkg[2];
                candidates.push.apply(candidates, workspaceWasmPkgResolveCandidates(repoRoot, pkgName, subpath));
            }
            else {
                candidates.push((0, node_path_1.resolve)(repoRoot, fsId));
            }
            var hit = candidates.find(function (abs) { return (0, node_fs_1.existsSync)(abs); });
            if (hit)
                return hit;
            return "".concat(exports.PLAYGROUND_WASM_STUB_PREFIX).concat(playgroundWasmStubKey(cleanId));
        },
        load: function (id) {
            var _a;
            if (!id.startsWith(exports.PLAYGROUND_WASM_STUB_PREFIX))
                return undefined;
            var cleanId = playgroundWasmStubKeyDecode((_a = id.slice(exports.PLAYGROUND_WASM_STUB_PREFIX.length).split("?", 1)[0]) !== null && _a !== void 0 ? _a : "");
            if (cleanId.endsWith(".wasm"))
                return "export default \"\";";
            return PLAYGROUND_WASM_JS_STUB;
        },
    };
}
/** @emoji 🧱️ Stubs Playwright when test-only regions are pulled into the browser graph. */
function playgroundPlaywrightDevStubPlugin() {
    return {
        name: "playground-playwright-dev-stub",
        enforce: "pre",
        resolveId: function (id) {
            if (id === "@playwright/test" || id === "playwright" || id === "playwright-core" || id === "chromium-bidi") {
                return PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID;
            }
            return undefined;
        },
        load: function (id) {
            if (id !== PLAYGROUND_PLAYWRIGHT_DEV_STUB_ID)
                return;
            return "export default {}; export const test = () => {}; export const expect = () => ({ toBe: () => {}, toEqual: () => {} });";
        },
    };
}
/** @emoji 🔄️ Full-reload connected clients when a stale optimized-dep chunk returns 504. */
function playgroundStaleOptimizeDepPlugin() {
    return {
        name: "playground-stale-optimize-dep",
        configureServer: function (server) {
            var prefix = playgroundOptimizedDepUrlPrefix(server.config.root, server.config.cacheDir);
            server.middlewares.use(function (req, res, next) {
                var _a;
                if (!isPlaygroundOptimizedDepUrl((_a = req.url) !== null && _a !== void 0 ? _a : "", prefix)) {
                    next();
                    return;
                }
                res.on("finish", function () {
                    if (res.statusCode === 504) {
                        server.ws.send({ type: "full-reload", path: "*" });
                    }
                });
                next();
            });
        },
    };
}
function contentTypeForUiAsset(filePath) {
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
function createUiAssetsMiddleware(assetsRoot) {
    var assetsRootResolved = (0, node_path_1.resolve)(assetsRoot);
    return function (req, res, next) {
        var rel = req.url ? (0, ____ts_4.assetPathFromRequest)(req.url) : null;
        if (rel === null) {
            next();
            return;
        }
        var filePath = (0, node_path_1.resolve)(assetsRootResolved, rel);
        var relToRoot = (0, node_path_1.relative)(assetsRootResolved, filePath);
        if (relToRoot.startsWith("..") || (0, node_path_1.isAbsolute)(relToRoot) || !(0, node_fs_1.existsSync)(filePath) || !(0, node_fs_1.statSync)(filePath).isFile()) {
            next();
            return;
        }
        var contentType = contentTypeForUiAsset(filePath);
        if (contentType) {
            res.setHeader("Content-Type", contentType);
        }
        (0, node_fs_1.createReadStream)(filePath).pipe(res);
    };
}
//#region 🔖️MeshCollectionAssetPlugin
function readMeshDeliveryCatalog(repoRoot, spec) {
    if (spec.route !== "/mesh")
        throw new Error("Unsupported mesh catalog route: ".concat(spec.route));
    var read = function (path) { return JSON.parse((0, node_fs_1.readFileSync)((0, node_path_1.resolve)(repoRoot, path), "utf8")); };
    return (0, ____ts_3.parseMeshDeliveryCatalog)(read(spec.catalog), read);
}
/** 🌐️ Serves only explicit catalog transport paths, retaining public identity at the caller boundary. */
function createMeshCollectionMiddleware(repoRoot, spec) {
    var route = "".concat(spec.route, "/");
    var catalog = new Map(readMeshDeliveryCatalog(repoRoot, spec).map(function (entry) { return ["".concat(route).concat(entry.path), entry]; }));
    return function (req, res, next) {
        var _a, _b;
        if (!((_a = req.url) === null || _a === void 0 ? void 0 : _a.startsWith(route))) {
            next();
            return;
        }
        var path;
        try {
            path = decodeURIComponent((_b = req.url.split(/[?#]/, 1)[0]) !== null && _b !== void 0 ? _b : "");
        }
        catch (_c) {
            res.statusCode = 400;
            res.end();
            return;
        }
        var entry = catalog.get(path);
        if (!entry)
            return next();
        var source = (0, node_path_1.resolve)(repoRoot, entry.source);
        if (!(0, node_fs_1.existsSync)(source) || !(0, node_fs_1.statSync)(source).isFile())
            return next();
        res.setHeader("Content-Type", "model/gltf-binary");
        (0, node_fs_1.createReadStream)(source).pipe(res);
    };
}
/** 📦️ Copies only admitted source entries to their exact handpicked nested delivery paths. */
function copyMeshCollectionGlbs(repoRoot, catalog, dest) {
    (0, node_fs_1.mkdirSync)(dest, { recursive: true });
    for (var _i = 0, catalog_1 = catalog; _i < catalog_1.length; _i++) {
        var entry = catalog_1[_i];
        var source = (0, node_path_1.resolve)(repoRoot, entry.source);
        if (!(0, node_fs_1.existsSync)(source) || !(0, node_fs_1.statSync)(source).isFile())
            throw new Error("Missing catalog mesh source: ".concat(entry.source));
        var destination = (0, node_path_1.resolve)(dest, entry.path);
        (0, node_fs_1.mkdirSync)((0, node_path_1.dirname)(destination), { recursive: true });
        (0, node_fs_1.cpSync)(source, destination);
    }
}
/** 🧊️ Dev and static delivery share one explicit public-ID/source/output authority. */
function meshCollectionVitePlugin(repoRoot, spec) {
    var serveMeshes = createMeshCollectionMiddleware(repoRoot, spec);
    var catalog = readMeshDeliveryCatalog(repoRoot, spec);
    var destName = spec.route.replace(/^\//, "");
    var outDir = (0, node_path_1.resolve)(process.cwd(), "dist");
    var writeOutput = true;
    return [
        {
            name: "mesh-collection-serve".concat(spec.route),
            enforce: "pre",
            configureServer: function (server) {
                server.middlewares.use(serveMeshes);
            },
            configurePreviewServer: function (server) {
                server.middlewares.use(serveMeshes);
            },
        },
        {
            name: "mesh-collection-build".concat(spec.route),
            apply: "build",
            enforce: "pre",
            configResolved: function (config) {
                outDir = (0, node_path_1.resolve)(config.root, config.build.outDir);
                writeOutput = config.build.write !== false;
            },
            closeBundle: function () {
                if (!writeOutput)
                    return;
                var dest = (0, node_path_1.resolve)(outDir, destName);
                (0, node_fs_1.mkdirSync)(outDir, { recursive: true });
                copyMeshCollectionGlbs(repoRoot, catalog, dest);
            },
        },
    ];
}
//#endregion 🔖️MeshCollectionAssetPlugin
//#region 🔖️HostHtmlPlugin
/** @emoji 🎬️ Inline shell paint before Tailwind finishes compiling the play stylesheet. */
exports.PLAYGROUND_PLAY_BOOT_INLINE_STYLE = "html{color-scheme:light dark}html,body,#root{height:100%;margin:0}body{background-color:#f7f3e3;color:#001117}html.dark body{background-color:#001117;color:#f7f3e3}html:not([data-semio-styled]) body{visibility:hidden}";
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
exports.PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT = "(function(){var d=document.documentElement,m=window.matchMedia(\"(prefers-color-scheme: dark)\");var a=null;try{var c=JSON.parse(localStorage.getItem(\"semio.os.config\")||\"null\");var l=c&&c.preferences&&c.preferences[\"os.config.ui-preferences\"];var g=l?JSON.parse(l):null;if(g&&g.version===1&&g.events&&g.events.length){for(var i=0;i<g.events.length;i++){var e=g.events[i];if(e&&e.mutation===\"setAppearance\")a=e.appearance||null}}}catch(e){}var dark=a===\"dark\"||(a!==\"light\"&&m.matches);d.classList.toggle(\"dark\",dark);d.dataset.uiAppearance=dark?\"dark\":\"light\";d.style.colorScheme=dark?\"dark\":\"light\";if(document.body){document.body.style.colorScheme=dark?\"dark\":\"light\";document.body.style.backgroundColor=dark?\"#001117\":\"#f7f3e3\";document.body.style.color=dark?\"#f7f3e3\":\"#001117\";}})();";
/** @emoji 👁️ Reveals the play shell after the linked globals stylesheet finishes loading. */
exports.PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT = "(function(){function reveal(){document.documentElement.dataset.semioStyled=\"ready\"}var link=document.getElementById(\"semio-play-styles\");if(link){if(link.sheet)reveal();else link.addEventListener(\"load\",reveal,{once:true})}else{reveal()}setTimeout(reveal,8000)})();";
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
exports.PLAYGROUND_PLAY_BOOT_THEME_SCRIPT = "(function(){try{var c=JSON.parse(localStorage.getItem(\"semio.os.config\")||\"null\");var l=c&&c.preferences&&c.preferences[\"os.config.ui-preferences\"];var g=l?JSON.parse(l):null;if(!g||g.version!==1||!g.events)return;var id=null,themes={};for(var i=0;i<g.events.length;i++){var e=g.events[i];if(!e)continue;if(e.mutation===\"setTheme\")id=e.themeId||null;if(e.mutation===\"setCustomTheme\"){if(e.theme)themes[e.themeId]=e.theme;else delete themes[e.themeId]}}var stored=id?themes[id]:null;var t=stored&&stored.config;if(!t||!t.colors)return;var d=document.documentElement;var dark=d.classList.contains(\"dark\");for(var k in t.colors){d.style.setProperty(\"--color-\"+k.replace(/_/g,\"-\"),t.colors[k])}if(t.spacing)for(var s in t.spacing){d.style.setProperty(\"--spacing-\"+s.replace(/_/g,\"-\"),t.spacing[s])}d.dataset.uiTheme=id;var appearance=t.appearances&&t.appearances[dark?\"dark\":\"light\"];var chrome=appearance&&appearance.chrome;function resolveSimple(ref){return ref&&ref.token&&t.colors[ref.token]?t.colors[ref.token]:undefined}var base=chrome&&resolveSimple(chrome.base);var fg=chrome&&resolveSimple(chrome.foreground);if(document.body){if(base)document.body.style.backgroundColor=base;if(fg)document.body.style.color=fg}}catch(e){}})();";
/** @emoji 🧬️ Boot-time head tags every semio host document shares (color-scheme inline style + synchronous
 * appearance/theme scripts) — single source both {@link semioHostHtmlString} and
 * {@link playgroundPlayBootHtmlPlugin} inject from, so the generalized host and playground never drift. */
function semioHostBootHeadTags() {
    return [
        { tag: "style", children: exports.PLAYGROUND_PLAY_BOOT_INLINE_STYLE, injectTo: "head-prepend" },
        { tag: "script", children: exports.PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, injectTo: "head-prepend" },
        { tag: "script", children: exports.PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, injectTo: "head-prepend" },
    ];
}
/** @emoji 🎬️ Vite: inject early appearance + theme + stylesheet link into play `🌐️.html` to avoid unstyled flashes — additive tag injection onto each play's own hand-authored `🌐️.html`, sharing its boot-head fragment ({@link semioHostBootHeadTags}) with {@link semioHostHtmlVitePlugin} instead of duplicating the style/script assembly. */
function playgroundPlayBootHtmlPlugin() {
    return {
        name: "playground-play-boot-html",
        transformIndexHtml: {
            order: "pre",
            handler: function () {
                return {
                    tags: __spreadArray(__spreadArray([], semioHostBootHeadTags(), true), [
                        { tag: "link", attrs: { rel: "stylesheet", href: "./🎨️.css", id: "semio-play-styles" }, injectTo: "head" },
                        { tag: "script", children: exports.PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, injectTo: "head" },
                    ], false),
                };
            },
        },
    };
}
/** @emoji 🔖️ Canonical semio emblem favicon `<link>` tags for playground and app `🌐️.html` heads. */
exports.SEMIO_FAVICON_HEAD_HTML = "<link rel=\"icon\" href=\"./".concat(____json_1.default.svg, "\" type=\"image/svg+xml\" />\n    <link rel=\"icon\" href=\"./").concat(____json_1.default.ico, "\" sizes=\"any\" />");
/** @emoji 🔖️ Repo-root paths for the round dark emblem SVG and ICO fallback (matches {@link SemioLogo}). */
function semioFaviconSources(repoRoot) {
    var logoRoot = (0, node_path_1.resolve)(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos");
    return {
        svg: (0, node_path_1.resolve)(logoRoot, "🛡️emblem/🌘️dark-round/🖋️vector.svg"),
        ico: (0, node_path_1.resolve)(logoRoot, "🌐️favicon/🌘️dark-round/📏️size-32.ico"),
    };
}
var SEMIO_FAVICON_BLEED_RECT = '<rect width="350" height="350" fill="#001117"/>';
/** @emoji 🔖️ Favicon SVG with opaque bleed so ICO rasterization avoids white matte outside the round emblem. */
function semioFaviconSvgMarkup(svgPath) {
    var _a;
    if (!(0, node_fs_1.existsSync)(svgPath)) {
        return undefined;
    }
    var raw = (0, node_fs_1.readFileSync)(svgPath, "utf8");
    if (raw.includes(SEMIO_FAVICON_BLEED_RECT)) {
        return raw;
    }
    var open = (_a = raw.match(/<svg[^>]*>/)) === null || _a === void 0 ? void 0 : _a[0];
    if (!open) {
        return raw;
    }
    return raw.replace(open, "".concat(open).concat(SEMIO_FAVICON_BLEED_RECT));
}
var STATIC_SITE_FAVICON_ALIASES = { svg: "favicon.svg", ico: "favicon.ico" };
function createFaviconMiddleware(content) {
    return function (req, res, next) {
        var _a, _b;
        var url;
        try {
            url = decodeURIComponent((_b = (_a = req.url) === null || _a === void 0 ? void 0 : _a.split(/[?#]/, 1)[0]) !== null && _b !== void 0 ? _b : "");
        }
        catch (_c) {
            next();
            return;
        }
        if ((url === "/".concat(____json_1.default.svg) || url === "/".concat(STATIC_SITE_FAVICON_ALIASES.svg)) && content.svgMarkup) {
            res.setHeader("Content-Type", "image/svg+xml");
            res.end(content.svgMarkup);
            return;
        }
        if ((url === "/".concat(____json_1.default.ico) || url === "/".concat(STATIC_SITE_FAVICON_ALIASES.ico)) && content.icoPath && (0, node_fs_1.existsSync)(content.icoPath)) {
            res.setHeader("Content-Type", "image/x-icon");
            (0, node_fs_1.createReadStream)(content.icoPath).pipe(res);
            return;
        }
        next();
    };
}
/** @emoji 🔖️ Vite: serve and copy the given emblem SVG and bookmark ICO under their exact publication names. */
function faviconVitePlugins(content) {
    var serveFavicon = createFaviconMiddleware(content);
    var outDir = (0, node_path_1.resolve)(process.cwd(), "dist");
    var writeOutput = true;
    return [
        {
            name: "semio-favicon-serve",
            enforce: "pre",
            configureServer: function (server) {
                server.middlewares.use(serveFavicon);
            },
            configurePreviewServer: function (server) {
                server.middlewares.use(serveFavicon);
            },
        },
        {
            name: "semio-favicon-build",
            apply: "build",
            enforce: "pre",
            configResolved: function (config) {
                outDir = (0, node_path_1.resolve)(config.root, config.build.outDir);
                writeOutput = config.build.write !== false;
            },
            closeBundle: function () {
                if (!writeOutput)
                    return;
                var dist = outDir;
                (0, node_fs_1.mkdirSync)(dist, { recursive: true });
                if (content.svgMarkup) {
                    (0, node_fs_1.writeFileSync)((0, node_path_1.resolve)(dist, ____json_1.default.svg), content.svgMarkup);
                    (0, node_fs_1.writeFileSync)((0, node_path_1.resolve)(dist, STATIC_SITE_FAVICON_ALIASES.svg), content.svgMarkup);
                }
                if (content.icoPath && (0, node_fs_1.existsSync)(content.icoPath)) {
                    (0, node_fs_1.cpSync)(content.icoPath, (0, node_path_1.resolve)(dist, ____json_1.default.ico));
                    (0, node_fs_1.cpSync)(content.icoPath, (0, node_path_1.resolve)(dist, STATIC_SITE_FAVICON_ALIASES.ico));
                }
            },
        },
    ];
}
/** @emoji 🔖️ Vite: serve and copy semio emblem favicons at `/🛡️favicon.svg` and `/🔖️favicon.ico`. */
function semioFaviconVitePlugin(repoRoot) {
    var favicons = semioFaviconSources(repoRoot);
    return faviconVitePlugins({ svgMarkup: semioFaviconSvgMarkup(favicons.svg), icoPath: favicons.ico });
}
/** @emoji 🚫️ Vite: writes `dist/.nojekyll` on every build (unconditionally — any static host that runs
 * Jekyll, e.g. GitHub Pages, silently drops files/dirs starting with `_` otherwise, breaking Vite's own
 * `__vite-browser-external-*.js` shim chunk) and `dist/CNAME` when a brand declares `cnameHost`. */
function staticDeployMarkerVitePlugins(cnameHost) {
    var outDir = (0, node_path_1.resolve)(process.cwd(), "dist");
    var writeOutput = true;
    return [
        {
            name: "static-deploy-markers",
            apply: "build",
            enforce: "pre",
            configResolved: function (config) {
                outDir = (0, node_path_1.resolve)(config.root, config.build.outDir);
                writeOutput = config.build.write !== false;
            },
            closeBundle: function () {
                if (!writeOutput)
                    return;
                (0, node_fs_1.mkdirSync)(outDir, { recursive: true });
                (0, node_fs_1.writeFileSync)((0, node_path_1.resolve)(outDir, ".nojekyll"), "");
                if (cnameHost)
                    (0, node_fs_1.writeFileSync)((0, node_path_1.resolve)(outDir, "CNAME"), "".concat(cnameHost, "\n"));
            },
        },
    ];
}
/** @emoji 🧭️ Rewrites Vite's SPA fallback target `/index.html` onto the constitutional emoji entry path. */
function rewriteSpaFallbackToEmojiEntry(url, entryPath) {
    var _a = url.split(/(?=[?#])/), pathOnly = _a[0], rest = _a.slice(1);
    var base = pathOnly !== null && pathOnly !== void 0 ? pathOnly : url;
    if (base !== "/index.html")
        return url;
    return "".concat(entryPath).concat(rest.join(""));
}
function semioEmojiIndexHtmlRootRewrite(entry) {
    return function (req, _res, next) {
        var _a;
        var url = (_a = req.url) !== null && _a !== void 0 ? _a : "";
        if (url === "/" || url.startsWith("/?"))
            req.url = "".concat(entry).concat(url.slice(1));
        next();
    };
}
function semioEmojiIndexHtmlSpaFallbackRewrite(entry) {
    return function (req, _res, next) {
        var _a;
        var url = (_a = req.url) !== null && _a !== void 0 ? _a : "";
        var nextUrl = rewriteSpaFallbackToEmojiEntry(url, entry);
        if (nextUrl !== url)
            req.url = nextUrl;
        next();
    };
}
/** @emoji 📄️ Conventional static-host entry filenames emitted beside the constitutional emoji HTML entry. */
exports.STATIC_SITE_HTML_ALIASES = ["index.html", "404.html"];
/** @emoji 🌐️ Vite: treat hand-authored `🌐️.html` as the app index (`/` + build input). Vite's default
 * `index.html` name does not match the constitutional emoji entry filename. */
function semioEmojiIndexHtmlVitePlugin(rootDir, fileName) {
    if (fileName === void 0) { fileName = "🌐️.html"; }
    var entry = "/".concat(fileName);
    var outDir = (0, node_path_1.resolve)(process.cwd(), "dist");
    var writeOutput = true;
    return {
        name: "semio-emoji-index-html",
        enforce: "pre",
        config: function () {
            return {
                build: {
                    rollupOptions: {
                        input: (0, node_path_1.resolve)(rootDir, fileName),
                    },
                },
            };
        },
        configResolved: function (config) {
            outDir = (0, node_path_1.resolve)(config.root, config.build.outDir);
            writeOutput = config.build.write !== false;
        },
        closeBundle: function () {
            if (!writeOutput)
                return;
            var source = (0, node_path_1.resolve)(outDir, fileName);
            if (!(0, node_fs_1.existsSync)(source))
                return;
            var html = (0, node_fs_1.readFileSync)(source);
            for (var _i = 0, STATIC_SITE_HTML_ALIASES_1 = exports.STATIC_SITE_HTML_ALIASES; _i < STATIC_SITE_HTML_ALIASES_1.length; _i++) {
                var alias = STATIC_SITE_HTML_ALIASES_1[_i];
                (0, node_fs_1.writeFileSync)((0, node_path_1.resolve)(outDir, alias), html);
            }
        },
        configureServer: function (server) {
            server.middlewares.use(semioEmojiIndexHtmlRootRewrite(entry));
            return function () {
                server.middlewares.use(semioEmojiIndexHtmlSpaFallbackRewrite(entry));
            };
        },
        configurePreviewServer: function (server) {
            server.middlewares.use(semioEmojiIndexHtmlRootRewrite(entry));
            return function () {
                server.middlewares.use(semioEmojiIndexHtmlSpaFallbackRewrite(entry));
            };
        },
    };
}
/** @emoji 🏷️ Vite: brand-aware host chrome — rewrites the `<title>` to the brand's `windowTitle`, serves/copies the brand mark at `/🛡️favicon.svg` (ICO only when the brand provides one), and writes the static-deploy markers above; no brand ⇒ canonical semio favicons (still with `.nojekyll`). */
function semioBrandHtmlVitePlugins(repoRoot, brand) {
    if (!brand)
        return __spreadArray(__spreadArray([], semioFaviconVitePlugin(repoRoot), true), staticDeployMarkerVitePlugins(undefined), true);
    return __spreadArray(__spreadArray(__spreadArray([], faviconVitePlugins({ svgMarkup: brand.logoSvg, icoPath: brand.faviconIcoPath ? (0, node_path_1.resolve)(repoRoot, brand.faviconIcoPath) : undefined }), true), staticDeployMarkerVitePlugins(brand.cnameHost), true), [
        {
            name: "semio-brand-html",
            transformIndexHtml: {
                order: "pre",
                handler: function (html) { return html.replace(/<title>[^<]*<\/title>/, "<title>".concat(brand.windowTitle, "</title>")); },
            },
        },
    ], false);
}
/** @emoji 🪧️ Pre-mount placeholder markup shown inside `#{rootId}` until the entry module mounts and
 * replaces it — inline-styled so it renders before any external stylesheet loads. */
function semioHostLoadingHtml(loading) {
    if (!loading) {
        return "";
    }
    return "<div style=\"display:flex;align-items:center;justify-content:center;height:100%;font:14px system-ui,sans-serif\">".concat(loading.title, "</div>");
}
/** @emoji 📄️ Generates a complete semio host `🌐️.html` document: doctype/head (title, favicon,
 * optional CSP, boot style + appearance/theme scripts) and body (`#{rootId}` mount with pre-mount loading
 * copy, the reveal script, and the entry module script) — the single source of truth
 * {@link semioHostHtmlVitePlugin} renders from, reusable as-is by non-Vite hosts such as a VS Code webview. */
function semioHostHtmlString(spec) {
    var _a;
    var rootId = (_a = spec.rootId) !== null && _a !== void 0 ? _a : "root";
    var cspTag = spec.csp ? "<meta http-equiv=\"Content-Security-Policy\" content=\"".concat(spec.csp, "\" />\n    ") : "";
    var headTags = semioHostBootHeadTags()
        .map(function (tag) { return (tag.tag === "style" ? "<style>".concat(tag.children, "</style>") : "<script>".concat(tag.children, "</script>")); })
        .join("\n    ");
    return "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    ".concat(cspTag, "<title>").concat(spec.title, "</title>\n    ").concat(exports.SEMIO_FAVICON_HEAD_HTML, "\n    ").concat(headTags, "\n  </head>\n  <body").concat(spec.bodyClass ? " class=\"".concat(spec.bodyClass, "\"") : "", ">\n    <div id=\"").concat(rootId, "\">").concat(semioHostLoadingHtml(spec.loading), "</div>\n    <script>").concat(exports.PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, "</script>\n    <script type=\"module\" src=\"").concat(spec.entry, "\"></script>\n  </body>\n</html>\n");
}
/** @emoji 🎬️ Vite: renders {@link semioHostHtmlString} as the app's `🌐️.html` on every request/build
 * (full-document replace, `order: "pre"` so later plugins such as `@vitejs/plugin-react`'s HMR preamble
 * still layer on top), bundles semio favicon serving ({@link semioFaviconVitePlugin}), and writes the
 * static-deploy markers ({@link staticDeployMarkerVitePlugins} — `.nojekyll` always, `CNAME` when
 * `spec.cnameHost` is set) — one call wires an app's whole boot + deploy surface instead of a
 * hand-authored `🌐️.html` plus a separate build-output step. */
function semioHostHtmlVitePlugin(repoRoot, spec) {
    return __spreadArray(__spreadArray(__spreadArray([], semioFaviconVitePlugin(repoRoot), true), staticDeployMarkerVitePlugins(spec.cnameHost), true), [
        {
            name: "semio-host-html",
            transformIndexHtml: {
                order: "pre",
                handler: function () {
                    return semioHostHtmlString(spec);
                },
            },
        },
    ], false);
}
//#endregion 🔖️HostHtmlPlugin
//#region 🔖️StatusSurfaceHtml
/** @emoji 🎨️ Light/dark background+foreground hex pair mirrored from {@link PLAYGROUND_PLAY_BOOT_INLINE_STYLE}
 * / {@link PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT} — this file has no `../🎨️styling/🔣️.json` import, so these are the
 * canonical values already baked into every other boot surface here, not new ones. */
var SEMIO_STATUS_SURFACE_COLORS = { lightBg: "#f7f3e3", lightFg: "#001117", darkBg: "#001117", darkFg: "#f7f3e3" };
var SEMIO_STATUS_SURFACE_GLYPH = { empty: "◌️", error: "⚠️", loading: "…" };
function semioStatusSurfaceInlineStyle() {
    var c = SEMIO_STATUS_SURFACE_COLORS;
    return "html{color-scheme:light dark}html,body{height:100%;margin:0}body{background-color:".concat(c.lightBg, ";color:").concat(c.lightFg, ";display:flex;align-items:center;justify-content:center;font-family:system-ui,sans-serif}@media (prefers-color-scheme: dark){body{background-color:").concat(c.darkBg, ";color:").concat(c.darkFg, "}}");
}
/** @emoji 🚦️ Minimal, standalone status document (empty/error/loading) for host-agnostic contexts that
 * can't run React — e.g. a WebView2 navigation-failure page — fully inline-styled so it renders with zero
 * external CSS/JS dependency, reusing the same light/dark hex values every other boot surface in this
 * file uses. */
function statusSurfaceHtml(spec) {
    var description = spec.description ? "<p style=\"margin:8px 0 0;font-size:14px;opacity:0.72\">".concat(spec.description, "</p>") : "";
    return "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>".concat(spec.title, "</title>\n    <style>").concat(semioStatusSurfaceInlineStyle(), "</style>\n  </head>\n  <body data-status-kind=\"").concat(spec.kind, "\">\n    <div style=\"text-align:center;max-width:28rem;padding:0 24px\">\n      <p style=\"margin:0 0 8px;font-size:28px\" aria-hidden=\"true\">").concat(SEMIO_STATUS_SURFACE_GLYPH[spec.kind], "</p>\n      <p style=\"margin:0;font-size:16px;font-weight:600\">").concat(spec.title, "</p>\n      ").concat(description, "\n    </div>\n  </body>\n</html>\n");
}
//#endregion 🔖️StatusSurfaceHtml
/** 🗂️ Canonical repo-relative root of the asset-owned public namespace. */
exports.SEMIO_ASSET_ROOT = "🧰️framework/🔨️modules/🖼️assets";
/** @emoji 📂 Resolves and validates the merged Semio asset package root (fonts required). */
function resolveSemioAssetRoot(repoRoot) {
    var assetsRoot = (0, node_path_1.resolve)(repoRoot, exports.SEMIO_ASSET_ROOT);
    var fontDir = (0, node_path_1.resolve)(assetsRoot, "🔤️fonts");
    if (!(0, node_fs_1.existsSync)(assetsRoot) || !(0, node_fs_1.statSync)(assetsRoot).isDirectory() || !(0, node_fs_1.existsSync)(fontDir)) {
        throw new Error("Missing Semio asset root at ".concat(assetsRoot, " (expected ").concat(exports.SEMIO_ASSET_ROOT, " with \uD83D\uDD24\uFE0Ffonts)"));
    }
    return assetsRoot;
}
function uiAssetsVitePluginsForRoot(assetsRoot) {
    var outDir = (0, node_path_1.resolve)(process.cwd(), "dist");
    var writeOutput = true;
    var serveAssets = createUiAssetsMiddleware(assetsRoot);
    return [
        {
            name: "ui-assets-serve",
            enforce: "pre",
            configureServer: function (server) {
                server.middlewares.use(serveAssets);
            },
            configurePreviewServer: function (server) {
                server.middlewares.use(serveAssets);
            },
        },
        {
            name: "ui-assets-build",
            apply: "build",
            enforce: "pre",
            configResolved: function (config) {
                outDir = (0, node_path_1.resolve)(config.root, config.build.outDir);
                writeOutput = config.build.write !== false;
            },
            closeBundle: function () {
                if (!writeOutput)
                    return;
                if (!(0, node_fs_1.existsSync)(assetsRoot)) {
                    return;
                }
                var dest = (0, node_path_1.resolve)(outDir, ____ts_4.SEMIO_ASSET_DIRECTORY);
                (0, node_fs_1.mkdirSync)(outDir, { recursive: true });
                (0, node_fs_1.cpSync)(assetsRoot, dest, { recursive: true });
            },
        },
    ];
}
/** 🌐️ Serves and copies shared fonts and cursors at `/🖼️assets/*`. */
function semioAssetsVitePlugin(repoRoot) {
    return uiAssetsVitePluginsForRoot(resolveSemioAssetRoot(repoRoot));
}
/** @emoji 🌐️ @deprecated Use {@link semioAssetsVitePlugin} — caller-supplied roots caused silent font 404s. */
function uiAssetsVitePlugin(assetsRoot) {
    var fontDir = (0, node_path_1.resolve)(assetsRoot, "🔤️fonts");
    if (!(0, node_fs_1.existsSync)(assetsRoot) || !(0, node_fs_1.existsSync)(fontDir)) {
        throw new Error("uiAssetsVitePlugin: invalid asset root ".concat(assetsRoot, " (missing \uD83D\uDD24\uFE0Ffonts); use semioAssetsVitePlugin(repoRoot)"));
    }
    return uiAssetsVitePluginsForRoot(assetsRoot);
}
function namedImportSpecifiersForModule(source, moduleId) {
    var _a;
    var escaped = moduleId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    var re = new RegExp("import\\s*\\{([^}]+)\\}\\s*from\\s*[\"']".concat(escaped, "[\"']"), "gs");
    var names = [];
    var match;
    while ((match = re.exec(source))) {
        for (var _i = 0, _b = match[1].split(","); _i < _b.length; _i++) {
            var part = _b[_i];
            var trimmed = part.trim();
            if (!trimmed)
                continue;
            var name_1 = (_a = trimmed
                .replace(/^type\s+/, "")
                .split(/\s+as\s+/)[0]) === null || _a === void 0 ? void 0 : _a.trim();
            if (name_1)
                names.push(name_1);
        }
    }
    return names;
}
/** @emoji 🔁️ Named import specifiers duplicated within the same module import block(s). */
function duplicateNamedImportsForModule(source, moduleId) {
    var names = namedImportSpecifiersForModule(source, moduleId);
    var seen = new Set();
    var dupes = [];
    for (var _i = 0, names_1 = names; _i < names_1.length; _i++) {
        var name_2 = names_1[_i];
        if (seen.has(name_2))
            dupes.push(name_2);
        else
            seen.add(name_2);
    }
    return dupes;
}
var PRESENTATION_RENDERER_VITEST_START = "//#region 🧪️Tests";
/** @emoji ✂️ Drops vitest regions from animate present renderer in browser dev. */
function animatePresentRendererVitestStripPlugin(animatePresentIndexPath) {
    return {
        name: "animate-present-renderer-vitest-strip",
        enforce: "pre",
        load: function (id) {
            if (process.env.VITEST)
                return;
            var filePath = id.split("?")[0];
            if (filePath !== animatePresentIndexPath)
                return;
            var source = (0, node_fs_1.readFileSync)(animatePresentIndexPath, "utf8");
            var testsStart = source.indexOf(PRESENTATION_RENDERER_VITEST_START);
            if (testsStart < 0)
                return source;
            return source.slice(0, testsStart);
        },
    };
}
/** @deprecated Use {@link animatePresentRendererVitestStripPlugin}. */
exports.presentationRendererVitestStripPlugin = animatePresentRendererVitestStripPlugin;
/** @emoji 🎬️ R3F packages that must resolve once with {@link sceneHostPort} and drei controls. */
exports.PLAYGROUND_SCENE_HOST_DEDUPE = ["@react-three/fiber", "@react-three/drei"];
/** @emoji 🎬️ Vite aliases that pin R3F to a single node_modules entry (avoids duplicate Canvas stores). */
function playgroundSceneHostResolveAliases(repoRoot) {
    return [
        { find: /^@react-three\/fiber$/, replacement: (0, node_path_1.resolve)(repoRoot, "node_modules/@react-three/fiber/dist/react-three-fiber.esm.js") },
        { find: /^@react-three\/drei$/, replacement: (0, node_path_1.resolve)(repoRoot, "node_modules/@react-three/drei/index.js") },
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
exports.PLAYGROUND_SCENE_HOST_CJS_INCLUDE = ["scheduler", "stats.js", "use-sync-external-store/shim/index.js", "use-sync-external-store/shim/with-selector.js"];
/** @emoji 🎬️ `optimizeDeps` preset for configs that use {@link playgroundSceneHostResolveAliases}: never prebundle R3F — a `.vite/deps` fiber copy and the aliased ESM entry are two Canvas stores, and drei's `PerspectiveCamera` then throws outside Canvas — but DO prebundle the CJS shims R3F's excluded graph imports ({@link PLAYGROUND_SCENE_HOST_CJS_INCLUDE}). */
function playgroundSceneHostOptimizeDeps(extra) {
    var _a, _b;
    var include = __spreadArray(__spreadArray(["three"], exports.PLAYGROUND_SCENE_HOST_CJS_INCLUDE, true), ((_a = extra === null || extra === void 0 ? void 0 : extra.include) !== null && _a !== void 0 ? _a : []), true).filter(function (id) { return !exports.PLAYGROUND_SCENE_HOST_DEDUPE.includes(id); });
    var exclude = __spreadArray(__spreadArray([], exports.PLAYGROUND_SCENE_HOST_DEDUPE, true), ((_b = extra === null || extra === void 0 ? void 0 : extra.exclude) !== null && _b !== void 0 ? _b : []), true);
    return { include: __spreadArray([], new Set(include), true), exclude: __spreadArray([], new Set(exclude), true) };
}
//#region 🔖️MapTileCache
/** @emoji 🗺️ Compliant User-Agent for OSM / MapLibre demotiles in map play. */
exports.GIS_MAP_TILE_USER_AGENT = "ComposeGisMapPlay/0.1 (+https://github.com/usalu/semio; dev playground)";
/** @emoji 🗺️ Default dev prefetch bounds (Switzerland) for GIS map play. */
exports.GIS_MAP_DEFAULT_PREFETCH_BOUNDS = {
    west: 5.95,
    south: 45.82,
    east: 10.52,
    north: 47.81,
};
exports.GIS_MAP_OSM_TILE_MAX_Z = 19;
/** @emoji 🗺️ OpenFreeMap / OpenMapTiles planet MVT (OSM); matches raster detail up to z14. */
exports.GIS_MAP_VECTOR_TILE_MAX_Z = 14;
exports.GIS_MAP_OPENFREEMAP_TILEJSON = "https://tiles.openfreemap.org/planet";
/** @emoji 🗺️ Highest zoom prefetched for offline map play (matches `GIS_MAP_LOD_TILE_Z` building band). */
exports.GIS_MAP_PREFETCH_RASTER_Z_MAX = 13;
exports.GIS_MAP_TILE_SERVE_MODE_ENV = "GIS_MAP_TILE_SERVE_MODE";
function resolveGisMapTileServeMode(value) {
    return value === "bundle" ? "bundle" : "fetch";
}
function mapTileCacheRoots(repoRoot) {
    return {
        osm: (0, node_path_1.resolve)(repoRoot, ".🧬semio/🗺️map", "osm-tiles"),
        vt: (0, node_path_1.resolve)(repoRoot, ".🧬semio/🗺️map", "openfreemap-vt"),
    };
}
/** @emoji 🧭️ Web Mercator tile index for a lon/lat at zoom `z`. */
function lonLatToTileXY(lon, lat, z) {
    var n = Math.pow(2, z);
    var x = Math.floor(((lon + 180) / 360) * n);
    var latRad = (lat * Math.PI) / 180;
    var y = Math.floor(((1 - Math.log(Math.tan(latRad) + 1 / Math.cos(latRad)) / Math.PI) / 2) * n);
    return { x: Math.max(0, Math.min(n - 1, x)), y: Math.max(0, Math.min(n - 1, y)) };
}
/** @emoji 📐️ Inclusive OSM tile index range covering `bounds` at zoom `z`. */
function tileRangeForBounds(bounds, z) {
    var sw = lonLatToTileXY(bounds.west, bounds.south, z);
    var ne = lonLatToTileXY(bounds.east, bounds.north, z);
    return {
        x0: Math.min(sw.x, ne.x),
        x1: Math.max(sw.x, ne.x),
        y0: Math.min(sw.y, ne.y),
        y1: Math.max(sw.y, ne.y),
    };
}
/** @emoji 📋️ Lists every tile in `bounds` for zoom levels `zMin`…`zMax` (inclusive). */
function listMapTilesForBounds(bounds, zMin, zMax) {
    var lo = Math.max(0, Math.min(zMin, zMax));
    var hi = Math.max(lo, zMax);
    var out = [];
    for (var z = lo; z <= hi; z++) {
        var _a = tileRangeForBounds(bounds, z), x0 = _a.x0, x1 = _a.x1, y0 = _a.y0, y1 = _a.y1;
        for (var x = x0; x <= x1; x++) {
            for (var y = y0; y <= y1; y++) {
                out.push({ z: z, x: x, y: y });
            }
        }
    }
    return out;
}
function fetchOsmTileToCache(cacheRoot, z, x, y) {
    return __awaiter(this, void 0, void 0, function () {
        var rel, filePath, relToRoot, upstream, _a, _b, _c, _d;
        return __generator(this, function (_e) {
            switch (_e.label) {
                case 0:
                    rel = "".concat(z, "/").concat(x, "/").concat(y, ".png");
                    filePath = (0, node_path_1.resolve)(cacheRoot, rel);
                    relToRoot = (0, node_path_1.relative)(cacheRoot, filePath);
                    if (relToRoot.startsWith("..") || (0, node_path_1.isAbsolute)(relToRoot)) {
                        return [2 /*return*/, false];
                    }
                    return [4 /*yield*/, (0, promises_1.mkdir)((0, node_path_1.resolve)(filePath, ".."), { recursive: true })];
                case 1:
                    _e.sent();
                    return [4 /*yield*/, fetch("https://tile.openstreetmap.org/".concat(z, "/").concat(x, "/").concat(y, ".png"), {
                            headers: { "User-Agent": exports.GIS_MAP_TILE_USER_AGENT },
                        })];
                case 2:
                    upstream = _e.sent();
                    if (!upstream.ok) {
                        return [2 /*return*/, false];
                    }
                    _a = promises_1.writeFile;
                    _b = [filePath];
                    _d = (_c = Buffer).from;
                    return [4 /*yield*/, upstream.arrayBuffer()];
                case 3: return [4 /*yield*/, _a.apply(void 0, _b.concat([_d.apply(_c, [_e.sent()])]))];
                case 4:
                    _e.sent();
                    return [2 /*return*/, true];
            }
        });
    });
}
var openFreeMapTileTemplate = (0, framework_1.ephemeralBox)("framework.modules.ui.styling.packages.rust.vite.elements.assets.ts.openFreeMapTileTemplate", null);
var openFreeMapTileTemplateAt = (0, framework_1.ephemeralBox)("framework.modules.ui.styling.packages.rust.vite.elements.assets.ts.openFreeMapTileTemplateAt", 0);
var OPENFREEMAP_TILE_TEMPLATE_TTL_MS = 7 * 24 * 60 * 60 * 1000;
function resolveOpenFreeMapTileTemplate() {
    return __awaiter(this, void 0, void 0, function () {
        var now, res, json, template;
        var _a;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    now = Date.now();
                    if (openFreeMapTileTemplate.current && now - openFreeMapTileTemplateAt.current < OPENFREEMAP_TILE_TEMPLATE_TTL_MS) {
                        return [2 /*return*/, openFreeMapTileTemplate.current];
                    }
                    return [4 /*yield*/, fetch(exports.GIS_MAP_OPENFREEMAP_TILEJSON, { headers: { "User-Agent": exports.GIS_MAP_TILE_USER_AGENT } })];
                case 1:
                    res = _b.sent();
                    if (!res.ok) {
                        throw new Error("OpenFreeMap TileJSON failed: ".concat(res.status));
                    }
                    return [4 /*yield*/, res.json()];
                case 2:
                    json = (_b.sent());
                    template = (_a = json.tiles) === null || _a === void 0 ? void 0 : _a[0];
                    if (typeof template !== "string" || !template.includes("{z}")) {
                        throw new Error("OpenFreeMap TileJSON missing tiles URL template");
                    }
                    openFreeMapTileTemplate.current = template;
                    openFreeMapTileTemplateAt.current = now;
                    return [2 /*return*/, template];
            }
        });
    });
}
function fetchVtTileToCache(cacheRoot, z, x, y) {
    return __awaiter(this, void 0, void 0, function () {
        var rel, filePath, relToRoot, template, url, upstream, buf, _a, _b;
        return __generator(this, function (_c) {
            switch (_c.label) {
                case 0:
                    rel = "".concat(z, "/").concat(x, "/").concat(y, ".pbf");
                    filePath = (0, node_path_1.resolve)(cacheRoot, rel);
                    relToRoot = (0, node_path_1.relative)(cacheRoot, filePath);
                    if (relToRoot.startsWith("..") || (0, node_path_1.isAbsolute)(relToRoot)) {
                        return [2 /*return*/, false];
                    }
                    return [4 /*yield*/, (0, promises_1.mkdir)((0, node_path_1.resolve)(filePath, ".."), { recursive: true })];
                case 1:
                    _c.sent();
                    return [4 /*yield*/, resolveOpenFreeMapTileTemplate()];
                case 2:
                    template = _c.sent();
                    url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
                    return [4 /*yield*/, fetch(url, { headers: { "User-Agent": exports.GIS_MAP_TILE_USER_AGENT } })];
                case 3:
                    upstream = _c.sent();
                    if (!upstream.ok) {
                        return [2 /*return*/, false];
                    }
                    _b = (_a = Buffer).from;
                    return [4 /*yield*/, upstream.arrayBuffer()];
                case 4:
                    buf = _b.apply(_a, [_c.sent()]);
                    if (buf.length === 0) {
                        return [2 /*return*/, false];
                    }
                    return [4 /*yield*/, (0, promises_1.writeFile)(filePath, buf)];
                case 5:
                    _c.sent();
                    return [2 /*return*/, true];
            }
        });
    });
}
/** @emoji ⬇️ Prefetch OSM PNG and MapLibre MVT tiles into `.🧬semio/🗺️map` for offline map play. */
function prefetchMapTiles(options) {
    return __awaiter(this, void 0, void 0, function () {
        var repoRoot, _a, bounds, _b, raster, _c, vector, _d, zMinRaster, _e, zMaxRaster, _f, zMinVector, _g, zMaxVector, _h, concurrency, _j, skipExisting, _k, delayMs, _l, log, _m, osm, vt, jobs, _i, _o, _p, z, x, y, _q, _r, _s, z, x, y, zoomLabel, skipped, pending, downloaded, failed, sleep, i, batch;
        var _this = this;
        return __generator(this, function (_t) {
            switch (_t.label) {
                case 0:
                    repoRoot = options.repoRoot, _a = options.bounds, bounds = _a === void 0 ? exports.GIS_MAP_DEFAULT_PREFETCH_BOUNDS : _a, _b = options.raster, raster = _b === void 0 ? true : _b, _c = options.vector, vector = _c === void 0 ? true : _c, _d = options.zMinRaster, zMinRaster = _d === void 0 ? 0 : _d, _e = options.zMaxRaster, zMaxRaster = _e === void 0 ? exports.GIS_MAP_PREFETCH_RASTER_Z_MAX : _e, _f = options.zMinVector, zMinVector = _f === void 0 ? 0 : _f, _g = options.zMaxVector, zMaxVector = _g === void 0 ? exports.GIS_MAP_VECTOR_TILE_MAX_Z : _g, _h = options.concurrency, concurrency = _h === void 0 ? 4 : _h, _j = options.skipExisting, skipExisting = _j === void 0 ? true : _j, _k = options.delayMs, delayMs = _k === void 0 ? 120 : _k, _l = options.log, log = _l === void 0 ? function (line) { return console.log(line); } : _l;
                    _m = mapTileCacheRoots(repoRoot), osm = _m.osm, vt = _m.vt;
                    jobs = [];
                    if (raster) {
                        for (_i = 0, _o = listMapTilesForBounds(bounds, zMinRaster, Math.min(zMaxRaster, exports.GIS_MAP_OSM_TILE_MAX_Z)); _i < _o.length; _i++) {
                            _p = _o[_i], z = _p.z, x = _p.x, y = _p.y;
                            jobs.push({ kind: "osm", z: z, x: x, y: y });
                        }
                    }
                    if (vector) {
                        for (_q = 0, _r = listMapTilesForBounds(bounds, zMinVector, Math.min(zMaxVector, exports.GIS_MAP_VECTOR_TILE_MAX_Z)); _q < _r.length; _q++) {
                            _s = _r[_q], z = _s.z, x = _s.x, y = _s.y;
                            jobs.push({ kind: "vt", z: z, x: x, y: y });
                        }
                    }
                    zoomLabel = "(raster z".concat(zMinRaster, "-").concat(zMaxRaster, ", vector z").concat(zMinVector, "-").concat(zMaxVector, ")");
                    skipped = 0;
                    pending = skipExisting
                        ? jobs.filter(function (job) {
                            var cacheRoot = job.kind === "osm" ? osm : vt;
                            var ext = job.kind === "osm" ? "png" : "pbf";
                            var filePath = (0, node_path_1.resolve)(cacheRoot, "".concat(job.z, "/").concat(job.x, "/").concat(job.y, ".").concat(ext));
                            if ((0, node_fs_1.existsSync)(filePath)) {
                                skipped++;
                                return false;
                            }
                            return true;
                        })
                        : jobs;
                    log("[gis/2d/play] prefetch ".concat(jobs.length, " tiles ").concat(zoomLabel) + (skipExisting ? " (".concat(skipped, " cached, ").concat(pending.length, " to fetch)") : ""));
                    if (pending.length === 0) {
                        log("[gis/2d/play] prefetch done: downloaded=0 skipped=".concat(skipped, " failed=0"));
                        return [2 /*return*/, { downloaded: 0, skipped: skipped, failed: 0 }];
                    }
                    downloaded = 0;
                    failed = 0;
                    sleep = function (ms) { return new Promise(function (r) { return setTimeout(r, ms); }); };
                    i = 0;
                    _t.label = 1;
                case 1:
                    if (!(i < pending.length)) return [3 /*break*/, 5];
                    batch = pending.slice(i, i + concurrency);
                    return [4 /*yield*/, Promise.all(batch.map(function (job) { return __awaiter(_this, void 0, void 0, function () {
                            var cacheRoot, ok, _a;
                            return __generator(this, function (_b) {
                                switch (_b.label) {
                                    case 0:
                                        cacheRoot = job.kind === "osm" ? osm : vt;
                                        if (!(job.kind === "osm")) return [3 /*break*/, 2];
                                        return [4 /*yield*/, fetchOsmTileToCache(cacheRoot, job.z, job.x, job.y)];
                                    case 1:
                                        _a = _b.sent();
                                        return [3 /*break*/, 4];
                                    case 2: return [4 /*yield*/, fetchVtTileToCache(cacheRoot, job.z, job.x, job.y)];
                                    case 3:
                                        _a = _b.sent();
                                        _b.label = 4;
                                    case 4:
                                        ok = _a;
                                        if (ok) {
                                            downloaded++;
                                        }
                                        else {
                                            failed++;
                                        }
                                        return [2 /*return*/];
                                }
                            });
                        }); }))];
                case 2:
                    _t.sent();
                    if (!(delayMs > 0 && i + concurrency < pending.length)) return [3 /*break*/, 4];
                    return [4 /*yield*/, sleep(delayMs)];
                case 3:
                    _t.sent();
                    _t.label = 4;
                case 4:
                    i += concurrency;
                    return [3 /*break*/, 1];
                case 5:
                    log("[gis/2d/play] prefetch done: downloaded=".concat(downloaded, " skipped=").concat(skipped, " failed=").concat(failed));
                    return [2 /*return*/, { downloaded: downloaded, skipped: skipped, failed: failed }];
            }
        });
    });
}
//#endregion 🔖️MapTileCache
//#region 🔖️TileProxyAssetPlugin
/** @emoji 🧩️ Extension implied by a resolved tile URL template's tail (`.png`, `.pbf`, …), `"bin"` if absent. */
function tileProxyExtFromTemplate(template) {
    var _a;
    var clean = (_a = template.split(/[?#]/, 1)[0]) !== null && _a !== void 0 ? _a : template;
    var ext = clean.split(".").pop();
    return ext && ext.length <= 4 ? ext : "bin";
}
function contentTypeForTileExt(ext) {
    if (ext === "png")
        return "image/png";
    if (ext === "pbf" || ext === "mvt")
        return "application/x-protobuf";
    return "application/octet-stream";
}
var tileProxyTemplateCache = (0, framework_1.ephemeralMap)("framework.modules.ui.styling.packages.rust.vite.elements.assets.ts.tileProxyTemplateCache");
var TILE_PROXY_TEMPLATE_TTL_MS = 7 * 24 * 60 * 60 * 1000;
/** @emoji 🧭️ Resolves a `tile-proxy` spec's `upstream` to a concrete `{z}/{x}/{y}` URL template: used
 * directly when it already contains `{z}`, otherwise treated as a TileJSON endpoint and resolved
 * (cached, 7-day TTL) — generalizes the previous OpenFreeMap-only MVT template resolution so any
 * TileJSON-backed upstream (not just OpenFreeMap) works the same way. */
function resolveTileProxyUrlTemplate(upstream) {
    return __awaiter(this, void 0, void 0, function () {
        var now, cached, res, json, template;
        var _a;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    if (upstream.includes("{z}")) {
                        return [2 /*return*/, upstream];
                    }
                    now = Date.now();
                    cached = tileProxyTemplateCache.get(upstream);
                    if (cached && now - cached.at < TILE_PROXY_TEMPLATE_TTL_MS) {
                        return [2 /*return*/, cached.template];
                    }
                    return [4 /*yield*/, fetch(upstream, { headers: { "User-Agent": exports.GIS_MAP_TILE_USER_AGENT } })];
                case 1:
                    res = _b.sent();
                    if (!res.ok) {
                        throw new Error("tile proxy upstream TileJSON failed: ".concat(res.status));
                    }
                    return [4 /*yield*/, res.json()];
                case 2:
                    json = (_b.sent());
                    template = (_a = json.tiles) === null || _a === void 0 ? void 0 : _a[0];
                    if (typeof template !== "string" || !template.includes("{z}")) {
                        throw new Error("tile proxy TileJSON missing tiles URL template");
                    }
                    tileProxyTemplateCache.set(upstream, { template: template, at: now });
                    return [2 /*return*/, template];
            }
        });
    });
}
function fetchTileProxyTileToCache(cacheRoot, upstream, z, x, y) {
    return __awaiter(this, void 0, void 0, function () {
        var template, ext, filePath, relToRoot, url, upstreamRes, buf, _a, _b;
        return __generator(this, function (_c) {
            switch (_c.label) {
                case 0: return [4 /*yield*/, resolveTileProxyUrlTemplate(upstream)];
                case 1:
                    template = _c.sent();
                    ext = tileProxyExtFromTemplate(template);
                    filePath = (0, node_path_1.resolve)(cacheRoot, "".concat(z, "/").concat(x, "/").concat(y, ".").concat(ext));
                    relToRoot = (0, node_path_1.relative)(cacheRoot, filePath);
                    if (relToRoot.startsWith("..") || (0, node_path_1.isAbsolute)(relToRoot)) {
                        return [2 /*return*/, { ok: false, ext: ext }];
                    }
                    return [4 /*yield*/, (0, promises_1.mkdir)((0, node_path_1.resolve)(filePath, ".."), { recursive: true })];
                case 2:
                    _c.sent();
                    url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
                    return [4 /*yield*/, fetch(url, { headers: { "User-Agent": exports.GIS_MAP_TILE_USER_AGENT } })];
                case 3:
                    upstreamRes = _c.sent();
                    if (!upstreamRes.ok) {
                        return [2 /*return*/, { ok: false, ext: ext }];
                    }
                    _b = (_a = Buffer).from;
                    return [4 /*yield*/, upstreamRes.arrayBuffer()];
                case 4:
                    buf = _b.apply(_a, [_c.sent()]);
                    if (buf.length === 0) {
                        return [2 /*return*/, { ok: false, ext: ext }];
                    }
                    return [4 /*yield*/, (0, promises_1.writeFile)(filePath, buf)];
                case 5:
                    _c.sent();
                    return [2 /*return*/, { ok: true, ext: ext }];
            }
        });
    });
}
/** @emoji 🌐️ Connect middleware serving `{route}/{z}/{x}/{y}.{ext}` tiles from `cacheRoot`, fetching
 * (and caching) from `upstream` on a miss — generalizes the previous OSM/OpenFreeMap/Terrarium
 * middlewares into one route-driven implementation. */
function createTileProxyMiddleware(route, cacheRoot, upstream, mode) {
    var _this = this;
    var prefix = route.endsWith("/") ? route : "".concat(route, "/");
    var pattern = new RegExp("^".concat(prefix.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"), "(\\d+)/(\\d+)/(\\d+)\\.(\\w+)(?:\\?.*)?$"));
    return function (req, res, next) { return __awaiter(_this, void 0, void 0, function () {
        var match, _a, zs, xs, ys, ext, z, x, y, filePath, relToRoot, result, _b;
        var _c;
        return __generator(this, function (_d) {
            switch (_d.label) {
                case 0:
                    match = (_c = req.url) === null || _c === void 0 ? void 0 : _c.match(pattern);
                    if (!match) {
                        next();
                        return [2 /*return*/];
                    }
                    _a = match, zs = _a[1], xs = _a[2], ys = _a[3], ext = _a[4];
                    z = Number(zs);
                    x = Number(xs);
                    y = Number(ys);
                    filePath = (0, node_path_1.resolve)(cacheRoot, "".concat(z, "/").concat(x, "/").concat(y, ".").concat(ext));
                    relToRoot = (0, node_path_1.relative)(cacheRoot, filePath);
                    if (relToRoot.startsWith("..") || (0, node_path_1.isAbsolute)(relToRoot)) {
                        next();
                        return [2 /*return*/];
                    }
                    if ((0, node_fs_1.existsSync)(filePath)) {
                        res.setHeader("Content-Type", contentTypeForTileExt(ext));
                        (0, node_fs_1.createReadStream)(filePath).pipe(res);
                        return [2 /*return*/];
                    }
                    if (mode === "bundle") {
                        res.statusCode = 404;
                        res.end();
                        return [2 /*return*/];
                    }
                    _d.label = 1;
                case 1:
                    _d.trys.push([1, 3, , 4]);
                    return [4 /*yield*/, fetchTileProxyTileToCache(cacheRoot, upstream, z, x, y)];
                case 2:
                    result = _d.sent();
                    if (!result.ok) {
                        res.statusCode = 404;
                        res.end();
                        return [2 /*return*/];
                    }
                    res.setHeader("Content-Type", contentTypeForTileExt(result.ext));
                    (0, node_fs_1.createReadStream)(filePath).pipe(res);
                    return [3 /*break*/, 4];
                case 3:
                    _b = _d.sent();
                    res.statusCode = 502;
                    res.end();
                    return [3 /*break*/, 4];
                case 4: return [2 /*return*/];
            }
        });
    }); };
}
/** @emoji 🌐️ Generic dev/preview/build Vite plugin pair for one `tile-proxy` asset spec — replaces the
 * previous `gisMapTilesVitePlugins`/`terrainTilesVitePlugins`/`osmTileProxyVitePlugin`/
 * `mapLibreVectorTileProxyVitePlugin` quartet with a single spec-driven implementation. */
function tileProxyVitePlugin(repoRoot, spec, mode) {
    if (mode === void 0) { mode = "fetch"; }
    var cacheRoot = (0, node_path_1.resolve)(repoRoot, ".🧬semio/🗺️map", spec.cache);
    var serveTiles = createTileProxyMiddleware(spec.route, cacheRoot, spec.upstream, mode);
    var outDir = (0, node_path_1.resolve)(process.cwd(), "dist");
    var writeOutput = true;
    var plugins = [
        {
            name: "tile-proxy-serve".concat(spec.route),
            enforce: "pre",
            configureServer: function (server) {
                server.middlewares.use(serveTiles);
            },
            configurePreviewServer: function (server) {
                server.middlewares.use(serveTiles);
            },
        },
    ];
    if (mode === "bundle") {
        plugins.push({
            name: "tile-proxy-build".concat(spec.route),
            apply: "build",
            enforce: "pre",
            configResolved: function (config) {
                outDir = (0, node_path_1.resolve)(config.root, config.build.outDir);
                writeOutput = config.build.write !== false;
            },
            closeBundle: function () {
                if (!writeOutput)
                    return;
                var dist = outDir;
                (0, node_fs_1.mkdirSync)(dist, { recursive: true });
                if ((0, node_fs_1.existsSync)(cacheRoot)) {
                    (0, node_fs_1.cpSync)(cacheRoot, (0, node_path_1.resolve)(dist, spec.route.replace(/^\//, "")), { recursive: true });
                }
            },
        });
    }
    return plugins;
}
/** @emoji 🌐️ Standalone HTTP server for every declared playground asset kind (tile-proxy, mesh-collection,
 * static-dir) — wgpu Trunk proxies and native-bin `SEMIO_ASSET_BASE_URL` hit this instead of Vite. */
function startAssetServer(repoRoot, port, specs, mode, host) {
    if (mode === void 0) { mode = "fetch"; }
    if (host === void 0) { host = "127.0.0.1"; }
    var seen = new Set();
    var middlewares = [];
    for (var _i = 0, specs_1 = specs; _i < specs_1.length; _i++) {
        var spec = specs_1[_i];
        var key = "".concat(spec.kind, ":").concat(spec.route);
        if (seen.has(key))
            continue;
        seen.add(key);
        if (spec.kind === "tile-proxy") {
            middlewares.push(createTileProxyMiddleware(spec.route, (0, node_path_1.resolve)(repoRoot, ".🧬semio/🗺️map", spec.cache), spec.upstream, mode));
        }
        else if (spec.kind === "mesh-collection") {
            middlewares.push(createMeshCollectionMiddleware(repoRoot, spec));
        }
        else {
            middlewares.push(createStaticDirMiddleware(repoRoot, spec));
        }
    }
    var server = (0, node_http_1.createServer)(function (req, res) {
        var run = function (i) {
            if (i >= middlewares.length) {
                res.statusCode = 404;
                res.end();
                return;
            }
            middlewares[i](req, res, function () { return run(i + 1); });
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
function playgroundAssetVitePlugins(repoRoot, specs, mode) {
    if (mode === void 0) { mode = "fetch"; }
    var seen = new Set();
    var plugins = [];
    for (var _i = 0, specs_2 = specs; _i < specs_2.length; _i++) {
        var spec = specs_2[_i];
        var key = "".concat(spec.kind, ":").concat(spec.route);
        if (seen.has(key)) {
            continue;
        }
        seen.add(key);
        if (spec.kind === "tile-proxy") {
            plugins.push.apply(plugins, tileProxyVitePlugin(repoRoot, spec, mode));
        }
        else if (spec.kind === "static-dir") {
            plugins.push.apply(plugins, staticDirVitePlugin(repoRoot, spec));
        }
        else {
            plugins.push.apply(plugins, meshCollectionVitePlugin(repoRoot, spec));
        }
    }
    return plugins;
}
//#endregion 🔖️PlaygroundAssetVitePlugins
/** @emoji 🦀️ Vite `optimizeDeps.exclude` entries for wasm-bindgen flow modules (must not be prebundled). */
exports.FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE = [
    "@semio-tech/flow-module-core",
    "@semio-tech/flow-module-math",
    "@semio-tech/flow-module-text",
    "@semio-tech/flow-module-logic",
    "@semio-tech/flow-module-dictionary",
    "@semio-tech/flow-module-list",
    "@semio-tech/flow-module-draw",
];
/** @emoji 🧭️ Workspace Vite resolve preset: dedupe, fs.allow, optimizeDeps.exclude, scene-host aliases. */
function createWorkspaceViteResolveConfig(repoRoot, extraAliases) {
    if (extraAliases === void 0) { extraAliases = []; }
    return {
        resolve: {
            alias: __spreadArray([], extraAliases, true),
            dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
        },
        server: {
            fs: { allow: [repoRoot] },
        },
        optimizeDeps: {
            exclude: __spreadArray(__spreadArray([], findWorkspacePackages(repoRoot), true), exports.FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE, true),
        },
    };
}
//#region 🔖️StaticDirAssetPlugin
function contentTypeForStaticDirAsset(filePath) {
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
function createStaticDirMiddleware(repoRoot, spec) {
    var fixtureRoot = (0, node_path_1.resolve)(repoRoot, spec.root);
    var route = spec.route.endsWith("/") ? spec.route : "".concat(spec.route, "/");
    return function (req, res, next) {
        var _a, _b;
        var rawUrl = (_a = req.url) !== null && _a !== void 0 ? _a : "";
        var pathOnly = (_b = rawUrl.split(/[?#]/, 1)[0]) !== null && _b !== void 0 ? _b : "";
        var decodedPath = pathOnly;
        try {
            decodedPath = decodeURIComponent(pathOnly);
        }
        catch (_c) {
            next();
            return;
        }
        if (!decodedPath.startsWith(route)) {
            next();
            return;
        }
        var rel = decodedPath.slice(route.length);
        var filePath = (0, node_path_1.resolve)(fixtureRoot, rel);
        var relToRoot = (0, node_path_1.relative)(fixtureRoot, filePath);
        if (relToRoot.startsWith("..") || (0, node_path_1.isAbsolute)(relToRoot)) {
            next();
            return;
        }
        if (!(0, node_fs_1.existsSync)(filePath) || !(0, node_fs_1.statSync)(filePath).isFile()) {
            res.statusCode = 404;
            res.end();
            return;
        }
        var contentType = contentTypeForStaticDirAsset(filePath);
        if (contentType) {
            res.setHeader("Content-Type", contentType);
        }
        (0, node_fs_1.createReadStream)(filePath).pipe(res);
    };
}
/** @emoji 🖼️ Generic dev/build Vite plugin pair for one `static-dir` asset spec: serves and copies
 * `spec.root` at `spec.route` — replaces the previous `cadFixtureVitePlugin`/`infiniteFixtureVitePlugin`
 * pair (byte-identical serving logic, now route/root-driven instead of hardcoded per fixture tree). */
function staticDirVitePlugin(repoRoot, spec) {
    var serveFixture = createStaticDirMiddleware(repoRoot, spec);
    var fixtureRoot = (0, node_path_1.resolve)(repoRoot, spec.root);
    var destName = spec.route.replace(/^\//, "");
    var outDir = (0, node_path_1.resolve)(process.cwd(), "dist");
    var writeOutput = true;
    return [
        {
            name: "static-dir-serve".concat(spec.route),
            enforce: "pre",
            configureServer: function (server) {
                server.middlewares.use(serveFixture);
            },
            configurePreviewServer: function (server) {
                server.middlewares.use(serveFixture);
            },
        },
        {
            name: "static-dir-build".concat(spec.route),
            apply: "build",
            enforce: "pre",
            configResolved: function (config) {
                // 🖼️ `config.build.outDir` is root-relative unless already absolute — `resolve` handles both, so a
                // brand's custom `outDir` (see `ShellBrand.distDir`) is honored instead of assuming `<root>/dist`.
                outDir = (0, node_path_1.resolve)(config.root, config.build.outDir);
                writeOutput = config.build.write !== false;
            },
            closeBundle: function () {
                if (!writeOutput)
                    return;
                if (!(0, node_fs_1.existsSync)(fixtureRoot)) {
                    return;
                }
                var dest = (0, node_path_1.resolve)(outDir, destName);
                (0, node_fs_1.mkdirSync)(outDir, { recursive: true });
                if ((0, node_fs_1.existsSync)(dest))
                    (0, node_fs_1.rmSync)(dest, { recursive: true, force: true });
                (0, node_fs_1.cpSync)(fixtureRoot, dest, { recursive: true });
            },
        },
    ];
}
/** @emoji 🌐️ Reference-plane assets every `*-play` static bundle serves unconditionally. */
exports.PLAYGROUND_PLAY_STATIC_ASSETS = [
    { kind: "static-dir", route: "/infinite-assets", root: "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets" },
];
//#endregion 🔖️StaticDirAssetPlugin
function findWorkspacePackages(repoRoot) {
    var packages = [];
    var scan = function (dir) {
        var entries;
        try {
            entries = (0, node_fs_1.readdirSync)(dir);
        }
        catch (_a) {
            return;
        }
        for (var _i = 0, entries_1 = entries; _i < entries_1.length; _i++) {
            var entry = entries_1[_i];
            if (entry === "node_modules" || entry === "dist" || entry === "target" || entry === "storybook-static" || entry.startsWith("."))
                continue;
            var full = (0, node_path_1.resolve)(dir, entry);
            try {
                var stat = (0, node_fs_1.statSync)(full);
                if (stat.isDirectory()) {
                    scan(full);
                }
                else if (entry === "package.json" && full !== (0, node_path_1.resolve)(repoRoot, "package.json")) {
                    var pkg = JSON.parse((0, node_fs_1.readFileSync)(full, "utf8"));
                    if (pkg.name && typeof pkg.name === "string" && pkg.name.startsWith("@semio-tech/")) {
                        packages.push(pkg.name);
                    }
                }
            }
            catch (_b) {
                /* ignore statSync or readFileSync errors (e.g. broken symlinks or unreadable files) */
            }
        }
    };
    scan(repoRoot);
    return packages;
}
/** @emoji 🛝️ `defineConfig` for `@puzzle/*-play` Vite entries with consistent renderer and core aliases. */
function createPlaygroundPlayViteConfig(options) {
    var _a, _b, _c, _d, _e;
    var playDir = options.playDir, repoRoot = options.repoRoot, playEntryKind = options.playEntryKind, _f = options.extraAliases, extraAliases = _f === void 0 ? [] : _f, _g = options.extraPlugins, extraPlugins = _g === void 0 ? [] : _g, watchIgnored = options.watchIgnored, build = options.build, server = options.server, optimizeDeps = options.optimizeDeps, resolveDedupe = options.resolveDedupe;
    var osHubAliases = playEntryKind === "s"
        ? [
            {
                find: "@semio-tech/graph-dsl-core",
                replacement: (0, node_path_1.resolve)(repoRoot, "mathematical/graph/dsl/core/js/index.ts"),
            },
        ]
        : [];
    var workspaceResolve = createWorkspaceViteResolveConfig(repoRoot, __spreadArray(__spreadArray([], extraAliases, true), osHubAliases, true));
    var workerStubPlugins = [playgroundPlaywrightDevStubPlugin(), playgroundVitestDevStubPlugin()];
    return (0, ____ts_1.defineOwnedBuildConfig)({
        root: playDir,
        base: "./",
        publicDir: (0, node_path_1.resolve)(playDir, "public"),
        assetsInclude: ["**/*.wasm"],
        worker: {
            format: "es",
            plugins: function () { return workerStubPlugins; },
        },
        define: __assign({}, (0, ____ts_2.playgroundPlayViteDefine)(playEntryKind ? { "import.meta.env.PLAYGROUND_APP_KIND": JSON.stringify(playEntryKind) } : {})),
        plugins: __spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray(__spreadArray([
            playgroundPlayBootHtmlPlugin(),
            playgroundFlowWasmDevStubPlugin(repoRoot)
        ], semioAssetsVitePlugin(repoRoot), true), semioFaviconVitePlugin(repoRoot), true), playgroundAssetVitePlugins(repoRoot, exports.PLAYGROUND_PLAY_STATIC_ASSETS), true), (0, ____ts_1.uiTailwindBuildPlugins)(), true), [
            (0, ____ts_1.uiReactBuildPlugin)(),
            playgroundPlaywrightDevStubPlugin(),
            playgroundVitestDevStubPlugin(),
            (0, ____ts_5.playgroundIframeEmbedHeadersPlugin)(),
            playgroundStaleOptimizeDepPlugin()
        ], false), extraPlugins, true),
        build: playgroundStaticSiteBuildOptions(build),
        server: __assign(__assign(__assign({}, workspaceResolve.server), (watchIgnored ? { watch: { ignored: watchIgnored } } : {})), server),
        resolve: __assign(__assign({}, workspaceResolve.resolve), { dedupe: __spreadArray(__spreadArray([], ((_b = (_a = workspaceResolve.resolve) === null || _a === void 0 ? void 0 : _a.dedupe) !== null && _b !== void 0 ? _b : []), true), (resolveDedupe !== null && resolveDedupe !== void 0 ? resolveDedupe : []), true) }),
        optimizeDeps: __assign(__assign(__assign({}, workspaceResolve.optimizeDeps), optimizeDeps), { exclude: __spreadArray(__spreadArray([], ((_d = (_c = workspaceResolve.optimizeDeps) === null || _c === void 0 ? void 0 : _c.exclude) !== null && _d !== void 0 ? _d : []), true), ((_e = optimizeDeps === null || optimizeDeps === void 0 ? void 0 : optimizeDeps.exclude) !== null && _e !== void 0 ? _e : []), true) }),
    });
}
if (import.meta.vitest) {
    var registerTests1 = (await Promise.resolve().then(function () { return require("../../🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts"); })).registerTests1;
    await registerTests1(import.meta.vitest, { GIS_MAP_DEFAULT_PREFETCH_BOUNDS: exports.GIS_MAP_DEFAULT_PREFETCH_BOUNDS, PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT: exports.PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, PLAYGROUND_PLAY_BOOT_INLINE_STYLE: exports.PLAYGROUND_PLAY_BOOT_INLINE_STYLE, PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT: exports.PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT: exports.PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, PLAYGROUND_WASM_STUB_PREFIX: exports.PLAYGROUND_WASM_STUB_PREFIX, SEMIO_ASSET_ROOT: exports.SEMIO_ASSET_ROOT, SEMIO_FAVICON_HEAD_HTML: exports.SEMIO_FAVICON_HEAD_HTML, contentTypeForStaticDirAsset: contentTypeForStaticDirAsset, createServer: node_http_1.createServer, createWorkspaceViteResolveConfig: createWorkspaceViteResolveConfig, existsSync: node_fs_1.existsSync, fileURLToPath: node_url_1.fileURLToPath, findWorkspacePackages: findWorkspacePackages, isPlaygroundOptimizedDepUrl: isPlaygroundOptimizedDepUrl, playgroundOptimizedDepUrlPrefix: playgroundOptimizedDepUrlPrefix, join: node_path_1.join, listMapTilesForBounds: listMapTilesForBounds, mapTileCacheRoots: mapTileCacheRoots, meshAssetTransportUrl: ____ts_3.meshAssetTransportUrl, meshCollectionVitePlugin: meshCollectionVitePlugin, mkdirSync: node_fs_1.mkdirSync, mkdtempSync: node_fs_1.mkdtempSync, playgroundAssetVitePlugins: playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin: playgroundFlowWasmDevStubPlugin, playgroundPlayBootHtmlPlugin: playgroundPlayBootHtmlPlugin, playgroundSceneHostOptimizeDeps: playgroundSceneHostOptimizeDeps, playgroundSceneHostResolveAliases: playgroundSceneHostResolveAliases, playgroundWasmStubKey: playgroundWasmStubKey, prefetchMapTiles: prefetchMapTiles, resolve: node_path_1.resolve, resolveGisMapTileServeMode: resolveGisMapTileServeMode, resolveMeshAsset: ____ts_3.resolveMeshAsset, resolveSemioAssetRoot: resolveSemioAssetRoot, rewriteSpaFallbackToEmojiEntry: rewriteSpaFallbackToEmojiEntry, rmSync: node_fs_1.rmSync, semioFaviconSources: semioFaviconSources, semioFaviconSvgMarkup: semioFaviconSvgMarkup, semioFaviconVitePlugin: semioFaviconVitePlugin, semioHostHtmlString: semioHostHtmlString, semioHostHtmlVitePlugin: semioHostHtmlVitePlugin, startAssetServer: startAssetServer, staticDirVitePlugin: staticDirVitePlugin, statusSurfaceHtml: statusSurfaceHtml, tileProxyVitePlugin: tileProxyVitePlugin, tmpdir: node_os_1.tmpdir, writeFileSync: node_fs_1.writeFileSync }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🔖️ViteElementsAssets
