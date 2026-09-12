import { existsSync, lstatSync, readFileSync, mkdirSync, writeFileSync, rmSync, mkdtempSync, chmodSync, renameSync } from "node:fs";
import { copyFile } from "node:fs/promises";
import { dirname, resolve, join } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { acquireResourceLease, type LeaseWait } from "../🔒️leases/🟦️.ts";

export type ArtifactPublicationOptions = { readonly signal?: AbortSignal; readonly leaseDirectory?: string; readonly onWait?: (progress: LeaseWait) => void };

/** 🧱️ Replaces only a previously owned deliverable tree and restores it if publication fails. */
export async function stageArtifacts(staging: string, owner: string, files: ReadonlyMap<string, string>, options: ArtifactPublicationOptions = {}): Promise<void> {
  const signal = options.signal ?? new AbortController().signal;
  const lease = await acquireResourceLease({ directory: options.leaseDirectory ?? join(getWorkspaceRoot(), ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), resource: `artifact:${resolve(staging)}`, mode: "exclusive", signal, onWait: options.onWait });
  let temporary: string | undefined;
  try {
    const marker = ".nx-artifact.json";
    for (let parent = resolve(staging); dirname(parent) !== parent; parent = dirname(parent)) if (existsSync(parent) && lstatSync(parent).isSymbolicLink()) throw new Error(`Symlink artifact destination: ${parent}`);
    if (existsSync(staging) && (!existsSync(join(staging, marker)) || JSON.parse(readFileSync(join(staging, marker), "utf8")).owner !== owner)) throw new Error(`Unowned artifact directory: ${staging}`);
    mkdirSync(dirname(staging), { recursive: true });
    temporary = mkdtempSync(`${staging}.stage-`);
    const previous = `${temporary}.previous`;
    for (const [name, source] of files) {
      signal.throwIfAborted();
      if (name.startsWith("/") || name.split(/[\\/]/).includes("..")) throw new Error(`Invalid artifact path ${name}`);
      const destination = join(temporary, name);
      mkdirSync(dirname(destination), { recursive: true });
      await copyFile(source, destination);
      chmodSync(destination, lstatSync(source).mode & 0o777);
    }
    writeFileSync(join(temporary, marker), JSON.stringify({ version: 1, owner, files: [...files.keys()].sort() }) + "\n");
    signal.throwIfAborted();
    if (existsSync(staging)) renameSync(staging, previous);
    try { renameSync(temporary, staging); }
    catch (error) { if (existsSync(previous)) renameSync(previous, staging); throw error; }
    rmSync(previous, { recursive: true, force: true });
  } finally { try { if (temporary) rmSync(temporary, { recursive: true, force: true }); } finally { lease.release(); } }
}
