import { normalizationSourceDeclarations } from "../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { expect, test } from "bun:test";
import { readFileSync, writeFileSync, mkdirSync, mkdtempSync, lstatSync, readdirSync, renameSync, symlinkSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { semanticOwnedInputFileSnapshot } from "../../🔍️discovery/🟦️.ts";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { getNodeValue, parseTree } from "jsonc-parser";
import ts from "typescript";

const library = resolve(import.meta.dir, "../.."), sourcePath = join(library, "🧹️normalization/🟦️.ts"), source = normalizationSourceDeclarations(sourcePath);
const tree = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🛟️transaction-recovery-authority/🔣️.json"), "utf8"));
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
    isTransactionRepositoryAuthorityError: () => false,
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
  expect(router).toContain('🧪️tests/🛟️' + row.id + '/🟦️.ts');
  const document = getNodeValue(parseTree(readFileSync(join(root, ".vscode/launch.json"), "utf8"))!);
  expect(document.configurations.filter((entry: any) => entry.name === row.name)).toEqual([{ name: row.name, type: "node-terminal", request: "launch", command: "bun nx run @semio-tech/repo-lib:test-" + row.id, cwd: "${workspaceFolder}", presentation: { group: "4_gate", order: row.order } }]);
});


for (const compiler of compilers) test(compiler.name + " preserves physical index observations against the actual Git stage oracle", () => {
  const contract = vector.indexObservations;
  const validate = new Ajv().compile({ type: "object", additionalProperties: false, required: ["schemaVersion", "contract", "cases"], properties: { schemaVersion: { const: 1 }, contract: { const: "physical-git-index-observation-v1" }, cases: { type: "array", minItems: 6, items: { type: "object", additionalProperties: false, required: ["id", "edit", "oracleReads"], properties: { id: { type: "string" }, edit: { enum: ["none", "stage", "head", "split", "include", "symlink"] }, oracleReads: { type: "integer", minimum: 1, maximum: 2 } } } } } });
  expect(validate(contract), JSON.stringify(validate.errors)).toBe(true);
  const names = ["sourceAdmissionIndexObservation", "sourceAdmissionGitRows"];
  const selected = tree.statements.filter((node) => ts.isFunctionDeclaration(node) && names.includes(node.name?.text ?? "") || ts.isVariableStatement(node) && node.declarationList.declarations.some((entry) => entry.name.getText(tree) === "sourceAdmissionIndexObservations"));
  expect(selected).toHaveLength(3);
  const code = selected.map((node) => node.getText(tree)).join("\n");
  for (const row of contract.cases) {
    const owner = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (!owner) throw new Error("Index observation proof needs explicit ticket output ownership");
    mkdirSync(owner, { recursive: true });
    const root = mkdtempSync(join(owner, "🧪️index-observation-"));
    const git = (args: string[]) => execFileSync("git", args, { cwd: root, encoding: "buffer" });
    git(["init", "--quiet", "--object-format=sha1"]);
    writeFileSync(join(root, "input.txt"), "first\n");
    git(["add", "--", "input.txt"]);
    let oracleReads = 0;
    const parse = (bytes: Buffer) => bytes.toString("utf8").split("\0").filter(Boolean).map((record) => {
      const [header, path] = record.split("\t"), [mode, objectId, stage] = header.split(" ");
      return { path, entry: { mode, objectId, stage: Number(stage) } };
    });
    const adapters = { process, Buffer, join, lstatSync, readdirSync, lstatOrNull: (path: string) => { try { return lstatSync(path); } catch { return null; } }, semanticOwnedInputFileSnapshot, sha256: (text: string) => createHash("sha256").update(text).digest("hex"), canonicalJson: (value: unknown): string => JSON.stringify(value), execFileSync: (command: string, args: string[], options: any) => { oracleReads++; return execFileSync(command, args, options); }, sourceAdmissionGitExclusions: () => [], sourceAdmissionGitRecords: (bytes: Buffer) => bytes.toString("utf8").split("\0").filter(Boolean), sourceAdmissionSafePath: (path: string) => path.length > 0 && !path.includes("..") };
    const read = new Function(...Object.keys(adapters), compiler.compile(code) + "\nreturn sourceAdmissionGitRows;")(...Object.values(adapters));
    const spec = { positivePathspec: ":(top)", exclusionPathspecs: [] };
    expect(read(root, spec)).toEqual(parse(git(["ls-files", "--stage", "-z"])));
    if (row.edit === "stage") { writeFileSync(join(root, "input.txt"), "second\n"); git(["add", "--", "input.txt"]); }
    if (row.edit === "head") writeFileSync(join(root, ".git/HEAD"), "ref: refs/heads/another\n");
    if (row.edit === "split") git(["update-index", "--split-index"]);
    if (row.edit === "include") { writeFileSync(join(root, ".git/extra"), "[core]\nfilemode = true\n"); git(["config", "include.path", "extra"]); }
    if (row.edit === "symlink") { renameSync(join(root, ".git/index"), join(root, ".git/retained-index")); symlinkSync("retained-index", join(root, ".git/index"), "file"); }
    expect(read(root, spec)).toEqual(parse(git(["ls-files", "--stage", "-z"])));
    expect(oracleReads, row.id).toBe(row.oracleReads);
  }
});
