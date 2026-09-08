import { lstat, readdir } from "node:fs/promises";
import { join, relative } from "node:path";

/** 🗂️ Collects regular staged files without following links or retaining compiler directory state. */
export async function collectArtifactFiles(root: string, signal?: AbortSignal): Promise<ReadonlyMap<string, string>> {
  signal?.throwIfAborted();
  const metadata = await lstat(root);
  if (!metadata.isDirectory() || metadata.isSymbolicLink()) throw new Error(`Invalid artifact root: ${root}`);
  const files = new Map<string, string>(), pending = [root];
  while (pending.length) {
    signal?.throwIfAborted();
    const directory = pending.pop()!;
    for (const entry of await readdir(directory, { withFileTypes: true })) {
      signal?.throwIfAborted();
      const path = join(directory, entry.name);
      if (entry.isDirectory()) pending.push(path);
      else if (entry.isFile()) files.set(relative(root, path).replaceAll("\\", "/"), path);
      else throw new Error(`Unsupported artifact file: ${path}`);
    }
  }
  return new Map([...files].sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0));
}
