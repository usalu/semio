/** 🔏️ The content digest of the code and schemas an export is produced by: the staffage modules and schema and the 3D kernel; the catalog documents are not part of it, because each asset already carries the digest of its own document (including its portable math). A resume that compares only the catalog document of an asset keeps files an older encoder wrote; with this digest in the settings of a set, any change of a producing file makes the next run rewrite the files whose bytes change, and an unchanged tree resumes as before.
 * Deterministic: files in sorted path order, each by its relative path and SHA-256; tests, fixtures, dependencies and build output are not producers.
 * @see ../🟦️.ts — `BatchSettings.producer`
 */
import { createHash } from "node:crypto";
import { readFile, readdir } from "node:fs/promises";
import { join, relative, resolve, sep } from "node:path";

const SKIPPED = new Set(["🧪️tests", "🧫️fixtures", "node_modules", "📤️dist", ".cache"]);
const EXTENSIONS = new Set([".ts", ".mjs", ".json"]);

async function walk(directory: string, signal: AbortSignal | undefined, out: string[]): Promise<void> {
  signal?.throwIfAborted();
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      if (!SKIPPED.has(entry.name)) await walk(join(directory, entry.name), signal, out);
    } else if (entry.isFile() && EXTENSIONS.has(entry.name.slice(entry.name.lastIndexOf(".")))) out.push(join(directory, entry.name));
  }
}

/** 🧮️ Digests the producing files below `roots` (absolute directories), relative to `base`. */
export async function producerDigest(roots: readonly string[], base: string, signal?: AbortSignal): Promise<string> {
  const files: string[] = [];
  for (const root of roots) await walk(resolve(root), signal, files);
  const names = files.map((file) => relative(base, file).split(sep).join("/")).sort();
  const hash = createHash("sha256");
  for (const name of names) {
    signal?.throwIfAborted();
    hash.update(name).update("\0").update(createHash("sha256").update(await readFile(join(base, name))).digest("hex")).update("\n");
  }
  return hash.digest("hex");
}
