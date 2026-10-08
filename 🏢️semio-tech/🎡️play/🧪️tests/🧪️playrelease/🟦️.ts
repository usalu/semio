import { describe, expect, test } from "bun:test";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { request } from "@playwright/test";
import { releaseAssetPath, releaseRequestUrl, servePublishedPlay } from "../../🔨️modules/🧪️e2e/📦️release/🟦️.ts";
import { playPageHost } from "../../🔨️modules/📦️site/📄pages/🟦️.ts";

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as { paths: { request: string; file: string | null }[]; origins: { request: string; expected: string | null }[]; files: { page: string; path: string; body: string }[]; requests: { page: string; path: string; status: number; mime: string; cors: string | null; cacheControl: string }[] };

describe("published Play verification", () => {
  for (const row of fixture.paths) test(`serves only the published file for ${row.request}`, () => {
    const root = resolve(import.meta.dir, "site");
    expect(releaseAssetPath(root, row.request)).toBe(row.file === null ? undefined : join(root, row.file));
    if (row.file !== null) expect(releaseAssetPath(root, row.request)).toBe(resolve(root, row.file));
  });
  for (const row of fixture.origins) test(`routes the exact publication host ${row.request}`, () => {
    const origins = { "https://modules.assets.semio-tech.com": "http://127.0.0.1:1234" };
    expect(releaseRequestUrl(row.request, origins)).toBe(row.expected ?? undefined);
    if (row.expected !== null) {
      const url = new URL(row.request);
      expect(releaseRequestUrl(row.request, origins)).toBe(new URL(url.pathname + url.search, origins[url.origin]!).href);
    }
  });
  test("serves actual published status, MIME, body, cache and CORS to independent clients", async () => {
    const generated = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated") : resolve(import.meta.dir, "../../dist/reports");
    const root = join(generated, `release-serving-${process.pid}`);
    for (const name of ["play", "map", "media", "modules"]) {
      mkdirSync(join(root, name), { recursive: true });
      writeFileSync(join(root, name, "CNAME"), playPageHost(name, "play.semio-tech.com"));
      if (name !== "play") writeFileSync(join(root, name, "_headers"), "/*\n  Access-Control-Allow-Origin: *\n");
    }
    for (const row of fixture.files) {
      const file = join(root, row.page, row.path);
      mkdirSync(dirname(file), { recursive: true });
      writeFileSync(file, row.body);
    }
    const service = servePublishedPlay(root), oracle = await request.newContext();
    try {
      for (const row of fixture.requests) {
        const url = new URL(row.path, service.origins[`https://${playPageHost(row.page, "play.semio-tech.com")}`]);
        const native = await fetch(url), reference = await oracle.get(url.href);
        expect(native.status).toBe(row.status);
        expect(native.status).toBe(reference.status());
        expect(native.headers.get("content-type")).toContain(row.mime);
        expect(native.headers.get("access-control-allow-origin")).toBe(row.cors);
        expect(native.headers.get("cache-control")).toBe(row.cacheControl);
        expect(native.headers.get("cache-control")).toBe(reference.headers()["cache-control"]);
        const body = await native.text();
        expect(body).toBe(await reference.text());
        if (row.status === 200) expect(body).toBe(fixture.files.find(file => file.page === row.page && `/${file.path}` === (row.path === "/" ? "/index.html" : row.path))!.body);
      }
      const missing = await fetch(new URL("/missing.wasm", service.baseURL));
      expect(await missing.text()).not.toContain("<html>");
    } finally { await oracle.dispose(); await service.stop(); rmSync(root, { recursive: true, force: true }); }
  });
});
