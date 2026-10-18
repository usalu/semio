/** 🧪️ Reload law of the build inference module: one revision token loads one complete component graph, a revisited revision keeps its components, and no component of an older revision survives a reload. The scenario lives in `./🔣️.json` as language-agnostic data; Bun and Node replay it and must agree with the corpus and with each other, and the actual component graph is audited against the independent TypeScript syntax tree. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import { readComponentRoster, registryKey } from "../🪢️revision-graph/🟨️.mjs";
import manifest from "../🗂️components/🔣️.json" with { type: "json" };
import manifestSchema from "../🗂️components/🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import { inferenceSourceFilesV1, readInferenceSourceV1 } from "./📖️source/🟦️.ts";

const inference = resolve(import.meta.dir, "..");
const kernel = pathToFileURL(join(inference, "🪢️revision-graph/🟨️.mjs")).href;

interface StepObservation { readonly id: string; readonly accepted: boolean; readonly render?: string; readonly error?: string; readonly wired: readonly string[]; readonly evaluations: Readonly<Record<string, number>>; readonly scope?: number; readonly shared?: boolean }
interface Observation { readonly steps: readonly StepObservation[]; readonly rosters: readonly { readonly id: string; readonly accepted: boolean }[] }

/** 📜️ The independent replay: writes the abstract components as ES modules, loads each step's revision through the kernel and prints what it observed as JSON. */
const driver = `
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { createRevisionLoader, readComponentRoster } from ${JSON.stringify(kernel)};
const [scenarioPath, directory] = process.argv.slice(2), scenario = JSON.parse(readFileSync(scenarioPath, "utf8"));
const key = (name) => name.replace(/-([a-z0-9])/g, (_, character) => character.toUpperCase());
const source = (component, value) => [
  "export const component = " + JSON.stringify(component.name) + ";",
  "const evaluations = (globalThis.__graphEvaluations ??= {});",
  "evaluations[component] = (evaluations[component] ?? 0) + 1;",
  "const key = " + key.toString() + ";",
  "const needs = " + JSON.stringify(component.needs) + ";",
  "let registry;",
  "export function wire(wired) {",
  "  if (globalThis.__graphRefuse === component) throw Error('wire refused: ' + component);",
  "  (globalThis.__graphWires ??= []).push(component);",
  "  registry = wired;",
  "}",
  "export function render() { return " + JSON.stringify(value) + " + (needs.length ? '(' + needs.map((need) => registry[key(need)].render()).join(',') + ')' : ''); }",
].join("\\n");
const roster = readComponentRoster({ version: 1, components: scenario.components.map((component) => ({ name: component.name, path: "./components/" + component.name + ".mjs", needs: component.needs })) });
mkdirSync(join(directory, "components"), { recursive: true });
const values = Object.fromEntries(scenario.components.map((component) => [component.name, component.value]));
const loader = createRevisionLoader({ roster, load: (specifier) => import(specifier), services: {}, register: () => undefined, url: import.meta.url });
const scopes = [], steps = [];
for (const step of scenario.steps) {
  Object.assign(values, step.set ?? {});
  for (const component of scenario.components) writeFileSync(join(directory, "components", component.name + ".mjs"), source(component, values[component.name]));
  globalThis.__graphRefuse = step.refuseWire ?? null;
  const before = (globalThis.__graphWires ??= []).length;
  const revision = createHash("sha256").update(step.revision).digest("hex");
  let outcome;
  try {
    const loaded = await Promise.all(Array.from({ length: step.concurrent ?? 1 }, () => loader.load(revision)));
    const scope = loaded[0];
    if (!scopes.includes(scope)) scopes.push(scope);
    outcome = { accepted: true, render: scope.modules[scenario.top].render(), scope: scopes.indexOf(scope), ...(step.concurrent ? { shared: loaded.every((row) => row === scope) } : {}) };
  } catch (error) { outcome = { accepted: false, error: error.message }; }
  steps.push({ id: step.id, ...outcome, wired: globalThis.__graphWires.slice(before), evaluations: { ...globalThis.__graphEvaluations } });
}
const rosters = scenario.rosters.map((row) => { try { readComponentRoster({ version: 1, components: row.components }); return { id: row.id, accepted: true }; } catch { return { id: row.id, accepted: false }; } });
console.log(JSON.stringify({ steps, rosters }));
`;

/** 🏃️ Replays the corpus under one runtime in a fresh directory and returns its observation. */
function replay(runtime: string): Observation {
  const directory = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "inference-reload-"));
  try {
    writeFileSync(join(directory, "driver.mjs"), driver);
    writeFileSync(join(directory, "scenario.json"), JSON.stringify(corpus));
    const child = spawnSync(runtime, [join(directory, "driver.mjs"), join(directory, "scenario.json"), directory], { encoding: "utf8", timeout: 60000 });
    expect(child.status, child.stdout + child.stderr).toBe(0);
    return JSON.parse(child.stdout.trim().split("\n").at(-1)!);
  } finally { rmSync(directory, { recursive: true, force: true }); }
}

test("reload corpus and roster manifest satisfy their closed schemas", () => {
  expect(new Ajv({ strict: true }).validate(corpusSchema, corpus)).toBe(true);
  const validate = new Ajv({ strict: true }).compile(manifestSchema);
  expect(validate(manifest), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(corpus.steps.map((step) => step.id)).size).toBe(corpus.steps.length);
  expect(new Set(corpus.rosters.map((row) => row.id)).size).toBe(corpus.rosters.length);
});

test("roster admission agrees between the kernel and the closed schema", () => {
  const validate = new Ajv({ strict: true }).compile(manifestSchema);
  for (const row of corpus.rosters) {
    const candidate = { version: 1, components: row.components };
    expect(validate(candidate), row.id).toBe(row.schema);
    let accepted = true;
    try { readComponentRoster(candidate); } catch { accepted = false; }
    expect(accepted, row.id).toBe(row.accepted);
  }
});

test("a revisioned reload reproduces the corpus under Bun and Node alike", () => {
  const bun = replay(process.execPath), node = replay("node");
  expect(node).toEqual(bun);
  expect(bun.rosters).toEqual(corpus.rosters.map((row) => ({ id: row.id, accepted: row.accepted })));
  expect(bun.steps.length).toBe(corpus.steps.length);
  for (const [index, step] of corpus.steps.entries()) {
    const actual = bun.steps[index]!, expected = step.expect as Partial<StepObservation>;
    expect(actual.id).toBe(step.id);
    expect(actual.accepted, step.id).toBe(expected.accepted);
    expect(actual.wired, step.id).toEqual(expected.wired);
    expect(actual.evaluations, step.id).toEqual(expected.evaluations);
    expect(actual.render, step.id).toBe(expected.render);
    expect(actual.error, step.id).toBe(expected.error);
    expect(actual.scope, step.id).toBe(expected.scope);
    expect(actual.shared, step.id).toBe(expected.shared);
  }
});

/** 🧪️ Parses one component and reports its roster name, exports, wiring reads and import specifiers through the TypeScript syntax tree. */
function audit(name: string) {
  const text = readInferenceSourceV1(name), tree = ts.createSourceFile(`${name}.mjs`, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const imports: string[] = [], reads = new Set<string>();
  let declared: string | undefined, wire: ts.FunctionDeclaration | undefined;
  for (const node of tree.statements) {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier)) imports.push(node.moduleSpecifier.text);
    if (ts.isFunctionDeclaration(node) && node.name?.text === "wire") wire = node;
    if (ts.isVariableStatement(node)) for (const item of node.declarationList.declarations) if (ts.isIdentifier(item.name) && item.name.text === "component" && item.initializer && ts.isStringLiteral(item.initializer)) declared = item.initializer.text;
  }
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteralLike(node.arguments[0])) imports.push(node.arguments[0].text);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  const parameter = wire?.parameters[0]?.name.getText(tree);
  const collect = (node: ts.Node): void => {
    if (parameter && ts.isPropertyAccessExpression(node) && node.expression.getText(tree) === parameter) reads.add(node.name.text);
    if (parameter && ts.isBinaryExpression(node) && node.operatorToken.kind === ts.SyntaxKind.EqualsToken && node.right.getText(tree) === parameter && ts.isObjectLiteralExpression(node.left)) for (const property of node.left.properties) if (ts.isShorthandPropertyAssignment(property)) reads.add(property.name.text);
    ts.forEachChild(node, collect);
  };
  if (wire?.body) collect(wire.body);
  return { declared, hasWire: wire !== undefined, imports, reads };
}

test("the actual components name themselves, import no sibling and read exactly the registry entries their needs declare", () => {
  const services = new Set(["services", "revisionIdentity", "register"]);
  const roster = readComponentRoster(manifest), directories = roster.map((entry) => resolve(inference, dirname(entry.path)));
  expect(new Set(roster.map((entry) => entry.name)).size).toBe(roster.length);
  expect(inferenceSourceFilesV1().filter((row) => row.name !== "root" && row.name !== "revision-graph" && row.name !== "components").map((row) => row.name)).toEqual(roster.map((entry) => entry.name));
  for (const entry of roster) {
    const row = audit(entry.name);
    expect(row.declared, entry.name).toBe(entry.name);
    expect(row.hasWire, entry.name).toBe(true);
    const base = resolve(inference, entry.path);
    for (const specifier of row.imports) if (specifier.startsWith(".")) expect(directories.filter((directory) => resolve(dirname(base), specifier).startsWith(directory + "/") && directory !== dirname(base)), `${entry.name} imports ${specifier}`).toEqual([]);
    const needed = new Set(entry.needs.map(registryKey));
    for (const read of row.reads) expect(needed.has(read) || services.has(read), `${entry.name} reads ${read}`).toBe(true);
    for (const need of needed) expect(row.reads.has(need), `${entry.name} declares unused need ${need}`).toBe(true);
  }
});

test("the root statically imports exactly the rostered components and the actual graph loads", async () => {
  const root = ts.createSourceFile("root.mjs", readInferenceSourceV1("root"), ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
  const specifiers = root.statements.filter(ts.isImportDeclaration).map((node) => (ts.isStringLiteral(node.moduleSpecifier) ? node.moduleSpecifier.text : ""));
  for (const entry of manifest.components) expect(specifiers.filter((specifier) => specifier === entry.path), entry.name).toHaveLength(1);
  expect(specifiers.filter((specifier) => specifier.endsWith("/🟨️.mjs") && specifier.startsWith("./")).length).toBe(manifest.components.length + 1);
  const actual = await import("../🟨️.mjs");
  expect(Object.keys(actual.cacheInternals).length).toBeGreaterThan(0);
  expect(actual.currentImplementationRevisionV1()).toMatch(/^[a-f0-9]{64}$/u);
});
