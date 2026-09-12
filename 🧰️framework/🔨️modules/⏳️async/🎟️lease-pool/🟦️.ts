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
