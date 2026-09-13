import { createHash } from "node:crypto";
import { createReadStream, readdirSync } from "node:fs";
import { join, relative } from "node:path";

export function artifactFiles(root: string): Map<string, string> {
  const files = new Map<string, string>();
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const path = join(directory, entry.name);
      if (entry.isSymbolicLink()) throw new Error(`Symlink in browser artifact: ${path}`);
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) files.set(relative(root, path), path);
      else throw new Error(`Unsupported browser artifact: ${path}`);
    }
  };
  walk(root);
  return files;
}

export async function fileDigest(path: string, signal?: AbortSignal): Promise<string> {
  signal?.throwIfAborted();
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path, { signal })) hash.update(chunk);
  return hash.digest("hex");
}
