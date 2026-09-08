/** @emoji 📦️ `@semio-tech/framework` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/🎯️action-bus/🟦️.ts";
export { blake3Hex, Blake3Hasher } from "../../🔨️modules/🔏️hash/🟦️.ts";
export * from "../../🔨️modules/🧩️action-argument-resolution/🟦️.ts";
export * from "../../🔨️modules/🧬️schema/🟦️.ts";
export * from "../../🔨️modules/🖥️platform/🟦️.ts";
export * from "../../🔨️modules/🔺️mesh/🟦️.ts";
export * from "../../🔨️modules/🛂️manifest/🟦️.ts";
// 🕹️wave-2b: named (not `export *`) — the 🕹️interaction module's own `InteractionDefinition`/`MergeMode`/…
// family is already re-exported above via `🛂️manifest` (owned-schema-generated mirror of the same Rust types),
// so a second blanket export of the module root would collide; only its presence-broadcast leaf types,
// which nothing else exports yet, are pulled in here for `@semio-tech/framework` consumers like the OS Shell.
export type { PresenceDomain, PresenceInteraction } from "../../🔨️modules/🕹️interaction/🧬️schema/🟦️.ts";
export * from "../../🔨️modules/🎠️kernel/🟦️.ts";
export * from "../../🔨️modules/🔄️machine/🟦️.ts";
export { NumericIndex, NumericIndexEdit, NumericIndexReader, NumericIndexRetirement, type NumericIndexGrant, type NumericIndexStep, type NumericIndexReadStep, type NumericIndexOrdinal } from "../../🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
export { RetainedUiPatchCursor, RetainedUiSnapshotCursor, RetainedUiSurfaceOwner, RetainedUiTransaction, type RetainedUiState, type RetainedUiStep, type RetainedUiResult, type RetainedUiRejection, type RetainedUiSurfaceIdentity, type RetainedUiAcknowledgement } from "../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️.ts";

import {
  organizeContextMenu,
  type ContextMenuItemSpec,
} from "../../🔨️modules/🔺️mesh/🟦️.ts";
import {
  createMemoryStoragePort,
  DockLayoutStore,
  DockUiStateStore,
  NamedLayoutStore,
  OsShellConfig,
  WindowPaneStateStore,
  type DockSkeleton,
  type DockUiState,
  type WindowPaneUiState,
} from "../../🔨️modules/🖥️platform/🟦️.ts";
import {
  createDevPluginSource,
  createExtensionSource,
  extensionSourceEventToPluginSourceEvent,
  multiplexPluginSources,
  resolvePlaygroundBoot,
  resolvePluginHostConfig,
  resolvePluginRegistryId,
  ephemeralBox,
  OsTransient,
  type EphemeralBox,
  type PluginCatalog,
  type PluginRegistryEntry,
  type PluginSourceEvent,
} from "../../🔨️modules/🎠️kernel/🟦️.ts";
import { effectiveActionArgs, missingRequiredArgs } from "../../🔨️modules/🧩️action-argument-resolution/🟦️.ts";
import { type ActionArgDef, type ArgSchema } from "../../🔨️modules/🛂️manifest/🟦️.ts";
import {
  ActionId,
  ActorId,
  ActorSystem,
  BitSet,
  checkInvariants,
  EventId,
  explore,
  GuardId,
  init,
  InvokeId,
  macrostep,
  Model,
  NodeId,
  NullInspector,
  persist,
  restore,
  runConformance,
  start,
  step,
  TestHost,
  TimerId,
  timerElapsed,
  TraceInspector,
  type Command,
  type ConformanceStep,
  type Invariant,
  type Machine,
  type MachineSpec,
  type Migration,
  type NodeDef,
  type PersistedSnapshot,
  type StatechartEvent,
  type TransitionDef,
} from "../../🔨️modules/🔄️machine/🟦️.ts";

//#region 🪶️LeasePool
// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): relocated unchanged from
// `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` — 11 total consumers, of which only 4 were plugin-
// specific (`📓️luna-consumers-audit.md`'s recon); the plugin-specific ones (`PluginModuleLease`/
// `acquirePluginModule`) are deleted along with `PluginWorkerClient`, replaced by `ActivationRegistry`,
// but this generic pool has real non-plugin callers (the renderer's engine-session cache and others)
// that must keep working — so it moves here instead of disappearing with its plugin-specific caller.

/** @emoji 🪶️ One caller's reference to a {@link LeasePool}-managed resource. `release()` is idempotent —
 * a second call is a no-op — and drops this caller's refcount; the pool only disposes the underlying
 * resource once every issued lease on that key has released (and, unless `lingerMs` is 0, only after
 * the linger window below elapses with no re-acquire). */
export interface Lease<T> {
  readonly value: T;
  release(): void;
}

export interface LeasePoolStats {
  readonly key: string;
  readonly refs: number;
  readonly state: "loading" | "resident" | "lingering";
}

export interface LeasePool<T> {
  acquire(key: string): Promise<Lease<T>>;
  /** Forces disposal of `key` (or every entry when omitted) right now, bypassing any linger timer.
   * A no-op (logged, not thrown) for a key with active leases — evicting a resource a caller still
   * holds would leave that caller's `Lease.value` silently dead underneath it. */
  evictNow(key?: string): void;
  stats(): readonly LeasePoolStats[];
}

type LeasePoolEntry<T> = {
  readonly promise: Promise<T>;
  refs: number;
  lingerTimer: ReturnType<typeof setTimeout> | null;
  settled: T | undefined;
};

/**
 * @emoji 🪶️ Generic refcounted resource pool with linger-based eviction — the shared mechanism the
 * renderer's engine-session cache and other non-plugin callers build on top of, instead of each
 * hand-rolling its own refcounting. A resource loads once per `key` and is shared by every caller;
 * when the last lease on a key releases, the resource isn't disposed immediately — it lingers for
 * `lingerMs` (default 30s) so a caller that re-acquires the same key shortly after (e.g. reopening a
 * just-closed window) reuses the still-live resource instead of paying full reload cost. `lingerMs: 0`
 * disposes the instant refs hit zero.
 */
export function createLeasePool<T>(load: (key: string) => Promise<T>, dispose: (value: T) => void, options?: { readonly lingerMs?: number; readonly label?: string }): LeasePool<T> {
  const lingerMs = options?.lingerMs ?? 30_000;
  const label = options?.label ?? "resource";
  const entries = new Map<string, LeasePoolEntry<T>>();

  function disposeEntry(key: string, entry: LeasePoolEntry<T>): void {
    if (entries.get(key) !== entry) return;
    entries.delete(key);
    if (entry.settled !== undefined) {
      console.log(`[DEBUG] ${label} evicted ${key}`);
      dispose(entry.settled);
    }
  }

  return {
    async acquire(key: string): Promise<Lease<T>> {
      let entry = entries.get(key);
      if (!entry) {
        const created: LeasePoolEntry<T> = { promise: load(key), refs: 0, lingerTimer: null, settled: undefined };
        created.promise.then(
          (value) => {
            created.settled = value;
          },
          () => {
            if (entries.get(key) === created) entries.delete(key);
          },
        );
        entries.set(key, created);
        entry = created;
      }
      const active = entry;
      if (active.lingerTimer !== null) {
        clearTimeout(active.lingerTimer);
        active.lingerTimer = null;
      }
      active.refs += 1;
      try {
        const value = await active.promise;
        let released = false;
        return {
          value,
          release: () => {
            if (released) return;
            released = true;
            active.refs -= 1;
            if (active.refs > 0) return;
            if (lingerMs <= 0) {
              disposeEntry(key, active);
              return;
            }
            active.lingerTimer = setTimeout(() => disposeEntry(key, active), lingerMs);
          },
        };
      } catch (error) {
        active.refs -= 1;
        throw error;
      }
    },
    evictNow(key?: string): void {
      for (const [entryKey, entry] of key ? ([[key, entries.get(key)]] as const) : entries) {
        if (!entry) continue;
        if (entry.refs > 0) {
          console.warn(`[DEBUG] ${label} evictNow(${entryKey}) skipped — ${entry.refs} active lease(s)`);
          continue;
        }
        if (entry.lingerTimer !== null) clearTimeout(entry.lingerTimer);
        disposeEntry(entryKey, entry);
      }
    },
    stats(): readonly LeasePoolStats[] {
      return Array.from(entries.entries()).map(([key, entry]) => ({
        key,
        refs: entry.refs,
        state: entry.settled === undefined ? "loading" : entry.lingerTimer !== null ? "lingering" : "resident",
      }));
    },
  };
}
//#endregion 🪶️LeasePool

//#region 🔁️RetryWithJitteredBackoff
/** @emoji 🔁️ Options for {@link retryWithJitteredBackoff}. */
export interface JitteredBackoffOptions {
  readonly minMs: number;
  readonly maxMs: number;
  readonly signal?: AbortSignal;
}

function abortReason(signal: AbortSignal): unknown {
  return signal.reason ?? new Error("retryWithJitteredBackoff: aborted");
}

function abortableDelay(ms: number, signal: AbortSignal | undefined): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    if (signal?.aborted) {
      reject(abortReason(signal));
      return;
    }
    const timer = setTimeout(() => {
      cleanup();
      resolve();
    }, ms);
    function cleanup(): void {
      clearTimeout(timer);
      signal?.removeEventListener("abort", onAbort);
    }
    function onAbort(): void {
      cleanup();
      reject(abortReason(signal!));
    }
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}

/**
 * @emoji 🔁️ Retries `fn` with full-jitter exponential backoff (delay is a random value drawn from
 * `[minMs, min(maxMs, minMs * 2^attempt)]`, not a fixed exponential curve — this is what stops many
 * reconnecting clients from ever synchronizing into a hammering herd) until it resolves, `signal`
 * aborts, or `signal` is already aborted. Never returns while looping silently: an abort always
 * settles the returned promise, surfacing the abort's reason rather than hanging forever. For short
 * connection shortages only — the app must not freeze while this retries, and must not hammer the
 * remote end.
 * Wiederholt `fn` mit „full jitter“-Backoff, bis es erfüllt wird oder `signal` abbricht; ein Abbruch
 * löst die zurückgegebene Promise immer auf, statt endlos zu warten.
 */
export async function retryWithJitteredBackoff<T>(fn: () => Promise<T>, options: JitteredBackoffOptions): Promise<T> {
  const { minMs, maxMs, signal } = options;
  let attempt = 0;
  for (;;) {
    if (signal?.aborted) throw abortReason(signal);
    try {
      return await fn();
    } catch (error) {
      if (signal?.aborted) throw abortReason(signal);
      attempt += 1;
      const cap = Math.min(maxMs, minMs * 2 ** attempt);
      const waitMs = cap <= minMs ? minMs : minMs + Math.random() * (cap - minMs);
      try {
        await abortableDelay(waitMs, signal);
      } catch {
        throw signal?.aborted ? abortReason(signal) : error;
      }
    }
  }
}
//#endregion 🔁️RetryWithJitteredBackoff

//#region 🥇️LatestWins
/**
 * @emoji 🥇️ Single-flight with a trailing coalescer: the first call launches `run()` and hands the
 * caller that exact run's promise. Every subsequent call that arrives while a run is still in
 * flight collapses into at most one queued follow-up run — they all share that one follow-up's
 * promise, so N concurrent callers never produce more than one extra `run()` call, and every one of
 * them observes the latest result. Used for folder revalidation, presence, and refresh, where firing
 * a fresh request per caller would be wasteful and stale-by-the-time-it-lands anyway.
 * Einzelflug mit nachlaufender Zusammenführung: gleichzeitige Aufrufe während eines laufenden `run()`
 * teilen sich höchstens einen einzigen Folgeauf-ruf und dessen Ergebnis.
 */
export function latestWins<T>(run: () => Promise<T>): () => Promise<T> {
  let current: Promise<T> | null = null;
  let queued: Promise<T> | null = null;

  function launch(): Promise<T> {
    let promise: Promise<T>;
    try {
      promise = Promise.resolve(run());
    } catch (error) {
      promise = Promise.reject(error);
    }
    current = promise;
    const advance = (): void => {
      if (current === promise) current = null;
      if (queued !== null) queued = null;
    };
    promise.then(advance, advance);
    return promise;
  }

  return function trigger(): Promise<T> {
    if (current === null) return launch();
    if (queued === null) queued = current.then(launch, launch);
    return queued;
  };
}
//#endregion 🥇️LatestWins

//#region ⏱️FetchWithTimeout
/** @emoji 📨️ Structural view of a fetch response — declared locally so this module's public API
 * never requires the ambient `Response` type from outside this codebase. */
export interface FetchTimeoutResponse {
  readonly ok: boolean;
  readonly status: number;
  readonly statusText: string;
  readonly headers: { get(name: string): string | null };
  json(): Promise<unknown>;
  text(): Promise<string>;
}

/** @emoji ⏱️ Options for {@link fetchWithTimeout}. */
export interface FetchTimeoutOptions {
  readonly timeoutMs: number;
  readonly signal?: AbortSignal;
}

/**
 * @emoji ⏱️ `fetch` composed with a timeout: the request aborts if it hasn't settled within
 * `timeoutMs`, and separately aborts if the caller-supplied `signal` aborts — either can cancel it,
 * neither leaks its timer/listener past this call (both are cleaned up on every exit path: success,
 * timeout, external abort, and thrown `fetch` error alike).
 * `fetch` mit Timeout: sowohl das Zeitlimit als auch das übergebene `signal` können den Aufruf
 * abbrechen; Timer und Listener werden in jedem Fall aufgeräumt.
 */
export async function fetchWithTimeout(url: string, init: RequestInit | undefined, options: FetchTimeoutOptions): Promise<FetchTimeoutResponse> {
  const { timeoutMs, signal: externalSignal } = options;
  if (externalSignal?.aborted) throw externalSignal.reason ?? new Error("fetchWithTimeout: aborted");

  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(new Error(`fetchWithTimeout: timed out after ${timeoutMs}ms`)), timeoutMs);
  function onExternalAbort(): void {
    controller.abort(externalSignal!.reason);
  }
  externalSignal?.addEventListener("abort", onExternalAbort, { once: true });

  try {
    return await fetch(url, { ...init, signal: controller.signal });
  } finally {
    clearTimeout(timer);
    externalSignal?.removeEventListener("abort", onExternalAbort);
  }
}
//#endregion ⏱️FetchWithTimeout

//#region 🔔️WaitForEvent
/** @emoji 🔔️ Subscribes `handler` to fire on the next occurrence and returns an unsubscribe. */
export type EventSubscribe<T> = (handler: (value: T) => void) => () => void;

/** @emoji 🔔️ Options for {@link waitForEvent}. */
export interface WaitForEventOptions {
  readonly signal?: AbortSignal;
}

/**
 * @emoji 🔔️ One-shot event-driven gate: resolves with the first value `subscribe` delivers, or
 * rejects if `signal` aborts first (including if it is already aborted). The subscription is torn
 * down on both exit paths — no listener survives past this call, unlike a fixed `setTimeout` wait
 * that either fires early/late or leaks if nothing ever arrives.
 * Einmaliges ereignisgesteuertes Warten: löst beim ersten Ereignis auf oder lehnt bei Abbruch ab; das
 * Abonnement wird in beiden Fällen entfernt.
 */
export function waitForEvent<T>(subscribe: EventSubscribe<T>, options?: WaitForEventOptions): Promise<T> {
  const signal = options?.signal;
  return new Promise<T>((resolve, reject) => {
    if (signal?.aborted) {
      reject(signal.reason ?? new Error("waitForEvent: aborted"));
      return;
    }
    let unsubscribe: (() => void) | null = null;
    function cleanup(): void {
      unsubscribe?.();
      unsubscribe = null;
      signal?.removeEventListener("abort", onAbort);
    }
    function onAbort(): void {
      cleanup();
      reject(signal!.reason ?? new Error("waitForEvent: aborted"));
    }
    unsubscribe = subscribe((value) => {
      cleanup();
      resolve(value);
    });
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}
//#endregion 🔔️WaitForEvent

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("../../🧪️tests/🧪️docklayoutstore/🟦️.ts");
  await registerTests1(import.meta.vitest, { ActionId, ActorId, ActorSystem, BitSet, DockLayoutStore, DockUiStateStore, EventId, GuardId, InvokeId, Model, NamedLayoutStore, NodeId, NullInspector, OsShellConfig, OsTransient, TestHost, TimerId, TraceInspector, WindowPaneStateStore, checkInvariants, createDevPluginSource, createExtensionSource, createLeasePool, createMemoryStoragePort, effectiveActionArgs, ephemeralBox, explore, extensionSourceEventToPluginSourceEvent, fetchWithTimeout, init, latestWins, macrostep, missingRequiredArgs, multiplexPluginSources, organizeContextMenu, persist, resolvePlaygroundBoot, resolvePluginHostConfig, resolvePluginRegistryId, restore, retryWithJitteredBackoff, runConformance, start, step, timerElapsed, waitForEvent }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
