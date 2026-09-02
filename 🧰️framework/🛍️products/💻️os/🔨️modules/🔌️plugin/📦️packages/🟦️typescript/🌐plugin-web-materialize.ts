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
import ts from "typescript";
import { ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, encodeActorInstanceLifecycle } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";
import { ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, encodeActorUiPatchReceipt, validateActorUiPatchPairing } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";
import { buildBudgetMs, resolveWorkspaceBin, runCmdStatus, runNodeBinStatus, semioBuildMode } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

export const PLUGIN_HOST_SHIM_FILE = "🟨️.js";
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
 * against A1's `🤖️generated/🟦️actor.ts`). The WIT-level `poll(events, commandPage, budget)` call this worker makes
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
 * function's own doc) — it settles a `🟨️.js` Promise, it is never itself a turn to run.
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
  self.postMessage({ kind: "trap", actorId: "*", activationGeneration: null, message });
  throw new Error(message);
}

const actors = new Map(); // actorId -> { api, moduleUrl }
const activatingActors = new Set();
let lastActivationGeneration = 0n;
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
  // 🩺️ A guest plugin rejects with a LIFTED FAULT RECORD, not an \`Error\` — a plain object whose
  // \`String()\` is the useless \`[object Object]\` that used to be all the host, the console, and the
  // on-screen error surface ever saw for the single most common failure there is. Serialize the
  // record itself; \`Error\` still reports its own message, and an unserializable value still falls
  // back to \`String\`.
  let reason;
  if (error instanceof Error) reason = error.message;
  else if (error && typeof error === "object") { try { reason = JSON.stringify(error); } catch { reason = String(error); } }
  else reason = String(error);
  self.postMessage({ kind: "result", requestId, ok: false, error: reason + detail, stack, type, framesBytes });
}

async function loadActor(actorId, activationGeneration, moduleUrl) {
  if (typeof activationGeneration !== "bigint" || activationGeneration <= lastActivationGeneration || activationGeneration > 0xffffffffffffffffn) throw new Error("actor-close.invalid-activation-generation");
  if (actors.has(actorId) || activatingActors.has(actorId)) throw new Error("actor-close.activation-already-owned");
  lastActivationGeneration = activationGeneration;
  activatingActors.add(actorId);
  try {
    const bridge = await import(/* @vite-ignore */ moduleUrl);
    const api = await bridge.createActorApi(actorId, activationGeneration);
    const entry = { api, moduleUrl, activationGeneration, pendingAssets: [] };
    actors.set(actorId, entry);
    return entry;
  } finally {
    activatingActors.delete(actorId);
  }
}

// 🧪️ terra-web-bridges: settles a \`🟨️.js\` \`effectRequest\` Promise from an \`effect-complete\`/
// \`effect-error\` envelope — see \`hostShimSource\`'s own doc for the wire shape this expects
// (\`envelope.payload.payload.requestId\`, \`.value\` on complete / \`.message\` on error). A missing actor
// (already disposed, or the envelope arrived before \`activate\`) is silently dropped rather than
// thrown, matching \`cancelJob\`'s own \`actor?.\` defensiveness above.
function deliverEffectResult(actorId, activationGeneration, envelope) {
  const actor = actors.get(actorId);
  if (!actor || actor.activationGeneration !== activationGeneration || envelope.to !== actorId || envelope.from?.kind !== "kernel") return;
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
    const actor = actors.get(msg.actorId);
    if (!actor || actor.activationGeneration !== msg.activationGeneration) return;
    actors.delete(msg.actorId);
    inFlightTurnActors.delete(msg.actorId);
    grantedBudgets.delete(msg.actorId);
    return;
  }
  // 🧪️ terra-web-bridges: an effect-complete/effect-error \`"frame"\` is a REPLY to something THIS
  // worker sent (\`🟨️.js\`'s \`effectRequest\`), never a request expecting a \`reply()\` of its
  // own — settled directly, before the generic requestId/actorId-gated dispatch below (which always
  // posts a \`"result"\` back, wrong for a message that is itself already an answer).
  if (kind === "frame" && msg.frame && msg.frame.kind === "Envelope" && msg.frame.envelope && msg.frame.envelope.payload && (msg.frame.envelope.payload.kind === "effect-complete" || msg.frame.envelope.payload.kind === "effect-error")) {
    deliverEffectResult(msg.actorId, msg.activationGeneration, msg.frame.envelope);
    return;
  }
  const { requestId, actorId } = msg;
  if (!requestId || !actorId) return;
  heartbeat();
  try {
    if (kind === "activate") {
      const entry = await loadActor(actorId, msg.activationGeneration, msg.moduleUrl);
      entry.pendingAssets = msg.assets ?? [];
      reply(requestId, undefined);
      return;
    }
    const actor = actors.get(actorId);
    if (!actor) throw new Error(\`shard worker: actor \${actorId} not activated\`);
    switch (kind) {
      case "turn": {
        if (actor.activationGeneration !== msg.activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
        if (inFlightTurnActors.has(actorId)) throw new Error(\`shard worker: actor \${actorId} already has a turn in flight\`);
        inFlightTurnActors.add(actorId);
        try {
          reply(requestId, await actor.api.poll(spliceInstanceOpenAssets(actor, msg.events), msg.commandPage, msg.budget));
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
          reply(requestId, await actor.api.poll(events, undefined, result.budget));
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
 * through. The component and its host imports use matching actor/activation/version URLs; a static
 * relative shim import would otherwise be shared across same-package activations. Each returned API
 * retains that immutable shim module for replies, including while other actors are interleaved.
 */
export function pluginComponentBridgeSource(componentBase: string, wasmFileName: string): string {
  return `/** @generated semio actor jco component bridge */

const ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES = ${ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES};
const encodeActorInstanceLifecycle = ${encodeActorInstanceLifecycle.toString()};
const ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = ${ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES};
const encodeActorUiPatchReceipt = ${encodeActorUiPatchReceipt.toString()};
const validateActorUiPatchPairing = ${validateActorUiPatchPairing.toString()};
const commandIngressKinds = new Map([[0, "idle"], [1, "page-accepted"], [2, "backpressure"], [3, "command-pending"], [4, "command-complete"], [5, "fault"]]);

/** 🎁️ jco lifts \`option<t>\` as a tagged \`{ tag: "none" | "some" }\` variant; every host-side reader
 * below wants the bare value (or nothing), so unwrap exactly that shape and pass anything else through. */
function unwrapOption(value) {
  if (value === null || value === undefined) return undefined;
  if (typeof value === "object" && (value.tag === "none" || value.tag === "some")) return value.tag === "none" ? undefined : value.val;
  return value;
}

function lifecycleBody(value) {
  return { lifetime: value.lifetime, requestSequence: BigInt(value.requestSequence), ...("closeGeneration" in value ? { closeGeneration: value.closeGeneration } : {}) };
}

function lifecycleEvent(kind, payload, activationGeneration) {
  if (kind === "patch-ack" || kind === "patch-rejected") {
    encodeActorUiPatchReceipt(payload.receipt);
    if (payload.receipt.lifetime.activationGeneration !== activationGeneration || payload.surface?.instance !== payload.receipt.lifetime.instanceId) throw new Error("actor-ui-patch.activation-mismatch");
    return { tag: kind, val: payload };
  }
  if (kind === "instance-open") {
    encodeActorInstanceLifecycle({ kind: "open", activationGeneration: payload.activationGeneration, instanceId: payload.instance, requestSequence: payload.requestSequence });
    if (payload.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
    return { tag: kind, val: { ...payload, requestSequence: BigInt(payload.requestSequence) } };
  }
  if (kind === "instance-close" || kind === "instance-lifecycle-ack") {
    if (payload?.kind !== (kind === "instance-close" ? "close" : "ack")) throw new Error("actor-lifecycle.event-kind");
    encodeActorInstanceLifecycle(payload);
    const receipt = kind === "instance-close" ? payload : payload.receipt;
    if (receipt.lifetime.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
    return { tag: kind, val: kind === "instance-close" ? lifecycleBody(receipt) : { tag: receipt.kind, val: lifecycleBody(receipt) } };
  }
  return kind === "wake" ? ({ tag: kind }) : ({ tag: kind, val: payload });
}

function lifecycleReceipt(raw, activationGeneration) {
  const value = unwrapOption(raw);
  if (value === undefined || value === null) return undefined;
  if (value.tag !== "captured" && value.tag !== "accepted" && value.tag !== "retired") throw new Error("actor-lifecycle.receipt-required");
  const body = value.val;
  if (typeof body?.requestSequence !== "bigint" || body.requestSequence <= 0n || body.requestSequence > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("actor-lifecycle.request-sequence");
  if (body.lifetime?.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
  return encodeActorInstanceLifecycle({ kind: value.tag, lifetime: body.lifetime, requestSequence: Number(body.requestSequence), ...(value.tag === "captured" ? {} : { closeGeneration: body.closeGeneration }) });
}

function normalizeCommandIngress(status) {
  const tag = commandIngressKinds.get(status.kind);
  if (!tag) throw new Error(\`unknown command ingress kind: \${status.kind}\`);
  if (tag === "idle") return { tag };
  if (tag === "fault") return { tag, val: { cursor: status.cursor, fault: { tag: "fault", val: status.fault } } };
  return { tag, val: status.cursor };
}

function uiPatchReceipt(result, activationGeneration) {
  if (!Array.isArray(result.uiPatches)) throw new Error("actor-ui-patch.envelope");
  const receipt = unwrapOption(result.uiPatchReceipt);
  validateActorUiPatchPairing(result.uiPatches.length, receipt);
  if (receipt === undefined || receipt === null) return undefined;
  if (receipt.lifetime.activationGeneration !== activationGeneration) throw new Error("actor-ui-patch.activation-mismatch");
  return encodeActorUiPatchReceipt(receipt);
}

export async function createActorApi(actorId, activationGeneration) {
  if (typeof activationGeneration !== "bigint" || activationGeneration <= 0n || activationGeneration > 0xffffffffffffffffn) throw new Error("actor-close.invalid-activation-generation");
  const componentUrl = new URL("./${componentBase}.js", import.meta.url);
  const rebuildVersion = new URL(import.meta.url).searchParams.get("v");
  componentUrl.searchParams.set("actor", actorId);
  componentUrl.searchParams.set("activation", activationGeneration.toString());
  if (rebuildVersion) componentUrl.searchParams.set("v", rebuildVersion);
  const hostUrl = new URL("./${PLUGIN_HOST_SHIM_FILE}", import.meta.url);
  hostUrl.search = componentUrl.search;
  const hostShim = await import(hostUrl.href);
  const { reactor, jobs, checkpoint, describe } = await import(componentUrl.href);
  return {
    poll: async (events, commandPage, budget) => {
      const result = await reactor.poll(events.map(({ kind, payload }) => lifecycleEvent(kind, payload, activationGeneration)), commandPage, { fuel: BigInt(budget.fuel), deadlineMs: budget.wallMs, maxEffects: budget.maxEffects, maxPatchBytes: budget.maxPatchBytes, maxFrames: 8 });
      return { ...result, nextWake: unwrapOption(result.nextWake) ?? null, lifecycleReceipt: lifecycleReceipt(result.lifecycleReceipt, activationGeneration), uiPatchReceipt: uiPatchReceipt(result, activationGeneration), commandIngress: normalizeCommandIngress(result.commandIngress) };
    },
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

const PREVIEW2_SHIM_IMPORT = /(from\s+['"])(?:@bytecodealliance\/preview2-shim|(?:\.\.\/)+(?:plugin-modules\/)?_vendor\/@bytecodealliance\/preview2-shim)\/([\w-]+)(?:\.js)?(['"])/g;

/** @emoji 🪢️ Rewrites bare or previously staged Preview2 imports to one caller-resolved directory prefix. */
export function rewritePreview2ShimImportSource(source: string, prefix: string): string {
  return source.replace(PREVIEW2_SHIM_IMPORT, (_match, lead, subpath, trail) => `${lead}${prefix}${subpath}.js${trail}`);
}

export function rewritePreview2ShimImports(componentJsPath: string, preview2VendorDir: string): void {
  const outDir = dirname(componentJsPath);
  const rel = relative(outDir, preview2VendorDir).replace(/\\/g, "/");
  const prefix = rel.endsWith("/") ? rel : `${rel}/`;
  const content = readFileSync(componentJsPath, "utf8");
  const rewritten = rewritePreview2ShimImportSource(content, prefix);
  if (rewritten !== content) writeFileSync(componentJsPath, rewritten);
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

//#region 🧬️JcoAsyncResultLifting
const JCO_INDIRECT_RESULT_MEMORY_GUARD = "if (!ctx.memory) {\n      _debugLog('missing memory despite indirect param usage'";
const JCO_RESOLVED_RESULT_MEMORY_GUARD = "if (!memory) {\n      _debugLog('missing memory despite indirect param usage'";
const JCO_TASK_RETURN_DIRECT_VALUES = 16;

function jcoResultField(node: ts.Node | undefined, name: string): ts.Expression {
  if (node && ts.isObjectLiteralExpression(node)) {
    for (const field of node.properties) {
      if (ts.isPropertyAssignment(field) && ((ts.isIdentifier(field.name) || ts.isStringLiteral(field.name)) && field.name.text === name)) return field.initializer;
    }
  }
  throw new Error(`jco task-return metadata is missing ${name}`);
}

function jcoResultEntries(node: ts.Node | undefined): readonly ts.Expression[] {
  if (node && ts.isArrayLiteralExpression(node)) return node.elements;
  throw new Error("jco task-return metadata must be an array");
}

function jcoResultFlatCount(node: ts.Expression): number {
  if (node.kind === ts.SyntaxKind.NullKeyword) return 0;
  if (ts.isIdentifier(node)) {
    if (/^_liftFlatString/.test(node.text)) return 2;
    if (/^_liftFlat(Bool|Char|[SU](8|16|32|64)|Float(32|64))$/.test(node.text)) return 1;
  }
  if (ts.isCallExpression(node) && ts.isIdentifier(node.expression)) {
    const meta = node.arguments[0];
    switch (node.expression.text) {
      case "_liftFlatList": return 2;
      case "_liftFlatEnum":
      case "_liftFlatOwn":
      case "_liftFlatBorrow": return 1;
      case "_liftFlatFlags": return Math.ceil(Number(jcoResultField(meta, "size32").getText()) / 4);
      case "_liftFlatRecord": return jcoResultEntries(jcoResultField(meta, "fieldMetas")).reduce((sum, field) => sum + jcoResultFlatCount(jcoResultEntries(field)[1]!), 0);
      case "_liftFlatTuple": return jcoResultEntries(jcoResultField(meta, "elemLiftFns")).reduce((sum, field) => sum + jcoResultFlatCount(jcoResultEntries(field)[0]!), 0);
      case "_liftFlatVariant":
      case "_liftFlatOption":
      case "_liftFlatResult": return 1 + Math.max(0, ...jcoResultEntries(jcoResultField(meta, "caseMetas")).map((field) => jcoResultFlatCount(jcoResultEntries(field)[1]!)));
    }
  }
  throw new Error(`unsupported jco task-return lift: ${node.getText()}`);
}

/** 🧬️ Derives callback directness from each generated result's canonical flattened shape; memory presence alone does not distinguish direct pointer/length values from an indirect return record. */
export function rewriteJcoAsyncResultLifting(source: string): string {
  source = source.replace(JCO_INDIRECT_RESULT_MEMORY_GUARD, JCO_RESOLVED_RESULT_MEMORY_GUARD);
  const parsed = ts.createSourceFile("component.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const edits: { start: number; end: number; value: string }[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && ts.isIdentifier(node.expression.expression) && node.expression.expression.text === "taskReturn" && node.expression.name.text === "bind") {
      const context = node.arguments[node.arguments.length - 1];
      const count = jcoResultEntries(jcoResultField(context, "liftFns")).reduce((sum, field) => sum + jcoResultFlatCount(field), 0);
      const direct = jcoResultField(context, "useDirectParams");
      edits.push({ start: direct.getStart(parsed), end: direct.end, value: String(count <= JCO_TASK_RETURN_DIRECT_VALUES) });
    }
    ts.forEachChild(node, visit);
  };
  visit(parsed);
  for (const edit of edits.sort((left, right) => right.start - left.start)) source = source.slice(0, edit.start) + edit.value + source.slice(edit.end);
  return source;
}

/** @emoji 💾️ Applies {@link rewriteJcoAsyncResultLifting} to one freshly transpiled jco module. */
function rewriteJcoAsyncResultLiftingAt(modulePath: string): void {
  const source = readFileSync(modulePath, "utf8");
  const rewritten = rewriteJcoAsyncResultLifting(source);
  if (rewritten !== source) writeFileSync(modulePath, rewritten);
}
//#endregion 🧬️JcoAsyncResultLifting

//#region 🧊️JcoComponentAssetVersioning
const JCO_HOST_SHIM_URL_HELPER = `function __semioActivationHostUrl() {
  const url = new URL("./🟨️.js", import.meta.url);
  const source = new URL(import.meta.url);
  for (const key of ["actor", "activation", "v"]) {
    const value = source.searchParams.get(key);
    if (value !== null) url.searchParams.set(key, value);
  }
  return url;
}`;

function rewriteJcoHostShimImports(source: string): string {
  const parsed = ts.createSourceFile("component.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const edits: { start: number; end: number; value: string }[] = [];
  for (const statement of parsed.statements) {
    if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier) || statement.moduleSpecifier.text !== `./${PLUGIN_HOST_SHIM_FILE}`) continue;
    const bindings = statement.importClause?.namedBindings;
    if (statement.importClause?.name || !bindings || !ts.isNamedImports(bindings)) throw new Error("unsupported jco host shim import shape");
    const fields = bindings.elements.map((field) => field.propertyName ? `${field.propertyName.getText(parsed)}: ${field.name.text}` : field.name.text).join(", ");
    edits.push({ start: statement.getStart(parsed), end: statement.end, value: `const { ${fields} } = await import(__semioActivationHostUrl().href);` });
  }
  for (const edit of edits.reverse()) source = source.slice(0, edit.start) + edit.value + source.slice(edit.end);
  return edits.length === 0 ? source : `${JCO_HOST_SHIM_URL_HELPER}\n\n${source}`;
}

const JCO_COMPONENT_ASSET_URL = /new URL\((['"])(\.\/[^'"]+\.core\d*\.wasm)\1,\s*import\.meta\.url\)/g;
const JCO_COMPONENT_ASSET_URL_HELPER = `function __semioVersionedComponentAssetUrl(path) {
  const url = new URL(path, import.meta.url);
  const rebuildVersion = new URL(import.meta.url).searchParams.get("v");
  if (rebuildVersion) url.searchParams.set("v", rebuildVersion);
  return url;
}`;

/** 🪪️ Preserves activation identity for host imports and rebuild identity for extracted core Wasm. */
export function rewriteJcoComponentAssetUrls(source: string): string {
  source = rewriteJcoHostShimImports(source);
  if (source.includes("function __semioVersionedComponentAssetUrl(path)")) return source;
  const rewritten = source.replace(JCO_COMPONENT_ASSET_URL, (_match, quote, assetPath) => `__semioVersionedComponentAssetUrl(${quote}${assetPath}${quote})`);
  return rewritten === source ? source : `${JCO_COMPONENT_ASSET_URL_HELPER}\n\n${rewritten}`;
}

/** @emoji 💾️ Applies {@link rewriteJcoComponentAssetUrls} to one freshly transpiled jco module. */
function rewriteJcoComponentAssetUrlsAt(modulePath: string): void {
  const source = readFileSync(modulePath, "utf8");
  const rewritten = rewriteJcoComponentAssetUrls(source);
  if (rewritten !== source) writeFileSync(modulePath, rewritten);
}
//#endregion 🧊️JcoComponentAssetVersioning

export function transpilePluginComponent(artifact: string, outDir: string, componentBase: string, ctx: PluginWebMaterializeContext): void {
  // 🧪️ terra-web-bridges (📓️terra-jco-spike-report.md "what must change" #2): NO `--async-mode`
  // flag — confirmed byte-identical to jco's bare/"sync" default for a component whose every WIT
  // function is already `async func` (`--async-mode jspi` was diffed against the bare transpile of
  // the SAME wasm and produced 0 bytes of difference). `world actor`'s import surface is now `pure`
  // (component.wit's `interface pure { log; now-ms; trace-span; }`, still plain `func`) PLUS
  // `host-async` (`interface host-async`, ~:887 — 24 `async func` imports + `emit`/`emit-patch`) —
  // both map to the SAME `🟨️.js`, which now implements both interfaces' exports from one file.
  if (
    runNodeBinStatus(
      ["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️.js", "--map", "semio:framework/host-async=./🟨️.js"],
      ctx.repoRoot,
    ) !== 0
  ) {
    throw new Error(`jco transpile failed for ${artifact}`);
  }
  rewriteJcoAsyncResultLiftingAt(join(outDir, `${componentBase}.js`));
  rewriteJcoComponentAssetUrlsAt(join(outDir, `${componentBase}.js`));
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
    await spawnNodeBinAsync(["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️.js", "--map", "semio:framework/host-async=./🟨️.js"], ctx.repoRoot);
  } catch {
    throw new Error(`jco transpile failed for ${artifact}`);
  }
  rewriteJcoAsyncResultLiftingAt(join(outDir, `${componentBase}.js`));
  rewriteJcoComponentAssetUrlsAt(join(outDir, `${componentBase}.js`));
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
 * `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`, never a second wire). Host state belongs to
 * an immutable actor/activation/version module URL shared by the component and its returned API.
 *
 * 🚧 UNPROVEN beyond the jcoprobe fixture (📓️terra-jco-spike-report.md): (a) whether jco expects a
 * `result<T, pack>`-returning host-async import to signal `Err` by throwing — jcoprobe's own
 * `probe-host` never used a `result<>` return, so `effectRequest` rejecting on `effect-error` follows
 * jco's documented host-import convention, not a spike-confirmed one; (b) generated JavaScript and
 * ShardClient ownership tests do not establish a fresh guest round trip through every host import.
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
const bindingUrl = new URL(import.meta.url);
const boundActorId = bindingUrl.searchParams.get("actor");
const generationText = bindingUrl.searchParams.get("activation");
const boundActivationGeneration = generationText && /^[1-9][0-9]*$/.test(generationText) && generationText.length <= 20 ? BigInt(generationText) : null;
let effectSeq = 0;
const pendingEffects = new Map();

function assertHostActivation() {
  if (!boundActorId || boundActivationGeneration === null || boundActivationGeneration > 0xffffffffffffffffn) throw new Error("actor-activation.host-unbound");
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
  assertHostActivation();
  const requestId = \`\${boundActorId}:\${boundActivationGeneration}:\${effect}:\${++effectSeq}\`;
  return new Promise((resolve, reject) => {
    pendingEffects.set(requestId, { resolve, reject });
    self.postMessage({
      kind: "frame",
      actorId: boundActorId,
      activationGeneration: boundActivationGeneration,
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
  assertHostActivation();
  self.postMessage({
    kind: "frame",
    actorId: boundActorId,
    activationGeneration: boundActivationGeneration,
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
