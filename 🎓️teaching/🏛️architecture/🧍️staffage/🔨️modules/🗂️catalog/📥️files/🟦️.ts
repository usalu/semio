/** 📥️ Node-only loader of the on-disk catalog (manifest plus one document per category) for command-line tools and tests.
 *
 * @see ../🟦️.ts — `validateAsset`, `validateManifest`
 * @see ../../../🗂️catalog/🔣️.json — the manifest
 */
import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Asset, CategoryDocument, Manifest } from "../../../🧬️schema/🟦️.ts";

/** 📥️ The catalog as read from disk. */
export type LoadedCatalog = { readonly root: string; readonly manifest: Manifest; readonly documents: readonly CategoryDocument[]; readonly assets: readonly Asset[] };

/** 📂️ The default catalog directory, `🗂️catalog` beside the modules. */
export const DEFAULT_CATALOG_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../../🗂️catalog");

async function readJson<T>(path: string, signal?: AbortSignal): Promise<T> {
  signal?.throwIfAborted();
  return JSON.parse(await readFile(path, "utf8")) as T;
}

/** 📥️ Reads `🔣️.json` of the catalog root and every category file it lists; category files that do not exist yet are skipped, so a partly authored archive still loads. */
export async function loadCatalogFromFiles(root: string = DEFAULT_CATALOG_ROOT, signal?: AbortSignal): Promise<LoadedCatalog> {
  const manifest = await readJson<Manifest>(resolve(root, "🔣️.json"), signal);
  const documents: CategoryDocument[] = [];
  for (const category of manifest.categories) {
    try {
      documents.push(await readJson<CategoryDocument>(resolve(root, category.file), signal));
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    }
  }
  return { root, manifest, documents, assets: documents.flatMap((document) => document.assets) };
}
