/** 📥️ Node-only loader of the on-disk catalog for command-line tools and tests: the manifest, one document per asset folder
 * (`<category folder>/<asset id>/🔣️.json`), the print profile document and the environment document.
 *
 * @see ../🟦️.ts — `validateAsset`, `validateManifest`
 * @see ../../../🗂️catalog/🔣️.json — the manifest
 */
import { readFile, readdir } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Asset, AssetDocument, EnvironmentDocument, Manifest, PrintProfileDocument, Slug } from "../../../🧬️schema/🟦️.ts";

/** 📥️ One asset as read from disk with the folder it came from. */
export type LoadedAsset = { readonly asset: Asset; readonly document: AssetDocument; readonly folder: string; readonly categoryFolder: string; readonly path: string };

/** 📥️ The catalog as read from disk. */
export type LoadedCatalog = { readonly root: string; readonly manifest: Manifest; readonly documents: readonly AssetDocument[]; readonly assets: readonly Asset[]; readonly loaded: readonly LoadedAsset[]; readonly profiles: PrintProfileDocument | null };

/** 📂️ The default catalog directory, `🗂️catalog` beside the modules. */
export const DEFAULT_CATALOG_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../../🗂️catalog");

/** 🖨️ The folder name of the print profile document under the catalog root. */
export const PRINT_PROFILES_FOLDER = "🖨️print-profiles";

/** 🌤️ The folder name of the environment document under the catalog root. */
export const ENVIRONMENTS_FOLDER = "🌤️environments";

async function readJson<T>(path: string, signal?: AbortSignal): Promise<T> {
  signal?.throwIfAborted();
  return JSON.parse(await readFile(path, "utf8")) as T;
}

async function readOptional<T>(path: string, signal?: AbortSignal): Promise<T | null> {
  try {
    return await readJson<T>(path, signal);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
    throw error;
  }
}

/** 🖨️ Reads the print profile document of a catalog root, `null` when it does not exist. */
export function loadPrintProfiles(root: string = DEFAULT_CATALOG_ROOT, signal?: AbortSignal): Promise<PrintProfileDocument | null> {
  return readOptional<PrintProfileDocument>(resolve(root, PRINT_PROFILES_FOLDER, "🔣️.json"), signal);
}

/** 🌤️ Reads the environment preset document of a catalog root, `null` when it does not exist. */
export function loadEnvironments(root: string = DEFAULT_CATALOG_ROOT, signal?: AbortSignal): Promise<EnvironmentDocument | null> {
  return readOptional<EnvironmentDocument>(resolve(root, ENVIRONMENTS_FOLDER, "🔣️.json"), signal);
}

/** 📥️ Reads `🔣️.json` of the catalog root, every asset document of every category folder it lists (asset folders without a document are skipped, so a partly authored archive still loads) and the print profiles. Assets come in folder order within manifest category order. */
export async function loadCatalogFromFiles(root: string = DEFAULT_CATALOG_ROOT, signal?: AbortSignal): Promise<LoadedCatalog> {
  const manifest = await readJson<Manifest>(resolve(root, "🔣️.json"), signal);
  const loaded: LoadedAsset[] = [];
  for (const category of manifest.categories) {
    let folders: string[];
    try {
      folders = (await readdir(resolve(root, category.folder), { withFileTypes: true })).filter((entry) => entry.isDirectory()).map((entry) => entry.name).sort();
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code === "ENOENT") continue;
      throw error;
    }
    for (const folder of folders) {
      const path = resolve(root, category.folder, folder, "🔣️.json");
      const document = await readOptional<AssetDocument>(path, signal);
      if (document !== null) loaded.push({ asset: document.asset, document, folder: folder as Slug, categoryFolder: category.folder, path });
    }
  }
  return { root, manifest, documents: loaded.map((entry) => entry.document), assets: loaded.map((entry) => entry.asset), loaded, profiles: await loadPrintProfiles(root, signal) };
}
