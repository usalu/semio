import { ephemeralMap } from "@semio-tech/framework-transient";
import { TILE_PROXY_TRANSPORT_LIMITS_V1, type TileProxyAssetSpecV1, parseTileProxyAssetSpecV1 } from "../../../../../🖼️assets/📦️packages/🟦️typescript/🗺️tile-proxy/🟦️.ts";
import { resolve, relative, isAbsolute } from "node:path";
import { mkdir, writeFile } from "node:fs/promises";
import type { AssetDeliveryModeV1 } from "../../../../../🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🧭️dispatch/🟦️.ts";
import type { OwnedBuildMiddleware, OwnedBuildPlugin } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { existsSync, mkdirSync, cpSync } from "node:fs";
import { serveFileWithValidatorsV1 } from "../📤️file-serving/🟦️.ts";

/** 🧩️ Extension implied by a resolved tile URL template's tail (`.png`, `.pbf`, …), `"bin"` if absent. */
function tileProxyExtFromTemplate(template: string): string {
  const clean = template.split(/[?#]/, 1)[0] ?? template;
  const ext = clean.split(".").pop();
  return ext && ext.length <= 4 ? ext : "bin";
}

function contentTypeForTileExt(ext: string): string {
  if (ext === "png") return "image/png";
  if (ext === "pbf" || ext === "mvt") return "application/x-protobuf";
  return "application/octet-stream";
}

const tileProxyTemplateCache = ephemeralMap<string, { readonly template: string; readonly at: number }>("framework.modules.ui.styling.packages.rust.vite.elements.assets.ts.tileProxyTemplateCache");
async function boundedTileProxyBytes(response: Response, maximum: number): Promise<Uint8Array> {
  if (Number(response.headers.get("content-length")) > maximum) { await response.body?.cancel(); throw Error("tile proxy body exceeds its byte budget"); }
  if (!response.body) return new Uint8Array();
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let length = 0;
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      length += value.byteLength;
      if (length > maximum) throw Error("tile proxy body exceeds its byte budget");
      chunks.push(value);
    }
  } finally { await reader.cancel(); }
  const output = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) { output.set(chunk, offset); offset += chunk.byteLength; }
  return output;
}

/** 🧭️ Resolves a `tile-proxy` spec's `upstream` to a concrete `{z}/{x}/{y}` URL template: used
 * directly when it already contains `{z}`, otherwise treated as a TileJSON endpoint and resolved
 * (cached, 7-day TTL) for the owner-authored request headers. */
async function resolveTileProxyUrlTemplate(upstream: string, userAgent: string, signal: AbortSignal): Promise<string> {
  if (upstream.includes("{z}")) {
    return upstream;
  }
  const now = Date.now();
  const cacheKey = JSON.stringify([upstream, userAgent]);
  const cached = tileProxyTemplateCache.get(cacheKey);
  if (cached && now - cached.at < TILE_PROXY_TRANSPORT_LIMITS_V1.templateTtlMs) {
    return cached.template;
  }
  const res = await fetch(upstream, { headers: { "User-Agent": userAgent }, signal });
  if (!res.ok) {
    throw new Error(`tile proxy upstream TileJSON failed: ${res.status}`);
  }
  const json = JSON.parse(new TextDecoder().decode(await boundedTileProxyBytes(res, TILE_PROXY_TRANSPORT_LIMITS_V1.templateBytes))) as { tiles?: string[] };
  const template = json.tiles?.[0];
  if (typeof template !== "string" || !template.includes("{z}")) {
    throw new Error("tile proxy TileJSON missing tiles URL template");
  }
  tileProxyTemplateCache.set(cacheKey, { template, at: now });
  return template;
}

async function fetchTileProxyTileToCache(cacheRoot: string, upstream: string, z: number, x: number, y: number, userAgent: string, signal: AbortSignal): Promise<{ readonly ok: boolean; readonly ext: string }> {
  const template = await resolveTileProxyUrlTemplate(upstream, userAgent, signal);
  const ext = tileProxyExtFromTemplate(template);
  const filePath = resolve(cacheRoot, `${z}/${x}/${y}.${ext}`);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return { ok: false, ext };
  }
  await mkdir(resolve(filePath, ".."), { recursive: true });
  const url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
  const upstreamRes = await fetch(url, { headers: { "User-Agent": userAgent }, signal });
  if (!upstreamRes.ok) {
    return { ok: false, ext };
  }
  const buf = await boundedTileProxyBytes(upstreamRes, TILE_PROXY_TRANSPORT_LIMITS_V1.tileBytes);
  if (buf.length === 0) {
    return { ok: false, ext };
  }
  signal.throwIfAborted();
  await writeFile(filePath, buf);
  return { ok: true, ext };
}

/** 🌐️ Connect middleware serving `{route}/{z}/{x}/{y}.{ext}` tiles from `cacheRoot`, fetching
 * (and caching) from the declared upstream on a miss. */
export function createTileProxyMiddleware(route: string, cacheRoot: string, upstream: string, mode: AssetDeliveryModeV1, userAgent: string): OwnedBuildMiddleware {
  const prefix = route.endsWith("/") ? route : `${route}/`;
  const pattern = new RegExp(`^${prefix.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}(\\d+)/(\\d+)/(\\d+)\\.(\\w+)(?:\\?.*)?$`);
  return async (req, res, next) => {
    const match = req.url?.match(pattern);
    if (!match) {
      next();
      return;
    }
    const [, zs, xs, ys, ext] = match as unknown as [string, string, string, string, string];
    const z = Number(zs);
    const x = Number(xs);
    const y = Number(ys);
    const filePath = resolve(cacheRoot, `${z}/${x}/${y}.${ext}`);
    const relToRoot = relative(cacheRoot, filePath);
    if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
      next();
      return;
    }
    if (existsSync(filePath)) {
      serveFileWithValidatorsV1(req, res, filePath, contentTypeForTileExt(ext));
      return;
    }
    if (mode === "bundle") {
      res.statusCode = 404;
      res.end();
      return;
    }
    const lifetime = new AbortController();
    const retire = () => lifetime.abort();
    req.once("aborted", retire);
    req.socket.once("close", retire);
    res.once("close", retire);
    const expiry = setTimeout(retire, TILE_PROXY_TRANSPORT_LIMITS_V1.requestMs);
    const signal = lifetime.signal;
    try {
      const result = await fetchTileProxyTileToCache(cacheRoot, upstream, z, x, y, userAgent, signal);
      if (!result.ok) {
        res.statusCode = 404;
        res.end();
        return;
      }
      serveFileWithValidatorsV1(req, res, filePath, contentTypeForTileExt(result.ext));
    } catch {
      if (!req.socket.destroyed && !res.destroyed && !res.writableEnded) { res.statusCode = 502; res.end(); }
    } finally {
      clearTimeout(expiry);
      req.removeListener("aborted", retire);
      req.socket.removeListener("close", retire);
      res.removeListener("close", retire);
    }
  };
}

/** 🌐️ Generic dev/preview/build Vite plugin pair for one `tile-proxy` asset spec — replaces the
 * owner-authored route and request coordinates into the neutral transport. */
export function tileProxyVitePlugin(repoRoot: string, spec: TileProxyAssetSpecV1, mode: AssetDeliveryModeV1 = "fetch"): OwnedBuildPlugin[] {
  const admitted = parseTileProxyAssetSpecV1(spec);
  const cacheRoot = resolve(repoRoot, admitted.cache);
  const serveTiles = createTileProxyMiddleware(admitted.route, cacheRoot, admitted.upstream, mode, admitted.userAgent);
  let outDir = resolve(process.cwd(), "dist");
  let writeOutput = true;
  const plugins: OwnedBuildPlugin[] = [
    {
      name: `tile-proxy-serve${spec.route}`,
      enforce: "pre",
      configureServer(server) {
        server.middlewares.use(serveTiles);
      },
      configurePreviewServer(server) {
        server.middlewares.use(serveTiles);
      },
    },
  ];
  if (mode === "bundle") {
    plugins.push({
      name: `tile-proxy-build${spec.route}`,
      apply: "build",
      enforce: "pre",
      configResolved(config) {
        outDir = resolve(config.root, config.build.outDir);
        writeOutput = config.build.write !== false;
      },
      closeBundle() {
        if (!writeOutput) return;
        const dist = outDir;
        mkdirSync(dist, { recursive: true });
        if (existsSync(cacheRoot)) {
          cpSync(cacheRoot, resolve(dist, spec.route.replace(/^\//, "")), { recursive: true });
        }
      },
    });
  }
  return plugins;
}
