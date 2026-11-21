import { describe, it, expect } from "bun:test";
import { readFileSync, realpathSync, mkdtempSync, writeFileSync, mkdirSync, rmSync, existsSync, renameSync } from "node:fs";
import { resolve, join, dirname } from "node:path";
import { assetPathFromRequest, assetTransportUrl, parseAssetDeliveryAuthority, SEMIO_ASSET_DIRECTORY, SEMIO_ASSET_ROUTE } from "@semio-tech/assets/resolver/delivery";
import { resolveSemioAssetRoot, semioAssetsVitePlugin, SEMIO_ASSET_ROOT } from "../../../🏗️builder/🌐️vite/🗃️assets/🟦️.ts";
import type { OwnedBuildMiddleware } from "@semio-tech/ui-react/build-tooling";
import { createServer } from "node:http";
import { EventEmitter, once } from "node:events";
import { createHash } from "node:crypto";
import { semioFaviconSources, semioFaviconSvgMarkup, semioFaviconVitePlugin, SEMIO_FAVICON_HEAD_HTML } from "../../../🏗️builder/🌐️vite/🔖️favicon/🟦️.ts";
import { tmpdir } from "node:os";
import { semioEmojiIndexHtmlVitePlugin, semioBrandHtmlVitePlugins } from "../../../🏗️builder/🌐️vite/🌐️site-html/🟦️.ts";
import { meshCollectionVitePlugin } from "../../../🏗️builder/🌐️vite/🗺️mesh-collection/🟦️.ts";
import { tileProxyVitePlugin } from "../../../🏗️builder/🌐️vite/🛰️tile-proxy/🟦️.ts";
import { staticDirVitePlugin, staticDirMountVitePlugins, TILE_PROXY_ASSET_PROVIDER_V1 } from "../../../🏗️builder/🌐️vite/🟦️.ts";
import { createAssetHttpServerV1 } from "@semio-tech/assets/resolver/dispatch";
import { PassThrough } from "node:stream";
import { serveFileWithValidatorsV1 } from "../../../🏗️builder/🌐️vite/📤️file-serving/🟦️.ts";
import { parseFontCatalog, fontCatalogSources, resolveFontSource, parseGoogleFontWoff2Map, resolveFontFaceUrl } from "../../../🔤️fonts/🟦️.ts";
import { ownedServer, paletteCss, repoRoot, uiCss } from "../🧰️suite-build-server/🟦️.ts";
import tileProxyFixture from "../../🧫️fixtures/🧪️tile-proxy-status.json";

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
    const native = readFileSync(resolve(import.meta.dir, "../../../../🎯️targets/🧊️wgpu/👆️cursor/🦀️.rs"), "utf8").split("pub fn semio_cursor_css")[1]!.split("pub fn apply_canvas_cursor")[0]!;
    const nativeUrls = [...native.matchAll(/url\(([^)]+)\)/g)].map(match => match[1]!);
    expect(cssUrls.length).toBe(42);
    expect(nativeUrls.length).toBe(27);
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
    const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
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
    plugin.configureServer!(ownedServer((value) => { handler = value; }));
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
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🌐️favicon-delivery.json"), "utf8"));
    const authority = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌐️favicon/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🧬️schema/🔣️.json"), "utf8"));
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(authority)).toBe(true);
    for (const invalid of [{ ...authority, svg: "favicon.svg" }, { ...authority, ico: "🖼️favicon.ico" }, { ...authority, extra: true }]) expect(validate(invalid)).toBe(false);
    const sources = semioFaviconSources(repoRoot);
    const expected = { svg: Buffer.from(semioFaviconSvgMarkup(sources.svg)!), ico: readFileSync(sources.ico) };
    for (const configure of ["configureServer", "configurePreviewServer"] as const) {
      let handler: OwnedBuildMiddleware | undefined;
      semioFaviconVitePlugin(repoRoot)[0]![configure]!(ownedServer((value) => { handler = value; }));
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
            expect(decodeURIComponent(url.pathname)).toBe(item.path ?? `/${authority[kind]}`);
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
      const visit = (node: import("parse5").DefaultTreeAdapterTypes.ChildNode) => {
        if ("tagName" in node && node.tagName === "link" && node.attrs.some(attribute => attribute.name === "rel" && attribute.value === "icon")) icons.push(node.attrs.find(attribute => attribute.name === "href")!.value);
        if ("childNodes" in node) node.childNodes.forEach(visit);
      };
      parse(markup).childNodes.forEach(visit);
      expect(icons.map(href => decodeURIComponent(new URL(href, "https://example.invalid").pathname))).toEqual([`/${authority.svg}`, `/${authority.ico}`]);
    }
  });
});

describe("declared HTML entry", () => {
  it("builds and serves the declared HTML entry and static-host aliases", async () => {
    const { build, preview } = await import("vite"), { parse } = await import("parse5");
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🏠️html-entry.json"), "utf8"));
    const sandbox = realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "html-entry-"))), output = join(sandbox, "📤️output");
    writeFileSync(join(sandbox, fixture.entry), fixture.html);
    const plugin = semioEmojiIndexHtmlVitePlugin(sandbox, fixture.entry);
    expect(await plugin.config!({}, { command: "build", mode: "production" })).toEqual({ build: { rollupOptions: { input: join(sandbox, fixture.entry) } } });
    const brand = semioBrandHtmlVitePlugins(sandbox, { windowTitle: fixture.title, logoSvg: "" }).find(item => item.name === "semio-brand-html")!;
    const transform = brand.transformIndexHtml;
    if (!transform || typeof transform === "function") throw new Error("Missing ordered brand HTML transform");
    expect(await transform.handler(fixture.html, { path: `/${fixture.entry}`, filename: join(sandbox, fixture.entry) })).toBe(fixture.html.replace("Neutral Entry", fixture.title));
    await build({ configFile: false, root: sandbox, publicDir: false, plugins: [plugin, brand] as import("vite").Plugin[], build: { outDir: output, emptyOutDir: false }, logLevel: "silent" });
    const server = await preview({ configFile: false, root: sandbox, plugins: [plugin as import("vite").Plugin], build: { outDir: output }, preview: { port: 0, host: "127.0.0.1", strictPort: false }, logLevel: "silent" });
    try {
      const address = server.httpServer.address();
      if (!address || typeof address === "string") throw new Error("Missing Vite preview address");
      for (const row of fixture.requests) {
        const response = await fetch(new URL(row.target, `http://127.0.0.1:${address.port}`), { headers: { accept: "text/html" } });
        expect(response.status).toBe(row.status);
        const markup = await response.text();
        expect(markup).toContain("runtime-kept");
        expect(markup).toContain(`<title>${fixture.title}</title>`);
        expect(parse(markup).childNodes.some(node => node.nodeName === "html")).toBe(true);
      }
      for (const path of fixture.requiredAliasFiles) {
        expect(readFileSync(join(output, path), "utf8")).toContain("runtime-kept");
        expect(readFileSync(join(output, path), "utf8")).toBe(readFileSync(join(output, fixture.entry), "utf8"));
      }
      expect(readFileSync(join(output, fixture.entry), "utf8")).toContain("runtime-kept");
    } finally { await server.close(); }
  });
});

describe("tile proxy upstream responses", () => {
  it.each(["vite", "native"] as const)("preserves definitive and transient HTTP status on %s", async (delivery) => {
    const fixture = tileProxyFixture;
    const sandbox = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "tile-status-"));
    let upstreamStatus = 200;
    let upstreamRequests = 0;
    let proxy: ReturnType<typeof createServer> | undefined;
    const upstream = createServer((_request, response) => {
      upstreamRequests++;
      response.writeHead(upstreamStatus, { "Set-Cookie": fixture.setCookie, "RateLimit-Remaining": fixture.rateLimit, "Retry-After": fixture.retryAfter });
      response.end(upstreamStatus === 200 ? fixture.tileBody : undefined);
    }).listen(0, "127.0.0.1");
    const origin = async (server: ReturnType<typeof createServer>): Promise<string> => {
      if (!server.listening) await once(server, "listening");
      const address = server.address();
      if (address === null || typeof address === "string") throw new Error("Expected test server TCP address");
      return `http://127.0.0.1:${address.port}`;
    };
    try {
      const tileSpec = { kind: "tile-proxy", route: "/tiles", cache: "cache", upstream: `${await origin(upstream)}/{z}/{x}/{y}.png`, userAgent: "Fixture/1" } as const;
      if (delivery === "vite") {
        let middleware: OwnedBuildMiddleware | undefined;
        tileProxyVitePlugin(sandbox, tileSpec)[0]!.configureServer!(ownedServer((value) => { middleware = value; }));
        proxy = createServer((request, response) => middleware!(request, response, () => response.writeHead(404).end())).listen(0, "127.0.0.1");
      } else {
        proxy = createAssetHttpServerV1(sandbox, 0, [tileSpec], [TILE_PROXY_ASSET_PROVIDER_V1]);
      }
      const url = `${await origin(proxy)}/tiles/${fixture.tile}`;
      const ask = async () => {
        const response = await fetch(url);
        expect(response.headers.get("Set-Cookie")).toBeNull();
        expect(response.headers.get("RateLimit-Remaining")).toBeNull();
        expect(response.headers.get("Retry-After")).toBeNull();
        return { status: response.status, cacheControl: response.headers.get("Cache-Control"), body: await response.text() };
      };
      for (const status of fixture.statuses) {
        upstreamStatus = status;
        expect(await ask()).toEqual({ status, cacheControl: status === 200 ? "no-cache" : "no-store", body: status === 200 ? fixture.tileBody : "" });
      }
      expect(upstreamRequests).toBe(fixture.statuses.length);
      expect(await ask()).toEqual({ status: 200, cacheControl: "no-cache", body: fixture.tileBody });
      expect(upstreamRequests).toBe(fixture.statuses.length);
    } finally {
      for (const server of [proxy, upstream]) {
        if (!server?.listening) continue;
        await new Promise<void>((done, fail) => server.close((error) => error ? fail(error) : done()));
      }
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it.each(tileProxyFixture.concurrent.cases)("shares or retires a cold tile when $name", async ({ cancelSibling, upstreamRequests: expectedRequests }) => {
    const fixture = tileProxyFixture;
    const sandbox = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "tile-cold-"));
    const cacheFile = join(sandbox, "cache", fixture.tile);
    const events = new EventEmitter();
    const finishUpstream = Promise.withResolvers<void>();
    const client = new AbortController();
    const siblingClient = new AbortController();
    let upstreamRequests = 0;
    let proxyRequests = 0;
    let proxy: ReturnType<typeof createServer> | undefined;
    const upstream = createServer((_request, response) => {
      upstreamRequests++;
      response.once("close", () => events.emit("upstreamClose"));
      response.write(fixture.concurrent.chunks[0]);
      events.emit("upstreamStarted");
      void finishUpstream.promise.then(() => response.end(fixture.concurrent.chunks[1]));
    }).listen(0, "127.0.0.1");
    const deadline = AbortSignal.timeout(3500);
    try {
      if (!upstream.listening) await once(upstream, "listening", { signal: deadline });
      const address = upstream.address();
      if (!address || typeof address === "string") throw new Error("Missing upstream TCP address");
      const spec = { kind: "tile-proxy", route: "/tiles", cache: "cache", upstream: `http://127.0.0.1:${address.port}/{z}/{x}/{y}.png`, userAgent: "Fixture/1" } as const;
      let middleware: OwnedBuildMiddleware | undefined;
      tileProxyVitePlugin(sandbox, spec)[0]!.configureServer!(ownedServer((value) => { middleware = value; }));
      proxy = createServer((request, response) => {
        middleware!(request, response, () => response.writeHead(404).end());
        if (++proxyRequests === 2) events.emit("siblingAdmitted");
      }).listen(0, "127.0.0.1");
      if (!proxy.listening) await once(proxy, "listening", { signal: deadline });
      const proxyAddress = proxy.address();
      if (!proxyAddress || typeof proxyAddress === "string") throw new Error("Missing proxy TCP address");
      const url = `http://127.0.0.1:${proxyAddress.port}/tiles/${fixture.tile}`;
      const started = once(events, "upstreamStarted", { signal: deadline });
      const first = fetch(url, { signal: AbortSignal.any([deadline, client.signal]) }).catch((error: unknown) => error);
      await started;
      const admitted = once(events, "siblingAdmitted", { signal: deadline });
      const sibling = fetch(url, { signal: AbortSignal.any([deadline, siblingClient.signal]) }).catch((error: unknown) => error);
      await admitted;
      client.abort();
      expect(await first).toMatchObject({ name: "AbortError" });
      expect(existsSync(cacheFile)).toBe(false);
      if (cancelSibling) {
        const closed = once(events, "upstreamClose", { signal: deadline });
        siblingClient.abort();
        expect(await sibling).toMatchObject({ name: "AbortError" });
        await closed;
        expect(upstreamRequests).toBe(1);
        expect(existsSync(cacheFile)).toBe(false);
      }
      finishUpstream.resolve();
      const response = cancelSibling ? await fetch(url, { signal: deadline }) : await sibling;
      expect(response).toBeInstanceOf(Response);
      if (!(response instanceof Response)) throw response;
      expect(response.status).toBe(200);
      const body = fixture.concurrent.chunks.join("");
      expect(await response.text()).toBe(body);
      expect(readFileSync(cacheFile, "utf8")).toBe(body);
      expect(upstreamRequests).toBe(expectedRequests);
      expect(await (await fetch(url, { signal: deadline })).text()).toBe(body);
      expect(upstreamRequests).toBe(expectedRequests);
    } finally {
      client.abort();
      siblingClient.abort();
      finishUpstream.resolve();
      for (const server of [proxy, upstream]) {
        if (server?.listening) await new Promise<void>((done, fail) => server.close((error) => error ? fail(error) : done()));
      }
      rmSync(sandbox, { recursive: true, force: true });
    }
  }, 5000);
});

describe("build output write authority", () => {
  it("preserves every retained byte in all seven adapters and writes only declared outputs", async () => {
    const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🛡️build-writes.json"), "utf8"));
    const { default: glob } = await import("fast-glob"), failures: string[] = [];
    const sandbox = mkdtempSync(join(tmpdir(), "semio-build-write-"));
    const put = (path: string, content: string) => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content); };
    const snapshot = async (root: string) => Object.fromEntries(await Promise.all((await glob("**/*", { cwd: root, onlyFiles: true, dot: true })).sort().map(async path => [path, createHash("sha256").update(readFileSync(resolve(root, path))).digest("hex")])));
    try {
      put(resolve(sandbox, "📥️input/🧊️model.glb"), fixture.payload);
      put(resolve(sandbox, "📥️input/🖼️icon.ico"), fixture.payload);
      put(resolve(sandbox, SEMIO_ASSET_ROOT, "🔤️fonts/🔤️font.ttf"), fixture.payload);
      put(resolve(sandbox, "tiles/0/0/0.png"), fixture.payload);
      for (const path of Object.values(fixture.tiles) as string[]) put(resolve(sandbox, path), fixture.payload);
      put(resolve(sandbox, "📇️catalog.json"), JSON.stringify({ $schema: "../🧬️schema/🔣️.json", version: 1, collections: [], entries: [{ url: "/mesh/🧊️model.glb", source: "📥️input/🧊️model.glb", path: "🧊️model.glb" }] }));
      for (const mode of fixture.modes) {
        const brand = semioBrandHtmlVitePlugins(sandbox, { windowTitle: "fixture", logoSvg: fixture.markup, faviconIcoPath: "📥️input/🖼️icon.ico", cnameHost: fixture.cname });
        const hooks = {
          mesh: { plugin: meshCollectionVitePlugin(sandbox, { kind: "mesh-collection", route: "/mesh", catalog: "📇️catalog.json" })[1]!, expected: { "mesh/🧊️model.glb": fixture.payload } },
          favicon: { plugin: brand.find(plugin => plugin.name === "semio-favicon-build")!, expected: { "🛡️favicon.svg": fixture.markup, "🔖️favicon.ico": fixture.payload, "favicon.svg": fixture.markup, "favicon.ico": fixture.payload } },
          markers: { plugin: brand.find(plugin => plugin.name === "static-deploy-markers")!, expected: { ".nojekyll": "", CNAME: fixture.cname + "\n" } },
          html: { plugin: semioEmojiIndexHtmlVitePlugin(sandbox), expected: { "🌐️.html": fixture.html, "index.html": fixture.html, "404.html": fixture.html } },
          assets: { plugin: semioAssetsVitePlugin(sandbox)[1]!, expected: { "🖼️assets/🔤️fonts/🔤️font.ttf": fixture.payload } },
          tile: { plugin: tileProxyVitePlugin(sandbox, { kind: "tile-proxy", route: "/tiles", cache: "tiles", upstream: "https://example.invalid/{z}/{x}/{y}.png", userAgent: "Fixture/1" }, "bundle")[1]!, expected: { "tiles/0/0/0.png": fixture.payload } },
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
          if (key === "tile" && mode.emits) {
            expect(existsSync(resolve(output, dirname(fixture.tiles.privateStage)))).toBe(false);
            for (const path of [fixture.tiles.retainedFile, fixture.tiles.retainedDirectory, fixture.tiles.retainedOutsideCoordinates]) {
              expect(readFileSync(resolve(output, path), "utf8")).toBe(fixture.payload);
            }
          }
          expect(readFileSync(resolve(output, "📌️retained.bin"), "utf8")).toBe(fixture.payload);
        }
      }
      expect(failures).toEqual([]);
    } finally { rmSync(sandbox, { recursive: true, force: true }); }
  });
});

describe("conditional delivery of dev-served files", () => {
  it("answers validators, 304 for a current tag or date, the file otherwise — as the `fresh` freshness judge decides", async () => {
    const root = resolve(import.meta.dir, "../../🧫️fixtures/🏷️conditional-delivery");
    const fixture = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
    const { default: Ajv2020 } = await import("ajv/dist/2020.js"), { default: fresh } = await import("fresh");
    const sandbox = realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "conditional-delivery-")));
    const filePath = join(sandbox, fixture.file.path);
    mkdirSync(dirname(filePath), { recursive: true });
    writeFileSync(filePath, fixture.file.content);
    const ask = (method: string, headers: Record<string, string>) => {
      const answer = { statusCode: 200, headers: {} as Record<string, string>, body: [] as Buffer[] };
      const done = new Promise<typeof answer>((resolveAnswer) => {
        const res = Object.assign(new PassThrough(), { statusCode: 200, setHeader(name: string, value: string) { answer.headers[name.toLowerCase()] = value; } });
        res.on("data", (chunk: Buffer) => answer.body.push(Buffer.from(chunk)));
        res.on("end", () => resolveAnswer({ ...answer, statusCode: res.statusCode }));
        serveFileWithValidatorsV1({ method, headers } as unknown as import("node:http").IncomingMessage, res as unknown as import("node:http").ServerResponse, filePath, fixture.file.contentType);
      });
      return done;
    };
    try {
      const first = await ask("GET", {});
      const etag = first.headers.etag!, lastModified = Date.parse(first.headers["last-modified"]!);
      expect(etag).toMatch(/^W\/"[0-9a-f]+-[0-9a-f]+"$/u);
      expect(first.headers["cache-control"]).toBe("no-cache");
      expect(first.headers["content-length"]).toBe(String(Buffer.byteLength(fixture.file.content)));
      expect(first.headers["content-type"]).toBe(fixture.file.contentType);
      const expand = (value: string): string => {
        if (value === "{etag}" || value.includes("{etag}")) return value.replace("{etag}", etag);
        if (value === "{etag:weakless}") return etag.replace(/^W\//u, "");
        const date = /^\{lastModified\}(?:([+-])(\d+)s)?$/u.exec(value);
        if (date) return new Date(lastModified + (date[1] === "-" ? -1 : 1) * Number(date[2] ?? 0) * 1000).toUTCString();
        return value;
      };
      let rewritten = false;
      for (const row of fixture.cases) {
        if (row.afterRewrite && !rewritten) {
          await new Promise((wait) => setTimeout(wait, 20));
          writeFileSync(filePath, fixture.rewrite);
          rewritten = true;
        }
        const headers = Object.fromEntries(Object.entries(row.headers as Record<string, string>).map(([name, value]) => [name, expand(value)]));
        const answer = await ask(row.method, headers);
        const body = Buffer.concat(answer.body).toString("utf8");
        expect({ name: row.name, status: answer.statusCode, body: body.length > 0 ? "full" : "none" }).toEqual({ name: row.name, status: row.status, body: row.body });
        if (row.body === "full") expect(body).toBe(rewritten ? fixture.rewrite : fixture.file.content);
        if (row.method === "GET") expect({ name: row.name, fresh: fresh(headers, { etag: answer.headers.etag, "last-modified": answer.headers["last-modified"] }) }).toEqual({ name: row.name, fresh: row.status === 304 });
      }
    } finally { rmSync(sandbox, { recursive: true, force: true }); }
  });

  it("releases the file handle when the client drops mid-download so the served directory can be renamed", async () => {
    const sandbox = realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "aborted-delivery-")));
    const live = join(sandbox, "live"), moved = join(sandbox, "moved");
    mkdirSync(live, { recursive: true });
    writeFileSync(join(live, "big.bin"), Buffer.alloc(8 * 1024 * 1024, 1));
    try {
      const res = Object.assign(new PassThrough({ highWaterMark: 1024 }), { statusCode: 200, setHeader() {} });
      serveFileWithValidatorsV1({ method: "GET", headers: {} } as unknown as import("node:http").IncomingMessage, res as unknown as import("node:http").ServerResponse, join(live, "big.bin"), undefined);
      await new Promise((wait) => setTimeout(wait, 50));
      res.destroy();
      await new Promise((wait) => setTimeout(wait, 50));
      renameSync(live, moved);
      expect(existsSync(join(moved, "big.bin"))).toBe(true);
    } finally { rmSync(sandbox, { recursive: true, force: true }); }
  });
});

describe("static-dir mount table", () => {
  it("serves nested and overlaid roots with native Vite publicDir parity and refuses a route claimed twice", async () => {
    const root = resolve(import.meta.dir, "../../🧫️fixtures/🗂️static-dir-mounts");
    const fixture = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
    const { default: Ajv } = await import("ajv"), { createServer: createViteServer } = await import("vite"), { default: glob } = await import("fast-glob");
    const sandbox = realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "static-dir-mounts-")));
    const put = (path: string, content: string) => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content); };
    const snapshot = async (tree: string) => Object.fromEntries((await glob("**/*", { cwd: tree, onlyFiles: true })).sort().map(path => [path, readFileSync(resolve(tree, path), "utf8")]));
    const specs = (mounts: { route: string; root: string }[]) => mounts.map(mount => ({ kind: "static-dir" as const, route: mount.route, root: mount.root }));
    const union = join(sandbox, "📤️union"), copied = "📤️copied";
    for (const mount of fixture.mounts) for (const [file, content] of Object.entries(mount.files as Record<string, string>)) {
      put(join(sandbox, mount.root, file), content);
      put(join(union, mount.route, file), content);
    }
    const plugins = staticDirMountVitePlugins(sandbox, specs(fixture.mounts));
    for (const plugin of plugins.filter(item => item.apply === "build")) { plugin.configResolved?.({ root: sandbox, build: { outDir: copied } }); await plugin.closeBundle?.(); }
    expect(await snapshot(join(sandbox, copied))).toEqual(await snapshot(union));
    const freePort = () => new Promise<number>((done, reject) => { const probe = createServer(); probe.once("error", reject); probe.listen(0, "127.0.0.1", () => { const address = probe.address(); probe.close(() => done(typeof address === "object" && address ? address.port : 0)); }); });
    const ours = await createViteServer({ configFile: false, root: sandbox, publicDir: false, plugins: plugins as import("vite").Plugin[], server: { host: "127.0.0.1", port: await freePort(), strictPort: true, watch: null }, optimizeDeps: { noDiscovery: true, include: [] }, logLevel: "silent" });
    const oracle = await createViteServer({ configFile: false, root: sandbox, publicDir: union, server: { host: "127.0.0.1", port: await freePort(), strictPort: true, watch: null }, optimizeDeps: { noDiscovery: true, include: [] }, logLevel: "silent" });
    try {
      await ours.listen(); await oracle.listen();
      const origin = (server: typeof ours) => { const address = server.httpServer!.address(); if (!address || typeof address === "string") throw new Error("Missing Vite address"); return `http://127.0.0.1:${address.port}`; };
      for (const request of fixture.requests) {
        const actual = await fetch(new URL(request.url, origin(ours)));
        expect({ url: request.url, status: actual.status }).toEqual({ url: request.url, status: request.status });
        if (request.status === 200) expect(await actual.text()).toBe(await (await fetch(new URL(request.url, origin(oracle)))).text());
        else expect(existsSync(join(union, decodeURIComponent(new URL(request.url, "https://example.invalid").pathname)))).toBe(false);
      }
    } finally { await ours.close(); await oracle.close(); rmSync(sandbox, { recursive: true, force: true }); }
    for (const refusal of fixture.refusals) expect(() => staticDirMountVitePlugins(sandbox, specs(refusal.mounts))).toThrow(refusal.error);
  }, 60_000);
});

describe("font source identity", () => {
  it("admits the catalog independently with JSON Schema and resolves all 219 exact binaries", async () => {
    const assetRoot = resolveSemioAssetRoot(repoRoot);
    const input = JSON.parse(readFileSync(resolve(assetRoot, "🔤️fonts/📇️catalog.json"), "utf8"));
    const schema = JSON.parse(readFileSync(resolve(assetRoot, "🔤️fonts/🧬️schema/🔣️.json"), "utf8"));
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
    const fixture = JSON.parse(readFileSync(resolve(resolveSemioAssetRoot(repoRoot), "🔤️fonts/🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
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
