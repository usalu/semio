import { SHARD_LIVENESS_POLICY } from "../🫀️liveness/🟦️.ts";

/** 🚦️ terra-shard-effect-bridge: default cap on CONCURRENT unresolved `effect-request`s per actor —
 * see {@link ShardClientOptions.maxOutstandingEffectsPerActor}'s own doc for why this mirrors
 * `QuotaSchema.outstanding_requests` without being it. */
export const DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR = 64;

export type ShardHeartbeatState = {
  lastHeartbeatAtMs: number;
  lastHeartbeatTurnSeq: number;
  lastHeartbeatPhase: string | null;
  lastLivenessAtMs: number;
  oldestPendingStartedAtMs: number | null;
  missedCount: number;
  lastMissCountedAtMs: number;
};

/** 🌱️ A freshly spawned shard has never actually heartbeated yet — `-Infinity` (never `nowMs`) so a turn
 * that starts in the very same tick as `spawnShard` doesn't spuriously count spawn-time as proof of
 * life for that turn's whole timeout window. */
export function freshHeartbeatState(nowMs: number): ShardHeartbeatState {
  return { lastHeartbeatAtMs: Number.NEGATIVE_INFINITY, lastHeartbeatTurnSeq: 0, lastHeartbeatPhase: null, lastLivenessAtMs: Number.NEGATIVE_INFINITY, oldestPendingStartedAtMs: null, missedCount: 0, lastMissCountedAtMs: nowMs };
}

/** 🫀️ One watchdog window's worth of input — every field the rule below reads, and nothing else, so
 * the same decision can be replayed from a JSON timeline with no `ShardClient` in the picture. */
export type ShardLivenessWindow = {
  readonly nowMs: number;
  readonly oldestPendingStartedAtMs: number | null;
  readonly lastLivenessAtMs: number;
  readonly missedCount: number;
  readonly lastMissCountedAtMs: number;
  readonly heartbeatTimeoutMs: number;
  readonly firstTurnTimeoutMs: number;
  readonly oldestPendingIsFirstTurn: boolean;
};

export type ShardLivenessDecision = {
  readonly missedCount: number;
  readonly lastMissCountedAtMs: number;
  readonly terminate: boolean;
};

/** 🚑️ design-runtime.md §1 `FailurePolicy`, restated so BUSY and DEAD stop being the same thing.
 *
 * A shard is only a candidate while it has an in-flight request — an idle shard can never be flagged.
 * The clock that matters is `lastLivenessAtMs`: the last moment ANY message arrived from that worker
 * (a start-of-request heartbeat, one of the generated worker's boundary/progress heartbeats, a
 * `result`, a `trap`, an effect `frame`). `max(lastLivenessAtMs, oldestPendingStartedAtMs)` is the
 * newest instant this shard is PROVEN to have been alive with work outstanding; anything older than
 * `heartbeatTimeoutMs` is real silence and costs one miss per window, `missedLimit` of them in a row
 * terminating the worker.
 *
 * The pre-fix rule compared `lastHeartbeatAtMs >= oldestPendingStartedAtMs` — an absolute timestamp
 * against a DIFFERENT request's start — so a worker that heartbeated once and then wedged forever
 * looked healthy, while a worker legitimately busy inside one multi-second `await import()` of a
 * multi-MB wasm component (nothing to heartbeat about mid-turn) was killed the moment an unrelated
 * newer request became the oldest pending. That is the `shard 0 terminated` boot fault this rule
 * replaces: liveness is now proven CONTINUOUSLY by the worker's progress ticker, so "busy" keeps
 * proving itself and "dead" — no messages at all — still dies after exactly the same `missedLimit`
 * windows.
 *
 * `firstTurnTimeoutMs` is the SECOND half of that rule, and it exists because the progress ticker's
 * premise ("it can only fire while the worker's event loop is running", therefore silence means
 * death) is FALSE for the one phase that matters at boot. A guest turn is synchronous on the
 * worker's single thread: JSPI suspends only at a host import, so a compute-bound turn runs to
 * completion with the event loop blocked and the `setInterval` ticker never fires. Measured
 * headlessly against the live staged components (`🐍️shard-component-probe.mjs`, ticket
 * `2026/09/09/PROCEDURAL-3D-END-TO-END`): every actor's FIRST `poll` after activation costs
 * 2 964–3 362 ms with a 1 s ticker firing **0** times inside it, while its second poll costs 1–2 ms.
 * The first turn carries one-off guest initialization; it is not a class of work the ordinary
 * `heartbeatTimeoutMs` ladder can measure, and pricing it there is what terminated a perfectly
 * healthy shard 0 mid-boot with no fault text at all. So an actor's first turn is measured against
 * its own schema-owned ceiling, and everything after it is back on the ordinary ladder. */
export function evaluateShardLiveness(window: ShardLivenessWindow): ShardLivenessDecision {
  const unchanged = { missedCount: window.missedCount, lastMissCountedAtMs: window.lastMissCountedAtMs, terminate: false };
  if (window.oldestPendingStartedAtMs === null) return unchanged;
  const timeoutMs = window.oldestPendingIsFirstTurn ? Math.max(window.heartbeatTimeoutMs, window.firstTurnTimeoutMs) : window.heartbeatTimeoutMs;
  const provenAliveAtMs = Math.max(window.lastLivenessAtMs, window.oldestPendingStartedAtMs);
  if (window.nowMs - provenAliveAtMs <= timeoutMs) return unchanged;
  if (window.nowMs - window.lastMissCountedAtMs < timeoutMs) return unchanged;
  const missedCount = window.missedCount + 1;
  return { missedCount, lastMissCountedAtMs: window.nowMs, terminate: missedCount >= SHARD_LIVENESS_POLICY.missedLimit };
}

/** 🩺️ Names the one failure the shell's boot path may retry rather than surface: the watchdog above
 * (or a `worker.onerror` crash) took this shard down under load, `rebuild()` already replaced it, and
 * every request in flight was rejected with this message. */
export function isShardLostError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : typeof error === "string" ? error : "";
  return /shard \d+ (?:terminated|worker crashed)/.test(message);
}

/** 🐝️ A `Worker`'s `onerror` hands the main thread an `ErrorEvent`, and `console.error`-ing that
 * object prints `Event` and nothing else — which is all the boot log said while four shards died on a
 * top-of-file `throw` inside the worker script. Reads the three fields that actually name the cause
 * (`message`/`filename`/`lineno`) and degrades explicitly when the event is the redacted, message-less
 * kind a cross-origin or failed-to-load worker script produces. */
export function describeShardWorkerError(event: unknown): string {
  const record = (event ?? {}) as { readonly message?: unknown; readonly filename?: unknown; readonly lineno?: unknown; readonly colno?: unknown; readonly type?: unknown; readonly error?: { readonly message?: unknown } };
  const message =
    typeof record.message === "string" && record.message.length > 0
      ? record.message
      : typeof record.error?.message === "string" && record.error.message.length > 0
        ? record.error.message
        : typeof record.type === "string"
          ? `redacted "${record.type}" event with no message — the worker script threw before it could report, or failed to load`
          : String(event);
  if (typeof record.filename !== "string" || record.filename.length === 0) return message;
  return `${message} at ${record.filename}:${typeof record.lineno === "number" ? record.lineno : "?"}:${typeof record.colno === "number" ? record.colno : "?"}`;
}

/** 🔇️ What one shard was doing when the watchdog gave up on it. Every field is state `ShardClient`
 * already holds; the point is that `shard 0 terminated` on its own names neither the actor, the
 * phase, nor how long the silence actually lasted — which is precisely how a boot-time termination
 * reaches a console looking like a spontaneous, causeless worker death. */
export type ShardSilenceReport = {
  readonly shardIndex: number;
  readonly nowMs: number;
  readonly lastLivenessAtMs: number;
  readonly lastHeartbeatPhase: string | null;
  readonly inFlight: readonly { readonly kind: string; readonly actorId: string; readonly startedAtMs: number; readonly firstTurn: boolean }[];
};

/** 🧾️ Composes {@link ShardSilenceReport} into the one line the terminated shard's rejections carry.
 * Keeps the `shard <n> terminated` prefix {@link isShardLostError} matches, then says how long the
 * silence lasted, what the worker last reported, and every request that was still outstanding — so a
 * host-side kill can never again be mistaken for a guest trap (or for nothing at all). */
export function describeShardSilence(report: ShardSilenceReport): string {
  const silence = Number.isFinite(report.lastLivenessAtMs) ? `silent for ${Math.max(0, Math.round(report.nowMs - report.lastLivenessAtMs))} ms` : "never sent a single message";
  const phase = report.lastHeartbeatPhase ? `, last reported phase "${report.lastHeartbeatPhase}"` : "";
  const requests = report.inFlight.length === 0
    ? "no request was outstanding"
    : report.inFlight
        .map((entry) => `${entry.kind}${entry.firstTurn ? " (first turn)" : ""} ${entry.actorId || "<no actor>"} started ${Math.max(0, Math.round(report.nowMs - entry.startedAtMs))} ms ago`)
        .join("; ");
  return `shard ${report.shardIndex} terminated by the host watchdog: the worker was ${silence}${phase}; outstanding: ${requests}. A guest turn that never yields blocks the worker's event loop and its progress ticker with it, so this reads identically to a dead worker — check the guest's own turn cost before suspecting a crash.`;
}

/** 🧯 Settles a failed instance open: runs the instance's cleanup, then rejects with the ORIGINAL
 * cause. A cleanup fault is reported on its own line and never becomes the caller's error — the
 * message that reaches the shell must name the fault that actually happened (a guest trap, a
 * terminated shard, a missing app) rather than whatever the teardown tripped over afterwards. Both
 * plugin bridges route their failed `createApp` through this one function, because both had grown
 * the same masking bug independently: a lifecycle captured before `open` leaves no UI owner behind,
 * and the teardown's own `…-ui.native-owner-required` replaced every real cause with itself. */
export async function settleFailedInstanceOpen(error: unknown, cleanup: () => Promise<void>): Promise<never> {
  try {
    await cleanup();
  } catch (cleanupError) {
  }
  throw error;
}

/** 💬️ A `messageerror` is the one worker event that is NEITHER a fault in the worker NOR a fault in
 * this client: a message crossed the boundary and could not be deserialized, so a request silently
 * never arrives (or a result silently never lands) and the shard simply goes quiet. Naming it is the
 * whole fix — it was not observed at all before. */
export function describeShardMessageError(shardIndex: number, event: unknown): string {
  const record = (event ?? {}) as { readonly type?: unknown; readonly data?: unknown };
  const shape = record.data === undefined ? "" : ` (undeserializable payload of type ${typeof record.data})`;
  return `shard ${shardIndex} worker message error: a structured-clone message could not be deserialized${shape} — the request or result it carried is lost`;
}

/** 👷️ One readable line out of a `worker-fault` payload — phase, actor, module URL and location, in
 * that order, so a boot log names WHERE inside the worker it died rather than only THAT it did. */
export function formatShardWorkerFault(shardIndex: number, fault: { readonly source: string; readonly phase: string; readonly actorId: string | null; readonly moduleUrl: string | null; readonly message: string; readonly filename?: string; readonly lineno?: number }): string {
  const where = fault.filename ? ` at ${fault.filename}:${fault.lineno ?? "?"}` : "";
  const actor = fault.actorId ? ` actor=${fault.actorId}` : "";
  const module = fault.moduleUrl ? ` module=${fault.moduleUrl}` : "";
  return `shard ${shardIndex} worker fault [${fault.source}/${fault.phase}]${actor}${module}: ${fault.message}${where}`;
}

/** 🧪️ The one capability every shard worker needs before it can host anything: jco's glue for the
 * fully async-lifted `world actor` unconditionally constructs `WebAssembly.Suspending` /
 * `WebAssembly.promising`, so without JSPI the worker script throws at MODULE TOP LEVEL — every shard
 * in the pool dies at spawn and the boot degrades into dozens of unrelated-looking load timeouts.
 * Probed on the main thread, BEFORE any worker exists, so that failure is reported once, by name,
 * instead of `missedLimit` windows later as four anonymous `Event`s. */
export const SHARD_JSPI_FAULT_CODE = "plugin.runtime.jspi-unavailable";

/** 🌐️ English first, German second, no default language (this is net-new operator vocabulary and the
 * shell's own chrome dictionary is not this module's to extend). */
export const SHARD_JSPI_FAULT_TEXT = Object.freeze({
  en: "This browser cannot run semio plugins: WebAssembly JavaScript Promise Integration (WebAssembly.Suspending / WebAssembly.promising) is unavailable. Chromium-based browsers ship it on by default; Firefox needs javascript.options.wasm_js_promise_integration in about:config, Node.js needs --experimental-wasm-jspi, and headless Chromium needs --enable-features=WebAssemblyJavaScriptPromiseIntegration.",
  de: "Dieser Browser kann semio-Plugins nicht ausführen: WebAssembly JavaScript Promise Integration (WebAssembly.Suspending / WebAssembly.promising) ist nicht verfügbar. Chromium-basierte Browser liefern sie standardmäßig aus; Firefox benötigt javascript.options.wasm_js_promise_integration in about:config, Node.js --experimental-wasm-jspi und headless Chromium --enable-features=WebAssemblyJavaScriptPromiseIntegration.",
});

export type ShardJspiScope = { readonly WebAssembly?: { readonly Suspending?: unknown; readonly promising?: unknown } };

export function shardJspiAvailable(scope: ShardJspiScope = globalThis as ShardJspiScope): boolean {
  const runtime = scope.WebAssembly;
  return typeof runtime === "object" && runtime !== null && typeof runtime.Suspending === "function" && typeof runtime.promising === "function";
}

/** ⚡️ Typed, bilingual, fail-fast form of the probe above — `code` is what `windowFaultFromError`
 * reads off the shell's error surface, `text` is what a human reads. */
export class ShardJspiUnavailableError extends Error {
  readonly code = SHARD_JSPI_FAULT_CODE;
  readonly text = SHARD_JSPI_FAULT_TEXT;
  constructor() {
    super(`${SHARD_JSPI_FAULT_CODE}: ${SHARD_JSPI_FAULT_TEXT.en} — ${SHARD_JSPI_FAULT_TEXT.de}`);
  }
}

export function assertShardJspiAvailable(scope: ShardJspiScope = globalThis as ShardJspiScope): void {
  if (!shardJspiAvailable(scope)) throw new ShardJspiUnavailableError();
}
