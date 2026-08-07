#!/usr/bin/env bun
/** @emoji 🌐 Shared jco transpile + plugin web glue (dev runner + extension store). */
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { buildBudgetMs, runCmdStatus, runNodeBinStatus, semioBuildMode } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

export const PLUGIN_HOST_SHIM_FILE = "🟨️host-shim.js";
export const PLUGIN_WORKER_FILE = "🟨️plugin-worker.js";

export type PluginWebMaterializeContext = {
  readonly repoRoot: string;
  readonly preview2VendorDir: string;
};

export function ensurePreview2ShimVendorAt(preview2VendorDir: string, repoRoot: string): void {
  const distDir = join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser");
  const libDir = join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/lib/browser");
  const sourceDir = existsSync(distDir) ? distDir : libDir;
  if (!existsSync(sourceDir)) throw new Error("missing @bytecodealliance/preview2-shim browser shims; run bun install");
  mkdirSync(preview2VendorDir, { recursive: true });
  for (const entry of readdirSync(sourceDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".js")) continue;
    copyFileSync(join(sourceDir, entry.name), join(preview2VendorDir, entry.name));
  }
}

export function pluginWorkerSource(): string {
  return `/** @generated semio plugin web worker */
let pluginApi = null;

async function loadPlugin(moduleUrl) {
  if (pluginApi) return pluginApi;
  const module = await import(moduleUrl);
  if (module.createPluginApi) {
    pluginApi = await module.createPluginApi();
    return pluginApi;
  }
  throw new Error("plugin module missing createPluginApi export");
}

function reply(requestId, type, payload) {
  self.postMessage({ requestId, type, ...payload });
}

function replyError(requestId, message) {
  self.postMessage({ requestId, type: "error", message });
}

self.addEventListener("message", async (event) => {
  const msg = event.data ?? {};
  const { type, requestId } = msg;
  // 🔗️ Backbone relay passthrough (main thread ⇄️ host-shim): inbound messages from the sync actor
  // (\`🟦️backbone-🟦️worker.ts\`) land in the shared queue the host-shim's \`backbonePoll\` drains; the shim's
  // \`backboneSend\` posts \`backboneOutbound\` straight up to the main thread, so there is nothing to do
  // for it here. These carry no requestId, so they must be handled before the request/response guard.
  if (type === "backboneInbound") {
    const queues = (globalThis.__semioBackboneInbound ??= new Map());
    const queue = queues.get(msg.uri) ?? [];
    for (const message of msg.messages ?? []) queue.push(message);
    queues.set(msg.uri, queue);
    return;
  }
  if (!requestId || !type) return;
  try {
    if (type === "init") {
      // 🪶️ GUESTSLIM: bytes forwarded from the main thread's \`acquirePluginModule\` fetch (a worker
      // never owns fetch itself); \`readAsset\` in \`🟨️host-shim.js\` reads from this global.
      if (msg.guestSlimAssets) {
        globalThis.__semioGuestSlimAssets = new Map(msg.guestSlimAssets.map(([handle, buffer]) => [handle, new Uint8Array(buffer)]));
      }
      await loadPlugin(msg.moduleUrl);
      reply(requestId, "init", { ok: true });
      return;
    }
    const api = pluginApi;
    if (!api) throw new Error("worker not initialized");
    switch (type) {
      case "manifest":
        reply(requestId, "manifest", { value: await api.manifest() });
        break;
      case "createApp":
        reply(requestId, "createApp", { instanceId: await api.createApp(msg.appId) });
        break;
      case "destroy":
        await api.destroyApp?.(msg.instanceId);
        reply(requestId, "destroy", { ok: true });
        break;
      case "exchange":
        reply(requestId, "exchange", { value: await api.exchange(msg.instanceId, msg.frames) });
        break;
      default:
        throw new Error(\`unknown worker message type: \${type}\`);
    }
  } catch (error) {
    const payload = error && typeof error === "object" && "payload" in error ? error.payload : undefined;
    const detail = payload !== undefined ? \` payload=\${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}\` : "";
    replyError(requestId, (error instanceof Error ? error.message : String(error)) + detail);
  }
});
`;
}

export function pluginComponentBridgeSource(componentBase: string, wasmFileName: string): string {
  return `/** @generated semio plugin jco component bridge */
import { plugin } from "./${componentBase}.js";

const apps = new Set();
let tail = Promise.resolve();
let pluginApiPromise = null;

function runSerialized(fn) {
  const job = tail.then(async () => {
    for (let attempt = 0; attempt < 8; attempt += 1) {
      try {
        return await fn();
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        const payload = error && typeof error === "object" && "payload" in error ? error.payload : undefined;
        const detail = payload !== undefined ? \`\${message} payload=\${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}\` : message;
        const busy = detail.includes("plugin instance busy") || detail.includes("plugin busy");
        const trapped = detail.includes("unreachable") || /trap|panicked/i.test(detail);
        if (busy || trapped) {
          try { plugin.clearInstanceGuard?.(); } catch { /* guard heal is best-effort */ }
        }
        if (!busy) throw error;
        await new Promise((resolve) => setTimeout(resolve, attempt + 1));
      }
    }
    return fn();
  }, async () => fn());
  tail = job.then(
    () => undefined,
    () => undefined,
  );
  return job;
}

async function createPluginApiInner() {
  const core = {
    async manifest() {
      return await plugin.manifest();
    },
    async createApp(appId) {
      // 🐚️ A random instance id (not \`appId\` itself) so two shells sharing this worker's plugin module
      // (see acquirePluginModule in framework/core) can each instantiate the same app without colliding
      // on the guest's instance-id-keyed \`INSTANCES\` table.
      const instanceId = await plugin.instantiateApp(appId, crypto.randomUUID());
      apps.add(instanceId);
      return instanceId;
    },
    async destroyApp(instanceId) {
      apps.delete(instanceId);
    },
    async exchange(instanceId, frames) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      return await plugin.exchange(instanceId, frames);
    },
  };
  return {
    manifest: () => runSerialized(() => core.manifest()),
    createApp: (appId) => runSerialized(() => core.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => core.destroyApp(instanceId)),
    exchange: (instanceId, frames) => runSerialized(() => core.exchange(instanceId, frames)),
  };
}

export async function createPluginApi() {
  if (!pluginApiPromise) pluginApiPromise = createPluginApiInner();
  return pluginApiPromise;
}
`;
}

export function rewritePreview2ShimImports(componentJsPath: string, preview2VendorDir: string): void {
  const outDir = dirname(componentJsPath);
  const rel = relative(outDir, preview2VendorDir).replace(/\\/g, "/");
  const prefix = rel.endsWith("/") ? rel : `${rel}/`;
  let content = readFileSync(componentJsPath, "utf8");
  const bareSpecifier = /(from\s+['"])@bytecodealliance\/preview2-shim\/([\w-]+)(['"])/g;
  if (!bareSpecifier.test(content)) return;
  content = content.replace(bareSpecifier, (_match, lead, subpath, trail) => `${lead}${prefix}${subpath}.js${trail}`);
  writeFileSync(componentJsPath, content);
}

const WASM_OPT_ARGS: readonly string[] = [
  "-Oz",
  "--low-memory-unused",
  "--strip-debug",
  "--strip-producers",
  "--enable-bulk-memory",
  "--enable-bulk-memory-opt",
  "--enable-call-indirect-overlong",
  "--enable-extended-const",
  "--enable-multivalue",
  "--enable-mutable-globals",
  "--enable-nontrapping-float-to-int",
  "--enable-reference-types",
  "--enable-sign-ext",
];

/** @emoji 🪶️ Runs binaryen's `wasm-opt` in place on every jco-extracted core wasm module in `outDir`
 * (`${componentBase}.core*.wasm`) — component binaries themselves aren't parseable by binaryen; this
 * is exactly what upstream `jco opt` does under the hood. `binaryen` ships an Emscripten JS+wasm build
 * of `wasm-opt` (already a transitive dep of `@bytecodealliance/jco`; pinned as an explicit
 * devDependency here so a future jco upgrade can't silently drop it), so this runs under `bun` with no
 * native binary and no per-platform setup. Skipped entirely in dev (`semioBuildMode() !== "ship"`).
 * `SEMIO_WASM_OPT=0` skips the pass in ship mode; `SEMIO_WASM_OPT_BIN` points at a native `wasm-opt`
 * binary instead, for iteration speed. */
function optimizePluginCoreModules(outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): void {
  if (semioBuildMode() !== "ship") return;
  if (process.env.SEMIO_WASM_OPT === "0") return;
  const wasmOptBin = process.env.SEMIO_WASM_OPT_BIN ?? join(ctx.repoRoot, "node_modules/binaryen/bin/wasm-opt");
  for (const file of readdirSync(outDir)) {
    if (!file.startsWith(`${componentBase}.core`) || !file.endsWith(".wasm")) continue;
    const coreWasm = join(outDir, file);
    const optimized = `${coreWasm}.opt`;
    if (runCmdStatus("bun", [wasmOptBin, coreWasm, ...WASM_OPT_ARGS, "-o", optimized], { cwd: ctx.repoRoot, budgetMs: buildBudgetMs() }) !== 0) {
      throw new Error(`wasm-opt failed for ${coreWasm}`);
    }
    renameSync(optimized, coreWasm);
  }
}

export function transpilePluginComponent(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): void {
  if (runNodeBinStatus(["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/host=./🟨️host-shim.js"], ctx.repoRoot) !== 0) {
    throw new Error(`jco transpile failed for ${artifact}`);
  }
  optimizePluginCoreModules(outDir, componentBase);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`), ctx.preview2VendorDir);
}


export function hostShimSource(): string {
  return `/** @generated semio plugin host shim */

export function log(level, message) {
  if (level === "error") console.error(\`[plugin] \${message}\`);
  else console.log(\`[plugin] \${message}\`);
}

export function nowMs() {
  return BigInt(Date.now());
}

export function readDocument(handle) {
  throw \`read-document unsupported: \${handle}\`;
}

export function writeDocument(handle, payloadJson) {
  throw \`write-document unsupported: \${handle}\`;
}

export function openWindow(kind, paramsJson) {
  throw \`open-window unsupported: \${kind}\`;
}

export function invokeAction(target, invocationJson) {
  throw \`invoke-action unsupported: \${target}\`;
}

export function readAsset(handle) {
  // 🪶️ GUESTSLIM: bytes are pushed into \`globalThis.__semioGuestSlimAssets\` by the worker
  // bootstrap's "init" handler (see \`🟨️plugin-worker.js\`), forwarded from the main thread's
  // \`acquirePluginModule\` fetch — a WASI-P2 program worker never owns fetch itself (see this
  // module's own doc comment above), so there is nothing to fetch synchronously here.
  const bytes = globalThis.__semioGuestSlimAssets?.get(handle);
  if (!bytes) throw \`read-asset unsupported: \${handle}\`;
  return bytes;
}

export function networkFetch(origin, path) {
  throw \`network-fetch unsupported: \${origin}\${path}\`;
}

// 📦️ Must match \`framework/os/core/js/index.ts\`'s \`BLOB_ENDPOINT_PATH\`.
const BLOB_ENDPOINT_PATH = "/semio-blob";

/** @emoji 📦️ Persists \`data\` to the dev server's content-addressed blob store, returning its hash.
 * \`write-blob\`/\`read-blob\` are declared synchronous in the WIT world (no \`async\` on the host import),
 * so this can't use \`fetch\` — a dedicated worker (unlike the main thread) still permits synchronous
 * \`XMLHttpRequest\`, which is the standard sync-bridge trick for exactly this constraint. */
export function writeBlob(data, mediaType) {
  const xhr = new XMLHttpRequest();
  xhr.open("PUT", \`\${BLOB_ENDPOINT_PATH}?mediaType=\${encodeURIComponent(mediaType)}\`, false);
  xhr.send(new Uint8Array(data));
  if (xhr.status < 200 || xhr.status >= 300) throw \`write-blob failed (\${xhr.status})\`;
  return JSON.parse(xhr.responseText).hash;
}

/** @emoji 📦️ Fetches a previously written blob's bytes by hash. See \`writeBlob\` for why this is a
 * synchronous XHR rather than \`fetch\`. */
export function readBlob(hash) {
  const xhr = new XMLHttpRequest();
  xhr.open("GET", \`\${BLOB_ENDPOINT_PATH}/\${encodeURIComponent(hash)}\`, false);
  xhr.responseType = "arraybuffer";
  xhr.send();
  if (xhr.status === 404) throw \`blob not found: \${hash}\`;
  if (xhr.status < 200 || xhr.status >= 300) throw \`read-blob failed (\${xhr.status})\`;
  return new Uint8Array(xhr.response);
}

// 🔗️ Per-uri inbound queues (serialized \`BackboneMessage\`s), shared on the worker global so the program
// worker's \`backboneInbound\` relay (see pluginWorkerSource) can fill them while this shim drains them —
// the two scripts live in the same worker realm but are separate modules.
function backboneInboundQueues() {
  return (globalThis.__semioBackboneInbound ??= new Map());
}
const backboneAttached = new Set();

/** @emoji 📤️ Enqueues an outbound message to the main thread, which relays it into \`🟦️backbone-🟦️worker.ts\`
 * (the sync actor). Inside a dedicated worker this is postMessage-only (a worker can't own the
 * socket/fetch itself); when this component is instead loaded directly on the main thread (the
 * no-\`Worker\`/component-model-load fallback in \`framework/core/js/index.ts\`), it reaches the same
 * relay through the well-known \`__semioMainThreadPluginBackboneOutbound\` global instead. */
export function backboneSend(uri, messageBytes) {
  backboneAttached.add(uri);
  if (typeof WorkerGlobalScope !== "undefined" && typeof self !== "undefined" && typeof self.postMessage === "function") {
    self.postMessage({ type: "backboneOutbound", uri, message: messageBytes });
  } else if (typeof globalThis.__semioMainThreadPluginBackboneOutbound === "function") {
    globalThis.__semioMainThreadPluginBackboneOutbound(uri, messageBytes);
  }
}

/** @emoji 📥️ Drains the inbound queue the worker filled from \`backboneInbound\` postMessages. Returns
 * serialized \`BackboneMessage\`s (never blocks — an empty queue yields \`[]\`). */
export function backbonePoll(uri) {
  backboneAttached.add(uri);
  const queues = backboneInboundQueues();
  const queue = queues.get(uri);
  if (!queue || queue.length === 0) return [];
  queues.set(uri, []);
  return queue;
}

/** @emoji 📶️ Reports whether this shim has seen traffic for a uri (the real transport health lives in
 * \`🟦️backbone-🟦️worker.ts\`; the sandboxed plugin only needs attached/detached). */
export function backboneStatus(uri) {
  return backboneAttached.has(uri) ? "attached" : "detached";
}
`;
}
