import { open, lstat, realpath } from "node:fs/promises";
import { isAbsolute } from "node:path";
import { createHash } from "node:crypto";
import { advanceScriptInvocation, checkScriptInvocation, type ScriptInvocation } from "../../../../🧭️routing/📥️invocation/🟦️.ts";

/** 🎛️ Retains the original caller's control while admitting awaited hashing progress. */
export type ExactCargoFingerprintControl = Readonly<{ invocation: ScriptInvocation; cancelled?: () => boolean; progress?: (completed: number, total: number) => void | Promise<void> }>;

/** 🔬️ Hashes bounded asynchronous reads through original cancellation, deadline, progress and continuation. */
export async function controlledExactExecutableFingerprint(path: string, control: ExactCargoFingerprintControl): Promise<{ path: string; sha256: string; byteLength: number }> {
  const check = (): void => {
    if (control.cancelled?.()) throw Error("Executable fingerprint cancelled");
    checkScriptInvocation(control.invocation);
  };
  check();
  if (!isAbsolute(path) || (await lstat(path)).isSymbolicLink()) throw Error("Executable must be one absolute regular file");
  check();
  const canonical = await realpath(path);
  check();
  const descriptor = await open(canonical, "r");
  try {
    check();
    const before = await descriptor.stat();
    const same = (other: typeof before): boolean => other.isFile() && !other.isSymbolicLink() && other.dev === before.dev && other.ino === before.ino && other.size === before.size && other.mtimeMs === before.mtimeMs && other.ctimeMs === before.ctimeMs;
    if (!before.isFile() || !same(await lstat(canonical)) || before.size <= 0 || before.size > 8 * 1024 ** 3 || (process.platform !== "win32" && (before.mode & 0o111) === 0)) throw Error("Executable size or type denied");
    const digest = createHash("sha256"), buffer = Buffer.alloc(64 * 1024);
    let count = 0, published = 0;
    while (count < before.size) {
      check();
      const { bytesRead } = await descriptor.read(buffer, 0, buffer.length, null);
      check();
      if (bytesRead === 0) throw Error("Executable changed while hashing");
      digest.update(buffer.subarray(0, bytesRead));
      count += bytesRead;
      if (count > before.size) throw Error("Executable changed while hashing");
      if (published === 0 || count - published >= 4 * 1024 ** 2 || count === before.size) {
        await control.progress?.(count, before.size);
        check();
        await advanceScriptInvocation(control.invocation, `exact-cargo:fingerprint:${count}/${before.size}`, "running");
        check();
        published = count;
      }
    }
    const after = await descriptor.stat();
    check();
    if (!same(after) || !same(await lstat(canonical))) throw Error("Executable changed while hashing");
    check();
    return { path: canonical, sha256: digest.digest("hex"), byteLength: count };
  } finally { await descriptor.close(); }
}
