import { closeSync, fstatSync, lstatSync, openSync, readSync } from "node:fs";
import { concurrentEditV1, retryOnConcurrentEditSyncV1 } from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

/** 🧊️ Captures one stable regular file through its retained descriptor and a shared byte admission; a file another process edits meanwhile is captured again (the `read` layer of the churn policy), against the admission it started with.
 * @see ../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json */
export function readStableBuildFile(path: string, maximum: number, admission: { remaining: number }, check: () => void): Uint8Array {
  const remaining = admission.remaining;
  return retryOnConcurrentEditSyncV1("read", () => { admission.remaining = remaining; return captureStableBuildFile(path, maximum, admission, check); });
}

function captureStableBuildFile(path: string, maximum: number, admission: { remaining: number }, check: () => void): Uint8Array {
  check();
  const info = lstatSync(path);
  if (!info.isFile() || info.isSymbolicLink()) throw new Error(`build input: not a regular file (${path})`);
  if (info.size > maximum) throw new Error(`build input: ${path} is ${info.size} bytes, over the ${maximum}-byte bound`);
  const file = openSync(path, "r");
  try {
    const before = fstatSync(file);
    const same = (left: typeof info, right: typeof info) => left.isFile() && !left.isSymbolicLink() && left.dev === right.dev && left.ino === right.ino && left.size === right.size && left.mtimeMs === right.mtimeMs && left.ctimeMs === right.ctimeMs;
    if (!same(before, info)) throw concurrentEditV1("identity", path, "build input: file changed");
    if (before.size > maximum || before.size > admission.remaining) throw new Error("build input: file exceeds aggregate bound");
    admission.remaining -= before.size;
    const bytes = Buffer.alloc(before.size);
    for (let offset = 0; offset < bytes.byteLength;) {
      check();
      const count = readSync(file, bytes, offset, Math.min(64 * 1024, bytes.byteLength - offset), offset);
      if (!count) throw concurrentEditV1("length", path, "build input: file shortened");
      offset += count;
    }
    if (!same(fstatSync(file), before) || !same(lstatSync(path), before)) throw concurrentEditV1("identity", path, "build input: file changed while reading");
    return bytes;
  } finally {
    closeSync(file);
  }
}
