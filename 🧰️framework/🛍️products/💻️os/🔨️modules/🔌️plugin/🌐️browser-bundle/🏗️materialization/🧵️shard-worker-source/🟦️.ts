/** 🧵️ Shard worker source of the plugin web materialization: the one package agnostic generated worker that every actor of a shard pool activates through. */

import { shardWorkerBootstrapSource } from "./🥾️worker-bootstrap-and-faults/🟦️.ts";
import { shardWorkerActorStateSource } from "./🎬️actor-state-and-lanes/🟦️.ts";
import { shardWorkerDrivePacingSource } from "./🚦️request-beats-and-drive-pacing/🟦️.ts";
import { shardWorkerActorLoadingSource } from "./🔌️actor-loading-and-replies/🟦️.ts";
import { shardWorkerMessageHandlerSource } from "./📨️message-handler/🟦️.ts";

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
  return [shardWorkerBootstrapSource(), shardWorkerActorStateSource(), shardWorkerDrivePacingSource(), shardWorkerActorLoadingSource(), shardWorkerMessageHandlerSource()].join("");
}
