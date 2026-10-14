import { expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
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
    for (const row of fixture.editorCommands) for (const language of ["en", "de"] as const) {
      const expected = `${row.labels[language]} [${language.toUpperCase()}]`;
      const launch = document.configurations.find((value: unknown) => typeof value === "object" && value !== null && (value as { name?: string }).name === expected);
      expect(launch, expected).toMatchObject({ type: "node-terminal", request: "launch", command: `bun repo ${row.args}`, cwd: "${workspaceFolder}", presentation: { group: row.group } });
    }
  }
});
