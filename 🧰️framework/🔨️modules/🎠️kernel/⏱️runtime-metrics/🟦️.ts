/** ⏱️ Runtime metrics: the per-actor rows, the snapshot and the publish interval rule.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { ShardClient } from "@semio-tech/framework-actor/shard-client";

// #region 🎠️Kernel
//#region 🐚️ActivationRegistry
/** 📈️ Mirrors `semio_framework_actor::ActorMetricsSample` where this registry has the data for it —
 * see `ActivationRegistry.runtimeMetricsActorRows`'s own doc comment for exactly which fields are an
 * honest gap here (no live `Kernel` on this side of the boundary). */
export interface RuntimeMetricsActorRow {
  readonly actorId: string;
  readonly pluginId: string;
  readonly resident: boolean;
  readonly shard: number | null;
}

/** 📈️ Mirrors `semio_framework_actor::RuntimeMetricsSnapshot`'s shape — `kernel` (the aggregate
 * `KernelMetrics`) is omitted here since this registry never holds a `Kernel` to sample it from.
 * `shards` is spelled as `ShardClient["shardMetricsSamples"]`'s own return type rather than importing
 * `ShardMetricsSample` by name — this file's `path_scope` is the `🐚️ActivationRegistry` region only,
 * and the top-of-file import list sits outside it. */
export interface RuntimeMetricsSnapshot {
  readonly actors: readonly RuntimeMetricsActorRow[];
  readonly shards: ReturnType<ShardClient["shardMetricsSamples"]>;
  readonly sampledAtMs: number;
}

/** ⏱️ 2Hz, matching `semio_framework_actor::RUNTIME_METRICS_PUBLISH_INTERVAL_MS`. */
export const RUNTIME_METRICS_PUBLISH_INTERVAL_MS = 500;

/** ⏱️ Pure cadence gate — the exact TS mirror of `semio_framework_actor::runtime_metrics_due`, kept
 * as its own exported function (not inlined into `startRuntimeMetricsPublisher`) so it is testable
 * without fake timers, same reasoning as the Rust side's own doc comment. */
export function runtimeMetricsDue(lastPublishedMs: number | null, nowMs: number): boolean {
  if (lastPublishedMs === null) return true;
  return nowMs - lastPublishedMs >= RUNTIME_METRICS_PUBLISH_INTERVAL_MS;
}
//#endregion 🐚️ActivationRegistry
// #endregion 🎠️Kernel
