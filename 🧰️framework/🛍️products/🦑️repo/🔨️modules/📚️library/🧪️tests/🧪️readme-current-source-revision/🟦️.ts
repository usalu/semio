import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { closeSync, constants, fstatSync, lstatSync, openSync, readFileSync } from "node:fs";
import { join, posix, resolve } from "node:path";
import Ajv from "ajv";
import stableStringify from "fast-json-stable-stringify";
import { parse as parseJson, type ParseError } from "jsonc-parser";
import ts from "typescript";
import * as discovery from "../../🔍️discovery/🟦️.ts";

const libraryRoot = resolve(import.meta.dir, "../.."), root = resolve(libraryRoot, "../../../../..");
const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8"));
const sha = (bytes: Uint8Array | string): string => createHash("sha256").update(bytes).digest("hex");
const revision = vector.revisions[vector.revisionId];

/** 📚️ Collects exact regular-file evidence without importing a catalog loader. */
function evidence(path: string) {
  if (posix.isAbsolute(path) || /\\|^[A-Za-z]:/u.test(path) || path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error("Unsafe pure-authority test input");
  const parts = path.split("/"), ancestorNodeKinds: string[] = [];
  let current = root;
  for (const [index, part] of parts.entries()) {
    current = join(current, part);
    const node = lstatSync(current);
    if (node.isSymbolicLink() || (index < parts.length - 1 ? !node.isDirectory() : !node.isFile())) throw new Error("Nonregular pure-authority test input");
    if (index < parts.length - 1) ancestorNodeKinds.push("directory");
  }
  const before = lstatSync(current), fd = openSync(current, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    const node = fstatSync(fd);
    if (node.dev !== before.dev || node.ino !== before.ino || !node.isFile()) throw new Error("Pure-authority evidence node changed");
    const bytes = readFileSync(fd), after = fstatSync(fd);
    if (after.size !== node.size || after.mtimeMs !== node.mtimeMs || after.mode !== node.mode) throw new Error("Pure-authority evidence bytes changed");
    return { path, nodeKind: "file", mode: node.mode & 0o7777, ancestorNodeKinds, bytes };
  } finally {
    closeSync(fd);
  }
}

const catalogBytes = evidence(vector.catalogPath).bytes, originalCatalog = JSON.parse(catalogBytes.toString("utf8"));
const fixtureAuthorityInput = evidence(vector.fixtureInputs.path), fixtureAuthority = JSON.parse(fixtureAuthorityInput.bytes.toString("utf8"));
const fixtureSchema = JSON.parse(evidence(posix.dirname(vector.fixtureInputs.path) + "/🧬️schema/🔣️.json").bytes.toString("utf8"));
if (sha(fixtureAuthorityInput.bytes) !== vector.fixtureInputs.sha256 || !new Ajv({ allErrors: true }).compile(fixtureSchema)(fixtureAuthority) || fixtureAuthority.catalog.path !== vector.catalogPath || fixtureAuthority.catalog.sha256 !== vector.catalogSha256 || fixtureAuthority.revision.id !== vector.revisionId) throw new Error("Reviewed fixture authority drift");

/** 🧫️ Maps verified fixture bytes to declared logical evidence without reading the historical live path. */
function reviewedEvidence(role: "source" | "expectation") {
  const row = fixtureAuthority.inputs.find((input: any) => input.role === role), captured = evidence(row.path);
  if (sha(captured.bytes) !== row.preimage.sha256 || captured.bytes.length !== row.preimage.size || captured.mode !== row.preimage.mode) throw new Error("Reviewed fixture preimage drift");
  const path = role === "source" ? originalCatalog.cases[31].sourcePath : revision.expectationsPath;
  return { ...captured, path, ancestorNodeKinds: path.split("/").slice(1).map(() => "directory") };
}

const currentSource = reviewedEvidence("source"), expectation = reviewedEvidence("expectation"), originalExpectation = JSON.parse(expectation.bytes.toString("utf8"));
const contract = { authorityCatalogPath: vector.catalogPath, authorityCatalogSha256: vector.catalogSha256 };
const functions = () => {
  const parse = Reflect.get(discovery, "parseSemanticOwnedCurrentSourceRevisions");
  const authority = Reflect.get(discovery, "semanticExactOwnedFileCurrentPreimageAuthority");
  expect(typeof parse).toBe("function");
  expect(typeof authority).toBe("function");
  return { parse, authority };
};

/** 🧪️ Applies authored neutral mutations only to cloned in-memory test records. */
function scenario(row: any) {
  const catalog = structuredClone(originalCatalog), revisions = structuredClone(vector.revisions), identity = structuredClone(contract);
  const owner = catalog.cases[31];
  const facts = { path: owner.sourcePath, nodeKind: "file", contentHash: revision.currentPreimage.sha256, size: revision.currentPreimage.size, mode: 0o644, expectations: [{ ...expectation, bytes: Buffer.from(expectation.bytes), ancestorNodeKinds: [...expectation.ancestorNodeKinds] }] };
  const state: Record<string, any> = { catalog, revisions, contract: identity, facts, expectationDocument: structuredClone(originalExpectation) };
  for (const operation of row.changes) {
    const segments = operation.pointer.split("/").slice(1).map((part: string) => part.replace(/~1/gu, "/").replace(/~0/gu, "~"));
    if (!segments.length) state[operation.target] = structuredClone(operation.value);
    else {
      let target = state[operation.target];
      for (const segment of segments.slice(0, -1)) target = target[segment];
      target[segments.at(-1)!] = structuredClone(operation.value);
    }
  }
  if (row.changes.some((operation: any) => operation.target === "expectationDocument")) state.facts.expectations[0].bytes = Buffer.from(JSON.stringify(state.expectationDocument));
  if (row.special === "missing-catalog-case") state.catalog.cases.pop();
  if (row.special === "missing-source-coordinate") delete state.revisions[vector.revisionId].sourcePath;
  if (row.special === "missing-expectation") state.facts.expectations = [];
  if (row.special === "duplicate-expectation") state.facts.expectations.push({ ...state.facts.expectations[0] });
  if (row.special === "append-expectation-space") state.facts.expectations[0].bytes = Buffer.concat([state.facts.expectations[0].bytes, Buffer.from(" ")]);
  if (row.special === "invalid-utf8-expectation") state.facts.expectations[0].bytes = Buffer.from([0xff, 0x7b, 0x7d]);
  if (row.special === "invalid-json-expectation") state.facts.expectations[0].bytes = Buffer.from("{");
  if (row.rebindExpectations) state.revisions[vector.revisionId].expectationsSha256 = sha(state.facts.expectations[0].bytes);
  return state;
}

/** 🔏️ Independent canonical-JSON oracle with explicitly enumerated digest authority. */
function oracleDigest(catalog: any, identity: any, id: string, row: any): string {
  const owner = catalog.cases[row.catalogCaseIndex], ownerEvidence = catalog.ownerEvidence[owner.ownerEvidenceId];
  const envelope = {
    kind: "exact-owner-current-source-revision-v1",
    catalogIdentity: { path: identity.authorityCatalogPath, sha256: identity.authorityCatalogSha256 },
    revisionId: id,
    revision: row,
    owner: {
      catalogCaseIndex: row.catalogCaseIndex,
      sourcePath: owner.sourcePath,
      destinationPath: owner.destinationPath,
      ownerEvidenceId: owner.ownerEvidenceId,
      ownerEvidence: { kind: ownerEvidence.kind, evidencePaths: ownerEvidence.evidencePaths },
      disposition: owner.disposition,
      fixedContractId: owner.fixedContractId,
      projectionContractId: owner.projectionContractId,
      generatorOwnerId: owner.generatorOwnerId,
      referenceOwners: owner.referenceOwnerIds.map((referenceId: string) => ({ id: referenceId, kind: catalog.referenceOwners[referenceId].kind, ownerPath: catalog.referenceOwners[referenceId].ownerPath })),
    },
  };
  return new Bun.CryptoHasher("sha256").update(stableStringify(envelope)).digest("hex");
}

test("current source revision neutral grammar agrees with Ajv and independent JSON parsing", () => {
  const validate = new Ajv({ allErrors: true }).compile(schema), errors: ParseError[] = [];
  expect(new Ajv({ allErrors: true }).compile(schema.definitions.execution)(vector.execution)).toBe(true);
  expect(new Ajv({ allErrors: true }).compile(schema.definitions.fixtureInputs)(vector.fixtureInputs)).toBe(true);
  expect(parseJson(fixtureAuthorityInput.bytes.toString("utf8"), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(fixtureAuthority);
  expect(parseJson(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(errors).toEqual([]);
  expect(validate(vector.revisions), JSON.stringify(validate.errors)).toBe(true);
  for (const row of vector.cases.filter((row: any) => row.phase === "grammar")) {
    const state = scenario(row), declared = state.revisions[vector.revisionId];
    const accepted = validate(state.revisions) && declared.currentPreimage.sha256 !== declared.baselinePreimage.sha256;
    expect(Boolean(accepted), row.id).toBe(row.accepted);
  }
  expect(sha(catalogBytes)).toBe(vector.catalogSha256);
  expect(sha(expectation.bytes)).toBe(revision.expectationsSha256);
  expect(oracleDigest(originalCatalog, contract, vector.revisionId, revision)).toBe(vector.expectedRevisionDigest);
});

test("current source revision pure grammar rejects every closed-field and index violation", () => {
  const actual = functions();
  expect(actual.parse(vector.revisions)).toEqual(vector.revisions);
  for (const row of vector.cases.filter((row: any) => row.phase === "grammar")) {
    const state = scenario(row);
    expect(() => actual.parse(state.revisions), row.id).toThrow(/current-source-revision-invalid/u);
  }
  for (const field of schema.properties[vector.revisionId].required) {
    const input = structuredClone(vector.revisions);
    delete input[vector.revisionId][field];
    expect(() => actual.parse(input), field).toThrow(/current-source-revision-invalid/u);
  }
});

test("current source revision cannot omit its declared raw source coordinate", () => {
  const input = structuredClone(vector.revisions);
  delete input[vector.revisionId].sourcePath;
  expect(() => functions().parse(input)).toThrow(/current-source-revision-invalid/u);
});

test("current source revision pure authority accepts only exact current tuple and owned expectation evidence", () => {
  const actual = functions();
  for (const row of vector.cases.filter((row: any) => row.phase !== "grammar")) {
    const state = scenario(row), before = JSON.stringify(state);
    const result = actual.authority(state.catalog, state.contract, state.revisions, state.facts);
    expect(result.disposition, row.id).toBe(row.accepted ? "revised" : "problem");
    if (row.accepted) {
      expect(result.preimage).toEqual(revision.currentPreimage);
      expect(result.revisionId).toBe(vector.revisionId);
      expect(result.revisionDigest).toBe(oracleDigest(state.catalog, state.contract, vector.revisionId, revision));
      expect(result.problems).toEqual([]);
    } else {
      expect(result.preimage, row.id).toBeNull();
      expect(result.problems.length, row.id).toBeGreaterThan(0);
    }
    expect(JSON.stringify(state), row.id).toBe(before);
  }
});

test("a reviewed current source leaves every other frozen catalog preimage unchanged", () => {
  const actual = functions(), state = scenario(vector.cases.find((row: any) => row.id === "exact-approved-current-tuple")), before = JSON.stringify(state.catalog);
  let unchanged = 0;
  for (const [index, row] of state.catalog.cases.entries()) {
    if (index === 31) continue;
    const facts = { ...state.facts, path: row.sourcePath, nodeKind: "file", contentHash: row.preimage.sha256, mode: parseInt(row.preimage.mode, 8), size: row.preimage.size, expectations: [] };
    const result = actual.authority(state.catalog, state.contract, state.revisions, facts);
    expect(result.disposition, row.sourcePath).toBe("catalog");
    expect(result.preimage).toEqual(row.preimage);
    expect(result.preimage).not.toBe(row.preimage);
    expect(result.revisionId).toBeNull();
    expect(result.revisionDigest).toBeNull();
    expect(result.problems).toEqual([]);
    unchanged++;
  }
  expect(unchanged).toBe(39);
  expect(JSON.stringify(state.catalog)).toBe(before);
});

test("current revision evidence is demanded only for its revised raw source", () => {
  const actual = functions();
  for (const row of vector.selectionBoundaries) {
    const state = scenario(vector.cases.find((row: any) => row.id === "exact-approved-current-tuple")), owner = state.catalog.cases[row.catalogCaseIndex ?? 31];
    const preimage = row.catalogCaseIndex === 31 ? revision.currentPreimage : owner.preimage;
    const facts = { ...state.facts, path: row.path ?? owner[row.coordinate], contentHash: preimage.sha256, mode: parseInt(preimage.mode, 8), size: preimage.size, expectations: [] };
    const before = JSON.stringify({ catalog: state.catalog, revisions: state.revisions, facts });
    const result = actual.authority(state.catalog, state.contract, state.revisions, facts);
    expect(result.disposition, row.id).toBe(row.expectedDisposition);
    expect(result.revisionId, row.id).toBeNull();
    expect(result.revisionDigest, row.id).toBeNull();
    expect(result.preimage, row.id).toEqual(row.expectedDisposition === "catalog" ? owner.preimage : null);
    expect(JSON.stringify({ catalog: state.catalog, revisions: state.revisions, facts }), row.id).toBe(before);
  }
});

test("current revision digest is canonical and binds the selected tuple and complete derived owner identities", () => {
  const actual = functions(), state = scenario(vector.cases.find((row: any) => row.id === "exact-approved-current-tuple"));
  const first = actual.authority(state.catalog, state.contract, state.revisions, state.facts);
  expect(first.revisionDigest).toBe(vector.expectedRevisionDigest);
  expect(first.revisionDigest).toBe(oracleDigest(state.catalog, state.contract, vector.revisionId, revision));
  const reverse = (value: any): any => Array.isArray(value) ? value.map(reverse) : value && typeof value === "object" ? Object.fromEntries(Object.entries(value).reverse().map(([key, child]) => [key, reverse(child)])) : value;
  const reordered = reverse(state.revisions);
  expect(actual.authority(state.catalog, state.contract, reordered, state.facts).revisionDigest).toBe(first.revisionDigest);
  const next = structuredClone(state.revisions);
  next[vector.revisionId].currentPreimage = { ...revision.currentPreimage, sha256: "3".repeat(64), size: 12528 };
  const facts = { ...state.facts, contentHash: "3".repeat(64), size: 12528 };
  const changed = actual.authority(state.catalog, state.contract, next, facts);
  expect(changed.disposition).toBe("revised");
  expect(changed.revisionDigest).not.toBe(first.revisionDigest);
  expect(changed.revisionDigest).toBe(oracleDigest(state.catalog, state.contract, vector.revisionId, next[vector.revisionId]));
});

test("reviewed README fixture bytes satisfy the selected pure authority without loading or changing the frozen catalog", () => {
  const actual = functions(), state = scenario(vector.cases.find((row: any) => row.id === "exact-approved-current-tuple")), source = currentSource;
  const result = actual.authority(state.catalog, state.contract, state.revisions, { ...state.facts, contentHash: sha(source.bytes), size: source.bytes.length, mode: source.mode });
  expect(result.disposition).toBe("revised");
  expect(result.preimage).toEqual(revision.currentPreimage);
  expect(sha(evidence(vector.catalogPath).bytes)).toBe(vector.catalogSha256);
});

test("current revision parser and authority contain no filesystem process or catalog-loader calls", () => {
  const path = join(libraryRoot, "🔍️discovery/🟦️.ts"), source = readFileSync(path, "utf8"), tree = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
  const names = new Set(["parseSemanticOwnedCurrentSourceRevisions", "semanticExactOwnedFileCurrentPreimageAuthority", "semanticOwnedCurrentRevisionCanonical", "exactOwnerPath"]);
  const declarations = tree.statements.filter((node) => ts.isFunctionDeclaration(node) && names.has(node.name?.text ?? ""));
  expect(declarations).toHaveLength(4);
  const forbidden = /^(?:readFileSync|writeFileSync|lstatSync|statSync|readdirSync|existsSync|execFileSync|spawn|spawnSync|semanticExactOwnedFileCatalog|loadTaxonomy)$/u;
  const inspect = (node: ts.Node): void => {
    if (ts.isCallExpression(node)) {
      const name = ts.isIdentifier(node.expression) ? node.expression.text : ts.isPropertyAccessExpression(node.expression) ? node.expression.name.text : "";
      expect(forbidden.test(name), name).toBe(false);
    }
    ts.forEachChild(node, inspect);
  };
  declarations.forEach(inspect);
});

test("current revision exact helper extraction has no strict compiler diagnostics", () => {
  const path = join(libraryRoot, "🔍️discovery/🟦️.ts"), source = readFileSync(path, "utf8"), tree = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
  const names = new Set(["SemanticOwnedCurrentSourceRevision", "SemanticOwnedCurrentSourceExpectation", "SemanticOwnedCurrentSourcePreimageResult", "SemanticExactOwnedFileProjectionContract", "SemanticOwnedDocumentCorrection", "SemanticExactOwnedFileCase", "SemanticExactOwnedFileCatalog", "parseSemanticOwnedCurrentSourceRevisions", "semanticExactOwnedFileCurrentPreimageAuthority", "semanticOwnedCurrentRevisionCanonical", "exactOwnerPath"]);
  const declarations = tree.statements.filter((node) => (ts.isFunctionDeclaration(node) || ts.isInterfaceDeclaration(node)) && names.has(node.name?.text ?? ""));
  expect(declarations).toHaveLength(names.size);
  const ambient = 'type PureBytes = Uint8Array & { toString(encoding: "utf8"): string; equals(value: Uint8Array): boolean }; declare const Buffer: { from(value: string | Uint8Array): PureBytes }; declare function createHash(algorithm: "sha256"): { update(value: string | Uint8Array): { digest(encoding: "hex"): string } }; declare const posix: { basename(value: string): string; dirname(value: string): string };';
  const virtualPath = join(import.meta.dir, "strict-input.ts"), text = ambient + "\n" + declarations.map((node) => node.getText(tree)).join("\n"), expected = vector.strictCompilation;
  const options: ts.CompilerOptions = { strict: expected.strict, noEmit: expected.noEmit, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, types: [], skipLibCheck: true };
  const host = ts.createCompilerHost(options, true), getSourceFile = host.getSourceFile;
  host.getSourceFile = (file, version, error, fresh) => file === virtualPath ? ts.createSourceFile(file, text, version, true) : getSourceFile(file, version, error, fresh);
  host.writeFile = () => { throw new Error("Pure authority strict compiler cannot emit"); };
  const program = ts.createProgram([virtualPath], options, host);
  const diagnostics = ts.getPreEmitDiagnostics(program).map((item) => ({ code: item.code, message: ts.flattenDiagnosticMessageText(item.messageText, "\n") }));
  expect(diagnostics).toEqual(expected.diagnostics);
});

test("revision gate registration matches the declared Nx route and both launch catalogs", () => {
  const expected = vector.execution, packagePath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript";
  const projectSource = evidence(packagePath + "/📋️project.json").bytes.toString("utf8"), project = JSON.parse(projectSource), errors: ParseError[] = [];
  expect(parseJson(projectSource, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(project);
  expect(errors).toEqual([]);
  expect(project.targets[expected.target]).toEqual({ executor: "nx:run-commands", options: { cwd: packagePath, command: expected.command } });
  const router = evidence(packagePath + "/📜️script.ts").bytes.toString("utf8"), tree = ts.createSourceFile("router.ts", router, ts.ScriptTarget.Latest, true), branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node) && ts.isBinaryExpression(node.expression) && node.expression.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken && node.expression.left.getText(tree) === "segments[0]" && ts.isStringLiteral(node.expression.right) && node.expression.right.text === expected.route) branches.push(node);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  expect(branches).toHaveLength(1);
  expect(branches[0]!.thenStatement.getText(tree)).toContain("join(this.repoRoot, " + JSON.stringify(expected.source) + ")");
  expect(branches[0]!.thenStatement.getText(tree)).toContain('await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });');
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const errors: ParseError[] = [], configurations = parseJson(evidence(path).bytes.toString("utf8"), errors).configurations;
    expect(errors, path).toEqual([]);
    expect(configurations.filter((row: any) => row.name === expected.launchName), path).toEqual([{ name: expected.launchName, type: "node-terminal", request: "launch", command: expected.launchCommand, cwd: "${workspaceFolder}", presentation: { group: expected.launchGroup, order: expected.launchOrder } }]);
    expect(configurations.filter((row: any) => row.presentation?.group === expected.launchGroup && row.presentation?.order === expected.launchOrder), path).toHaveLength(1);
  }
});
