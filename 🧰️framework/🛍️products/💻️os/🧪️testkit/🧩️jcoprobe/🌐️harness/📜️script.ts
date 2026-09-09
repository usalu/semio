#!/usr/bin/env bun
/** 🧪️ Serve the canonical JCO browser test, its support, and guest examples. */
import { statSync } from "node:fs";
import { join, extname, relative, sep } from "node:path";

const ROOT = join(import.meta.dir, "../../..");
const OWNERS = ["🧪️testkit/🧩️jcoprobe/🌐️harness", "🧪️tests/🧩️jco-callback", "🧫️fixtures/🧩️jcoprobe/🌐️harness"];
const PORT = Number(process.env.SEMIO_JCO_PROBE_PORT ?? 8846);

const MIME: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".wasm": "application/wasm",
  ".ts": "text/plain; charset=utf-8",
  ".json": "application/json; charset=utf-8",
};

const server = Bun.serve({
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    let path: string;
    try { path = url.pathname === "/" ? "🧪️testkit/🧩️jcoprobe/🌐️harness/🌐️.html" : decodeURIComponent(url.pathname).replace(/^\/+/, ""); }
    catch { return new Response("invalid path", { status: 400 }); }
    const full = join(ROOT, path), normalized = relative(ROOT, full).split(sep).join("/");
    if (!OWNERS.some(owner => normalized.startsWith(owner + "/"))) return new Response("not found", { status: 404 });
    try { if (!statSync(full).isFile()) return new Response("not found", { status: 404 }); }
    catch { return new Response("not found", { status: 404 }); }
    const file = Bun.file(full);
    const type = MIME[extname(full)] ?? "application/octet-stream";
    return new Response(file, { headers: { "content-type": type } });
  },
});

console.log(`[jco-probe] serving ${ROOT} on http://localhost:${server.port}`);
