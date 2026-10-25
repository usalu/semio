/** 🧯️ The resident actor cap under memory pressure: the memory probe and the cap it yields.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region 🐚️ActivationRegistry
//#region 🧮️MemoryPressureCap
/** 🧮️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-activation): what {@link residentActorCapFromMemory}
 * reads — `navigator.deviceMemory` (Chromium-only, coarse GiB bucket) and/or
 * `performance.memory.jsHeapSizeLimit` (Chromium-only heap ceiling). Safari/Firefox report neither, in
 * which case `ActivationRegistryOptions.maxResidentActors`'s hardcoded fallback still applies. A plain
 * data record, not a live binding — {@link MemoryProbe} is what makes it injectable. */
export interface MemoryProbeReading {
  readonly deviceMemoryGiB?: number;
  readonly jsHeapSizeLimitBytes?: number;
}

/** 🧠️ Injectable seam — CLAUDE.md forbids this class depending on the ambient `navigator`/
 * `performance` globals directly (an external implementation detail); tests inject a fake reading
 * instead of touching a real browser. {@link defaultMemoryProbe} is the only production caller of
 * those globals. */
export type MemoryProbe = () => MemoryProbeReading;

/** 🚧️ The pre-existing hardcoded LRU cap (design-runtime.md §1 `FailurePolicy`) — now the FALLBACK for
 * when neither memory signal is available, not the only source. */
export const DEFAULT_MAX_RESIDENT_ACTORS = 24;

const MIN_MAX_RESIDENT_ACTORS = 4;
const MAX_MAX_RESIDENT_ACTORS = 96;
/** 💻️ Heuristic: one resident actor's worker-side wasm instance + checkpoint costs roughly this many
 * bytes of device-memory headroom — tuned so a ~4 GiB `deviceMemory` bucket (a typical mid-range
 * laptop) lands near {@link DEFAULT_MAX_RESIDENT_ACTORS}. */
const RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB = 6;
const BYTES_PER_RESIDENT_ACTOR = 64 * 1024 * 1024;

function clampResidentActors(value: number): number {
  return Math.min(MAX_MAX_RESIDENT_ACTORS, Math.max(MIN_MAX_RESIDENT_ACTORS, Math.round(value)));
}

/** 📱️ `navigator.deviceMemory` first (coarser but a direct GiB figure), else
 * `performance.memory.jsHeapSizeLimit`, else {@link DEFAULT_MAX_RESIDENT_ACTORS} unchanged. Both casts
 * are needed because neither field is in the standard DOM lib — `deviceMemory` is the Chromium-only
 * Device Memory API, `performance.memory` a Chromium-only non-standard extension. */
export function defaultMemoryProbe(): MemoryProbeReading {
  const nav = globalThis.navigator as (Navigator & { readonly deviceMemory?: number }) | undefined;
  const perf = globalThis.performance as (Performance & { readonly memory?: { readonly jsHeapSizeLimit?: number } }) | undefined;
  return { deviceMemoryGiB: nav?.deviceMemory, jsHeapSizeLimitBytes: perf?.memory?.jsHeapSizeLimit };
}

/** 💎️ Pure — same reasoning as `runtimeMetricsDue` below for being its own exported function rather
 * than inlined into the constructor: testable without touching a real `navigator`/`performance`. */
export function residentActorCapFromMemory(reading: MemoryProbeReading, fallback: number = DEFAULT_MAX_RESIDENT_ACTORS): number {
  if (typeof reading.deviceMemoryGiB === "number" && reading.deviceMemoryGiB > 0) return clampResidentActors(reading.deviceMemoryGiB * RESIDENT_ACTORS_PER_DEVICE_MEMORY_GIB);
  if (typeof reading.jsHeapSizeLimitBytes === "number" && reading.jsHeapSizeLimitBytes > 0) return clampResidentActors(reading.jsHeapSizeLimitBytes / BYTES_PER_RESIDENT_ACTOR);
  return fallback;
}
//#endregion 🧮️MemoryPressureCap
//#endregion 🐚️ActivationRegistry
// #endregion 🎠️Kernel
