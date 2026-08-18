#!/usr/bin/env bun
/** @emoji 🌐 Shared jco transpile + plugin web glue (dev runner + extension store).
 *
 * MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): `pluginWorkerSource`/`PLUGIN_WORKER_FILE`
 * (one Worker per plugin) are replaced by `shardWorkerSource`/`SHARD_WORKER_FILE` — ONE
 * package-agnostic `🟨️shard-worker.js`, served from `/plugin-modules/_shard/`, multiplexed by
 * `actorId` across a bounded shard pool (see `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`'s
 * `ShardClient`, the client-side transport this worker pairs with). V8 reserves a 4 GiB guard region
 * per wasm module per worker — one-worker-per-plugin capped the browser at ~20 plugins; this is the
 * change that lifts that ceiling. */
import { spawn } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { buildBudgetMs, resolveWorkspaceBin, runCmdStatus, runNodeBinStatus, semioBuildMode } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

export const PLUGIN_HOST_SHIM_FILE = "🟨️host-shim.js";
export const SHARD_WORKER_FILE = "🟨️shard-worker.js";

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

/**
 * @emoji 🧵️ ONE package-agnostic worker bootstrap shared by every actor this tab's shard pool
 * activates — pairs with `ShardClient` (`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`), which
 * owns exactly K of these workers (design-runtime.md §1 `ShardTable`: `min(hardwareConcurrency-1,
 * 4)` on web) instead of one per plugin. Keeps `Map<actorId, {api, instance}>` and dynamically
 * `import()`s each actor's own jco bridge module (`pluginComponentBridgeSource`'s output) on its
 * first `activate` — never at worker-bootstrap time, since one worker now hosts many unrelated
 * plugins' actors over its lifetime.
 *
 * Runs *one turn at a time per actor*: two `turn` requests for the SAME `actorId` overlapping is a
 * caller bug (the scheduler's job to prevent, not this worker's), enforced here defensively by
 * rejecting a second in-flight turn rather than corrupting interleaved guest state. Different actors
 * DO interleave — every request handler is `async`, so a long `await` inside one actor's turn lets
 * another actor's message be picked up and start in the meantime; nothing here blocks the worker's
 * event loop across actors.
 *
 * Heartbeats: posts `{kind:"heartbeat", turnSeq}` at the START of every request (before running any
 * guest code) — see `ShardClient`'s watchdog, which times a turn out at `2×wallMs` and, after three
 * such windows, terminates and rebuilds this worker. Also mirrors the same `turnSeq` into the shared
 * `Atomics.store` heartbeat slot when `attachHeartbeatSab` provided one (COOP/COEP already served ⇒
 * `SharedArrayBuffer` available) — purely a faster read path for `ShardClient`; the `postMessage`
 * heartbeat above is unconditional, so correctness never depends on the SAB path being available.
 *
 * 🚧 See `🧵️shard-client.ts`'s header doc for the one open gap this generated worker inherits: `turn`
 * events/results here are the interim JSON `ShardEventEnvelope[]`/plain-object shape, not the real
 * hand-rolled `Envelope`/`TurnResult` pack encoding (no TS mirror of that codec exists yet — tracked
 * against A1's `🤖️generated/🟦️actor.ts`). The WIT-level `poll(events, budget)` call this worker makes
 * against the guest's own jco bindings is unaffected either way (jco marshals those to/from the wasm
 * component boundary itself); only the Kernel↔Shard wire between this worker and `ShardClient` is
 * interim JSON rather than pack bytes.
 */
export function shardWorkerSource(): string {
  return `/** @generated semio shard worker (H2 — bounded pool, actorId-multiplexed) */
// 🩺️ SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION (1-B): raise the captured-frame
// cap BEFORE anything else runs so a deep guest recursion's real stack survives \`error.stack\`
// instead of being truncated to V8's 10-frame default — this worker's stack is otherwise destroyed
// before \`ShardClient\` ever sees it (the main thread only ever saw one frame: \`at worker.onmessage\`).
Error.stackTraceLimit = 200;

const actors = new Map(); // actorId -> { api, moduleUrl }
const inFlightTurnActors = new Set();
let turnSeq = 0;
let heartbeatSabView = null;
let heartbeatShardIndex = -1;

function heartbeat() {
  turnSeq += 1;
  self.postMessage({ kind: "heartbeat", turnSeq });
  if (heartbeatSabView) Atomics.store(heartbeatSabView, heartbeatShardIndex, turnSeq);
}

function reply(requestId, value) {
  self.postMessage({ kind: "result", requestId, ok: true, value });
}

// 🩺️ \`frames\` is the request's own bulk payload (the \`turn\` message's \`events\` array — the largest,
// most recursion-prone field a request carries) — sized WITHOUT ever JSON.stringify-ing it first
// unless it isn't already a binary buffer, so a huge/cyclic payload can't itself blow the stack while
// we're trying to report a stack overflow.
function replyError(requestId, error, frames) {
  const payload = error && typeof error === "object" && "payload" in error ? error.payload : undefined;
  const detail = payload !== undefined ? \` payload=\${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}\` : "";
  let stack;
  try { stack = error && error.stack ? String(error.stack) : undefined; } catch { stack = undefined; }
  let type;
  try { type = (error && error.constructor && error.constructor.name) || typeof error; } catch { type = typeof error; }
  let framesBytes;
  try {
    framesBytes = frames instanceof Uint8Array || frames instanceof ArrayBuffer ? frames.byteLength : frames !== undefined ? JSON.stringify(frames).length : undefined;
  } catch { framesBytes = undefined; }
  self.postMessage({ kind: "result", requestId, ok: false, error: (error instanceof Error ? error.message : String(error)) + detail, stack, type, framesBytes });
}

async function loadActor(actorId, moduleUrl) {
  const existing = actors.get(actorId);
  if (existing && existing.moduleUrl === moduleUrl) return existing;
  const bridge = await import(/* @vite-ignore */ moduleUrl);
  const api = await bridge.createActorApi();
  const entry = { api, moduleUrl, pendingAssets: [] };
  actors.set(actorId, entry);
  return entry;
}

// 🪶️ GUESTSLIM (design-runtime.md §3): world \`actor\` exports NO \`activate\` function — activation is
// pure bookkeeping (load the module, cache the named asset packs the main thread fetched) until the
// KERNEL's own first \`turn\` for this actor carries a real \`instance-open\` event (it alone knows
// \`app-id\`/\`config\`/\`quotas\`). This worker's only job is splicing the cached asset bytes into that
// event's \`assets\` field right before the first \`poll\` — they must be resident before the guest's
// first \`surface-visible\`, not fetched lazily on read.
function spliceInstanceOpenAssets(entry, events) {
  if (entry.pendingAssets.length === 0) return events;
  const pending = entry.pendingAssets;
  entry.pendingAssets = [];
  return events.map((event) => {
    if (event.kind !== "instance-open") return event;
    return { kind: event.kind, payload: { ...event.payload, assets: [...(event.payload.assets ?? []), ...pending] } };
  });
}

self.addEventListener("message", async (event) => {
  const msg = event.data ?? {};
  const { kind } = msg;
  if (kind === "attachHeartbeatSab") {
    heartbeatShardIndex = msg.shardIndex;
    heartbeatSabView = new Int32Array(msg.sab);
    return;
  }
  if (kind === "cancelJob") {
    const actor = actors.get(msg.actorId);
    actor?.api.cancelJob(msg.job);
    return;
  }
  if (kind === "dispose") {
    actors.delete(msg.actorId);
    inFlightTurnActors.delete(msg.actorId);
    return;
  }
  const { requestId, actorId } = msg;
  if (!requestId || !actorId) return;
  heartbeat();
  try {
    if (kind === "activate") {
      const entry = await loadActor(actorId, msg.moduleUrl);
      entry.pendingAssets = msg.assets ?? [];
      reply(requestId, undefined);
      return;
    }
    const actor = actors.get(actorId);
    if (!actor) throw new Error(\`shard worker: actor \${actorId} not activated\`);
    switch (kind) {
      case "turn": {
        if (inFlightTurnActors.has(actorId)) throw new Error(\`shard worker: actor \${actorId} already has a turn in flight\`);
        inFlightTurnActors.add(actorId);
        try {
          reply(requestId, await actor.api.poll(spliceInstanceOpenAssets(actor, msg.events), msg.budget));
        } finally {
          inFlightTurnActors.delete(actorId);
        }
        break;
      }
      case "startJob":
        await actor.api.startJob(msg.job, msg.jobKind, msg.input);
        reply(requestId, undefined);
        break;
      case "stepJob":
        reply(requestId, await actor.api.stepJob(msg.job, msg.budget));
        break;
      case "checkpoint":
        reply(requestId, await actor.api.checkpoint());
        break;
      case "restore":
        await actor.api.restore(msg.state);
        reply(requestId, undefined);
        break;
      default:
        throw new Error(\`unknown shard worker message kind: \${kind}\`);
    }
  } catch (error) {
    replyError(requestId, error, msg.events);
  }
});
`;
}

/**
 * @emoji 🌉️ Normalizes ONE actor's jco-transpiled component (`world actor`: exports `reactor`/
 * `jobs`/`checkpoint`/`describe`, imports only `pure` — see `component.wit`) behind the flat
 * `createActorApi()` shape `🟨️shard-worker.js` calls: `poll`/`startJob`/`stepJob`/`cancelJob`/
 * `checkpoint`/`restore`.
 *
 * DROPS the old `runSerialized` retry/reload loop entirely (design-runtime.md §3: "recovery is the
 * kernel's job now"). Under the old ABI a guest panic (`panic = "abort"`, no unwind) permanently
 * killed the wasm32-wasip2 instance, and — with no host-side supervisor — the ONLY recovery available
 * was this bridge silently re-importing the module and replaying. Now `ActivationRegistry`'s
 * `FailurePolicy` (design-runtime.md §1) owns that: a trap here just throws, `🟨️shard-worker.js`'s
 * `replyError` propagates it to `ShardClient` as a rejected turn, and the KERNEL decides
 * `Trapped{restarts}` → drop + re-instantiate (fresh `activate`) + `restore()` the last checkpoint —
 * the SAME re-instantiation this bridge used to do blindly, now a supervised decision instead of a
 * local guess with no visibility into checkpoint state.
 *
 * 🚧 UNVERIFIED against a real compiled artifact (B1b's `GuestRuntime`/wasip2 guest build is still
 * landing as of this packet): the exact jco-generated export shape for a world that exports several
 * *interfaces* (rather than bare functions) is assumed here to be one JS binding per interface, named
 * for the interface (\`reactor\`/\`jobs\`/\`checkpoint\`/\`describe\`), field names camelCased from the
 * WIT's kebab-case. If jco nests these differently, only the four destructured names below need to
 * change — every other line here is interface-shape-agnostic.
 */
export function pluginComponentBridgeSource(componentBase: string, wasmFileName: string): string {
  return `/** @generated semio actor jco component bridge */
const { reactor, jobs, checkpoint, describe } = await import("./${componentBase}.js");

export async function createActorApi() {
  return {
    poll: (events, budget) => reactor.poll(events, budget),
    startJob: (job, kind, input) => jobs.startJob(job, kind, input),
    stepJob: (job, budget) => jobs.stepJob(job, budget),
    cancelJob: (job) => jobs.cancelJob(job),
    checkpoint: () => checkpoint.checkpoint(),
    restore: (state) => checkpoint.restore(state),
    describe: () => describe.describe(),
  };
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
  // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2/H2): `world actor`'s only import is `pure`
  // (component.wit's `interface pure { log; now-ms; trace-span; }`), replacing the old `host`
  // interface — `🟨️host-shim.js` below implements only these three functions now.
  if (runNodeBinStatus(["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️host-shim.js"], ctx.repoRoot) !== 0) {
    throw new Error(`jco transpile failed for ${artifact}`);
  }
  optimizePluginCoreModules(outDir, componentBase, ctx);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`), ctx.preview2VendorDir);
}

/** @emoji 🚀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): non-blocking subprocess spawn, used ONLY
 * by {@link transpilePluginComponentAsync} below. The shared repo-lib's `runNodeBinStatus`/
 * `runCmdStatus` (used by the SYNC {@link transpilePluginComponent} above, which stays exactly as-is
 * for its one other caller, the extension store's `webMaterialize`) both wrap Node's `spawnSync` —
 * correct and desired for a genuinely one-at-a-time step (e.g. `cargo build`), but fatal to any attempt
 * at running several plugins' jco transpile CONCURRENTLY: an async concurrency limiter wrapped around a
 * synchronous blocking call achieves zero real overlap, since nothing else in this process can run
 * while the thread is stuck inside `spawnSync`. `stdio` is piped rather than inherited for the same
 * reason: several of these may be in flight at once, and `"inherit"` would interleave unrelated
 * processes' output byte-by-byte on the parent's own stdout/stderr; buffered output is instead
 * surfaced (as one block) only on failure. Reuses the shared repo-lib's `resolveWorkspaceBin` for the
 * exact same monorepo-aware `.bin/` lookup `runNodeBinStatus` itself uses, rather than
 * reimplementing it. */
function spawnAsync(cmd: string, args: readonly string[], cwd: string): Promise<void> {
  return new Promise((resolveSpawn, rejectSpawn) => {
    const child = spawn(cmd, args as string[], { cwd, shell: false, stdio: ["ignore", "pipe", "pipe"] });
    let output = "";
    child.stdout?.on("data", (chunk: Buffer) => {
      output += chunk;
    });
    child.stderr?.on("data", (chunk: Buffer) => {
      output += chunk;
    });
    child.on("error", (error) => rejectSpawn(error));
    child.on("close", (code) => {
      if (code === 0) {
        resolveSpawn();
        return;
      }
      rejectSpawn(new Error(`${cmd} ${args.join(" ")} exited with status ${code}\n${output}`));
    });
  });
}

function spawnNodeBinAsync(args: readonly string[], cwd: string): Promise<void> {
  const binName = args[0]!;
  const resolved = resolveWorkspaceBin(binName, cwd);
  const executable = resolved ?? binName;
  return spawnAsync("node", [executable, ...args.slice(1)], cwd);
}

/** @emoji 🪶️ Async twin of {@link optimizePluginCoreModules} — same ship-mode-only `wasm-opt` pass,
 * same `WASM_OPT_ARGS`, just spawned via {@link spawnAsync} instead of `runCmdStatus`'s `spawnSync` so
 * it can run concurrently with sibling plugins' own optimize pass under
 * `📜️script.ts`'s bounded-parallel materialize stage (T-P8). */
async function optimizePluginCoreModulesAsync(outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): Promise<void> {
  if (semioBuildMode() !== "ship") return;
  if (process.env.SEMIO_WASM_OPT === "0") return;
  const wasmOptBin = process.env.SEMIO_WASM_OPT_BIN ?? join(ctx.repoRoot, "node_modules/binaryen/bin/wasm-opt");
  for (const file of readdirSync(outDir)) {
    if (!file.startsWith(`${componentBase}.core`) || !file.endsWith(".wasm")) continue;
    const coreWasm = join(outDir, file);
    const optimized = `${coreWasm}.opt`;
    try {
      await spawnAsync("bun", [wasmOptBin, coreWasm, ...WASM_OPT_ARGS, "-o", optimized], ctx.repoRoot);
    } catch {
      throw new Error(`wasm-opt failed for ${coreWasm}`);
    }
    renameSync(optimized, coreWasm);
  }
}

/** @emoji 🚀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): async twin of
 * {@link transpilePluginComponent} — identical jco invocation, ship-mode `wasm-opt` pass, and
 * preview2-shim-import rewrite, but spawned non-blockingly so `📜️script.ts`'s bounded-parallel
 * MATERIALIZE stage (`buildPluginCatalog`) can actually overlap several plugins' transpile/optimize
 * work in wall-clock time — the sync {@link transpilePluginComponent} above cannot provide that overlap
 * no matter how it is scheduled from the caller side (see {@link spawnAsync}'s doc). Kept as a SEPARATE
 * export rather than changing the sync function in place: the extension store's `webMaterialize`
 * (`🏪️store/📜️store.ts`, outside this packet's owned paths) calls the sync version without awaiting
 * it, relying on it blocking until done before it deletes the temp artifact directory in its own
 * `finally` — flipping that function to async out from under that caller would silently race the
 * artifact's cleanup against jco still reading it. */
export async function transpilePluginComponentAsync(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): Promise<void> {
  try {
    await spawnNodeBinAsync(["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️host-shim.js"], ctx.repoRoot);
  } catch {
    throw new Error(`jco transpile failed for ${artifact}`);
  }
  await optimizePluginCoreModulesAsync(outDir, componentBase, ctx);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`), ctx.preview2VendorDir);
}


/**
 * @emoji 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2/H2, design-abi.md §1): `world actor`'s ONLY
 * import — `interface pure { log; now-ms; trace-span; }` — replacing the old `host` world's much
 * larger synchronous surface (`read-document`/`write-document`/`open-window`/`invoke-action`/
 * `read-asset`/`network-fetch`/`write-blob`/`read-blob`, and the ad hoc `backboneSend`/`backbonePoll`/
 * `backboneStatus` worker-postMessage relay). Every one of those is gone: reads/writes/network/dialogs
 * /jobs are now `effect`s returned from `poll`, answered by an `event` on a later `poll` — never a
 * synchronous host-shim call — which is what makes a pooled, multi-instance-per-worker actor
 * `Send`-free and reentrancy-safe in the first place (component.wit's own doc comment on `pure`).
 * `writeBlob`/`readBlob`'s synchronous XHR trick and the `backbonePoll` shared queue are the two
 * pieces design-runtime.md §3 calls out by name for deletion; both are subsumed by the same effect/
 * event turn loop (`document-read`/`document-write`/`blob-load`/`blob-write` effects, `message-event`
 * for backbone-shaped traffic).
 */
export function hostShimSource(): string {
  return `/** @generated semio actor host shim — implements ONLY the \`pure\` import interface */

export function log(level, message) {
  if (level === "error") console.error(\`[actor] \${message}\`);
  else console.log(\`[actor] \${message}\`);
}

export function nowMs() {
  return BigInt(Date.now());
}

export function traceSpan(name) {
  if (typeof performance !== "undefined" && typeof performance.mark === "function") performance.mark(name);
}
`;
}
