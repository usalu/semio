import { inspectRustCompileReferences } from "@semio-tech/framework/compiler/syntax/rust";
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { join as oracleJoin, normalize as oracleNormalize } from "pathe";
import { rustSourceDirectionEdges, rustSourceTargets } from "../../🕸️dependencies/🧭️direction/🦀️source/🟦️.ts";
import { WORKSPACE_ROOT } from "../../🗂️workspaces/📍️checkout-root/🟦️.ts";

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧱️norm-definition-ownership/🔣️.json"), "utf8")) as Readonly<{ plugin: string; contract: string; project: string; source: string; mount: string; definition: string; retired: string; forbiddenPrefix: string; expectedEdges: ReturnType<typeof rustSourceDirectionEdges> }>;
const plugin = join(WORKSPACE_ROOT, fixture.plugin);

test("the language-neutral definition ownership corpus is closed", () => {
  const references = inspectRustCompileReferences(readFileSync(join(plugin, fixture.source), "utf8"));
  const targets = rustSourceTargets(fixture.source, references.filter(row => row.kind === "path" && row.path === fixture.mount));
  expect(targets.map(row => row.to)).toEqual([fixture.definition]);
  expect(targets.some(row => row.to === fixture.retired)).toBe(false);
});

test("the real contract resolves its assembler outside removable artifact inputs", () => {
  const source = readFileSync(join(plugin, fixture.source), "utf8");
  const references = inspectRustCompileReferences(source).filter(row => row.kind === "path" && (row.path === fixture.mount || row.path.includes(fixture.forbiddenPrefix)));
  const targets = rustSourceTargets(fixture.source, references);
  expect(targets.map(row => row.to)).toEqual([fixture.definition]);
  expect(targets.map(row => row.to)).toEqual(references.map(row => oracleNormalize(oracleJoin(fixture.contract, row.path))));
  expect(rustSourceDirectionEdges(fixture.source, references, [{ name: "norm-contract-no-artifact-input", severity: "error", from: { path: [`^${fixture.contract}/`] }, to: { path: ["^🗿️artifacts/"] } }])).toEqual(fixture.expectedEdges);
  expect(existsSync(join(plugin, fixture.retired))).toBe(false);
  expect(existsSync(join(plugin, fixture.definition))).toBe(true);
});

test("the assembler and declared owner inputs contain no artifact implementation", () => {
  const definition = readFileSync(join(plugin, fixture.definition), "utf8");
  expect(inspectRustCompileReferences(definition)).toEqual([]);
  const project = JSON.parse(readFileSync(join(plugin, fixture.project), "utf8"));
  expect(project.namedInputs.nativeSources.every((path: string) => !path.includes("/🗿️artifacts/"))).toBe(true);
  expect(definition).toContain("pub fn assemble_definition");
});
