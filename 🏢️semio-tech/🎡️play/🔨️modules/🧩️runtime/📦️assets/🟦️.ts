import { join, relative } from "node:path";
import { playRuntimeComponentIds } from "../🟦️.ts";
import { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";
import type { BrowserArtifactSource } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🟦️.ts";
import type { PlaygroundAssetSpec } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

/** @emoji 🎡️ Selects only immutable component, browser-support and font outputs for play. */
export function playRuntimeAssetSources(workspace: string, profile: "dev" | "release"): readonly BrowserArtifactSource[] {
  if (!["dev", "release"].includes(profile)) throw new Error(`Unknown play profile: ${profile}`);
  const moduleRoot = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist", profile, "🔌️plugin-modules");
  const catalog = new Map([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => [row.pluginId, row]));
  const pluginRoute = MODULE_PLUGIN_ROUTE.slice(1);
  return [
    ...playRuntimeComponentIds().map(id => {
      const row = catalog.get(id)!, name = moduleDirectoryName(id), extension = row.role === "extension";
      return { root: join(moduleRoot, name), destination: `${(extension ? MODULE_EXTENSION_ROUTE : MODULE_PLUGIN_ROUTE).slice(1)}/${name}`, owner: `${row.cratePath}/Cargo.toml:browser:${profile}`, ...(extension ? { shimDirectory: `${pluginRoute}/${PREVIEW2_VENDOR_RELATIVE}` } : {}) };
    }),
    { root: join(moduleRoot, PREVIEW2_VENDOR_RELATIVE), destination: `${pluginRoute}/${PREVIEW2_VENDOR_RELATIVE}`, owner: `browser-support:${profile}:preview2` },
    { root: join(moduleRoot, MODULE_SHARD_DIRECTORY), destination: `${pluginRoute}/${MODULE_SHARD_DIRECTORY}`, owner: `browser-support:${profile}:shard` },
    { root: join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts"), destination: `${pluginRoute}/${MODULE_VENDOR_DIRECTORY}`, owner: "infinite:fonts" },
  ];
}

/** @emoji 🗺️ The dev serve's route table: exactly the sources {@link playRuntimeAssetSources} copies for a
 * release, mounted in place, with each extension served from its activated install directory. One root per
 * route (`staticDirMountVitePlugins` refuses a second claim): the staging `🪞️vendor` directory mounted beside
 * the font pack on the same route shadowed `🔤️guestslim-typst-fonts.bin` with a 404 on every pane. */
export function playDevStaticDirMounts(workspace: string, extensionDirectory: (name: string) => string): readonly Extract<PlaygroundAssetSpec, { kind: "static-dir" }>[] {
  const extensionRoute = MODULE_EXTENSION_ROUTE.slice(1) + "/";
  return playRuntimeAssetSources(workspace, "dev").map(row => ({ kind: "static-dir", route: `/${row.destination}`, root: relative(workspace, row.destination.startsWith(extensionRoute) ? extensionDirectory(row.destination.slice(extensionRoute.length)) : row.root) }));
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playdevmounts/🟦️.ts");
  await registerTests1(import.meta.vitest, { playDevStaticDirMounts });
}
