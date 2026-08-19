// #region 🧲️Header
/** @emoji 🧵️ Renderer-agnostic bootstrap for the pooled shard-worker runtime (design-runtime.md
 * §1/§3) — `ShardClient` (bounded worker pool, `actorId`-multiplexed) + the pool-sizing/default-budget
 * constants every caller of it needs, factored out of `PluginRuntime/🟦️component.tsx`'s own
 * `getShardClient`/`poolConcurrency`/`buildShardClientOptions` (`🔖️ActorAdapter` region) so a SECOND
 * renderer target (MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, `wgpu-web-shard`) does not hand-roll a
 * third copy of this wiring — exactly the "shared logic must be shared, not duplicated" rule that
 * packet's own brief states. `PluginRuntime` predates this module and still carries its own inline
 * copy of the same constants/logic (outside `wgpu-web-shard`'s lease to edit); a future packet should
 * point it here too, at which point `SHARD_WORKER_URL`/`DEFAULT_SHARD_BUDGET`/`poolConcurrency` become
 * the single source of truth for both renderers instead of two hand-synced literals. */
// #endregion 🧲️Header

// #region 🔌️Imports
import { ShardClient, type ShardBudget, type ShardClientOptions, type ShardWorkerLike } from "./🧵️shard-client.ts";
// #endregion 🔌️Imports

//#region 🔖️Defaults
/** 🌐️ The one shard-worker bundle every pooled-actor consumer boots against — `🌐plugin-web-materialize.ts`
 * is the generator, `dev/vite.config.ts`'s `pluginModuleDirNames` is what copies it into a production
 * build's `dist/plugin-modules/_shard/` output. */
export const SHARD_WORKER_URL = "/plugin-modules/_shard/🟨️shard-worker.js";

/** ⛽️ Provisional constant turn budget — same honestly-flagged gap `ProgramBridge/🧊️component.rs`'s
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
 * same split `PluginRuntime/🟦️component.tsx`'s own `buildShardClientOptions` doc explains: this
 * function itself is untestable in isolation since it hardcodes a real DOM `Worker`, which a `jsdom`
 * suite doesn't provide). */
export function buildShardClientOptions(
  overrides: Partial<ShardClientOptions> & { readonly onShardLost: ShardClientOptions["onShardLost"]; readonly onActorTrap?: ShardClientOptions["onActorTrap"] },
): ShardClientOptions {
  return {
    shardCount: poolConcurrency(),
    createWorker: () => new Worker(SHARD_WORKER_URL, { type: "module" }) as unknown as ShardWorkerLike,
    ...overrides,
  };
}
//#endregion 🔖️Options

//#region 🔖️PooledActorRuntime
export interface PooledActorRuntime {
  readonly shardClient: ShardClient;
}

/** 🐚️ Constructs ONE `ShardClient` and starts its watchdog — the mechanical worker-pool half of what
 * `PluginRuntime/🟦️component.tsx`'s `getShardClient` does (the `ActivationRegistry` half is left to
 * the caller, since its `defaultBudget`/eviction knobs are legitimately per-consumer). A caller keeps
 * ITS OWN module-level singleton holding the returned value — this factory does not memoize, so two
 * calls make two independent pools; that choice belongs to the renderer target, not this module (a
 * single JS realm hosting more than one renderer target is out of scope here, same limitation
 * `PluginRuntime`'s own `currentPluginRuntimeActor` doc already flags for actor identity). */
export function createPooledActorRuntime(options: { readonly onActorTrap?: ShardClientOptions["onActorTrap"]; readonly onShardLost: ShardClientOptions["onShardLost"]; readonly createWorker?: ShardClientOptions["createWorker"] }): PooledActorRuntime {
  const shardClient = new ShardClient(buildShardClientOptions(options));
  // 🚑️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): before that packet, neither
  // `checkHeartbeats` nor `pollHeartbeatSab` had a production caller anywhere in the repo — a wedged
  // shard went undetected forever. Self-ticks at `startWatchdog`'s own default cadence.
  shardClient.startWatchdog();
  return { shardClient };
}
//#endregion 🔖️PooledActorRuntime
