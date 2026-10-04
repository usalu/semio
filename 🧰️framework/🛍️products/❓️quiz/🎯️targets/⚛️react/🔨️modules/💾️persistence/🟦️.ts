/** 💾️ Persisted local-only storage of the quiz client, shared by every tab of the site.
 *
 * Every tab reads and writes the same origin-wide area, so nothing that several tabs change may be stored as one value:
 * single-valued slices (introduction seen, learner, cached catalog and learner view, preferences) are last-writer-wins,
 * and collections (the answer outbox, cached run views) keep one key per record, so a tab's write never replaces a
 * record another tab wrote. Changes made by other tabs arrive through {@link LocalStore.watch} — the `storage` event
 * in a browser. Unavailable, full or corrupted storage degrades to "nothing stored", never to an exception.
 *
 * @see https://html.spec.whatwg.org/multipage/webstorage.html#the-storage-event
 */

import { CHALLENGES, type CatalogView, type Challenge, type Id, type LearnerView, type RunView } from "@semio-tech/quiz";

//#region 🗄️Area
/** 🗄️ An origin-wide string key/value area every tab shares, with key enumeration and notification of the changes
 * other tabs make — the owned seam over `localStorage` and its `storage` event. `watch` reports the changed key, or
 * `null` when the whole area was cleared. */
export interface StorageArea {
  get(key: string): string | null;
  set(key: string, value: string): void;
  remove(key: string): void;
  keys(): readonly string[];
  watch(listener: (key: string | null) => void): () => void;
}

function browserArea(): Storage | undefined {
  try {
    return typeof localStorage === "undefined" ? undefined : localStorage;
  } catch {
    return undefined;
  }
}

/** 🌐️ The browser's `localStorage` as a {@link StorageArea}; inert when storage is unavailable or refuses writes. */
export function browserStorageArea(): StorageArea {
  return {
    get: (key) => {
      try {
        return browserArea()?.getItem(key) ?? null;
      } catch {
        return null;
      }
    },
    set: (key, value) => {
      try {
        browserArea()?.setItem(key, value);
      } catch {
        return;
      }
    },
    remove: (key) => {
      try {
        browserArea()?.removeItem(key);
      } catch {
        return;
      }
    },
    keys: () => {
      try {
        const area = browserArea();
        return area === undefined ? [] : Array.from({ length: area.length }, (_, index) => area.key(index)).filter((key): key is string => key !== null);
      } catch {
        return [];
      }
    },
    watch: (listener) => {
      if (typeof window === "undefined") return () => undefined;
      const changed = (event: StorageEvent): void => {
        if (event.storageArea === browserArea()) listener(event.key);
      };
      window.addEventListener("storage", changed);
      return () => window.removeEventListener("storage", changed);
    },
  };
}

/** 🧠️ An in-memory origin: every {@link StorageArea} from `tab()` shares its values and, like the `storage` event, is
 * told asynchronously about changes the other tabs make — never about its own. */
export function memoryStorageOrigin(): { readonly tab: () => StorageArea } {
  const values = new Map<string, string>();
  const watchers = new Set<{ readonly tab: object; readonly listener: (key: string | null) => void }>();
  return {
    tab: () => {
      const self = {};
      const changed = (key: string): void => {
        for (const watcher of [...watchers]) if (watcher.tab !== self) queueMicrotask(() => watchers.has(watcher) && watcher.listener(key));
      };
      return {
        get: (key) => values.get(key) ?? null,
        set: (key, value) => {
          if (values.get(key) === value) return;
          values.set(key, value);
          changed(key);
        },
        remove: (key) => {
          if (values.delete(key)) changed(key);
        },
        keys: () => [...values.keys()],
        watch: (listener) => {
          const watcher = { tab: self, listener };
          watchers.add(watcher);
          return () => watchers.delete(watcher);
        },
      };
    },
  };
}
//#endregion 🗄️Area

//#region 💾️Store
/** 🗄️ The single-valued persisted local-only slices of one tenant. */
export type LocalSlice = "introduced" | "learner" | "catalog" | "learner-view" | "preferences";

/** 🗃️ The persisted local-only collections of one tenant, one storage key per record. */
export type LocalCollection = "outbox" | "runs";

/** 🔔️ A change another tab made: one slice, one record of a collection, or everything at once. */
export type LocalChange = { readonly kind: "slice"; readonly slice: LocalSlice } | { readonly kind: "record"; readonly collection: LocalCollection; readonly id: string } | { readonly kind: "cleared" };

const SLICES: readonly string[] = ["introduced", "learner", "catalog", "learner-view", "preferences"] satisfies readonly LocalSlice[];
const COLLECTIONS: readonly string[] = ["outbox", "runs"] satisfies readonly LocalCollection[];

/** 💾️ Typed JSON access to one tenant's persisted local-only slices and collections. */
export interface LocalStore {
  read(slice: LocalSlice): unknown;
  write(slice: LocalSlice, value: unknown): void;
  remove(slice: LocalSlice): void;
  records(collection: LocalCollection): ReadonlyMap<string, unknown>;
  record(collection: LocalCollection, id: string): unknown;
  put(collection: LocalCollection, id: string, value: unknown): void;
  drop(collection: LocalCollection, id: string): void;
  watch(listener: (change: LocalChange) => void): () => void;
}

function parsed(raw: string | null): unknown {
  if (raw === null) return undefined;
  try {
    return JSON.parse(raw) as unknown;
  } catch {
    return undefined;
  }
}

/** 🔑️ The change a key of another tab's write stands for, when it belongs to the store under `prefix`. */
export function localChange(key: string | null, prefix: string): LocalChange | undefined {
  if (key === null) return { kind: "cleared" };
  if (!key.startsWith(prefix)) return undefined;
  const rest = key.slice(prefix.length);
  const slash = rest.indexOf("/");
  if (slash < 0) return SLICES.includes(rest) ? { kind: "slice", slice: rest as LocalSlice } : undefined;
  const collection = rest.slice(0, slash);
  return COLLECTIONS.includes(collection) ? { kind: "record", collection: collection as LocalCollection, id: rest.slice(slash + 1) } : undefined;
}

/** 💾️ A {@link LocalStore} over `area`: slices at `semio.quiz.<tenant>.<slice>`, records at
 * `semio.quiz.<tenant>.<collection>/<id>`. */
export function localStore(area: StorageArea, tenant: string): LocalStore {
  const prefix = `semio.quiz.${tenant}.`;
  const recordKey = (collection: LocalCollection, id: string): string => `${prefix}${collection}/${id}`;
  return {
    read: (slice) => parsed(area.get(`${prefix}${slice}`)),
    write: (slice, value) => area.set(`${prefix}${slice}`, JSON.stringify(value)),
    remove: (slice) => area.remove(`${prefix}${slice}`),
    records: (collection) => {
      const start = `${prefix}${collection}/`;
      const found = new Map<string, unknown>();
      for (const key of area.keys()) {
        if (!key.startsWith(start)) continue;
        const value = parsed(area.get(key));
        if (value !== undefined) found.set(key.slice(start.length), value);
      }
      return found;
    },
    record: (collection, id) => parsed(area.get(recordKey(collection, id))),
    put: (collection, id, value) => area.set(recordKey(collection, id), JSON.stringify(value)),
    drop: (collection, id) => area.remove(recordKey(collection, id)),
    watch: (listener) =>
      area.watch((key) => {
        const change = localChange(key, prefix);
        if (change !== undefined) listener(change);
      }),
  };
}

/** 🧱️ Whether `value` is a plain JSON object. */
export function isRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** 🧗️ Whether a stored `value` is one of the {@link CHALLENGES}. */
export function isChallenge(value: unknown): value is Challenge {
  return (CHALLENGES as readonly unknown[]).includes(value);
}
//#endregion 💾️Store

//#region 🧾️Shapes
/** 📚️ Whether `value` — stored, or read from the proctor — has the shape of a catalog view this client renders. */
export function isCatalogView(value: unknown): value is CatalogView {
  return isRecord(value) && typeof value.id === "string" && Array.isArray(value.quizzes) && Array.isArray(value.badges);
}

function isBest(value: unknown): boolean {
  return isRecord(value) && isChallenge(value.challenge) && typeof value.score === "number" && typeof value.points === "number";
}

/** 🧑‍🎓️ Whether `value` has the shape of a learner view of this contract: every run listed with its challenge and every
 * best run with its challenge, score and points. */
export function isLearnerView(value: unknown): value is LearnerView {
  if (!isRecord(value) || typeof value.learner !== "string" || !Array.isArray(value.runs) || !Array.isArray(value.badges) || !isRecord(value.best)) return false;
  return value.runs.every((summary) => isRecord(summary) && typeof summary.run === "string" && isChallenge(summary.challenge)) && Object.values(value.best).every(isBest);
}

/** 🏃️ Whether `value` has the shape of a run view of this contract — of `run` when it names one: a sheet at one of the
 * challenges with its tasks, the answers and a status. */
export function isRunView(value: unknown, run?: Id): value is RunView {
  return isRecord(value) && typeof value.run === "string" && (run === undefined || value.run === run) && typeof value.learner === "string" && isRecord(value.sheet) && isChallenge(value.sheet.challenge) && Array.isArray(value.sheet.tasks) && isRecord(value.answers) && typeof value.status === "string";
}

export { isTimestamp } from "@semio-tech/quiz";
//#endregion 🧾️Shapes
