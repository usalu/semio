import { createHash, randomUUID } from "node:crypto";
import { chmodSync, linkSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { ownProcessIdentityV1, processRecordAliveV1 } from "../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import { readBuildOwnerRegistryV1 } from "../../🚪️admission/🟦️.ts";

/** 🏁️ The race a compare-and-swap detects: the registry file changed between the read of a writer and its rename, so the writer computes again on the new file. */
export class OwnerRegistryRaceV1 extends Error {}

/** 🪝️ Options of the shared writer: the rename that publishes the staged file (a test seam) and the permission mode the published file takes (default: the process default). */
export type OwnerRegistryWriteHooksV1 = Readonly<{ rename?: (from: string, to: string) => void; mode?: number }>;

const ownerRegistryLockPathV1 = (file: string): string => join(tmpdir(), `semio-build-owners-${createHash("sha256").update(file).digest("hex").slice(0, 16)}.lock`);

const ownerRegistryLockHolderAliveV1 = (pid: number, startMs?: number): boolean => processRecordAliveV1(pid, startMs);

/** 🔒️ Runs `run` as the only writer of the registry file on this machine: an atomic lock directory in the temporary directory (never inside the replicated work tree) names its holder, a lock whose holder ended, or whose holder is unreadable and that is older than `staleMilliseconds`, is taken over by an atomic rename (a live holder keeps its lock however long it runs, so two writers never overlap), and a writer that waits longer than `waitMilliseconds` refuses instead of hanging. */
export function withOwnerRegistryLockV1<T>(file: string, run: () => T, options: { staleMilliseconds?: number; waitMilliseconds?: number } = {}): T {
  const lock = ownerRegistryLockPathV1(file), stale = options.staleMilliseconds ?? 60_000, wait = options.waitMilliseconds ?? 30_000, began = Date.now();
  for (;;) {
    try { mkdirSync(lock); writeFileSync(join(lock, "owner"), `${process.pid} ${Date.now()} ${ownProcessIdentityV1().startMs}\n`); break; } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") { rmSync(lock, { recursive: true, force: true }); throw error; }
    }
    const facts = statSync(lock, { throwIfNoEntry: false });
    if (facts !== undefined) {
      let pid = Number.NaN, startMs: number | undefined;
      try { const parts = readFileSync(join(lock, "owner"), "utf8").trim().split(" "); pid = Number(parts[0]); startMs = parts[2] === undefined ? undefined : Number(parts[2]); } catch {}
      const known = Number.isInteger(pid);
      if (known ? !ownerRegistryLockHolderAliveV1(pid, startMs) : Date.now() - facts.mtimeMs > stale) {
        try { renameSync(lock, `${lock}.${randomUUID()}.stale`); } catch {}
        for (const name of readdirSync(tmpdir()).filter((entry) => entry.startsWith(`${basename(lock)}.`) && entry.endsWith(".stale"))) rmSync(join(tmpdir(), name), { recursive: true, force: true });
        continue;
      }
    }
    if (Date.now() - began > wait) throw new Error(`Build owner registry is locked by another writer: ${lock}`);
    Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 25);
  }
  try { return run(); } finally { rmSync(lock, { recursive: true, force: true }); }
}

/** 🛡️ Stages `text` next to `file` (same directory, so the final step is one atomic rename or link), refuses it unless `readBuildOwnerRegistryV1` accepts it, runs `publish` on the staged path and always removes the staged file; the registry file is never opened for writing, so a refusal or a crash leaves the old file byte for byte. */
function stageOwnerRegistryV1<T>(file: string, text: string, publish: (pending: string) => T, mode?: number): T {
  readBuildOwnerRegistryV1(JSON.parse(text));
  const pending = `${file}.${randomUUID()}.tmp`;
  try {
    writeFileSync(pending, text, { flag: "wx" });
    if (mode !== undefined) chmodSync(pending, mode);
    return publish(pending);
  } finally {
    rmSync(pending, { force: true });
  }
}

/** 🧷️ Replaces the registry file with `text` when it still holds `source`: validated first, written beside it, compared with the current content and renamed; a file that changed meanwhile raises {@link OwnerRegistryRaceV1}. */
export function publishOwnerRegistryTextV1(file: string, source: string, text: string, hooks: OwnerRegistryWriteHooksV1 = {}): void {
  stageOwnerRegistryV1(file, text, (pending) => {
    if (readFileSync(file, "utf8") !== source) throw new OwnerRegistryRaceV1("Build owner registry changed during reconciliation");
    (hooks.rename ?? renameSync)(pending, file);
  }, hooks.mode);
}

/** 🌱️ Creates the registry file with `text` only when none exists (validated, staged, then hard-linked into place so a concurrent creator wins whole or not at all) and tells whether it was created. */
export function createOwnerRegistryTextV1(file: string, text: string): boolean {
  return stageOwnerRegistryV1(file, text, (pending) => {
    try { linkSync(pending, file); return true; } catch (error) { if ((error as NodeJS.ErrnoException).code === "EEXIST") return false; throw error; }
  });
}

/** ✍️ The one writer of `📋️build-owners.json`: under the registry lock it reads the file, lets `compute` derive the new text from exactly that content (no text means nothing to write), validates it, publishes it atomically only if the file is still the content that was read and otherwise computes again on the new content, so concurrent adders never lose or duplicate a row and an invalid registry is never published. */
export function updateBuildOwnerRegistryV1<T>(file: string, compute: (source: string) => { value: T; text?: string }, hooks: OwnerRegistryWriteHooksV1 = {}): T {
  return withOwnerRegistryLockV1(file, () => {
    for (let attempt = 0; ; attempt++) {
      const source = readFileSync(file, "utf8");
      const { value, text } = compute(source);
      if (text === undefined) return value;
      try { publishOwnerRegistryTextV1(file, source, text, hooks); return value; } catch (error) { if (!(error instanceof OwnerRegistryRaceV1) || attempt >= 3) throw error; }
    }
  });
}
