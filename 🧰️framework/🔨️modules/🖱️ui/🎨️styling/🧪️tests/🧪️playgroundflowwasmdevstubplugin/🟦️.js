"use strict";
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
Object.defineProperty(exports, "__esModule", { value: true });
exports.registerTests1 = registerTests1;
function registerTests1(vitest, dependencies, source) {
    return __awaiter(this, void 0, void 0, function () {
        var GIS_MAP_DEFAULT_PREFETCH_BOUNDS, PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, PLAYGROUND_PLAY_BOOT_INLINE_STYLE, PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, PLAYGROUND_WASM_STUB_PREFIX, SEMIO_ASSET_ROOT, SEMIO_FAVICON_HEAD_HTML, contentTypeForStaticDirAsset, createServer, createWorkspaceViteResolveConfig, existsSync, fileURLToPath, findWorkspacePackages, isPlaygroundOptimizedDepUrl, playgroundOptimizedDepUrlPrefix, listMapTilesForBounds, mapTileCacheRoots, meshAssetTransportUrl, meshCollectionVitePlugin, mkdirSync, mkdtempSync, playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundPlayBootHtmlPlugin, playgroundSceneHostOptimizeDeps, playgroundSceneHostResolveAliases, playgroundWasmStubKey, prefetchMapTiles, resolve, resolveGisMapTileServeMode, resolveMeshAsset, resolveSemioAssetRoot, rewriteSpaFallbackToEmojiEntry, rmSync, semioFaviconSources, semioFaviconSvgMarkup, semioFaviconVitePlugin, semioHostHtmlString, semioHostHtmlVitePlugin, startAssetServer, staticDirVitePlugin, statusSurfaceHtml, tileProxyVitePlugin, tmpdir, writeFileSync, join, describe, expect, it, repoRoot;
        var _this = this;
        return __generator(this, function (_a) {
            GIS_MAP_DEFAULT_PREFETCH_BOUNDS = dependencies.GIS_MAP_DEFAULT_PREFETCH_BOUNDS, PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT = dependencies.PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, PLAYGROUND_PLAY_BOOT_INLINE_STYLE = dependencies.PLAYGROUND_PLAY_BOOT_INLINE_STYLE, PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT = dependencies.PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT = dependencies.PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, PLAYGROUND_WASM_STUB_PREFIX = dependencies.PLAYGROUND_WASM_STUB_PREFIX, SEMIO_ASSET_ROOT = dependencies.SEMIO_ASSET_ROOT, SEMIO_FAVICON_HEAD_HTML = dependencies.SEMIO_FAVICON_HEAD_HTML, contentTypeForStaticDirAsset = dependencies.contentTypeForStaticDirAsset, createServer = dependencies.createServer, createWorkspaceViteResolveConfig = dependencies.createWorkspaceViteResolveConfig, existsSync = dependencies.existsSync, fileURLToPath = dependencies.fileURLToPath, findWorkspacePackages = dependencies.findWorkspacePackages, isPlaygroundOptimizedDepUrl = dependencies.isPlaygroundOptimizedDepUrl, playgroundOptimizedDepUrlPrefix = dependencies.playgroundOptimizedDepUrlPrefix, listMapTilesForBounds = dependencies.listMapTilesForBounds, mapTileCacheRoots = dependencies.mapTileCacheRoots, meshAssetTransportUrl = dependencies.meshAssetTransportUrl, meshCollectionVitePlugin = dependencies.meshCollectionVitePlugin, mkdirSync = dependencies.mkdirSync, mkdtempSync = dependencies.mkdtempSync, playgroundAssetVitePlugins = dependencies.playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin = dependencies.playgroundFlowWasmDevStubPlugin, playgroundPlayBootHtmlPlugin = dependencies.playgroundPlayBootHtmlPlugin, playgroundSceneHostOptimizeDeps = dependencies.playgroundSceneHostOptimizeDeps, playgroundSceneHostResolveAliases = dependencies.playgroundSceneHostResolveAliases, playgroundWasmStubKey = dependencies.playgroundWasmStubKey, prefetchMapTiles = dependencies.prefetchMapTiles, resolve = dependencies.resolve, resolveGisMapTileServeMode = dependencies.resolveGisMapTileServeMode, resolveMeshAsset = dependencies.resolveMeshAsset, resolveSemioAssetRoot = dependencies.resolveSemioAssetRoot, rewriteSpaFallbackToEmojiEntry = dependencies.rewriteSpaFallbackToEmojiEntry, rmSync = dependencies.rmSync, semioFaviconSources = dependencies.semioFaviconSources, semioFaviconSvgMarkup = dependencies.semioFaviconSvgMarkup, semioFaviconVitePlugin = dependencies.semioFaviconVitePlugin, semioHostHtmlString = dependencies.semioHostHtmlString, semioHostHtmlVitePlugin = dependencies.semioHostHtmlVitePlugin, startAssetServer = dependencies.startAssetServer, staticDirVitePlugin = dependencies.staticDirVitePlugin, statusSurfaceHtml = dependencies.statusSurfaceHtml, tileProxyVitePlugin = dependencies.tileProxyVitePlugin, tmpdir = dependencies.tmpdir, writeFileSync = dependencies.writeFileSync, join = dependencies.join;
            describe = vitest.describe, expect = vitest.expect, it = vitest.it;
            repoRoot = resolve(fileURLToPath(new URL(".", source.url)), "../../../../../..");
            describe("playgroundFlowWasmDevStubPlugin", function () {
                var importer = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx");
                var plugin = playgroundFlowWasmDevStubPlugin(repoRoot);
                var resolveId = plugin.resolveId;
                it("resolves bare @semio-tech/flow-core to the wasm-pack entry, not the stub", function () {
                    var resolved = resolveId("@semio-tech/flow-core", importer);
                    expect(resolved).toBeDefined();
                    expect(resolved).not.toContain("playground-wasm-stub");
                    expect(resolved).toMatch(/flow_core\.js$/);
                    expect(existsSync(resolved)).toBe(true);
                });
                it("falls back to stub for an unbuilt @semio-tech wasm package subpath", function () {
                    var id = "@semio-tech/__playground_wasm_stub_test_missing__/pkg/entry.js";
                    var resolved = resolveId(id, importer);
                    expect(resolved).toBe("".concat(PLAYGROUND_WASM_STUB_PREFIX).concat(playgroundWasmStubKey(id)));
                });
            });
            describe("isPlaygroundOptimizedDepUrl", function () {
                it("matches Vite prebundle chunk URLs", function () {
                    var classic = playgroundOptimizedDepUrlPrefix("/repo", "/repo/node_modules/.vite");
                    var shared = playgroundOptimizedDepUrlPrefix("/repo", "/repo/.🧬semio/🦑️repo/⚡️cache/vite/os-dev/draw-react");
                    expect(classic).toBe("/node_modules/.vite/deps/");
                    expect(isPlaygroundOptimizedDepUrl("/node_modules/.vite/deps/chunk-ABC.js?v=1", classic)).toBe(true);
                    expect(isPlaygroundOptimizedDepUrl(encodeURI("/.🧬semio/🦑️repo/⚡️cache/vite/os-dev/draw-react/deps/chunk-ABC.js?v=1"), shared)).toBe(true);
                    expect(isPlaygroundOptimizedDepUrl("/index.ts", shared)).toBe(false);
                    expect(isPlaygroundOptimizedDepUrl("/%E0%A4%A", shared)).toBe(false);
                    expect(playgroundOptimizedDepUrlPrefix("/repo/app", "/cache/vite")).toBe("/@fs/cache/vite/deps/");
                });
            });
            describe("playgroundSceneHostResolveAliases", function () {
                it("pins fiber and drei to node_modules entries", function () {
                    var aliases = playgroundSceneHostResolveAliases(repoRoot);
                    expect(aliases.some(function (row) { return String(row.find).includes("fiber") && row.replacement.endsWith("react-three-fiber.esm.js"); })).toBe(true);
                    expect(aliases.some(function (row) { return String(row.find).includes("drei") && row.replacement.endsWith("@react-three/drei/index.js"); })).toBe(true);
                });
            });
            describe("playgroundSceneHostOptimizeDeps", function () {
                it("never prebundles R3F packages pinned by scene-host aliases", function () {
                    var deps = playgroundSceneHostOptimizeDeps({ include: ["@react-three/fiber"], exclude: ["playwright"] });
                    expect(deps.include).toContain("three");
                    expect(deps.include).not.toContain("@react-three/fiber");
                    expect(deps.exclude).toEqual(expect.arrayContaining(["@react-three/fiber", "@react-three/drei", "playwright"]));
                });
                it("prebundles the CommonJS packages the excluded R3F graph still imports (fiber → scheduler, drei → stats.js, drei → tunnel-rat → zustand → use-sync-external-store)", function () {
                    var deps = playgroundSceneHostOptimizeDeps();
                    expect(deps.include).toEqual(expect.arrayContaining(["scheduler", "stats.js", "use-sync-external-store/shim/index.js", "use-sync-external-store/shim/with-selector.js"]));
                    expect(deps.exclude).not.toEqual(expect.arrayContaining(["use-sync-external-store/shim/with-selector.js"]));
                });
            });
            describe("resolveGisMapTileServeMode", function () {
                it("defaults to fetch", function () {
                    expect(resolveGisMapTileServeMode(undefined)).toBe("fetch");
                    expect(resolveGisMapTileServeMode("")).toBe("fetch");
                    expect(resolveGisMapTileServeMode("online")).toBe("fetch");
                });
                it("selects bundle only for bundle", function () {
                    expect(resolveGisMapTileServeMode("bundle")).toBe("bundle");
                });
            });
            describe("tileProxyVitePlugin", function () {
                var osmSpec = {
                    kind: "tile-proxy",
                    route: "/osm",
                    upstream: "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
                    cache: "osm-tiles",
                };
                it("adds a build copy plugin only for bundle mode", function () {
                    var fetchPlugins = tileProxyVitePlugin(repoRoot, osmSpec, "fetch");
                    var bundlePlugins = tileProxyVitePlugin(repoRoot, osmSpec, "bundle");
                    expect(fetchPlugins.some(function (plugin) { return plugin.name === "tile-proxy-build/osm"; })).toBe(false);
                    expect(bundlePlugins.some(function (plugin) { return plugin.name === "tile-proxy-build/osm"; })).toBe(true);
                });
            });
            describe("playgroundAssetVitePlugins", function () {
                it("dispatches each asset kind to its generic factory and dedupes by kind+route", function () {
                    var specs = [
                        { kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🧫️fixtures" },
                        { kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🧫️fixtures" },
                    ];
                    var plugins = playgroundAssetVitePlugins(repoRoot, specs);
                    expect(plugins.filter(function (plugin) { return plugin.name === "static-dir-serve/cad-fixture"; })).toHaveLength(1);
                });
            });
            describe("contentTypeForStaticDirAsset", function () {
                it("assigns module script mime types for wasm plugin artifacts", function () {
                    expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/⛏️sourcing/sourcing_plugin.js")).toBe("text/javascript");
                    expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js")).toBe("text/javascript");
                    expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/🧩️puzzle/🕸️puzzle_plugin.wasm")).toBe("application/wasm");
                });
            });
            describe("staticDirVitePlugin", function () {
                it("answers 404 for missing files under the static route instead of SPA fallback", function () { return __awaiter(_this, void 0, void 0, function () {
                    var sandbox, inputDir, middleware_1, plugin, missingStatus;
                    var _a;
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0:
                                sandbox = mkdtempSync(join(tmpdir(), "semio-static-dir-404-"));
                                _b.label = 1;
                            case 1:
                                _b.trys.push([1, , 3, 4]);
                                inputDir = join(sandbox, "📥️input");
                                mkdirSync(inputDir, { recursive: true });
                                writeFileSync(join(inputDir, "present.png"), Buffer.from([0x89, 0x50, 0x4e, 0x47]));
                                plugin = staticDirVitePlugin(sandbox, { kind: "static-dir", route: "/fixture", root: "📥️input" })[0];
                                (_a = plugin.configureServer) === null || _a === void 0 ? void 0 : _a.call(plugin, { middlewares: { use: function (fn) { middleware_1 = fn; } } });
                                expect(middleware_1).toBeDefined();
                                return [4 /*yield*/, new Promise(function (resolvePromise) {
                                        var response = { statusCode: 200, end: function () { resolvePromise(this.statusCode); } };
                                        middleware_1({ url: "/fixture/missing.png" }, response, function () { return resolvePromise(200); });
                                    })];
                            case 2:
                                missingStatus = _b.sent();
                                expect(missingStatus).toBe(404);
                                return [3 /*break*/, 4];
                            case 3:
                                rmSync(sandbox, { recursive: true, force: true });
                                return [7 /*endfinally*/];
                            case 4: return [2 /*return*/];
                        }
                    });
                }); });
            });
            describe("listMapTilesForBounds", function () {
                it("covers Switzerland at z0 with a single world tile", function () {
                    var tiles = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 0, 0);
                    expect(tiles).toEqual([{ z: 0, x: 0, y: 0 }]);
                });
                it("returns more tiles at higher zoom", function () {
                    // 🇨️🇭️ Switzerland still fits inside a single OSM tile up to z6 (~5.6°/tile > its ~4.6° span), so
                    // the comparison needs a zoom gap wide enough to actually straddle a tile boundary.
                    var z2 = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 2, 2).length;
                    var z8 = listMapTilesForBounds(GIS_MAP_DEFAULT_PREFETCH_BOUNDS, 8, 8).length;
                    expect(z8).toBeGreaterThan(z2);
                });
            });
            describe("prefetchMapTiles", function () {
                it("skips tiles already present in cache without fetching", function () { return __awaiter(_this, void 0, void 0, function () {
                    var osm, tile, filePath, hadCache, lines, result, unlinkSync;
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0:
                                osm = mapTileCacheRoots(repoRoot).osm;
                                tile = { z: 0, x: 0, y: 0 };
                                filePath = resolve(osm, "".concat(tile.z, "/").concat(tile.x, "/").concat(tile.y, ".png"));
                                hadCache = existsSync(filePath);
                                if (!hadCache) {
                                    mkdirSync(resolve(filePath, ".."), { recursive: true });
                                    writeFileSync(filePath, Buffer.from([0x89, 0x50, 0x4e, 0x47]));
                                }
                                lines = [];
                                return [4 /*yield*/, prefetchMapTiles({
                                        repoRoot: repoRoot,
                                        bounds: GIS_MAP_DEFAULT_PREFETCH_BOUNDS,
                                        raster: true,
                                        vector: false,
                                        zMinRaster: 0,
                                        zMaxRaster: 0,
                                        concurrency: 4,
                                        delayMs: 0,
                                        log: function (line) { return lines.push(line); },
                                    })];
                            case 1:
                                result = _a.sent();
                                expect(result.skipped).toBeGreaterThan(0);
                                expect(result.downloaded).toBe(0);
                                expect(lines.some(function (line) { return line.includes("cached"); })).toBe(true);
                                if (!!hadCache) return [3 /*break*/, 3];
                                return [4 /*yield*/, Promise.resolve().then(function () { return require("node:fs"); })];
                            case 2:
                                unlinkSync = (_a.sent()).unlinkSync;
                                unlinkSync(filePath);
                                _a.label = 3;
                            case 3: return [2 /*return*/];
                        }
                    });
                }); });
            });
            describe("resolveSemioAssetRoot", function () {
                it("resolves the merged asset package with fonts", function () {
                    var root = resolveSemioAssetRoot(repoRoot);
                    expect(root.endsWith(SEMIO_ASSET_ROOT.split("/").pop())).toBe(true);
                    expect(existsSync(resolve(root, "🔤️fonts/🚀️anta/🏛️latin/📖️regular/🗜️compressed.woff2"))).toBe(true);
                });
                it("throws when fonts are missing", function () {
                    expect(function () { return resolveSemioAssetRoot(resolve(repoRoot, ".🧬semio")); }).toThrow(/Missing Semio asset root/);
                });
            });
            describe("playgroundPlayBootHtmlPlugin", function () {
                it("registers index html boot injection", function () {
                    expect(playgroundPlayBootHtmlPlugin().name).toBe("playground-play-boot-html");
                });
                it("exposes inline appearance and reveal scripts", function () {
                    expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).toContain("prefers-color-scheme");
                    // 🌓️ The ONE document the OS shell writes — never `ui.chrome.appearance`, which nothing has
                    // written since the shell moved to the event-sourced config lane (the fixture-driven law in
                    // `🧪️tests/🧩️suite/🟦️.ts` is what states this; this line keeps the retired key out).
                    expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).toContain("semio.os.config");
                    expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).not.toContain("ui.chrome.appearance");
                    expect(PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT).toContain("semio-play-styles");
                    expect(PLAYGROUND_PLAY_BOOT_INLINE_STYLE).toContain("data-semio-styled");
                });
                it("exposes an inline theme bootstrap script replaying the persisted ui-preference log", function () {
                    expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("os.config.ui-preferences");
                    expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("setCustomTheme");
                    expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).not.toContain("ui.chrome.theme.snapshot");
                    expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("--color-");
                    expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("dataset.uiTheme");
                });
                it("injects the theme script after the appearance script and before the stylesheet link", function () { return __awaiter(_this, void 0, void 0, function () {
                    var hook, injected, tags, kinds;
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0:
                                hook = playgroundPlayBootHtmlPlugin().transformIndexHtml;
                                if (typeof hook !== "object")
                                    throw new Error("playground play boot html hook must declare its order");
                                return [4 /*yield*/, hook.handler("", { path: "/🌐️.html", filename: "🌐️.html" })];
                            case 1:
                                injected = _a.sent();
                                if (typeof injected !== "object" || injected === null || Array.isArray(injected) || !("tags" in injected))
                                    throw new Error("playground play boot html hook must return injected tags");
                                tags = injected.tags;
                                kinds = tags.map(function (tag) { return (tag.children === PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT ? "appearance" : tag.children === PLAYGROUND_PLAY_BOOT_THEME_SCRIPT ? "theme" : tag.attrs && "href" in tag.attrs ? "stylesheet" : "other"); });
                                expect(kinds.indexOf("appearance")).toBeLessThan(kinds.indexOf("theme"));
                                expect(kinds.indexOf("theme")).toBeLessThan(kinds.indexOf("stylesheet"));
                                return [2 /*return*/];
                        }
                    });
                }); });
            });
            describe("rewriteSpaFallbackToEmojiEntry", function () {
                var entry = "/🌐️.html";
                it("rewrites /index.html to the emoji entry", function () {
                    expect(rewriteSpaFallbackToEmojiEntry("/index.html", entry)).toBe(entry);
                });
                it("preserves query and hash on /index.html", function () {
                    expect(rewriteSpaFallbackToEmojiEntry("/index.html?x=1#frag", entry)).toBe("".concat(entry, "?x=1#frag"));
                });
                it("leaves asset paths unchanged", function () {
                    expect(rewriteSpaFallbackToEmojiEntry("/spaces/space-1", entry)).toBe("/spaces/space-1");
                });
            });
            describe("semioHostHtmlString", function () {
                it("renders title, entry module, root mount, favicon links, and boot scripts", function () {
                    var html = semioHostHtmlString({ title: "Semio App", entry: "/js/index.tsx" });
                    expect(html).toContain("<title>Semio App</title>");
                    expect(html).toContain('<script type="module" src="/js/index.tsx"></script>');
                    expect(html).toContain('<div id="root">');
                    expect(html).toContain(SEMIO_FAVICON_HEAD_HTML);
                    expect(html).toContain(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT);
                    expect(html).toContain(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT);
                    expect(html).toContain(PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT);
                });
                it("honors rootId, bodyClass, csp, and loading overrides", function () {
                    var html = semioHostHtmlString({
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
            describe("semioHostHtmlVitePlugin", function () {
                it("bundles favicon serving and static-deploy-marker plugins alongside the host html plugin", function () {
                    var plugins = semioHostHtmlVitePlugin(repoRoot, { title: "Semio App", entry: "/js/index.tsx" });
                    expect(plugins.map(function (plugin) { return plugin.name; })).toEqual(["semio-favicon-serve", "semio-favicon-build", "static-deploy-markers", "semio-host-html"]);
                });
                it("renders the same document semioHostHtmlString produces", function () {
                    var spec = { title: "Semio App", entry: "/js/index.tsx" };
                    var plugin = semioHostHtmlVitePlugin(repoRoot, spec).find(function (p) { return p.name === "semio-host-html"; });
                    var result = plugin.transformIndexHtml.handler();
                    expect(result).toBe(semioHostHtmlString(spec));
                });
            });
            describe("statusSurfaceHtml", function () {
                it("renders title, description, and status kind with no external CSS dependency", function () {
                    var html = statusSurfaceHtml({ kind: "error", title: "Something went wrong", description: "Try again later." });
                    expect(html).toContain("Something went wrong");
                    expect(html).toContain("Try again later.");
                    expect(html).toContain('data-status-kind="error"');
                    expect(html).not.toContain("<link");
                    expect(html).not.toContain('rel="stylesheet"');
                });
                it("omits the description paragraph when none is given", function () {
                    var html = statusSurfaceHtml({ kind: "loading", title: "Loading…" });
                    expect(html).toContain('data-status-kind="loading"');
                    expect(html).not.toContain("<p style=\"margin:8px 0 0");
                });
            });
            describe("semioFaviconVitePlugin", function () {
                it("points at round dark emblem svg and ico under asset/logo", function () {
                    var _a = semioFaviconSources(repoRoot), svg = _a.svg, ico = _a.ico;
                    expect(svg).toBe(resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg"));
                    expect(ico).toBe(resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos/🌐️favicon/🌘️dark-round/📏️size-32.ico"));
                    expect(existsSync(svg)).toBe(true);
                    expect(existsSync(ico)).toBe(true);
                });
                it("registers serve and build plugins", function () {
                    var plugins = semioFaviconVitePlugin(repoRoot);
                    expect(plugins.map(function (plugin) { return plugin.name; })).toEqual(["semio-favicon-serve", "semio-favicon-build"]);
                });
                it("injects opaque bleed into round dark favicon svg", function () {
                    var svg = semioFaviconSources(repoRoot).svg;
                    var markup = semioFaviconSvgMarkup(svg);
                    expect(markup).toContain('<rect width="350" height="350" fill="#001117"/>');
                });
            });
            describe("meshCollectionVitePlugin", function () {
                var puzzle3dMeshSpec = {
                    kind: "mesh-collection",
                    route: "/mesh",
                    catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json",
                };
                it("points at metabolism and abbau-aufbau kit glbs plus shared placeholder", function () {
                    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️capsule_J.glb").source))).toBe(true);
                    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️capsule-with-balcony_slash.glb").source))).toBe(true);
                    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb").source))).toBe(true);
                    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️placeholder.glb").source))).toBe(true);
                });
                it("registers serve and build plugins named after the route", function () {
                    var plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
                    expect(plugins.map(function (plugin) { return plugin.name; })).toEqual(["mesh-collection-serve/mesh", "mesh-collection-build/mesh"]);
                });
                it("startAssetServer serves 🧊️base.glb as model/gltf-binary", function () { return __awaiter(_this, void 0, void 0, function () {
                    var probe, address, port, server, response, bytes, _a;
                    return __generator(this, function (_b) {
                        switch (_b.label) {
                            case 0:
                                probe = createServer();
                                return [4 /*yield*/, new Promise(function (resolveListen) { return probe.listen(0, "127.0.0.1", function () { return resolveListen(); }); })];
                            case 1:
                                _b.sent();
                                address = probe.address();
                                if (!address || typeof address === "string")
                                    throw new Error("expected TCP address");
                                port = address.port;
                                return [4 /*yield*/, new Promise(function (resolveClose, reject) { return probe.close(function (err) { return (err ? reject(err) : resolveClose()); }); })];
                            case 2:
                                _b.sent();
                                server = startAssetServer(repoRoot, port, [puzzle3dMeshSpec]);
                                _b.label = 3;
                            case 3:
                                _b.trys.push([3, , 6, 8]);
                                return [4 /*yield*/, fetch("http://127.0.0.1:".concat(port).concat(meshAssetTransportUrl("/mesh/🧊️base.glb")))];
                            case 4:
                                response = _b.sent();
                                expect(response.status).toBe(200);
                                expect(response.headers.get("content-type")).toBe("model/gltf-binary");
                                _a = Uint8Array.bind;
                                return [4 /*yield*/, response.arrayBuffer()];
                            case 5:
                                bytes = new (_a.apply(Uint8Array, [void 0, _b.sent()]))();
                                expect(String.fromCharCode(bytes[0], bytes[1], bytes[2], bytes[3])).toBe("glTF");
                                return [3 /*break*/, 8];
                            case 6: return [4 /*yield*/, new Promise(function (resolveClose, reject) { return server.close(function (err) { return (err ? reject(err) : resolveClose()); }); })];
                            case 7:
                                _b.sent();
                                return [7 /*endfinally*/];
                            case 8: return [2 /*return*/];
                        }
                    });
                }); });
            });
            describe("createWorkspaceViteResolveConfig", function () {
                // ⏱️ `findWorkspacePackages` walks the whole repo tree — past the 5s default on this monorepo's size.
                it("pins scene hosts and excludes workspace packages from optimizeDeps", function () {
                    var _a, _b, _c, _d, _e;
                    var config = createWorkspaceViteResolveConfig(repoRoot);
                    expect((_a = config.resolve) === null || _a === void 0 ? void 0 : _a.dedupe).toContain("react");
                    expect((_b = config.resolve) === null || _b === void 0 ? void 0 : _b.dedupe).toContain("three");
                    expect((_d = (_c = config.server) === null || _c === void 0 ? void 0 : _c.fs) === null || _d === void 0 ? void 0 : _d.allow).toContain(repoRoot);
                    expect((_e = config.optimizeDeps) === null || _e === void 0 ? void 0 : _e.exclude).toContain("@semio-tech/flow-module-core");
                }, 20000);
            });
            describe("findWorkspacePackages", function () {
                it("discovers workspace packages while skipping hidden dot directories", function () {
                    var pkgs = findWorkspacePackages(repoRoot);
                    expect(pkgs).toContain("@semio-tech/ui-react");
                    expect(pkgs.every(function (p) { return p.startsWith("@semio-tech/"); })).toBe(true);
                }, 20000);
            });
            return [2 /*return*/];
        });
    });
}
