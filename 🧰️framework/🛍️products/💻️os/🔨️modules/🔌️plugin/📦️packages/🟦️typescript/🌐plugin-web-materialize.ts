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
 *
 * 📨️ terra-web-shardframe: also handles the NEW `"frame"` message kind — `ShardClient.grant`/
 * `ShardClient.envelope`'s `ShardFrame::Grant`/`ShardFrame::Envelope` wire, additive alongside `"turn"`
 * above (unchanged). `interpretFrame`/`grantedBudgets`/`orderEnvelopesByLane` below are a hand-
 * transcribed mirror of `🧵️shard-client.ts`'s `interpretShardFrame`/`GrantedBudgetTracker`/
 * `orderEnvelopesByLane` — this string can't `import` that module, so the logic is duplicated by
 * necessity; that file's own in-source tests are what's actually exercised for this behavior, this
 * copy is kept byte-for-byte equivalent by hand. A `Grant` remembers its budget for `frame.actor`;
 * a later budget-less `Envelope` for that actor runs under it (falling back to
 * `MAINTENANCE_LANE_DEFAULT_BUDGET` — `semio_framework_actor::lane_defaults::budget_for(Lane::
 * Maintenance)` — for an actor never granted one) instead of any caller-cached constant. An unknown
 * frame `kind` this worker has never heard of is acknowledged as `{ ignored: true }` rather than
 * thrown, so a future Rust-side `ShardFrame` variant can reach a live worker before its TS mirror
 * lands without wedging it. An `Envelope` whose `payload.kind` is `"effect-complete"`/`"effect-error"`
 * is routed to `deliverEffectResult` instead of a normal turn (🧪️ terra-web-bridges, see that
 * function's own doc) — it settles a `🟨️host-shim.js` Promise, it is never itself a turn to run.
 *
 * 🧪️ terra-web-bridges (async-worlds): every WIT function the target world exports/imports is now
 * `async func`, and jco's JS glue for that ALWAYS calls `new WebAssembly.Suspending(...)`/
 * `WebAssembly.promising(...)` regardless of `--async-mode` (📓️terra-jco-spike-report.md's VERDICT:
 * GO-jspi — confirmed no flag produces JSPI-free output). Without JSPI the failure is hard and early:
 * a `TypeError` at MODULE TOP-LEVEL, before any call, with no graceful degradation. The guard right
 * below turns that opaque failure into an explicit, actionable one — a diagnostic, NOT a fallback;
 * there is no code path here that runs a plugin without JSPI.
 */
export function shardWorkerSource(): string {
  return `/** @generated semio shard worker (H2 — bounded pool, actorId-multiplexed) */
// 🩺️ SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION (1-B): raise the captured-frame
// cap BEFORE anything else runs so a deep guest recursion's real stack survives \`error.stack\`
// instead of being truncated to V8's 10-frame default — this worker's stack is otherwise destroyed
// before \`ShardClient\` ever sees it (the main thread only ever saw one frame: \`at worker.onmessage\`).
Error.stackTraceLimit = 200;

// 🧪️ terra-web-bridges: explicit JSPI capability gate — see this file's own header doc ("what must
// change" #1 in 📓️terra-jco-spike-report.md). Every plugin component this worker will ever \`import()\`
// is fully async-lifted, and jco's glue for that unconditionally needs \`WebAssembly.Suspending\`/
// \`WebAssembly.promising\`; without them the FIRST \`import()\` throws \`TypeError: WebAssembly.Suspending
// is not a constructor\` at module top-level, before any plugin call — an opaque failure the spike
// reproduced verbatim under plain Node 24. Posting a \`"trap"\` BEFORE throwing gives \`ShardClient\`'s
// \`onActorTrap\` its best chance at a readable message even where a cross-context Worker \`onerror\`
// gets redacted to \`"undefined undefined undefined"\` (also reproduced by the spike) — \`actorId: "*"\`
// is a worker-wide sentinel, not a real actor, since no actor has activated yet at this point.
if (typeof WebAssembly === "undefined" || typeof WebAssembly.Suspending !== "function" || typeof WebAssembly.promising !== "function") {
  const message = "semio shard worker: this browser/engine lacks JavaScript Promise Integration (JSPI) — WebAssembly.Suspending/WebAssembly.promising are required to run semio's async-lifted plugin components and there is no fallback. Chrome/Edge/Chromium-based browsers ship JSPI on by default; Firefox needs the javascript.options.wasm_js_promise_integration flag in about:config; Node.js needs --experimental-wasm-jspi.";
  self.postMessage({ kind: "trap", actorId: "*", message });
  throw new Error(message);
}

const actors = new Map(); // actorId -> { api, moduleUrl }
const inFlightTurnActors = new Set();
let turnSeq = 0;
let heartbeatSabView = null;
let heartbeatShardIndex = -1;
const MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4096;

// 📨️ terra-web-shardframe: ShardFrame::Grant/Envelope support — see this file's own header doc.
const MAINTENANCE_LANE_DEFAULT_BUDGET = { fuel: 80000000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152 };
const SHARD_FRAME_LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
const grantedBudgets = new Map(); // actorId -> last ShardFrame::Grant budget, mirrors ShardLoop::granted_budgets

function orderEnvelopesByLane(envelopes) {
  return envelopes
    .map((envelope, index) => ({ envelope, index }))
    .sort((left, right) => {
      const rank = SHARD_FRAME_LANE_ORDER.indexOf(left.envelope.lane) - SHARD_FRAME_LANE_ORDER.indexOf(right.envelope.lane);
      return rank !== 0 ? rank : left.index - right.index;
    })
    .map((entry) => entry.envelope);
}

// 🧠️ Mirrors 🧵️shard-client.ts's interpretShardFrame — see that function's own doc for the semantics.
function interpretFrame(frame, actorId) {
  switch (frame.kind) {
    case "Register":
      return { action: "register" };
    case "Unregister":
      return { action: "unregister" };
    case "Grant":
      grantedBudgets.set(frame.actor, frame.budget);
      return { action: "runEnvelopes", budget: frame.budget, envelopes: orderEnvelopesByLane(frame.envelopes) };
    case "Envelope":
      return { action: "runEnvelopes", budget: grantedBudgets.has(actorId) ? grantedBudgets.get(actorId) : MAINTENANCE_LANE_DEFAULT_BUDGET, envelopes: [frame.envelope] };
    default:
      return { action: "unknown" };
  }
}

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
  // 🧪️ terra-web-bridges: \`actorId\` now threads into \`createActorApi\` so \`🟨️host-shim.js\` can bind
  // its \`host-async\` effect-request envelopes to the right actor — see \`pluginComponentBridgeSource\`'s
  // own doc for why this is safe (one moduleUrl per actor ⇒ one shim module instance per actor).
  const api = await bridge.createActorApi(actorId);
  const entry = { api, moduleUrl, pendingAssets: [] };
  actors.set(actorId, entry);
  return entry;
}

// 🧪️ terra-web-bridges: settles a \`🟨️host-shim.js\` \`effectRequest\` Promise from an \`effect-complete\`/
// \`effect-error\` envelope — see \`hostShimSource\`'s own doc for the wire shape this expects
// (\`envelope.payload.payload.requestId\`, \`.value\` on complete / \`.message\` on error). A missing actor
// (already disposed, or the envelope arrived before \`activate\`) is silently dropped rather than
// thrown, matching \`cancelJob\`'s own \`actor?.\` defensiveness above.
function deliverEffectResult(actorId, envelope) {
  const actor = actors.get(actorId);
  if (!actor) return;
  const { kind, payload } = envelope.payload;
  if (kind === "effect-complete") actor.api.resolveEffect(payload.requestId, payload.value);
  else if (kind === "effect-error") actor.api.rejectEffect(payload.requestId, payload.message);
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
    grantedBudgets.delete(msg.actorId);
    return;
  }
  // 🧪️ terra-web-bridges: an effect-complete/effect-error \`"frame"\` is a REPLY to something THIS
  // worker sent (\`🟨️host-shim.js\`'s \`effectRequest\`), never a request expecting a \`reply()\` of its
  // own — settled directly, before the generic requestId/actorId-gated dispatch below (which always
  // posts a \`"result"\` back, wrong for a message that is itself already an answer).
  if (kind === "frame" && msg.frame && msg.frame.kind === "Envelope" && msg.frame.envelope && msg.frame.envelope.payload && (msg.frame.envelope.payload.kind === "effect-complete" || msg.frame.envelope.payload.kind === "effect-error")) {
    deliverEffectResult(msg.actorId, msg.frame.envelope);
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
      case "takeSegmentedDownloadChunk": {
        if (!Number.isSafeInteger(msg.instanceId) || msg.instanceId < 0 || typeof msg.operationId !== "bigint" || msg.operationId <= 0n || msg.operationId > ((1n << 64n) - 1n)) throw new Error("segmented-download-authority-invalid");
        const chunk = await actor.api.takeSegmentedDownloadChunk(msg.instanceId, msg.operationId);
        if (chunk !== undefined && chunk !== null && (Object.prototype.toString.call(chunk) !== "[object Uint8Array]" || chunk.byteLength === 0 || chunk.byteLength > MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES)) throw new Error("segmented-download-worker-limit");
        reply(requestId, chunk ?? undefined);
        break;
      }
      case "checkpoint":
        reply(requestId, await actor.api.checkpoint());
        break;
      case "restore":
        await actor.api.restore(msg.state);
        reply(requestId, undefined);
        break;
      case "frame": {
        const result = interpretFrame(msg.frame, actorId);
        if (result.action === "register") {
          reply(requestId, undefined);
          break;
        }
        if (result.action === "unregister") {
          grantedBudgets.delete(actorId);
          reply(requestId, undefined);
          break;
        }
        if (result.action === "unknown") {
          reply(requestId, { ignored: true });
          break;
        }
        if (inFlightTurnActors.has(actorId)) throw new Error(\`shard worker: actor \${actorId} already has a turn in flight\`);
        inFlightTurnActors.add(actorId);
        try {
          const events = spliceInstanceOpenAssets(actor, result.envelopes.map((envelope) => envelope.payload));
          reply(requestId, await actor.api.poll(events, result.budget));
        } finally {
          inFlightTurnActors.delete(actorId);
        }
        break;
      }
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
 * `takeSegmentedDownloadChunk`/`checkpoint`/`restore`.
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
 * 🚧 UNVERIFIED against a real compiled artifact of the PRODUCTION `world actor` component (the
 * wasm32-wasip2 fleet does not currently compile — a large in-flight conversion tracked elsewhere on
 * this ticket): the exact jco-generated export shape for a world that exports several *interfaces*
 * (rather than bare functions) is assumed here to be one JS binding per interface, named for the
 * interface (\`reactor\`/\`jobs\`/\`checkpoint\`/\`describe\`), field names camelCased from the WIT's
 * kebab-case. **This IS confirmed for a single-export-interface world against a real transpiled
 * component** (📓️terra-jco-spike-report.md's jcoprobe fixture: \`export * as probe from
 * './interfaces/...'\`, camelCased function names) — the multi-interface-export case (4 interfaces,
 * matching \`world actor\`) is extrapolated from that single-interface evidence plus jco's documented
 * per-interface naming convention, not independently re-confirmed here. If jco nests these
 * differently, only the four destructured names below need to change — every other line here is
 * interface-shape-agnostic.
 *
 * 🧪️ terra-web-bridges: every destructured method now returns a Promise (every WIT function in the
 * target world is `async func`) — made EXPLICITLY `async` here rather than relying on bare pass-
 * through, so the shape is self-documenting and robust even if a future jco version wraps a export in
 * something that ISN'T already a thenable. Also now takes `actorId` and binds it into the shim
 * (`__bindHostBridge`) BEFORE returning the api object — see `🟨️host-shim.js` (`hostShimSource`)'s own
 * header doc for why this binding is safe as per-module state and why it's needed at all (every
 * `host-async` import must tag its outbound `effect-request` envelope with the actor it belongs to).
 * `🟨️shard-worker.js`'s `loadActor` is the one caller, updated to pass `actorId` alongside this change.
 */
export function pluginComponentBridgeSource(componentBase: string, wasmFileName: string): string {
  return `/** @generated semio actor jco component bridge */
import * as hostShim from "./${PLUGIN_HOST_SHIM_FILE}";
const { reactor, jobs, checkpoint, describe } = await import("./${componentBase}.js");

export async function createActorApi(actorId) {
  hostShim.__bindHostBridge(actorId);
  return {
    poll: async (events, budget) => reactor.poll(events, budget),
    startJob: async (job, kind, input) => jobs.startJob(job, kind, input),
    stepJob: async (job, budget) => jobs.stepJob(job, budget),
    cancelJob: async (job) => jobs.cancelJob(job),
    takeSegmentedDownloadChunk: async (instanceId, operationId) => jobs.takeSegmentedDownloadChunk(instanceId, operationId),
    checkpoint: async () => checkpoint.checkpoint(),
    restore: async (state) => checkpoint.restore(state),
    describe: async () => describe.describe(),
    resolveEffect: (requestId, value) => hostShim.__resolveEffect(requestId, value),
    rejectEffect: (requestId, message) => hostShim.__rejectEffect(requestId, message),
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
  // 🧪️ terra-web-bridges (📓️terra-jco-spike-report.md "what must change" #2): NO `--async-mode`
  // flag — confirmed byte-identical to jco's bare/"sync" default for a component whose every WIT
  // function is already `async func` (`--async-mode jspi` was diffed against the bare transpile of
  // the SAME wasm and produced 0 bytes of difference). `world actor`'s import surface is now `pure`
  // (component.wit's `interface pure { log; now-ms; trace-span; }`, still plain `func`) PLUS
  // `host-async` (`interface host-async`, ~:887 — 24 `async func` imports + `emit`/`emit-patch`) —
  // both map to the SAME `🟨️host-shim.js`, which now implements both interfaces' exports from one file.
  if (
    runNodeBinStatus(
      ["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️host-shim.js", "--map", "semio:framework/host-async=./🟨️host-shim.js"],
      ctx.repoRoot,
    ) !== 0
  ) {
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
    // 🧪️ terra-web-bridges: same flags/map pair as the sync {@link transpilePluginComponent} above —
    // see that function's own doc for why no `--async-mode` flag is needed and why `host-async` maps
    // to the same shim file `pure` already does.
    await spawnNodeBinAsync(["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️host-shim.js", "--map", "semio:framework/host-async=./🟨️host-shim.js"], ctx.repoRoot);
  } catch {
    throw new Error(`jco transpile failed for ${artifact}`);
  }
  await optimizePluginCoreModulesAsync(outDir, componentBase, ctx);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`), ctx.preview2VendorDir);
}


/**
 * @emoji 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2/H2, design-abi.md §1) + 🧪️ terra-web-bridges
 * (async-worlds). `pure` (`interface pure { log; now-ms; trace-span; }`, component.wit ~:823) stays
 * plain synchronous `func` and is unchanged from H2 — the old `host` world's larger surface
 * (`read-document`/`write-document`/`open-window`/`invoke-action`/`read-asset`/`network-fetch`/
 * `write-blob`/`read-blob`, plus the ad hoc `backboneSend`/`backbonePoll`/`backboneStatus`
 * worker-postMessage relay) is still gone.
 *
 * NEW: `host-async` (component.wit ~:887 — 24 `async func` imports plus the two fire-and-forget
 * `emit`/`emit-patch` doors) is now ALSO implemented in this one file, mapped alongside `pure` by
 * `transpilePluginComponent`'s `--map` pair. Every async import posts an `effect-request` and returns
 * a Promise settled by a later `effect-complete`/`effect-error` — see `effectRequest`'s own doc below
 * for the exact `ShardFrame`/`ShardEnvelope` shape it rides (reused verbatim from
 * `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`, never a second wire) and `__bindHostBridge`'s
 * doc for why per-actor correlation is safe as this module's own top-level state.
 *
 * 🚧 UNPROVEN beyond the jcoprobe fixture (📓️terra-jco-spike-report.md): (a) whether jco expects a
 * `result<T, pack>`-returning host-async import to signal `Err` by throwing — jcoprobe's own
 * `probe-host` never used a `result<>` return, so `effectRequest` rejecting on `effect-error` follows
 * jco's documented host-import convention, not a spike-confirmed one; (b) the kernel-side responder
 * for `effect-request`/`effect-complete` does not exist yet — `ShardClient`'s `InboundMessage` union
 * (`🧵️shard-client.ts`) only recognizes `result`/`heartbeat`/`trap` today, so a real round trip through
 * a live `ShardClient` has not been exercised; only the SHAPE this shim emits/expects is fixed here,
 * for a sibling packet owning that file to start answering.
 */
export function hostShimSource(): string {
  return `/** @generated semio actor host shim — implements the pure AND host-async import interfaces
 * (component.wit ~:823 / ~:887). See plugin-web-materialize.ts's hostShimSource doc for the design
 * this file is generated from. */

//#region 🧬️pure
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
//#endregion 🧬️pure

//#region 🌉️host-async
let boundActorId = null;
let effectSeq = 0;
const pendingEffects = new Map();

// 🌉️ Called once by \`createActorApi(actorId)\` (\`pluginComponentBridgeSource\`), right after this
// module is imported for that actor — every subsequent \`effectRequest\` in THIS module instance is
// tagged with \`actorId\`. Safe as module-scoped (not global) state ONLY because \`🟨️shard-worker.js\`
// dynamically \`import()\`s a distinct moduleUrl per actor (\`loadActor\`'s own doc), so each actor gets
// its own copy of this file's top-level state — never shared across actors, even under the
// cross-actor interleaving this worker's header doc describes.
export function __bindHostBridge(actorId) {
  boundActorId = actorId;
}

// 🌉️ Settles the Promise \`effectRequest\` handed back for \`requestId\` — called by \`🟨️shard-worker.js\`
// when an \`effect-complete\`/\`effect-error\` envelope for this actor arrives.
export function __resolveEffect(requestId, value) {
  const entry = pendingEffects.get(requestId);
  if (!entry) return;
  pendingEffects.delete(requestId);
  entry.resolve(value);
}

export function __rejectEffect(requestId, message) {
  const entry = pendingEffects.get(requestId);
  if (!entry) return;
  pendingEffects.delete(requestId);
  entry.reject(new Error(message));
}

// 🌊️ jco's proven shape for a guest-consumable \`stream<u8>\` is a plain async generator yielding ONE
// byte at a time — confirmed against a real component (jcoprobe's \`fetchBody\`/\`read-body\`, S4 in
// 📓️terra-jco-spike-report.md), NOT a \`ReadableStream\` directly. An \`effect-complete\` for
// \`http-fetch\`/\`blob-read\` is expected to carry a \`ReadableStream\` (structured-clone-transferable
// across \`postMessage\`); this adapts it into the proven per-byte generator shape. Also accepts an
// already-async-iterable value so a kernel handing back a plain byte array still works.
async function* streamToByteGenerator(body) {
  if (body == null) return;
  if (typeof body[Symbol.asyncIterator] === "function") {
    for await (const chunk of body) {
      if (chunk instanceof Uint8Array) { for (const byte of chunk) yield byte; } else yield chunk;
    }
    return;
  }
  const reader = body.getReader();
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) return;
      if (value instanceof Uint8Array) { for (const byte of value) yield byte; } else yield value;
    }
  } finally {
    reader.releaseLock();
  }
}

// 🚪️ Every \`host-async\` ASYNC import funnels through here — posts one \`ShardFrame::Envelope\` up to
// the kernel over the SAME shape \`🧵️shard-client.ts\` declares (\`to\`/\`from\`/\`lane\`/\`seq\`/
// \`deadlineMs\`/\`coalesce\`/\`cancelOf\`/\`payload\`), with \`payload: {kind: "effect-request", payload:
// {effect, requestId, params}}\` — the SAME \`{kind, payload}\` envelope-payload shape
// \`ShardEventEnvelope\` already uses for turn events, reused rather than inventing a new one. Resolves
// or rejects once \`__resolveEffect\`/\`__rejectEffect\` fires for the matching \`requestId\`.
function effectRequest(effect, params) {
  const requestId = \`\${boundActorId}:\${effect}:\${++effectSeq}\`;
  return new Promise((resolve, reject) => {
    pendingEffects.set(requestId, { resolve, reject });
    self.postMessage({
      kind: "frame",
      actorId: boundActorId,
      frame: {
        kind: "Envelope",
        envelope: {
          to: "kernel",
          from: { kind: "actor", id: boundActorId },
          lane: "Background",
          seq: effectSeq,
          deadlineMs: null,
          coalesce: null,
          cancelOf: null,
          payload: { kind: "effect-request", payload: { effect, requestId, params } },
        },
      },
    });
  });
}

// 🚪️ \`emit\`/\`emit-patch\` are plain (non-async) WIT \`func\`s — the ONE fire-and-forget door for the
// ~24 one-way \`effect\` variants (plus \`respond\`) and for UI patches. No \`requestId\`/Promise: posts
// and returns immediately, same envelope shape as \`effectRequest\` above minus the correlation.
function postFireAndForget(kind, payload) {
  self.postMessage({
    kind: "frame",
    actorId: boundActorId,
    frame: {
      kind: "Envelope",
      envelope: { to: "kernel", from: { kind: "actor", id: boundActorId }, lane: "Background", seq: ++effectSeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload } },
    },
  });
}

const call = (effect) => (params) => effectRequest(effect, params);
export const storageRead = call("storage-read");
export const storageWrite = call("storage-write");
export const storageDelete = call("storage-delete");
export const blobLoad = call("blob-load");
export const blobWrite = call("blob-write");
export const blobRead = (hash) => effectRequest("blob-read", { hash }).then(streamToByteGenerator);
export const httpFetch = (params) => effectRequest("http-fetch", params).then((response) => ({ ...response, body: streamToByteGenerator(response.body) }));
export const documentRead = call("document-read");
export const documentWrite = call("document-write");
export const linkResolve = (link) => effectRequest("link-resolve", { link });
export const registryQuery = call("registry-query");
export const ioCompose = call("io-compose");
export const ioRun = call("io-run");
export const cacheDerive = call("cache-derive");
export const cacheRead = call("cache-read");
export const invokeExtension = call("invoke-extension");
export const openWindow = call("open-window");
export const openDialog = call("open-dialog");
export const dispatchAction = call("dispatch-action");
export const spawnPluginInstance = call("spawn-plugin-instance");
export const requestFileOpen = call("request-file-open");
export const requestMediaFrames = call("request-media-frames");
export const requestCapability = call("request-capability");
export const spawnJob = (job, kind, input, placement) => effectRequest("spawn-job", { job, kind, input, placement });

export function emit(value) {
  postFireAndForget("effect-emit", value);
}

export function emitPatch(patch) {
  postFireAndForget("ui-patch-emit", patch);
}
//#endregion 🌉️host-async
`;
}
