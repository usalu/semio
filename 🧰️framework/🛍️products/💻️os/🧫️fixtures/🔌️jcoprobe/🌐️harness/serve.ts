#!/usr/bin/env bun
/** 🧪️ terra-jco-spike static file server — Python's `http.server --directory` hit a sandbox
 * `PermissionError` on this machine (Xcode's bundled python3), so this is a minimal `Bun.serve`
 * static server instead, launched via `.claude/launch.json` -> `preview_start`, never via bare
 * Bash per the ticket's binding rules. Serves `out-callback/` (the jco-transpiled component + the
 * Web Worker harness) with correct MIME types for `.js`/`.wasm`/`.html`. */
import { existsSync } from "node:fs";
import { join, extname } from "node:path";

const ROOT = join(import.meta.dir, "out-callback");
const PORT = 8846;

const MIME: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".wasm": "application/wasm",
  ".ts": "text/plain; charset=utf-8",
  ".json": "application/json; charset=utf-8",
};

Bun.serve({
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    let path = url.pathname === "/" ? "/index.html" : url.pathname;
    const full = join(ROOT, path);
    if (!full.startsWith(ROOT) || !existsSync(full)) {
      return new Response("not found", { status: 404 });
    }
    const file = Bun.file(full);
    const type = MIME[extname(full)] ?? "application/octet-stream";
    return new Response(file, { headers: { "content-type": type } });
  },
});

console.log(`[terra-jco-spike] serving ${ROOT} on http://localhost:${PORT}`);
