import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import executionSchema from "../../../🔌️nx-plugin/🏃️execution/🧬️schema/🔣️.json";
import vectors from "../../🧫️fixtures/🐹️executable-ownership/🔣️.json";

test("repository Go executable integration has one Specific owner and leaves the General process independent", () => {
  const root = resolve(import.meta.dir, "../../../../../../../..");
  const script = readFileSync(resolve(root, vectors.general.script), "utf8");
  const parsed = ts.createSourceFile(vectors.general.script, script, ts.ScriptTarget.Latest, true);
  const strings: string[] = [];
  const visit = (node: ts.Node): void => { if (ts.isStringLiteral(node)) strings.push(node.text); ts.forEachChild(node, visit); };
  visit(parsed);
  expect(strings.includes(vectors.selector)).toBe(false);
  expect(JSON.parse(readFileSync(resolve(root, vectors.general.project), "utf8")).targets[vectors.target]).toBeUndefined();
  const project = JSON.parse(readFileSync(resolve(root, vectors.repositoryProject), "utf8")), validate = new Ajv({ strict: true }).compile(executionSchema);
  for (const [target, expected] of Object.entries(vectors.executionContracts)) {
    const actual = project.targets[target].metadata.semio.execution;
    expect(validate(actual)).toBe(true);
    expect(actual).toEqual(expected);
    expect(project.targets[target].options.command.startsWith("bun ./📜️script.ts test ")).toBe(true);
  }
  expect(existsSync(resolve(root, vectors.general.source))).toBe(false);
  expect(existsSync(resolve(root, vectors.repository.source))).toBe(true);
  const source = readFileSync(resolve(root, vectors.repository.source), "utf8");
  const imports = ts.preProcessFile(source, true, true).importedFiles.map(row => row.fileName);
  expect(imports.some(path => path.includes("🏃️process/📦️packages/🟦️typescript/🧭️routing"))).toBe(true);
  const fixture = JSON.parse(readFileSync(resolve(root, vectors.repository.fixture), "utf8"));
  expect(fixture.modes).toEqual(vectors.modes);
  expect(fixture.owners.map((row: { owner: string }) => row.owner)).toEqual(vectors.owners);
  expect(fixture.protocol).toBe(vectors.protocol);
  expect(fixture.serverName).toBe(vectors.serverName);
  expect(fixture.health).toBe(vectors.health);
  console.log("[DEBUG] Actual TypeScript owner census and original repository Go/MCP/coordinator fixture agree with the deletion ownership vectors");
});
