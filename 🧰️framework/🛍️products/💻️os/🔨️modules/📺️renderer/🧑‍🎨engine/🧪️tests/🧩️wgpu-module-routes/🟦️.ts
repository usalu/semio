import { describe, expect, it } from "vitest";
import { fromPairs } from "lodash-es";
import { join, resolve } from "node:path";
import laws from "../🧫️fixtures/🧩️wgpu-module-routes/🔣️.json";
import { wgpuBrowserMounts, type WgpuBrowserConfiguration } from "@semio-tech/framework-renderer-wgpu/server";
import { wgpuCompletedFrameworkRootsV1 } from "../../🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts";
import { MODULE_ROUTES, MODULE_VENDOR_DIRECTORY, moduleRoutePath } from "@semio-tech/plugin-registry/deployment";
import { frameworkOsViteArtifactPathsV1 } from "@semio-tech/framework-os-dev/builder/vite/artifacts";
import { repositoryCachePartitionRootDataV1 } from "@semio-tech/repo-lib/storage";
import { RUNTIME_ASSET_CHANNELS } from "@semio-tech/framework-canvas-ts/runtime-assets";
import { packageAssetMounts } from "@semio-tech/framework-kernel";

/** 🚏️ The native HTTP/cache contract additionally executes these routes under Vite. */
describe("WGPU completed-artifact routes", () => {
  it("serves exactly the runtime asset channels Canvas declares", () => {
    expect(RUNTIME_ASSET_CHANNELS.map(channel => channel.route)).toEqual(laws.runtimeAssetRoutes);
  });
  it("uses the neutral deployment route authority", () => {
    expect(laws.routes).toEqual({ plugin: MODULE_ROUTES.plugin, extension: MODULE_ROUTES.extension });
  });
  for (const profile of laws.profiles) it("mounts one source per route for " + profile, () => {
    const options = { ...Object.fromEntries(Object.values(laws.mounts).map(key => [key, profile + "/" + key])), workspace: resolve("/", "semio-wgpu-module-routes", profile, "workspace"), profile };
    const mounts = wgpuBrowserMounts(options as unknown as WgpuBrowserConfiguration);
    const actual = Object.fromEntries(mounts);
    const assets = fromPairs(laws.runtimeAssetRoutes.map(route => [route, resolve(options.workspace, RUNTIME_ASSET_CHANNELS.find(channel => channel.route === route)!.directory)]));
    const packageAssets = fromPairs(packageAssetMounts().map(([route, directory]) => [route, resolve(options.workspace, directory)]));
    const preview2Route = MODULE_ROUTES.plugin + "/" + laws.preview2Vendor.relativeRoot;
    const preview2Root = join(options.moduleRoot, laws.preview2Vendor.relativeRoot);
    const vendor = {
      [preview2Route]: preview2Root,
      [MODULE_ROUTES.plugin + "/" + MODULE_VENDOR_DIRECTORY]: frameworkOsViteArtifactPathsV1(repositoryCachePartitionRootDataV1(options.workspace, "products"), profile as "prototype" | "release").fonts,
    };
    expect(mounts.find(([route]) => laws.preview2Vendor.request.startsWith(route + "/"))).toEqual([preview2Route, preview2Root]);
    expect(actual).toEqual({ ...fromPairs(Object.entries(laws.mounts).map(([route, key]) => [route, options[key as keyof typeof options]])), ...vendor, ...assets, ...packageAssets });
    expect(Object.keys(actual)).toHaveLength(Object.keys(laws.mounts).length + Object.keys(vendor).length + laws.runtimeAssetRoutes.length + packageAssetMounts().length);
  });
  for (const profile of laws.profiles) it("projects completed roots into the runtime proof for " + profile, () => {
    const roots = wgpuCompletedFrameworkRootsV1(profile as "prototype" | "release");
    const mounts = fromPairs(wgpuBrowserMounts(roots));
    expect(roots.compilerRoot).toBe(frameworkOsViteArtifactPathsV1(repositoryCachePartitionRootDataV1(roots.workspace, "products"), profile as "prototype" | "release").renderer.directory);
    expect(roots.profile).toBe(profile);
    expect(mounts["/renderer-modules/wgpu"]).toBe(roots.compilerRoot);
    expect(mounts[MODULE_ROUTES.plugin]).toBe(roots.moduleRoot);
    expect(mounts[MODULE_ROUTES.plugin + "/" + laws.preview2Vendor.relativeRoot]).toBe(join(roots.moduleRoot, laws.preview2Vendor.relativeRoot));
  });
  it("recognizes descriptors under both mounted module routes", () => {
    for (const route of Object.values(MODULE_ROUTES)) {
      const request = route + "/" + laws.descriptorProbeDirectory + "/" + laws.descriptorFile;
      expect(moduleRoutePath(request)).toBe(request);
    }
  });
});
