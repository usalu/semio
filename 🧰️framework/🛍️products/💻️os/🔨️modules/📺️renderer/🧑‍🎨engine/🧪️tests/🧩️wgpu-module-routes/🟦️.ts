import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import laws from "../../🧫️fixtures/🧩️wgpu-module-routes/🔣️.json";
import { assertBundleModuleRoutes, bundleCopyDirectives } from "../../🎯️targets/🧊️wgpu/⚙️browser-build/🟦️.ts";
import { MODULE_ROUTES, moduleRoutePath } from "../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { pluginModulesRoot } from "../../../../🧑‍💻dev/♻️activation/🟦️.ts";

const suiteDir = dirname(fileURLToPath(import.meta.url));
const bundleRoot = resolve(suiteDir, "../..", laws.bundleRoot);

describe("wgpu bundle module routes", () => {
  it("pins the fixture to the neutral deployment route authority", () => {
    expect(laws.routes).toEqual({ plugin: MODULE_ROUTES.plugin, extension: MODULE_ROUTES.extension });
  });

  it("carries one trunk copy-dir per declared module route, each at its declared root", () => {
    const directives = bundleCopyDirectives(readFileSync(join(bundleRoot, laws.bundleDocument), "utf8"));
    for (const route of Object.values(MODULE_ROUTES)) {
      const href = directives.get(route);
      expect(href, `no copy-dir publishes ${route}`).toBeTypeOf("string");
      expect(href, route).toBe((laws.routeRelativeRoots as Record<string, string>)[route]);
      expect(existsSync(resolve(bundleRoot, href!))).toBe(true);
    }
    expect(() => assertBundleModuleRoutes(bundleRoot)).not.toThrow();
  });

  it("copies the ONE staging root for the plugin route, never a second module tree", () => {
    const directives = bundleCopyDirectives(readFileSync(join(bundleRoot, laws.bundleDocument), "utf8"));
    expect(resolve(bundleRoot, directives.get(MODULE_ROUTES.plugin)!)).toBe(pluginModulesRoot("dev"));
    expect(resolve(bundleRoot, laws.pluginStagingRelativeRoot)).toBe(pluginModulesRoot("dev"));
  });

  it("refuses a bundle whose plugin copy-dir points away from the one staging root", () => {
    const root = mkdtempSync(join(tmpdir(), "wgpu-routes-second-tree-"));
    for (const directory of ["🔌️plugin-modules", "🧩️extension-modules"]) mkdirSync(join(root, directory), { recursive: true });
    writeFileSync(join(root, laws.bundleDocument), `<link data-trunk rel="copy-dir" href="./🔌️plugin-modules" data-target-path="🔌️plugin-modules" />\n<link data-trunk rel="copy-dir" href="./🧩️extension-modules" data-target-path="🧩️extension-modules" />\n`);
    expect(() => assertBundleModuleRoutes(root)).toThrow(/one staging root/u);
  });

  it("refuses a bundle that publishes only the plugin route", () => {
    const root = mkdtempSync(join(tmpdir(), "wgpu-routes-plugin-only-"));
    mkdirSync(join(root, "🔌️plugin-modules"), { recursive: true });
    writeFileSync(join(root, laws.bundleDocument), laws.pluginOnlyDocument);
    expect(bundleCopyDirectives(laws.pluginOnlyDocument).get(MODULE_ROUTES.extension)).toBeUndefined();
    expect(() => assertBundleModuleRoutes(root)).toThrow(new RegExp(MODULE_ROUTES.extension));
  });

  it("refuses a copy-dir whose source directory does not exist", () => {
    const root = mkdtempSync(join(tmpdir(), "wgpu-routes-dangling-"));
    writeFileSync(join(root, laws.bundleDocument), laws.danglingDocument);
    expect(() => assertBundleModuleRoutes(root, [MODULE_ROUTES.extension])).toThrow(/missing directory/u);
  });

  it("accepts a descriptor request under either route, so one unmounted route is a silent SPA fallthrough", () => {
    for (const route of Object.values(MODULE_ROUTES)) {
      const request = `${route}/${laws.descriptorProbeDirectory}/${laws.descriptorFile}`;
      expect(moduleRoutePath(request)).toBe(request);
    }
  });
});
