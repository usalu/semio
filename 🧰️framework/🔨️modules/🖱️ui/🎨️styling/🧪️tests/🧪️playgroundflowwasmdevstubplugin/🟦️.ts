type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { GIS_MAP_DEFAULT_PREFETCH_BOUNDS, PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT, PLAYGROUND_PLAY_BOOT_INLINE_STYLE, PLAYGROUND_PLAY_BOOT_REVEAL_SCRIPT, PLAYGROUND_PLAY_BOOT_THEME_SCRIPT, PLAYGROUND_WASM_STUB_PREFIX, SEMIO_ASSET_ROOT, SEMIO_FAVICON_HEAD_HTML, contentTypeForStaticDirAsset, createServer, createWorkspaceViteResolveConfig, existsSync, fileURLToPath, findWorkspacePackages, isPlaygroundOptimizedDepUrl, playgroundOptimizedDepUrlPrefix, listMapTilesForBounds, mapTileCacheRoots, meshAssetTransportUrl, meshCollectionVitePlugin, mkdirSync, playgroundAssetVitePlugins, playgroundFlowWasmDevStubPlugin, playgroundPlayBootHtmlPlugin, playgroundSceneHostResolveAliases, playgroundWasmStubKey, prefetchMapTiles, resolve, resolveGisMapTileServeMode, resolveMeshAsset, resolveSemioAssetRoot, rewriteSpaFallbackToEmojiEntry, semioFaviconSources, semioFaviconSvgMarkup, semioFaviconVitePlugin, semioHostHtmlString, semioHostHtmlVitePlugin, startAssetServer, statusSurfaceHtml, tileProxyVitePlugin, writeFileSync } = dependencies;
  type PlaygroundAssetSpec = any;

  const { describe, expect, it } = vitest;
  const repoRoot = resolve(fileURLToPath(new URL(".", source.url)), "../../../../../..");

  describe("playgroundFlowWasmDevStubPlugin", () => {
    const importer = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx");
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
      expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/⛏️sourcing/sourcing_plugin.js")).toBe("text/javascript");
      expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js")).toBe("text/javascript");
      expect(contentTypeForStaticDirAsset("/🔌️plugin-modules/🧩️puzzle/🕸️puzzle_plugin.wasm")).toBe("application/wasm");
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
    const puzzle3dMeshSpec: Extract<PlaygroundAssetSpec, { kind: "mesh-collection" }> = {
      kind: "mesh-collection",
      route: "/mesh",
      catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json",
    };

    it("points at metabolism and abbau-aufbau kit glbs plus shared placeholder", () => {
      expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️capsule_J.glb").source))).toBe(true);
      expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️capsule-with-balcony_slash.glb").source))).toBe(true);
      expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb").source))).toBe(true);
      expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️placeholder.glb").source))).toBe(true);
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
        const response = await fetch(`http://127.0.0.1:${port}${meshAssetTransportUrl("/mesh/🧊️base.glb")}`);
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
