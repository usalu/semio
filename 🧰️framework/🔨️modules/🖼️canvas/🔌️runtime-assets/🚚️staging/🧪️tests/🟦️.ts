import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { join, relative, resolve, posix } from "node:path";
import { Script, createContext } from "node:vm";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { canvasRuntimeAssetSources, runtimeAssetStageTrees } from "../🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("neutral Canvas assets have one stable managed tree per mode and no source-tree publication", () => {
  const root = resolve("/products"), validate = new Ajv({ strict: true }).compile(schema), database = new Database(":memory:"), roots = new Set<string>();
  try {
    for (const mode of fixture.modes) {
      expect(validate({ version: fixture.version, mode })).toBe(true);
      const trees = (runtimeAssetStageTrees as (...args: unknown[]) => ReturnType<typeof runtimeAssetStageTrees>)(root, mode);
      expect(trees.map(tree => relative(join(root, fixture.namespace, mode), tree.root).replaceAll("\\", "/"))).toEqual(fixture.trees);
      expect(trees.map(tree => tree.root)).toEqual((canvasRuntimeAssetSources as (...args: unknown[]) => ReturnType<typeof canvasRuntimeAssetSources>)(root, mode).map(tree => tree.root));
      for (const tree of trees) {
        roots.add(tree.root);
        expect((database.query("select instr(?1, ?2)=1 and instr(?1, 'dist')=0 confined").get(tree.root, root) as { confined: number }).confined).toBe(1);
        expect(tree.root.length).toBeLessThanOrEqual(224);
        expect(tree.files.every(file => join(tree.root, file.name).length <= 256)).toBe(true);
      }
    }
    expect(roots.size).toBe(fixture.modes.length * fixture.trees.length);
    for (const mode of fixture.invalidModes) {
      expect(validate({ version: 1, mode })).toBe(false);
      expect(() => (runtimeAssetStageTrees as (...args: unknown[]) => unknown)(root, mode)).toThrow();
    }
    for (const directory of fixture.invalidRoots) expect(() => (runtimeAssetStageTrees as (...args: unknown[]) => unknown)(directory, "prototype")).toThrow();
    console.log("[DEBUG] Canvas publication paths match independent SQLite containment and closed schema modes without opening storage");
  } finally { database.close(); }
});


test("neutral Canvas Nx cache outputs select the actual mode through the existing portable recipe", async () => {
  const repository = resolve(import.meta.dir, "../../../../../.."), projectRoot = "🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust", project = JSON.parse(readFileSync(join(repository, projectRoot, "📋️project.json"), "utf8")), require = createRequire(import.meta.url);
  const { targetBuildModes } = require("../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🟨️.mjs"), { neutralBuildTargetV1, resolveBuildTargetV1 } = await import("../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts"), { projectBuildTargetForNxV1 } = require("../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"), sdk = createRequire(join(repository, ".nx/installation/package.json")), typescript = require("typescript"), nxPath = sdk.resolve("nx/src/tasks-runner/utils"), nxSource = readFileSync(nxPath, "utf8"), syntax = typescript.createSourceFile(nxPath, nxSource, typescript.ScriptTarget.Latest, true), names = new Set(["interpolate", "_interpolate"]), functions: string[] = [];
  for (const statement of syntax.statements) if (typescript.isFunctionDeclaration(statement) && names.has(statement.name?.text)) functions.push(statement.getText(syntax));
  expect(functions.length).toBe(2);
  const context = createContext({ posix_1: posix }), interpolate = new Script("const replacementRegex = /{([\\s\\S]+?)}/g;" + functions.join("\n") + ";interpolate").runInContext(context) as (value: string, data: unknown) => string;
  const target = neutralBuildTargetV1(targetBuildModes("stage-runtime-assets", project.targets["stage-runtime-assets"])), projected = projectBuildTargetForNxV1(target);
  for (const mode of fixture.modes) {
    const selected = resolveBuildTargetV1(target, mode), actual = projected.outputs.map((output: string) => interpolate(output, { projectRoot, projectName: project.name, options: { ...projected.options, ...projected.configurations[mode] } }));
    expect(selected.execution.env?.SEMIO_BUILD_MODE).toBe(mode);
    expect(actual).toEqual([`.cache/b/p/m/canvas/${mode}/port-assets`, `.cache/b/p/m/canvas/${mode}/guest-assets`]);
    expect(actual.every((path: string) => !path.includes(mode === "prototype" ? "/release/" : "/prototype/"))).toBe(true);
  }
  expect(project.targets["stage-runtime-assets"].inputs).toContain("sharedGlobals");
  console.log("[DEBUG] Installed Nx isolated pure output interpolation agrees with mode-specific managed Canvas layout");
});
