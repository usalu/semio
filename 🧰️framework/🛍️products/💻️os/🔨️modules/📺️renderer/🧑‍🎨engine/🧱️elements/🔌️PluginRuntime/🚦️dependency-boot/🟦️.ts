/** 🚦️ Dependency-ordered plugin boot: dependency levels, the bounded worker pool and `loadPluginModulesInDependencyOrder`.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { type PluginGraphError, type PluginRegistryEntry, orderPluginRegistryEntries } from "@semio-tech/framework";
import { type PluginWasmHandle } from "../🧷️handle-contract/🟦️.ts";
import { poolConcurrency } from "../🛰️runtime-singletons/🟦️.ts";
import { loadPluginModule } from "../🧩️module-loader/🟦️.ts";

//#region 🔖️DependencyOrderedBoot
/** 🎯️ One plugin `loadPluginModule` itself rejected on (network/activation failure) — kept SEPARATE
 * from {@link PluginGraphError} (a static graph fault: missing dependency, version mismatch, cycle)
 * since the two are different failure classes with different callers-facing meaning; conflating a
 * runtime load failure into a `PluginGraphError` code would mislead `pluginGraphErrorMessage`
 * (`@semio-tech/framework`) into describing a graph problem that never happened. */
export interface PluginLoadFailure {
  readonly pluginId: string;
  readonly error: unknown;
}

/** 🧮️ Groups `order`'s already-topologically-sorted entries into dependency LEVELS: level 0 has no
 * dependency inside `order` at all, level N+1's members depend on at least one level-N member (and on
 * nothing deeper). Every entry in the SAME level is independent of every other entry in that level —
 * that's exactly the "siblings may run in parallel" property {@link loadPluginModulesInDependencyOrder}
 * needs. Walking `order` in its own sequence (rather than re-deriving a topological order here) is
 * sufficient: Kahn's-algorithm output (`orderPluginRegistryEntries`'s own implementation) guarantees
 * every dependency of an entry appears strictly before it, so `levelOf` is always populated for a
 * dependency by the time its dependent is visited. A dependency id that ISN'T in `order` (already
 * dropped as blocked by a graph fault, or simply not part of this call's own entry set) contributes no
 * edge — same fail-soft posture as `orderPluginRegistryEntries` itself. */
export function computeDependencyLevels(order: readonly PluginRegistryEntry[]): readonly (readonly PluginRegistryEntry[])[] {
  const levelOf = new Map<string, number>();
  const levels: PluginRegistryEntry[][] = [];
  for (const entry of order) {
    let level = 0;
    for (const dependency of entry.dependencies ?? []) {
      const dependencyLevel = levelOf.get(dependency.pluginId);
      if (dependencyLevel !== undefined) level = Math.max(level, dependencyLevel + 1);
    }
    levelOf.set(entry.pluginId, level);
    (levels[level] ??= []).push(entry);
  }
  return levels;
}

/** 🧵️ Runs `run` over every item in `items`, at most `limit` concurrently — a plain worker-pool loop
 * (each of `limit` workers pulls the next unclaimed index until the list is exhausted), not a batched
 * `Promise.all` chunking, so a fast item's slot is reused immediately rather than waiting for its whole
 * batch to finish. */
export async function runBounded<T>(items: readonly T[], limit: number, run: (item: T) => Promise<void>): Promise<void> {
  let cursor = 0;
  const workerCount = Math.max(1, Math.min(limit, items.length));
  await Promise.all(
    Array.from({ length: workerCount }, async () => {
      while (cursor < items.length) {
        const item = items[cursor]!;
        cursor += 1;
        await run(item);
      }
    }),
  );
}

/**
 * 🎯️ Loads several plugin modules in dependency order (scout-2 §4: "boot must walk the dependency
 * order from `PluginGraph` instead of relying on array order") — a dependency's WHOLE level finishes
 * before any of its dependents starts, but independent siblings within one level load CONCURRENTLY,
 * bounded to `options.concurrency` (default {@link poolConcurrency}, the exact `min(hardwareConcurrency
 * -1, 4)` bound `getShardClient`'s own worker pool uses).
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): before this packet, this
 * function was a strict serial `for` loop — dependencies WERE respected, but so were completely
 * independent siblings (~20 plugins boot strictly one-after-another on the real app's real cold boot,
 * `📌️important.md`'s own "Flaky OS Dev Preview" note). The concurrency bound is deliberately the SAME
 * number as the shard pool's own worker count: `activate()`'s real cost (worker-side wasm
 * instantiation) is bounded by how many shard workers exist to run it, not by how many `activate()`
 * calls are in flight — requesting more concurrent activations than there are shards to service them
 * would only add memory pressure (`ActivationRegistry.evictForMemoryPressure`'s LRU thrashing) for zero
 * extra real parallelism, so `poolConcurrency()` (not, say, `entries.length`) is the bound that
 * actually reflects the hardware.
 *
 * Two independent failure classes, BOTH fail-soft (never abort the whole boot): a static graph fault
 * (missing dependency/version mismatch/cycle, `errors`, `orderPluginRegistryEntries`'s own existing
 * posture) drops the entry before it is ever attempted; a RUNTIME `loadPluginModule` rejection
 * (`loadFailures`, new in this packet) drops that entry AND cascades to skip every not-yet-attempted
 * descendant that (transitively) depends on it — a dependent can't sensibly boot on a dependency that
 * never loaded. `options.signal`, when aborted, stops STARTING new loads (in-flight ones still settle
 * naturally) — forwarded to `loadPluginModule`'s own descriptor-fetch abort for whichever entries
 * haven't started yet. `🐙️handles` stays in the same topological sequence `order` always had, regardless
 * of the level-parallel loading order underneath, for caller stability. */
export async function loadPluginModulesInDependencyOrder(
  entries: readonly PluginRegistryEntry[],
  options?: {
    readonly loadModule?: (pluginId: string, moduleUrl: string, signal?: AbortSignal) => Promise<PluginWasmHandle>;
    readonly concurrency?: number;
    readonly signal?: AbortSignal;
  },
): Promise<{ readonly handles: readonly PluginWasmHandle[]; readonly errors: readonly PluginGraphError[]; readonly loadFailures: readonly PluginLoadFailure[] }> {
  const loadModule = options?.loadModule ?? loadPluginModule;
  const limit = options?.concurrency ?? poolConcurrency();
  const signal = options?.signal;
  const { order, errors } = orderPluginRegistryEntries(entries);
  const levels = computeDependencyLevels(order);
  const handleByPluginId = new Map<string, PluginWasmHandle>();
  const failedPluginIds = new Set<string>();
  const loadFailures: PluginLoadFailure[] = [];

  for (const level of levels) {
    const loadable = level.filter((entry) => {
      const blockedDependency = (entry.dependencies ?? []).find((dependency) => failedPluginIds.has(dependency.pluginId));
      if (!blockedDependency) return true;
      failedPluginIds.add(entry.pluginId);
      loadFailures.push({ pluginId: entry.pluginId, error: new Error(`loadPluginModulesInDependencyOrder: ${entry.pluginId} skipped — dependency ${blockedDependency.pluginId} failed to load`) });
      return false;
    });
    await runBounded(loadable, limit, async (entry) => {
      if (signal?.aborted) {
        failedPluginIds.add(entry.pluginId);
        loadFailures.push({ pluginId: entry.pluginId, error: new Error(`loadPluginModulesInDependencyOrder: ${entry.pluginId} skipped — boot aborted`) });
        return;
      }
      try {
        handleByPluginId.set(entry.pluginId, await loadModule(entry.pluginId, entry.moduleUrl, signal));
      } catch (error) {
        failedPluginIds.add(entry.pluginId);
        loadFailures.push({ pluginId: entry.pluginId, error });
      }
    });
  }

  const handles = order.map((entry) => handleByPluginId.get(entry.pluginId)).filter((handle): handle is PluginWasmHandle => handle !== undefined);
  return { handles, errors, loadFailures };
}
//#endregion 🔖️DependencyOrderedBoot
