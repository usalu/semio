import { describe, expect, it } from "vitest";
import { fromPairs } from "lodash-es";
import laws from "../../🧫️fixtures/🧩️wgpu-module-routes/🔣️.json";
import { wgpuBrowserMounts, type WgpuBrowserConfiguration } from "../../🎯️targets/🧊️wgpu/🌐️server/🟦️.ts";
import { MODULE_ROUTES, moduleRoutePath } from "../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

/** 🚏️ The native HTTP/cache contract additionally executes these routes under Vite. */
describe("WGPU completed-artifact routes", () => {
  it("uses the neutral deployment route authority", () => {
    expect(laws.routes).toEqual({ plugin: MODULE_ROUTES.plugin, extension: MODULE_ROUTES.extension });
  });
  for (const profile of laws.profiles) it("mounts one source per route for " + profile, () => {
    const options = Object.fromEntries(Object.values(laws.mounts).map(key => [key, profile + "/" + key]));
    const actual = Object.fromEntries(wgpuBrowserMounts(options as unknown as WgpuBrowserConfiguration));
    expect(actual).toEqual(fromPairs(Object.entries(laws.mounts).map(([route, key]) => [route, options[key]])));
    expect(Object.keys(actual)).toHaveLength(Object.keys(laws.mounts).length);
  });
  it("recognizes descriptors under both mounted module routes", () => {
    for (const route of Object.values(MODULE_ROUTES)) {
      const request = route + "/" + laws.descriptorProbeDirectory + "/" + laws.descriptorFile;
      expect(moduleRoutePath(request)).toBe(request);
    }
  });
});
