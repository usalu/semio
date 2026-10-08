import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { describe, expect, test } from "vitest";

const root = resolve(import.meta.dirname, "../..");
const workspace = resolve(root, "../../../../../..");
describe("host variant Nx fan-out", () => {
  test("a host crate's web prepare stays boot-scoped while native still fans out every registered component", async () => {
    const { cacheInternals, libraryBootstrap } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
    await libraryBootstrap;
    const base = join(workspace, ".🧬semio/🦑️repo/⚡️cache/🧪️plugin-registry/host-activation");
    mkdirSync(base, { recursive: true });
    const fixture = mkdtempSync(join(base, "run-"));
    try {
      const put = (path: string, text: string): void => { mkdirSync(dirname(join(fixture, path)), { recursive: true }); writeFileSync(join(fixture, path), text); };
      const project = (path: string, name: string): void => put(join(path, "📋️project.json"), JSON.stringify({ name, targets: { wasm: {}, "wasm-release": {} } }));
      project("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript", "renderer");
      project("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust", "flow");
      const crate = (directory: string, id: string, extra: string): void => {
        project(directory, id);
        put(join(directory, "Cargo.toml"), `[package]\nname = "${id}"\nversion = "0.1.0"\n[package.metadata.component]\npackage = "semio:${id}"\n[package.metadata.semio]\ncomponent-kind = "plugin"\n${extra}[[package.metadata.semio.playground]]\nvariant = "${id}"\nports = { react = 6100, wgpu = 6110 }\n`);
      };
      crate("hub", "hub", 'host = { landing = "home", shell = "studio" }\n');
      crate("alpha", "alpha", "");
      crate("beta", "beta", "");
      const targets = cacheInternals.playgroundPreparationTargets(["hub/Cargo.toml", "alpha/Cargo.toml", "beta/Cargo.toml"], fixture, "owner");
      for (const profile of ["dev", "release"]) {
        for (const renderer of ["react", "wgpu"] as const) {
          const prepare = targets[`prepare-hub-${renderer}-${profile}`];
          expect(prepare, `prepare-hub-${renderer}-${profile}`).toBeDefined();
          expect(prepare.dependsOn, `prepare-hub-${renderer}-${profile} hub`).toContain(`hub:materialize-${profile}`);
          expect(prepare.dependsOn, `prepare-hub-${renderer}-${profile} alpha`).not.toContain(`alpha:materialize-${profile}`);
          expect(prepare.dependsOn, `prepare-hub-${renderer}-${profile} beta`).not.toContain(`beta:materialize-${profile}`);
        }
        const nativePrepare = targets[`prepare-hub-native-${profile}`];
        expect(nativePrepare, `prepare-hub-native-${profile}`).toBeDefined();
        for (const id of ["hub", "alpha", "beta"]) expect(nativePrepare.dependsOn, `prepare-hub-native-${profile} ${id}`).toContain(`${id}:materialize-${profile}`);
        // 🚫️ A non-host variant must NOT inherit the fan-out — that is the regression this pins.
        expect(targets[`prepare-alpha-react-${profile}`].dependsOn).not.toContain(`beta:materialize-${profile}`);
        for (const renderer of ["react", "wgpu"]) expect(targets[`activate-hub-${renderer}-${profile}`].dependsOn).toEqual([`prepare-hub-${renderer}-${profile}`]);
      }
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });
});
