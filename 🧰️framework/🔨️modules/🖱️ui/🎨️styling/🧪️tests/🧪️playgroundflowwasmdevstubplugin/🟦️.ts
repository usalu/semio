import type { AssetDeliveryDeclarationV1 } from "../../../../🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts";
import type { TileProxyAssetSpecV1 } from "../../../../🖼️assets/🗺️tile-proxy/🟦️.ts";
import type { MeshCollectionAssetSpecV1 } from "../../🏗️builder/🌐️vite/🟦️.ts";
type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../../../🖼️assets/🥽️mesh/🟦️.ts"), "meshAssetTransportUrl" | "resolveMeshAsset"> & Pick<typeof import("../../../../🖼️assets/🥽️mesh/📇️catalog/🟦️.ts"), "MESH_DELIVERY_CATALOG"> & Pick<typeof import("../../../../🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts"), "createAssetBuildPluginsV1" | "createAssetHttpServerV1" | "resolveAssetDeliveryModeV1"> & Pick<typeof import("../../🏗️builder/🌐️vite/🟦️.ts"), "TILE_PROXY_ASSET_PROVIDER_V1" | "MESH_COLLECTION_ASSET_PROVIDER_V1" | "STATIC_DIRECTORY_ASSET_PROVIDER_V1" | "PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT" | "PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT" | "PLAYGROUND_PLAY_BOOT_INLINE_STYLE" | "PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT" | "PLAYGROUND_PLAY_BOOT_THEME_SCRIPT" | "PLAYGROUND_WASM_STUB_PREFIX" | "SEMIO_ASSET_ROOT" | "SEMIO_FAVICON_HEAD_HTML" | "contentTypeForStaticDirAsset" | "createWorkspaceViteResolveConfig" | "findWorkspacePackages" | "isPlaygroundOptimizedDepUrl" | "meshCollectionVitePlugin" | "playgroundFlowWasmDevStubPlugin" | "playgroundOptimizedDepUrlPrefix" | "playgroundPlayBootHtmlPlugin" | "playgroundSceneHostOptimizeDeps" | "playgroundSceneHostResolveAliases" | "playgroundWasmStubKey" | "resolveSemioAssetRoot" | "rewriteSpaFallbackToEmojiEntry" | "semioFaviconSources" | "semioFaviconSvgMarkup" | "semioFaviconVitePlugin" | "semioHostHtmlString" | "semioHostHtmlVitePlugin" | "staticDirVitePlugin" | "statusSurfaceHtml" | "tileProxyVitePlugin"> & Pick<typeof import("node:fs"), "existsSync" | "mkdirSync" | "mkdtempSync" | "rmSync" | "symlinkSync" | "writeFileSync"> & Pick<typeof import("node:http"), "createServer"> & Pick<typeof import("node:os"), "tmpdir"> & Pick<typeof import("node:path"), "join" | "resolve"> & Pick<typeof import("node:url"), "fileURLToPath">, source: TestSource): Promise<void> {
  const { MESH_DELIVERY_CATALOG, TILE_PROXY_ASSET_PROVIDER_V1, MESH_COLLECTION_ASSET_PROVIDER_V1, STATIC_DIRECTORY_ASSET_PROVIDER_V1, PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT, PLAYGROUND_PLAY_BOOT_INLINE_STYLE, PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, PLAYGROUND_WASM_STUB_PREFIX, SEMIO_ASSET_ROOT, SEMIO_FAVICON_HEAD_HTML, contentTypeForStaticDirAsset, createServer, createWorkspaceViteResolveConfig, existsSync, fileURLToPath, findWorkspacePackages, isPlaygroundOptimizedDepUrl, playgroundOptimizedDepUrlPrefix, meshAssetTransportUrl, meshCollectionVitePlugin, mkdirSync, mkdtempSync, createAssetBuildPluginsV1, playgroundFlowWasmDevStubPlugin, playgroundPlayBootHtmlPlugin, playgroundSceneHostOptimizeDeps, playgroundSceneHostResolveAliases, playgroundWasmStubKey, resolve, resolveAssetDeliveryModeV1, resolveMeshAsset, resolveSemioAssetRoot, rewriteSpaFallbackToEmojiEntry, rmSync, semioFaviconSources, semioFaviconSvgMarkup, semioFaviconVitePlugin, semioHostHtmlString, semioHostHtmlVitePlugin, createAssetHttpServerV1, staticDirVitePlugin, statusSurfaceHtml, symlinkSync, tileProxyVitePlugin, tmpdir, writeFileSync, join } = dependencies;

  const ASSET_PROVIDERS_V1 = [TILE_PROXY_ASSET_PROVIDER_V1, MESH_COLLECTION_ASSET_PROVIDER_V1, STATIC_DIRECTORY_ASSET_PROVIDER_V1];
  const { describe, expect, it } = vitest;
  const repoRoot = resolve(fileURLToPath(new URL(".", source.url)), "../../../../../..");

  describe("playgroundFlowWasmDevStubPlugin", () => {
    const importer = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx");
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
      const classic = playgroundOptimizedDepUrlPrefix("/repo", "/repo/node_modules/.vite");
      const shared = playgroundOptimizedDepUrlPrefix("/repo", "/repo/.🧬semio/🦑️repo/⚡️cache/vite/os-dev/draw-react");
      expect(classic).toBe("/node_modules/.vite/deps/");
      expect(isPlaygroundOptimizedDepUrl("/node_modules/.vite/deps/chunk-ABC.js?v=1", classic)).toBe(true);
      expect(isPlaygroundOptimizedDepUrl(encodeURI("/.🧬semio/🦑️repo/⚡️cache/vite/os-dev/draw-react/deps/chunk-ABC.js?v=1"), shared)).toBe(true);
      expect(isPlaygroundOptimizedDepUrl("/index.ts", shared)).toBe(false);
      expect(isPlaygroundOptimizedDepUrl("/%E0%A4%A", shared)).toBe(false);
      expect(playgroundOptimizedDepUrlPrefix("/repo/app", "/cache/vite")).toBe("/@fs/cache/vite/deps/");
    });
  });

  describe("playgroundSceneHostResolveAliases", () => {
    it("pins fiber and drei to node_modules entries", () => {
      const aliases = playgroundSceneHostResolveAliases(repoRoot);
      expect(aliases.some((row) => String(row.find).includes("fiber") && row.replacement.endsWith("react-three-fiber.esm.js"))).toBe(true);
      expect(aliases.some((row) => String(row.find).includes("drei") && row.replacement.endsWith("@react-three/drei/index.js"))).toBe(true);
    });
  });

  describe("playgroundSceneHostOptimizeDeps", () => {
    it("never prebundles R3F packages pinned by scene-host aliases", () => {
      const deps = playgroundSceneHostOptimizeDeps({ include: ["@react-three/fiber"], exclude: ["playwright"] });
      expect(deps.include).toContain("three");
      expect(deps.include).not.toContain("@react-three/fiber");
      expect(deps.exclude).toEqual(expect.arrayContaining(["@react-three/fiber", "@react-three/drei", "playwright"]));
    });
    it("prebundles the CommonJS packages the excluded R3F graph still imports (fiber → scheduler, drei → stats.js, drei → tunnel-rat → zustand → use-sync-external-store)", () => {
      const deps = playgroundSceneHostOptimizeDeps();
      expect(deps.include).toEqual(expect.arrayContaining(["scheduler", "stats.js", "use-sync-external-store/shim/index.js", "use-sync-external-store/shim/with-selector.js"]));
      expect(deps.exclude).not.toEqual(expect.arrayContaining(["use-sync-external-store/shim/with-selector.js"]));
    });
    it("prebundles three-stdlib, the barrel every drei control imports, so a cold boot fetches one module instead of 282", () => {
      const deps = playgroundSceneHostOptimizeDeps();
      expect(deps.include).toEqual(expect.arrayContaining(["three", "three-stdlib"]));
      expect(deps.exclude).not.toContain("three-stdlib");
    });
  });

  describe("resolveAssetDeliveryModeV1", () => {
    it("executes the complete asset builder with all product runtime and type loading refused", async () => {
      const { spawnSync } = await import("node:child_process");
      const { dirname } = await import("node:path");
      const { readFileSync } = await import("node:fs");
      const directory = dirname(fileURLToPath(source.url));
      const products = resolve(directory, "../../../../../🛍️products").replaceAll("\\", "/") + "/";
      const corpus = JSON.parse(readFileSync(resolve(directory, "../../../../🖼️assets/🗺️tile-proxy/🧫️fixtures/🔣️.json"), "utf8"));
      const program = `import {tileProxyVitePlugin} from ${JSON.stringify(fileURLToPath(source.url))};import {resolveAssetDeliveryModeV1} from ${JSON.stringify(resolve(directory, "../../../../🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts"))};console.log(JSON.stringify(${JSON.stringify(corpus.cases)}.map(row=>{try{return{accepted:true,plugins:tileProxyVitePlugin(${JSON.stringify(directory)},row.value,resolveAssetDeliveryModeV1("bundle")).map(plugin=>plugin.name)}}catch{return{accepted:false,plugins:[]}}})));`;
      const standalone = String.raw`import { build } from "esbuild";import ts from "typescript";import { readFileSync } from "node:fs";import { dirname,resolve } from "node:path";const products=${JSON.stringify(products)};const bundle=await build({stdin:{contents:${JSON.stringify(program)},resolveDir:${JSON.stringify(directory)}},define:{"import.meta.vitest":"undefined","import.meta.url":${JSON.stringify(JSON.stringify(source.url))}},bundle:true,platform:"node",format:"esm",write:false,plugins:[{name:"neutral-asset-builder",setup(builder){builder.onLoad({filter:/.*/},input=>{if(input.path.replaceAll("\\","/").startsWith(products))return{errors:[{text:"Asset builder loads a concrete product: "+input.path}]};if(/\.[cm]?tsx?$/.test(input.path)){const unit=ts.createSourceFile(input.path,readFileSync(input.path,"utf8"),ts.ScriptTarget.Latest,true);for(const statement of unit.statements){if((ts.isImportDeclaration(statement)||ts.isExportDeclaration(statement))&&statement.moduleSpecifier&&ts.isStringLiteral(statement.moduleSpecifier)&&statement.moduleSpecifier.text.startsWith(".")){const dependency=resolve(dirname(input.path),statement.moduleSpecifier.text).replaceAll("\\","/");if(dependency.startsWith(products))return{errors:[{text:"Asset builder type or value interface loads a concrete product: "+dependency}]}}}}})}}]});await import("data:text/javascript;base64,"+Buffer.from(bundle.outputFiles[0].text).toString("base64"));`;
      const native = spawnSync("node", ["--input-type=module"], { cwd: directory, input: standalone, encoding: "utf8" });
      expect(native.status, native.stderr).toBe(0);
      const actual = JSON.parse(native.stdout);
      expect(actual).toEqual(corpus.cases.map((row: { accepted: boolean; value: { route: string } }) => ({ accepted: row.accepted, plugins: row.accepted ? ["tile-proxy-serve" + row.value.route, "tile-proxy-build" + row.value.route] : [] })));
    });

    it("defaults to fetch", () => {
      expect(resolveAssetDeliveryModeV1(undefined)).toBe("fetch");
      expect(resolveAssetDeliveryModeV1("")).toBe("fetch");
      expect(() => resolveAssetDeliveryModeV1("online")).toThrow();
    });

    it("selects bundle only for bundle", () => {
      expect(resolveAssetDeliveryModeV1("bundle")).toBe("bundle");
    });
  });

  describe("tileProxyVitePlugin", () => {
    const osmSpec: TileProxyAssetSpecV1 = {
      kind: "tile-proxy",
      route: "/osm",
      upstream: "https://fixture.example/{z}/{x}/{y}.png",
      cache: ".cache/tiles",
      userAgent: "Fixture/1",
    };

    it("adds a build copy plugin only for bundle mode", () => {
      const fetchPlugins = tileProxyVitePlugin(repoRoot, osmSpec, "fetch");
      const bundlePlugins = tileProxyVitePlugin(repoRoot, osmSpec, "bundle");
      expect(fetchPlugins.some((plugin) => plugin.name === "tile-proxy-build/osm")).toBe(false);
      expect(bundlePlugins.some((plugin) => plugin.name === "tile-proxy-build/osm")).toBe(true);
    });
  });

  describe("createAssetBuildPluginsV1", () => {
    it("dispatches each asset kind to its generic factory and dedupes by kind+route", () => {
      const specs: AssetDeliveryDeclarationV1[] = [
        { kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🧫️fixtures" },
        { kind: "static-dir", route: "/cad-fixture", root: "✏️s/🔌️plugins/📐️cad/🧫️fixtures" },
      ];
      const plugins = createAssetBuildPluginsV1(repoRoot, specs, ASSET_PROVIDERS_V1);
      expect(plugins.filter((plugin) => plugin.name === "static-dir-serve/cad-fixture")).toHaveLength(1);
    });
  });

  describe("contentTypeForStaticDirAsset", () => {
    it("assigns module script mime types for wasm plugin artifacts", () => {
      expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/⛏️sourcing/sourcing_plugin.js")).toBe("text/javascript");
      expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js")).toBe("text/javascript");
      expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/🧩️puzzle/🕸️puzzle_plugin.wasm")).toBe("application/wasm");
    });
  });

  describe("staticDirVitePlugin", () => {
    it("answers 404 for missing files under the static route instead of SPA fallback", async () => {
      const sandbox = mkdtempSync(join(tmpdir(), "semio-static-dir-404-"));
      try {
        const inputDir = join(sandbox, "📥️input");
        mkdirSync(inputDir, { recursive: true });
        writeFileSync(join(inputDir, "present.png"), Buffer.from([0x89, 0x50, 0x4e, 0x47]));
        let middleware: ((req: import("node:http").IncomingMessage, res: import("node:http").ServerResponse, next: () => void) => void) | undefined;
        const plugin = staticDirVitePlugin(sandbox, { kind: "static-dir", route: "/fixture", root: "📥️input" })[0]!;
        plugin.configureServer?.({ middlewares: { use: (fn: typeof middleware) => { middleware = fn; } } } as never);
        expect(middleware).toBeDefined();
        const missingStatus = await new Promise<number>((resolvePromise) => {
          const response = { statusCode: 200, end() { resolvePromise(this.statusCode); } };
          middleware!({ url: "/fixture/missing.png" } as import("node:http").IncomingMessage, response as import("node:http").ServerResponse, () => resolvePromise(200));
        });
        expect(missingStatus).toBe(404);
      } finally {
        rmSync(sandbox, { recursive: true, force: true });
      }
    });
  });

  describe("resolveSemioAssetRoot", () => {
    it("resolves the merged asset package with fonts", () => {
      const root = resolveSemioAssetRoot(repoRoot);
      expect(root.endsWith(SEMIO_ASSET_ROOT.split("/").pop()!)).toBe(true);
      expect(existsSync(resolve(root, "🔤️fonts/🚀️anta/🏛️latin/📖️regular/🗜️compressed.woff2"))).toBe(true);
    });

    it("throws when fonts are missing", () => {
      expect(() => resolveSemioAssetRoot(resolve(repoRoot, ".🧬semio"))).toThrow(/Missing Semio asset root/);
    });
  });

  describe("playgroundPlayBootHtmlPlugin", () => {
    it("registers index html boot injection", () => {
      expect(playgroundPlayBootHtmlPlugin().name).toBe("playground-play-boot-html");
    });

    it("exposes inline appearance and reveal scripts", () => {
      expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).toContain("prefers-color-scheme");
      // 🌓️ The ONE document the OS shell writes — never `ui.chrome.appearance`, which nothing has
      // written since the shell moved to the event-sourced config lane (the fixture-driven law in
      // `🧪️tests/🧩️suite/🟦️.ts` is what states this; this line keeps the retired key out).
      expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).toContain("semio.os.config");
      expect(PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT).not.toContain("ui.chrome.appearance");
      expect(PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT).toContain("semio-play-styles");
      expect(PLAYGROUND_PLAY_BOOT_INLINE_STYLE).toContain("data-semio-styled");
      expect(PLAYGROUND_PLAY_BOOT_INLINE_STYLE).toContain("--ui-available-height");
      expect(PLAYGROUND_PLAY_BOOT_INLINE_STYLE).not.toContain("height:100%");
      expect(PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT).toContain("visualViewport");
    });

    it("exposes an inline theme bootstrap script replaying the persisted ui-preference log", () => {
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("os.config.ui-preferences");
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("setCustomTheme");
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).not.toContain("ui.chrome.theme.snapshot");
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("--color-");
      expect(PLAYGROUND_PLAY_BOOT_THEME_SCRIPT).toContain("dataset.uiTheme");
    });

    it("injects the theme script after the appearance script and before the stylesheet link", async () => {
      const hook = playgroundPlayBootHtmlPlugin().transformIndexHtml;
      if (typeof hook !== "object") throw new Error("playground play boot html hook must declare its order");
      const injected = await hook.handler("", { path: "/🌐️.html", filename: "🌐️.html" });
      if (typeof injected !== "object" || injected === null || Array.isArray(injected) || !("tags" in injected)) throw new Error("playground play boot html hook must return injected tags");
      const tags = injected.tags;
      const kinds = tags.map((tag) => (tag.children === PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT ? "appearance" : tag.children === PLAYGROUND_PLAY_BOOT_THEME_SCRIPT ? "theme" : tag.attrs && "href" in tag.attrs ? "stylesheet" : "other"));
      expect(kinds.indexOf("appearance")).toBeLessThan(kinds.indexOf("theme"));
      expect(kinds.indexOf("theme")).toBeLessThan(kinds.indexOf("stylesheet"));
    });
  });

  describe("rewriteSpaFallbackToEmojiEntry", () => {
    const entry = "/🌐️.html";
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
      expect(html).toContain(PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT);
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
      expect(svg).toBe(resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg"));
      expect(ico).toBe(resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/🪧️logos/🌐️favicon/🌘️dark-round/📏️size-32.ico"));
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
    const puzzle3dMeshSpec: MeshCollectionAssetSpecV1 = {
      kind: "mesh-collection",
      route: "/mesh",
      catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json",
    };

    it("points at metabolism and abbau-aufbau kit glbs plus shared placeholder", () => {
      expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️capsule_J.glb", MESH_DELIVERY_CATALOG).source))).toBe(true);
      expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️capsule-with-balcony_slash.glb", MESH_DELIVERY_CATALOG).source))).toBe(true);
      expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb", MESH_DELIVERY_CATALOG).source))).toBe(true);
      expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️placeholder.glb", MESH_DELIVERY_CATALOG).source))).toBe(true);
    });

    it("registers serve and build plugins named after the route", () => {
      const plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
      expect(plugins.map((plugin) => plugin.name)).toEqual(["mesh-collection-serve/mesh", "mesh-collection-build/mesh"]);
    });

    it("createAssetHttpServerV1 serves 🧊️base.glb as model/gltf-binary", async () => {
      const probe = createServer();
      await new Promise<void>((resolveListen) => probe.listen(0, "127.0.0.1", () => resolveListen()));
      const address = probe.address();
      if (!address || typeof address === "string") throw new Error("expected TCP address");
      const port = address.port;
      await new Promise<void>((resolveClose, reject) => probe.close((err) => (err ? reject(err) : resolveClose())));
      const server = createAssetHttpServerV1(repoRoot, port, [puzzle3dMeshSpec], ASSET_PROVIDERS_V1);
      try {
        const response = await fetch(`http://127.0.0.1:${port}${meshAssetTransportUrl("/mesh/🧊️base.glb", MESH_DELIVERY_CATALOG)}`);
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
    // ⏱️ `findWorkspacePackages` walks the whole repo tree: measured 14.4 s cold, 27 s under fleet load.
    // The first caller in the process pays it; the memo makes every later one free.
    it(
      "pins scene hosts and excludes workspace packages from optimizeDeps",
      () => {
        const config = createWorkspaceViteResolveConfig(repoRoot);
        expect(config.resolve?.dedupe).toContain("react");
        expect(config.resolve?.dedupe).toContain("three");
        expect(config.server?.fs?.allow).toContain(repoRoot);
        expect(config.optimizeDeps?.exclude).toContain("@semio-tech/flow-module-core");
      },
      90000,
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
      90000,
    );

    it("terminates on a self-referential symlink and never reads through it", () => {
      const root = mkdtempSync(join(tmpdir(), "semio-workspace-scan-"));
      try {
        const real = join(root, "📦️packages");
        mkdirSync(real, { recursive: true });
        writeFileSync(join(real, "package.json"), JSON.stringify({ name: "@semio-tech/scan-probe" }));
        const generated = join(root, "🗑️generated", "test-artifacts");
        mkdirSync(generated, { recursive: true });
        writeFileSync(join(generated, "package.json"), JSON.stringify({ name: "@semio-tech/scan-generated-output" }));
        symlinkSync(generated, join(generated, "linked-ancestor-publication-owner-0"));
        symlinkSync(real, join(root, "linked-packages"));
        expect(findWorkspacePackages(root)).toEqual(["@semio-tech/scan-probe"]);
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    });
  });

}
