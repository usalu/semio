import { playE2eInvocationPid } from "../../🔨️modules/🧩️runtime/🧪️e2e/🟦️.ts";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";

/** 🧪️ Proves the fresh release contract against the language-neutral fixture, independent argument parser and a third-party JSONC parser: one build invocation, no nested build invocation, every component produced by the release build driver in a cold generation. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: { playFreshBuildEnvironment: () => Readonly<Record<string, string>>; playFreshBuildArgs: (resources: { cpus: number; memoryBytes: number }) => readonly string[]; playFreshParallel: (resources: { cpus: number; memoryBytes: number }) => number; PLAY_FRESH_BUILD_TARGET: string }): Promise<void> {
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
    it("is exactly one selected-backend invocation of the complete release graph", () => {
      const resources = { cpus: fixture.buildMachine.cpus, memoryBytes: fixture.buildMachine.memoryGiB * 1024 ** 3 }, args = dependencies.playFreshBuildArgs(resources);
      for (const file of fixture.launch.files) {
        const entries = jsonc.parse(read(file)).configurations.filter((entry: any) => entry.name === fixture.launch.name);
        expect(entries, file).toHaveLength(1);
        expect(entries[0].command, file).toBe(`${fixture.launch.runner} ${args.join(" ")}`);
        expect(entries[0].env, file).toEqual(dependencies.playFreshBuildEnvironment());
      }
      const parsed = parse(args, { boolean: ["skip-cache", "skip-remote-cache"] });
      expect(parsed._).toEqual(["run", fixture.target]);
      expect(dependencies.PLAY_FRESH_BUILD_TARGET).toBe(fixture.target);
      expect(parsed["skip-cache"]).toBe(true);
      expect(parsed.output).toBe("stream");
      expect(args).toEqual(fixture.arguments);
      expect(parsed["skip-remote-cache"]).toBe(true);
      expect(parsed.excludeTaskDependencies).toBeUndefined();
      const project = JSON.parse(readFileSync(new URL("../../📋️project.json", import.meta.url), "utf8"));
      expect(Object.values(project.targets).filter((target: any) => / nx(?:\.js)? (?:run|run-many) /u.test(String(target.options?.command ?? "")))).toEqual([]);
      expect(readFileSync(new URL("../../🔨️modules/📦️site/📜️script.ts", import.meta.url), "utf8").includes("runOwnedNxInvocationV1"), "the site script never starts a nested backend invocation").toBe(false);
    });
    it("selects a cold release driver generation and bypasses cache reads", () => {
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
    it("routes every release driver through the repository compiler store owner and selects its cold generation by the driver's own variable", () => {
      const ts = require("typescript");
      expect(read(fixture.driver.ownerModule).includes(`export function ${fixture.driver.owner}(`)).toBe(true);
      for (const caller of fixture.driver.callers) {
        const text = read(caller), source = ts.createSourceFile(caller, text, ts.ScriptTarget.Latest, true), calls: any[] = [], names = new Set<string>();
        const visit = (node: any): void => { if (ts.isIdentifier(node)) names.add(node.text); if (ts.isCallExpression(node) && node.expression.getText(source) === fixture.driver.owner) calls.push(node); ts.forEachChild(node, visit); };
        visit(source);
        expect(names.has(fixture.driver.bypass), caller).toBe(false);
        expect(calls, caller).toHaveLength(1);
        const options = calls[0].arguments[2];
        expect(ts.isObjectLiteralExpression(options), caller).toBe(true);
        expect(options.properties.map((property: any) => property.name.getText(source)).filter((name: string) => fixture.driver.ownerArguments.includes(name)).sort(), caller).toEqual([...fixture.driver.ownerArguments].sort());
      }
      for (const file of fixture.launch.files) {
        const entries = jsonc.parse(read(file)).configurations.filter((entry: any) => entry.name === fixture.driver.freshLaunch);
        expect(entries, file).toHaveLength(1);
        expect(entries[0].env, file).toEqual({ [fixture.driver.generationEnvironment]: "fresh" });
      }
    });
    it("bounds parallel build tasks by cores and by memory", () => {
      for (const row of fixture.parallelRows) {
        const resources = { cpus: row.cpus, memoryBytes: row.memoryGiB * 1024 ** 3 };
        expect(dependencies.playFreshParallel(resources)).toBe(row.parallel);
        expect(parse(dependencies.playFreshBuildArgs(resources)).parallel).toBe(row.parallel);
      }
    });
    it("requires the shared backend-neutral E2E invocation identity", () => {
      const validate = new (require("ajv"))().compile(fixture.invocationPidSchema);
      for (const row of fixture.invocationPidRows) {
        const environment = row.value === null ? {} : { [fixture.invocationPidEnvironment]: row.value };
        const admitted = validate(row.value) && Number.isSafeInteger(Number(row.value));
        expect(admitted ? Number(row.value) : null).toBe(row.expected);
        if (row.expected === null) expect(() => playE2eInvocationPid(environment)).toThrow();
        else expect(playE2eInvocationPid(environment)).toBe(row.expected);
      }
      console.log("[DEBUG] Play E2E accepts shared build invocation IDs and refuses absent or malformed identities");
    });
    it("keeps Play and Pets development manifests independent of runner settings", () => {
      for (const path of fixture.developmentProjects) {
        const project = JSON.parse(read(path));
        for (const target of Object.values(project.targets) as any[]) {
          expect(Object.keys(target.options?.env ?? {}).filter(name => name.startsWith("NX_")), path).toEqual([]);
        }
      }
    });
    it("takes its Vite cache from the bounded product store the Vite owner issues, never from a caller-chosen directory", async () => {
      const ts = require("typescript"), text = read(fixture.vite.config), source = ts.createSourceFile("config.ts", text, ts.ScriptTarget.Latest, true), calls: any[] = [];
      const visit = (node: any): void => { if (ts.isCallExpression(node) && node.expression.getText(source) === fixture.vite.owner) calls.push(node); ts.forEachChild(node, visit); };
      visit(source);
      expect(calls).toHaveLength(1);
      expect(calls[0].arguments.map((argument: any) => argument.getText(source))[0]).toBe(fixture.vite.workspaceArgument);
      expect(calls[0].arguments).toHaveLength(2);
      const { viteCacheDirectory } = await import(new URL(fixture.vite.module, repository).href), { repositoryCacheBudgetV1 } = await import(new URL(fixture.vite.budgetModule, repository).href);
      const root = resolve(new URL(".", repository).pathname), issued = join(repositoryCacheBudgetV1(root).partitions.products.directory, ...fixture.vite.issued);
      for (const mode of fixture.vite.modes) expect(viteCacheDirectory(root, { SEMIO_BUILD_MODE: mode, SEMIO_VITE_CACHE_DIRECTORY: issued })).toBe(issued);
      expect(() => viteCacheDirectory(root, { SEMIO_BUILD_MODE: "release" })).toThrow(fixture.vite.unissued);
      expect(() => viteCacheDirectory(root, { SEMIO_BUILD_MODE: "release", SEMIO_VITE_CACHE_DIRECTORY: resolve("shared-vite") })).toThrow();
      expect(() => viteCacheDirectory(root, { SEMIO_BUILD_MODE: "ship", SEMIO_VITE_CACHE_DIRECTORY: issued })).toThrow();
    });
  });
}
