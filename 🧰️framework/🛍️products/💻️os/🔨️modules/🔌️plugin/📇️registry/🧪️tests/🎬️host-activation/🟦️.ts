import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { describe, expect, test } from "vitest";

const root = resolve(import.meta.dirname, "../..");
const workspace = resolve(root, "../../../../../..");
describe("host variant Nx fan-out", () => {
  test("a host crate's web prepare stays boot-scoped while native still fans out every registered component", async () => {
    const { cacheInternals, libraryBootstrap } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs")).href);
    await libraryBootstrap;
    const base = join(workspace, ".🧬semio/🦑️repo/⚡️cache/🧪️plugin-registry/host-activation");
    mkdirSync(base, { recursive: true });
    const fixture = mkdtempSync(join(base, "run-"));
    try {
      const put = (path: string, text: string): void => { mkdirSync(dirname(join(fixture, path)), { recursive: true }); writeFileSync(join(fixture, path), text); };
      const project = (path: string, name: string): void => put(join(path, "📋️project.json"), JSON.stringify({ name, targets: { wasm: { configurations: { prototype: { env: { SEMIO_BUILD_MODE: "prototype" } }, release: { env: { SEMIO_BUILD_MODE: "release" } } } }, "wasm-release": {} } }));
      project("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript", "renderer");
      project("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust", "flow");
      const { frameworkOsEngineSourceOwnersV1, frameworkOsEnginePackageNameV1 } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts")).href);
      for (const owner of frameworkOsEngineSourceOwnersV1("distribution")) project(owner, frameworkOsEnginePackageNameV1(owner));
      const crate = (directory: string, id: string, extra: string): void => {
        project(directory, id);
        put(join(directory, "Cargo.toml"), `[package]\nname = "${id}"\nversion = "0.1.0"\n[package.metadata.component]\npackage = "semio:${id}"\n[package.metadata.semio]\ncomponent-kind = "plugin"\ndeployment-directory = "🧩️${id}"\n${extra}[[package.metadata.semio.playground]]\nvariant = "${id}"\nports = { react = 6100, wgpu = 6110 }\n`);
      };
      crate("hub", "hub", 'host = { landing = "home", shell = "studio" }\n');
      crate("alpha", "alpha", "");
      crate("beta", "beta", "");
      const { parseCatalogBuildContributionV1 } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts")).href);
      const library = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
      const corpus = JSON.parse(readFileSync(join(library, "⚡️caching/🧪️tests/🧊️preparation/🔣️.json"), "utf8")), contracts = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8")).generatorContracts;
      const targets = await cacheInternals.playgroundPreparationTargets(["hub/Cargo.toml", "alpha/Cargo.toml", "beta/Cargo.toml"], fixture, "owner", contracts, [parseCatalogBuildContributionV1({ ...corpus.catalog, publication: { ...corpus.catalog.publication, contributions: ["hub", "alpha", "beta"].map((root) => ({ ownerRoot: root, packageRoots: [root] })) } })], { preparationControl: { async advance() {} } });
      for (const profile of ["prototype", "release"]) {
        for (const renderer of ["react", "wgpu"] as const) {
          const prepare = targets[`prepare-hub-${renderer}-${profile}`];
          expect(prepare, `prepare-hub-${renderer}-${profile}`).toBeDefined();
          const outputs = prepare.outputs as string[], modules = outputs.filter((output) => output.includes("/modules/")).map((output) => output.slice(output.lastIndexOf("/") + 1));
          expect(outputs.filter((output) => output.includes("/components/")).length, `prepare-hub-${renderer}-${profile} builds one component`).toBe(1);
          expect(modules, `prepare-hub-${renderer}-${profile} builds only the host's boot closure`).toEqual(["🧩️hub"]);
        }
        const nativePrepare = targets[`prepare-hub-native-${profile}`];
        expect(nativePrepare, `prepare-hub-native-${profile}`).toBeDefined();
        expect((nativePrepare.outputs as string[]).filter((output) => output.includes("/components/")).length, `prepare-hub-native-${profile} builds the whole host session`).toBe(3);
        expect((targets[`prepare-alpha-react-${profile}`].outputs as string[]).filter((output) => output.includes("/modules/")).map((output) => output.slice(output.lastIndexOf("/") + 1)), "a non-host variant never inherits the host fan-out").toEqual(["🧩️alpha"]);
        for (const renderer of ["react", "wgpu"]) expect(targets[`activate-hub-${renderer}-${profile}`].dependsOn).toEqual([`prepare-hub-${renderer}-${profile}`]);
      }
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });
});
