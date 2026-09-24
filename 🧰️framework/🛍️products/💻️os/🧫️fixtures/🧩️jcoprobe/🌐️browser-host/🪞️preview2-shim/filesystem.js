import { environment } from "./environment.js";
import { InMemoryFilesystemAdapter } from "./in-memory-filesystem.js";
import { _setCwd } from "./config.js";
export { _setCwd } from "./config.js";
export { InMemoryFilesystemAdapter } from "./in-memory-filesystem.js";
export { OpfsFilesystemAdapter, loadOpfsCapability } from "./opfs-filesystem.js";
class DirectoryEntryStream {
    #implementation;
    static _create(implementation) {
        const stream = new DirectoryEntryStream();
        stream.#implementation = implementation;
        return stream;
    }
    readDirectoryEntry() {
        return this.#implementation.readDirectoryEntry();
    }
}
const directoryEntryStreamCreate = DirectoryEntryStream._create;
// @ts-expect-error - Deleting static method
delete DirectoryEntryStream._create;
class Descriptor {
    #implementation;
    _getImplementation(descriptor) {
        return descriptor.#implementation;
    }
    static _create(implementation) {
        const descriptor = new Descriptor();
        descriptor.#implementation = implementation;
        return descriptor;
    }
    readViaStream(offset) {
        return this.#implementation.readViaStream(offset);
    }
    writeViaStream(offset) {
        return this.#implementation.writeViaStream(offset);
    }
    appendViaStream() {
        return this.#implementation.appendViaStream();
    }
    advise(offset, length, advice) {
        return this.#implementation.advise(offset, length, advice);
    }
    syncData() {
        return this.#implementation.syncData();
    }
    getFlags() {
        return this.#implementation.getFlags();
    }
    getType() {
        return this.#implementation.getType();
    }
    setSize(size) {
        return this.#implementation.setSize(size);
    }
    setTimes(dataAccessTimestamp, dataModificationTimestamp) {
        return this.#implementation.setTimes(dataAccessTimestamp, dataModificationTimestamp);
    }
    read(length, offset) {
        return this.#implementation.read(length, offset);
    }
    write(buffer, offset) {
        return this.#implementation.write(buffer, offset);
    }
    readDirectory() {
        return directoryEntryStreamCreate(this.#implementation.readDirectory());
    }
    sync() {
        return this.#implementation.sync();
    }
    createDirectoryAt(path) {
        return this.#implementation.createDirectoryAt(path);
    }
    stat() {
        return this.#implementation.stat();
    }
    statAt(pathFlags, path) {
        return this.#implementation.statAt(pathFlags, path);
    }
    setTimesAt(pathFlags, path, dataAccessTimestamp, dataModificationTimestamp) {
        return this.#implementation.setTimesAt(pathFlags, path, dataAccessTimestamp, dataModificationTimestamp);
    }
    linkAt(oldPathFlags, oldPath, newDescriptor, newPath) {
        return this.#implementation.linkAt(oldPathFlags, oldPath, descriptorGetImplementation(newDescriptor), newPath);
    }
    openAt(pathFlags, path, openFlags, flags) {
        return descriptorCreate(this.#implementation.openAt(pathFlags, path, openFlags, flags));
    }
    readlinkAt(path) {
        return this.#implementation.readlinkAt(path);
    }
    removeDirectoryAt(path) {
        return this.#implementation.removeDirectoryAt(path);
    }
    renameAt(oldPath, newDescriptor, newPath) {
        return this.#implementation.renameAt(oldPath, descriptorGetImplementation(newDescriptor), newPath);
    }
    symlinkAt(oldPath, newPath) {
        return this.#implementation.symlinkAt(oldPath, newPath);
    }
    unlinkFileAt(path) {
        return this.#implementation.unlinkFileAt(path);
    }
    isSameObject(other) {
        return this.#implementation.isSameObject(descriptorGetImplementation(other));
    }
    metadataHash() {
        return this.#implementation.metadataHash();
    }
    metadataHashAt(pathFlags, path) {
        return this.#implementation.metadataHashAt(pathFlags, path);
    }
    lockShared() {
        return this.#implementation.lockShared?.();
    }
    lockExclusive() {
        return this.#implementation.lockExclusive?.();
    }
    tryLockShared() {
        return this.#implementation.tryLockShared?.() ?? false;
    }
    tryLockExclusive() {
        return this.#implementation.tryLockExclusive?.() ?? false;
    }
    unlock() {
        return this.#implementation.unlock?.();
    }
}
const descriptorGetImplementation = Descriptor.prototype._getImplementation;
// @ts-expect-error - Deleting prototype method
delete Descriptor.prototype._getImplementation;
const descriptorCreate = Descriptor._create;
// @ts-expect-error - Deleting static method
delete Descriptor._create;
const defaultAdapter = new InMemoryFilesystemAdapter();
let _fileData = { dir: {} };
let _preopens = [];
let _rootPreopen = null;
export const preopens = {
    getDirectories() {
        return _preopens;
    },
};
/** Create isolated filesystem namespaces backed by an application-selected adapter. */
export function createFilesystem({ adapter, preopens: configuredPreopens, }) {
    const entries = Object.entries(configuredPreopens).map(([guestPath, capability]) => [descriptorCreate(adapter.getRoot(capability)), guestPath]);
    let disposed = false;
    return {
        types,
        preopens: {
            getDirectories() {
                if (disposed) {
                    throw new Error("filesystem adapter has been disposed");
                }
                return [...entries];
            },
        },
        dispose() {
            if (disposed) {
                return;
            }
            disposed = true;
            adapter.dispose?.();
        },
    };
}
export function _setFileData(fileData) {
    _fileData = fileData;
    if (_rootPreopen) {
        _rootPreopen[0] = descriptorCreate(defaultAdapter.getRoot(fileData));
    }
    else {
        _setPreopens({ "/": fileData });
    }
    const cwd = environment.initialCwd();
    _setCwd(cwd || "/");
}
export function _getFileData() {
    return JSON.stringify(_fileData);
}
/**
 * Replace all preopens with the given set.
 * @param preopensConfig - Map of virtual paths to file data entries
 */
export function _setPreopens(preopensConfig) {
    _preopens = [];
    _rootPreopen = null;
    for (const [virtualPath, fileData] of Object.entries(preopensConfig)) {
        _addPreopen(virtualPath, fileData);
    }
}
/**
 * Add a single preopen mapping.
 * @param virtualPath - The virtual path visible to the guest
 * @param fileData - The file data object representing the directory
 */
export function _addPreopen(virtualPath, fileData) {
    const descriptor = descriptorCreate(defaultAdapter.getRoot(fileData));
    const entry = [descriptor, virtualPath];
    _preopens.push(entry);
    if (virtualPath === "/") {
        _rootPreopen = entry;
    }
}
/**
 * Add a single preopen backed by a custom adapter (e.g. `OpfsFilesystemAdapter`) instead of the
 * default in-memory one. Lets a host wire an alternative `BrowserFilesystemAdapter` into the
 * top-level `wasi:filesystem/preopens` singleton that transpiled components import statically.
 * @param virtualPath - The virtual path visible to the guest
 * @param adapter - The adapter that will back this preopen
 * @param capability - The adapter-specific capability to load as the preopen's root
 */
export function _addPreopenWithAdapter(virtualPath, adapter, capability) {
    const descriptor = descriptorCreate(adapter.getRoot(capability));
    const entry = [descriptor, virtualPath];
    _preopens.push(entry);
    if (virtualPath === "/") {
        _rootPreopen = entry;
    }
}
/** Clear all preopens, giving the guest no filesystem access. */
export function _clearPreopens() {
    _preopens = [];
    _rootPreopen = null;
}
/** Get current preopens configuration. */
export function _getPreopens() {
    return [..._preopens];
}
/** Reject host paths because browser filesystems require explicit capabilities. */
export function _createPreopenDescriptor(hostPreopen) {
    throw new TypeError(`browser preopen ${JSON.stringify(hostPreopen)} is a host path; configure browser file data or an adapter instead`);
}
export const types = {
    Descriptor,
    DirectoryEntryStream,
    filesystemErrorCode: (err) => {
        let message;
        if ("payload" in err) {
            message = err.payload;
        }
        else if ("message" in err) {
            message = err.message;
        }
        return convertFsError(message);
    },
};
function convertFsError(e) {
    switch (e.code) {
        case "EACCES":
            return "access";
        case "EAGAIN":
        case "EWOULDBLOCK":
            return "would-block";
        case "EALREADY":
            return "already";
        case "EBADF":
            return "bad-descriptor";
        case "EBUSY":
            return "busy";
        case "EDEADLK":
            return "deadlock";
        case "EDQUOT":
            return "quota";
        case "EEXIST":
            return "exist";
        case "EFBIG":
            return "file-too-large";
        case "EILSEQ":
            return "illegal-byte-sequence";
        case "EINPROGRESS":
            return "in-progress";
        case "EINTR":
            return "interrupted";
        case "EINVAL":
            return "invalid";
        case "EIO":
            return "io";
        case "EISDIR":
            return "is-directory";
        case "ELOOP":
            return "loop";
        case "EMLINK":
            return "too-many-links";
        case "EMSGSIZE":
            return "message-size";
        case "ENAMETOOLONG":
            return "name-too-long";
        case "ENODEV":
            return "no-device";
        case "ENOENT":
            return "no-entry";
        case "ENOLCK":
            return "no-lock";
        case "ENOMEM":
            return "insufficient-memory";
        case "ENOSPC":
            return "insufficient-space";
        case "ENOTDIR":
        case "ERR_FS_EISDIR":
            return "not-directory";
        case "ENOTEMPTY":
            return "not-empty";
        case "ENOTRECOVERABLE":
            return "not-recoverable";
        case "ENOTSUP":
            return "unsupported";
        case "ENOTTY":
            return "no-tty";
        // windows gives this error for badly structured `//` reads
        // this seems like a slightly better error than unknown given
        // that it's a common footgun
        case -4094:
        case "ENXIO":
            return "no-such-device";
        case "EOVERFLOW":
            return "overflow";
        case "EPERM":
            return "not-permitted";
        case "EPIPE":
            return "pipe";
        case "EROFS":
            return "read-only";
        case "ESPIPE":
            return "invalid-seek";
        case "ETXTBSY":
            return "text-file-busy";
        case "EXDEV":
            return "cross-device";
        case "UNKNOWN":
            switch (e.errno) {
                case -4094:
                    return "no-such-device";
                default:
                    throw e;
            }
        default:
            throw e;
    }
}
