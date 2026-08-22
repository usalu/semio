/** @emoji 📦️ `@semio-tech/framework` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/🎯️action-bus/🟦️component.ts";
export * from "../../🔨️modules/🧮️action-argument-resolution/🟦️component.ts";
export * from "../../🔨️modules/🧬️schema/🟦️component.ts";
export * from "../../🔨️modules/🖥️platform/🟦️component.ts";
export * from "../../🔨️modules/🔺️mesh/🟦️component.ts";
export * from "../../🔨️modules/🛂️manifest/🟦️component.ts";
// 🕹️wave-2b: named (not `export *`) — the 🕹️interaction module's own `InteractionDefinition`/`MergeMode`/…
// family is already re-exported above via `🛂️manifest` (owned-schema-generated mirror of the same Rust types),
// so a second blanket export of the module root would collide; only its presence-broadcast leaf types,
// which nothing else exports yet, are pulled in here for `@semio-tech/framework` consumers like the OS Shell.
export type { PresenceDomain, PresenceInteraction } from "../../🔨️modules/🕹️interaction/🧬️schema/🟦️component.ts";
export * from "../../🔨️modules/🎠️kernel/🟦️component.ts";
export * from "../../🔨️modules/🔄️machine/🟦️component.ts";

import {
  organizeContextMenu,
  type ContextMenuItemSpec,
} from "../../🔨️modules/🔺️mesh/🟦️component.ts";
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
} from "../../🔨️modules/🖥️platform/🟦️component.ts";
import {
  createDevPluginSource,
  createExtensionSource,
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
} from "../../🔨️modules/🎠️kernel/🟦️component.ts";
import { effectiveActionArgs, missingRequiredArgs } from "../../🔨️modules/🧮️action-argument-resolution/🟦️component.ts";
import { type ActionArgDef, type ArgSchema } from "../../🔨️modules/🛂️manifest/🟦️component.ts";
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
} from "../../🔨️modules/🔄️machine/🟦️component.ts";

//#region 🪶️LeasePool
// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): relocated unchanged from
// `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` — 11 total consumers, of which only 4 were plugin-
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
  const { describe, expect, it, vi } = import.meta.vitest;

  describe("DockLayoutStore", () => {
    const emptySkeleton = (): DockSkeleton => ({
      version: 3,
      anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
    });

    it("returns null when nothing persisted", () => {
      const store = new DockLayoutStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      const appSkeleton: DockSkeleton = { ...emptySkeleton(), anchors: { ...emptySkeleton().anchors, "top-left": [{ id: "a" }] } };
      store.saveOs(osSkeleton);
      store.save(appSkeleton);
      expect(store.getSnapshot()).toEqual(appSkeleton);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      store.saveOs(osSkeleton);
      expect(store.getSnapshot()).toEqual(osSkeleton);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.save(emptySkeleton());
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts.apps["my-app"]).toEqual(emptySkeleton());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.saveOs(emptySkeleton());
      store.save(emptySkeleton());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new DockLayoutStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockLayouts: { os: { version: 1, corners: { "top-left": [{ id: "a" }] } } as unknown as DockSkeleton, apps: {} } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockLayouts: { os: { version: 2, anchors: {} } as unknown as DockSkeleton, apps: {} } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });

  describe("DockUiStateStore", () => {
    const emptyUiState = (): DockUiState => ({ version: 3, anchors: {} });

    it("returns null when nothing persisted", () => {
      const store = new DockUiStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState = emptyUiState();
      const appState: DockUiState = { ...emptyUiState(), anchors: { "top-left": { visible: true, size: 320 } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState: DockUiState = { ...emptyUiState(), pathMemory: { "framework.category.workbench": "framework.panel.artifact" } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.save(emptyUiState());
      expect(new OsShellConfig(storage).getSnapshot().dockUi.apps["my-app"]).toEqual(emptyUiState());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().dockUi.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.saveOs(emptyUiState());
      store.save(emptyUiState());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().dockUi).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new DockUiStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockUi: { os: { version: 1, corners: {} } as unknown as DockUiState, apps: {} } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockUi: { os: { version: 2, anchors: {} } as unknown as DockUiState, apps: {} } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it('keeps dock layout and dock ui as distinct projections for an app literally named "ui"', () => {
      const storage = createMemoryStoragePort();
      new DockLayoutStore(storage, "ui").save({
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      });
      new DockUiStateStore(storage).saveOs(emptyUiState());
      const config = new OsShellConfig(storage).getSnapshot();
      expect(config.dockLayouts.apps.ui).toBeDefined();
      expect(config.dockUi.os).toEqual(emptyUiState());
    });
  });

  describe("WindowPaneStateStore", () => {
    const emptyPaneState = (): WindowPaneUiState => ({ version: 1, windows: {} });

    it("returns null when nothing persisted", () => {
      const store = new WindowPaneStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState = emptyPaneState();
      const appState: WindowPaneUiState = { version: 1, windows: { "puzzle3d.play": { utilities: { anchor: "bottom-left", folded: false, size: 280 } } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState: WindowPaneUiState = { version: 1, windows: { "puzzle3d.play": { measures: { anchor: "top-right", size: 320 } } } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.save(emptyPaneState());
      expect(new OsShellConfig(storage).getSnapshot().windowPanes.apps["my-app"]).toEqual(emptyPaneState());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().windowPanes.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.saveOs(emptyPaneState());
      store.save(emptyPaneState());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().windowPanes).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new WindowPaneStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a foreign-version blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, windowPanes: { os: { version: 2, windows: {} } as unknown as WindowPaneUiState, apps: {} } }));
      const store = new WindowPaneStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });

  describe("OsShellConfig", () => {
    it("consolidates all four persisted shell projections into one config document", () => {
      const values = new Map<string, string>();
      const storage = {
        get: (key: string) => values.get(key) ?? null,
        set: (key: string, value: string) => void values.set(key, value),
        remove: (key: string) => void values.delete(key),
      };
      const skeleton: DockSkeleton = {
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      };
      new NamedLayoutStore("draw", storage).save({ id: "wide", label: "Wide", origin: "user", layout: { root: { kind: "stack", children: [] } } });
      new DockLayoutStore(storage, "draw").save(skeleton);
      new DockUiStateStore(storage, "draw").save({ version: 3, anchors: { "left-middle": { visible: true } } });
      new WindowPaneStateStore(storage, "draw").save({ version: 1, windows: {} });
      new OsShellConfig(storage).setPreference("ui.chrome.locale", "de");

      expect([...values.keys()]).toEqual(["semio.os.config"]);
      const snapshot = new OsShellConfig(storage).getSnapshot();
      expect(snapshot.namedLayouts.draw?.[0]?.id).toBe("wide");
      expect(snapshot.dockLayouts.apps.draw).toEqual(skeleton);
      expect(snapshot.dockUi.apps.draw?.anchors["left-middle"]?.visible).toBe(true);
      expect(snapshot.windowPanes.apps.draw).toEqual({ version: 1, windows: {} });
      expect(snapshot.preferences["ui.chrome.locale"]).toBe("de");
    });
  });

  // 🧪️ Framework-level tests must not know about real product plugin ids (`"puzzle3d"`, `"s"`, …) — a
  // small synthetic catalog stands in for the OS product's generated `PLUGIN_CATALOG` here.
  const SYNTHETIC_PLUGIN_CATALOG: PluginCatalog = {
    plugins: [
      { pluginId: "alpha", wasmOut: "alpha.wasm", role: "plugin", contributes: [], consumes: [] },
      { pluginId: "beta", wasmOut: "beta.wasm", role: "plugin", contributes: [], consumes: [] },
    ],
    extensions: [{ pluginId: "beta-extension-gamma", wasmOut: "beta_gamma.wasm", role: "extension", contributes: ["beta.module"], consumes: [] }],
    hosts: [{ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" }],
    playgrounds: [
      { variant: "alpha", pluginId: "alpha", aliases: [] },
      { variant: "beta-play", pluginId: "beta", app: "beta-play-app", aliases: ["b", "beta play"] },
    ],
    moduleUrl: (pluginId, wasmOut) => `/plugin-modules/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`,
    extensionModuleUrl: (pluginId, wasmOut) => `/extensions/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`,
  };

  describe("PlaygroundResolution", () => {
    it("resolves host config from the injected catalog", () => {
      expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "alpha")).toEqual({ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" });
      expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "beta-play")).toBeUndefined();
    });

    it("resolves playground aliases to registry plugin ids", () => {
      expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "b")).toBe("beta");
      expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "beta play")).toBe("beta");
    });

    it("rebuilds program rows when the generated session variant is stale", () => {
      const boot = resolvePlaygroundBoot(SYNTHETIC_PLUGIN_CATALOG, "beta-play", {
        variant: "alpha",
        defaultAppId: "alpha-app",
        plugins: [{ pluginId: "alpha", moduleUrl: "/plugin-modules/alpha/alpha_plugin.js" }],
      });
      expect(boot.variant).toBe("beta-play");
      expect(boot.defaultAppId).toBe("beta-play-app");
      expect(boot.plugins).toEqual([{ pluginId: "beta", moduleUrl: "/plugin-modules/beta/beta.js", contributes: [], consumes: [] }]);
    });
  });

  describe("effectiveActionArgs", () => {
    const TEXT_SCHEMA: ArgSchema = { kind: "string", options: [] };
    const textArg = (id: string, extra: Partial<ActionArgDef> = {}): ActionArgDef => ({
      id,
      label: id,
      schema: TEXT_SCHEMA,
      required: false,
      ...extra,
    });

    it("keeps a seeded arg that is not a declared form field, alongside the form's own staged fields (26/08/16 HUB-SPACES shareSpace regression: spaceId must reach the dispatched descriptor)", () => {
      const defs = [textArg("email")];
      const effective = effectiveActionArgs(defs, { email: "user2@semio.dev" }, { spaceId: "sp-1" });
      expect(effective).toEqual({ spaceId: "sp-1", email: "user2@semio.dev" });
    });

    it("a seed value for a declared field pre-fills it until the form stages its own value (renameSpace's current-name prefill)", () => {
      const defs = [textArg("name")];
      expect(effectiveActionArgs(defs, {}, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "Old Name" });
      expect(effectiveActionArgs(defs, { name: "New Name" }, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "New Name" });
    });

    it("a zero-declared-field confirm dialog passes seed+staged through wholesale (deleteSpace's confirm/cancel shape)", () => {
      expect(effectiveActionArgs([], {}, { spaceId: "sp-1", confirmed: true })).toEqual({ spaceId: "sp-1", confirmed: true });
    });

    it("missingRequiredArgs is unaffected by extra seed keys", () => {
      const defs = [textArg("email", { required: true })];
      const effective = effectiveActionArgs(defs, {}, { spaceId: "sp-1" });
      expect(missingRequiredArgs(defs, effective)).toEqual(["email"]);
    });
  });

  describe("organizeContextMenu", () => {
    const menuLeaf = (id: string): ContextMenuItemSpec => ({ id, label: id, action: id });
    const menuDestructive = (id: string): ContextMenuItemSpec => ({ ...menuLeaf(id), destructive: true });

    it("keeps a flat within-budget menu as-is, with groups sorted after leaves", () => {
      const items = [menuLeaf("a"), menuLeaf("b"), { id: "menu.group.view", children: [menuLeaf("c")] }];
      expect(organizeContextMenu(items, () => undefined)).toEqual(items);
    });

    it("shares the Rust fixture's grouped structure for a flat 12-item over-budget menu", () => {
      // 🗂️ Mirrors `organize_context_menu_buckets_overflow_leaves_by_category_of` (5 primaries + N
      // categorized overflow leaves) combined with `organize_context_menu_puts_destructive_leaves_last_after_a_separator`
      // (a trailing destructive leaf) — same shape the Rust test suite asserts for an equivalent input.
      const items: ContextMenuItemSpec[] = [
        menuLeaf("primary0"),
        menuLeaf("primary1"),
        menuLeaf("primary2"),
        menuLeaf("primary3"),
        menuLeaf("primary4"),
        menuLeaf("overflow0"),
        menuLeaf("overflow1"),
        menuLeaf("overflow2"),
        menuLeaf("overflow3"),
        menuLeaf("overflow4"),
        menuLeaf("overflow5"),
        menuDestructive("delete"),
      ];
      const categoryOf = (id: string): string | undefined => (id.startsWith("overflow") ? "view" : undefined);
      const organized = organizeContextMenu(items, categoryOf);

      expect(organized.map((item) => item.id)).toEqual([
        "primary0",
        "primary1",
        "primary2",
        "primary3",
        "primary4",
        "menu.group.view",
        "separator-organized-6",
        "delete",
      ]);
      expect(organized[5]!.children?.map((child) => child.id)).toEqual([
        "overflow0",
        "overflow1",
        "overflow2",
        "overflow3",
        "overflow4",
        "overflow5",
      ]);
      expect(organized[6]!.separator).toBe(true);
      expect(organized[6]!.label).toBeUndefined();
      expect(organized[7]!.destructive).toBe(true);
    });
  });

  describe("PluginSource", () => {
    const registry: readonly PluginRegistryEntry[] = [
      { pluginId: "note", moduleUrl: "/plugin-modules/note/note_plugin.js" },
      { pluginId: "s", moduleUrl: "/plugin-modules/s/s_plugin.js" },
    ];

    it("list() returns the registry it was created with", async () => {
      const source = createDevPluginSource(registry);
      expect(source.id).toBe("dev");
      await expect(source.list()).resolves.toEqual(registry);
    });

    it("moduleUrl() passes through unbusted without rebuiltAt", () => {
      const source = createDevPluginSource(registry);
      expect(source.moduleUrl("note")).toBe("/plugin-modules/note/note_plugin.js");
    });

    it("moduleUrl() cache-busts with a rebuiltAt query param", () => {
      const source = createDevPluginSource(registry);
      expect(source.moduleUrl("note", 1785789943669)).toBe("/plugin-modules/note/note_plugin.js?v=1785789943669");
    });

    it("moduleUrl() throws for an unknown pluginId", () => {
      const source = createDevPluginSource(registry);
      expect(() => source.moduleUrl("missing")).toThrow(/missing/);
    });

    it("subscribe() is a harmless no-op without a global EventSource (node/vitest)", () => {
      const source = createDevPluginSource(registry);
      const events: PluginSourceEvent[] = [];
      const unsubscribe = source.subscribe((event) => events.push(event));
      expect(() => unsubscribe()).not.toThrow();
      expect(events).toEqual([]);
    });

    it("multiplexPluginSources() merges list() and resolves moduleUrl from the matching child", async () => {
      const catalog: PluginCatalog = {
        plugins: [],
        extensions: [{ pluginId: "gamma-extension", wasmOut: "gamma.wasm", role: "extension", contributes: [], consumes: [] }],
        hosts: [],
        playgrounds: [],
        moduleUrl: (pluginId, wasmOut) => `/plugin-modules/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`,
        extensionModuleUrl: (pluginId, wasmOut) => `/extensions/${pluginId}/${wasmOut.replace(/\.wasm$/, ".js")}`,
      };
      const dev = createDevPluginSource(registry);
      const extensions = createExtensionSource(catalog);
      const multiplexed = multiplexPluginSources(dev, extensions);
      expect(multiplexed.id).toBe("dev+extensions");
      const listed = await multiplexed.list();
      expect(listed.map((entry) => entry.pluginId).sort()).toEqual([...registry.map((entry) => entry.pluginId), ...catalog.extensions.map((entry) => entry.pluginId)].sort());
      expect(multiplexed.moduleUrl("note")).toBe("/plugin-modules/note/note_plugin.js");
      expect(() => multiplexed.moduleUrl("missing")).toThrow(/missing/);
    });
  });

  describe("ephemeralBox", () => {
    it("stores a function-typed init as the current value (not as a lazy factory)", () => {
      const identity = (id: string) => id;
      const box = ephemeralBox<(id: string) => string>(`test.ephemeralBox.fn.${Math.random()}`, identity);
      expect(typeof box.current).toBe("function");
      expect(box.current("ui.nav.back")).toBe("ui.nav.back");
    });

    it("stores a no-op function init without invoking it", () => {
      let calls = 0;
      const noop = () => {
        calls += 1;
      };
      const box = ephemeralBox<() => void>(`test.ephemeralBox.noop.${Math.random()}`, noop);
      expect(typeof box.current).toBe("function");
      expect(calls).toBe(0);
      box.current();
      expect(calls).toBe(1);
    });

    it("is owned by an isolatable, resettable OsTransient lane", () => {
      const left = new OsTransient();
      const right = new OsTransient();
      const leftBox = left.box("cursor", { x: 1 });
      leftBox.current.x = 2;
      expect(left.box("cursor", { x: 99 })).toBe(leftBox);
      expect(right.box("cursor", { x: 3 }).current.x).toBe(3);

      const oldMap = left.map<string, number>("measurements");
      oldMap.set("width", 42);
      left.reset();
      expect(left.map<string, number>("measurements")).not.toBe(oldMap);
      expect(left.map<string, number>("measurements").size).toBe(0);
      expect(oldMap.get("width")).toBe(42);
    });
  });

  describe("LeasePool evictNow (hot-swap reload eviction)", () => {
    it("disposes a fully-released key immediately", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
        { lingerMs: 30_000 },
      );
      const lease = await pool.acquire("url-v1");
      lease.release();
      expect(disposed).toEqual([]);
      pool.evictNow("url-v1");
      expect(disposed).toEqual(["value:url-v1"]);
    });

    it("skips (does not throw) a key with an active lease, matching a reload that hasn't released the old handle yet", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
      );
      const lease = await pool.acquire("url-v1");
      expect(() => pool.evictNow("url-v1")).not.toThrow();
      expect(disposed).toEqual([]);
      lease.release();
      pool.evictNow("url-v1");
      expect(disposed).toEqual(["value:url-v1"]);
    });

    it("treats two cache-busted URLs of the same pluginId as independent keys", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
      );
      const oldLease = await pool.acquire("note.js?v=1");
      const newLease = await pool.acquire("note.js?v=2");
      oldLease.release();
      pool.evictNow("note.js?v=1");
      expect(disposed).toEqual(["value:note.js?v=1"]);
      newLease.release();
      pool.evictNow("note.js?v=2");
      expect(disposed).toEqual(["value:note.js?v=1", "value:note.js?v=2"]);
    });
  });

  //#region 🔄️MachineFixtures
  class UnitFlipEvent implements StatechartEvent {
    readonly eventCount = 1;
    eventId(): EventId {
      return EventId(0);
    }
    eventName(): string {
      return "Flip";
    }
  }
  const UNIT_FLIP = new UnitFlipEvent();

  interface UnitToggleSpec extends MachineSpec {
    readonly Context: { readonly count: number };
    readonly Event: UnitFlipEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }

  const UNIT_TOGGLE_NODES: readonly NodeDef[] = [
    { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
    { stableId: "off", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
    { stableId: "on", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
  ];
  const UNIT_TOGGLE_TRANSITIONS: readonly TransitionDef[] = [
    { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
    { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 1 },
  ];
  const UNIT_TOGGLE_MACHINE: Machine<UnitToggleSpec> = {
    definition: { id: "unit_toggle", nodes: UNIT_TOGGLE_NODES, transitions: UNIT_TOGGLE_TRANSITIONS, contextFromInput: () => ({ count: 0 }), guards: [], actions: [], fingerprint: 42n, manifestJson: "{}" },
  };

  interface ToggleSpec extends MachineSpec {
    readonly Context: { count: number; allow: boolean };
    readonly Event: UnitFlipEvent;
    readonly Input: boolean;
    readonly Output: void;
    readonly Effect: void;
  }
  const TOGGLE_MACHINE: Machine<ToggleSpec> = {
    definition: {
      id: "toggle",
      nodes: UNIT_TOGGLE_NODES,
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [ActionId(0)], docIndex: 0 },
        { source: NodeId(2), trigger: { kind: "event", event: EventId(0) }, guard: GuardId(0), targets: [NodeId(1)], kind: "external", actions: [ActionId(0)], docIndex: 1 },
      ],
      contextFromInput: (allow) => ({ count: 0, allow }),
      guards: [(ctx) => ctx.allow],
      actions: [(ctx) => (ctx.count += 1)],
      fingerprint: 1n,
      manifestJson: "{}",
    },
  };

  class PlayerEvent implements StatechartEvent {
    private static readonly IDS = { open: 0, pause: 1, play: 2, stop: 3, resume: 4 } as const;
    private static readonly NAMES = ["Open", "Pause", "Play", "Stop", "Resume"];
    readonly eventCount = 5;
    readonly type: keyof typeof PlayerEvent.IDS;
    constructor(type: keyof typeof PlayerEvent.IDS) {
      this.type = type;
    }
    eventId(): EventId {
      return EventId(PlayerEvent.IDS[this.type]);
    }
    eventName(id: EventId): string {
      return PlayerEvent.NAMES[id] ?? "?";
    }
  }
  interface PlayerSpec extends MachineSpec {
    readonly Context: Record<string, never>;
    readonly Event: PlayerEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }
  const PLAYER_MACHINE: Machine<PlayerSpec> = {
    definition: {
      id: "player",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(3)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "closed", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "playing", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "open", kind: "compound", parent: NodeId(0), initial: NodeId(2), children: [NodeId(2), NodeId(4), NodeId(5)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "paused", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 4 },
        { stableId: "open.history", kind: "historyShallow", parent: NodeId(3), initial: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(3)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(2), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(4)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 2 },
        { source: NodeId(3), trigger: { kind: "event", event: EventId(3) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 3 },
        { source: NodeId(1), trigger: { kind: "event", event: EventId(4) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 4 },
      ],
      contextFromInput: () => ({}),
      guards: [],
      actions: [],
      fingerprint: 2n,
      manifestJson: "{}",
    },
  };

  class RecorderEvent implements StatechartEvent {
    private static readonly IDS = { start: 0, audioStop: 1, videoStop: 2 } as const;
    private static readonly NAMES = ["Start", "AudioStop", "VideoStop"];
    readonly eventCount = 3;
    readonly type: keyof typeof RecorderEvent.IDS;
    constructor(type: keyof typeof RecorderEvent.IDS) {
      this.type = type;
    }
    eventId(): EventId {
      return EventId(RecorderEvent.IDS[this.type]);
    }
    eventName(id: EventId): string {
      return RecorderEvent.NAMES[id] ?? "?";
    }
  }
  interface RecorderSpec extends MachineSpec {
    readonly Context: Record<string, never>;
    readonly Event: RecorderEvent;
    readonly Input: void;
    readonly Output: void;
    readonly Effect: void;
  }
  const RECORDER_MACHINE: Machine<RecorderSpec> = {
    definition: {
      id: "recorder",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "idle", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "recording", kind: "parallel", parent: NodeId(0), children: [NodeId(3), NodeId(6)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "audio", kind: "compound", parent: NodeId(2), initial: NodeId(4), children: [NodeId(4), NodeId(5)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "audio.capturing", kind: "atomic", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 4 },
        { stableId: "audio.done", kind: "final", parent: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
        { stableId: "video", kind: "compound", parent: NodeId(2), initial: NodeId(7), children: [NodeId(7), NodeId(8)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 6 },
        { stableId: "video.capturing", kind: "atomic", parent: NodeId(6), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 7 },
        { stableId: "video.done", kind: "final", parent: NodeId(6), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 8 },
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(1) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(7), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(8)], kind: "external", actions: [], docIndex: 2 },
        { source: NodeId(2), trigger: { kind: "done", node: NodeId(2) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 3 },
      ],
      contextFromInput: () => ({}),
      guards: [],
      actions: [],
      fingerprint: 3n,
      manifestJson: "{}",
    },
  };

  class CheckoutEvent implements StatechartEvent {
    private static readonly IDS = { confirm: 0, selectMethod: 1, paymentSucceeded: 2, paymentFailed: 3, retry: 4, cancel: 5, resume: 6, shipDone: 7, invoiceDone: 8 } as const;
    private static readonly NAMES = ["Confirm", "SelectMethod", "PaymentSucceeded", "PaymentFailed", "Retry", "Cancel", "Resume", "ShipDone", "InvoiceDone"];
    readonly eventCount = 9;
    readonly type: keyof typeof CheckoutEvent.IDS;
    constructor(type: keyof typeof CheckoutEvent.IDS) {
      this.type = type;
    }
    eventId(): EventId {
      return EventId(CheckoutEvent.IDS[this.type]);
    }
    eventName(id: EventId): string {
      return CheckoutEvent.NAMES[id] ?? "?";
    }
  }
  type CheckoutContext = { attempts: number; methodSet: boolean };
  type Receipt = { readonly attempts: number };
  interface CheckoutSpec extends MachineSpec {
    readonly Context: CheckoutContext;
    readonly Event: CheckoutEvent;
    readonly Input: void;
    readonly Output: Receipt;
    readonly Effect: void;
  }
  // 🧾️ Hand-compiled TS twin of the Rust `checkout_integration` module's `statechart!` DSL machine —
  // TS has no `statechart!`/`export_wasm_machine!` derive macros, so its dense node/transition tables
  // are authored directly (see the twin's WasmBridge decision in the ticket report). Node ids: 0 root,
  // 1 cart, 2 payment, 3 selecting, 4 processing, 5 failed, 6 payment_history, 7 fulfilment,
  // 8 shipping, 9 ship_pending, 10 ship_done, 11 invoicing, 12 invoice_pending, 13 invoice_done, 14 done.
  const CHECKOUT_MACHINE: Machine<CheckoutSpec> = {
    definition: {
      id: "checkout",
      nodes: [
        { stableId: "root", kind: "compound", initial: NodeId(1), children: [NodeId(1), NodeId(2), NodeId(7), NodeId(14)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 0 },
        { stableId: "cart", kind: "atomic", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 1 },
        { stableId: "payment", kind: "compound", parent: NodeId(0), initial: NodeId(3), children: [NodeId(3), NodeId(4), NodeId(5), NodeId(6)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 2 },
        { stableId: "selecting", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 3 },
        { stableId: "processing", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [InvokeId(0)], timers: [[TimerId(0), 5000]], docIndex: 4 },
        { stableId: "failed", kind: "atomic", parent: NodeId(2), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 5 },
        { stableId: "payment_history", kind: "historyShallow", parent: NodeId(2), initial: NodeId(3), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 6 },
        { stableId: "fulfilment", kind: "parallel", parent: NodeId(0), children: [NodeId(8), NodeId(11)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 7 },
        { stableId: "shipping", kind: "compound", parent: NodeId(7), initial: NodeId(9), children: [NodeId(9), NodeId(10)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 8 },
        { stableId: "ship_pending", kind: "atomic", parent: NodeId(8), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 9 },
        { stableId: "ship_done", kind: "final", parent: NodeId(8), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 10 },
        { stableId: "invoicing", kind: "compound", parent: NodeId(7), initial: NodeId(12), children: [NodeId(12), NodeId(13)], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 11 },
        { stableId: "invoice_pending", kind: "atomic", parent: NodeId(11), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 12 },
        { stableId: "invoice_done", kind: "final", parent: NodeId(11), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 13 },
        { stableId: "done", kind: "final", parent: NodeId(0), children: [], entryActions: [], exitActions: [], invokes: [], timers: [], docIndex: 14 },
      ],
      transitions: [
        { source: NodeId(1), trigger: { kind: "event", event: EventId(0) }, targets: [NodeId(2)], kind: "external", actions: [], docIndex: 0 },
        { source: NodeId(1), trigger: { kind: "event", event: EventId(6) }, targets: [NodeId(6)], kind: "external", actions: [], docIndex: 1 },
        { source: NodeId(3), trigger: { kind: "event", event: EventId(1) }, guard: GuardId(0), targets: [NodeId(4)], kind: "external", actions: [ActionId(0)], docIndex: 2 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(2) }, targets: [NodeId(7)], kind: "external", actions: [], docIndex: 3 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(3) }, targets: [NodeId(5)], kind: "external", actions: [], docIndex: 4 },
        { source: NodeId(4), trigger: { kind: "event", event: EventId(5) }, targets: [NodeId(1)], kind: "external", actions: [], docIndex: 5 },
        { source: NodeId(4), trigger: { kind: "timer", timer: TimerId(0) }, targets: [NodeId(5)], kind: "external", actions: [ActionId(1)], docIndex: 6 },
        { source: NodeId(5), trigger: { kind: "event", event: EventId(4) }, targets: [NodeId(4)], kind: "external", actions: [], docIndex: 7 },
        { source: NodeId(9), trigger: { kind: "event", event: EventId(7) }, targets: [NodeId(10)], kind: "external", actions: [], docIndex: 8 },
        { source: NodeId(12), trigger: { kind: "event", event: EventId(8) }, targets: [NodeId(13)], kind: "external", actions: [], docIndex: 9 },
        { source: NodeId(7), trigger: { kind: "done", node: NodeId(7) }, targets: [NodeId(14)], kind: "external", actions: [], docIndex: 10 },
      ],
      contextFromInput: () => ({ attempts: 0, methodSet: false }),
      makeOutput: (ctx) => ({ attempts: ctx.attempts }),
      guards: [(ctx) => ctx.attempts < 3],
      actions: [(ctx) => (ctx.methodSet = true), (ctx) => (ctx.attempts += 1)],
      fingerprint: 100n,
      manifestJson: "{}",
    },
  };
  //#endregion 🔄️MachineFixtures

  describe("machine: TestHost", () => {
    it("advance fires due timers only", () => {
      const host = new TestHost<UnitToggleSpec>();
      host.schedule(ActorId(0), TimerId(0), 100);
      host.schedule(ActorId(0), TimerId(1), 300);
      expect(host.advance(150)).toEqual([[0, 0]]);
      expect(host.advance(200)).toEqual([[0, 1]]);
    });

    it("cancelTimer removes pending", () => {
      const host = new TestHost<UnitToggleSpec>();
      host.schedule(ActorId(0), TimerId(0), 100);
      host.cancelTimer(ActorId(0), TimerId(0));
      expect(host.advance(200)).toEqual([]);
    });

    it("records effects and task lifecycle", () => {
      const host = new TestHost<{ Context: void; Event: UnitFlipEvent; Input: void; Output: void; Effect: string }>();
      host.executeEffect(ActorId(0), "audit");
      expect(host.effects()).toEqual([[0, "audit"]]);
      host.startTask(ActorId(0), InvokeId(0));
      expect(host.startedTasks()).toEqual([[0, 0]]);
      host.cancelTask(ActorId(0), InvokeId(0));
      expect(host.startedTasks()).toEqual([]);
      expect(host.cancelledTasks()).toEqual([[0, 0]]);
    });
  });

  describe("machine: TraceInspector", () => {
    it("records one microstep per transition", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const inspector = new TraceInspector<UnitToggleSpec>();
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(inspector.entries.length).toBe(2);
      expect(inspector.entries[0]!.exited).toEqual([NodeId(1)]);
      expect(inspector.entries[0]!.entered).toEqual([NodeId(2)]);
      expect(inspector.entries[1]!.exited).toEqual([NodeId(2)]);
      expect(inspector.entries[1]!.entered).toEqual([NodeId(1)]);
    });
  });

  describe("machine: kernel", () => {
    it("flat machine toggles and counts", () => {
      const sink: Command<ToggleSpec>[] = [];
      const snapshot = init(TOGGLE_MACHINE, true, sink);
      expect(snapshot.matches("off")).toBe(true);
      const inspector = new NullInspector<ToggleSpec>();
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      expect(snapshot.context.count).toBe(1);
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("off")).toBe(true);
      expect(snapshot.context.count).toBe(2);
    });

    it("guard blocks transition when false", () => {
      const sink: Command<ToggleSpec>[] = [];
      const snapshot = init(TOGGLE_MACHINE, false, sink);
      const inspector = new NullInspector<ToggleSpec>();
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      macrostep(TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, inspector);
      expect(snapshot.matches("on")).toBe(true);
      expect(snapshot.context.count).toBe(1);
    });

    it("hierarchical machine enters default descendant", () => {
      const sink: Command<PlayerSpec>[] = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      expect(snapshot.matches("closed")).toBe(true);
      expect(snapshot.matches("open")).toBe(false);
    });

    it("hierarchical machine transitions into compound default", () => {
      const sink: Command<PlayerSpec>[] = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("open"), sink, new NullInspector());
      expect(snapshot.matches("open")).toBe(true);
      expect(snapshot.matches("playing")).toBe(true);
    });

    it("shallow history restores last active child", () => {
      const sink: Command<PlayerSpec>[] = [];
      const snapshot = init(PLAYER_MACHINE, undefined, sink);
      const inspector = new NullInspector<PlayerSpec>();
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("open"), sink, inspector);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("pause"), sink, inspector);
      expect(snapshot.matches("paused")).toBe(true);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("stop"), sink, inspector);
      expect(snapshot.matches("closed")).toBe(true);
      expect(snapshot.matches("open")).toBe(false);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("resume"), sink, inspector);
      expect(snapshot.matches("open")).toBe(true);
      expect(snapshot.matches("paused")).toBe(true);
      expect(snapshot.matches("playing")).toBe(false);
      macrostep(PLAYER_MACHINE, snapshot, new PlayerEvent("play"), sink, inspector);
      expect(snapshot.matches("playing")).toBe(true);
      expect(snapshot.matches("paused")).toBe(false);
    });

    it("parallel regions enter together", () => {
      const sink: Command<RecorderSpec>[] = [];
      const snapshot = init(RECORDER_MACHINE, undefined, sink);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("start"), sink, new NullInspector());
      expect(snapshot.matches("recording")).toBe(true);
      expect(snapshot.matches("audio.capturing")).toBe(true);
      expect(snapshot.matches("video.capturing")).toBe(true);
    });

    it("parallel done bubbles only once every region finishes", () => {
      const sink: Command<RecorderSpec>[] = [];
      const snapshot = init(RECORDER_MACHINE, undefined, sink);
      const inspector = new NullInspector<RecorderSpec>();
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("start"), sink, inspector);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("audioStop"), sink, inspector);
      expect(snapshot.matches("audio.done")).toBe(true);
      expect(snapshot.matches("recording")).toBe(true);
      macrostep(RECORDER_MACHINE, snapshot, new RecorderEvent("videoStop"), sink, inspector);
      expect(snapshot.matches("idle")).toBe(true);
      expect(snapshot.matches("recording")).toBe(false);
    });
  });

  describe("machine: persist/restore", () => {
    it("persist then restore round-trips active state", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      macrostep(UNIT_TOGGLE_MACHINE, snapshot, UNIT_FLIP, sink, new NullInspector());
      expect(snapshot.matches("on")).toBe(true);

      const persisted = persist(UNIT_TOGGLE_MACHINE, snapshot);
      expect(persisted.fingerprint).toBe(UNIT_TOGGLE_MACHINE.definition.fingerprint);
      expect(persisted.states).toContain("on");

      const restored = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, []);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("on")).toBe(true);
    });

    it("restore rejects fingerprint mismatch without migration", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const persisted: PersistedSnapshot = { ...persist(UNIT_TOGGLE_MACHINE, snapshot), fingerprint: 9999n };
      const result = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, []);
      expect(result).toEqual({ ok: false, error: { kind: "fingerprintMismatch" } });
    });

    it("restore applies migration chain until fingerprint matches", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const persisted: PersistedSnapshot = { ...persist(UNIT_TOGGLE_MACHINE, snapshot), fingerprint: 9999n };
      const migration: Migration = { sourceFingerprint: 9999n, migrate: (s) => ({ ...s, fingerprint: UNIT_TOGGLE_MACHINE.definition.fingerprint }) };
      const restored = restore(UNIT_TOGGLE_MACHINE, persisted, { count: 0 }, [migration]);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("off")).toBe(true);
    });
  });

  describe("machine: ActorSystem", () => {
    it("drains sent events through one macrostep each", () => {
      const system = new ActorSystem<UnitToggleSpec>(new TestHost<UnitToggleSpec>(), UNIT_TOGGLE_MACHINE);
      const root = system.spawnRoot(undefined);
      expect(system.snapshot(root)!.matches("off")).toBe(true);

      system.send(root, UNIT_FLIP);
      const reports = system.drain();
      expect(reports.length).toBe(1);
      expect(system.snapshot(root)!.matches("on")).toBe(true);

      system.send(root, UNIT_FLIP);
      system.drain();
      expect(system.snapshot(root)!.matches("off")).toBe(true);
      expect(system.snapshot(root)!.context).toEqual({ count: 0 });
    });
  });

  describe("machine: testing (Model/Coverage/Invariant/Conformance)", () => {
    it("explore reaches both toggle states", () => {
      const model = new Model<UnitToggleSpec>([UNIT_FLIP]);
      const coverage = explore(UNIT_TOGGLE_MACHINE, model, undefined);
      expect(coverage.reachedStableIds).toContain("off");
      expect(coverage.reachedStableIds).toContain("on");
      expect(coverage.visitedConfigurations).toBe(2);
    });

    it("conformance fixture passes for matching sequence", () => {
      const steps: ConformanceStep<UnitToggleSpec>[] = [
        { event: UNIT_FLIP, expectActive: ["on"] },
        { event: UNIT_FLIP, expectActive: ["off"] },
      ];
      expect(runConformance(UNIT_TOGGLE_MACHINE, undefined, steps).ok).toBe(true);
    });

    it("conformance fixture fails with a descriptive message", () => {
      const steps: ConformanceStep<UnitToggleSpec>[] = [{ event: UNIT_FLIP, expectActive: ["off"] }];
      const result = runConformance(UNIT_TOGGLE_MACHINE, undefined, steps);
      expect(result.ok).toBe(false);
      expect(!result.ok && result.error.message).toContain("step 0");
      expect(!result.ok && result.error.message).toContain("off");
    });

    it("invariant reports violation by name", () => {
      const sink: Command<UnitToggleSpec>[] = [];
      const snapshot = init(UNIT_TOGGLE_MACHINE, undefined, sink);
      const invariants: Invariant<UnitToggleSpec>[] = [{ name: "never off", check: (s) => (s.matches("off") ? { ok: false, error: { kind: "violation", message: "was off" } } : { ok: true }) }];
      expect(checkInvariants(snapshot, invariants)).toEqual(["never off: was off"]);
    });
  });

  describe("machine: BitSet", () => {
    it("set/clear/contains", () => {
      const bits = new BitSet();
      expect(bits.contains(NodeId(3))).toBe(false);
      bits.set(NodeId(3));
      expect(bits.contains(NodeId(3))).toBe(true);
      bits.clear(NodeId(3));
      expect(bits.contains(NodeId(3))).toBe(false);
    });

    it("iterOnes ascends regardless of insertion order", () => {
      const bits = new BitSet();
      bits.set(NodeId(100));
      bits.set(NodeId(0));
      bits.set(NodeId(64));
      bits.set(NodeId(63));
      expect([...bits.iterOnes()]).toEqual([0, 63, 64, 100]);
    });

    it("clearAll and isEmpty", () => {
      const bits = new BitSet();
      expect(bits.isEmpty()).toBe(true);
      bits.set(NodeId(5));
      expect(bits.isEmpty()).toBe(false);
      bits.clearAll();
      expect(bits.isEmpty()).toBe(true);
    });
  });

  // 🎫️ End-to-end proof that the hand-compiled `checkout` DSL twin → kernel → runtime → `TestHost`
  // timers/invoke → persist/restore → inspection trace → model coverage all compose over one real
  // machine, mirroring Rust's `checkout_integration` module.
  describe("machine: checkout DSL twin (integration)", () => {
    it("walks cart to receipt", () => {
      const host = new TestHost<CheckoutSpec>();
      const system = new ActorSystem<CheckoutSpec>(host, CHECKOUT_MACHINE);
      const root = system.spawnRoot(undefined);
      expect(system.snapshot(root)!.matches("cart")).toBe(true);

      system.send(root, new CheckoutEvent("confirm"));
      system.drain();
      expect(system.snapshot(root)!.matches("selecting")).toBe(true);

      system.send(root, new CheckoutEvent("selectMethod"));
      system.drain();
      expect(system.snapshot(root)!.matches("processing")).toBe(true);
      expect(system.snapshot(root)!.context.methodSet).toBe(true);
      expect(host.startedTasks()).toEqual([[root, 0]]);

      system.send(root, new CheckoutEvent("paymentSucceeded"));
      system.drain();
      expect(system.snapshot(root)!.matches("ship_pending")).toBe(true);
      expect(system.snapshot(root)!.matches("invoice_pending")).toBe(true);
      expect(host.cancelledTasks()).toEqual([[root, 0]]);

      system.send(root, new CheckoutEvent("shipDone"));
      system.drain();
      expect(system.snapshot(root)!.matches("ship_done")).toBe(true);
      expect(system.snapshot(root)!.matches("invoice_pending")).toBe(true);
      system.send(root, new CheckoutEvent("invoiceDone"));
      system.drain();

      const finalStatus = system.snapshot(root)!.status;
      expect(finalStatus.kind).toBe("done");
      expect(finalStatus.kind === "done" && finalStatus.output.attempts).toBe(0);
    });

    it("cancel/resume round-trips via shallow history", () => {
      const sink: Command<CheckoutSpec>[] = [];
      const snapshot = init(CHECKOUT_MACHINE, undefined, sink);
      const inspector = new TraceInspector<CheckoutSpec>();

      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("confirm"), sink, inspector);
      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("selectMethod"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);

      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("cancel"), sink, inspector);
      expect(snapshot.matches("cart")).toBe(true);
      expect(snapshot.matches("payment")).toBe(false);

      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("resume"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);
      expect(snapshot.matches("selecting")).toBe(false);
      expect(inspector.entries.length).toBeGreaterThan(0);

      const fired = timerElapsed(CHECKOUT_MACHINE, snapshot, TimerId(0), sink, inspector);
      expect(fired.microsteps).toBe(1);
      expect(snapshot.matches("failed")).toBe(true);
      expect(snapshot.context.attempts).toBe(1);

      macrostep(CHECKOUT_MACHINE, snapshot, new CheckoutEvent("retry"), sink, inspector);
      expect(snapshot.matches("processing")).toBe(true);

      const persisted = persist(CHECKOUT_MACHINE, snapshot);
      expect(persisted.fingerprint).toBe(CHECKOUT_MACHINE.definition.fingerprint);
      const restored = restore(CHECKOUT_MACHINE, persisted, { ...snapshot.context }, []);
      expect(restored.ok).toBe(true);
      expect(restored.ok && restored.snapshot.matches("processing")).toBe(true);
    });

    it("model coverage reaches every declared state", () => {
      const model = new Model<CheckoutSpec>(
        (["confirm", "selectMethod", "paymentSucceeded", "paymentFailed", "retry", "cancel", "resume", "shipDone", "invoiceDone"] as const).map((type) => new CheckoutEvent(type)),
      );
      const coverage = explore(CHECKOUT_MACHINE, model, undefined);
      for (const expected of ["cart", "selecting", "processing", "failed", "ship_pending", "ship_done", "invoice_pending", "invoice_done", "done"]) {
        expect(coverage.reachedStableIds).toContain(expected);
      }
    });

    //#region 🔖️StepTests
    it("start produces a persistable initial configuration", () => {
      const initial = start(CHECKOUT_MACHINE, undefined);
      expect(initial.isActive("cart")).toBe(true);
      expect(initial.entered).toEqual([]);
      expect(initial.persisted.fingerprint).toBe(CHECKOUT_MACHINE.definition.fingerprint);
    });

    it("step round-trips through persisted state only", () => {
      let carried = start(CHECKOUT_MACHINE, undefined).persisted;
      let context: CheckoutContext = { attempts: 0, methodSet: false };
      for (const [type, expected] of [
        ["confirm", "selecting"],
        ["selectMethod", "processing"],
      ] as const) {
        const outcome = step(CHECKOUT_MACHINE, carried, context, new CheckoutEvent(type), []);
        expect(outcome.ok).toBe(true);
        if (!outcome.ok) continue;
        expect(outcome.step.isActive(expected)).toBe(true);
        context = { ...context, methodSet: true };
        carried = outcome.step.persisted;
      }
    });

    it("step reports entered and exited states", () => {
      const initial = start(CHECKOUT_MACHINE, undefined).persisted;
      const outcome = step(CHECKOUT_MACHINE, initial, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(outcome.ok).toBe(true);
      if (!outcome.ok) return;
      expect(outcome.step.exited).toContain("cart");
      expect(outcome.step.entered).toContain("payment");
      expect(outcome.step.entered).toContain("selecting");
      expect(outcome.step.isActive("cart")).toBe(false);
    });

    it("step with a blocked guard leaves the configuration untouched", () => {
      const initial = start(CHECKOUT_MACHINE, undefined).persisted;
      const confirmed = step(CHECKOUT_MACHINE, initial, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(confirmed.ok).toBe(true);
      if (!confirmed.ok) return;
      const blocked = step(CHECKOUT_MACHINE, confirmed.step.persisted, { attempts: 3, methodSet: false }, new CheckoutEvent("selectMethod"), []);
      expect(blocked.ok).toBe(true);
      if (!blocked.ok) return;
      expect(blocked.step.isActive("selecting")).toBe(true);
      expect(blocked.step.entered).toEqual([]);
    });

    it("step rejects a persisted snapshot from another machine shape", () => {
      const foreign: PersistedSnapshot = { ...start(CHECKOUT_MACHINE, undefined).persisted };
      const mismatched: PersistedSnapshot = { ...foreign, fingerprint: foreign.fingerprint ^ 0xffffffffn };
      const outcome = step(CHECKOUT_MACHINE, mismatched, { attempts: 0, methodSet: false }, new CheckoutEvent("confirm"), []);
      expect(outcome.ok).toBe(false);
    });
    //#endregion 🔖️StepTests
  });

  //#region 🔖️RetryWithJitteredBackoffTests
  describe("retryWithJitteredBackoff", () => {
    it("keeps every backoff delay within [minMs, maxMs] and varies it across attempts", async () => {
      vi.useFakeTimers();
      try {
        const setTimeoutSpy = vi.spyOn(globalThis, "setTimeout");
        let calls = 0;
        const fn = vi.fn(async () => {
          calls += 1;
          if (calls < 6) throw new Error("retry me");
          return "ok";
        });
        const resultPromise = retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 100 });
        for (let i = 0; i < 5; i++) {
          await vi.advanceTimersByTimeAsync(100);
        }
        await expect(resultPromise).resolves.toBe("ok");
        expect(calls).toBe(6);
        const delays = setTimeoutSpy.mock.calls.map(([, ms]) => Number(ms));
        expect(delays.length).toBe(5);
        for (const delay of delays) {
          expect(delay).toBeGreaterThanOrEqual(10);
          expect(delay).toBeLessThanOrEqual(100);
        }
        expect(new Set(delays).size).toBeGreaterThan(1);
      } finally {
        vi.useRealTimers();
      }
    });

    it("rejects promptly on abort mid-wait and makes no further attempt", async () => {
      vi.useFakeTimers();
      try {
        const controller = new AbortController();
        let calls = 0;
        const fn = vi.fn(async () => {
          calls += 1;
          throw new Error("always fails");
        });
        const promise = retryWithJitteredBackoff(fn, { minMs: 50, maxMs: 50, signal: controller.signal });
        await vi.advanceTimersByTimeAsync(0);
        expect(calls).toBe(1);
        controller.abort(new Error("stop"));
        await expect(promise).rejects.toThrow("stop");
        expect(calls).toBe(1);
      } finally {
        vi.useRealTimers();
      }
    });

    it("throws immediately for an already-aborted signal without calling fn", async () => {
      const controller = new AbortController();
      controller.abort(new Error("pre-aborted"));
      const fn = vi.fn(async () => "unreachable");
      await expect(retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 10, signal: controller.signal })).rejects.toThrow("pre-aborted");
      expect(fn).not.toHaveBeenCalled();
    });
  });
  //#endregion 🔖️RetryWithJitteredBackoffTests

  //#region 🔖️LatestWinsTests
  describe("latestWins", () => {
    it("collapses N concurrent calls during an in-flight run into exactly one trailing run, and every caller sees the latest result", async () => {
      let calls = 0;
      const resolvers: Array<(value: number) => void> = [];
      const run = vi.fn(() => {
        calls += 1;
        const callIndex = calls;
        return new Promise<number>((resolve) => resolvers.push(() => resolve(callIndex)));
      });
      const trigger = latestWins(run);

      const first = trigger();
      expect(calls).toBe(1);
      const concurrentA = trigger();
      const concurrentB = trigger();
      const concurrentC = trigger();
      expect(calls).toBe(1);

      resolvers[0]!(0);
      await new Promise((resolve) => setTimeout(resolve, 0));
      expect(calls).toBe(2);

      resolvers[1]!(0);
      await expect(first).resolves.toBe(1);
      await expect(concurrentA).resolves.toBe(2);
      await expect(concurrentB).resolves.toBe(2);
      await expect(concurrentC).resolves.toBe(2);
    });

    it("starts a fresh run when called again after everything has settled", async () => {
      let n = 0;
      const trigger = latestWins(async () => {
        n += 1;
        return n;
      });
      await expect(trigger()).resolves.toBe(1);
      await expect(trigger()).resolves.toBe(2);
    });
  });
  //#endregion 🔖️LatestWinsTests

  //#region 🔖️FetchWithTimeoutTests
  describe("fetchWithTimeout", () => {
    it("aborts the underlying fetch when the timeout elapses first", async () => {
      vi.useFakeTimers();
      try {
        const fetchMock = vi.fn(
          (_url: string, init?: RequestInit) =>
            new Promise((_resolve, reject) => {
              init?.signal?.addEventListener("abort", () => reject(new Error("aborted-by-fetch")));
            }),
        );
        vi.stubGlobal("fetch", fetchMock);
        const promise = fetchWithTimeout("https://example.test", undefined, { timeoutMs: 50 });
        const assertion = expect(promise).rejects.toThrow();
        await vi.advanceTimersByTimeAsync(50);
        await assertion;
      } finally {
        vi.unstubAllGlobals();
        vi.useRealTimers();
      }
    });

    it("aborts the underlying fetch when the external signal aborts", async () => {
      const controller = new AbortController();
      const fetchMock = vi.fn(
        (_url: string, init?: RequestInit) =>
          new Promise((_resolve, reject) => {
            init?.signal?.addEventListener("abort", () => reject(new Error("aborted-by-fetch")));
          }),
      );
      vi.stubGlobal("fetch", fetchMock);
      try {
        const promise = fetchWithTimeout("https://example.test", undefined, { timeoutMs: 10_000, signal: controller.signal });
        controller.abort(new Error("caller cancelled"));
        await expect(promise).rejects.toThrow();
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("clears the timer and removes the abort listener on the success path", async () => {
      vi.useFakeTimers();
      try {
        const clearTimeoutSpy = vi.spyOn(globalThis, "clearTimeout");
        const controller = new AbortController();
        const removeEventListenerSpy = vi.spyOn(controller.signal, "removeEventListener");
        const response: FetchTimeoutResponse = { ok: true, status: 200, statusText: "OK", headers: { get: () => null }, json: async () => ({}), text: async () => "" };
        vi.stubGlobal(
          "fetch",
          vi.fn(async () => response),
        );
        const result = await fetchWithTimeout("https://example.test", undefined, { timeoutMs: 1000, signal: controller.signal });
        expect(result).toBe(response);
        expect(clearTimeoutSpy).toHaveBeenCalled();
        expect(removeEventListenerSpy).toHaveBeenCalledWith("abort", expect.any(Function));
        await vi.advanceTimersByTimeAsync(5000);
      } finally {
        vi.unstubAllGlobals();
        vi.useRealTimers();
      }
    });
  });
  //#endregion 🔖️FetchWithTimeoutTests

  //#region 🔖️WaitForEventTests
  describe("waitForEvent", () => {
    it("resolves with the first delivered value and unsubscribes", async () => {
      let handler: ((value: number) => void) | undefined;
      let unsubscribed = false;
      const subscribe: EventSubscribe<number> = (h) => {
        handler = h;
        return () => {
          unsubscribed = true;
        };
      };
      const promise = waitForEvent(subscribe);
      expect(unsubscribed).toBe(false);
      handler!(42);
      await expect(promise).resolves.toBe(42);
      expect(unsubscribed).toBe(true);
    });

    it("rejects on abort and unsubscribes without ever having received a value", async () => {
      let unsubscribed = false;
      const subscribe: EventSubscribe<number> = () => () => {
        unsubscribed = true;
      };
      const controller = new AbortController();
      const promise = waitForEvent(subscribe, { signal: controller.signal });
      controller.abort(new Error("cancelled"));
      await expect(promise).rejects.toThrow("cancelled");
      expect(unsubscribed).toBe(true);
    });

    it("rejects immediately for an already-aborted signal, never subscribing", async () => {
      const controller = new AbortController();
      controller.abort(new Error("pre-aborted"));
      let subscribed = false;
      const subscribe: EventSubscribe<number> = () => {
        subscribed = true;
        return () => {};
      };
      await expect(waitForEvent(subscribe, { signal: controller.signal })).rejects.toThrow("pre-aborted");
      expect(subscribed).toBe(false);
    });
  });
  //#endregion 🔖️WaitForEventTests
}
//#endregion 🧪️Tests
