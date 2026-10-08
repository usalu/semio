import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, relative } from "node:path";

/** 🧪️ Proves fresh release isolation against the language-neutral contract and Nx's argument parser. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: { playFreshBuildEnvironment: (workspace: string, generation: string, inherited: Readonly<Record<string, string | undefined>>) => Record<string, string | undefined>; playViteCacheDirectory: (fallback: string, environment: Readonly<Record<string, string | undefined>>) => string; PLAY_FRESH_BUILD_ARGS: readonly string[] }): Promise<void> {
  const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const parse = createRequire(import.meta.url)("yargs-parser");
  const { describe, expect, it } = vitest;
  describe("fresh Play release", () => {
    it("refreshes the canonical catalog after descriptor preparation and before bundling", () => {
      const project = JSON.parse(readFileSync(new URL("../../📋️project.json", import.meta.url), "utf8"));
      for (let index = 1; index < fixture.publicationOrder.length; index++) expect(project.targets[fixture.publicationOrder[index]]?.dependsOn).toContain(fixture.publicationOrder[index - 1]);
      expect(project.targets[fixture.publicationOrder[0]]?.cache).toBe(false);
    });
    it("refuses stale catalog owners before publishing the final release catalog", () => {
      const ts = createRequire(import.meta.url)("typescript");
      const text = readFileSync(new URL("../../🔨️modules/📦️site/📜️script.ts", import.meta.url), "utf8"), source = ts.createSourceFile("script.ts", text, ts.ScriptTarget.Latest, true);
      const calls: any[] = [];
      const visit = (node: any): void => { if (ts.isCallExpression(node)) calls.push(node); ts.forEachChild(node, visit); };
      visit(source);
      const admission = calls.find(node => node.expression.getText(source) === "renderCatalogFiles"), publication = calls.find(node => node.expression.getText(source).includes("new GenerateScript(") && node.expression.getText(source).endsWith(".run"));
      expect(admission?.arguments[2]?.text).toBe(fixture.catalogAdmission);
      expect(admission?.getStart(source)).toBeLessThan(publication?.getStart(source));
    });
    it("isolates compiler and graph storage while preserving unrelated caller configuration", () => {
      const inherited = { PATH: "toolchain", SEMIO_TICKET_DIR: "ticket", CARGO_TARGET_DIR: "shared-target", CARGO_BUILD_BUILD_DIR: "shared-build", NX_CACHE_DIRECTORY: "shared-nx", NX_WORKSPACE_DATA_DIRECTORY: "shared-graph", ...Object.fromEntries(fixture.discard.map((key: string) => [key, "outer-task"])) };
      const environments = fixture.generations.map((generation: string) => dependencies.playFreshBuildEnvironment("/workspace", join("/workspace", "isolated", generation), inherited));
      for (let index = 0; index < environments.length; index++) {
        const environment = environments[index];
        for (const [key, path] of Object.entries(fixture.storage)) expect(relative(join("/workspace", "isolated", fixture.generations[index]), environment[key])).toBe(String(path).split("/").join(process.platform === "win32" ? "\\" : "/"));
        for (const [key, value] of Object.entries(fixture.isolation)) expect(environment[key]).toBe(value);
        for (const key of fixture.discard) expect(environment[key]).toBeUndefined();
        expect(environment.PATH).toBe(inherited.PATH);
        expect(environment.SEMIO_TICKET_DIR).toBe(inherited.SEMIO_TICKET_DIR);
      }
      expect(environments[0].CARGO_BUILD_BUILD_DIR).not.toBe(environments[1].CARGO_BUILD_BUILD_DIR);
      expect(inherited.CARGO_TARGET_DIR).toBe("shared-target");
    });
    it("isolates Vite storage inside the current invocation's graph storage", () => {
      const generation = join("/workspace", "isolated", fixture.generations[0]), environment = dependencies.playFreshBuildEnvironment("/workspace", generation, {});
      expect(relative(generation, dependencies.playViteCacheDirectory("shared-vite", environment))).toBe(fixture.viteStorage.split("/").join(process.platform === "win32" ? "\\" : "/"));
      expect(dependencies.playViteCacheDirectory("shared-vite", {})).toBe("shared-vite");
    });
    it("requests the complete Nx build graph with task and remote cache bypass", () => {
      const parsed = parse(dependencies.PLAY_FRESH_BUILD_ARGS, { boolean: ["skip-nx-cache", "skip-remote-cache"] });
      expect(parsed._).toEqual(["run", fixture.target]);
      expect(parsed["skip-nx-cache"]).toBe(true);
      expect(parsed["skip-remote-cache"]).toBe(true);
      expect(parsed.parallel).toBe(fixture.parallel);
      expect(parsed.excludeTaskDependencies).toBeUndefined();
    });
  });
}
