import { expect, test } from "bun:test";
import { join, resolve, posix } from "node:path";
import { Script, createContext } from "node:vm";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import fixture from "@semio-tech/framework-canvas-ts/runtime-assets/staging/testing.json";

test("neutral Canvas Nx cache outputs select the actual mode through the existing portable recipe", async () => {
  const repository = findWorkspaceRoot(import.meta.dir), projectRoot = "🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust", project = JSON.parse(readFileSync(join(repository, projectRoot, "📋️project.json"), "utf8")), require = createRequire(import.meta.url);
  const { targetBuildModes } = require("../../../🎚️mode/🟨️.mjs"), { neutralBuildTargetV1, resolveBuildTargetV1 } = await import("../../🟦️.ts"), { projectBuildTargetForNxV1 } = require("../../../../🟨️.mjs"), sdk = createRequire(join(repository, ".nx/installation/package.json")), typescript = require("typescript"), nxPath = sdk.resolve("nx/src/tasks-runner/utils"), nxSource = readFileSync(nxPath, "utf8"), syntax = typescript.createSourceFile(nxPath, nxSource, typescript.ScriptTarget.Latest, true), names = new Set(["interpolate", "_interpolate"]), functions: string[] = [];
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
});
