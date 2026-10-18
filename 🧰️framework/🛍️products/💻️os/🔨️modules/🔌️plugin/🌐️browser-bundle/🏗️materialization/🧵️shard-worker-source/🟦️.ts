/** 🧵️ Shard worker source of the plugin web materialization: the one package agnostic generated worker that every actor of a shard pool activates through. */
import { SEGMENTED_DOWNLOAD_CHUNK_BYTES, SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS, SHARD_RUNTIME_DIAGNOSTICS_KEY, SHARD_TURN_DRIVE_STEP_CEILING, SHARD_TURN_GUEST_COST_MS, SHARD_WORKER_DIAGNOSTICS_PARAM } from "../🧾️contract/🟦️.ts";

/**
 * 🧵️ ONE package-agnostic worker bootstrap shared by every actor this tab's shard pool
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
 * Heartbeats: a beat is taken at the START of every request (before running any guest code) and at
 * every STEP BOUNDARY — the instant a `turn`/`stepJob` hands control back. The step beat is what
 * proves a guest running a BUDGETED job is alive, because such a guest blocks the worker's event loop
 * (and the while-busy ticker with it) for the whole of each step and is otherwise indistinguishable
 * from a dead worker (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). See `ShardClient`'s watchdog, which
 * times a turn out at `2×wallMs` and, after three such windows, terminates and rebuilds this worker.
 *
 * Neither of those two beats costs a `postMessage` of its own any more: they RIDE the crossing that is
 * already happening, as the reply's `beat` field, and `ShardClient.handleMessage` folds them into the
 * same `recordHeartbeat` a dedicated `{kind:"heartbeat"}` message reaches. One message carries
 * liveness. A beat with NO crossing to ride still posts — `loadActor`'s three await boundaries and
 * the while-busy `progress` ticker — because there is nothing else to carry it. Every beat also
 * mirrors its `turnSeq` into the shared `Atomics.store` heartbeat slot when `attachHeartbeatSab`
 * provided one (COOP/COEP already served ⇒ `SharedArrayBuffer` available) — purely a faster read path;
 * the messages above are unconditional, so correctness never depends on the SAB path.
 *
 * 🚧 See `🧵️shard-client.ts`'s header doc for the one open gap this generated worker inherits: `turn`
 * events/results here are the interim JSON `ShardEventEnvelope[]`/plain-object shape, not the real
 * hand-rolled `Envelope`/`TurnResult` pack encoding (no TS mirror of that codec exists yet — tracked
 * against A1's `🤖️generated/🎭️actor/🟦️.ts`). The WIT-level `stage-command-page`/`stage-cold-pair-page`
 * then `poll(events, budget)` calls this worker makes
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
import { monotonicClock } from "../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/clocks.js";
const operationClockUs = () => monotonicClock.now() / 1000n;
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

// 🩺️ Nothing inside a Worker is visible from the page unless the worker says it: an exception that
// escapes a message handler, or a promise nobody awaits, reaches the parent as an \`ErrorEvent\` whose
// \`message\` the platform routinely redacts to nothing (exactly what the 2026-09-05 boot logged, four
// times, with no other evidence). These three variables are the breadcrumb trail — updated at every
// boundary so a fault report names the phase, the actor and the module URL it died on — and the two
// global listeners below are what turn an otherwise-anonymous \`Event\` into a readable payload on the
// host side. Registered BEFORE the "message" listener so a permissive test stub that keeps only the
// last handler still keeps the dispatcher.
let faultPhase = "bootstrap";
let faultActorId = null;
let faultModuleUrl = null;

function describeFault(value) {
  if (value instanceof Error) return { message: value.message, stack: value.stack };
  if (value && typeof value === "object" && (typeof value.stack === "string" || typeof value.message === "string")) return { message: String(value), stack: typeof value.stack === "string" ? value.stack : undefined };
  if (value && typeof value === "object") { try { return { message: JSON.stringify(value) }; } catch { return { message: String(value) }; } }
  return { message: String(value) };
}

function reportWorkerFault(source, value, event) {
  const reason = value !== undefined && value !== null ? value : event && typeof event.message === "string" && event.message.length > 0 ? event.message : value;
  const described = describeFault(reason);
  self.postMessage({
    kind: "worker-fault",
    source,
    phase: faultPhase,
    actorId: faultActorId,
    moduleUrl: faultModuleUrl,
    message: described.message,
    stack: described.stack,
    filename: event && typeof event.filename === "string" ? event.filename : undefined,
    lineno: event && typeof event.lineno === "number" ? event.lineno : undefined,
  });
}

self.addEventListener("error", (event) => reportWorkerFault("error", event && event.error, event));
self.addEventListener("unhandledrejection", (event) => reportWorkerFault("unhandledrejection", event && event.reason, null));

const actors = new Map(); // actorId -> { api, moduleUrl }
const activatingActors = new Set();
let lastActivationGeneration = 0n;
const inFlightTurnActors = new Set();
let turnSeq = 0;
let heartbeatSabView = null;
let heartbeatShardIndex = -1;
// 📤️ Per-chunk byte cap of the segmented-download lane, interpolated from the ONE schema-owned chunk
// contract (\`🎭️actor/📮️shard-client/📤️segmented-download/🧫️fixtures/🔣️.json\`'s \`contract\`, mirrored by
// \`SEGMENTED_DOWNLOAD_CONTRACT\`) that the guest producer slices by and the host drain admits against —
// never a literal of this worker's own.
const SEGMENTED_DOWNLOAD_CHUNK_BYTES = ${SEGMENTED_DOWNLOAD_CHUNK_BYTES};
// 🫀️ Progress-heartbeat cadence, interpolated from the ONE schema-owned liveness policy
// (\`🎭️actor/📮️shard-client/🧫️fixtures/🔣️.json\`'s \`policy\`, mirrored by \`SHARD_LIVENESS_POLICY\`) the
// host watchdog reads — never a literal of this worker's own.
const PROGRESS_HEARTBEAT_INTERVAL_MS = ${SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS};
// 🚚️ The worker-owned MoreWork drive, interpolated from \`SHARD_TURN_GUEST_COST_MS\`/
// \`SHARD_TURN_DRIVE_STEP_CEILING\` — the MEASURED cost of one whole guest turn, which is what the
// drive's step ceiling is derived FROM. Twin of \`🎭️actor/🖼️wire-turn/🟦️.ts\`'s
// \`driveShardTurnMoreWorkV1\`/\`shardTurnDriveStepsV1\`, which own the law.
const GUEST_TURN_COST_MS = ${SHARD_TURN_GUEST_COST_MS};
const DRIVE_STEP_CEILING = ${SHARD_TURN_DRIVE_STEP_CEILING};
let progressHandle = null;
let inFlightRequests = 0;
// 📬️ Every message the host posts bumps this, and a drive that started at one value crosses back the
// moment it sees another — that is the whole "a host-owned input interrupts the drive" contract. It
// counts messages rather than naming kinds on purpose: a kind this worker has never heard of is still
// the host speaking, and the drive must never be the reason it waits.
let hostInputSeq = 0;

// 📨️ terra-web-shardframe: ShardFrame::Grant/Envelope support — see this file's own header doc.
const MAINTENANCE_LANE_DEFAULT_BUDGET = { fuel: 80000000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152,maxFrames:8 };
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

// 🫀️ ONE beat door for every liveness signal this worker emits. It advances the sequence and mirrors
// it into the shared \`Atomics\` slot, and it RETURNS the beat instead of posting it — because a beat
// taken at a boundary that is about to cross anyway rides the crossing (\`reply\`/\`replyError\` carry
// \`beat\`), and a message that carries liveness is one message, not two.
//
// 🐛️ It used to post unconditionally, so every turn crossing cost THREE main-thread messages: a
// start-of-request \`heartbeat\`, a \`turn-step\` \`heartbeat\`, and the \`result\` itself — on a main
// thread the hop measurement shows is the binding constraint
// (\`📓️reactor-reconcile-spin-2026-09-14.md\` §7 item 3, which named this and left it). Neither posted
// beat told the host anything the reply does not: \`ShardClient.noteLiveness\` treats EVERY inbound
// message as proof of life, and the request's own start instant already counts as proven-alive in
// \`evaluateShardLiveness\`. What is NOT foldable is a beat with no crossing to ride — \`loadActor\`'s
// await boundaries and the while-busy ticker — so those still call \`postBeat\`.
function beat(phase) {
  turnSeq += 1;
  if (heartbeatSabView) Atomics.store(heartbeatSabView, heartbeatShardIndex, turnSeq);
  return { turnSeq, phase: phase === undefined ? null : phase };
}

function postBeat(phase) {
  self.postMessage({ kind: "heartbeat", ...beat(phase) });
}

// 🫀️ The whole busy-versus-dead discriminator: while ANY request is outstanding, an interval beats
// every PROGRESS_HEARTBEAT_INTERVAL_MS. A worker legitimately parked on one multi-second \`await\`
// (fetching, compiling and instantiating a multi-MB wasm component) still runs its event loop, so it
// keeps beating and the host's watchdog keeps its miss count at zero; a worker wedged inside
// synchronous guest code cannot run this callback at all, so it still dies after the same
// \`missedLimit\` windows. Guarded on \`setInterval\` existing so a bare VM/test context is unaffected.
function beginRequest() {
  inFlightRequests += 1;
  if (progressHandle !== null || typeof setInterval !== "function") return;
  progressHandle = setInterval(() => postBeat("progress"), PROGRESS_HEARTBEAT_INTERVAL_MS);
}

function endRequest() {
  inFlightRequests -= 1;
  if (inFlightRequests > 0 || progressHandle === null) return;
  clearInterval(progressHandle);
  progressHandle = null;
}

// ⏱️ The ONE clock this worker shares with the page. A Worker's \`performance.now()\` counts from its
// OWN \`timeOrigin\`, so only \`timeOrigin + now()\` — the same Unix-epoch millisecond on both sides at
// sub-millisecond resolution — lets the CROSSINGS be measured (post → receive, reply → receive)
// rather than inferred by subtracting the parts from the whole. Twin of
// \`🔨️modules/⏱️trace/🟦️.ts\`'s \`hopTraceEpochNowMs\`, which is what reads these back on the page.
const hopEpochNow = () => (typeof performance === "object" && typeof performance.now === "function" ? (typeof performance.timeOrigin === "number" ? performance.timeOrigin : 0) + performance.now() : Date.now());

// 🏷️ The ONE spelling of a turn status. jco lifts \`more-work\` kebab-cased, the host's own fixtures
// spell it \`moreWork\`, and \`🖼️wire-turn/🟦️.ts\`'s \`wireTurnStatusTag\` reconciles both — this is its
// verbatim twin, because a drive that misreads the status would hand back a turn nobody asked for.
function shardTurnStatusTag(result) {
  const tag = result && typeof result === "object" && result.status && typeof result.status === "object" ? result.status.tag : undefined;
  return typeof tag === "string" ? tag.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase() : "";
}

// 🤫️ Whether ONE reactor turn result carried nothing the host could act on: no ui patch, no effect,
// no presence, no wake, neither receipt, and both ingress lanes idle. Every field of \`turn-result\`
// (\`🔌️plugin/🧬️schema/📜️reactor.wit\`) is named here on purpose — a carrier this predicate forgot would be
// DROPPED by the drive below, so the list is exhaustive by construction and asserted as such by
// \`🎭️actor/🖼️wire-turn/🟦️.ts\`'s law. \`fuelUsed\` is the one field with no host reader.
function shardTurnCarriesNothing(result) {
  if (!result || typeof result !== "object") return false;
  if (!Array.isArray(result.uiPatches) || result.uiPatches.length !== 0) return false;
  if (!Array.isArray(result.effects) || result.effects.length !== 0) return false;
  if (result.presence !== undefined && (!Array.isArray(result.presence) || result.presence.length !== 0)) return false;
  if (result.nextWake !== null && result.nextWake !== undefined) return false;
  if (result.lifecycleReceipt !== undefined && result.lifecycleReceipt !== null) return false;
  if (result.uiPatchReceipt !== undefined && result.uiPatchReceipt !== null) return false;
  if (!result.commandIngress || result.commandIngress.tag !== "idle") return false;
  if (!result.coldPairIngress || result.coldPairIngress.tag !== "idle") return false;
  return true;
}

// 🚚️ The drive's two bounds, DERIVED from the wall the host granted for this crossing and what one
// guest turn measures — never a constant of this worker's own. Verbatim twins of
// \`🎭️actor/🖼️wire-turn/🟦️.ts\`'s \`shardTurnDriveStepsV1\`/\`shardTurnDriveBudgetMsV1\`.
function shardTurnDriveSteps(grantWallMs, guestTurnCostMs) {
  if (!Number.isFinite(grantWallMs) || !Number.isFinite(guestTurnCostMs) || guestTurnCostMs <= 0) return 1;
  return Math.max(1, Math.min(DRIVE_STEP_CEILING, Math.floor(grantWallMs / guestTurnCostMs)));
}

function shardTurnDriveBudgetMs(grantWallMs, guestTurnCostMs) {
  if (!Number.isFinite(grantWallMs) || grantWallMs <= 0) return Math.max(0, guestTurnCostMs);
  return Math.max(grantWallMs, guestTurnCostMs);
}

// 📬️ ONE macrotask, so a message the host already posted is DELIVERED before the drive takes its next
// step. Without it \`hostInputSeq\` could never move mid-drive: an \`await\` settles on the microtask
// queue, which never drains the message queue, so the drive would be uninterruptible by construction.
// A \`MessageChannel\` task and not \`setTimeout(0)\`, for the reason the host's own
// \`hostContinuations\` gives: a timer chain is throttled to one tick per second in a hidden tab (once
// per minute under intensive throttling) and a channel message is a macrotask visibility never
// throttles. Waiters are a QUEUE because two actors may be driving this worker at the same time.
const driveYieldChannel = typeof MessageChannel === "function" ? new MessageChannel() : null;
const driveYieldWaiters = [];
if (driveYieldChannel) driveYieldChannel.port1.onmessage = () => { const resolve = driveYieldWaiters.shift(); if (resolve) resolve(); };

function driveYield() {
  if (!driveYieldChannel) return Promise.resolve();
  return new Promise((resolve) => { driveYieldWaiters.push(resolve); driveYieldChannel.port2.postMessage(0); });
}

// 🫀️ EVERY reply carries a beat — an answered request is a proven-alive worker, and the caller's own
// \`ShardClient\` folds it into exactly the state a dedicated \`heartbeat\` message would have reached.
// A boundary that has a name of its own (\`turn-step\`) passes it; everything else beats unphased.
function reply(requestId, value, timings, carriedBeat) {
  const carried = carriedBeat === undefined ? beat() : carriedBeat;
  if (!timings) { self.postMessage({ kind: "result", requestId, ok: true, value, beat: carried }); return; }
  timings.repliedAtEpochMs = hopEpochNow();
  self.postMessage({ kind: "result", requestId, ok: true, value, timings, beat: carried });
  // ⏱️ Written AFTER the post so the host can separate the two halves of the reply: the structured
  // CLONE this call performs synchronously (\`clonedAtEpochMs\` − \`repliedAtEpochMs\`, mutated on the
  // object the clone already took a copy of, so the host reads it from the NEXT reply's carry) from
  // the main thread's own pickup latency. A worker cannot amend a message it already posted, so the
  // clone cost of turn N is carried on turn N+1 — one turn of lag, exact either way.
  lastReplyCloneMs = hopEpochNow() - timings.repliedAtEpochMs;
}
let lastReplyCloneMs = 0;

// 🩺️ \`frames\` is the request's own bulk payload (the \`turn\` message's \`events\` array — the largest,
// most recursion-prone field a request carries) — sized WITHOUT ever JSON.stringify-ing it first
// unless it isn't already a binary buffer, so a huge/cyclic payload can't itself blow the stack while
// we're trying to report a stack overflow. A \`turn\`'s \`events\` is an ARRAY of wire buffers, never a
// lone buffer, so summing member byte lengths is the only path that reports wire bytes: the stringify
// fallback renders each byte as \`"index":value\` and inflated a 6 MB payload to 63 MB, which is a
// memory diagnosis this ticket had to walk back (26/09/09/PROCEDURAL-3D-END-TO-END).
function replyError(requestId, error, frames, retryableLifecycle) {
  const payload = error && typeof error === "object" && "payload" in error ? error.payload : undefined;
  const detail = payload !== undefined ? \` payload=\${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}\` : "";
  let stack;
  try { stack = error && error.stack ? String(error.stack) : undefined; } catch { stack = undefined; }
  let type;
  try { type = (error && error.constructor && error.constructor.name) || typeof error; } catch { type = typeof error; }
  let framesBytes;
  try {
    framesBytes = frames instanceof Uint8Array || frames instanceof ArrayBuffer ? frames.byteLength
      : Array.isArray(frames) && frames.every((frame) => frame instanceof Uint8Array || frame instanceof ArrayBuffer) ? frames.reduce((total, frame) => total + frame.byteLength, 0)
      : frames !== undefined ? JSON.stringify(frames).length : undefined;
  } catch { framesBytes = undefined; }
  // 🩺️ A guest plugin rejects with a LIFTED FAULT RECORD, not an \`Error\` — a plain object whose
  // \`String()\` is the useless \`[object Object]\` that used to be all the host, the console, and the
  // on-screen error surface ever saw for the single most common failure there is. Serialize the
  // record itself; \`Error\` still reports its own message, and an unserializable value still falls
  // back to \`String\`.
  // 🩺️ \`instanceof Error\` is false for an Error thrown in ANOTHER realm (a \`vm\` context, or any
  // cross-context host callback this worker invokes), and \`JSON.stringify\` of an Error is the useless
  // \`"{}"\` — which is what the host used to be told for exactly the faults it most needs to read.
  // Duck-typing on \`stack\`/\`message\` catches the cross-realm case without ever mis-serializing a
  // genuine lifted fault record, which has neither.
  let reason;
  if (error instanceof Error) reason = error.message;
  else if (error && typeof error === "object" && (typeof error.stack === "string" || typeof error.message === "string")) reason = String(error);
  else if (error && typeof error === "object") { try { reason = JSON.stringify(error); } catch { reason = String(error); } }
  else reason = String(error);
  self.postMessage({ kind: "result", requestId, ok: false, error: reason + detail, stack, type, framesBytes, retryableLifecycle: retryableLifecycle === true, beat: beat("fault") });
}

// 🩺️ Hands every component this worker hosts the guest-side diagnostics switch through
// \`wasi:cli/environment\` — the ONE schema-declared door a \`wasm32-wasip2\` component's own
// \`std::env::var\` reads (\`semio_framework_trace::RUNTIME_DIAGNOSTICS_ENV\`). Before this, every
// \`[TRACE]\` line the guest's Rust hot path prints was unreachable from a browser session: the page's
// \`localStorage\` switch only ever armed TypeScript-side traces, and \`runtime_diagnostics_from_environment\`
// resolved against an environment nobody populated (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
// \`📓️audit-guest-tick-cost-2026-09-12.md\` §0/§4 rank 1).
//
// 🚪️ The page resolved the switch and stamped it on this worker's OWN url (\`shardWorkerUrl\`), since a
// Worker realm owns no \`localStorage\`. The shim module is the SAME instance the transpiled component
// imports — both resolve \`🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js\` against the shared
// \`🔌️plugin-modules/\` root — so seeding it here seeds the guest. Disarmed is the default and costs
// one url read; a shim that cannot be loaded degrades to silent traces, never to a failed boot.
let guestRuntimeDiagnostics = null;
async function armGuestRuntimeDiagnostics() {
  if (guestRuntimeDiagnostics !== null) return guestRuntimeDiagnostics;
  guestRuntimeDiagnostics = (async () => {
    if (new URL(self.location.href).searchParams.get("${SHARD_WORKER_DIAGNOSTICS_PARAM}") !== "1") return false;
    try {
      const { _setEnv } = await import(/* @vite-ignore */ "../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js");
      _setEnv({ "${SHARD_RUNTIME_DIAGNOSTICS_KEY}": "1" });
      return true;
    } catch {
      return false;
    }
  })();
  return guestRuntimeDiagnostics;
}

async function loadActor(actorId, activationGeneration, moduleUrl, originalAdmission) {
  if (typeof activationGeneration !== "bigint" || activationGeneration <= lastActivationGeneration || activationGeneration > 0xffffffffffffffffn) throw new Error("actor-close.invalid-activation-generation");
  if (actors.has(actorId) || activatingActors.has(actorId)) throw new Error("actor-close.activation-already-owned");
  lastActivationGeneration = activationGeneration;
  activatingActors.add(actorId);
  try {
    // 🫀️ The three await boundaries of the longest turn this worker ever runs: the dynamic import
    // (network fetch + WebAssembly.compile + instantiate, all opaque inside one await) and the guest's
    // own \`createActorApi\`. Each boundary beats explicitly so a console trace names where a slow boot
    // actually sat; the ticker above is what carries liveness THROUGH each of them.
    faultModuleUrl = moduleUrl;
    faultPhase = "load-bridge";
    postBeat("module-fetch");
    await armGuestRuntimeDiagnostics();
    const bridge = await import(/* @vite-ignore */ moduleUrl);
    faultPhase = "instantiate";
    postBeat("module-ready");
    const api = await bridge.createActorApi(actorId, activationGeneration, originalAdmission, operationClockUs, { receipt: receipt => self.postMessage({ kind: "original-turn-receipt", actorId, activationGeneration, receipt }), hold: refusal => self.postMessage({ kind: "original-producer-held", actorId, activationGeneration, operation: originalAdmission.operation, generation: originalAdmission.generation, refusal }) });
    faultPhase = "actor-ready";
    postBeat("actor-ready");
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
// thrown — this envelope is itself an ANSWER, so nobody is awaiting a reply to it. That is exactly
// what makes it different from \`cancelJob\`, which IS a request and is answered in the switch below.
function deliverEffectResult(actorId, activationGeneration, envelope) {
  const actor = actors.get(actorId);
  if (!actor || actor.activationGeneration !== activationGeneration || envelope.to !== actorId || envelope.from?.kind !== "kernel") return;
  const { kind, payload } = envelope.payload;
  if (kind === "effect-complete") actor.api.resolveEffect(payload.requestId, payload.value);
  else if (kind === "effect-error") actor.api.rejectEffect(payload.requestId, payload.message);
}

async function closeOriginalActor(actorId, activationGeneration) {
  const actor = actors.get(actorId);
  if (!actor || actor.activationGeneration !== activationGeneration || inFlightTurnActors.has(actorId)) return;
  try {
    const closed = await actor.api.closeOriginal();
    if (closed.sourceClosed) { actors.delete(actorId); grantedBudgets.delete(actorId); }
    self.postMessage({ kind: "original-source-close", actorId, activationGeneration, ...closed });
  } catch (failure) {
    self.postMessage({ kind: "original-source-close", actorId, activationGeneration, operation: actor.api.originalSource.operation, generation: actor.api.originalSource.generation, sourceClosed: false });
  }
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

// ⏱️ The browser twin of the host's own \`retryable_lifecycle_turn\` (\`🔌️plugin/🖥️host/🦀️.rs\`, driven
// by \`🖥️host/🔁️lifecycle/🧫️fixtures/🔣️.json\`): a RETRYABLE \`plugin.reactor-turn-deadline\` on a turn
// that carries at most one lifecycle event is a YIELD, not a death — the guest retained its receipt,
// so the same events replay on the next tick and the open continues. Native \`ShardLoop::pump\` has
// always re-granted these; without this the browser reported the identical verdict as a worker fault
// and \`onActorTrap\` killed the actor on its very first step (ticket 26/09/09, boot #6).
const LIFECYCLE_TURN_EVENT_KINDS = ["instance-open", "instance-close", "instance-lifecycle-ack"];
const REACTOR_TURN_DEADLINE_CODE = "plugin.reactor-turn-deadline";

function guestFaultRecord(error) {
  for (const value of [error, error?.payload, error?.payload?.val, error?.val]) {
    if (!value) continue;
    if (typeof value.code === "string") return value;
    if (value instanceof Uint8Array) {
      try { return JSON.parse(new TextDecoder().decode(value)); } catch { return null; }
    }
  }
  return null;
}

function retryableLifecycleTurn(error, events, commandPage) {
  const fault = guestFaultRecord(error);
  if (!fault || fault.code !== REACTOR_TURN_DEADLINE_CODE || fault.retryable !== true) return false;
  return (commandPage === undefined || commandPage === null) && Array.isArray(events) && events.length <= 1 && events.every((entry) => entry && LIFECYCLE_TURN_EVENT_KINDS.includes(entry.kind));
}

self.addEventListener("message", async (event) => {
  // ⏱️ FIRST statement of the handler: everything after it is already this worker's own cost, and
  // \`postedAtEpochMs\` − this instant is the post crossing nobody could see from outside (the CDP
  // \`Performance\` domain answers nothing on a worker target —
  // \`📓️react-hop-latency-2026-09-14.md\` §1). Twin vocabulary: \`semio.hop.worker.*\`.
  const receivedAtEpochMs = hopEpochNow();
  // 📬️ The host said something. A drive in flight reads this counter every step and crosses back the
  // moment it moves, so an ingress message, a cancel or a view-state change never waits behind a
  // guest the worker is pumping. Bumped BEFORE any dispatch, for every kind without exception.
  hostInputSeq += 1;
  const msg = event.data ?? {};
  const { kind } = msg;
  if (kind === "attachHeartbeatSab") {
    heartbeatShardIndex = msg.shardIndex;
    heartbeatSabView = msg.sab ? new Int32Array(msg.sab) : null;
    return;
  }
  if (kind === "original-root-refusal") {
    const actor = actors.get(msg.actorId);
    if (actor && actor.activationGeneration === msg.activationGeneration && actor.api.originalSource.nativeRefusal === undefined) actor.api.originalSource.holdNativeRefusal(msg.refusal);
    return;
  }
  if (kind === "dispose") {
    const actor = actors.get(msg.actorId);
    if (!actor || actor.activationGeneration !== msg.activationGeneration) return;
    actor.disposeRequested = true;
    if (inFlightTurnActors.has(msg.actorId)) { actor.api.originalSource.cancel(); return; }
    await closeOriginalActor(msg.actorId, msg.activationGeneration);
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
  const timings = { postedAtEpochMs: typeof msg.postedAtEpochMs === "number" ? msg.postedAtEpochMs : null, receivedAtEpochMs, guestEnteredAtEpochMs: null, guestLeftAtEpochMs: null, repliedAtEpochMs: null, previousReplyCloneMs: lastReplyCloneMs, events: Array.isArray(msg.events) ? msg.events.length : 0, eventKinds: Array.isArray(msg.events) ? [...new Set(msg.events.map((entry) => (entry && typeof entry.kind === "string" ? entry.kind : "?")))].slice(0, 4).join("+") : "", patches: 0, commandPageBytes: msg.commandPage && msg.commandPage.bytes ? msg.commandPage.bytes.byteLength ?? msg.commandPage.bytes.length ?? 0 : 0 };
  beat();
  beginRequest();
  faultPhase = kind;
  faultActorId = actorId;
  try {
    if (kind === "activate") {
      const entry = await loadActor(actorId, msg.activationGeneration, msg.moduleUrl, msg.originalAdmission);
      entry.pendingAssets = msg.assets ?? [];
      reply(requestId, undefined);
      return;
    }
    const actor = actors.get(actorId);
    // 🛑️ A cancel for an actor that is already gone is a no-op the CALLER still has to be told about:
    // \`ShardClient.cancelJob\` awaits a \`"result"\`, so answering nothing leaves the request outstanding
    // in \`pending\` forever and the watchdog kills the shard over it (ticket
    // 26/09/02/PUZZLE-3D-END-TO-END wave B42). Every other kind genuinely needs the actor.
    if (!actor && kind === "cancelJob") {
      reply(requestId, undefined);
      return;
    }
    if (!actor) throw new Error(\`shard worker: actor \${actorId} not activated\`);
    switch (kind) {
      case "turn": {
        if (actor.activationGeneration !== msg.activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
        if (inFlightTurnActors.has(actorId)) throw new Error(\`shard worker: actor \${actorId} already has a turn in flight\`);
        inFlightTurnActors.add(actorId);
        faultPhase = (actor.turns ?? 0) === 0 ? "first-step" : "turn";
        actor.turns = (actor.turns ?? 0) + 1;
        try {
          const admitted = spliceInstanceOpenAssets(actor, msg.events);
          timings.guestEnteredAtEpochMs = hopEpochNow();
          let result = await actor.api.poll(admitted, msg.commandPage, undefined, msg.budget);
          // 🚚️ THE worker-owned MoreWork drive. A \`more-work\` answer that carried nothing is not a
          // message for the host — it is the guest asking to be pumped, and in the browser the host
          // round trip IS the pump. So the WORKER owns that pump: it runs the next turn itself and
          // crosses only to deliver something the host can act on. Every discarded result satisfied
          // \`shardTurnCarriesNothing\`, so the drive is lossless by construction.
          //
          // Five other stops, and each one is a promise to somebody: the guest went \`idle\`;
          // \`hostInputSeq\` moved, so a host-owned input is pending behind the drive and must not wait
          // for it; the actor was disposed or re-activated; the derived step ceiling is spent; the
          // derived wall budget is spent. The ceiling and the budget come from
          // \`shardTurnDriveSteps\`/\`shardTurnDriveBudgetMs\` — the wall the host GRANTED for this
          // crossing divided by what one turn MEASURES — never from the reactor's own 8 ms executor
          // slice, which is smaller than one whole turn and therefore admitted no further turn at all.
          const originalDriveSource = actor.api.originalSource;
          const remainingUs = originalDriveSource.deadlineUs - originalDriveSource.operationClockUs();
          const grantWallMs = Number(remainingUs > 0n ? remainingUs : 0n) / 1000;
          // 🚚️ The WALL bounds this drive; \`DRIVE_STEP_CEILING\` is the hard backstop against an
          // unbounded loop, and \`shardTurnDriveSteps\` is the grant's derived EXPECTATION, reported so
          // the host can read what one crossing was meant to cover.
          const driveSteps = DRIVE_STEP_CEILING;
          const driveExpectedSteps = shardTurnDriveSteps(grantWallMs, GUEST_TURN_COST_MS);
          const driveDeadline = originalDriveSource.deadlineUs;
          const inputMark = hostInputSeq;
          let drivePolls = 0;
          let driveStopped = shardTurnStatusTag(result) === "more-work" ? (shardTurnCarriesNothing(result) ? "" : "carried") : "idle";
          while (driveStopped === "") {
            if (hostInputSeq !== inputMark) { driveStopped = "input"; break; }
            if (actors.get(actorId) !== actor || actor.activationGeneration !== msg.activationGeneration) { driveStopped = "closed"; break; }
            if (originalDriveSource.operationClockUs() >= driveDeadline || originalDriveSource.fuelRemaining === 0) { driveStopped = "budget"; break; }
            if (drivePolls + 1 >= driveSteps) { driveStopped = "steps"; break; }
            // 📬️ One macrotask before the next turn, so a message the host already posted is
            // DELIVERED and \`hostInputSeq\` can actually move. Without it the drive is
            // uninterruptible: an \`await\` settles on the microtask queue, which never drains the
            // message queue.
            await driveYield();
            if (hostInputSeq !== inputMark) { driveStopped = "input"; break; }
            if (actors.get(actorId) !== actor || actor.activationGeneration !== msg.activationGeneration) { driveStopped = "closed"; break; }
            result = await actor.api.poll([], undefined, undefined, msg.budget);
            drivePolls += 1;
            driveStopped = shardTurnStatusTag(result) === "more-work" ? (shardTurnCarriesNothing(result) ? "" : "carried") : "idle";
          }
          timings.guestLeftAtEpochMs = hopEpochNow();
          timings.drivePolls = drivePolls;
          timings.driveSteps = driveExpectedSteps;
          timings.driveStopped = driveStopped;
          timings.patches = result && Array.isArray(result.uiPatches) ? result.uiPatches.length : 0;
          timings.status = result && result.status && typeof result.status.tag === "string" ? result.status.tag : String(result && result.status);
          // 🫀️ THE step boundary. A guest running a BUDGETED job (a resumable tessellation, a
          // resumable boolean) crosses this point once per step and blocks the event loop in
          // between, so the while-busy ticker cannot fire and the only thing that distinguishes it
          // from a dead worker is a beat taken HERE, the moment the guest hands control back
          // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
          // \`📓️extension-evaluate-budget-2026-09-12.md\`). It RIDES the reply rather than costing its
          // own \`postMessage\`, because the reply is already crossing at this exact instant and
          // \`ShardClient\` reads liveness off every inbound message it receives.
          reply(requestId, result, timings, beat("turn-step"));
        } finally {
          inFlightTurnActors.delete(actorId);
          if (actor.disposeRequested) await closeOriginalActor(actorId, actor.activationGeneration);
        }
        break;
      }
      case "startJob":
        await actor.api.startJob(msg.job, msg.jobKind, msg.input);
        reply(requestId, undefined);
        break;
      case "stepJob": {
        // 🫀️ Same step boundary as \`turn\` above: a job step is exactly the unit a budgeted guest
        // yields at, so it is exactly where liveness is provable.
        const step = await actor.api.stepJob(msg.job, msg.budget);
        reply(requestId, step, undefined, beat("turn-step"));
        break;
      }
      case "cancelJob":
        await actor.api.cancelJob(msg.job);
        reply(requestId, undefined);
        break;
      case "takeSegmentedDownloadChunk": {
        if (!Number.isSafeInteger(msg.instanceId) || msg.instanceId < 0 || typeof msg.operationId !== "bigint" || msg.operationId <= 0n || msg.operationId > ((1n << 64n) - 1n)) throw new Error("segmented-download-authority-invalid");
        const chunk = await actor.api.takeSegmentedDownloadChunk(msg.instanceId, msg.operationId);
        // 📤️ One bound per refusal, never one "limit" for four different violations: a chunk of the wrong
        // TYPE is a wire-adaptation defect (the guest's \`option<list<u8>>\` reaching this worker as jco's
        // tagged \`{ tag, val }\` object instead of a \`Uint8Array\` — what the 2026-09-12 battery read as
        // an exceeded byte cap while nothing was over any cap), an empty or oversized one is a producer
        // that sliced by the wrong constant.
        if (chunk !== undefined && chunk !== null) {
          if (Object.prototype.toString.call(chunk) !== "[object Uint8Array]") throw new Error("segmented-download-chunk-type");
          if (chunk.byteLength === 0) throw new Error("segmented-download-chunk-empty");
          if (chunk.byteLength > SEGMENTED_DOWNLOAD_CHUNK_BYTES) throw new Error("segmented-download-chunk-over-cap");
        }
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
      case "codec": {
        if (actor.activationGeneration !== msg.activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
        if (inFlightTurnActors.has(actorId)) throw new Error(\`shard worker: actor \${actorId} already has a turn in flight\`);
        inFlightTurnActors.add(actorId);
        try {
          reply(requestId, await actor.api.codec(msg.request), undefined, beat("turn-step"));
        } finally {
          inFlightTurnActors.delete(actorId);
        }
        break;
      }
      case "frame": {
        if (actor.activationGeneration !== msg.activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
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
          timings.events = events.length;
          timings.guestEnteredAtEpochMs = hopEpochNow();
          const polled = await actor.api.poll(events, undefined, undefined, result.budget);
          timings.guestLeftAtEpochMs = hopEpochNow();
          reply(requestId, polled, timings);
        } finally {
          inFlightTurnActors.delete(actorId);
        }
        break;
      }
      default:
        throw new Error(\`unknown shard worker message kind: \${kind}\`);
    }
  } catch (error) {
    // 🩺️ Reported on BOTH channels on purpose: \`replyError\` answers the one caller that is awaiting
    // this request, \`reportWorkerFault\` names the phase/actor/module to the shell's console for the
    // boot faults nobody is awaiting a reply for. A retryable lifecycle-turn deadline is neither —
    // it is a yield the client replays, so it never reaches \`onActorTrap\`.
    const retryableLifecycle = kind === "turn" && retryableLifecycleTurn(error, msg.events, msg.commandPage);
    if (retryableLifecycle) console.log(\`shard worker: retryable lifecycle deadline on \${faultPhase} for actor \${actorId}; receipt retained, replaying next tick\`);
    else reportWorkerFault("handler", error, null);
    replyError(requestId, error, msg.events, retryableLifecycle);
  } finally {
    endRequest();
    faultPhase = "idle";
  }
});
`;
}
