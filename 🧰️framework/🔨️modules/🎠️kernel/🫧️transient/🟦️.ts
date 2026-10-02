/** 🫧 Process-local box for module ephemeral values. */
export type EphemeralBox<T> = { current: T };

/** 🫧️ Owned authority for ephemeral local-only state. It deliberately has no storage,
 * serialization, history, sync, or undo surface; a shell/runtime may own an isolated instance while
 * module-level helpers share {@link defaultTransientStore}. */
export class TransientStore {
  private readonly boxes = new Map<string, EphemeralBox<unknown>>();
  private readonly maps = new Map<string, Map<unknown, unknown>>();
  private readonly sets = new Map<string, Set<unknown>>();
  private readonly weakMaps = new Map<string, WeakMap<object, unknown>>();

  box<T>(key: string, init: T): EphemeralBox<T> {
    let box = this.boxes.get(key) as EphemeralBox<T> | undefined;
    if (!box) {
      box = { current: init };
      this.boxes.set(key, box as EphemeralBox<unknown>);
    }
    return box;
  }

  map<K, V>(key: string): Map<K, V> {
    let map = this.maps.get(key) as Map<K, V> | undefined;
    if (!map) {
      map = new Map();
      this.maps.set(key, map as Map<unknown, unknown>);
    }
    return map;
  }

  set<T>(key: string): Set<T> {
    let set = this.sets.get(key) as Set<T> | undefined;
    if (!set) {
      set = new Set();
      this.sets.set(key, set as Set<unknown>);
    }
    return set;
  }

  weakMap<K extends object, V>(key: string): WeakMap<K, V> {
    let map = this.weakMaps.get(key) as WeakMap<K, V> | undefined;
    if (!map) {
      map = new WeakMap();
      this.weakMaps.set(key, map as WeakMap<object, unknown>);
    }
    return map;
  }

  /** 🧹️ Drops every local transient allocation owned by this runtime. Existing references remain
   * valid but are no longer returned by subsequent lookups, matching a shell/session teardown. */
  reset(): void {
    this.boxes.clear();
    this.maps.clear();
    this.sets.clear();
    this.weakMaps.clear();
  }
}

export const defaultTransientStore = new TransientStore();

/** 🫧 Get-or-create a mutable box keyed for a transient draft snapshot.
 * Init is stored as-is — never treat a function-typed `T` as a lazy factory (that would
 * invoke identity/no-op resolvers and leave `.current` undefined). */
export function ephemeralBox<T>(key: string, init: T): EphemeralBox<T> {
  return defaultTransientStore.box(key, init);
}

/** 🫧 Get-or-create a process-local Map owned by the ephemeral lane. */
export function ephemeralMap<K, V>(key: string): Map<K, V> {
  return defaultTransientStore.map(key);
}

/** 🫧 Get-or-create a process-local Set owned by the ephemeral lane. */
export function ephemeralSet<T>(key: string): Set<T> {
  return defaultTransientStore.set(key);
}

/** 🫧 Get-or-create a process-local WeakMap owned by the ephemeral lane. */
export function ephemeralWeakMap<K extends object, V>(key: string): WeakMap<K, V> {
  return defaultTransientStore.weakMap(key);
}
