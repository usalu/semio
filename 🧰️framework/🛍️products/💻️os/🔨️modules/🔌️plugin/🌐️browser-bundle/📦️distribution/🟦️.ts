import { constants } from "node:fs";
import { copyFile, lstat, mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";
import schemaModule from "../🧬️schema/🔣️.json";
import { rewritePreview2ShimImportSource } from "../🕸️imports/🟦️.ts";

export type BrowserArtifactSource = {
  readonly root: string;
  readonly destination: string;
  readonly owner: string;
  readonly shimDirectory?: string;
};
export type BrowserArtifactCopyOptions = { readonly signal?: AbortSignal; readonly progress?: (file: string) => void };
const pathPattern = new RegExp(schemaModule.$defs.BrowserArtifactDistributionV1.properties.files.items.pattern, "u");

/** 🛣️ Admits relative artifact paths with one canonical separator representation. */
function artifactPath(value: unknown): string {
  if (typeof value !== "string" || !pathPattern.test(value)) throw new Error(`Invalid browser artifact path: ${String(value)}`);
  return value.replaceAll("\\", "/");
}

/** 🗂️ Rejects symlink parents before reading or writing owned artifact trees. */
async function regularParents(path: string): Promise<void> {
  for (let current = resolve(path); dirname(current) !== current; current = dirname(current)) {
    try { if ((await lstat(current)).isSymbolicLink()) throw new Error(`Symlink browser artifact path: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
  }
}

/** 📦️ Copies declared immutable browser files into a caller-owned staging directory. */
export async function copyBrowserArtifacts(outputRoot: string, sources: readonly BrowserArtifactSource[], options: BrowserArtifactCopyOptions = {}): Promise<number> {
  options.signal?.throwIfAborted();
  await regularParents(outputRoot);
  const files: { source: string; output: string; name: string; shimPrefix?: string }[] = [], destinations = new Set<string>();
  for (const source of sources) {
    options.signal?.throwIfAborted();
    const destination = artifactPath(source.destination), manifestPath = join(source.root, ".nx-artifact.json");
    await regularParents(manifestPath);
    const stat = await lstat(manifestPath);
    if (!stat.isFile() || stat.size > 1024 * 1024) throw new Error(`Invalid browser artifact manifest: ${manifestPath}`);
    const marker = JSON.parse(await readFile(manifestPath, "utf8"));
    if (Object.keys(marker).sort().join() !== "files,owner,version" || marker.version !== 1 || !source.owner || marker.owner !== source.owner) throw new Error(`Browser artifact owner mismatch: ${source.root}`);
    if (!Array.isArray(marker.files) || marker.files.length === 0) throw new Error(`Missing browser artifact paths: ${source.root}`);
    for (const value of marker.files) {
      const name = artifactPath(value), input = join(source.root, name), output = join(outputRoot, destination, name), key = resolve(output);
      if (destinations.has(key)) throw new Error(`Browser artifact path collision: ${key}`);
      destinations.add(key);
      await regularParents(input);
      await regularParents(output);
      if (!(await lstat(input)).isFile()) throw new Error(`Invalid browser artifact file: ${input}`);
      try { await lstat(output); throw new Error(`Browser artifact destination exists: ${output}`); }
      catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
      const shimPrefix = source.shimDirectory === undefined ? undefined : relative(dirname(output), join(outputRoot, artifactPath(source.shimDirectory))).replaceAll("\\", "/") + "/";
      files.push({ source: input, output, name: destination + "/" + name, shimPrefix });
    }
  }
  for (const file of files) {
    options.signal?.throwIfAborted();
    await mkdir(dirname(file.output), { recursive: true });
    if (file.shimPrefix !== undefined && file.source.endsWith(".js")) await writeFile(file.output, rewritePreview2ShimImportSource(await readFile(file.source, "utf8"), file.shimPrefix), { flag: "wx", signal: options.signal });
    else await copyFile(file.source, file.output, constants.COPYFILE_EXCL);
    options.progress?.(file.name);
  }
  options.signal?.throwIfAborted();
  return files.length;
}
