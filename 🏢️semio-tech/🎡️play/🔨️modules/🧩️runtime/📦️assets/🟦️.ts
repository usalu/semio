import { repositoryCachePartitionRootDataV1 } from "@semio-tech/repo-lib/storage";
import {frameworkOsViteArtifactPathsV1} from "@semio-tech/framework-os-dev/builder/vite/artifacts";
import type { BuildMode } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { COMPONENT_MODULE_DIRECTORIES } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { join, relative } from "node:path";
import { playRuntimeComponentIds } from "../🟦️.ts";
import { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { componentModuleOwnerV1, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "@semio-tech/plugin-registry/deployment";
import { PREVIEW2_VENDOR_RELATIVE } from "@semio-tech/framework-plugin/browser-bundle/imports";
import type { BrowserArtifactSource } from "@semio-tech/framework-plugin/browser-bundle/distribution";
import { builtPluginModulesRootIn } from "@semio-tech/framework-os-dev/activation";
import type { StaticDirectoryAssetSpecV1 } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

/** 🎡️ Selects only immutable component, browser-support, font and canvas runtime asset outputs for play, from the one plugin-modules root every build driver materializes into,
 * each under the exact ownership key its producer staged it with. */
export function playRuntimeAssetSources(workspace: string, mode: BuildMode, source: "built" | "activated" = "built"): readonly BrowserArtifactSource[] {
  const moduleRoot = builtPluginModulesRootIn(workspace,mode);
  const staged = frameworkOsViteArtifactPathsV1(repositoryCachePartitionRootDataV1(workspace, "products"), mode);
  const catalog = new Map([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => [row.pluginId, row]));
  const pluginRoute = MODULE_PLUGIN_ROUTE.slice(1);
  return [
    ...playRuntimeComponentIds().map(id => {
      const row = catalog.get(id)!, name = moduleDirectoryName(id, COMPONENT_MODULE_DIRECTORIES), extension = row.role === "extension";
      return { root: join(moduleRoot, name), destination: `${(extension ? MODULE_EXTENSION_ROUTE : MODULE_PLUGIN_ROUTE).slice(1)}/${name}`, owner: componentModuleOwnerV1(row.cratePath), ...(extension ? { shimDirectory: `${pluginRoute}/${PREVIEW2_VENDOR_RELATIVE}` } : {}) };
    }),
    { root: join(moduleRoot, PREVIEW2_VENDOR_RELATIVE), destination: `${pluginRoute}/${PREVIEW2_VENDOR_RELATIVE}`, owner: "browser-support:preview2" },
    { root: join(moduleRoot, MODULE_SHARD_DIRECTORY), destination: `${pluginRoute}/${MODULE_SHARD_DIRECTORY}`, owner: "browser-support:shard" },
    { root: staged.fonts, destination: `${pluginRoute}/${MODULE_VENDOR_DIRECTORY}`, owner: "infinite:fonts" },
    ...staged.canvas.map(({root,destination,owner})=>({root,destination,owner})),
  ];
}

/** 🗺️ The dev serve's route table: exactly the sources {@link playRuntimeAssetSources} copies for a
 * release, except Canvas which dev mounts from Source, with each extension served from its activated install directory. One root per
 * route (`staticDirMountVitePlugins` refuses a second claim): the staging `🪞️vendor` directory mounted beside
 * the font pack on the same route shadowed `🔤️guestslim-typst-fonts.bin` with a 404 on every pane. */
export function playDevStaticDirMounts(workspace: string, extensionDirectory: (name: string) => string, mode: BuildMode): readonly StaticDirectoryAssetSpecV1[] {
  const extensionRoute = MODULE_EXTENSION_ROUTE.slice(1) + "/";
  return playRuntimeAssetSources(workspace, mode, "activated").filter(row=>!row.owner.startsWith("canvas:")).map(row => ({ kind: "static-dir", route: `/${row.destination}`, root: relative(workspace, row.destination.startsWith(extensionRoute) ? extensionDirectory(row.destination.slice(extensionRoute.length)) : row.root) }));
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../🧪️tests/🧪️playdevmounts/🟦️.ts");
  await registerTests1(import.meta.vitest, { playDevStaticDirMounts });
}
