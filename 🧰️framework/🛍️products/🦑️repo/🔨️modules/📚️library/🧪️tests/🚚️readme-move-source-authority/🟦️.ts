import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, posix, resolve } from "node:path";
import Ajv from "ajv";
import stableStringify from "fast-json-stable-stringify";
import { getNodeValue, parse as parseJson, parseTree, type ParseError } from "jsonc-parser";
import ts from "typescript";

const library = resolve(import.meta.dir, "../.."), sourcePath = join(library, "🧹️normalization/🟦️.ts");
const source = readFileSync(sourcePath, "utf8"), tree = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🚚️readme-move-source-authority/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../🚚️readme-move-source-authority/🧬️schema/🔣️.json"), "utf8"));
const sha = (value: string | Uint8Array): string => createHash("sha256").update(value).digest("hex");
const compilers = [
  { name: "Bun", compile: (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code) },
  { name: "TypeScript", compile: (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];
const declarations = new Map(tree.statements.filter(ts.isFunctionDeclaration).map((node) => [node.name?.text ?? "", node]));

/** 🧮️ Builds the independent canonical oracle from parsed JSON and the declared identifier-array law. */
function oracleJson(value: unknown): string {
  const parsed = getNodeValue(parseTree(JSON.stringify(value))!);
  const key = (entry: any): string | null => {
    if (!entry || Array.isArray(entry) || typeof entry !== "object") return null;
    const fields = vector.canonicalArrayKeys.filter((field: string) => typeof entry[field] === "string");
    return fields.length ? fields.map((field: string) => field + ":" + entry[field]).join("\0") : null;
  };
  const ordered = (entry: any): any => {
    if (Array.isArray(entry)) {
      const values = entry.map(ordered);
      return values.every((item) => key(item) !== null) ? values.sort((a, b) => Buffer.compare(Buffer.from(key(a)!), Buffer.from(key(b)!))) : values;
    }
    return entry && typeof entry === "object" ? Object.fromEntries(Object.entries(entry).map(([field, child]) => [field, ordered(child)])) : entry;
  };
  return stableStringify(ordered(parsed));
}

const oracleHash = (value: unknown): string => new Bun.CryptoHasher("sha256").update(oracleJson(value)).digest("hex");
const preimage = (contents: string, mode = 420) => ({ nodeKind: "file", contentHash: sha(contents), mode, size: Buffer.byteLength(contents) });
const operationId = (move: any): string => new Bun.CryptoHasher("sha256").update("move-v2\0" + oracleJson({ sourcePath: move.sourcePath, destinationPath: move.destinationPath, sourcePreimage: move.sourcePreimage })).digest("hex").slice(0, 24);

/** 🧾️ Creates a synthetic parser-only plan; no fixture path is materialized or applied. */
function fixture(context = "revised") {
  const sourceAuthority = { ...vector.revision, inputs: vector.inputs.map((input: any) => ({ role: input.role, path: input.path, preimage: preimage(input.contents, input.mode) })) };
  const move: any = { operationId: "", sourcePath: vector.source.path, destinationPath: vector.source.destinationPath, sourcePreimage: preimage(vector.source.contents, vector.source.mode), rationaleRule: "readme-license-owner-projection-v1", ownerId: vector.source.ownerId, referenceEdits: [] };
  move.operationId = operationId(move);
  if (context === "revised") move.sourceAuthority = sourceAuthority;
  const plan: any = { schemaVersion: 2, taxonomySchemaVersion: 7, baselineCommit: "9f449b10659b95148c8bcb3f91ce583bf7446973", scope: "🧪️owner", sourceTreeDigest: "1".repeat(64), excludedTreeDigests: [], moves: [move], embeddedTicketRoots: [], embeddedTicketRootRelocations: [], symlinkTargetEdits: [], evidenceRemovals: [], destinationAncestorPreimages: [], edits: [], regenerations: [], unresolved: [], expectedAffectedPreStateDigest: "2".repeat(64), expectedPostStateDigest: "3".repeat(64), planDigest: "" };
  return { plan: seal(plan), sourceAuthority };
}

/** 🔏️ Derives only parser fixture identities using the independent canonical oracle. */
function seal(plan: any) {
  for (const move of plan.moves) move.operationId = operationId(move);
  const ancestors = new Set<string>();
  for (const move of plan.moves) for (let path = posix.dirname(move.destinationPath); path !== "."; path = posix.dirname(path)) ancestors.add(path);
  plan.destinationAncestorPreimages = [...ancestors].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))).map((path) => ({ path, state: "absent" }));
  const { planDigest: _digest, ...digestible } = plan;
  plan.planDigest = oracleHash(digestible);
  return plan;
}

/** 🧪️ Applies neutral mutations to an in-memory submitted move, never to its fresh authority. */
function scenario(row: any) {
  const expected = fixture(row.context).plan, candidate = structuredClone(expected), move = candidate.moves[0];
  const set = (target: any, pointer: string, value: unknown, remove = false): void => {
    const fields = pointer.split("/").slice(1);
    for (const field of fields.slice(0, -1)) target = target[field];
    if (remove) delete target[fields.at(-1)!]; else target[fields.at(-1)!] = structuredClone(value);
  };
  for (const change of row.changes) {
    if (change.op === "remove" || change.op === "set") set(move, change.pointer, change.value, change.op === "remove");
    if (change.op === "copy-authority") move.sourceAuthority = fixture().sourceAuthority;
    if (change.op === "set-input") set(move.sourceAuthority.inputs.find((input: any) => input.role === change.role), change.pointer, change.value);
    if (change.op === "remove-input") move.sourceAuthority.inputs = move.sourceAuthority.inputs.filter((input: any) => input.role !== change.role);
    if (change.op === "duplicate-input") move.sourceAuthority.inputs.push(structuredClone(move.sourceAuthority.inputs.find((input: any) => input.role === change.role)));
    if (change.op === "duplicate-path") move.sourceAuthority.inputs.find((input: any) => input.role === change.to).path = move.sourceAuthority.inputs.find((input: any) => input.role === change.from).path;
    if (change.op === "reverse-inputs") move.sourceAuthority.inputs.reverse();
  }
  return { expected, candidate: seal(candidate) };
}

/** 🔬️ Compiles exact production parser, digest, input-mapping and preflight-comparison bodies. */
function implementation(compiler: typeof compilers[number], adapters: Record<string, unknown> = {}) {
  const names = ["sha256", "canonicalArrayKey", "canonicalValue", "canonicalJson", "planRecord", "planString", "planPath", "planInteger", "parseLeafPreimage", "dispositionOperationId", "parseReferenceEdit", "parseMove", "parseTaxonomyPlan", "taxonomyPlanDigest", "generatorPathCompare", "normalizeRelative", "sourceRelative", "resumeGeneratorInputAuthority", "resumeGeneratorInputPhysicalPath", "resumeGeneratorInputRecord"];
  const selected = names.map((name) => { const node = declarations.get(name); if (!node) throw new Error("Missing production body: " + name); return node.getText(tree).replace(/^export\s+/u, ""); });
  for (const name of ["parseMoveSourceAuthority", "validateForwardMoveSourceInputs"]) {
    const optional = declarations.get(name);
    if (optional) selected.push(optional.getText(tree).replace(/^export\s+/u, ""));
  }
  const moveError = tree.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === vector.forwardBoundary.typedError);
  if (moveError) selected.push(moveError.getText(tree));
  const constants = tree.statements.filter(ts.isVariableStatement).filter((node) => node.declarationList.declarations.some((item) => ["PLAN_HASH", "PLAN_OPERATION_ID", "PLAN_COMMIT_ID"].includes(item.name.getText(tree))));
  const apply = declarations.get("applyTaxonomyPlan")!, blocks: ts.VariableStatement[] = [];
  const visit = (node: ts.Node): void => { if (ts.isVariableStatement(node) && node.declarationList.declarations.some((item) => item.name.getText(tree) === "operationSets")) blocks.push(node); ts.forEachChild(node, visit); };
  visit(apply);
  if (blocks.length !== 1 || !ts.isBlock(blocks[0]!.parent)) throw new Error("The actual preflight operation comparison is not uniquely identified");
  const siblings = (blocks[0]!.parent as ts.Block).statements, offset = siblings.indexOf(blocks[0]!);
  const guard = siblings.slice(offset, offset + 3).map((node) => node.getText(tree)).join("\n");
  if (!guard.includes("const mismatch") || !guard.includes("cannot be rederived exactly")) throw new Error("Preflight comparison source boundary drifted");
  const environment = { createHash, posix, ...adapters };
  const code = constants.map((node) => node.getText(tree)).join("\n") + "\n" + selected.join("\n") + "\nfunction exactPreflight(plan, authorityPlan) {\n" + guard + "\n}\n";
  return new Function(...Object.keys(environment), compiler.compile(code) + "\nreturn { parse: parseTaxonomyPlan, canonical: canonicalJson, digest: taxonomyPlanDigest, preflight: exactPreflight, inputAuthority: resumeGeneratorInputAuthority, physicalPath: resumeGeneratorInputPhysicalPath, inputRecord: resumeGeneratorInputRecord, forwardInputs: typeof validateForwardMoveSourceInputs === 'function' ? validateForwardMoveSourceInputs : null, moveInputError: typeof TaxonomyMoveSourceInputDriftError === 'function' ? TaxonomyMoveSourceInputDriftError : null };")(...Object.values(environment));
}

const apis = compilers.map((compiler) => ({ compiler, api: implementation(compiler) }));

test("move source authority has a closed neutral grammar with independent JSON and file-hash parity", () => {
  expect(getNodeValue(parseTree(readFileSync(join(import.meta.dir, "../🚚️readme-move-source-authority/🔣️.json"), "utf8"))!)).toEqual(vector);
  expect(new Ajv({ allErrors: true }).compile(schema.definitions.execution)(vector.execution)).toBe(true);
  expect(new Ajv({ allErrors: true }).compile(schema.definitions.strictCompilation)(vector.strictCompilation)).toBe(true);
  const validate = new Ajv({ allErrors: true }).compile(schema);
  for (const row of vector.cases) {
    const move = scenario(row).candidate.moves[0], inputs = move.sourceAuthority?.inputs;
    const accepted = validate(move) && (!inputs || new Set(inputs.map((input: any) => input.path)).size === inputs.length);
    expect(Boolean(accepted), row.id + ": " + JSON.stringify(validate.errors)).toBe(row.syntaxAccepted);
  }
  for (const input of [vector.source, ...vector.inputs]) expect(new Bun.CryptoHasher("sha256").update(input.contents).digest("hex")).toBe(preimage(input.contents, input.mode).contentHash);
});

for (const { compiler, api } of apis) test(compiler.name + " exact plan parser preserves the new authority and rejects closed-shape negatives", () => {
  let firstRejection = "";
  const actual = vector.cases.map((row: any) => {
    const { candidate } = scenario(row);
    let parsed: any;
    try { parsed = api.parse(candidate); }
    catch (error) { if (row.syntaxAccepted && !firstRejection) firstRejection = row.id + ": " + String(error); }
    if (!parsed) return { id: row.id, accepted: false };
    expect(api.canonical(parsed)).toBe(oracleJson(candidate));
    expect(api.digest(parsed)).toBe(candidate.planDigest);
    expect(parsed.moves[0].operationId).toBe(candidate.moves[0].operationId);
    expect(parsed.moves[0].sourceAuthority).toEqual(candidate.moves[0].sourceAuthority);
    return { id: row.id, accepted: true };
  });
  expect(actual, firstRejection).toEqual(vector.cases.map((row: any) => ({ id: row.id, accepted: row.syntaxAccepted })));
});

test("existing canonical plan digest binds authority while move operation identity remains unchanged", () => {
  const first = fixture().plan, ordinary = fixture("ordinary").plan;
  expect(first.moves[0].operationId).toBe(ordinary.moves[0].operationId);
  expect(first.planDigest).not.toBe(ordinary.planDigest);
  for (const { api } of apis) for (const row of vector.cases) {
    const { expected, candidate } = scenario(row);
    expect(api.digest(candidate), row.id).toBe(candidate.planDigest);
    expect(api.canonical(candidate), row.id).toBe(oracleJson(candidate));
    expect(candidate.planDigest === expected.planDigest, row.id).toBe(row.freshAccepted);
  }
});

for (const { compiler, api } of apis) test(compiler.name + " actual apply preflight rejects omitted foreign and stale source authority", () => {
  for (const row of vector.cases) {
    const { expected, candidate } = scenario(row), before = JSON.stringify({ expected, candidate });
    if (row.freshAccepted) expect(() => api.preflight(candidate, expected), row.id).not.toThrow();
    else expect(() => api.preflight(candidate, expected), row.id).toThrow(/cannot be rederived exactly/u);
    expect(JSON.stringify({ expected, candidate }), row.id).toBe(before);
  }
});

/** 🗺️ Supplies deterministic file records to the existing logical-input resolver without filesystem writes. */
function phase(compiler: typeof compilers[number], row: any) {
  const plan = fixture().plan, move = plan.moves[0], input = row.query === "source" ? { role: "source", path: vector.source.path, contents: vector.source.contents, mode: vector.source.mode } : vector.inputs.find((entry: any) => entry.role === row.query);
  const journal: any = { stagingRoot: "🧪️run/📥️stage", backupRoot: "🧪️run/📚️backup", backups: {}, stagedMoveIds: [], installedMoveIds: [], stagedEmbeddedRelocationIds: [], installedEmbeddedRelocationIds: [], stagedEvidenceRemovalIds: [], stagedEmbeddedRootIds: [], stagedSymlinkTargetEditIds: [] };
  const memory = new Map<string, any>([vector.source, ...vector.inputs].map((entry: any) => [entry.path, { ...preimage(entry.contents, entry.mode), contents: entry.contents }]));
  const expectation = vector.inputs.find((entry: any) => entry.role === "expectation");
  const expectationMove: any = { sourcePath: expectation.path, destinationPath: expectation.destinationPath, sourcePreimage: preimage(expectation.contents), ownerId: posix.dirname(expectation.path), rationaleRule: "fixture", referenceEdits: [] };
  expectationMove.operationId = operationId(expectationMove);
  if (row.state === "staged") {
    journal.stagedMoveIds.push(move.operationId);
    memory.set(journal.stagingRoot + "/" + move.operationId, memory.get(move.sourcePath));
    memory.delete(move.sourcePath);
  } else if (row.state !== "source") {
    journal.stagedMoveIds.push(move.operationId); journal.installedMoveIds.push(move.operationId);
    memory.set(move.destinationPath, memory.get(move.sourcePath)); memory.delete(move.sourcePath);
  }
  if (row.state === "expectation-staged" || row.state === "expectation-installed-edit") {
    plan.moves.push(expectationMove);
    journal.stagedMoveIds.push(expectationMove.operationId);
    const location = row.state === "expectation-staged" ? journal.stagingRoot + "/" + expectationMove.operationId : expectationMove.destinationPath;
    if (row.state === "expectation-installed-edit") journal.installedMoveIds.push(expectationMove.operationId);
    memory.set(location, memory.get(expectation.path)); memory.delete(expectation.path);
  }
  if (["owned-edit", "bad-backup", "unplanned-backup", "expectation-installed-edit"].includes(row.state)) {
    const finalPath = row.state === "expectation-installed-edit" ? expectationMove.destinationPath : expectation.path;
    journal.backups[finalPath] = { kind: "file", backupPath: "expectation-preimage", ...preimage(expectation.contents) };
    if (row.state !== "unplanned-backup") plan.edits.push({ path: finalPath });
    memory.set(finalPath, { ...preimage(expectation.postContents), contents: expectation.postContents });
    const contents = row.state === "bad-backup" ? "foreign backup\n" : expectation.contents;
    memory.set(journal.backupRoot + "/expectation-preimage", { ...preimage(contents), contents });
  }
  if (row.state === "foreign") memory.set(input.path, { ...preimage("foreign input\n"), contents: "foreign input\n" });
  if (row.state === "missing") memory.delete(input.path);
  if (row.state === "symlink") memory.set(input.path, { nodeKind: "symlink", contentHash: sha("elsewhere"), mode: 420, size: 9, target: "elsewhere" });
  if (row.state === "directory") memory.set(input.path, { nodeKind: "directory", contentHash: sha("directory"), mode: 493 });
  const reads: string[] = [], before = JSON.stringify([...memory]);
  const api = implementation(compiler, {
    assertLexicalInputOutsideOpaque: (_root: string, path: string) => {
      if (/^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error("Opaque input");
      return path;
    },
    generatorNodeRecord: (_root: string, path: string) => {
      reads.push(path);
      const value = memory.get(path);
      if (!value) throw new Error("Exact input is absent");
      const { contents: _contents, ...record } = value;
      return { path, ...record };
    },
  });
  const authority = api.inputAuthority(plan, journal), physical = api.physicalPath(authority, journal, input.path);
  const variables: Record<string, string> = { source: move.sourcePath, destination: move.destinationPath, input: input.path, stagingRoot: journal.stagingRoot, backupRoot: journal.backupRoot, sourceOperationId: move.operationId, expectationOperationId: expectationMove.operationId };
  const expectedPath = row.expectedPath.replace(/\$\{([A-Za-z]+)\}/gu, (_: string, name: string) => variables[name]!);
  const frozen = { path: input.path, ...preimage(input.contents, input.mode) };
  let outcome = "unreadable";
  try { outcome = oracleJson(api.inputRecord("/virtual", authority, journal, frozen, {})) === oracleJson(frozen) ? "same" : "changed"; } catch { }
  const mappingReads = [...reads];
  let forward = "unimplemented";
  if (api.forwardInputs) {
    try { api.forwardInputs("/virtual", plan, journal, {}); forward = "valid"; }
    catch (error) { forward = api.moveInputError && error instanceof api.moveInputError ? "input-drift" : "unclassified"; }
  }
  return { physical, expectedPath, outcome, forward, reads: mappingReads, unchanged: JSON.stringify([...memory]) === before };
}

for (const compiler of compilers) test(compiler.name + " journal input mapping distinguishes planned expectation edits from external drift", () => {
  for (const row of vector.phaseCases) {
    const result = phase(compiler, row);
    expect(result.physical, row.id).toBe(result.expectedPath);
    expect(result.outcome, row.id).toBe(row.expected);
    expect(result.reads, row.id).toEqual([result.expectedPath]);
    expect(result.unchanged, row.id).toBe(true);
  }
});

for (const compiler of compilers) test(compiler.name + " generator-free revised moves classify every forward authority input drift", () => {
  const actual = vector.phaseCases.map((row: any) => ({ id: row.id, forward: phase(compiler, row).forward }));
  expect(actual).toEqual(vector.phaseCases.map((row: any) => ({ id: row.id, forward: row.expected === "same" ? "valid" : "input-drift" })));
});

test("actual selected-resume catch inverse-recovers typed move input drift but not unknown owned drift", () => {
  const apply = declarations.get("applyTaxonomyPlan")!, catches: ts.CatchClause[] = [];
  const visit = (node: ts.Node): void => { if (ts.isCatchClause(node) && node.block.getText(tree).includes("TaxonomyGeneratorInputDriftError") && node.block.getText(tree).includes("rollbackTransaction")) catches.push(node); ts.forEachChild(node, visit); };
  visit(apply);
  expect(catches).toHaveLength(1);
  class MoveInputDrift extends Error {}
  class GeneratorInputDrift extends Error {}
  class StartedPartial extends Error {}
  for (const compiler of compilers) for (const row of vector.resumeErrorCases) {
    const calls: string[] = [], plan = fixture().plan, journal: any = { state: "editing" }, error = row.kind === "move" ? new MoveInputDrift("move input drift") : row.kind === "generator" ? new GeneratorInputDrift("generator input drift") : row.kind === "partial" ? new StartedPartial("partial output") : new Error("unknown owned state");
    const environment = { plan, journal, repoRoot: "/virtual", journalPath: "virtual-journal", taxonomy: {}, options: {}, digest: plan.planDigest, TaxonomyMoveSourceInputDriftError: MoveInputDrift, TaxonomyGeneratorInputDriftError: GeneratorInputDrift, TaxonomyStartedRegenerationPartialError: StartedPartial, persistJournal: () => calls.push("persist"), rollbackTransaction: () => calls.push("rollback"), releaseLease: () => calls.push("release") };
    const run = new Function(...Object.keys(environment), compiler.compile("return function(error) { try { throw error; } catch (error) " + catches[0]!.block.getText(tree) + " }") )(...Object.values(environment));
    let outcome = "thrown";
    try { outcome = run(error).state; } catch { }
    expect(outcome, compiler.name + ":" + row.id).toBe(row.expected);
    expect(calls, row.id).toEqual(row.expected === "rolled-back" ? ["persist", "rollback", "release"] : []);
    expect(journal.state, row.id).toBe(row.expected === "rolled-back" ? "rolling-back" : "editing");
  }
});

test("actual final forward gate validates revised moves even when there are no generators", () => {
  const apply = declarations.get("applyTaxonomyPlan")!, gates: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => { if (ts.isIfStatement(node) && node.expression.getText(tree).includes("plan.regenerations.length") && node.expression.getText(tree).includes("validateResumeTuples")) gates.push(node); ts.forEachChild(node, visit); };
  visit(apply);
  expect(gates).toHaveLength(1);
  for (const compiler of compilers) for (const row of vector.finalGateCases) {
    const plan = fixture(row.revised ? "revised" : "ordinary").plan;
    plan.regenerations = Array.from({ length: row.generators }, () => ({}));
    let validations = 0;
    const environment = { plan, repoRoot: "/virtual", journal: {}, taxonomy: {}, journalPath: "virtual-journal", validateResumeTuples: () => { validations++; return false; }, persistJournal: () => { throw new Error("Unchanged tuple proof must not persist"); } };
    new Function(...Object.keys(environment), compiler.compile(gates[0]!.getText(tree)))(...Object.values(environment));
    expect(validations, compiler.name + ":" + row.id).toBe(row.validations);
  }
});

test("forward continuation requires move input validation after owned reconciliation without adding it to inverse recovery", () => {
  const rule = vector.forwardBoundary, calls = (node: ts.Node): string[] => {
    const result: string[] = [];
    const visit = (child: ts.Node): void => { if (ts.isCallExpression(child) && ts.isIdentifier(child.expression)) result.push(child.expression.text); ts.forEachChild(child, visit); };
    visit(node); return result;
  };
  const forward = calls(declarations.get(rule.function)!);
  expect(forward).toContain(rule.moveInputFunction);
  expect(forward.indexOf(rule.ownedFunction)).toBeLessThan(forward.indexOf(rule.moveInputFunction));
  const validator = declarations.get(rule.moveInputFunction);
  expect(validator).toBeDefined();
  expect(validator!.getText(tree)).toContain("sourceAuthority");
  expect(calls(validator!)).toContain("resumeGeneratorInputRecord");
  for (const name of ["reconcileTransactionOwnedTuples", "reconcileRollbackTuples", "rollbackTransaction", "reconcileJournalWal"]) expect(calls(declarations.get(name)!)).not.toContain(rule.moveInputFunction);
});

test("actual forward wrapper stops before external reads when owned reconciliation fails", () => {
  const wrapper = declarations.get(vector.forwardBoundary.function)!;
  for (const compiler of compilers) for (const row of vector.forwardOrderCases) {
    const calls: string[] = [], step = (name: string): boolean => { calls.push(name); if (row.failure === name) throw new Error(name); return row.ownedChanged; };
    const environment = { reconcileTransactionOwnedTuples: () => step("owned"), validateForwardMoveSourceInputs: () => step("move"), validateForwardGeneratorInputs: () => step("generator") };
    const run = new Function(...Object.keys(environment), compiler.compile(wrapper.getText(tree)) + "\nreturn validateResumeTuples;")(...Object.values(environment));
    let result: unknown;
    try { result = run("/virtual", {}, {}, {}); } catch (error) { result = (error as Error).message; }
    expect({ calls, result }, compiler.name + ":" + row.id).toEqual({ calls: row.calls, result: row.result });
  }
});

test("exact new move authority declarations satisfy independent strict TypeScript compilation", () => {
  const names = new Set(["TaxonomyLeafPreimage", "TaxonomyMoveSourceAuthority", "TaxonomyMove", "TaxonomyGeneratorNodeRecord", "parseMoveSourceAuthority", "parseMove", "TaxonomyMoveSourceInputDriftError", "validateForwardMoveSourceInputs"]);
  const selected = tree.statements.filter((node) => (ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node) || ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node)) && node.name && names.has(node.name.text));
  expect(selected).toHaveLength(names.size);
  const ambient = `
type ReferenceEdit = object;
type TaxonomyPlan = { moves: readonly TaxonomyMove[] };
type MutableJournalRecord = object;
type LoadedTaxonomy = object;
declare const PLAN_HASH: RegExp, PLAN_OPERATION_ID: RegExp;
declare const Buffer: { from(value: string): { toString(encoding: "utf8"): string } };
declare function planRecord(value: unknown, name: string, required: readonly string[], optional?: readonly string[]): Record<string, unknown>;
declare function planString(value: unknown, name: string, pattern?: RegExp): string;
declare function planPath(value: unknown, name: string): string;
declare function parseLeafPreimage(value: unknown, name: string): TaxonomyLeafPreimage;
declare function parseReferenceEdit(value: unknown, name: string): ReferenceEdit;
declare function dispositionOperationId(domain: string, value: object): string;
declare function canonicalJson(value: unknown): string;
declare function resumeGeneratorInputAuthority(plan: TaxonomyPlan, journal: MutableJournalRecord): object;
declare function resumeGeneratorInputRecord(root: string, authority: object, journal: MutableJournalRecord, input: TaxonomyGeneratorNodeRecord, taxonomy: LoadedTaxonomy): TaxonomyGeneratorNodeRecord;
`;
  const virtualPath = join(import.meta.dir, "strict-input.ts"), text = ambient + selected.map((node) => node.getText(tree)).join("\n"), expected = vector.strictCompilation;
  const options: ts.CompilerOptions = { strict: expected.strict, noEmit: expected.noEmit, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, types: [], skipLibCheck: true };
  const host = ts.createCompilerHost(options, true), getSourceFile = host.getSourceFile;
  host.getSourceFile = (file, version, error, fresh) => file === virtualPath ? ts.createSourceFile(file, text, version, true) : getSourceFile(file, version, error, fresh);
  host.writeFile = () => { throw new Error("Move authority strict compiler cannot emit"); };
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([virtualPath], options, host)).map((item) => ({ code: item.code, message: ts.flattenDiagnosticMessageText(item.messageText, "\n") }));
  expect(diagnostics).toEqual(expected.diagnostics);
});

test("test preparation leaves normalizer bytes unchanged and never materializes synthetic paths", () => {
  expect(sha(readFileSync(sourcePath))).toBe(sha(source));
  expect(vector.scope).toBe("in-memory-only");
  expect(vector.phaseCases).toHaveLength(13);
});

test("move source authority gate registration matches the package Nx router and both launch catalogs", () => {
  const expected = vector.execution, repoRoot = resolve(library, "../../../../.."), packagePath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript";
  const parsed = (path: string, jsonc = false): any => {
    const text = readFileSync(join(repoRoot, path), "utf8"), errors: ParseError[] = [], value = parseJson(text, errors, { disallowComments: !jsonc, allowTrailingComma: jsonc });
    expect(errors, path).toEqual([]);
    if (!jsonc) expect(value, path).toEqual(JSON.parse(text));
    return value;
  };
  const project = parsed(packagePath + "/📋️project.json"), manifest = parsed(packagePath + "/package.json");
  const router = readFileSync(join(repoRoot, packagePath, "📜️script.ts"), "utf8"), routerTree = ts.createSourceFile("router.ts", router, ts.ScriptTarget.Latest, true), branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node) && ts.isBinaryExpression(node.expression) && node.expression.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken && node.expression.left.getText(routerTree) === "segments[0]" && ts.isStringLiteral(node.expression.right) && node.expression.right.text === expected.route) branches.push(node);
    ts.forEachChild(node, visit);
  };
  visit(routerTree);
  const launches = [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"].map((path) => {
    const configurations = parsed(path, true).configurations;
    return { path, rows: configurations.filter((row: any) => row.name === expected.launchName), orderRows: configurations.filter((row: any) => row.presentation?.group === expected.launchGroup && row.presentation?.order === expected.launchOrder).length };
  });
  expect({ packageName: manifest.name, packageCommand: manifest.scripts?.[expected.target], target: project.targets[expected.target], branches: branches.length, launches }).toEqual({
    packageName: expected.packageName,
    packageCommand: expected.packageCommand,
    target: { executor: "nx:run-commands", options: { cwd: packagePath, command: expected.command } },
    branches: 1,
    launches: launches.map(({ path }) => ({ path, rows: [{ name: expected.launchName, type: "node-terminal", request: "launch", command: expected.launchCommand, cwd: "${workspaceFolder}", presentation: { group: expected.launchGroup, order: expected.launchOrder } }], orderRows: 1 })),
  });
  expect(branches[0]!.thenStatement.getText(routerTree)).toContain("join(this.repoRoot, " + JSON.stringify(expected.source) + ")");
  expect(branches[0]!.thenStatement.getText(routerTree)).toContain('await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });');
});
