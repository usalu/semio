import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧪️ Proves the fresh release contract against the language-neutral fixture, Nx's argument parser and a third-party JSONC parser: one Nx invocation, no nested Nx, every component produced by the release build driver in a cold generation. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: { playFreshBuildEnvironment: () => Readonly<Record<string, string>>; playViteCacheDirectory: (fallback: string, environment: Readonly<Record<string, string | undefined>>) => string; playFreshBuildArgs: (resources: { cpus: number; memoryBytes: number }) => readonly string[]; playFreshParallel: (resources: { cpus: number; memoryBytes: number }) => number; PLAY_FRESH_BUILD_TARGET: string }): Promise<void> {
  const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const require = createRequire(import.meta.url), parse = require("yargs-parser"), jsonc = require("jsonc-parser");
  const repository = new URL("../../../../", import.meta.url), read = (path: string): string => readFileSync(new URL(path, repository), "utf8");
  const { describe, expect, it } = vitest;
  describe("fresh Play release", () => {
    it("is one uncached release driver task whose jobs bundle before any component links and assemble pages after every module", () => {
      const project = JSON.parse(readFileSync(new URL("../../📋️project.json", import.meta.url), "utf8")), build = project.targets[fixture.release.target];
      for (const removed of fixture.release.removedTargets) expect(project.targets[removed], removed).toBeUndefined();
      expect(build.cache).toBe(false);
      expect(build.metadata.semio.orchestrator).toBe(true);
      expect(build.options.command).toBe(fixture.release.command);
      const pipeline = build.metadata.semio.release, job = (id: string) => pipeline.jobs.find((row: any) => row.id === id);
      expect(pipeline.prerequisites).toEqual(fixture.release.prerequisites);
      expect(job("bundle").after).toEqual(fixture.release.bundleAfter);
      expect(job("bundle").after.some((name: string) => name.startsWith("components:"))).toBe(false);
      expect(job("pages").after).toEqual(fixture.release.pagesAfter);
      expect(pipeline.jobs.filter((row: any) => row.lane !== undefined).map((row: any) => row.id)).toEqual(fixture.release.laneJobs);
    });
    it("refuses stale catalog owners in the release gate instead of rewriting the catalog during a build", () => {
      const ts = require("typescript");
      const text = readFileSync(new URL("../../🔨️modules/📦️site/📜️script.ts", import.meta.url), "utf8"), source = ts.createSourceFile("script.ts", text, ts.ScriptTarget.Latest, true);
      const calls: any[] = [];
      const visit = (node: any): void => { if (ts.isCallExpression(node)) calls.push(node); ts.forEachChild(node, visit); };
      visit(source);
      expect(calls.find(node => node.expression.getText(source) === "renderCatalogFiles")?.arguments[2]?.text).toBe(fixture.catalogAdmission);
      expect(calls.some(node => node.expression.getText(source).includes("GenerateScript"))).toBe(false);
      const project = JSON.parse(readFileSync(new URL("../../📋️project.json", import.meta.url), "utf8"));
      expect(project.targets[fixture.release.verifyTarget].options.command).toBe(fixture.release.verifyCommand);
    });
    it("is exactly one Nx invocation of the complete release graph, launched without any wrapping Nx task", () => {
      const resources = { cpus: fixture.buildMachine.cpus, memoryBytes: fixture.buildMachine.memoryGiB * 1024 ** 3 }, args = dependencies.playFreshBuildArgs(resources);
      for (const file of fixture.launch.files) {
        const entries = jsonc.parse(read(file)).configurations.filter((entry: any) => entry.name === fixture.launch.name);
        expect(entries, file).toHaveLength(1);
        expect(entries[0].command, file).toBe(`${fixture.launch.runner} ${args.join(" ")}`);
        expect(entries[0].env, file).toEqual(dependencies.playFreshBuildEnvironment());
      }
      const parsed = parse(args, { boolean: ["skip-nx-cache", "skip-remote-cache"] });
      expect(parsed._).toEqual(["run", fixture.target]);
      expect(dependencies.PLAY_FRESH_BUILD_TARGET).toBe(fixture.target);
      expect(parsed["skip-nx-cache"]).toBe(true);
      expect(parsed["skip-remote-cache"]).toBe(true);
      expect(parsed.excludeTaskDependencies).toBeUndefined();
      const project = JSON.parse(readFileSync(new URL("../../📋️project.json", import.meta.url), "utf8"));
      expect(Object.values(project.targets).filter((target: any) => / nx(?:\.js)? (?:run|run-many) /u.test(String(target.options?.command ?? "")))).toEqual([]);
      expect(readFileSync(new URL("../../🔨️modules/📦️site/📜️script.ts", import.meta.url), "utf8").includes("runOwnedNxInvocationV1"), "the site script never starts an Nx invocation").toBe(false);
    });
    it("selects a cold release driver generation and bypasses every Nx cache read", () => {
      expect(dependencies.playFreshBuildEnvironment()).toEqual(fixture.environment);
      expect(dependencies.playFreshBuildEnvironment()[fixture.driver.generationEnvironment]).toBe("fresh");
      const driver = JSON.parse(read("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧬️schema/🔣️.json")).definitions.Policy.const;
      expect(driver.generationEnvironment).toBe(fixture.driver.generationEnvironment);
      expect(driver.generations).toContain("fresh");
      expect(driver.generationPrefix).toBe(fixture.driver.generationPrefix);
      expect(driver.stores).toEqual(fixture.driver.stores);
    });
    it("keeps the workspace release driver, which no task deadline wraps, beside the play driver", () => {
      const root = JSON.parse(read("📋️project.json")), release = root.targets[fixture.driver.target];
      expect(release?.cache).toBe(false);
      expect(release?.options?.command).toBe(fixture.driver.command);
      expect(release?.metadata?.semio?.orchestrator).toBe(true);
      expect(Array.isArray(release?.metadata?.semio?.release?.jobs)).toBe(true);
    });
    it("bounds parallel Nx tasks by cores and by memory", () => {
      for (const row of fixture.parallelRows) {
        const resources = { cpus: row.cpus, memoryBytes: row.memoryGiB * 1024 ** 3 };
        expect(dependencies.playFreshParallel(resources)).toBe(row.parallel);
        expect(parse(dependencies.playFreshBuildArgs(resources)).parallel).toBe(row.parallel);
      }
    });
    it("isolates Vite storage inside an explicitly selected invocation cache", () => {
      const release = dependencies.playViteCacheDirectory("shared-vite", { NX_CACHE_DIRECTORY: "/cache", SEMIO_BUILD_MODE: "release" });
      expect(release.startsWith(join("/cache", fixture.viteStorage))).toBe(true);
      expect(dependencies.playViteCacheDirectory("shared-vite", { NX_CACHE_DIRECTORY: "/cache", SEMIO_BUILD_MODE: "prototype" })).not.toBe(release);
      expect(dependencies.playViteCacheDirectory("shared-vite", { SEMIO_BUILD_MODE: "prototype" }).startsWith("shared-vite")).toBe(true);
      expect(() => dependencies.playViteCacheDirectory("shared-vite", { SEMIO_BUILD_MODE: "ship" })).toThrow();
    });
  });
}
