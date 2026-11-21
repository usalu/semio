// #region 🔌️Adapters
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { validateAsset, type Issue } from "../../../🔨️modules/🗂️catalog/🟦️.ts";
import type { Asset } from "../../../🧬️schema/🟦️.ts";
import type { OwnedBuildPlugin } from "../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🛠️build-tooling/🟦️.ts";
// #endregion 🔌️Adapters

/** 🔖️ The import query under which a catalog asset file is served as its shipped payload; `import.meta.glob` needs it as a literal, so the loader spells it out. */
export const STAFFAGE_ASSET_QUERY = "?staffage-asset";

/** 📦️ One authored asset, its revision and the semantic findings computed when Vite serves or builds its module. */
export type StaffageAssetPayload = { readonly revision: string; readonly json: string; readonly issues: readonly Issue[] };

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
  const asset = (JSON.parse(text) as { asset: Asset }).asset;
  const negativeZero = (JSON as JSON & { rawJSON(value: string): unknown }).rawJSON("-0");
  const json = JSON.stringify(asset, (_key, value) => Object.is(value, -0) ? negativeZero : value);
  try {
    return { revision, json, issues: validateAsset(asset) };
  } catch {
    return { revision, json, issues: [{ path: "$", code: "validation-threw", message: { en: "The check of this asset stopped before it finished.", de: "Die Prüfung dieses Assets brach vorzeitig ab." } }] };
  }
}

/** 🧍️ Serves a revisioned asset module. Releases reject validation findings, while development and Vitest expose them to the archive.
 * @see {@link staffageAssetPayload} */
export function staffageAssetVitePlugin(release = false): OwnedBuildPlugin {
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
      const payload = staffageAssetPayload(file);
      if (release && payload.issues.length) {
        const findings = payload.issues.map(({ code, path, message }) => `${path} [${code}] ${message.en} / ${message.de}`);
        throw new Error([file, ...findings].join("\n"));
      }
      return `export default ${JSON.stringify(payload)};`;
    },
  };
}
