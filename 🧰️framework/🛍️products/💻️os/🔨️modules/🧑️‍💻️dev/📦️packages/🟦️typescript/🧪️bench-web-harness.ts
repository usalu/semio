/** 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (bench-web-rows): the web half of the 50-plugin ×
 * 50-extension scale proof. Bundled for the BROWSER (see `📜️script.ts`'s `buildBenchWebHarnessBundle`,
 * `Bun.build({ target: "browser" })`) and run inside a real headless Chromium page (Playwright — already
 * a repo dependency, same pattern this same `📜️script.ts` already uses for the collab/studio e2e's) —
 * NEVER inside Bash, matching the ticket's "Use the Browser pane for anything needing a page" rule for
 * anything that needs one.
 *
 * ⚠️ HONESTY SCOPE — read before trusting any row this file produces. `semio-framework-plugin` (the
 * guest SDK) does not compile yet this session, so **no real fleet wasm component exists** to activate.
 * This harness therefore drives the REAL, production `ShardClient` (`🎭️actor/📦️packages/🟦️typescript/
 * 🧵️shard-client.ts` — unmodified, imported verbatim) against REAL browser `Worker`s, but each worker
 * runs a tiny STUB protocol handler (`STUB_SHARD_WORKER_SOURCE` below) instead of the real
 * `shardWorkerSource()` (which `import()`s a compiled jco bridge module that does not exist yet).
 * Consequently:
 *   - Budgets 3/4/6/7/8 below are STRUCTURAL: they exercise `ShardClient`'s own real sharding,
 *     heartbeat/trap, terminate/rebuild, and checkpoint/restore wire logic, which is renderer- and
 *     wasm-independent. A pass here is a real pass of that logic at 100-actor scale in a real browser.
 *   - Budgets 2/5 are TIMING measurements that necessarily exclude real per-plugin wasm instantiation
 *     and real guest compute (the dominant real-world cost) — `📜️script.ts` labels their status
 *     `pass-stub-worker`/`fail-stub-worker` rather than plain `pass`/`fail` so this is visible in the
 *     report without having to read this file's own doc.
 *   - Capability "revocation" (budget 8) has no dedicated wire message on `ShardClient` today (only
 *     `activate`'s `caps` list at grant time) — this test activates an actor WITHOUT the capability it
 *     will be asked for, standing in for "revoked", not a genuine revoke-mid-life round trip. Flagged
 *     again in `📜️script.ts`'s row `note`.
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-bench-web-rows-report.md
 */
import { ShardClient, type ShardBudget, type ShardWorkerLike } from "../../../../../../../🧰️framework/🔨️modules/🎭️actor/🧵️shard-client/🟦️.ts";
import { OwnedResidentLedger } from "../../../../../../../🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️.ts";

//#region 🟨️StubWorker
/** 🟨️ Protocol-faithful stand-in for the real, generated `🟨️shard-worker.js` (`shardWorkerSource()`).
 * Implements exactly the subset of `ShardClient`'s wire contract this harness exercises —
 * `activate`/`turn`/`checkpoint`/`restore`/`dispose` — with synthetic, deterministic per-actor state
 * (a single `counter`) instead of a real wasm guest. `moduleUrl` doubles as this stub's own tiny config
 * channel (opaque to `ShardClient` itself, which never inspects it): `stub://hang?overrunMs=N` marks an
 * actor that never answers `turn` and instead posts a `trap` frame after `N`ms, simulating a guest whose
 * own fuel/wallMs accounting caught a runaway turn — everything else activates as `stub://ok`. */
const STUB_SHARD_WORKER_SOURCE = `
const actors = new Map();
function parseModuleUrl(moduleUrl) {
  if (typeof moduleUrl === "string" && moduleUrl.startsWith("stub://hang")) {
    const match = /overrunMs=(\\d+)/.exec(moduleUrl);
    return { hang: true, overrunMs: match ? Number(match[1]) : 25 };
  }
  return { hang: false, overrunMs: 0 };
}
self.onmessage = (event) => {
  const msg = event.data;
  if (!msg || typeof msg !== "object") return;
  if (msg.kind === "attachHeartbeatSab") return;
  if (msg.kind === "activate") {
    const cfg = parseModuleUrl(msg.moduleUrl);
    const caps = new Set((msg.caps || []).map((c) => c.id));
    actors.set(msg.actorId, { caps, counter: 0, hang: cfg.hang, overrunMs: cfg.overrunMs });
    self.postMessage({ kind: "result", requestId: msg.requestId, ok: true, value: undefined });
    return;
  }
  if (msg.kind === "turn") {
    const actor = actors.get(msg.actorId);
    if (!actor) { self.postMessage({ kind: "result", requestId: msg.requestId, ok: false, error: "stub: unknown actor " + msg.actorId }); return; }
    if (actor.hang) {
      setTimeout(() => self.postMessage({ kind: "trap", actorId: msg.actorId, message: "wallMs budget exceeded (2x) [stub]" }), actor.overrunMs);
      return; // deliberately never resolves this requestId — the caller must terminate()/rebuild() the shard
    }
    actor.counter += 1;
    const firstEvent = Array.isArray(msg.events) ? msg.events[0] : undefined;
    const requiredCap = firstEvent && firstEvent.payload && firstEvent.payload.requireCapability;
    if (requiredCap && !actor.caps.has(requiredCap)) {
      self.postMessage({ kind: "result", requestId: msg.requestId, ok: true, value: { denied: true, capability: requiredCap, counter: actor.counter } });
      return;
    }
    self.postMessage({ kind: "result", requestId: msg.requestId, ok: true, value: { effects: [], counter: actor.counter } });
    return;
  }
  if (msg.kind === "checkpoint") {
    const actor = actors.get(msg.actorId);
    const bytes = new Uint8Array(4);
    new DataView(bytes.buffer).setUint32(0, actor ? actor.counter : 0, true);
    self.postMessage({ kind: "result", requestId: msg.requestId, ok: true, value: bytes });
    return;
  }
  if (msg.kind === "restore") {
    const view = new DataView(msg.state.buffer, msg.state.byteOffset, msg.state.byteLength);
    const existing = actors.get(msg.actorId) || { caps: new Set(), counter: 0, hang: false, overrunMs: 0 };
    existing.counter = view.getUint32(0, true);
    actors.set(msg.actorId, existing);
    self.postMessage({ kind: "result", requestId: msg.requestId, ok: true, value: undefined });
    return;
  }
  if (msg.kind === "dispose") { actors.delete(msg.actorId); return; }
  // startJob/stepJob/cancelJob/frame: not exercised by this harness — intentionally unhandled.
};
`;

function createStubWorker(sink: ShardWorkerLike[]): ShardWorkerLike {
  const blob = new Blob([STUB_SHARD_WORKER_SOURCE], { type: "application/javascript" });
  const url = URL.createObjectURL(blob);
  const worker = new Worker(url) as unknown as ShardWorkerLike;
  sink.push(worker);
  return worker;
}
//#endregion 🟨️StubWorker

//#region 🧮️Helpers
function percentile(sortedAscending: readonly number[], p: number): number {
  if (sortedAscending.length === 0) return 0;
  const index = Math.min(sortedAscending.length - 1, Math.ceil((p / 100) * sortedAscending.length) - 1);
  return sortedAscending[Math.max(0, index)]!;
}

/** 🔢️ FNV-1a — deterministic, dependency-free (no `crypto.subtle`, whose secure-context availability on
 * a bare `page.setContent` document is not worth depending on for a bench harness). Only used to compare
 * checkpoint bytes before/after a restore round trip; not a security primitive. */
function fnv1aHex(bytes: Uint8Array): string {
  let hash = 0x811c9dc5;
  for (let i = 0; i < bytes.length; i++) {
    hash ^= bytes[i]!;
    hash = Math.imul(hash, 0x01000193);
  }
  return (hash >>> 0).toString(16).padStart(8, "0");
}

function bytesEqual(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return false;
  return true;
}
//#endregion 🧮️Helpers

const STUB_BUDGET: ShardBudget = { fuel: 1_000_000, wallMs: 16, memoryBytes: 32 * 1024 * 1024, uiNodes: 256, mailboxLen: 32, maxEffects: 16, maxPatchBytes: 65_536 };

export type BenchWebRawRow = { readonly id: number; readonly ok: boolean; readonly measured: unknown; readonly note: string };

export type BenchWebInput = {
  readonly pluginIds: readonly string[];
  readonly firstPluginExtensionIds: readonly string[];
  readonly shardCount: number;
};

//#region ▶️Run
export async function runBenchWebBudgets(input: BenchWebInput): Promise<BenchWebRawRow[]> {
  const rows: BenchWebRawRow[] = [];
  const mainWorkers: ShardWorkerLike[] = [];
  const t0 = performance.now();
  // 🚫️ exclusiveShardCount:0 — this proof activates the WHOLE pool as round-robin so "distinct shards
  // used" measures all K, matching the budget-3 wording ("shards==K") rather than K minus the ≤2 shards
  // `ShardClient`'s default reserves for `leaseExclusive`, which this bench never calls.
  const residentLedger = new OwnedResidentLedger({ bytes: 33554432, slots: 262144, owners: 262144, control: { bytes: 65536, slots: 1024, owners: 1024 } });
  const client = new ShardClient({ residentLedger, shardCount: input.shardCount, exclusiveShardCount: 0, createWorker: () => createStubWorker(mainWorkers) });

  //#region 2️⃣ Cold boot to first interactive frame (STUB-WORKER TIMING — see file header)
  const firstActorId = input.pluginIds[0]!;
  await client.activate(firstActorId, "stub://ok", [], STUB_BUDGET);
  const coldBootMs = performance.now() - t0;
  rows.push({ id: 2, ok: true, measured: { coldBootMs, shardCount: input.shardCount }, note: "STUB-WORKER TIMING: real Worker pool spin-up + one real activate() round trip, NO real plugin wasm instantiated (semio-framework-plugin does not compile this session) — excludes the dominant real-world cold-boot cost. Lower bound only." });
  //#endregion

  //#region 3️⃣ Activate 50 plugins + 50 extensions of plugin[0]: active_actors==100, shards==K, no shard>ceil(100/K)+1
  const allActors = [...input.pluginIds, ...input.firstPluginExtensionIds];
  await Promise.all(allActors.map((id) => (id === firstActorId ? Promise.resolve() : client.activate(id, "stub://ok", [], STUB_BUDGET))));
  const shardOf = new Map<string, number>();
  for (const id of allActors) {
    const shard = client.shardIndexFor(id);
    if (shard !== undefined) shardOf.set(id, shard);
  }
  const perShardCounts = new Map<number, number>();
  for (const shard of shardOf.values()) perShardCounts.set(shard, (perShardCounts.get(shard) ?? 0) + 1);
  const activeActors = shardOf.size;
  const distinctShards = perShardCounts.size;
  const maxPerShard = perShardCounts.size > 0 ? Math.max(...perShardCounts.values()) : 0;
  const ceilBound = Math.ceil(allActors.length / input.shardCount) + 1;
  const budget3Ok = activeActors === 100 && distinctShards === input.shardCount && maxPerShard <= ceilBound;
  rows.push({ id: 3, ok: budget3Ok, measured: { activeActors, distinctShards, expectedShards: input.shardCount, maxPerShard, ceilBound }, note: "REAL: ShardClient's own round-robin assignShard() at 100-actor scale (stub workers, real client logic)." });
  //#endregion

  //#region 4️⃣ Memory ceiling — web: Worker count == K (native RSS N/A on web)
  const workerCountOk = mainWorkers.length === input.shardCount;
  rows.push({ id: 4, ok: workerCountOk, measured: { workerCount: mainWorkers.length, expectedShardCount: input.shardCount }, note: "REAL: counts actual browser Worker instances ShardClient spawned. Bytes-level memory ceiling (K×512MiB+256MiB) is not evaluable without real wasm modules resident — not attempted." });
  //#endregion

  //#region 5️⃣ Interactive p95 command->patch (STUB-WORKER TIMING — see file header)
  const TURN_SAMPLES = 50;
  const latenciesMs: number[] = [];
  for (let i = 0; i < TURN_SAMPLES; i++) {
    const turnStart = performance.now();
    await client.turn(firstActorId, [{ kind: "app-command", payload: { command: "noop" } }], STUB_BUDGET);
    latenciesMs.push(performance.now() - turnStart);
  }
  const sorted = [...latenciesMs].sort((a, b) => a - b);
  const p95Ms = percentile(sorted, 95);
  rows.push({ id: 5, ok: true, measured: { p95Ms, samples: TURN_SAMPLES, minMs: sorted[0] ?? 0, maxMs: sorted[sorted.length - 1] ?? 0 }, note: "STUB-WORKER TIMING: real postMessage/structured-clone round trip latency for a no-op turn on one actor while 99 siblings sit idle on the pool; excludes real guest compute and UI patch application. Lower bound only." });
  //#endregion

  //#region 6️⃣ Hang actor killed and rebuilt, siblings restored, total pause budget
  const hangWorkers: ShardWorkerLike[] = [];
  const trapEvents: { actorId: string; message: string }[] = [];
  const hangClient = new ShardClient({ residentLedger, shardCount: 1, exclusiveShardCount: 0, createWorker: () => createStubWorker(hangWorkers), onActorTrap: (actorId, message) => trapEvents.push({ actorId, message }) });
  await hangClient.activate("bench-hang-actor", "stub://hang?overrunMs=25", [], STUB_BUDGET);
  await hangClient.activate("bench-sibling-a", "stub://ok", [], STUB_BUDGET);
  await hangClient.activate("bench-sibling-b", "stub://ok", [], STUB_BUDGET);
  await hangClient.turn("bench-sibling-a", [], STUB_BUDGET);
  await hangClient.turn("bench-sibling-a", [], STUB_BUDGET);
  const siblingABefore = await hangClient.checkpoint("bench-sibling-a");
  await hangClient.turn("bench-sibling-b", [], STUB_BUDGET);
  const siblingBBefore = await hangClient.checkpoint("bench-sibling-b");

  const pauseStart = performance.now();
  void hangClient.turn("bench-hang-actor", [], STUB_BUDGET).catch(() => {});
  const trapDeadline = pauseStart + 2000;
  while (trapEvents.length === 0 && performance.now() < trapDeadline) await new Promise((resolve) => setTimeout(resolve, 2));
  let totalPauseMs = Number.POSITIVE_INFINITY;
  let siblingsRestored = false;
  if (trapEvents.length > 0) {
    const hangShardIndex = hangClient.shardIndexFor("bench-hang-actor")!;
    hangClient.terminate(hangShardIndex);
    hangClient.rebuild(hangShardIndex);
    await hangClient.activate("bench-sibling-a", "stub://ok", [], STUB_BUDGET);
    await hangClient.restore("bench-sibling-a", siblingABefore);
    await hangClient.activate("bench-sibling-b", "stub://ok", [], STUB_BUDGET);
    await hangClient.restore("bench-sibling-b", siblingBBefore);
    totalPauseMs = performance.now() - pauseStart;
    const siblingAAfter = await hangClient.checkpoint("bench-sibling-a");
    const siblingBAfter = await hangClient.checkpoint("bench-sibling-b");
    siblingsRestored = bytesEqual(siblingABefore, siblingAAfter) && bytesEqual(siblingBBefore, siblingBAfter);
  }
  const budget6Ok = trapEvents.length > 0 && totalPauseMs <= 250 && siblingsRestored;
  rows.push({ id: 6, ok: budget6Ok, measured: { trapped: trapEvents.length > 0, totalPauseMs: Number.isFinite(totalPauseMs) ? totalPauseMs : null, siblingsRestored }, note: "REAL: worker sends a trap frame (simulating a guest fuel/wallMs overrun) → real ShardClient.terminate()+rebuild() → real re-activate+restore of two siblings from real checkpoint bytes taken before the hang, all real postMessage round trips." });
  hangClient.disposeAll();
  //#endregion

  //#region 7️⃣ Checkpoint/resume state-hash equality
  const statefulActorId = input.pluginIds[1]!;
  await client.activate(statefulActorId, "stub://ok", [], STUB_BUDGET);
  await client.turn(statefulActorId, [], STUB_BUDGET);
  await client.turn(statefulActorId, [], STUB_BUDGET);
  await client.turn(statefulActorId, [], STUB_BUDGET);
  const stateBefore = await client.checkpoint(statefulActorId);
  const hashBefore = fnv1aHex(stateBefore);
  client.dispose(statefulActorId);
  await client.activate(statefulActorId, "stub://ok", [], STUB_BUDGET);
  await client.restore(statefulActorId, stateBefore);
  const stateAfter = await client.checkpoint(statefulActorId);
  const hashAfter = fnv1aHex(stateAfter);
  rows.push({ id: 7, ok: hashBefore === hashAfter, measured: { hashBefore, hashAfter, stateByteLength: stateBefore.length }, note: "REAL: checkpoint()→dispose()→activate()→restore()→checkpoint() end-to-end over real postMessage/structured-clone Uint8Array transfer; hash compares the two checkpoints byte-for-byte." });
  //#endregion

  //#region 8️⃣ Capability revocation
  const capActorId = "bench-capability-actor";
  const requiredCapability = "scale-fixture.io.example";
  await client.activate(capActorId, "stub://ok", [], STUB_BUDGET); // 🚫️ deliberately NO caps granted — see file header's honesty note
  const denialResult = (await client.turn(capActorId, [{ kind: "app-command", payload: { requireCapability: requiredCapability } }], STUB_BUDGET)) as { denied?: boolean };
  const stillAlive = client.shardIndexFor(capActorId) !== undefined;
  rows.push({ id: 8, ok: denialResult.denied === true && stillAlive, measured: { denied: denialResult.denied === true, actorAlive: stillAlive }, note: "PARTIAL: ShardClient has no dedicated revoke-mid-life wire message — this activates an actor WITHOUT the capability up front (standing in for 'revoked') and confirms the stub denies the effect while the actor stays registered. Not a genuine runtime-revocation-of-a-live-grant round trip." });
  //#endregion

  client.disposeAll();
  return rows;
}
//#endregion ▶️Run
