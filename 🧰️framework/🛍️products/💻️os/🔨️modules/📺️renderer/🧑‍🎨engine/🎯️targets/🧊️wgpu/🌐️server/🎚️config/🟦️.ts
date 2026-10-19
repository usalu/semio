import {frameworkOsViteArtifactPathsV1} from "@semio-tech/framework-os-dev/builder/vite/artifacts";
import { repositoryCachePartitionRootDataV1 } from "@semio-tech/repo-lib/storage";
import {emptyNativeCatalogV1,readNativeCatalogV1,type NativeCatalogV1} from "@semio-tech/plugin-registry/runtime";
import { resolveAssetDeliveryModeV1 } from "@semio-tech/assets/resolver/dispatch";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createWgpuBrowserConfig } from "../🟦️.ts";
import { ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot, builtPluginModulesRootIn } from "@semio-tech/framework-os-dev/activation";
import { semioAssetsVitePlugin } from "@semio-tech/ui-styling/builder/vite/assets";
import { semioServeCloseVitePlugin } from "@semio-tech/ui-styling/builder/vite/status-serve";
import { semioBackboneVitePlugin } from "@semio-tech/framework-os-dev/vite-plugins/backbone-vite-plugin";
import { semioSourceFreshnessVitePlugins } from "@semio-tech/framework-os-dev/vite-plugins/source-freshness";
import { readBuildMode } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const workspace = resolve(root, "../../../../../../../../..");
/** 🎚️ Selects the server's actual completed build profile. */
export function wgpuCompletedProfileV1(): "prototype" | "release" {
  return readBuildMode(process.env.SEMIO_BUILD_MODE) === "release" ? "release" : "prototype";
}
/** 📂️ Owns the completed native/browser roots shared by server mounts and runtime graph verification. */
export function wgpuCompletedFrameworkRootsV1(profile: "prototype" | "release") {
  return {
    compilerRoot: frameworkOsViteArtifactPathsV1(repositoryCachePartitionRootDataV1(workspace, "products"), profile).renderer.directory,
    bootRoot: resolve(root, "../🚀️browser-boot/🤖️generated"),
    libraryRoot: resolve(root, "../🎬️renderer-boot/🤖️generated"),
    workerRoot: resolve(root, "../🎞️frame-worker/🤖️generated"),
  };
}
export function createCompletedWgpuConfigurationV1(options: { readonly variant: string | undefined; readonly nativeCatalog?: NativeCatalogV1; readonly catalog: import("@semio-tech/framework/plugin-catalog").PluginCatalogRowsV1; readonly assets: readonly import("@semio-tech/assets/resolver/dispatch").AssetDeliveryDeclarationV1[] }) {
  const variant = options.variant;
  const profile = wgpuCompletedProfileV1();
  const moduleRoot = builtPluginModulesRootIn(workspace,profile);
  const runtime = developmentRuntimeRoot(workspace, variant!, profile, "wgpu");
  const config = createWgpuBrowserConfig({
    workspace, root, profile, variant,
    catalog: options.catalog, nativeCatalog: options.nativeCatalog ?? emptyNativeCatalogV1(),
    ...wgpuCompletedFrameworkRootsV1(profile),
    moduleRoot,
    extensionRoot: join(runtime, "extensions"),
    reloadFile: join(runtime, "activation", ACTIVATION_RECEIPT_FILE),
    assets: options.assets,
    assetServeMode: resolveAssetDeliveryModeV1(process.env.SEMIO_ASSET_SERVE_MODE),
  });
  return { ...config, plugins: [semioServeCloseVitePlugin(), ...config.plugins!, semioBackboneVitePlugin(), ...semioAssetsVitePlugin(workspace), ...semioSourceFreshnessVitePlugins({ repoRoot: workspace })], server: { ...config.server, ...(process.env.S_LOCAL_RELAY_URL ? { proxy: { "/_semio": { target: process.env.S_LOCAL_RELAY_URL, changeOrigin: false, headers: process.env.S_LOCAL_RELAY_SECRET ? { "x-semio-local-relay": process.env.S_LOCAL_RELAY_SECRET } : undefined } } } : {}) } };
}

export default () => createCompletedWgpuConfigurationV1({ variant: undefined, catalog: { version: 1, targets: [], hosts: [], playgrounds: [] }, assets: [] });
