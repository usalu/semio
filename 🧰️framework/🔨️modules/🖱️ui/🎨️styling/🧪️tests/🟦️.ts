import { describe, expect, it } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import {
  clearColorResolveCache,
  resolveColorHex,
  resolveColorRgba,
  resolveSemanticColorHex,
  resolveSpatialAxisColors,
  serializeCanvasThemeJson,
  syncSessionCanvasTheme,
  SPATIAL_AXIS_COLOR_REFS,
  STYLING_BOARD_PALETTES,
  STYLING_PRESENCE_PALETTES,
  STYLING_TOKENS,
  elementStateAttributes,
  elementStateHidden,
  resolveElementFillKind,
  resolveElementState,
} from "../📦️packages/🟦️typescript/🟦️.ts";
import { meshCollectionVitePlugin, resolveSemioAssetRoot, SEMIO_ASSET_ROOT, SEMIO_FAVICON_HEAD_HTML, semioAssetsVitePlugin, semioBrandHtmlVitePlugins, semioEmojiIndexHtmlVitePlugin, semioFaviconSources, semioFaviconSvgMarkup, semioFaviconVitePlugin, staticDirVitePlugin, tileProxyVitePlugin, type PlaygroundAssetSpec } from "../🟦️.ts";
import { fontCatalogSources, parseFontCatalog, parseGoogleFontWoff2Map, resolveFontFaceUrl, resolveFontSource } from "../📦️packages/🦀️rust/📜️script.ts";
import type { OwnedBuildMiddleware } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🏗️build-tooling.ts";
import { MESH_DELIVERY_CATALOG, parseMeshDeliveryCatalog, meshAssetTransportUrl, resolveMeshAsset } from "../../../🖼️assets/🥽️mesh/🟦️.ts";
import { assetPathFromRequest, assetTransportUrl, parseAssetDeliveryAuthority, SEMIO_ASSET_DIRECTORY, SEMIO_ASSET_ROUTE } from "../../../🖼️assets/🔍️resolver/🌐️delivery.ts";

const repoRoot = resolve(import.meta.dir, "../../../../..");
const uiCss = readFileSync(resolve(import.meta.dir, "../🖌️ui.css"), "utf8");
const paletteCss = readFileSync(resolve(import.meta.dir, "../🎨️palette.css"), "utf8");

describe("scrollbar selector", () => {
  it("uses the hovered owner for the WebKit thumb", () => {
    expect(uiCss).toContain("*:hover::-webkit-scrollbar-thumb {");
    expect(uiCss).not.toContain("*::-webkit-scrollbar:hover::-webkit-scrollbar-thumb");
  });
});

describe("shared asset delivery", () => {
  it("resolves every CSS and native cursor URL against its exact handpicked source", async () => {
    const { parse } = await import("postcss");
    const cssUrls: string[] = [];
    parse(uiCss).walkDecls(declaration => {
      if (declaration.prop.startsWith("--cursor-")) for (const match of declaration.value.matchAll(/url\("([^\"]+)"\)/g)) cssUrls.push(match[1]!);
    });
    const native = readFileSync(resolve(import.meta.dir, "../../📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🖱️cursor.rs"), "utf8").split("pub fn semio_cursor_css")[1]!.split("pub fn apply_canvas_cursor")[0]!;
    const nativeUrls = [...native.matchAll(/url\(([^)]+)\)/g)].map(match => match[1]!);
    expect(cssUrls.length).toBe(42);
    expect(nativeUrls.length).toBe(25);
    for (const url of [...cssUrls, ...nativeUrls]) {
      const path = assetPathFromRequest(url);
      expect(path).not.toBeNull();
      if (path === null) throw new Error(`Unregistered cursor URL: ${url}`);
      const source = readFileSync(resolve(resolveSemioAssetRoot(repoRoot), path), "utf8");
      expect(source).toContain("<svg");
      expect(decodeURIComponent(new URL(assetTransportUrl(path), "https://example.invalid").pathname)).toBe(url);
    }
  });

  it("agrees with JSON Schema and WHATWG URL on neutral publication cases", async () => {
    const root = resolve(resolveSemioAssetRoot(repoRoot), "🔍️resolver");
    const fixture = JSON.parse(readFileSync(resolve(root, "🧪️delivery-cases.json"), "utf8"));
    const schema = JSON.parse(readFileSync(resolve(root, "🧬️delivery.schema.json"), "utf8"));
    const authority = JSON.parse(readFileSync(resolve(root, "🚚️delivery.json"), "utf8"));
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(authority)).toBe(true);
    expect(parseAssetDeliveryAuthority(authority).directoryName).toBe(SEMIO_ASSET_DIRECTORY);
    for (const invalid of fixture.invalidAuthorities) {
      expect(validate(invalid)).toBe(false);
      expect(() => parseAssetDeliveryAuthority(invalid)).toThrow();
    }
    for (const item of fixture.requests) {
      expect(assetPathFromRequest(item.target)).toBe(item.path);
      if (item.path !== null) {
        expect(decodeURIComponent(new URL(item.target, "https://example.invalid").pathname)).toBe(`${SEMIO_ASSET_ROUTE}/${item.path}`);
        expect(assetPathFromRequest(assetTransportUrl(item.path))).toBe(item.path);
      }
    }
  });

  it("serves encoded handpicked asset routes with exact source bytes and rejects the old route", async () => {
    let handler: OwnedBuildMiddleware | undefined;
    const plugin = semioAssetsVitePlugin(repoRoot)[0]!;
    plugin.configureServer!({ middlewares: { use(value: OwnedBuildMiddleware) { handler = value; } } });
    const server = createServer((request, response) => handler!(request, response, () => { response.statusCode = 404; response.end(); }));
    try {
      await new Promise<void>(done => server.listen(0, "127.0.0.1", done));
      const address = server.address();
      if (!address || typeof address === "string") throw new Error("Missing test server address");
      const origin = `http://127.0.0.1:${address.port}`;
      for (const path of ["README.md", "🔤️fonts/🚀️anta/🏛️latin/📖️regular/🗜️compressed.woff2"]) {
        const response = await fetch(`${origin}/${encodeURIComponent("🖼️assets")}/${path.split("/").map(encodeURIComponent).join("/")}`);
        expect(response.status).toBe(200);
        const actual = new Uint8Array(await response.arrayBuffer());
        expect(createHash("sha256").update(actual).digest("hex")).toBe(createHash("sha256").update(readFileSync(resolve(resolveSemioAssetRoot(repoRoot), path))).digest("hex"));
      }
      expect((await fetch(`${origin}/asset/README.md`)).status).toBe(404);
    } finally {
      await new Promise<void>((done, reject) => server.close(error => error ? reject(error) : done()));
    }
  });
});

describe("favicon delivery", () => {
  it("serves only the exact handpicked browser-icon routes with unchanged payloads", async () => {
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🌐️favicon-delivery.json"), "utf8"));
    const authority = JSON.parse(readFileSync(resolve(import.meta.dir, "../🌐️favicon.json"), "utf8"));
    const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧬️favicon.schema.json"), "utf8"));
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(authority)).toBe(true);
    for (const invalid of [{ ...authority, svg: "favicon.svg" }, { ...authority, ico: "🖼️favicon.ico" }, { ...authority, extra: true }]) expect(validate(invalid)).toBe(false);
    const sources = semioFaviconSources(repoRoot);
    const expected = { svg: Buffer.from(semioFaviconSvgMarkup(sources.svg)!), ico: readFileSync(sources.ico) };
    for (const configure of ["configureServer", "configurePreviewServer"] as const) {
      let handler: OwnedBuildMiddleware | undefined;
      semioFaviconVitePlugin(repoRoot)[0]![configure]!({ middlewares: { use(value: OwnedBuildMiddleware) { handler = value; } } });
      const server = createServer((request, response) => handler!(request, response, () => { response.statusCode = 404; response.end(); }));
      try {
        await new Promise<void>(done => server.listen(0, "127.0.0.1", done));
        const address = server.address();
        if (!address || typeof address === "string") throw new Error("Missing test server address");
        for (const item of fixture.requests) {
          const url = new URL(item.target, `http://127.0.0.1:${address.port}`);
          const response = await fetch(url);
          expect(response.status).toBe(item.kind === null ? 404 : 200);
          if (item.kind !== null) {
            const kind = item.kind as keyof typeof expected;
            expect(decodeURIComponent(url.pathname)).toBe(`/${authority[kind]}`);
            expect(response.headers.get("content-type")).toBe(kind === "svg" ? "image/svg+xml" : "image/x-icon");
            expect(createHash("sha256").update(new Uint8Array(await response.arrayBuffer())).digest("hex")).toBe(createHash("sha256").update(expected[kind]).digest("hex"));
          }
        }
      } finally { await new Promise<void>((done, reject) => server.close(error => error ? reject(error) : done())); }
    }
    for (const kind of ["svg", "ico"] as const) expect(SEMIO_FAVICON_HEAD_HTML).toContain(`href="./${authority[kind]}"`);
    const { parse } = await import("parse5");
    for (const markup of [SEMIO_FAVICON_HEAD_HTML, ...fixture.documents.map((path: string) => readFileSync(resolve(repoRoot, path), "utf8"))]) {
      const icons: string[] = [];
      const visit = (node: ReturnType<typeof parse>["childNodes"][number]) => {
        if ("tagName" in node && node.tagName === "link" && node.attrs.some(attribute => attribute.name === "rel" && attribute.value === "icon")) icons.push(node.attrs.find(attribute => attribute.name === "href")!.value);
        if ("childNodes" in node) node.childNodes.forEach(visit);
      };
      parse(markup).childNodes.forEach(visit);
      expect(icons.map(href => decodeURIComponent(new URL(href, "https://example.invalid").pathname))).toEqual([`/${authority.svg}`, `/${authority.ico}`]);
    }
    console.log("[DEBUG] verified exact favicon identity, encoded HTTP and unchanged SVG/ICO content");
  });
});

describe("declared HTML entry", () => {
  it("builds and serves the declared HTML entry without publishing generic aliases", async () => {
    const { build, preview } = await import("vite"), { parse } = await import("parse5");
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🏠️html-entry.json"), "utf8"));
    const sandbox = realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "html-entry-"))), output = join(sandbox, "📤️output");
    writeFileSync(join(sandbox, fixture.entry), fixture.html);
    const plugin = semioEmojiIndexHtmlVitePlugin(sandbox, fixture.entry);
    await build({ configFile: false, root: sandbox, publicDir: false, plugins: [plugin as import("vite").Plugin], build: { outDir: output, emptyOutDir: false }, logLevel: "silent" });
    const server = await preview({ configFile: false, root: sandbox, plugins: [plugin as import("vite").Plugin], build: { outDir: output }, preview: { port: 0, host: "127.0.0.1", strictPort: false }, logLevel: "silent" });
    try {
      const address = server.httpServer.address();
      if (!address || typeof address === "string") throw new Error("Missing Vite preview address");
      for (const row of fixture.requests) {
        const response = await fetch(new URL(row.target, `http://127.0.0.1:${address.port}`), { headers: { accept: "text/html" } });
        expect(response.status).toBe(row.status);
        const markup = await response.text();
        expect(markup).toContain("runtime-kept");
        expect(parse(markup).childNodes.some(node => node.nodeName === "html")).toBe(true);
      }
      for (const path of fixture.forbiddenFiles) expect(existsSync(join(output, path))).toBe(false);
      expect(readFileSync(join(output, fixture.entry), "utf8")).toContain("runtime-kept");
      console.log("[DEBUG] actual Vite build and preview served only the declared HTML output");
    } finally { await server.close(); }
  });
});

describe("build output write authority", () => {
  it("preserves every retained byte in all seven adapters and writes only declared outputs", async () => {
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🛡️build-writes.json"), "utf8"));
    const { default: glob } = await import("fast-glob"), failures: string[] = [];
    const sandbox = mkdtempSync(join(tmpdir(), "semio-build-write-"));
    const put = (path: string, content: string) => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content); };
    const snapshot = async (root: string) => Object.fromEntries(await Promise.all((await glob("**/*", { cwd: root, onlyFiles: true, dot: true })).sort().map(async path => [path, createHash("sha256").update(readFileSync(resolve(root, path))).digest("hex")])));
    try {
      put(resolve(sandbox, "📥️input/🧊️model.glb"), fixture.payload);
      put(resolve(sandbox, "📥️input/🖼️icon.ico"), fixture.payload);
      put(resolve(sandbox, SEMIO_ASSET_ROOT, "🔤️fonts/🔤️font.ttf"), fixture.payload);
      put(resolve(sandbox, ".🧬semio/🗺️map/tiles/0/0/0.png"), fixture.payload);
      put(resolve(sandbox, "📇️catalog.json"), JSON.stringify({ $schema: "./🧬️catalog.schema.json", version: 1, collections: [], entries: [{ url: "/mesh/🧊️model.glb", source: "📥️input/🧊️model.glb", path: "🧊️model.glb" }] }));
      for (const mode of fixture.modes) {
        const brand = semioBrandHtmlVitePlugins(sandbox, { windowTitle: "fixture", logoSvg: fixture.markup, faviconIcoPath: "📥️input/🖼️icon.ico", cnameHost: fixture.cname });
        const hooks = {
          mesh: { plugin: meshCollectionVitePlugin(sandbox, { kind: "mesh-collection", route: "/mesh", catalog: "📇️catalog.json" })[1]!, expected: { "mesh/🧊️model.glb": fixture.payload } },
          favicon: { plugin: brand.find(plugin => plugin.name === "semio-favicon-build")!, expected: { "🛡️favicon.svg": fixture.markup, "🔖️favicon.ico": fixture.payload } },
          markers: { plugin: brand.find(plugin => plugin.name === "static-deploy-markers")!, expected: { ".nojekyll": "", CNAME: fixture.cname + "\n" } },
          html: { plugin: semioEmojiIndexHtmlVitePlugin(sandbox), expected: { "🌐️.html": fixture.html } },
          assets: { plugin: semioAssetsVitePlugin(sandbox)[1]!, expected: { "🖼️assets/🔤️fonts/🔤️font.ttf": fixture.payload } },
          tile: { plugin: tileProxyVitePlugin(sandbox, { kind: "tile-proxy", route: "/tiles", cache: "tiles", upstream: "https://example.invalid/{z}/{x}/{y}.png" }, "bundle")[1]!, expected: { "tiles/0/0/0.png": fixture.payload } },
          static: { plugin: staticDirVitePlugin(sandbox, { kind: "static-dir", route: "/fixture", root: "📥️input" })[1]!, expected: { "fixture/🧊️model.glb": fixture.payload, "fixture/🖼️icon.ico": fixture.payload } },
        };
        expect(Object.keys(hooks)).toEqual(fixture.hooks);
        for (const key of fixture.hooks as (keyof typeof hooks)[]) {
          const outDir = `📤️output/${String(mode.write)}/${key}`, output = resolve(sandbox, outDir), item = hooks[key];
          put(resolve(output, "🌐️.html"), fixture.html);
          put(resolve(output, "📌️retained.bin"), fixture.payload);
          const before = await snapshot(output);
          item.plugin.configResolved?.({ root: sandbox, build: { outDir, ...(mode.write === null ? {} : { write: mode.write }) } });
          await item.plugin.closeBundle?.();
          if (!mode.emits && JSON.stringify(await snapshot(output)) !== JSON.stringify(before)) failures.push(key);
          if (mode.emits) for (const [path, content] of Object.entries(item.expected)) expect(readFileSync(resolve(output, path), "utf8")).toBe(content);
          expect(readFileSync(resolve(output, "📌️retained.bin"), "utf8")).toBe(fixture.payload);
        }
      }
      expect(failures).toEqual([]);
      console.log("[DEBUG] verified seven build adapters across no-write, write and default modes");
    } finally { rmSync(sandbox, { recursive: true, force: true }); }
  });
});

describe("font source identity", () => {
  it("admits the catalog independently with JSON Schema and resolves all 219 exact binaries", async () => {
    const assetRoot = resolveSemioAssetRoot(repoRoot);
    const input = JSON.parse(readFileSync(resolve(assetRoot, "🔤️fonts/📇️catalog.json"), "utf8"));
    const schema = JSON.parse(readFileSync(resolve(assetRoot, "🔤️fonts/🧬️catalog.schema.json"), "utf8"));
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(input)).toBe(true);
    const catalog = parseFontCatalog(input);
    const sources = fontCatalogSources(catalog);
    expect(sources.length).toBe(219);
    expect(new Set(sources.map(source => source.path)).size).toBe(219);
    for (const source of sources) {
      expect(resolveFontSource(source.path, catalog)).toEqual(source);
      expect(existsSync(resolve(assetRoot, source.path))).toBe(true);
    }
    const invalid = structuredClone(input);
    invalid.families[0].directory = "anta";
    expect(validate(invalid)).toBe(false);
    expect(() => parseFontCatalog(invalid)).toThrow();
    expect(() => resolveFontSource("🔤️fonts/anta/latin.woff2", catalog)).toThrow();
  });

  it("resolves neutral explicit subset identities without reading handpicked filenames", async () => {
    const fixture = JSON.parse(readFileSync(resolve(resolveSemioAssetRoot(repoRoot), "🔤️fonts/🧪️tests/🔣️.json"), "utf8"));
    const parsed = parseGoogleFontWoff2Map(fixture.css);
    expect(Object.fromEntries(parsed)).toEqual(fixture.expectedSubsets);
    const { parse } = await import("postcss");
    const independent = new Map<string, string>();
    parse(fixture.css).walkAtRules("font-face", rule => {
      const previous = rule.prev();
      rule.walkDecls("src", declaration => {
        const url = declaration.value.match(/url\(([^)]+)\)/)?.[1];
        const subset = previous?.type === "comment" ? previous.text.trim() : url?.match(/\.(\d+)\.woff2$/)?.[1];
        if (url && subset) independent.set(subset, url);
      });
    });
    expect(Object.fromEntries(independent)).toEqual(fixture.expectedSubsets);
    for (const item of fixture.cases) expect(resolveFontFaceUrl(item.source, parsed) ?? null).toBe(item.expectedUrl);
  });
});

describe("palette asset urls", () => {
  it("every @font-face url in palette.css resolves under SEMIO_ASSET_ROOT", () => {
    const assetRoot = resolveSemioAssetRoot(repoRoot);
    const urls = [...paletteCss.matchAll(/url\("([^\"]+)"\)/g)].map((m) => m[1]!);
    expect(urls.length).toBeGreaterThan(0);
    for (const url of urls) {
      const rel = assetPathFromRequest(url);
      expect(rel).not.toBeNull();
      if (rel === null) throw new Error(`Unregistered asset URL: ${url}`);
      expect(existsSync(resolve(assetRoot, rel))).toBe(true);
    }
  });
});

describe("styling resolve", () => {
  it("selection fill uses accent with emphasized text color so muted gray stays readable", () => {
    expect(uiCss).toMatch(/::selection\s*\{\s*background-color:\s*var\(--accent\);\s*color:\s*var\(--border-emphasized-color\);/);
    expect(uiCss).toMatch(/::-moz-selection\s*\{\s*background-color:\s*var\(--accent\);\s*color:\s*var\(--border-emphasized-color\);/);
  });

  it("keeps panel-tab toggle dividers normal even when the active fill recolors other borders", () => {
    expect(uiCss).toMatch(/\[data-slot="panel-tabs"\] > \[data-slot="panel-tab-button"\]\s*\{\s*border-inline-end-color:\s*var\(--border-normal-color\) !important;/);
  });

  it("leaves flowing chips borderless while their silhouette owns the continuous outline", () => {
    expect(uiCss).toMatch(
      /\[data-window-silhouette-chip\],\s*\[data-window-silhouette-chip\] > \*\s*\{\s*border-width:\s*0 !important;\s*border-style:\s*none !important;\s*box-shadow:\s*none;/,
    );
    expect(uiCss).toMatch(
      /\[data-window-silhouette-chip\] > :is\([\s\S]*?\[data-slot="button-group"\][\s\S]*?\)\s*\{\s*-webkit-backdrop-filter:\s*none !important;\s*backdrop-filter:\s*none !important;\s*background-color:\s*transparent !important;/,
    );
  });

  it("keeps accessibility glass fallbacks scoped to painted regions while gaps stay cut out", () => {
    expect(uiCss).toMatch(
      /@supports not \(\(-webkit-backdrop-filter: blur\(1px\)\) or \(backdrop-filter: blur\(1px\)\)\) \{[\s\S]*?:is\(\.ui-glass, \.ui-veil\) \{\s*background-color: var\(--surface-bg\);/,
    );
    expect(uiCss).toMatch(
      /@media \(prefers-reduced-transparency: reduce\) \{[\s\S]*?:is\(\.ui-glass, \.ui-veil\) \{[\s\S]*?backdrop-filter: none;[\s\S]*?background-color: var\(--surface-bg\);[\s\S]*?\[data-window-silhouette-gap\] \{\s*background: transparent !important;\s*background-color: transparent !important;/,
    );
    expect(uiCss).toMatch(
      /@media \(forced-colors: active\) \{[\s\S]*?:is\(\.ui-glass, \.ui-veil\) \{[\s\S]*?background-color: Canvas;\s*color: CanvasText;[\s\S]*?\[data-window-silhouette-border\] path \{\s*stroke: CanvasText !important;[\s\S]*?\[data-window-silhouette-gap\] \{\s*background: transparent !important;\s*background-color: transparent !important;\s*forced-color-adjust: none;/,
    );
  });

  it("expands the clipped content plane without changing document or auto-size clearances", () => {
    expect(uiCss).toMatch(
      /\.window-silhouette-content-plane\s*\{\s*margin-block-start: calc\(-1 \* var\(--window-silhouette-top-clearance, 0px\)\);\s*margin-block-end: calc\(-1 \* var\(--window-silhouette-bottom-clearance, 0px\)\);\s*padding-block-start: var\(--window-silhouette-top-clearance, 0px\);\s*padding-block-end: var\(--window-silhouette-bottom-clearance, 0px\);/,
    );
    expect(uiCss).toMatch(
      /\.window-silhouette-content-plane:has\(\s*\[data-window-content-layout="edgeless"\],\s*\[data-slot="window-dead-line-scroll"\]\s*\)\s*\{\s*padding-block-start: 0;\s*padding-block-end: 0;/,
    );
  });

  it("resolveColorHex resolves palette var refs headlessly", () => {
    clearColorResolveCache();
    expect(resolveColorHex("var(--color-secondary)", "gray")).toBe("#34d1bf");
    expect(resolveSemanticColorHex("border-element-color", "gray")).toBe("#7b827d");
  });

  it("resolveSpatialAxisColors maps X/Y/Z to primary/secondary/tertiary permanently", () => {
    clearColorResolveCache();
    expect(resolveSpatialAxisColors()).toEqual({ x: "#ff344f", y: "#34d1bf", z: "#fa9500" });
    expect(SPATIAL_AXIS_COLOR_REFS).toEqual({ x: "var(--color-primary)", y: "var(--color-secondary)", z: "var(--color-tertiary)" });
  });

  it("resolveColorRgba returns byte tuple", () => {
    clearColorResolveCache();
    expect(resolveColorRgba("var(--color-gray)", "gray")).toEqual([123, 130, 125, 255]);
  });

  it("serializeCanvasThemeJson dark labelFill differs from light", () => {
    const light = JSON.parse(serializeCanvasThemeJson("light")) as { labelFill: number[] };
    const dark = JSON.parse(serializeCanvasThemeJson("dark")) as { labelFill: number[] };
    expect(light.labelFill).toEqual(STYLING_BOARD_PALETTES.light.labelFill);
    expect(dark.labelFill).toEqual(STYLING_BOARD_PALETTES.dark.labelFill);
    expect(dark.labelFill).not.toEqual(light.labelFill);
  });

  it("resolveColorHex foreground flips with html.dark appearance", () => {
    clearColorResolveCache();
    const previousDocument = globalThis.document;
    const classSet = new Set<string>();
    globalThis.document = {
      documentElement: {
        get className() {
          return [...classSet].join(" ");
        },
        set className(value: string) {
          classSet.clear();
          for (const part of value.split(/\s+/u).filter(Boolean)) classSet.add(part);
        },
        classList: {
          contains: (name: string) => classSet.has(name),
          add: (...names: string[]) => {
            for (const name of names) classSet.add(name);
          },
          remove: (...names: string[]) => {
            for (const name of names) classSet.delete(name);
          },
        },
      },
      createElement: () => {
        throw new Error("css probe unavailable in this test");
      },
    } as unknown as Document;
    try {
      document.documentElement.classList.remove("dark");
      clearColorResolveCache();
      const lightFg = resolveColorHex("var(--color-foreground)", "dark");
      document.documentElement.classList.add("dark");
      clearColorResolveCache();
      const darkFg = resolveColorHex("var(--color-foreground)", "light");
      expect(lightFg).toBe(STYLING_TOKENS.dark);
      expect(darkFg).toBe(STYLING_TOKENS.light);
      expect(darkFg).not.toBe(lightFg);
    } finally {
      globalThis.document = previousDocument;
      clearColorResolveCache();
    }
  });

  it("syncSessionCanvasTheme pushes serialized palette into a session", () => {
    const calls: string[] = [];
    syncSessionCanvasTheme({
      setCanvasThemeJson(json: string) {
        calls.push(json);
      },
    });
    expect(calls).toHaveLength(1);
    const parsed = JSON.parse(calls[0]!) as { labelFill: number[] };
    expect(parsed.labelFill).toEqual(STYLING_BOARD_PALETTES.light.labelFill);
  });
});

describe("nested mesh source identity", () => {
  it("preserves explicit neutral source ownership and nested output paths in HTTP and static output", async () => {
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🧊️mesh-collection.json"), "utf8"));
    const sandbox = mkdtempSync(join(tmpdir(), "semio-mesh-collection-"));
    let server: ReturnType<typeof createServer> | undefined;
    try {
      for (const root of fixture.roots) {
        for (const file of root.files) {
          const path = resolve(sandbox, root.path, file.path);
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, file.content);
        }
      }
      writeFileSync(resolve(sandbox, fixture.placeholder.path), fixture.placeholder.content);
      writeFileSync(resolve(sandbox, "📇️catalog.json"), JSON.stringify(fixture.catalog));
      const { default: glob } = await import("fast-glob");
      const independent = new Map<string, string>();
      for (const root of fixture.roots) {
        for (const path of await glob("**/*.glb", { cwd: resolve(sandbox, root.path), onlyFiles: true })) {
          if (!independent.has(path)) independent.set(path, readFileSync(resolve(sandbox, root.path, path), "utf8"));
        }
      }
      independent.set(fixture.placeholder.path, fixture.placeholder.content);
      const expected = new Map<string, string>(fixture.expected.map((file: { path: string; content: string }) => [file.path, file.content]));
      expect(independent).toEqual(expected);
      const plugins = meshCollectionVitePlugin(sandbox, {
        kind: "mesh-collection", route: "/mesh", catalog: "📇️catalog.json",
      });
      const catalog = parseMeshDeliveryCatalog(fixture.catalog, () => { throw new Error("Unexpected source catalog"); });
      let handler: OwnedBuildMiddleware | undefined;
      plugins[0]!.configureServer!({ middlewares: { use(value) { handler = value; } } });
      server = createServer((request, response) => handler!(request, response, () => { response.statusCode = 404; response.end(); }));
      await new Promise<void>(done => server!.listen(0, "127.0.0.1", done));
      const address = server.address();
      if (!address || typeof address === "string") throw new Error("Mesh test server has no TCP address");
      for (const entry of catalog) {
        const response = await fetch(`http://127.0.0.1:${address.port}${meshAssetTransportUrl(entry.url, catalog)}`);
        expect(response.status).toBe(200);
        expect(response.headers.get("content-type")).toBe("model/gltf-binary");
        expect(await response.text()).toBe(expected.get(entry.path));
      }
      expect((await fetch(`http://127.0.0.1:${address.port}/mesh/unregistered.glb`)).status).toBe(404);
      expect((await fetch(`http://127.0.0.1:${address.port}/mesh/💊️capsules/🪝️j/📐️source.3dm`)).status).toBe(404);
      plugins[1]!.configResolved!({ root: sandbox, build: { outDir: "output" } });
      plugins[1]!.closeBundle!();
      const output = resolve(sandbox, "output/mesh");
      expect((await glob("**/*", { cwd: output, onlyFiles: true })).sort()).toEqual([...expected.keys()].sort());
      for (const [path, content] of expected) expect(readFileSync(resolve(output, path), "utf8")).toBe(content);
    } finally {
      if (server) await new Promise<void>((done, reject) => server!.close(error => error ? reject(error) : done()));
      rmSync(sandbox, { recursive: true, force: true });
    }
  });
});

describe("puzzle3d mesh-collection asset spec", () => {
  const puzzle3dMeshSpec: Extract<PlaygroundAssetSpec, { kind: "mesh-collection" }> = {
    kind: "mesh-collection",
    route: "/mesh",
    catalog: "🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json",
  };

  it("delivers every current public mesh identity byte-identically through dev and nested static output", async () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-current-mesh-"));
    const plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
    let handler: OwnedBuildMiddleware | undefined;
    plugins[0]!.configureServer!({ middlewares: { use(value) { handler = value; } } });
    const server = createServer((request, response) => handler!(request, response, () => { response.statusCode = 404; response.end(); }));
    try {
      await new Promise<void>(done => server.listen(0, "127.0.0.1", done));
      const address = server.address();
      if (!address || typeof address === "string") throw new Error("Mesh test server has no TCP address");
      plugins[1]!.configResolved!({ root: sandbox, build: { outDir: "output" } });
      plugins[1]!.closeBundle!();
      const hash = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
      for (const entry of MESH_DELIVERY_CATALOG) {
        const expected = hash(readFileSync(resolve(repoRoot, entry.source)));
        const response = await fetch(`http://127.0.0.1:${address.port}${meshAssetTransportUrl(entry.url)}`);
        expect(response.status).toBe(200);
        expect(response.headers.get("content-type")).toBe("model/gltf-binary");
        expect(hash(new Uint8Array(await response.arrayBuffer()))).toBe(expected);
        expect(hash(readFileSync(resolve(sandbox, "output/mesh", entry.path)))).toBe(expected);
      }
      const { default: glob } = await import("fast-glob");
      expect((await glob("**/*", { cwd: resolve(sandbox, "output/mesh"), onlyFiles: true })).sort()).toEqual(MESH_DELIVERY_CATALOG.map(entry => entry.path).sort());
      expect(MESH_DELIVERY_CATALOG).toHaveLength(93);
      expect(() => meshAssetTransportUrl("/mesh/🧊️ellipsoid-🧊️capsule_J.glb")).toThrow();
    } finally {
      await new Promise<void>((done, reject) => server.close(error => error ? reject(error) : done()));
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it("resolves kit glb roots and shared placeholder", () => {
    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️capsule_J.glb").source))).toBe(true);
    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️placeholder.glb").source))).toBe(true);
  });

  it("registers a generic mesh-collection serve/build program pair", () => {
    const plugins = meshCollectionVitePlugin(repoRoot, puzzle3dMeshSpec);
    expect(plugins.map((plugin) => plugin.name)).toEqual(["mesh-collection-serve/mesh", "mesh-collection-build/mesh"]);
  });

  it("includes 🧊️base.glb for shooting's default fixture", () => {
    expect(existsSync(resolve(repoRoot, resolveMeshAsset("/mesh/🧊️base.glb").source))).toBe(true);
  });
});

describe("elementState", () => {
  it("resolveElementState defaults every axis to inert", () => {
    expect(resolveElementState()).toEqual({ state: "normal", status: "idle", hover: false, selected: false });
    expect(resolveElementState({ selected: true })).toEqual({ state: "normal", status: "idle", hover: false, selected: true });
  });

  it("elementStateHidden is true only for state === hidden", () => {
    expect(elementStateHidden({ state: "hidden" })).toBe(true);
    for (const state of ["introducing", "celebrating", "previewed", "normal", "disabled"] as const) {
      expect(elementStateHidden({ state })).toBe(false);
    }
  });

  it("elementStateAttributes omits every axis at default", () => {
    expect(elementStateAttributes(resolveElementState())).toEqual({});
  });

  it("elementStateAttributes returns {} for hidden regardless of other axes", () => {
    expect(elementStateAttributes({ state: "hidden", status: "loading", hover: true, selected: true })).toEqual({});
  });

  it("elementStateAttributes stamps data-ui-state plus data-introduced for introducing", () => {
    expect(elementStateAttributes(resolveElementState({ state: "introducing" }))).toEqual({
      "data-ui-state": "introducing",
      "data-introduced": "true",
    });
  });

  it("elementStateAttributes stamps data-ui-state plus data-celebrated for celebrating", () => {
    expect(elementStateAttributes(resolveElementState({ state: "celebrating" }))).toEqual({
      "data-ui-state": "celebrating",
      "data-celebrated": "true",
    });
  });

  it("elementStateAttributes stamps data-ui-state for previewed/disabled without data-introduced", () => {
    expect(elementStateAttributes(resolveElementState({ state: "previewed" }))).toEqual({ "data-ui-state": "previewed" });
    expect(elementStateAttributes(resolveElementState({ state: "disabled" }))).toEqual({ "data-ui-state": "disabled" });
  });

  it("elementStateAttributes stamps data-ui-status for non-idle status", () => {
    expect(elementStateAttributes(resolveElementState({ status: "loading" }))).toEqual({ "data-ui-status": "loading" });
    expect(elementStateAttributes(resolveElementState({ status: "waiting" }))).toEqual({ "data-ui-status": "waiting" });
    expect(elementStateAttributes(resolveElementState({ status: "finished" }))).toEqual({ "data-ui-status": "finished" });
  });

  it("elementStateAttributes stamps data-ui-hover and data-ui-selected independently", () => {
    expect(elementStateAttributes(resolveElementState({ hover: true }))).toEqual({ "data-ui-hover": "true" });
    expect(elementStateAttributes(resolveElementState({ selected: true }))).toEqual({ "data-ui-selected": "true" });
  });

  it("elementStateAttributes composes all four axes simultaneously", () => {
    expect(elementStateAttributes(resolveElementState({ state: "previewed", status: "waiting", hover: true, selected: true }))).toEqual({
      "data-ui-state": "previewed",
      "data-ui-status": "waiting",
      "data-ui-hover": "true",
      "data-ui-selected": "true",
    });
  });

  it("resolveElementFillKind follows disabled > celebrated > selected > previewed > hovered > neutral precedence", () => {
    expect(resolveElementFillKind(resolveElementState())).toBe("neutral");
    expect(resolveElementFillKind(resolveElementState({ hover: true }))).toBe("hovered");
    expect(resolveElementFillKind(resolveElementState({ state: "previewed" }))).toBe("previewed");
    expect(resolveElementFillKind(resolveElementState({ state: "previewed", hover: true }))).toBe("previewed");
    expect(resolveElementFillKind(resolveElementState({ selected: true, hover: true }))).toBe("selected");
    expect(resolveElementFillKind(resolveElementState({ selected: true, state: "previewed" }))).toBe("selected");
    expect(resolveElementFillKind(resolveElementState({ state: "celebrating", selected: true }))).toBe("celebrated");
    expect(resolveElementFillKind(resolveElementState({ state: "disabled", selected: true }))).toBe("disabled");
  });

  it("resolveElementFillKind returns null for hidden", () => {
    expect(resolveElementFillKind({ state: "hidden", status: "idle", hover: true, selected: true })).toBeNull();
  });
});

//#region 👥️Presence
// 🎨️ Accessibility guarantee for the session-color wheel (contract freeze §C7.5): each of the 12 base
// (`k=0`) presence swatches must read against its appearance's base surface, and neighboring wheel
// entries (the pair an actor sees when two peers join back-to-back and get consecutive palette
// indices) must stay perceptually distinct. All-pairs oklab separation across 12 hues spanning a full
// 360° wheel is not achievable at any (s, l) — oklab compresses the green/yellow-green arc far more
// than red/blue/purple, so hues 90/120/150 cap out near ΔE 0.07 regardless of lightness/saturation —
// so this checks the pairwise metric the hub-assigned index sequence actually exercises: consecutive
// wheel neighbors (verified never below ΔE ≈0.19 at these tokens, comfortably past the 0.12 floor).
function hslToRgb01(h: number, s: number, l: number): readonly [number, number, number] {
  const hue = ((h % 360) + 360) % 360;
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((hue / 60) % 2) - 1));
  const m = l - c / 2;
  const sector = Math.floor(hue / 60) % 6;
  const [r1, g1, b1] = sector === 0 ? [c, x, 0] : sector === 1 ? [x, c, 0] : sector === 2 ? [0, c, x] : sector === 3 ? [0, x, c] : sector === 4 ? [x, 0, c] : [c, 0, x];
  return [r1 + m, g1 + m, b1 + m];
}

function srgbToLinear(c: number): number {
  return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
}

function relativeLuminance01([r, g, b]: readonly [number, number, number]): number {
  return 0.2126 * srgbToLinear(r) + 0.7152 * srgbToLinear(g) + 0.0722 * srgbToLinear(b);
}

function contrastRatio01(a: readonly [number, number, number], b: readonly [number, number, number]): number {
  const la = relativeLuminance01(a);
  const lb = relativeLuminance01(b);
  const lighter = Math.max(la, lb);
  const darker = Math.min(la, lb);
  return (lighter + 0.05) / (darker + 0.05);
}

function hexToRgb01(hex: string): readonly [number, number, number] {
  const v = hex.replace(/^#/u, "");
  return [Number.parseInt(v.slice(0, 2), 16) / 255, Number.parseInt(v.slice(2, 4), 16) / 255, Number.parseInt(v.slice(4, 6), 16) / 255];
}

function linearToOklab([r, g, b]: readonly [number, number, number]): readonly [number, number, number] {
  const l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
  const m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
  const s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
  const l_ = Math.cbrt(l);
  const m_ = Math.cbrt(m);
  const s_ = Math.cbrt(s);
  return [0.2104542553 * l_ + 0.793617785 * m_ - 0.0040720468 * s_, 1.9779984951 * l_ - 2.428592205 * m_ + 0.4505937099 * s_, 0.0259040371 * l_ + 0.7827717662 * m_ - 0.808675766 * s_];
}

function oklabOfHsl(h: number, s: number, l: number): readonly [number, number, number] {
  const [r, g, b] = hslToRgb01(h, s, l);
  return linearToOklab([srgbToLinear(r), srgbToLinear(g), srgbToLinear(b)]);
}

function oklabDeltaE(a: readonly [number, number, number], b: readonly [number, number, number]): number {
  return Math.sqrt((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2);
}

describe("presence palette", () => {
  it("carries 12 hues plus a light/dark s/l pair for both appearances", () => {
    expect(STYLING_PRESENCE_PALETTES.hues).toHaveLength(12);
    expect(STYLING_PRESENCE_PALETTES.light).toEqual({ s: expect.any(Number), l: expect.any(Number) });
    expect(STYLING_PRESENCE_PALETTES.dark).toEqual({ s: expect.any(Number), l: expect.any(Number) });
  });

  it("every base-cycle swatch clears 3:1 contrast against its appearance's base surface", () => {
    const bases = { light: hexToRgb01(STYLING_TOKENS.light), dark: hexToRgb01(STYLING_TOKENS.dark) } as const;
    for (const appearance of ["light", "dark"] as const) {
      const { s, l } = STYLING_PRESENCE_PALETTES[appearance];
      for (const h of STYLING_PRESENCE_PALETTES.hues) {
        const ratio = contrastRatio01(hslToRgb01(h, s, l), bases[appearance]);
        expect(ratio, `${appearance} h=${h} contrast`).toBeGreaterThanOrEqual(3);
      }
    }
  });

  it("consecutive wheel neighbors clear ΔE >= 0.12 in oklab, both appearances", () => {
    for (const appearance of ["light", "dark"] as const) {
      const { s, l } = STYLING_PRESENCE_PALETTES[appearance];
      const hues = STYLING_PRESENCE_PALETTES.hues;
      const oklabs = hues.map((h) => oklabOfHsl(h, s, l));
      for (let i = 0; i < oklabs.length; i++) {
        const de = oklabDeltaE(oklabs[i]!, oklabs[(i + 1) % oklabs.length]!);
        expect(de, `${appearance} neighbor ${hues[i]} vs ${hues[(i + 1) % hues.length]}`).toBeGreaterThanOrEqual(0.12);
      }
    }
  });
});
//#endregion 👥️Presence
