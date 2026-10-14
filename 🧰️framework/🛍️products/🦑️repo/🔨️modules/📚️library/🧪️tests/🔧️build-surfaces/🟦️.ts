import { expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import {parseArgsStringToArgv} from "string-argv";
import { parseBuildInvocationV1 } from "../../🏗️build/🧭️invocation/🟦️.ts";
import { validateBuildTaskArgumentsV1 } from "../../🏗️build/🧭️selection/🟦️.ts";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import fixture from "../🧫️fixtures/🔧️build-surfaces/🔣️.json";
import schema from "../../🧬️schema/🔧️build-surfaces/🔣️.json";
import projectSchema from "../../🏗️build/🧬️schema/📋️project/🔣️.json";
import { nativeHostContractLaws } from "../../🎮️playground/🖥️native-host/🧪️tests/🟦️.ts";

const root = findWorkspaceRoot(import.meta.dir);
const read = (path: string): string => readFileSync(join(root, path), "utf8");

test("portable developer entry points agree with the independent schema oracle", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const manifest = JSON.parse(read("package.json"));
  for (const [name, expected] of Object.entries(fixture.scripts)) expect(manifest.scripts[name], name).toBe(expected);
  expect(manifest.scripts.nx).toBeUndefined();
  expect(manifest.nx).toBeUndefined();
  const config = ts.parseConfigFileTextToJson("devcontainer.json", read(".devcontainer/devcontainer.json"));
  expect(config.error).toBeUndefined();
  expect(config.config.postCreateCommand).toEqual(fixture.postCreateCommand);
});

test("workspace context uses the neutral execution identity", () => {
  for (const row of fixture.contexts) expect(getWorkspaceRoot(row.environment)).toBe(resolve(row.expected));
});

test("root orchestration, editor settings and CI use neutral build commands", () => {
  const source = read("📜️script.ts");
  expect(/\["nx",\s*"run(?:-many)?"/u.test(source)).toBe(false);
  expect(source.includes("NX_TASK_TARGET_PROJECT")).toBe(false);
  expect(source).toContain('.registerLazy("repo"');
  for (const path of [".vscode/settings.json", ".devcontainer/devcontainer.json", ".devcontainer/Dockerfile"]) expect(read(path), path).not.toMatch(/\bNX_[A-Z_]+/u);
  expect(read(".github/workflows/architecture-quiz.yml").includes(["bun", "nx"].join(" "))).toBe(false);
});

test("project declarations expose neutral executors through the owned schema", () => {
  const validate = new Ajv({ strict: true }).compile(projectSchema);
  expect(validate({ name: "sample", targets: { build: { executor: "repo:script", options: { command: "bun ./📜️script.ts build" } }, noop: { executor: "repo:noop" } } })).toBe(true);
  expect(validate({ name: "sample", targets: { build: { executor: "nx:run-commands" } } })).toBe(false);
  const project = JSON.parse(read("📋️project.json"));
  expect(validate(project), JSON.stringify(validate.errors)).toBe(true);
  expect(project.$schema).toContain("🏗️build/🧬️schema/📋️project/🔣️.json");
});

test("native executable producers admit portable project declarations", () => nativeHostContractLaws(root));

test("editor entry points expose every build infrastructure command in both languages", () => {
  for (const file of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) {
    const document = JSON.parse(read(file));
    for (const input of fixture.editorInputs) expect(document.inputs.find((row: { id?: string }) => row.id === input.id)).toEqual(input);
    for (const row of fixture.editorCommands) for (const language of ["en", "de"] as const) {
      const expected = `${row.labels[language]} [${language.toUpperCase()}]`;
      const launch = document.configurations.find((value: unknown) => typeof value === "object" && value !== null && (value as { name?: string }).name === expected);
      expect(launch, expected).toMatchObject({ type: "node-terminal", request: "launch", command: `bun repo ${row.args}`, cwd: "${workspaceFolder}", presentation: { group: row.group } });
    }
  }
});

test("application task environments use neutral execution controls", () => {
  for (const path of fixture.environmentSources) expect(read(path), path).not.toMatch(/\bNX_[A-Z_]+/u);
});

test("editor task arguments cross the neutral argument boundary", () => {
  const commands = (value: unknown): Record<string, unknown>[] => Array.isArray(value) ? value.flatMap(commands) : value && typeof value === "object" ? [...(typeof (value as Record<string, unknown>).command === "string" ? [value as Record<string, unknown>] : []), ...Object.values(value).flatMap(commands)] : [];
  for (const file of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) for (const row of commands(JSON.parse(read(file)))) {
    if (typeof row !== "object" || row === null || typeof row.command !== "string") continue;
    expect(row.command, String(row.name)).not.toMatch(/(?:^|\s)nx\s+(?:run|run-many|exec|show|graph|watch|affected)\b/u);
    if (!row.command.startsWith("bun repo ")) continue;
    const parsed = parseBuildInvocationV1(parseArgsStringToArgv(row.command.replaceAll("${input:repoBuildTool}", "nx")).slice(2), {}).args, boundary = parsed.findIndex(value => /^(?:\d*[<>]|&&|\|\||;)$/u.test(value));
    const args = boundary < 0 ? parsed : parsed.slice(0, boundary);
    if (["run", "run-many", "affected", "exec", "watch"].includes(args[0]!)) expect(() => validateBuildTaskArgumentsV1(args), String(row.name)).not.toThrow();
  }
});

test("native orchestration and rebuild declarations use neutral build commands", () => {
  const manifest = JSON.parse(read("package.json"));
  for (const command of Object.values(manifest.scripts)) expect(command).not.toMatch(/(?:^|\s)(?:bun\s+(?:(?:run|x)\s+)?)?nx\s/u);
  for (const path of fixture.commandSources) expect(read(path), path).not.toMatch(/\bbun\s+nx\s|--skip-nx-cache|--excludeTaskDependencies|--outputStyle|"nx"\s*,|node_modules\/nx\/(?:dist\/)?bin\/nx\.js/u);
});

test("Bazel configuration files have explicit repository filename authority", () => {
  const taxonomy = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"));
  for (const row of fixture.fixedBuildFiles) {
    expect(taxonomy.fixedFilenameContracts[row.id], row.path).toMatchObject({ pathPattern: row.path, authority: "Bazel", scope: { kind: "repository-root" } });
    expect(read(row.path).length).toBeGreaterThan(0);
  }
});
