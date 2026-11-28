/** 🎯️ A command or a suite reads only the catalog assets it names, and a pass over the catalog holds one asset at a time.
 *
 * THE REFERENCE is a catalog folder whose other asset document is not JSON: any read of it fails, so a success proves it was never opened.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { cpSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { assetPathsFromFiles, loadCatalogFromFiles, streamAssetsFromFiles } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { drawingOf, parseArguments } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { STAFFAGE_ROOT, withScratch } from "../🧰️support/🧰️.ts";

const REAL = join(STAFFAGE_ROOT, "🗂️catalog");
const manifest = JSON.parse(readFileSync(join(REAL, "🔣️.json"), "utf8")) as { categories: { folder: string }[] };
const objects = manifest.categories.map((category) => category.folder).find((folder) => folder.includes("objects"))!;

const withCatalog = async (body: (root: string) => Promise<void>): Promise<void> =>
  withScratch(async (directory) => {
    const root = join(directory, "catalog");
    mkdirSync(join(root, objects, "tealight"), { recursive: true });
    mkdirSync(join(root, objects, "zzz-poison"), { recursive: true });
    writeFileSync(join(root, "🔣️.json"), JSON.stringify({ ...manifest, categories: [{ ...manifest.categories.find((c) => c.folder === objects)! }] }));
    cpSync(join(REAL, objects, "tealight", "🔣️.json"), join(root, objects, "tealight", "🔣️.json"));
    writeFileSync(join(root, objects, "zzz-poison", "🔣️.json"), "{ this is not json");
    await body(root);
  });

describe("🎯️ the catalog is read asset by asset", () => {
  it("loads the named asset and never opens the other document", () =>
    withCatalog(async (root) => {
      const catalog = await loadCatalogFromFiles(root, undefined, new Set(["tealight"]));
      expect(catalog.assets.map((asset) => asset.id)).toEqual(["tealight"]);
      await expect(loadCatalogFromFiles(root)).rejects.toThrow();
    }));

  it("prepares the drawing of an export command from its asset alone", () =>
    withCatalog(async (root) => {
      const options = { ...parseArguments(["--asset", "tealight", "--pose", "lit", "--view", "light-solid", "--scale", "50", "--format", "svg", "--viewpoint", "elevation", "--out", "unused", "--lang", "en"]), catalog: root };
      const { asset } = await drawingOf(options);
      expect(asset.id).toBe("tealight");
    }));

  it("advances a pass over the catalog one document at a time", () =>
    withCatalog(async (root) => {
      expect((await assetPathsFromFiles(root)).map((entry) => entry.folder)).toEqual(["tealight", "zzz-poison"]);
      const stream = streamAssetsFromFiles(root);
      expect((await stream.next()).value?.asset.id).toBe("tealight");
      await stream.return();
    }));
});
