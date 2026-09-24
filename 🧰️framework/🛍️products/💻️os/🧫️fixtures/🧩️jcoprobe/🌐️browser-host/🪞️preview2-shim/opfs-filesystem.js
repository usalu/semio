import { InMemoryFilesystemAdapter, UNWRAP_DESCRIPTOR, _onTouch } from "./in-memory-filesystem.js";
// OPFS has no native symlink concept. All symlinks under a root are recorded in a
// single hidden sidecar file at that root, keyed by their path relative to it, so
// they survive a flush + reload instead of only living for the lifetime of one
// capability.
const SYMLINKS_FILE = ".__wasi_symlinks__.json";
async function readSymlinksFile(handle) {
    try {
        const fileHandle = await handle.getFileHandle(SYMLINKS_FILE);
        const file = await fileHandle.getFile();
        return JSON.parse(await file.text());
    }
    catch {
        return {};
    }
}
async function writeSymlinksFile(handle, symlinks) {
    if (Object.keys(symlinks).length === 0) {
        try {
            await handle.removeEntry(SYMLINKS_FILE);
        }
        catch {
            // no sidecar to remove
        }
        return;
    }
    const fileHandle = await handle.getFileHandle(SYMLINKS_FILE, { create: true });
    const writable = await fileHandle.createWritable();
    await writable.write(JSON.stringify(symlinks));
    await writable.close();
}
/** Collect every symlink under `dir` into a flat path -> target map, relative to `dir` itself. */
function collectSymlinks(dir, prefix, out) {
    for (const [name, entry] of Object.entries(dir)) {
        const path = prefix ? `${prefix}/${name}` : name;
        if (entry.symlink !== undefined) {
            out[path] = entry.symlink;
        }
        else if (entry.dir) {
            collectSymlinks(entry.dir, path, out);
        }
    }
}
/** Place each recorded symlink back into the tree at its path, skipping any path whose parent is missing. */
function injectSymlinks(root, symlinks) {
    for (const [path, target] of Object.entries(symlinks)) {
        const parts = path.split("/").filter(Boolean);
        const name = parts.pop();
        if (!name) {
            continue;
        }
        let dir = root;
        for (const part of parts) {
            dir = dir[part]?.dir;
            if (!dir) {
                break;
            }
        }
        if (dir && !(name in dir)) {
            dir[name] = { symlink: target };
        }
    }
}
async function readEntries(handle) {
    const dir = {};
    for await (const [name, child] of handle.entries()) {
        if (name === SYMLINKS_FILE) {
            continue;
        }
        if (child.kind === "directory") {
            dir[name] = { dir: await readEntries(child) };
        }
        else {
            const file = await child.getFile();
            dir[name] = { source: new Uint8Array(await file.arrayBuffer()) };
        }
    }
    return dir;
}
async function writeEntries(handle, dir) {
    const seen = new Set(Object.keys(dir));
    seen.add(SYMLINKS_FILE);
    for await (const [name] of handle.entries()) {
        if (!seen.has(name)) {
            await handle.removeEntry(name, { recursive: true });
        }
    }
    for (const [name, entry] of Object.entries(dir)) {
        if (entry.symlink !== undefined) {
            // Recorded separately in the root's symlink sidecar file.
            continue;
        }
        if (entry.dir) {
            await writeEntries(await handle.getDirectoryHandle(name, { create: true }), entry.dir);
            continue;
        }
        const fileHandle = await handle.getFileHandle(name, { create: true });
        const writable = await fileHandle.createWritable();
        const source = entry.source ?? new Uint8Array();
        const bytes = typeof source === "string" ? new TextEncoder().encode(source) : source;
        await writable.write(bytes.slice());
        await writable.close();
    }
}
/** Load an OPFS directory into an in-memory tree, ready to hand to `OpfsFilesystemAdapter.getRoot`. */
export async function loadOpfsCapability(handle) {
    const dir = await readEntries(handle);
    injectSymlinks(dir, await readSymlinksFile(handle));
    return { handle, data: { dir } };
}
const LOCK_METHODS = new Set([
    "lockShared",
    "lockExclusive",
    "tryLockShared",
    "tryLockExclusive",
    "unlock",
]);
function lockModeOf(prop) {
    return prop.endsWith("Exclusive") ? "exclusive" : "shared";
}
/** Join an OPFS-relative path onto an existing one, for naming a cross-tab lock. */
function joinLockPath(base, segment) {
    let path = base;
    for (const part of segment.split("/")) {
        if (part === "" || part === ".") {
            continue;
        }
        if (part === "..") {
            path = path.slice(0, path.lastIndexOf("/"));
            continue;
        }
        path = path ? `${path}/${part}` : part;
    }
    return path;
}
/**
 * Wrap a descriptor so its advisory locking also makes a best-effort request against
 * `BrowserLockManager`, in addition to the default same-process reader/writer lock.
 * The Web Locks request is fire-and-forget - there's no way to block a synchronous
 * `lockExclusive()` on a cross-tab grant without JSPI, so this only coordinates
 * tabs that are already idle/cooperating.
 */
function withCrossTabLocking(descriptor, lockName, lockManager) {
    let release = null;
    function requestCrossTabLock(mode) {
        if (!lockManager) {
            return;
        }
        lockManager
            .request(lockName, { mode }, () => new Promise((resolve) => (release = resolve)))
            .catch(() => {
            // best-effort only; local same-process locking already enforced correctness
        });
    }
    function releaseCrossTabLock() {
        release?.();
        release = null;
    }
    return new Proxy(descriptor, {
        get(target, prop, receiver) {
            if (prop === UNWRAP_DESCRIPTOR) {
                return target;
            }
            const value = Reflect.get(target, prop, receiver);
            if (prop === "openAt") {
                return (...args) => {
                    const child = Reflect.apply(value, target, args);
                    return withCrossTabLocking(child, joinLockPath(lockName, args[1]), lockManager);
                };
            }
            if (typeof prop === "string" && LOCK_METHODS.has(prop) && typeof value === "function") {
                return (...args) => {
                    const result = Reflect.apply(value, target, args);
                    if (prop === "unlock") {
                        releaseCrossTabLock();
                    }
                    else if (prop.startsWith("tryLock")) {
                        if (result) {
                            requestCrossTabLock(lockModeOf(prop));
                        }
                    }
                    else {
                        requestCrossTabLock(lockModeOf(prop));
                    }
                    return result;
                };
            }
            return typeof value === "function" ? value.bind(target) : value;
        },
    });
}
/**
 * Browser filesystem adapter backed by the Origin Private File System.
 *
 * Every guest-facing `Descriptor` operation - including advisory locking, which
 * comes for free as `InMemoryFilesystemAdapter`'s same-process reader/writer lock -
 * is delegated to `InMemoryFilesystemAdapter` and stays synchronous. Persistence to
 * OPFS happens automatically shortly after any mutation (debounced to a microtask,
 * so a burst of writes only triggers one round-trip); `flush()`/`dispose()` remain
 * available to force it explicitly, e.g. before navigating away.
 *
 * @see https://developer.mozilla.org/en-US/docs/Web/API/File_System_API/Origin_private_file_system
 */
export class OpfsFilesystemAdapter {
    #inMemory = new InMemoryFilesystemAdapter();
    #roots = [];
    #lockManager;
    #flushScheduled = false;
    #unsubscribeTouch;
    constructor(options = {}) {
        this.#lockManager = options.lockManager;
        this.#unsubscribeTouch = _onTouch(() => this.#scheduleFlush());
    }
    getRoot(capability) {
        this.#roots.push(capability);
        const descriptor = this.#inMemory.getRoot(capability.data);
        return this.#lockManager
            ? withCrossTabLocking(descriptor, capability.handle.name, this.#lockManager)
            : descriptor;
    }
    #scheduleFlush() {
        if (this.#flushScheduled) {
            return;
        }
        this.#flushScheduled = true;
        queueMicrotask(() => {
            this.#flushScheduled = false;
            void this.flush();
        });
    }
    /** Persist every loaded root's current in-memory state back to OPFS. */
    async flush() {
        await Promise.all(this.#roots.map(async (root) => {
            const dir = root.data.dir ?? {};
            await writeEntries(root.handle, dir);
            const symlinks = {};
            collectSymlinks(dir, "", symlinks);
            await writeSymlinksFile(root.handle, symlinks);
        }));
    }
    dispose() {
        this.#unsubscribeTouch();
        void this.flush();
    }
}
