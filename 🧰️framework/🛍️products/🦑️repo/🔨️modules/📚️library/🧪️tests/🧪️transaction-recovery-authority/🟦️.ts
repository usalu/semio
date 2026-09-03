import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { getNodeValue, parseTree } from "jsonc-parser";
import ts from "typescript";

const library = resolve(import.meta.dir, "../.."), sourcePath = join(library, "🧹️normalization/🟦️.ts"), source = readFileSync(sourcePath, "utf8");
const tree = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧪️🪪️🐸️transaction-recovery-authority/🔣️.json"), "utf8"));
const functionNames = ["reconcileTransactionOwnedTuples", "validateForwardMoveSourceInputs", "validateForwardGeneratorInputs", "validateResumeTuples"];
const classNames = ["TaxonomyStartedRegenerationPartialError", "TaxonomyMoveSourceInputDriftError", "TaxonomyGeneratorInputDriftError"];
const compilers = [
  { name: "Bun", compile: (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code) },
  { name: "TypeScript", compile: (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];

/** 🧮️ Runs the actual extracted ownership/read-set boundaries with observable deterministic filesystem adapters. */
function evaluate(compiler: typeof compilers[number], row: any, forward: boolean) {
  const selected = tree.statements.filter((node) => ts.isFunctionDeclaration(node) && functionNames.includes(node.name?.text ?? "") || ts.isClassDeclaration(node) && classNames.includes(node.name?.text ?? ""));
  expect(selected).toHaveLength(functionNames.length + classNames.length);
  const state = { inputReads: 0, membershipReads: 0, outputReads: 0 };
  const input = { path: "🔣️inputs.json", nodeKind: "file", contentHash: "a".repeat(64), mode: 420, size: 2 };
  const plan = { edits: [], moves: [], embeddedTicketRootRelocations: [], evidenceRemovals: [], embeddedTicketRoots: [], symlinkTargetEdits: [], regenerations: [{ id: "fixture", contractId: "fixture", inputs: [input], preOutputs: [], outputs: [], outputRoots: ["🧪️outputs"] }] };
  const journal = { stagingRoot: "stage", backupRoot: "backup", backups: {}, appliedEditPaths: [], startedRegenerationIds: [], completedRegenerationIds: [] };
  const adapters = {
    absolutePath: (root: string, path: string) => join(root, path), join,
    canonicalJson: (value: unknown) => JSON.stringify(value),
    generatorPathCompare: (left: string, right: string) => left.localeCompare(right),
    resumeGeneratorInputAuthority: () => ({}),
    resumeGeneratorInputView: () => ({}),
    resumeGeneratorInputRecord: () => {
      state.inputReads++;
      if (row.input === "unreadable") throw new Error("exact input unreadable");
      return row.input === "changed" ? { ...input, contentHash: "b".repeat(64) } : input;
    },
    generatorInputPaths: () => {
      state.membershipReads++;
      if (row.membership === "unreadable") throw new Error("exact membership unreadable");
      return row.membership === "added" ? [input.path, "🧪️inputs/🧪️added"] : row.membership === "missing" ? [] : [input.path];
    },
    generatorTreeInventory: () => { state.outputReads++; return row.output === "foreign" ? [{ path: "🟦️outputs.ts", nodeKind: "file", contentHash: "c".repeat(64), mode: 420, size: 1 }] : []; },
  };
  const code = selected.map((node) => node.getText(tree)).join("\n");
  const api = new Function(...Object.keys(adapters), compiler.compile(code) + "\nreturn { owned: reconcileTransactionOwnedTuples, forward: validateResumeTuples, inputError: TaxonomyGeneratorInputDriftError };")(...Object.values(adapters));
  let outcome = "valid", reason = "";
  try { (forward ? api.forward : api.owned)("/fixture", plan, journal, { schema: { generatorContracts: { fixture: {} } } }); }
  catch (error) { outcome = error instanceof api.inputError ? "input-drift" : "owned-drift"; reason = String(error); }
  return { outcome, reason, ...state };
}

test("owned recovery and strict forward authority have a language-neutral transition contract", () => {
  const validate = new Ajv().compile({ type: "object", required: ["schemaVersion", "contract", "semantics", "cases"], properties: { schemaVersion: { const: 1 }, contract: { const: "transaction-owned-recovery-versus-forward-inputs-v1" }, cases: { type: "array", minItems: 8, items: { type: "object", required: ["id", "input", "membership", "output", "owned", "forward", "inputReads", "membershipReads"] } } } });
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
});

for (const compiler of compilers) test(compiler.name + " preserves inverse ownership while rejecting changed forward input authority", () => {
  for (const row of vector.cases) {
    const owned = evaluate(compiler, row, false), forward = evaluate(compiler, row, true);
    expect({ outcome: owned.outcome, inputReads: owned.inputReads, membershipReads: owned.membershipReads }, row.id).toEqual({ outcome: row.owned, inputReads: 0, membershipReads: 0 });
    expect({ outcome: forward.outcome, inputReads: forward.inputReads, membershipReads: forward.membershipReads }, row.id).toEqual({ outcome: row.forward, inputReads: row.inputReads, membershipReads: row.membershipReads });
    expect(owned.outputReads, row.id).toBe(1);
    expect(forward.outputReads, row.id).toBe(1);
    if (row.owned === "owned-drift") expect(forward.reason).toContain("regeneration outputs");
  }
});

test("WAL and selected-resume snapshots use owned proof while forward execution retains its strict wrapper", () => {
  const declaration = (name: string) => tree.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === name)!;
  const calls = (node: ts.Node): string[] => {
    const result: string[] = [];
    const visit = (part: ts.Node) => { if (ts.isCallExpression(part) && ts.isIdentifier(part.expression)) result.push(part.expression.text); ts.forEachChild(part, visit); };
    visit(node);
    return result;
  };
  expect(calls(declaration("reconcileJournalWal")).filter((name) => name === "reconcileTransactionOwnedTuples")).toHaveLength(2);
  expect(calls(declaration("reconcileJournalWal"))).not.toContain("validateResumeTuples");
  const apply = declaration("applyTaxonomyPlan");
  let selected: ts.VariableDeclaration | undefined;
  const visit = (node: ts.Node) => { if (ts.isVariableDeclaration(node) && node.name.getText(tree) === "validateSelectedResumeSnapshot") selected = node; ts.forEachChild(node, visit); };
  visit(apply);
  expect(selected).toBeDefined();
  expect(calls(selected!)).toContain("reconcileTransactionOwnedTuples");
  expect(calls(selected!)).not.toContain("validateResumeTuples");
  expect(calls(apply).filter((name) => name === "validateResumeTuples")).toHaveLength(3);
});

test("recovery authority is mounted through its exact Nx and launch registrations", () => {
  const row = vector.registration, root = resolve(library, "../../../../.."), packagePath = join(library, "📦️packages/🟦️typescript");
  const project = JSON.parse(readFileSync(join(packagePath, "📋️project.json"), "utf8"));
  expect(project.targets["test-" + row.id]).toEqual({ executor: "nx:run-commands", options: { cwd: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript", command: "bun ./📜️script.ts test " + row.id } });
  const router = readFileSync(join(packagePath, "📜️script.ts"), "utf8");
  expect(router).toContain('segments[0] === "' + row.id + '"');
  expect(router).toContain('🧪️🧪️🏔️🦋️tests/🧪️' + row.id + '../🧪️🪪️🐸️transaction-recovery-authority/🟦️.ts');
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const document = getNodeValue(parseTree(readFileSync(join(root, path), "utf8"))!);
    expect(document.configurations.filter((entry: any) => entry.presentation?.group === "4_gate" && entry.presentation?.order === row.order)).toHaveLength(1);
    expect(document.configurations.filter((entry: any) => entry.name === row.name)).toEqual([{ name: row.name, type: "node-terminal", request: "launch", command: "bun nx run @semio-tech/repo-lib:test-" + row.id + " --skip-nx-cache", cwd: "${workspaceFolder}", presentation: { group: "4_gate", order: row.order } }]);
  }
});
