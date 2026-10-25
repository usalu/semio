/** 🗄️ Device-local key-value storage port with browser, in-memory and namespaced implementations. */

/** 💾️ Synchronous string storage a shell persists its device-local configuration through. */
export interface StoragePort {
  get(key: string): string | null;
  set(key: string, value: string): void;
  remove(key: string): void;
}

/** 🌐️ {@link StoragePort} over `localStorage`; a blocked or absent storage degrades to an empty one. */
export function createBrowserStoragePort(): StoragePort {
  return {
    get: (key) => {
      try {
        return typeof localStorage !== "undefined" ? localStorage.getItem(key) : null;
      } catch {
        return null;
      }
    },
    set: (key, value) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.setItem(key, value);
      } catch {}
    },
    remove: (key) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.removeItem(key);
      } catch {}
    },
  };
}

/** 🧠️ In-memory {@link StoragePort} — used by ephemeral branded shells so nothing survives a window refresh. */
export function createMemoryStoragePort(): StoragePort {
  const map = new Map<string, string>();
  return {
    get: (key) => map.get(key) ?? null,
    set: (key, value) => {
      map.set(key, value);
    },
    remove: (key) => {
      map.delete(key);
    },
  };
}

/** 🐚️ Namespaces a {@link StoragePort} under `semio.shell.<namespace>.` so several shell instances sharing one
 * browser storage origin (e.g. several demonstrator panes) don't read/write each other's `semio.os.dock`/`ui.chrome.*`
 * keys. Not needed for a single page-owning shell — that shell's default (unprefixed) storage is the intended shared surface. */
export function createScopedStoragePort(base: StoragePort, namespace: string): StoragePort {
  const prefix = `semio.shell.${namespace}.`;
  return {
    get: (key) => base.get(`${prefix}${key}`),
    set: (key, value) => base.set(`${prefix}${key}`, value),
    remove: (key) => base.remove(`${prefix}${key}`),
  };
}
