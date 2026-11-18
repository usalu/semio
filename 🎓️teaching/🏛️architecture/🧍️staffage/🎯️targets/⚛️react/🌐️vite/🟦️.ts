// #region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import type { OwnedBuildPlugin } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
// #endregion 🔌️Adapters

/** 🔖️ The import query under which a catalog asset file is served as its shipped payload; `import.meta.glob` needs it as a literal, so the loader spells it out. */
export const STAFFAGE_ASSET_QUERY = "?staffage-asset";

/** 📦️ What a catalog asset file ships as: its revision, precomputed from the authored text, and the asset as compact JSON text. */
export type StaffageAssetPayload = { readonly revision: string; readonly json: string };

/** 👀️ The part of a Vite or Rollup plugin context that registers a file whose change invalidates the module. */
export type WatchContext = { readonly addWatchFile: (file: string) => void };

const PREFIX = "\0staffage-asset:";
const SUFFIX = ".mjs";

/** 🗜️ Reads one authored asset file: the revision is `sha256:` of its text with line breaks normalised to `\n`, exactly as `revisionOfText` states it, and the asset is stripped of its indentation and envelope so the browser parses it without hashing anything.
 * @see ../🔨️modules/📚️catalog/🟦️.ts — the loader
 * @see ../../../🔨️modules/🏷️revision/🟦️.ts — `revisionOfText` */
export function staffageAssetPayload(file: string): StaffageAssetPayload {
  const text = readFileSync(file, "utf8");
  const revision = `sha256:${createHash("sha256").update(text.replace(/\r\n/gu, "\n"), "utf8").digest("hex")}`;
  return { revision, json: JSON.stringify((JSON.parse(text) as { asset: unknown }).asset) };
}

/** 🧍️ Vite: serves `<asset file>?staffage-asset` as `export default { revision, json }` so a chunk carries compact JSON and a precomputed revision instead of the pretty-printed file. Works for the dev server, the build and Vitest; the file is watched, so an edited asset reloads in the dev server.
 * @see {@link staffageAssetPayload} */
export function staffageAssetVitePlugin(): OwnedBuildPlugin {
  let root = process.cwd();
  return {
    name: "staffage-asset-payload",
    enforce: "pre",
    configResolved(config) {
      root = config.root;
    },
    resolveId(id, importer) {
      if (!id.endsWith(STAFFAGE_ASSET_QUERY)) return null;
      const path = id.slice(0, -STAFFAGE_ASSET_QUERY.length);
      const from = importer === undefined ? root : dirname(importer.split("?")[0]!);
      const file = path.startsWith(".") ? resolve(from, path) : existsSync(path) ? path : resolve(root, `.${path}`);
      return `${PREFIX}${file}${SUFFIX}`;
    },
    load(this: WatchContext | void, id: string) {
      if (!id.startsWith(PREFIX)) return null;
      const file = id.slice(PREFIX.length, -SUFFIX.length);
      this?.addWatchFile(file);
      const { revision, json } = staffageAssetPayload(file);
      return `export default { revision: ${JSON.stringify(revision)}, json: ${JSON.stringify(json)} };`;
    },
  };
}
