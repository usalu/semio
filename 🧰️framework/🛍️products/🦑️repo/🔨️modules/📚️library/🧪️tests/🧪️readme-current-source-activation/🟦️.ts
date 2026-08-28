import { afterAll, expect, test } from "bun:test";
import { createHash, randomUUID } from "node:crypto";
import { chmodSync, closeSync, constants, fstatSync, lstatSync, mkdirSync, openSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, posix, relative, resolve } from "node:path";
import Ajv from "ajv";
import stableStringify from "fast-json-stable-stringify";
import { findNodeAtLocation, getNodeValue, parse as parseJson, parseTree, type ParseError } from "jsonc-parser";
import ts from "typescript";
import * as discovery from "../../🔍️discovery/🟦️component.ts";
import * as normalization from "../../🧹️normalization/🟦️.ts";

const library = resolve(import.meta.dir, "../.."), root = resolve(library, "../../../../..");
const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8"));
const sha = (bytes: string | Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const observations = new Map<string, { bytes: Buffer; mode: number; sha256: string; size: number }>();
const outcomes: any[] = [];
let retainedOwner: string | undefined;

/** 🛡️ Rejects opaque or ambiguous coordinates before any filesystem observation. */
function safePath(path: string): string {
  if (!path || path !== path.normalize("NFC") || /[\\:*?"<>|\u0000-\u001f]/u.test(path) || Buffer.from(path).toString("utf8") !== path || path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error("Unsafe activation test coordinate");
  return path;
}

/** 📚️ Captures exact regular input bytes with no-follow ancestry and descriptor identity. */
function readInput(repoRoot: string, path: string) {
  const parts = safePath(path).split("/");
  let current = repoRoot;
  for (const [index, part] of parts.entries()) {
    current = join(current, part);
    const stat = lstatSync(current);
    if (stat.isSymbolicLink() || (index === parts.length - 1 ? !stat.isFile() : !stat.isDirectory())) throw new Error("Nonregular activation input: " + path);
  }
  const before = lstatSync(current), fd = openSync(current, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    const stat = fstatSync(fd);
    if (!stat.isFile() || stat.dev !== before.dev || stat.ino !== before.ino) throw new Error("Activation input changed during open: " + path);
    const bytes = readFileSync(fd), after = fstatSync(fd);
    if (after.size !== stat.size || after.mtimeMs !== stat.mtimeMs || after.mode !== stat.mode) throw new Error("Activation input changed during read: " + path);
    return { bytes, mode: stat.mode & 0o7777, sha256: sha(bytes), size: bytes.byteLength };
  } finally { closeSync(fd); }
}

const capture = (path: string) => {
  const value = readInput(root, path);
  const before = observations.get(path);
  if (before && (before.sha256 !== value.sha256 || before.size !== value.size || before.mode !== value.mode || !before.bytes.equals(value.bytes))) throw new Error("Repeated activation input drift: " + path);
  if (!before) observations.set(path, value);
  return value;
};
const revisionInput = capture(vector.revisionInput), revisionVector = JSON.parse(revisionInput.bytes.toString("utf8"));
const revision = revisionVector.revisions[vector.revisionId], catalogInput = capture(vector.catalogPath), catalogDocument = JSON.parse(catalogInput.bytes.toString("utf8"));
const fixtureAuthorityInput = capture(revisionVector.fixtureInputs.path), fixtureAuthority = JSON.parse(fixtureAuthorityInput.bytes.toString("utf8"));
const fixtureSchema = JSON.parse(capture(posix.dirname(revisionVector.fixtureInputs.path) + "/🧬️schema/🔣️.json").bytes.toString("utf8"));
if (fixtureAuthorityInput.sha256 !== revisionVector.fixtureInputs.sha256 || !new Ajv({ allErrors: true }).compile(fixtureSchema)(fixtureAuthority) || fixtureAuthority.catalog.path !== vector.catalogPath || fixtureAuthority.catalog.sha256 !== vector.catalogSha256 || fixtureAuthority.revision.id !== vector.revisionId) throw new Error("Reviewed activation fixture authority drift");

/** 🧫️ Captures only retained reviewed payloads before installing declared logical paths inside isolated repositories. */
function reviewedInput(role: "source" | "expectation") {
  const row = fixtureAuthority.inputs.find((input: any) => input.role === role), value = capture(row.path);
  if (value.sha256 !== row.preimage.sha256 || value.size !== row.preimage.size || value.mode !== row.preimage.mode) throw new Error("Reviewed activation fixture preimage drift");
  return value;
}

const ownerRow = catalogDocument.cases[vector.catalogCaseIndex], currentInput = reviewedInput("source"), expectationInput = reviewedInput("expectation");
const taxonomyInput = capture(vector.taxonomyPath), originalTaxonomy = JSON.parse(taxonomyInput.bytes.toString("utf8"));
const normalizerInput = capture(relative(root, join(library, "🧹️normalization/🟦️.ts")).replaceAll("\\", "/"));
capture(relative(root, join(library, "🔍️discovery/🟦️component.ts")).replaceAll("\\", "/"));
let baselineBytes: Buffer | undefined;

/** 🧬️ Verifies the immutable Git object independently of all expectation declarations. */
function baseline() {
  if (baselineBytes) return baselineBytes;
  const run = (args: string[]): Buffer => {
    const result = Bun.spawnSync(["git", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    return Buffer.from(result.stdout);
  };
  const blob = run(["rev-parse", vector.baseline.commit + ":" + safePath(ownerRow.sourcePath)]).toString().trim();
  expect(blob).toBe(vector.baseline.blob);
  const treeEntry = run(["ls-tree", "-z", vector.baseline.commit, "--", ownerRow.sourcePath]).toString("utf8").split("\t")[0];
  expect(treeEntry).toBe("100644 blob " + blob);
  const bytes = run(["cat-file", "blob", blob]);
  expect({ sha256: sha(bytes), size: bytes.length }).toEqual({ sha256: vector.baseline.sha256, size: vector.baseline.size });
  expect(new Bun.CryptoHasher("sha1").update(Buffer.concat([Buffer.from("blob " + bytes.length + "\0"), bytes])).digest("hex")).toBe(blob);
  baselineBytes = bytes;
  return bytes;
}

/** 📁️ Creates only new no-follow directories beneath an already validated fixture owner. */
function directories(repoRoot: string, path: string): string {
  let current = repoRoot;
  for (const part of safePath(path).split("/")) {
    current = join(current, part);
    try {
      const node = lstatSync(current);
      if (!node.isDirectory() || node.isSymbolicLink()) throw new Error("Unsafe fixture directory: " + current);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
      mkdirSync(current);
    }
  }
  return current;
}

/** 🔖️ Allocates one fresh semantic run owner and retains every prepared case. */
function runOwner(): string {
  if (retainedOwner) return retainedOwner;
  if (vector.runParent !== ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️readme-current-source/📓️activation/🧾️runs") throw new Error("Unregistered activation output owner");
  let parent = root;
  for (const part of safePath(vector.runParent).split("/")) {
    parent = join(parent, part);
    const stat = lstatSync(parent);
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Activation run parent is not an existing no-follow directory");
  }
  retainedOwner = join(parent, "🔖️" + randomUUID());
  mkdirSync(retainedOwner);
  writeFileSync(join(retainedOwner, "📝️.md"), "# Reviewed README Activation Fixture\n\nThis fresh owner retains loader/planner inputs and failures. No producer, apply or cleanup runs. Fixture Git commits are isolated and distinct from the independently verified original baseline lineage.\n", { flag: "wx" });
  console.log("[DEBUG] README activation owner", retainedOwner);
  return retainedOwner;
}

/** 🧾️ Writes a fresh fixture leaf without following or overwriting an existing node. */
function put(repoRoot: string, path: string, bytes: string | Uint8Array, mode = 0o644): void {
  safePath(path);
  if (dirname(path) !== ".") directories(repoRoot, dirname(path));
  const absolute = join(repoRoot, path);
  writeFileSync(absolute, bytes, { flag: "wx", mode });
  chmodSync(absolute, mode);
}

/** 🧷️ Prepares exact bytes or an intentional internal symlink negative without touching production paths. */
function install(repoRoot: string, path: string, bytes: Buffer, state: string, label: string): void {
  if (state === "missing") return;
  if (state === "symlink" || state === "parent-symlink") {
    const redirect = "🧪️fixtures/🧪️redirect-" + label + "/" + posix.basename(path);
    put(repoRoot, redirect, bytes);
    const link = state === "symlink" ? path : posix.dirname(path), target = state === "symlink" ? redirect : posix.dirname(redirect);
    if (posix.dirname(link) !== ".") directories(repoRoot, posix.dirname(link));
    symlinkSync(posix.relative(posix.dirname(link), target), join(repoRoot, safePath(link)), state === "parent-symlink" ? "dir" : "file");
    return;
  }
  let content = bytes;
  if (state === "changed") content = Buffer.concat([bytes, Buffer.from("\n")]);
  if (state === "post-edit") {
    const value = JSON.parse(bytes.toString("utf8"));
    value.documents.readme = ownerRow.destinationPath;
    content = Buffer.from(JSON.stringify(value, null, 2) + "\n");
  }
  put(repoRoot, path, content, state === "mode" ? 0o600 : 0o644);
}

/** 🧪️ Builds the proposed schema only inside a retained isolated repository. */
function candidateTaxonomy() {
  const value = structuredClone(originalTaxonomy), isolation = vector.generatorIsolation;
  value.semanticOwnedFileProjectionContracts[vector.contractId].currentSourceRevisions = structuredClone(revisionVector.revisions);
  delete value.generatorContracts[isolation.contractId][isolation.removeField];
  value.generatorContracts[isolation.contractId].inputPatterns = [...isolation.inputPatterns];
  return value;
}

/** 🏗️ Materializes one declared case; real inventory and planning remain unmodified. */
function fixture(row: any, catalogState = "exact") {
  const holder = directories(runOwner(), "🧪️" + row.id), repo = directories(holder, "🧪️repository");
  const taxonomy = candidateTaxonomy(), bytes = Buffer.from(JSON.stringify(taxonomy, null, 2) + "\n");
  put(repo, vector.taxonomyPath, bytes);
  install(repo, vector.catalogPath, catalogInput.bytes, catalogState, "catalog");
  install(repo, revision.expectationsPath, expectationInput.bytes, row.expectation, "expectation");
  const source = row.source === "baseline" ? baseline() : row.source === "changed" ? Buffer.concat([currentInput.bytes, Buffer.from("\nReviewed fixture-only body change.\n")]) : currentInput.bytes;
  const scope = row.layout === "canonical" ? ownerRow.destinationPath : row.layout === "unregistered" ? vector.unregisteredPath : ownerRow.sourcePath;
  if (row.source === "symlink") install(repo, scope, source, "symlink", "source");
  else put(repo, scope, row.layout === "unregistered" ? "# Unregistered canonical owner\n" : source, row.sourceMode);
  if (row.destination !== "absent") {
    const target = row.destination === "folded" ? ownerRow.destinationPath.replace("📃️readme", "📃readme") : ownerRow.destinationPath;
    expect(target === ownerRow.destinationPath).toBe(row.destination === "exact");
    put(repo, target, currentInput.bytes);
  }
  const git = (args: string[]): string => {
    const relativeRepo = relative(runOwner(), repo).replaceAll("\\", "/");
    safePath(relativeRepo);
    if (resolve(runOwner(), relativeRepo) !== repo || lstatSync(repo).isSymbolicLink()) throw new Error("Git fixture owner escaped");
    const result = Bun.spawnSync(["git", ...args], { cwd: repo, stdout: "pipe", stderr: "pipe" });
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    return result.stdout.toString().trim();
  };
  git(["init", "-q"]);
  expect(resolve(git(["rev-parse", "--show-toplevel"]))).toBe(repo);
  git(["add", "--all"]);
  git(["-c", "user.name=README Activation Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "-qm", "Reviewed README activation fixture"]);
  const fixtureBaseline = git(["rev-parse", "HEAD"]);
  const observation = { id: row.id, repo, scope, fixtureBaseline, originalBaseline: vector.baseline, proposedTaxonomySha256: sha(bytes), catalogState, state: row };
  put(holder, "📋️inputs/🔣️.json", JSON.stringify(observation, null, 2) + "\n");
  return { holder, repo, scope, taxonomy, fixtureBaseline };
}

/** 🔏️ Independently enumerates the pure revision envelope without using production canonicalization. */
function revisionDigest(): string {
  const evidence = catalogDocument.ownerEvidence[ownerRow.ownerEvidenceId];
  const envelope = {
    kind: "exact-owner-current-source-revision-v1", catalogIdentity: { path: vector.catalogPath, sha256: vector.catalogSha256 }, revisionId: vector.revisionId, revision,
    owner: { catalogCaseIndex: vector.catalogCaseIndex, sourcePath: ownerRow.sourcePath, destinationPath: ownerRow.destinationPath, ownerEvidenceId: ownerRow.ownerEvidenceId, ownerEvidence: { kind: evidence.kind, evidencePaths: evidence.evidencePaths }, disposition: ownerRow.disposition, fixedContractId: ownerRow.fixedContractId, projectionContractId: ownerRow.projectionContractId, generatorOwnerId: ownerRow.generatorOwnerId, referenceOwners: ownerRow.referenceOwnerIds.map((id: string) => ({ id, kind: catalogDocument.referenceOwners[id].kind, ownerPath: catalogDocument.referenceOwners[id].ownerPath })) },
  };
  return new Bun.CryptoHasher("sha256").update(stableStringify(getNodeValue(parseTree(JSON.stringify(envelope))!))).digest("hex");
}

test("neutral activation inputs retain exact baseline provenance and independent JSON and digest parity", () => {
  const validate = new Ajv({ allErrors: true }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  expect(getNodeValue(parseTree(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"))!)).toEqual(vector);
  expect(revisionInput.sha256).toBe(vector.revisionInputSha256);
  expect(catalogInput.sha256).toBe(vector.catalogSha256);
  expect({ sha256: currentInput.sha256, size: currentInput.size, mode: currentInput.mode }).toEqual({ ...revision.currentPreimage, mode: 0o644 });
  expect(expectationInput.sha256).toBe(revision.expectationsSha256);
  expect(baseline()).toHaveLength(vector.baseline.size);
  expect(ownerRow.preimage).toEqual({ sha256: vector.baseline.sha256, size: vector.baseline.size, mode: "0644" });
  expect(revisionDigest()).toBe(vector.expectedRevisionDigest);
  expect(revisionVector.expectedRevisionDigest).toBe(vector.expectedRevisionDigest);
});

test("shipped schema binds the reviewed revision and three immutable input coordinates without live source reads", () => {
  expect(new Ajv({ allErrors: true }).compile(schema)(vector)).toBe(true);
  const expected = vector.shippedPublication, parsed = getNodeValue(parseTree(taxonomyInput.bytes.toString("utf8"))!);
  const declared = discovery.parseSemanticOwnedCurrentSourceRevisions(revisionVector.revisions);
  const inputs = new Map<string, { path: string; value: ReturnType<typeof capture> }>([
    ["revision", { path: vector.revisionInput, value: revisionInput }],
    ["reviewed-expectation", { path: fixtureAuthority.inputs.find((input: any) => input.role === "expectation").path, value: expectationInput }],
  ]);
  const rows = Object.fromEntries(expected.evidence.map((row: any) => {
    const input = inputs.get(row.inputRole)!;
    return [row.id, { path: input.path, sha256: input.value.sha256, schemaVersion: row.schemaVersion, coordinates: row.coordinates }];
  }));
  expect(Object.keys(rows)).toHaveLength(expected.expectedFrozenDocuments);
  expect(expected.evidence.reduce((count: number, row: any) => count + row.coordinates.length, 0)).toBe(expected.expectedFrozenSelectors);
  expect(discovery.validateFrozenCoordinateEvidenceContracts(rows)).toEqual([]);
  for (const path of [ownerRow.sourcePath, ownerRow.destinationPath, revision.expectationsPath]) expect([...observations.keys()]).not.toContain(path);
  const installed = originalTaxonomy.semanticOwnedFileProjectionContracts[vector.contractId];
  const actual = { revisions: installed.currentSourceRevisions ?? null, evidence: Object.fromEntries(expected.evidence.map((row: any) => [row.id, originalTaxonomy.frozenCoordinateEvidenceContracts[row.id] ?? null])) };
  expect(actual).toEqual({ revisions: declared, evidence: rows });
  expect(getNodeValue(findNodeAtLocation(parseTree(taxonomyInput.bytes.toString("utf8"))!, expected.revisionPointer)!)).toEqual(declared);
  expect(parsed).toEqual(originalTaxonomy);
  expect(discovery.parseSemanticOwnedCurrentSourceRevisions(installed.currentSourceRevisions)).toEqual(declared);
  expect({ path: installed.authorityCatalogPath, sha256: installed.authorityCatalogSha256 }).toEqual({ path: vector.catalogPath, sha256: vector.catalogSha256 });
  for (const row of expected.evidence) {
    const input = inputs.get(row.inputRole)!, tree = parseTree(input.value.bytes.toString("utf8"))!;
    const spans = normalization.frozenCoordinateEvidenceCoordinates(input.path, input.value.bytes, originalTaxonomy.frozenCoordinateEvidenceContracts)!;
    expect(spans).toHaveLength(row.coordinates.length);
    for (const span of spans) {
      const node = findNodeAtLocation(tree, span.pointer.slice(1).split("/").map((part: string) => /^\d+$/u.test(part) ? Number(part) : part))!;
      expect({ value: span.value, start: span.start, end: span.end }).toEqual({ value: ownerRow.sourcePath, start: node.offset + 1, end: node.offset + node.length - 1 });
      expect(node.value).toBe(ownerRow.sourcePath);
    }
  }
  const catalog = { cases: catalogDocument.cases, ownerEvidence: catalogDocument.ownerEvidence, referenceOwners: catalogDocument.referenceOwners, generatorOwners: catalogDocument.generatorOwners };
  const selected = discovery.semanticExactOwnedFileCurrentPreimageAuthority(catalog, installed, installed.currentSourceRevisions, { path: ownerRow.sourcePath, nodeKind: "file", contentHash: currentInput.sha256, mode: currentInput.mode, size: currentInput.size, expectations: [{ path: revision.expectationsPath, nodeKind: "file", mode: expectationInput.mode, ancestorNodeKinds: revision.expectationsPath.split("/").slice(1).map(() => "directory"), bytes: expectationInput.bytes }] });
  expect(selected).toMatchObject({ disposition: "revised", revisionDigest: vector.expectedRevisionDigest, problems: [] });
  const canonical = discovery.semanticExactOwnedFileProjectionAuthority(catalog, { path: ownerRow.destinationPath, nodeKind: "file", contentHash: sha("edited canonical content"), mode: 0o644, size: Buffer.byteLength("edited canonical content"), sourcePresent: false, destinationPresent: true, occupiedPaths: [] });
  expect(canonical).toMatchObject({ disposition: "canonical", problems: [] });
});

test("schema activation admits only the explicit current-source revision member", () => {
  const candidate = candidateTaxonomy(), untouched = structuredClone(candidate);
  delete untouched.semanticOwnedFileProjectionContracts[vector.contractId].currentSourceRevisions;
  expect(discovery.validateTaxonomy(untouched)).toEqual([]);
  expect(discovery.validateTaxonomy(candidate)).toEqual([]);
});

for (const row of vector.catalogCases) test("real catalog loader: " + row.id, () => {
  const setup = fixture({ id: row.id, layout: "raw", source: "current", expectation: "missing", sourceMode: 0o644, destination: "absent" }, row.state);
  let result: discovery.SemanticExactOwnedFileCatalog | null = null, error: unknown, observed: discovery.SemanticOwnedInputFileSnapshot | undefined;
  try { result = discovery.semanticExactOwnedFileCatalog(setup.repo, setup.taxonomy, (snapshot) => { observed = snapshot; }); } catch (caught) { error = caught; }
  outcomes.push({ id: row.id, kind: "catalog", accepted: Boolean(result), error: error instanceof Error ? error.message : error });
  if (!row.accepted) { expect(error).toBeDefined(); expect(String(error)).toMatch(/Exact owner catalog (?:digest drift|mode drift|must be a regular file)/u); expect(observed).toBeUndefined(); return; }
  expect(error).toBeUndefined();
  expect(result!.cases).toEqual(catalogDocument.cases);
  expect(result!.cases).toHaveLength(40);
  expect(readInput(setup.repo, vector.catalogPath)).toMatchObject({ sha256: vector.catalogSha256, mode: 0o644 });
  expect(observed).toMatchObject({ path: vector.catalogPath, nodeKind: "file", contentHash: catalogInput.sha256, mode: catalogInput.mode, size: catalogInput.size, bytes: catalogInput.bytes, ancestorNodeKinds: vector.catalogPath.split("/").slice(1).map(() => "directory") });
  expect(new Bun.CryptoHasher("sha256").update(observed!.bytes).digest("hex")).toBe(observed!.contentHash);
  for (const [index, entry] of result!.cases.entries()) {
    if (index === vector.catalogCaseIndex) continue;
    const selected = discovery.semanticExactOwnedFileCurrentPreimageAuthority(result!, setup.taxonomy.semanticOwnedFileProjectionContracts[vector.contractId], revisionVector.revisions, { path: entry.sourcePath, nodeKind: "file", contentHash: entry.preimage.sha256, mode: 0o644, size: entry.preimage.size, expectations: [] });
    expect(selected.disposition, String(index)).toBe("catalog");
    expect(selected.preimage, String(index)).toEqual(entry.preimage);
  }
});

for (const row of vector.cases) test("real reviewed README loader and planner: " + row.id, () => {
  const setup = fixture(row, row.catalog ?? "exact"), started = performance.now(), progress: any[] = [];
  let inventory: normalization.TaxonomyInventory | undefined, plan: normalization.TaxonomyPlan | undefined, error: unknown;
  try {
    inventory = normalization.inventoryTaxonomy({ repoRoot: setup.repo, scope: setup.scope, workers: 1, cancelFile: join(setup.repo, "🧪️control/🛑️cancel"), progress: (event) => progress.push(event) });
    plan = normalization.planTaxonomy(inventory, { baselineCommit: setup.fixtureBaseline, excludedTreeDigests: [], cancelFile: join(setup.repo, "🧪️control/🛑️cancel") });
  } catch (caught) { error = caught; }
  const result = { id: row.id, expected: row.expected, milliseconds: performance.now() - started, error: error instanceof Error ? error.message : error, violations: inventory?.violations, moves: plan?.moves.length, edits: plan?.edits.length, regenerations: plan?.regenerations.length, unresolved: plan?.unresolved, progress };
  put(setup.holder, "📊️outcome/🔣️.json", JSON.stringify(result, null, 2) + "\n");
  if (plan) put(setup.holder, "🧾️plan/🔣️.json", normalization.canonicalJson(plan) + "\n");
  outcomes.push(result);
  console.log("[DEBUG] README activation case", JSON.stringify({ id: row.id, milliseconds: result.milliseconds, error: result.error, moves: result.moves, unresolved: result.unresolved?.length }));
  expect(error, row.id).toBeUndefined();
  expect(plan!.regenerations, row.id).toEqual([]);
  if (row.expected === "revised-move") expect(plan!.edits.filter((edit) => edit.path === vector.taxonomyPath && edit.oldValue === revision.sourcePath)).toEqual([]);
  if (row.expected === "authority-problem") {
    expect(inventory!.violations.some((problem) => problem.path === ownerRow.sourcePath && problem.code === "owner-leaf-authority-invalid"), row.id).toBe(true);
    expect(plan!.moves.filter((move) => move.sourcePath === ownerRow.sourcePath), row.id).toEqual([]);
    return;
  }
  expect(plan!.unresolved, row.id).toEqual([]);
  if (row.expected === "empty-plan") {
    expect({ moves: plan!.moves, edits: plan!.edits, removals: plan!.evidenceRemovals }, row.id).toEqual({ moves: [], edits: [], removals: [] });
    return;
  }
  expect(plan!.moves).toHaveLength(1);
  const move = plan!.moves[0]!;
  expect({ sourcePath: move.sourcePath, destinationPath: move.destinationPath, sourcePreimage: move.sourcePreimage, rationaleRule: move.rationaleRule }).toEqual({ sourcePath: ownerRow.sourcePath, destinationPath: ownerRow.destinationPath, sourcePreimage: { nodeKind: "file", contentHash: currentInput.sha256, mode: currentInput.mode, size: currentInput.size }, rationaleRule: "readme-license-owner-projection-v1" });
  const inputs = [{ role: "schema", path: vector.taxonomyPath }, { role: "catalog", path: vector.catalogPath }, { role: "expectation", path: revision.expectationsPath }].map(({ role, path }) => {
    const input = readInput(setup.repo, path);
    return { role, path, preimage: { nodeKind: "file", contentHash: input.sha256, mode: input.mode, size: input.size } };
  });
  expect(normalization.canonicalJson(move.sourceAuthority)).toBe(normalization.canonicalJson({ kind: "exact-owner-current-source-revision-v1", revisionId: vector.revisionId, revisionDigest: revisionDigest(), inputs }));
  expect(normalization.parseTaxonomyPlan(JSON.parse(normalization.canonicalJson(plan))).moves[0]!.sourceAuthority).toEqual(move.sourceAuthority);
  const expectationEdits = plan!.edits.filter((edit) => edit.path === revision.expectationsPath && edit.oldValue === ownerRow.sourcePath && edit.newValue === ownerRow.destinationPath);
  expect(expectationEdits).toHaveLength(1);
  expect(plan!.edits.filter((edit) => edit.path === vector.catalogPath)).toEqual([]);
  expect(readInput(setup.repo, vector.catalogPath).bytes).toEqual(catalogInput.bytes);
});

for (const row of vector.driftCases) test("fresh reviewed README planning: " + row.id, () => {
  const setup = fixture({ id: row.id, layout: "raw", source: "current", expectation: "exact", sourceMode: 0o644, destination: "absent" });
  let inventory: normalization.TaxonomyInventory | undefined, plan: normalization.TaxonomyPlan | undefined, error: unknown, changed = false;
  try {
    inventory = normalization.inventoryTaxonomy({ repoRoot: setup.repo, scope: setup.scope, workers: 1, cancelFile: join(setup.repo, "🧪️control/🛑️cancel") });
    expect(inventory.violations.filter((problem) => problem.path === ownerRow.sourcePath && problem.severity === "error")).toEqual([]);
    if (row.target === "inventory") {
      const entries = inventory.entries.map((entry) => entry.sourcePath === ownerRow.sourcePath ? { ...entry, contentHash: "0".repeat(64) } : entry);
      const tuples = entries.map((entry) => ({ sourcePath: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, symlinkTarget: entry.symlinkTarget }));
      inventory = { ...inventory, entries, sourceTreeDigest: sha(normalization.canonicalJson(tuples)) };
    } else {
      const path = row.target === "catalog" ? vector.catalogPath : row.target === "expectation" ? revision.expectationsPath : row.target === "source" ? ownerRow.sourcePath : vector.taxonomyPath;
      const before = readInput(setup.repo, path);
      let bytes = Buffer.concat([before.bytes, Buffer.from("\n")]);
      if (row.target === "schema") {
        const document = JSON.parse(before.bytes.toString("utf8"));
        document.semanticOwnedFileProjectionContracts[vector.contractId].currentSourceRevisions[vector.revisionId].currentPreimage.sha256 = "0".repeat(64);
        bytes = Buffer.from(JSON.stringify(document, null, 2) + "\n");
      }
      writeFileSync(join(setup.repo, safePath(path)), bytes);
    }
    changed = true;
    plan = normalization.planTaxonomy(inventory, { baselineCommit: setup.fixtureBaseline, excludedTreeDigests: [], cancelFile: join(setup.repo, "🧪️control/🛑️cancel") });
  } catch (caught) { error = caught; }
  const outcome = { id: row.id, kind: "between-inventory-and-plan", changed, error: error instanceof Error ? error.message : error, moves: plan?.moves, unresolved: plan?.unresolved };
  put(setup.holder, "📊️outcome/🔣️.json", JSON.stringify(outcome, null, 2) + "\n");
  outcomes.push(outcome);
  expect(changed, "Must reach the deliberate post-inventory mutation").toBe(true);
  if (row.expected === "reject") {
    expect(Boolean(error) || Boolean(plan?.unresolved.some((problem) => problem.code === "owner-leaf-authority-invalid"))).toBe(true);
    expect(plan?.moves.filter((move) => move.sourcePath === ownerRow.sourcePath) ?? []).toEqual([]);
    return;
  }
  expect(error).toBeUndefined();
  expect(plan!.unresolved).toEqual([]);
  const snapshot = readInput(setup.repo, vector.taxonomyPath);
  expect(plan!.moves[0]!.sourceAuthority?.inputs.find((input) => input.role === "schema")).toEqual({ role: "schema", path: vector.taxonomyPath, preimage: { nodeKind: "file", contentHash: snapshot.sha256, mode: snapshot.mode, size: snapshot.size } });
});

test("schema raw-coordinate freezing binds exact parsed bytes catalog owner and baseline only", () => {
  const setup = fixture({ id: "frozen-binding-authority", layout: "raw", source: "current", expectation: "missing", sourceMode: 0o644, destination: "absent" });
  const source = normalizerInput.bytes.toString("utf8"), tree = ts.createSourceFile("normalization.ts", source, ts.ScriptTarget.Latest, true);
  const functions = tree.statements.filter((node) => ts.isFunctionDeclaration(node) && ["isFrozenSourceCoordinateToken", "jsonStringCoordinates"].includes(node.name?.text ?? ""));
  expect(functions).toHaveLength(2);
  const compiled = ts.transpileModule(functions.map((node) => node.getText(tree)).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
  const names = ["frozenEvidenceCoordinateAuthority", "frozenPlanCoordinateAuthority", "sourceRelative", "relative", "sha256", "canonicalJson", "exactOwnedFileCatalog", "semanticPackageProjectionCatalog", "parseSemanticOwnedCurrentSourceRevisions", "frozenCoordinateCache"];
  const predicate = new Function(...names, compiled + ";return isFrozenSourceCoordinateToken;")(() => null, () => ({ coordinates: new Set() }), (path: string) => path.replaceAll("\\", "/"), relative, sha, normalization.canonicalJson, (repo: string, taxonomy: any) => discovery.semanticExactOwnedFileCatalog(repo, taxonomy.discoverySchema), () => null, discovery.parseSemanticOwnedCurrentSourceRevisions, new WeakMap());
  for (const row of vector.frozenBindingCases) {
    const candidate = candidateTaxonomy(), binding = candidate.semanticOwnedFileProjectionContracts[vector.contractId].currentSourceRevisions[vector.revisionId];
    if (row.change === "owner") binding.sourcePath = vector.unregisteredPath.replace("📝️.md", "README.md");
    if (row.change === "baseline") binding.baselinePreimage.sha256 = "0".repeat(64);
    if (row.change === "unowned") candidate.unowned = { sourcePath: ownerRow.sourcePath };
    const captured = Buffer.from(JSON.stringify(candidate)), bytes = row.change === "buffer" ? Buffer.concat([captured, Buffer.from("\n")]) : captured;
    const pointer = row.change === "unowned" ? ["unowned", "sourcePath"] : ["semanticOwnedFileProjectionContracts", vector.contractId, "currentSourceRevisions", vector.revisionId, "sourcePath"];
    const tokenNode = findNodeAtLocation(parseTree(bytes.toString("utf8"))!, pointer)!;
    const token = { start: tokenNode.offset + 1, end: tokenNode.offset + tokenNode.length - 1, value: getNodeValue(tokenNode) };
    const taxonomy = { path: join(setup.repo, vector.taxonomyPath), schema: candidate, discoverySchema: candidate, input: { path: vector.taxonomyPath, contentHash: sha(captured), size: captured.length, mode: 0o644, nodeKind: "file", bytes: captured } };
    expect(predicate(vector.taxonomyPath, bytes, token, token.value, taxonomy, setup.repo), row.id).toBe(row.frozen);
  }
});

test("actual schema loader parses and binds one captured input without a second byte read", () => {
  const source = normalizerInput.bytes.toString("utf8"), tree = ts.createSourceFile("normalization.ts", source, ts.ScriptTarget.Latest, true);
  const declaration = tree.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "loadTaxonomy")!;
  const compiled = ts.transpileModule(declaration.getText(tree), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
  for (const row of vector.schemaSnapshotCases) {
    const bytes = row.state === "lossy" ? Buffer.from([0xff]) : Buffer.from('{"schemaVersion":7,"captured":"first"}');
    const input = row.state === "missing" ? null : { path: vector.taxonomyPath, nodeKind: "file", contentHash: sha(bytes), mode: 0o644, size: bytes.length, ancestorNodeKinds: vector.taxonomyPath.split("/").slice(1).map(() => "directory"), bytes };
    let captures = 0, parses = 0;
    const load = new Function("assertLexicalInputOutsideOpaque", "semanticOwnedInputFileSnapshot", "relative", "resolve", "parseTaxonomy", "TAXONOMY_RELATIVE_PATH", compiled + ";return loadTaxonomy;")((repo: string, path: string) => join(repo, path), () => { if (++captures !== 1) throw new Error("Second input capture is forbidden"); return input; }, relative, resolve, (parsed: unknown, path: string) => { parses++; expect(parsed).toEqual(getNodeValue(parseTree(bytes.toString("utf8"))!)); return { path, schema: parsed }; }, vector.taxonomyPath);
    if (!row.accepted) expect(() => load({ repoRoot: root })).toThrow(/Taxonomy schema (?:is absent|has lossy UTF-8)/u);
    else {
      const loaded = load({ repoRoot: root });
      expect(loaded.input).toBe(input);
      expect(loaded.input.contentHash).toBe(new Bun.CryptoHasher("sha256").update(bytes).digest("hex"));
      expect(loaded.schema).toEqual({ schemaVersion: 7, captured: "first" });
    }
    expect({ captures, parses }, row.id).toEqual({ captures: 1, parses: row.accepted ? 1 : 0 });
  }
});

test("activation preparation invokes no producer or apply and preserves production normalizer bytes", () => {
  const content = readFileSync(import.meta.path, "utf8"), tree = ts.createSourceFile(import.meta.path, content, ts.ScriptTarget.Latest, true), forbidden: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && ["applyTaxonomyPlan", "previewGenerated", "generateCatalogs"].includes(node.expression.name.text)) forbidden.push(node.expression.name.text);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  expect(forbidden).toEqual([]);
  expect(sha(readFileSync(join(library, "🧹️normalization/🟦️.ts")))).toBe(normalizerInput.sha256);
});

test("activation gate registration matches the package router and both launch catalogs", () => {
  const expected = vector.execution, packagePath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript";
  const parsed = (path: string, jsonc = false): any => {
    const text = readInput(root, path).bytes.toString("utf8"), errors: ParseError[] = [], value = parseJson(text, errors, { disallowComments: !jsonc, allowTrailingComma: jsonc });
    expect(errors, path).toEqual([]);
    if (!jsonc) expect(value, path).toEqual(JSON.parse(text));
    return value;
  };
  const project = parsed(packagePath + "/📋️project.json"), manifest = parsed(packagePath + "/package.json");
  const router = readInput(root, packagePath + "/📜️script.ts").bytes.toString("utf8"), tree = ts.createSourceFile("router.ts", router, ts.ScriptTarget.Latest, true), branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node) && ts.isBinaryExpression(node.expression) && node.expression.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken && node.expression.left.getText(tree) === "segments[0]" && ts.isStringLiteral(node.expression.right) && node.expression.right.text === expected.route) branches.push(node);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  const launches = [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"].map((path) => {
    const configurations = parsed(path, true).configurations;
    return { path, rows: configurations.filter((row: any) => row.name === expected.launchName), orderRows: configurations.filter((row: any) => row.presentation?.group === expected.launchGroup && row.presentation?.order === expected.launchOrder).length };
  });
  expect({ packageName: manifest.name, packageCommand: manifest.scripts?.[expected.target], target: project.targets[expected.target], branches: branches.length, launches }).toEqual({ packageName: expected.packageName, packageCommand: expected.packageCommand, target: { executor: "nx:run-commands", options: { cwd: packagePath, command: expected.command } }, branches: 1, launches: launches.map(({ path }) => ({ path, rows: [{ name: expected.launchName, type: "node-terminal", request: "launch", command: expected.launchCommand, cwd: "${workspaceFolder}", presentation: { group: expected.launchGroup, order: expected.launchOrder } }], orderRows: 1 })) });
  expect(branches[0]!.thenStatement.getText(tree)).toContain("join(this.repoRoot, " + JSON.stringify(expected.source) + ")");
  expect(branches[0]!.thenStatement.getText(tree)).toContain('await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });');
});

afterAll(() => {
  const identities = [...observations].map(([path, before]) => {
    const after = readInput(root, path);
    return { path, before: { sha256: before.sha256, size: before.size, mode: before.mode }, after: { sha256: after.sha256, size: after.size, mode: after.mode } };
  });
  if (retainedOwner) put(retainedOwner, "📊️summary/🔣️.json", JSON.stringify({ schemaVersion: 1, contract: vector.contract, outcomes, identities }, null, 2) + "\n");
  for (const row of identities) expect(row.after, row.path).toEqual(row.before);
});
