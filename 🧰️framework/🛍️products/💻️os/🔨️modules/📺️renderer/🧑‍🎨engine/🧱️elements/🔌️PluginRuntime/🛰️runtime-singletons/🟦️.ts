/** 🛰️ The process-wide actor-runtime singletons: shared shard client and activation registry, shard-loss recovery records, the boot shard-lost fault and the isolated-job drive store.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { rendererOriginalWorkStock } from "@semio-tech/framework/canvas/renderer-work";
import { ActivationRegistry } from "@semio-tech/framework";
import { assertShardJspiAvailable } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/💓️heartbeat/🟦️.ts";
import { ShardClient } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { type ShardBudget } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts";
import { type ShardWorkerLike } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🚨️quota/🟦️.ts";
import { shardWorkerUrl,DEFAULT_SHARD_BUDGET } from "../../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
import { OwnedResidentLedger } from "@semio-tech/framework/value/resident";
import { rendererResidentLedger } from "@semio-tech/framework/canvas/renderer-resident";
import { notePluginLoadProgressForInFlightV1 } from "../🫀️load-progress/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
/**
 * 🧵️ H1-react — replaces the deleted `acquirePluginModule`/`PluginModuleLease` (one Worker per
 * plugin, `📓️terra-H2-web-shard-report.md`'s "must not exist" list) with the pooled `ShardClient` +
 * `ActivationRegistry` design-runtime.md §1/§3 specifies. ONE `ShardClient` (bounded worker pool,
 * `min(hardwareConcurrency-1,4)` shards) and ONE `ActivationRegistry` (manifest-only activation, LRU
 * suspend/resume) for the whole tab — every `loadPluginModule` call shares them, matching the design's
 * "ShardClient... replaces PluginWorkerClient" framing (one pool, not one per caller). Lazily
 * constructed on first use so a pure SSR/test import of this module never touches `Worker`/
 * `navigator`. */
/** 🛣️ 🩺️ The shard worker URL is now the ONE `SHARD_WORKER_URL` `🎭️actor/🧵️shard-runtime/🟦️.ts`
 * already publishes (and the wgpu bridge already uses), asserted equal to the schema-owned
 * distribution route authority (`🔌️plugin/📇️registry/📦️deployment/🛣️routes.json` +
 * `MODULE_SHARD_DIRECTORY` + `SHARD_WORKER_FILE`) by that module's own suite.
 *
 * This file used to declare its own worker path: an ASCII transliteration of the route (`_shard`
 * beneath an ASCII `plugin-modules`) that exists NOWHERE else in the repository. Vite answered it with the
 * SPA fallback (`200 text/html`, measured against the live served shell on 2026-09-05, versus
 * `200 text/javascript` for the canonical route), a module `Worker` refuses an HTML body, and that
 * failure reaches the page as a parent-side `error` event carrying no message at all. Every shard in
 * the pool therefore died at spawn, the boot logged four anonymous `shard N worker error Event` lines,
 * and the ~60 downstream `timeout loading <plugin>` faults all descended from that one dead path. */

/** ⛽️ Provisional constant turn budget — same honestly-flagged gap `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s
 * native `TURN_BUDGET` documents ("until the DRR scheduler threads a real per-lane one through");
 * this is that same budget's web twin, field-for-field against `ShardBudget`. */
export {DEFAULT_SHARD_BUDGET};

/** 💼️ Isolated `step-job` slices that share one serialized actor admission.
 * One slice per admission lets a 120ms `fillBuildTick` poll starve a fill plan
 * (browser-measured 2026-09-10: 5 slices / 30s, still in token-admit, Count=0).
 * Native `ShardLoop::pump` grants many job slices per turn; the browser driver
 * must batch the same way. */
export function isolatedJobStepsPerSerializedAdmission(stepsPerYield: number): number {
  return Math.max(1, stepsPerYield);
}

/** 🗳️ True on every `stride` Isolated step so fillBuildTick can poll Count without grabbing every admission. */
export function isolatedJobUiPollEverySteps(step: number, stride = 128): boolean {
  return step > 0 && step % Math.max(1, stride) === 0;
}

let isolatedJobDriveDepth = 0;
let isolatedJobUiPollDue = 0;
const isolatedJobDriveListeners = new Set<() => void>();

function notifyIsolatedJobDrive(): void {
  for (const listener of isolatedJobDriveListeners) listener();
}

/** 🏗️ Marks an Isolated job pump as holding the actor so fillBuildTick can yield the serialize lock. */
export function beginIsolatedJobDrive(): void {
  isolatedJobDriveDepth += 1;
  notifyIsolatedJobDrive();
}

/** 🏗️ Ends one Isolated job pump; clears leftover UI-poll tokens when the last drive drops. */
export function endIsolatedJobDrive(): void {
  isolatedJobDriveDepth = Math.max(0, isolatedJobDriveDepth - 1);
  if (isolatedJobDriveDepth === 0) isolatedJobUiPollDue = 0;
  notifyIsolatedJobDrive();
}

/** 🏗️ Queues one fillBuildTick while a drive is active so the guest can poll Count without starving step-job. */
export function requestIsolatedJobUiPoll(): void {
  isolatedJobUiPollDue += 1;
  notifyIsolatedJobDrive();
}

/** 🏗️ Consumes one queued Isolated-job UI poll. */
export function takeIsolatedJobUiPoll(): boolean {
  if (isolatedJobUiPollDue <= 0) return false;
  isolatedJobUiPollDue -= 1;
  notifyIsolatedJobDrive();
  return true;
}

/** 🏗️ True while PluginRuntime is pumping an Isolated job. */
export function isolatedJobDriveIsActive(): boolean {
  return isolatedJobDriveDepth > 0;
}

/** 🏗️ Subscribe to Isolated-job drive / poll-token changes. */
export function subscribeIsolatedJobDrive(listener: () => void): () => void {
  isolatedJobDriveListeners.add(listener);
  return () => {
    isolatedJobDriveListeners.delete(listener);
  };
}

/** 🏗️ Snapshot of the Isolated-job drive store for `useSyncExternalStore`. */
export function isolatedJobDriveSnapshot(): { readonly driving: boolean; readonly pollDue: number } {
  return { driving: isolatedJobDriveDepth > 0, pollDue: isolatedJobUiPollDue };
}

/** 🧮️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): `min(hardwareConcurrency-1,
 * 4)` — the SAME physical bound `getShardClient`'s own worker-pool `shardCount` uses (design-runtime.md
 * §1 `ShardTable`), factored out once so it never drifts between the two call sites that both mean
 * "how many wasm-boundary hops can genuinely run at once on this device": the shard pool itself, and
 * {@link loadPluginModulesInDependencyOrder}'s per-level boot concurrency (that function's own doc has
 * the "why reuse this exact number" reasoning). Falls back to `5` (so the clamp lands on `4`) when
 * `navigator.hardwareConcurrency` is unavailable (SSR/test), matching `getShardClient`'s own fallback. */
export function poolConcurrency(): number {
  const hardwareConcurrency = typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" ? navigator.hardwareConcurrency : 5;
  return Math.max(1, Math.min(hardwareConcurrency - 1, 4));
}

/** 🚑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): `ShardClientOptions.onShardLost`'s
 * real production wiring — before this packet, the callback here only `console.error`'d (a shard's
 * whole actor roster silently stopped receiving turns, undetectable short of reading the console).
 * `ActivationRegistry.handleShardLost` (`🎠️kernel`, already coordinator-verified) is
 * the REAL restore path: it bumps each affected actor's generation, cancels its stale queue, and
 * `resume()`s it from its last checkpoint on a freshly rebuilt shard. Split out from
 * {@link buildShardClientOptions} (rather than inlined as an arrow there) purely so a test can call it
 * directly against a monkey-patched {@link sharedActivationRegistry} without constructing a real
 * `ShardClient`/`Worker`. */
export function handlePluginShardLost(shardIndex: number, actorIds: readonly string[]): void {
  // 🚑️ Restoring the ACTOR is not restoring the APP. `ActivationRegistry.restoreActor` re-activates
  // the program on a rebuilt shard and restores whatever checkpoint it has — and a watchdog kill
  // takes no checkpoint, so the guest comes back EMPTY: no `createApp` instance, no open document,
  // no lifecycle slot. Every later command page then fails the guest's own liveness check with
  // `plugin.command-page-invalid` and the session is dead until the user reloads, which is exactly
  // what was measured on 2026-09-12 after a 16 s `brep.bool.cut` tripped the watchdog
  // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`).
  //
  // So the instances those actors owned are retired HERE, before the restore, and announced — the
  // runtime never keeps serving an instance whose guest state no longer exists, and whoever created
  // the instance (the shell) is told it must create it again.
  const lost = releaseInstancesForLostActors(actorIds);
  getActivationRegistry().handleShardLost(shardIndex, actorIds);
  if (lost.length > 0) notifyPluginInstancesLost(shardIndex, lost);
}

/** 🚑️ One app instance that died with its shard — the plugin that owned it, the instance id the
 * shell knows it by, and the actor that hosted it. */
export type LostPluginInstance = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly actorId: string;
};

/** 🩺️ The typed fault a surface publishes when an app instance was lost with its worker and could
 * not be rebuilt. Distinct from `plugin.boot.shard-lost` on purpose: that one means the FIRST boot
 * was killed, this one means a live session was. */
export const PLUGIN_ACTOR_INSTANCE_LOST_FAULT = "plugin.actor-instance-lost";

/** 🚑️ Every live instance's recovery record, keyed by the actor that hosts it. Module-level (not a
 * per-handle closure) because `onShardLost` is a transport-level callback that knows only actor
 * ids — it has no handle to ask. Written by `createApp`, cleared by `destroyApp`/`dispose`. */
const instanceRecoveryByActor = new Map<string, LostPluginInstance & { readonly release: () => void }>();

const pluginInstanceLostSubscribers = new Set<(shardIndex: number, lost: readonly LostPluginInstance[]) => void>();

/** 🚑️ Subscribes to "an app instance died with its worker". Returns the unsubscribe. The shell is
 * the only correct listener: it is what called `createApp`, so it is the only party that can call it
 * again. */
export function onPluginInstancesLost(listener: (shardIndex: number, lost: readonly LostPluginInstance[]) => void): () => void {
  pluginInstanceLostSubscribers.add(listener);
  return () => { pluginInstanceLostSubscribers.delete(listener); };
}

function notifyPluginInstancesLost(shardIndex: number, lost: readonly LostPluginInstance[]): void {
  console.error(`PluginRuntime: ${PLUGIN_ACTOR_INSTANCE_LOST_FAULT} — ${lost.map((entry) => `${entry.pluginId}#${entry.instanceId}`).join(", ")} died with shard ${shardIndex} and must be re-created`);
  for (const listener of Array.from(pluginInstanceLostSubscribers)) {
    try {
      listener(shardIndex, lost);
    } catch (error) {
      console.error("PluginRuntime: an instance-lost listener threw", error);
    }
  }
}

/** 🧹️ Retires the runtime bookkeeping of every instance hosted by a lost actor and reports them.
 * Idempotent: an actor with no live instance (an extension's request actor, an instance already
 * destroyed) contributes nothing. */
function releaseInstancesForLostActors(actorIds: readonly string[]): readonly LostPluginInstance[] {
  const lost: LostPluginInstance[] = [];
  for (const actorId of actorIds) {
    const record = instanceRecoveryByActor.get(actorId);
    if (!record) continue;
    instanceRecoveryByActor.delete(actorId);
    lost.push({ pluginId: record.pluginId, instanceId: record.instanceId, actorId });
    try {
      record.release();
    } catch (error) {
      console.error(`PluginRuntime: releasing lost instance ${record.pluginId}#${record.instanceId} threw`, error);
    }
  }
  return lost;
}

/** 🚑️ Test seam + the one writer: binds an actor to the instance it hosts so a shard loss can
 * retire and announce it. */
export function rememberInstanceForRecovery(record: LostPluginInstance & { readonly release: () => void }): void {
  instanceRecoveryByActor.set(record.actorId, record);
}

/** 🧹️ Forgets a recovery record for an instance that closed normally. */
export function forgetInstanceForRecovery(actorId: string): void {
  instanceRecoveryByActor.delete(actorId);
}

/** 🎭️ Split out from {@link getShardClient} so a test can construct a REAL `ShardClient` (exercising
 * its actual lane/heartbeat/dispose machinery) against a FAKE `createWorker` — `getShardClient` itself
 * is untestable in isolation since it hardcodes a real DOM `Worker`, which this suite's `jsdom`
 * environment doesn't provide. `createWorker` defaults to the real `Worker` constructor for every
 * production call (`getShardClient` never passes an override). */
export function buildShardClientOptions(createWorker: () => ShardWorkerLike = () => new Worker(shardWorkerUrl(), { type: "module" }) as unknown as ShardWorkerLike): {
  readonly residentLedger: OwnedResidentLedger;
  readonly originalWorkStock: ReturnType<typeof rendererOriginalWorkStock>;
  readonly shardCount: number;
  readonly createWorker: () => ShardWorkerLike;
  readonly heartbeatTimeoutMs: number;
  readonly onActorTrap: (actorId: string, message: string) => void;
  readonly onShardLost: (shardIndex: number, actorIds: readonly string[]) => void;
  readonly onLiveness: (shardIndex: number, phase: string | null) => void;
} {
  return {
    residentLedger: rendererResidentLedger(),
    originalWorkStock: rendererOriginalWorkStock(),
    shardCount: poolConcurrency(),
    // 🎭️ A real DOM `Worker` satisfies `ShardWorkerLike` structurally at runtime (same claim
    // `🟦️.ts`'s own doc makes) — the cast only bridges `onmessage`/`onerror`'s
    // wider native `MessageEvent`/`ErrorEvent` handler types down to the interface's minimal
    // `{data: unknown}`/`unknown` shape, which a `MessageEvent`/`ErrorEvent` handler always satisfies.
    createWorker,
    // [TRACE] temporary watchdog widening — measures whether the setContributions turn is a HANG or a
    // long-but-finite install. REMOVE with the measurement (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    heartbeatTimeoutMs: 120_000,
    onActorTrap: (actorId, message) => undefined,
    onShardLost: handlePluginShardLost,
    // 🫀️ Every proof a shard is alive is also proof for the loads queued behind its module fetch —
    // see `notePluginLoadProgressForInFlightV1`.
    onLiveness: () => void notePluginLoadProgressForInFlightV1(),
  };
}

/** 🩺️ Typed boot fault for the ONE failure the shell retries rather than surfaces: the shard hosting
 * the primary plugin's first turn was taken down (watchdog kill under load, or a worker crash) and
 * `ShardClient.rebuild` already replaced it. Carries `code` so `windowFaultFromError` names the cause
 * on the error surface instead of leaking `shard 0 terminated` as the whole story. */
export const PLUGIN_BOOT_SHARD_LOST_FAULT = "plugin.boot.shard-lost";

export class PluginBootShardLostError extends Error {
  readonly code = PLUGIN_BOOT_SHARD_LOST_FAULT;
  constructor(pluginId: string, cause: unknown) {
    super(`${PLUGIN_BOOT_SHARD_LOST_FAULT}: primary program ${pluginId} lost its shard twice while booting (${cause instanceof Error ? cause.message : String(cause)})`);
  }
}

let sharedShardClient: ShardClient | null = null;
export function getShardClient(): ShardClient {
  if (sharedShardClient) return sharedShardClient;
  // 🧪️ Probed ONCE on the main thread before the first worker exists. Without JSPI every shard's
  // script throws at module top level, `worker.onerror` fires for all of them, and the boot degrades
  // into dozens of unrelated-looking `timeout loading <plugin>` lines with the real cause nowhere in
  // the log — observed verbatim on 2026-09-05 (four `shard N worker error Event` lines, no beacon).
  assertShardJspiAvailable();
  sharedShardClient = new ShardClient(buildShardClientOptions());
  // 🚑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): before this packet, neither
  // `checkHeartbeats` nor `pollHeartbeatSab` had a production caller anywhere in the repo (`ShardClient`'s
  // own doc on `startWatchdog` — "the watchdog's whole failure ladder was wired but nothing in
  // production ever turned the crank"), so a wedged shard went undetected forever in the real app.
  // Self-ticks at `startWatchdog`'s own default cadence (`heartbeatTimeoutMs`) — see that method's doc.
  sharedShardClient.startWatchdog();
  return sharedShardClient;
}

/** 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime) — metrics-publisher ownership
 * decision: `autoStartMetricsPublisher` is left at its default (`false`) here, DELIBERATELY. This
 * function is the constructor call site, yes, but it is not the DELIBERATE-choice construction site
 * `ActivationRegistryOptions.autoStartMetricsPublisher`'s own doc asks for — `getActivationRegistry` is
 * a lazy, shared, module-wide singleton created on the FIRST `loadPluginModule` call from anywhere (a
 * plugin boot, not a user action), with no relationship to whether anyone is actually watching
 * `metricsBus` right now. Turning it on here would start a real 2 Hz `setInterval` the moment the
 * first plugin loads, for the lifetime of the tab, regardless of whether the task-manager window
 * (`🧵️TaskManager/🟦️.tsx`'s own header doc: still registrar-only, unmounted work) — the
 * registry's own ONLY real subscriber — is even open. The deliberate choice belongs to whichever
 * construction site mounts that consumer (`🏛️ShellHost`, registrar-only per this packet's lease) —
 * turning this on here would silently change a shared default for every other current/future
 * consumer of this singleton, exactly what `autoStartMetricsPublisher` exists to prevent. */
let sharedActivationRegistry: ActivationRegistry | null = null;
export function getActivationRegistry(): ActivationRegistry {
  sharedActivationRegistry ??= new ActivationRegistry({ shardClient: getShardClient(), defaultBudget: DEFAULT_SHARD_BUDGET });
  return sharedActivationRegistry;
}

/** 🧵️ The actor runtime this tab's plugins run on, for `🧵️TaskManager` — `null` until the first plugin load
 * has created it. Never creates one itself: a task manager watching an empty runtime must not start the
 * shard worker pool just by being opened. */
export function pluginRuntimeActivationRegistryV1(): ActivationRegistry | null {
  return sharedActivationRegistry;
}

/** 🧩️ Indexes a loaded extension program under its loaded parent program, so activating the parent activates the
 * extension — for programs outside the build-time catalog (a hub document's catalog-resolved programs). */
export function registerProgramExtensionV1(parentPluginId: string, extensionPluginId: string): void {
  getActivationRegistry().registerExtension(parentPluginId, extensionPluginId);
}
//#endregion 🔖️ActorAdapter
//#endregion 🔖️plugin-runtime

//#region 🔖️DependencyOrderedBoot
/** 🧪️ The live state cells of the shared shard client and activation registry, as a getter/setter pair a test swaps. */
export function pluginRuntimeShardTestStateV1() {
  return {
    get sharedActivationRegistry() { return sharedActivationRegistry; },
    set sharedActivationRegistry(value: typeof sharedActivationRegistry) { sharedActivationRegistry = value; },
    get sharedShardClient() { return sharedShardClient; },
    set sharedShardClient(value: typeof sharedShardClient) { sharedShardClient = value; },
  };
}
//#endregion 🔖️DependencyOrderedBoot
