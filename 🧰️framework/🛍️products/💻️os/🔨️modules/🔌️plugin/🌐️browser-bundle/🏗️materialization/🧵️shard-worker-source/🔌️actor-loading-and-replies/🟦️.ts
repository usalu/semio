/** 🔌️ The shard worker reply, runtime diagnostics and actor loading text. */
import { SHARD_RUNTIME_DIAGNOSTICS_KEY, SHARD_WORKER_DIAGNOSTICS_PARAM } from "../../🧾️contract/🟦️.ts";

/** 📦 Returns the worker text of the reply helpers, the runtime diagnostics and the actor loading and closing. */
export function shardWorkerActorLoadingSource(): string {
  return `const driveYieldChannel = typeof MessageChannel === "function" ? new MessageChannel() : null;
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
`;
}
