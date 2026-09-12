// #region 🧲️Header
/** @emoji 🧵️ Renderer-agnostic bootstrap for the pooled shard-worker runtime (design-runtime.md
 * §1/§3) — `ShardClient` (bounded worker pool, `actorId`-multiplexed) + the pool-sizing/default-budget
 * constants every caller of it needs, factored out of `PluginRuntime/🟦️.tsx`'s own
 * `getShardClient`/`poolConcurrency`/`buildShardClientOptions` (`🔖️ActorAdapter` region) so a SECOND
 * renderer target (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, `wgpu-web-shard`) does not hand-roll a
 * third copy of this wiring — exactly the "shared logic must be shared, not duplicated" rule that
 * packet's own brief states. `PluginRuntime` predates this module and still carries its own inline
 * copy of the same constants/logic (outside `wgpu-web-shard`'s lease to edit); a future packet should
 * point it here too, at which point `SHARD_WORKER_URL`/`DEFAULT_SHARD_BUDGET`/`poolConcurrency` become
 * the single source of truth for both renderers instead of two hand-synced literals. */
// #endregion 🧲️Header

// #region 🔌️Imports
import { ShardClient, type ShardBudget, type ShardClientOptions, type ShardWorkerLike } from "../📮️shard-client/🟦️.ts";
// #endregion 🔌️Imports

//#region 🔖️Defaults
/** 🌐️ The one shard-worker bundle every pooled-actor consumer boots against — `../🧵️shard-runtime/🟦️.ts`
 * is the generator, `dev/vite.config.ts`'s `pluginModuleDirNames` is what copies it into a production
 * build's `dist/🔌️plugin-modules/🧵️shard/` output. */
export const SHARD_WORKER_URL = "/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js";

/** 🩺️ Mirrors the guest-side `RUNTIME_DIAGNOSTICS_ENV` (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs`) and
 * the renderer's own `RUNTIME_DIAGNOSTICS_KEY`/`TURN_DIAGNOSTICS_KEY` — one name, now three
 * languages. Declared here rather than imported because this module is a framework leaf and both
 * other copies live in product code; the engine-contract suite holds all of them equal. */
export const SHARD_RUNTIME_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";

/** 🩺️ The query parameter {@link shardWorkerUrl} stamps on the worker URL when diagnostics are
 * armed. A Worker realm owns no `localStorage`, so the switch the PAGE resolved has to cross the
 * worker boundary somehow, and the worker's own URL is the only channel that exists before its first
 * message. The worker reads it back and hands it to every component it hosts through
 * `wasi:cli/environment` — see `shardWorkerSource`'s `armGuestRuntimeDiagnostics`. */
export const SHARD_WORKER_DIAGNOSTICS_PARAM = "diagnostics";

/** 🩺️ Whether this realm has armed runtime diagnostics: the stored
 * {@link SHARD_RUNTIME_DIAGNOSTICS_KEY} preference, read defensively because a Worker (and a
 * storage-blocked browser) throws on the accessor itself rather than returning null. */
function shardRuntimeDiagnosticsArmed(): boolean {
  try {
    const stored = globalThis.localStorage?.getItem(SHARD_RUNTIME_DIAGNOSTICS_KEY)?.trim().toLowerCase();
    return stored === "1" || stored === "true" || stored === "on" || stored === "yes";
  } catch {
    return false;
  }
}

/** 🩺️ {@link SHARD_WORKER_URL}, plus the diagnostics stamp when this page armed them. Every
 * `new Worker(...)` in the repo goes through this so the guest's own `[DEBUG]` trace sites are
 * reachable from a browser session without a second build. */
export function shardWorkerUrl(): string {
  return shardRuntimeDiagnosticsArmed() ? `${SHARD_WORKER_URL}?${SHARD_WORKER_DIAGNOSTICS_PARAM}=1` : SHARD_WORKER_URL;
}

/** ⛽️ Provisional constant turn budget — same honestly-flagged gap `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s
 * native `TURN_BUDGET` documents ("until the DRR scheduler threads a real per-lane one through");
 * this is that budget's shared web default. A caller with a genuinely different budget need (a batch
 * job actor, say) still passes its own `ShardBudget` to `ActivationRegistry`/turn calls directly —
 * this constant is only the sane default for "a plugin instance doing normal interactive work". */
export const DEFAULT_SHARD_BUDGET: ShardBudget = { fuel: 50_000_000, wallMs: 100, memoryBytes: 256 * 1024 * 1024, uiNodes: 20_000, mailboxLen: 64, maxEffects: 64, maxPatchBytes: 1 << 20 };

/** 🧮️ `min(hardwareConcurrency-1, 4)` (design-runtime.md §1 `ShardTable`) — "how many wasm-boundary
 * hops can genuinely run at once on this device". Falls back to `5` (so the clamp lands on `4`) when
 * `navigator.hardwareConcurrency` is unavailable (SSR/test). */
export function poolConcurrency(): number {
  const hardwareConcurrency = typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" ? navigator.hardwareConcurrency : 5;
  return Math.max(1, Math.min(hardwareConcurrency - 1, 4));
}
//#endregion 🔖️Defaults

//#region 🔖️Options
/** 🎭️ Builds the `ShardClientOptions` every consumer needs, pool-sizing/worker-construction defaulted
 * — `createWorker` defaults to a real DOM `Worker` against {@link SHARD_WORKER_URL}; a test overrides
 * it to exercise `ShardClient`'s real lane/heartbeat/dispose machinery against a fake transport (the
 * same split `PluginRuntime/🟦️.tsx`'s own `buildShardClientOptions` doc explains: this
 * function itself is untestable in isolation since it hardcodes a real DOM `Worker`, which a `jsdom`
 * suite doesn't provide). */
export function buildShardClientOptions(
  overrides: Partial<ShardClientOptions> & { readonly residentLedger: ShardClientOptions["residentLedger"]; readonly onShardLost: ShardClientOptions["onShardLost"]; readonly onActorTrap?: ShardClientOptions["onActorTrap"] },
): ShardClientOptions {
  return {
    shardCount: poolConcurrency(),
    createWorker: () => new Worker(shardWorkerUrl(), { type: "module" }) as unknown as ShardWorkerLike,
    ...overrides,
  };
}
//#endregion 🔖️Options

//#region 🔖️PooledActorRuntime
export interface PooledActorRuntime {
  readonly shardClient: ShardClient;
}

/** 🐚️ Constructs ONE `ShardClient` and starts its watchdog — the mechanical worker-pool half of what
 * `PluginRuntime/🟦️.tsx`'s `getShardClient` does (the `ActivationRegistry` half is left to
 * the caller, since its `defaultBudget`/eviction knobs are legitimately per-consumer). A caller keeps
 * ITS OWN module-level singleton holding the returned value — this factory does not memoize, so two
 * calls make two independent pools; that choice belongs to the renderer target, not this module (a
 * single JS realm hosting more than one renderer target is out of scope here, same limitation
 * `PluginRuntime`'s own `currentPluginRuntimeActor` doc already flags for actor identity). */
export function createPooledActorRuntime(options: {
  readonly residentLedger: ShardClientOptions["residentLedger"];
  readonly onActorTrap?: ShardClientOptions["onActorTrap"];
  readonly onShardLost: ShardClientOptions["onShardLost"];
  readonly createWorker?: ShardClientOptions["createWorker"];
  /** 🌉️ terra-shard-effect-bridge: passed straight through to {@link ShardClient} — see that class's
   * own `onHostEffect`/`maxOutstandingEffectsPerActor` doc. Omitted here means every host-import
   * effect-request fails fast with "no host effect handler installed", same as omitting it directly. */
  readonly onHostEffect?: ShardClientOptions["onHostEffect"];
  readonly maxOutstandingEffectsPerActor?: ShardClientOptions["maxOutstandingEffectsPerActor"];
}): PooledActorRuntime {
  const shardClient = new ShardClient(buildShardClientOptions(options));
  // 🚑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): before that packet, neither
  // `checkHeartbeats` nor `pollHeartbeatSab` had a production caller anywhere in the repo — a wedged
  // shard went undetected forever. Self-ticks at `startWatchdog`'s own default cadence.
  shardClient.startWatchdog();
  return { shardClient };
}
//#endregion 🔖️PooledActorRuntime
