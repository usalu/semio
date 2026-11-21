import { ephemeralMap } from "../../../../../🫧️transient/🟦️.ts";
import { TILE_PROXY_TRANSPORT_LIMITS_V1, type TileProxyAssetSpecV1, parseTileProxyAssetSpecV1 } from "../../../../../🖼️assets/📦️packages/🟦️typescript/🗺️tile-proxy/🟦️.ts";
import { basename, dirname, join, resolve, relative, isAbsolute, sep } from "node:path";
import { link, mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import type { AssetDeliveryModeV1 } from "../../../../../🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🧭️dispatch/🟦️.ts";
import type { OwnedBuildMiddleware, OwnedBuildPlugin } from "../../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
import { existsSync, lstatSync, mkdirSync, cpSync } from "node:fs";
import { serveFileWithValidatorsV1 } from "../📤️file-serving/🟦️.ts";

const TILE_STAGE_PREFIX = ".tile-";
const TILE_STAGE_NAME = /^\.tile-[\da-z]{6,}$/iu;

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
type TileProxyResult = { readonly ok: true; readonly ext: string } | { readonly ok: false; readonly status: number };
type TileProxyDownload = { readonly controller: AbortController; readonly result: Promise<TileProxyResult>; consumers: number };
const tileProxyDownloads = ephemeralMap<string, TileProxyDownload>("framework.modules.ui.styling.vite.tileProxyDownloads");

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

async function fetchTileProxyTileToCache(cacheRoot: string, upstream: string, z: number, x: number, y: number, userAgent: string, signal: AbortSignal): Promise<TileProxyResult> {
  const template = await resolveTileProxyUrlTemplate(upstream, userAgent, signal);
  const ext = tileProxyExtFromTemplate(template);
  const filePath = resolve(cacheRoot, `${z}/${x}/${y}.${ext}`);
  const relToRoot = relative(cacheRoot, filePath);
  if (relToRoot.startsWith("..") || isAbsolute(relToRoot)) {
    return { ok: false, status: 404 };
  }
  if (existsSync(filePath)) return { ok: true, ext };
  const url = template.replace("{z}", String(z)).replace("{x}", String(x)).replace("{y}", String(y));
  const upstreamRes = await fetch(url, { headers: { "User-Agent": userAgent }, signal });
  if (!upstreamRes.ok) {
    await upstreamRes.body?.cancel();
    const status = upstreamRes.status;
    return { ok: false, status: status >= 400 && status < 600 ? status : 502 };
  }
  const buf = await boundedTileProxyBytes(upstreamRes, TILE_PROXY_TRANSPORT_LIMITS_V1.tileBytes);
  if (buf.length === 0) {
    return { ok: false, status: 404 };
  }
  signal.throwIfAborted();
  const directory = dirname(filePath);
  await mkdir(directory, { recursive: true });
  const stage = await mkdtemp(join(directory, TILE_STAGE_PREFIX));
  const body = join(stage, "body");
  try {
    await writeFile(body, buf, { signal });
    signal.throwIfAborted();
    await link(body, filePath).catch((error: NodeJS.ErrnoException) => {
      if (error.code !== "EEXIST") throw error;
    });
  } finally {
    await rm(stage, { recursive: true, force: true });
  }
  return { ok: true, ext };
}

function acquireTileProxyDownload(key: string, fetchTile: (signal: AbortSignal) => Promise<TileProxyResult>): TileProxyDownload {
  const current = tileProxyDownloads.get(key);
  if (current && !current.controller.signal.aborted) {
    current.consumers++;
    return current;
  }
  const controller = new AbortController();
  const deadline = setTimeout(() => controller.abort(), TILE_PROXY_TRANSPORT_LIMITS_V1.requestMs);
  const result = fetchTile(controller.signal).finally(() => {
    clearTimeout(deadline);
    if (tileProxyDownloads.get(key) === download) tileProxyDownloads.delete(key);
  });
  const download = { controller, result, consumers: 1 };
  tileProxyDownloads.set(key, download);
  return download;
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
    const key = JSON.stringify([filePath, upstream, userAgent]);
    const download = acquireTileProxyDownload(key, (signal) => fetchTileProxyTileToCache(cacheRoot, upstream, z, x, y, userAgent, signal));
    let retired = false;
    const retire = (): void => {
      if (retired) return;
      retired = true;
      if (--download.consumers === 0 && tileProxyDownloads.get(key) === download) {
        tileProxyDownloads.delete(key);
        download.controller.abort();
      }
    };
    req.once("aborted", retire);
    req.socket.once("close", retire);
    res.once("close", retire);
    try {
      const result = await download.result;
      if (retired || req.socket.destroyed || res.destroyed || res.writableEnded) return;
      if (!result.ok) {
        res.statusCode = result.status;
        res.setHeader("Cache-Control", "no-store");
        res.end();
        return;
      }
      serveFileWithValidatorsV1(req, res, filePath, contentTypeForTileExt(result.ext));
    } catch {
      if (!retired && !req.socket.destroyed && !res.destroyed && !res.writableEnded) {
        res.statusCode = 502;
        res.setHeader("Cache-Control", "no-store");
        res.end();
      }
    } finally {
      retire();
      req.removeListener("aborted", retire);
      req.socket.removeListener("close", retire);
      res.removeListener("close", retire);
    }
  };
}

/** 🀄️ Generic dev/preview/build Vite plugin pair for one `tile-proxy` asset spec — replaces the
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
          cpSync(cacheRoot, resolve(dist, spec.route.replace(/^\//, "")), {
            recursive: true,
            filter: (source) => {
              if (!TILE_STAGE_NAME.test(basename(source))) return true;
              const parents = relative(cacheRoot, dirname(source)).split(sep);
              if (parents.length !== 2 || parents.some((part) => !/^\d+$/u.test(part))) return true;
              return lstatSync(source, { throwIfNoEntry: false })?.isDirectory() === false;
            },
          });
        }
      },
    });
  }
  return plugins;
}
