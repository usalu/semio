/** 📦️ Production and play bundles for the runtime asset port (Node side; the browser side is `../🟦️.ts`). The three asset directories are
 * staged once by `semio-framework-canvas:stage-runtime-assets` into provenance-marked trees, and every bundle pipeline copies exactly those
 * trees to the routes the browser fetches: `/renderer-modules/<channel>/<file>` for the seven port assets, and
 * `/renderer-modules/package-assets/<repository directory>/<file>` for the three a wasip2 guest receives as package assets (the route
 * `🎠️kernel/📦️package-assets` fetches declared names from). */
import { GUEST_RUNTIME_ASSET_FILES, RUNTIME_ASSET_CHANNELS } from "../🟦️.ts";
import { isAbsolute, join, resolve } from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🏷️ The ownership keys the staged trees carry in their `.semio-artifact.json`. */
export const RUNTIME_ASSET_STAGE_OWNER = "canvas:runtime-assets";
export const PACKAGE_ASSET_STAGE_OWNER = "canvas:package-assets";

/** 🚏️ The package asset route without its leading slash, as a bundle destination. */
export const PACKAGE_ASSET_DESTINATION = "renderer-modules/package-assets";

/** 🌳️ Names an absolute managed output root and its repository-relative immutable inputs. */
export interface RuntimeAssetStageTree {
  readonly root: string;
  readonly destination: string;
  readonly owner: string;
  readonly files: readonly { readonly name: string; readonly source: string }[];
}

/** 🌳️ Six trees: per channel the whole directory's port files, and the guest subset under its package asset name. */
export function runtimeAssetStageTrees(productsRoot: string, mode: string): readonly RuntimeAssetStageTree[] {
  if (!isAbsolute(productsRoot) || resolve(productsRoot) !== productsRoot || !schema.properties.mode.enum.includes(mode)) throw Error("Canonical Canvas products root and build mode required");
  const root = join(productsRoot, "canvas", mode);
  const guest = new Set(GUEST_RUNTIME_ASSET_FILES.map(({ channel, file }) => `${channel}/${file}`));
  const port = RUNTIME_ASSET_CHANNELS.map(({ id, route, directory, files }): RuntimeAssetStageTree => ({ root: join(root, "port-assets", id), destination: route.slice(1), owner: RUNTIME_ASSET_STAGE_OWNER, files: files.map(file => ({ name: file, source: `${directory}/${file}` })) }));
  const packages = RUNTIME_ASSET_CHANNELS.map(({ id, directory, files }): RuntimeAssetStageTree => ({ root: join(root, "guest-assets", id), destination: `${PACKAGE_ASSET_DESTINATION}/${directory}`, owner: PACKAGE_ASSET_STAGE_OWNER, files: files.filter(file => guest.has(`${id}/${file}`)).map(file => ({ name: file, source: `${directory}/${file}` })) }));
  const trees = [...port, ...packages];
  if (trees.some(tree => tree.root.length > 224 || tree.files.some(file => join(tree.root, file.name).length > 256))) throw Error("Canvas artifact path capacity exceeded");
  return Object.freeze(trees.map(tree => Object.freeze({ ...tree, files: Object.freeze(tree.files.map(file => Object.freeze(file))) })));
}

/** 🚚️ Resolves finite source descriptors; their actual reader must retain publication custody. */
export function canvasRuntimeAssetSources(productsRoot: string, mode: string): readonly { readonly root: string; readonly destination: string; readonly owner: string }[] {
  return runtimeAssetStageTrees(productsRoot, mode).map(({ root, destination, owner }) => ({ root, destination, owner }));
}
