import corpus from "./🧫️fixtures/🔣️.json";
import { mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { runInNewContext } from "node:vm";
import { URL as OracleURL } from "whatwg-url";
import { Request as OracleRequest } from "undici";
import { globSync } from "glob";
import { init, parse } from "es-module-lexer";
import CachePolicy from "http-cache-semantics";

export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: {
    assignPlayPages: (entries: readonly { name: string; bytes: number }[], apex: string, budget?: number) => { pages: readonly { name: string; bytes: number; host: string }[]; origins: Readonly<Record<string, string>> };
    playPageOrigins: (apex: string) => Record<string, string>;
    publishPlayPages: (siteDir: string, pagesDir: string, apex: string, budget?: number) => readonly { name: string; bytes: number }[];
    PLAY_PAGE_BUDGET_BYTES: number;
  },
): Promise<void> {
  const { assignPlayPages, playPageOrigins, publishPlayPages, PLAY_PAGE_BUDGET_BYTES } = dependencies;
  const { describe, expect, it } = vitest;
  const { shardWorkerUrl } = await import("../../../../🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts");
  const { publishedPageUrl, relocatePublishedRequestUrl } = await import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts");

  describe("play CDN pages", () => {
    const origins = playPageOrigins("play.semio-tech.com");
    for (const row of corpus.cases) it(`relocates ${row.id}`, () => {
      expect(relocatePublishedRequestUrl(row.input, corpus.pageOrigin, origins)).toBe(row.output);
      if (new OracleURL(row.input, corpus.pageOrigin).origin !== corpus.pageOrigin || row.id === "malformed-encoding") return;
      const expected = new OracleURL(row.output, corpus.pageOrigin).href;
      const url = new OracleURL(row.input, corpus.pageOrigin).href;
      expect(new OracleURL(relocatePublishedRequestUrl(url, corpus.pageOrigin, origins), corpus.pageOrigin).href).toBe(expected);
      expect(new OracleURL(relocatePublishedRequestUrl(new OracleRequest(url).url, corpus.pageOrigin, origins), corpus.pageOrigin).href).toBe(expected);
    });

    it("publishes a worker that relocates native requests and imports diagnostics from the module satellite", async () => {
      const root = resolve(import.meta.dirname, "../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/publication-worker-test");
      const workerPath = corpus.worker.path;
      const shimPath = corpus.worker.shimPath;
      const site = join(root, "site"), pages = join(root, "pages");
      rmSync(root, { recursive: true, force: true });
      mkdirSync(dirname(join(site, workerPath)), { recursive: true });
      mkdirSync(dirname(join(site, shimPath)), { recursive: true });
      writeFileSync(join(site, "index.html"), "<html></html>");
      writeFileSync(join(site, workerPath), corpus.worker.source);
      writeFileSync(join(site, shimPath), "export const _setEnv = () => {};");
      try {
        const published = publishPlayPages(site, pages, "play.semio-tech.com");
        for (const page of published) {
          const directory = join(pages, page.name);
          const bytes = globSync("**/*", { cwd: directory, dot: true, nodir: true }).reduce((sum, name) => sum + statSync(join(directory, name)).size, 0);
          expect(page.bytes).toBe(bytes);
        }
        const worker = readFileSync(join(pages, "play", workerPath), "utf8");
        expect(worker).toBe(readFileSync(join(pages, "modules", workerPath), "utf8"));
        const imports = worker.match(/import\([^)]*"([^"]+)"\)/u);
        expect(new OracleURL(`https://modules.assets.semio-tech.com/${shimPath}`).href).toBe(corpus.worker.importUrl);
        expect(imports?.[1]).toBe(corpus.worker.importUrl);
        const requests: { input: string | URL | Request; init?: RequestInit }[] = [];
        const scope = { self: { location: { origin: corpus.pageOrigin } }, URL, Request, fetch: async (input: string | URL | Request, init?: RequestInit) => { requests.push({ input, init }); return new Response(); } };
        runInNewContext(worker, scope);
        for (const row of corpus.cases) {
          if (row.id === "malformed-encoding") continue;
          await scope.fetch(row.input);
          expect(requests.at(-1)?.input).toBe(row.output);
          const url = new URL(row.input, corpus.pageOrigin);
          const expected = new OracleURL(row.output, corpus.pageOrigin).href;
          await scope.fetch(url);
          expect(new URL(String(requests.at(-1)?.input), corpus.pageOrigin).href).toBe(expected);
          const request = new Request(url, { method: "HEAD", headers: { "X-Semio-Test": row.id }, credentials: "include" });
          await scope.fetch(request);
          const received = requests.at(-1)?.input as Request;
          expect(received.url).toBe(expected);
          expect(received.method).toBe("HEAD");
          expect(received.headers.get("X-Semio-Test")).toBe(row.id);
          expect(received.credentials).toBe("include");
          const post = new Request(url, { method: "POST", body: "content" });
          await scope.fetch(post);
          expect(requests.at(-1)?.input).toBe(post);
        }
        console.log("[DEBUG] Published worker URL/Request routing and diagnostic import verified");
      } finally { rmSync(root, { recursive: true, force: true }); }
    });
    for (const row of corpus.publication.cases) it(`counts final publication bytes for ${row.id}`, () => {
      const root = resolve(import.meta.dirname, `../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/publication-budget-${row.id}`);
      const site = join(root, "site"), pages = join(root, "pages");
      mkdirSync(site, { recursive: true });
      for (const file of corpus.publication.files) writeFileSync(join(site, file.path), file.content);
      try {
        const run = () => publishPlayPages(site, pages, "play.semio-tech.com", row.budget);
        if (!row.accepted) { expect(run).toThrow(/CDN page limit/); return; }
        const published = run(), directory = join(pages, "play");
        const bytes = globSync("**/*", { cwd: directory, dot: true, nodir: true }).reduce((sum, name) => sum + statSync(join(directory, name)).size, 0);
        expect(published[0]!.bytes).toBe(bytes);
        expect(bytes).toBeLessThan(row.budget);
      } finally { rmSync(root, { recursive: true, force: true }); }
    });
    it("publishes every literal import in the actual generated shard worker and preserves activation expressions", async () => {
      const { shardWorkerSource } = await import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts");
      await init;
      const source = shardWorkerSource();
      const importsOf = (text: string) => parse(text)[0].map(row => ({ kind: row.d === -1 ? "static" : row.d === -2 ? "import.meta" : "dynamic", specifier: row.n ?? null, expression: text.slice(row.s, row.e) }));
      expect(importsOf(source)).toEqual(corpus.worker.generatedImports);
      const root = resolve(import.meta.dirname, "../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/publication-actual-worker-test");
      const site = join(root, "site"), pages = join(root, "pages");
      mkdirSync(dirname(join(site, corpus.worker.path)), { recursive: true });
      writeFileSync(join(site, "index.html"), "x");
      writeFileSync(join(site, corpus.worker.path), source);
      try {
        publishPlayPages(site, pages, "play.semio-tech.com");
        const worker = readFileSync(join(pages, "play", corpus.worker.path), "utf8");
        const imports = importsOf(worker);
        expect(imports.map(row => ({ kind: row.kind, specifier: row.specifier }))).toEqual(corpus.worker.generatedImports.map(row => ({ kind: row.kind, specifier: row.specifier === null ? null : corpus.worker.importUrl })));
        expect(imports.find(row => row.specifier === null)?.expression).toBe("moduleUrl");
        console.log("[DEBUG] Actual generated worker import forms and publication targets verified");
      } finally { rmSync(root, { recursive: true, force: true }); }
    });
    it("requires revalidation of stable assets on every published host", () => {
      const root = resolve(import.meta.dirname, "../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🗑️generated/publication-cache-test");
      const site = join(root, "site"), pages = join(root, "pages");
      for (const file of corpus.cache.files) { mkdirSync(dirname(join(site, file.path)), { recursive: true }); writeFileSync(join(site, file.path), file.content); }
      for (const file of corpus.cache.obsoleteFiles) { mkdirSync(dirname(join(pages, file)), { recursive: true }); writeFileSync(join(pages, file), "obsolete"); }
      try {
        publishPlayPages(site, pages, "play.semio-tech.com");
        const names = new Set(globSync("**/*", { cwd: pages, dot: true, nodir: true }));
        for (const file of corpus.cache.obsoleteFiles) expect(names.has(file)).toBe(false);
        for (const name of corpus.cache.pages) {
          const text = readFileSync(join(pages, name, "_headers"), "utf8");
          const headers = Object.fromEntries(text.split("\n").slice(1).filter(line => line.trim()).map(line => { const colon = line.indexOf(":"); return [line.slice(0, colon).trim().toLowerCase(), line.slice(colon + 1).trim()]; }));
          expect(headers["cache-control"]).toBe(corpus.cache.control);
          const request = { url: "https://" + (name === "play" ? "play.semio-tech.com" : name + ".assets.semio-tech.com") + "/stable.js", method: "GET", headers: {} };
          const previous = new CachePolicy(request, { status: 200, headers: corpus.cache.responseHeaders });
          expect(previous.satisfiesWithoutRevalidation(request)).toBe(true);
          const current = new CachePolicy(request, { status: 200, headers: { ...corpus.cache.responseHeaders, ...headers } });
          expect(current.satisfiesWithoutRevalidation(request)).toBe(corpus.cache.satisfiesWithoutRevalidation);
          if (name !== "play") expect(headers["access-control-allow-origin"]).toBe("*");
        }
        console.log("[DEBUG] Every published host requires stable asset cache revalidation");
      } finally { rmSync(root, { recursive: true, force: true }); }
    });
    const entries = [
      { name: "index.html", bytes: 4_000 },
      { name: "assets", bytes: 180_000_000 },
      { name: "🖼️assets", bytes: 175_000_000 },
      { name: "mesh", bytes: 23_000_000 },
      { name: "cad-assets", bytes: 2_000_000 },
      { name: "infinite-assets", bytes: 1_000_000 },
      { name: "🔌️plugin-modules", bytes: 620_000_000 },
      { name: "🧩️extension-modules", bytes: 90_000_000 },
      { name: "osm", bytes: 277_000_000 },
      { name: "vt", bytes: 329_000_000 },
      { name: "dem", bytes: 2_000_000 },
    ];

    it("keeps every page under 1GB", () => {
      const { pages } = assignPlayPages(entries, "play.semio-tech.com");
      expect(pages.map((page) => page.name)).toEqual(["play", "map", "media", "modules"]);
      for (const page of pages) expect(page.bytes).toBeLessThan(PLAY_PAGE_BUDGET_BYTES);
      expect(pages.find((page) => page.name === "map")?.host).toBe("map.assets.semio-tech.com");
      expect(pages.find((page) => page.name === "media")?.host).toBe("media.assets.semio-tech.com");
      expect(pages.find((page) => page.name === "modules")?.host).toBe("modules.assets.semio-tech.com");
      expect(pages.find((page) => page.name === "modules")?.bytes).toBe(710_000_000);
    });

    it("refuses a page that reaches 1GB", () => {
      expect(() => assignPlayPages([{ name: "assets", bytes: PLAY_PAGE_BUDGET_BYTES }], "play.semio-tech.com")).toThrow(/CDN page limit/);
    });

    it("prefixes satellite routes and leaves the app origin relative", () => {
      const origins = playPageOrigins("play.semio-tech.com");
      expect(publishedPageUrl("/osm/{z}/{x}/{y}.png", origins)).toBe("https://map.assets.semio-tech.com/osm/{z}/{x}/{y}.png");
      expect(publishedPageUrl("/mesh/a.glb", origins)).toBe("https://media.assets.semio-tech.com/mesh/a.glb");
      expect(publishedPageUrl("/cad-assets/a.3dm", origins)).toBe("https://media.assets.semio-tech.com/cad-assets/a.3dm");
      expect(publishedPageUrl("/infinite-assets/plan.jpg", origins)).toBe("https://media.assets.semio-tech.com/infinite-assets/plan.jpg");
      expect(publishedPageUrl("/🖼️assets/fonts/a.woff2", origins)).toBe("https://media.assets.semio-tech.com/🖼️assets/fonts/a.woff2");
      expect(publishedPageUrl("/🔌️plugin-modules/cad/bridge.js", origins)).toBe("https://modules.assets.semio-tech.com/🔌️plugin-modules/cad/bridge.js");
      expect(publishedPageUrl("./assets/app.js", origins)).toBe("./assets/app.js");
      expect(shardWorkerUrl().includes("modules.assets")).toBe(false);
      expect(shardWorkerUrl().startsWith("/")).toBe(true);
      expect(publishedPageUrl("https://cdn.example/osm/0/0/0.png", origins)).toBe("https://cdn.example/osm/0/0/0.png");
      expect(relocatePublishedRequestUrl("https://play.semio-tech.com/mesh/a.glb", "https://play.semio-tech.com", origins)).toBe("https://media.assets.semio-tech.com/mesh/a.glb");
      expect(relocatePublishedRequestUrl("https://cdn.example/mesh/a.glb", "https://play.semio-tech.com", origins)).toBe("https://cdn.example/mesh/a.glb");
    });
  });
}
