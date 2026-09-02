//#region Imports
import { describe, expect, test } from "bun:test";
import { createHash, webcrypto } from "node:crypto";
import { closeSync, constants, existsSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import fastGlob from "fast-glob";
import { findNodeAtLocation, getNodeValue, parseTree } from "jsonc-parser";
import { clearDiscoveryCache, discoverPackageProblems, loadCatalogTaxonomy, validateFrozenCoordinateEvidenceContracts, type FrozenCoordinateEvidenceContract } from "../../🔍️discovery/🟦️.ts";
import { applyTaxonomyPlan, frozenCoordinateEvidenceCoordinates, inventoryTaxonomy, planTaxonomy } from "../../🧹️normalization/🟦️.ts";
//#endregion Imports

//#region Authority
const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(libraryRoot, "../../../../..");
const ticketRoot = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const goldenPath = join(libraryRoot, vector.historicalDocument.libraryRelativePath);
const goldenBytes = readFileSync(goldenPath);
const golden = JSON.parse(goldenBytes.toString());
const sha = (bytes: Uint8Array | string) => createHash("sha256").update(bytes).digest("hex");
const sourceBytes = () => {
  const bytes = Buffer.from(vector.sourcePreimage.bytes, vector.sourcePreimage.encoding);
  if (sha(bytes) !== vector.sourcePreimage.sha256 || bytes.length !== vector.sourcePreimage.size) throw new Error("Retained preimage drift");
  return bytes;
};
const declarationSchema = {
  type: "object", additionalProperties: false, required: ["pointer", "kind", "representation", "identityPrefix"],
  properties: { pointer: { type: "string" }, kind: { const: "source" }, representation: { const: "recorded-package-owner-identity" }, identityPrefix: { const: "unmarked:" } },
};
//#endregion Authority

//#region Tests
test("historical owner identity declaration and suffix agree with Ajv and jsonc-parser", () => {
  const validate = new Ajv({ allErrors: true }).compile(declarationSchema);
  for (const row of vector.identityCases) {
    const bytes = Buffer.from(JSON.stringify({ schemaVersion: 1, packageId: row.value }));
    const coordinate = { pointer: "/packageId", kind: row.kind, representation: "recorded-package-owner-identity", identityPrefix: row.identityPrefix };
    const contracts = { owner: { path: "🔣️history.json", sha256: sha(bytes), schemaVersion: 1, coordinates: [coordinate] } };
    expect(validate(coordinate)).toBe(row.identityPrefix === "unmarked:" && row.kind === "source");
    expect(validateFrozenCoordinateEvidenceContracts(contracts).length === 0).toBe(validate(coordinate));
    const run = () => frozenCoordinateEvidenceCoordinates(contracts.owner.path, bytes, contracts as never);
    if (row.expected === "reject") expect(run, row.id).toThrow(/frozen-coordinate-evidence-invalid/u);
    else {
      const node = findNodeAtLocation(parseTree(bytes.toString())!, ["packageId"])!;
      expect(run()).toEqual([{ pointer: "/packageId", kind: "source", start: node.offset + 1 + row.identityPrefix.length, end: node.offset + node.length - 1, value: row.coordinate }]);
    }
  }
});

test("genuine census retains exact bytes and five typed historical coordinates", () => {
  expect(sha(goldenBytes)).toBe(vector.historicalDocument.sha256);
  expect(goldenBytes.length).toBe(vector.historicalDocument.size);
  const contract = { path: relative(repoRoot, goldenPath).replaceAll("\\", "/"), sha256: sha(goldenBytes), schemaVersion: 1, coordinates: vector.historicalCoordinates };
  const actual = frozenCoordinateEvidenceCoordinates(contract.path, goldenBytes, { purity: contract })!;
  expect(actual).toHaveLength(5);
  const tree = parseTree(goldenBytes.toString())!;
  for (const declaration of vector.historicalCoordinates) {
    const node = findNodeAtLocation(tree, declaration.pointer.slice(1).split("/").map((part: string) => /^\d+$/.test(part) ? Number(part) : part))!;
    const prefix = declaration.identityPrefix ?? "";
    expect(actual.find((row) => row.pointer === declaration.pointer)).toEqual({ pointer: declaration.pointer, kind: declaration.kind, start: node.offset + 1 + prefix.length, end: node.offset + node.length - 1, value: node.value.slice(prefix.length) });
  }
  const mapping = golden.mappings[vector.historicalDocument.mappingIndex];
  expect(mapping[3]).toBe("unmarked:" + mapping[4]);
  expect(sha(sourceBytes())).toBe(mapping[1]);
  const fields = Object.fromEntries(golden.mappingFields.map((name: string, index: number) => [name, index]));
  expect(sha(JSON.stringify(golden.mappings.map((row: unknown[]) => [row[fields.classifierRole], row[fields.sourcePath]])))).toBe(golden.census.sourceSetSha256);
  expect(() => frozenCoordinateEvidenceCoordinates(contract.path, Buffer.concat([goldenBytes, Buffer.from("\n")]), { purity: contract })).toThrow(/frozen-coordinate-evidence-invalid/u);
});

test("genuine historical purity authority is registered with no broader span ownership than the deliberately widened set", () => {
  const live = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts["remaining-package-purity-history-v1"];
  expect(live).toMatchObject({ path: relative(repoRoot, goldenPath).replaceAll("\\", "/"), sha256: vector.historicalDocument.sha256, schemaVersion: 1 });
  // The four non-sourcePath fields this identity test itself depends on (row 29's owner-identity,
  // ownerRoot, packageRoot, canonicalPackageRoot) are still exactly, individually, what they always
  // were — only the sourcePath declaration changed shape (see below), never these four.
  expect(live!.coordinates.filter((row) => row.pointer !== "/mappings/*/0" && row.pointer !== "/mappings/69/10")).toEqual(vector.historicalCoordinates.filter((row: { pointer: string }) => row.pointer !== "/mappings/29/0"));
  // The only widening beyond that: a row-index wildcard over the sourcePath column (covering row
  // 29's own sourcePath along with every other row's), plus one explicit destinationPath addition
  // for a row a later, wider rename scope touched. `🧪️frozen-coordinate-wildcard-coverage` proves
  // that widening is sound (safe to wildcard column 0, unsafe to wildcard column 10) and exactly
  // bounded (does not touch row 29's other four fields).
  expect(live!.coordinates.some((row) => row.pointer === "/mappings/*/0" && row.kind === "source")).toBe(true);
  expect(live!.coordinates.some((row) => row.pointer === "/mappings/69/10" && row.kind === "destination")).toBe(true);
  expect(live!.coordinates).toHaveLength(vector.historicalCoordinates.length + 1);
});

test("historical owner identity gate is registered in Nx and both launch catalogs", () => {
  const target = vector.execution;
  const projectBytes = readFileSync(join(libraryRoot, target.projectLibraryRelativePath), "utf8");
  const project = JSON.parse(projectBytes);
  expect(project).toEqual(getNodeValue(parseTree(projectBytes)!));
  expect(project.targets[target.target]?.options.command).toBe(target.command);
  const launches = target.launchCatalogs.map((path: string) => {
    const catalog = getNodeValue(parseTree(readFileSync(join(repoRoot, path), "utf8"))!);
    const entries = catalog.configurations.filter((entry: { name: string }) => entry.name === target.launchName);
    expect(entries).toHaveLength(1);
    expect(entries[0]).toEqual({ name: target.launchName, type: "node-terminal", request: "launch", command: target.launchCommand, cwd: "${workspaceFolder}", presentation: { group: target.group, order: target.order } });
    return entries[0];
  });
  expect(launches[0]).toEqual(launches[1]);
});

test("current isolated census follows moved added and retired files without reopening historical source paths", async () => {
  const root = mkdtempSync(join(ticketRoot, "🧪️purity-current-"));
  const taxonomy = loadCatalogTaxonomy();
  const init = Bun.spawnSync(["git", "init", "-q"], { cwd: root, stdout: "pipe", stderr: "pipe" });
  expect(init.exitCode, init.stderr.toString()).toBe(0);
  const write = (path: string, bytes: string | Buffer) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), bytes); };
  const move = (source: string, destination: string) => { mkdirSync(dirname(join(root, destination)), { recursive: true }); renameSync(join(root, source), join(root, destination)); };
  const observe = (expected: string[]) => {
    const git = Bun.spawnSync(["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", "🧰️framework"], { cwd: root, stdout: "pipe", stderr: "pipe" });
    expect(git.exitCode, git.stderr.toString()).toBe(0);
    const admitted = new Set(git.stdout.toString().split("\0").filter(Boolean));
    const current = () => { clearDiscoveryCache(); return discoverPackageProblems(root, taxonomy).filter((row) => admitted.has(row.path) && (row.kind === "package-implementation" || row.kind === "package-role-unresolved")).map((row) => row.path).sort(); };
    expect(current()).toEqual([...expected].sort());
    expect(current()).toEqual(fastGlob.sync("**/📦️packages/**/*.rs", { cwd: root, onlyFiles: true, dot: true, followSymbolicLinks: false }).sort());
    expect(sha(readFileSync(goldenPath))).toBe(vector.historicalDocument.sha256);
    expect(sha(sourceBytes())).toBe(vector.sourcePreimage.sha256);
  };
  write(vector.fixture.source, sourceBytes());
  write(join(dirname(vector.fixture.source), "Cargo.toml"), vector.fixture.manifest);
  observe([vector.fixture.source]);
  move(vector.fixture.source, vector.fixture.relocated);
  write(join(dirname(vector.fixture.relocated), "Cargo.toml"), vector.fixture.manifest);
  expect(existsSync(join(root, vector.fixture.source))).toBe(false);
  observe([vector.fixture.relocated]);
  write(vector.fixture.added, vector.fixture.extraSource);
  observe([vector.fixture.relocated, vector.fixture.added]);
  write(vector.fixture.relocated, vector.fixture.extraSource);
  expect(sha(readFileSync(join(root, vector.fixture.relocated)))).not.toBe(vector.sourcePreimage.sha256);
  expect(sha(vector.fixture.extraSource)).toBe(Buffer.from(await webcrypto.subtle.digest("SHA-256", Buffer.from(vector.fixture.extraSource))).toString("hex"));
  move(vector.fixture.relocated, "🦀️retired.rs");
  move(vector.fixture.added, "➕️retiredextra.rs");
  observe([]);
  write(vector.fixture.source, sourceBytes());
  observe([vector.fixture.source]);
});
test("genuine historical census remains unchanged through a scoped Draw transaction", () => {
  const root = mkdtempSync(join(ticketRoot, "🧪️purity-transaction-"));
  const write = (path: string, bytes: string | Buffer) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), bytes); };
  const git = (args: string[]) => { const run = Bun.spawnSync(["git", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" }); if (run.exitCode) throw new Error(run.stderr.toString()); return run.stdout.toString().trim(); };
  const taxonomy = structuredClone(loadCatalogTaxonomy());
  const schemaPath = golden.taxonomy.path;
  const historyPath = relative(repoRoot, goldenPath).replaceAll("\\", "/");
  const catalog = JSON.parse(readFileSync(join(libraryRoot, "📦️packages/🟦️typescript/🧫️fixtures/🔣️cad-draw-path-projection.json"), "utf8"));
  const projection = catalog.projections[1];
  for (const row of projection.mappings) write(row.sourcePath, row.sourcePath === golden.mappings[29][0] ? sourceBytes() : row.sourcePath.endsWith("Cargo.toml") ? vector.fixture.manifest : "pub fn fixture() -> usize { 1 }\n");
  delete taxonomy.generatorContracts["plugin-registry"]!.inputDiscovery;
  taxonomy.generatorContracts["plugin-registry"]!.inputPatterns = ["🧪️unrelated-input"];
  taxonomy.frozenCoordinateEvidenceContracts = { ...taxonomy.frozenCoordinateEvidenceContracts, "remaining-package-purity-history-v1": { path: historyPath, sha256: sha(goldenBytes), schemaVersion: 1, coordinates: vector.historicalCoordinates } };
  write(schemaPath, JSON.stringify(taxonomy));
  write(historyPath, goldenBytes);
  write("🔣️neighbor.json", JSON.stringify({ sourcePath: golden.mappings[29][0] }));
  git(["init", "-q"]);
  git(["config", "user.name", "Historical Purity Fixture"]);
  git(["config", "user.email", "historical-purity@invalid.example"]);
  git(["config", "commit.gpgsign", "false"]);
  git(["add", "--all"]);
  git(["commit", "-q", "-m", "historical purity fixture"]);
  const baselineCommit = git(["rev-parse", "HEAD"]);
  const plan = (scope: string) => planTaxonomy(inventoryTaxonomy({ repoRoot: root, scope, workers: 1 }), { baselineCommit, excludedTreeDigests: [] });
  const source = plan(projection.sourceRoot);
  expect(source.unresolved.filter((row) => row.severity === "error")).toEqual([]);
  expect(source.moves).toHaveLength(11);
  expect(source.regenerations).toHaveLength(0);
  expect(source.edits.filter((row) => row.path === historyPath)).toHaveLength(0);
  expect(source.edits.filter((row) => row.path === "🔣️neighbor.json")).toHaveLength(1);
  const changed = Buffer.from(JSON.stringify({ ...golden, extraOwner: golden.mappings[29][3] }));
  const altered = structuredClone(taxonomy);
  altered.frozenCoordinateEvidenceContracts["remaining-package-purity-history-v1"] = { ...altered.frozenCoordinateEvidenceContracts["remaining-package-purity-history-v1"]!, sha256: sha(changed) };
  write(historyPath, changed);
  write(schemaPath, JSON.stringify(altered));
  const unowned = plan(projection.sourceRoot);
  expect(unowned.unresolved.some((row) => row.code === "frozen-coordinate-evidence-unowned" && row.path === historyPath)).toBe(true);
  expect(unowned.edits.filter((row) => row.path === historyPath)).toHaveLength(0);
  write(historyPath, goldenBytes);
  write(schemaPath, JSON.stringify(taxonomy));
  const options = { repoRoot: root, ticketDir: join(root, "🧪️transaction"), expectedBaselineCommit: baselineCommit, expectedPlanDigest: source.planDigest };
  expect(applyTaxonomyPlan(source, { ...options, injectFailureAt: "after-edits" }).state).toBe("rolled-back");
  expect(readFileSync(join(root, historyPath))).toEqual(goldenBytes);
  expect(existsSync(join(root, golden.mappings[29][0]))).toBe(true);
  expect(applyTaxonomyPlan(source, options).state).toBe("committed");
  expect(readFileSync(join(root, historyPath))).toEqual(goldenBytes);
  const canonical = plan(projection.destinationRoot);
  expect({ moves: canonical.moves.length, edits: canonical.edits.length, errors: canonical.unresolved.filter((row) => row.severity === "error") }).toEqual({ moves: 0, edits: 0, errors: [] });
  expect(lstatSync(options.ticketDir).isDirectory()).toBe(true);
  rmSync(options.ticketDir, { recursive: true });
  expect(existsSync(options.ticketDir)).toBe(false);
}, 120_000);
//#endregion Tests

describe("Energy historical source coordinates", () => {
  const input = JSON.parse(readFileSync(join(import.meta.dir, "🧬️energy-source-coordinates/🔣️.json"), "utf8"));
  const expected = Object.fromEntries(input.documents.map((row: { id: string; registration: FrozenCoordinateEvidenceContract; energy: { declaration: FrozenCoordinateEvidenceContract["coordinates"][number] }[] }) => [row.id, { ...row.registration, coordinates: [...row.registration.coordinates, ...row.energy.map((entry) => entry.declaration)] }])) as Record<string, FrozenCoordinateEvidenceContract>;
  const schema = loadCatalogTaxonomy();
  const contracts = Object.fromEntries(Object.keys(expected).map((id) => [id, schema.frozenCoordinateEvidenceContracts[id]!]));
  const valuePin = ({ value, ...coordinate }: { pointer: string; kind: string; start: number; end: number; value: string }) => ({ ...coordinate, valueSha256: sha(value), valueUtf8Bytes: Buffer.byteLength(value, "utf8") });
  const pointerParts = (pointer: string) => pointer.slice(1).split("/").map((part) => /^(?:0|[1-9][0-9]*)$/u.test(part) ? Number(part) : part.replaceAll("~1", "/").replaceAll("~0", "~"));
  const oracle = (content: string, pointer: string) => {
    const errors: import("jsonc-parser").ParseError[] = [];
    const tree = parseTree(content, errors, { allowTrailingComma: false, disallowComments: true })!;
    expect(errors).toEqual([]);
    expect(getNodeValue(tree)).toEqual(JSON.parse(content));
    const node = findNodeAtLocation(tree, pointerParts(pointer))!;
    expect(node.type).toBe("string");
    expect(content.slice(node.offset + 1, node.offset + node.length - 1)).toBe(node.value);
    return { pointer, kind: "source", start: node.offset + 1, end: node.offset + node.length - 1, value: node.value };
  };
  const document = (row: { registration: FrozenCoordinateEvidenceContract; size: number; mode: number }) => {
    const parts = row.registration.path.split("/");
    expect(parts.every((part) => part && part !== "." && part !== "..")).toBe(true);
    expect(/^(?:compose|temp\/compose)(?:\/|$)/u.test(row.registration.path)).toBe(false);
    let path = repoRoot;
    for (const [index, part] of parts.entries()) {
      path = join(path, part);
      const stat = lstatSync(path);
      expect(stat.isSymbolicLink()).toBe(false);
      expect(index === parts.length - 1 ? stat.isFile() : stat.isDirectory()).toBe(true);
    }
    const fd = openSync(path, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
    try {
      const before = fstatSync(fd), bytes = readFileSync(fd), after = fstatSync(fd), linked = lstatSync(path);
      expect(before.isFile()).toBe(true);
      expect({ size: before.size, mode: before.mode & 0o777, sha256: sha(bytes) }).toEqual({ size: row.size, mode: row.mode, sha256: row.registration.sha256 });
      expect([after.dev, after.ino, after.size, after.mode, after.mtimeMs, after.ctimeMs]).toEqual([before.dev, before.ino, before.size, before.mode, before.mtimeMs, before.ctimeMs]);
      expect([linked.dev, linked.ino, linked.isSymbolicLink()]).toEqual([before.dev, before.ino, false]);
      return bytes;
    } finally { closeSync(fd); }
  };

  test("register exactly ten additions without changing the eight historical authorities", () => {
    expect(input.counts).toEqual({ documents: 8, existingDraw: 23, addedEnergy: 10, total: 33 });
    expect(input.coordinateValueEncoding).toEqual({ algorithm: "sha256", lengthUnit: "utf8-byte", spanUnit: "utf16-code-unit", endExclusive: true });
    expect(input.negativeCases).toHaveLength(16);
    expect(new Set(input.negativeCases.map((row: { id: string }) => row.id)).size).toBe(16);
    expect(input.documents.reduce((count: number, row: { registration: FrozenCoordinateEvidenceContract }) => count + row.registration.coordinates.length, 0)).toBe(23);
    expect(input.documents.reduce((count: number, row: { energy: unknown[] }) => count + row.energy.length, 0)).toBe(10);
    expect(contracts).toEqual(expected);
    expect(validateFrozenCoordinateEvidenceContracts(contracts)).toEqual([]);
    for (const contract of Object.values(contracts)) {
      expect(contract.schemaVersion).toBeNull();
      expect(contract.coordinates.every((coordinate) => coordinate.kind === "source" && !coordinate.pointer.includes("*"))).toBe(true);
    }
  });

  test("public helper preserves all 33 exact spans with independent JSON oracle parity", () => {
    let count = 0;
    for (const row of input.documents) {
      const bytes = document(row), content = bytes.toString(), actual = frozenCoordinateEvidenceCoordinates(row.registration.path, bytes, contracts)!;
      const prior = frozenCoordinateEvidenceCoordinates(row.registration.path, bytes, { historical: row.registration })!;
      const coordinates = expected[row.id]!.coordinates.map((declaration) => oracle(content, declaration.pointer)).sort((left, right) => left.start - right.start);
      expect(actual, row.id).toEqual(coordinates);
      expect(prior).toEqual(coordinates.filter((coordinate) => row.registration.coordinates.some((declaration: { pointer: string }) => declaration.pointer === coordinate.pointer)));
      for (const addition of row.energy) {
        expect(valuePin(oracle(content, addition.declaration.pointer))).toEqual(addition.coordinate);
        expect(valuePin(actual.find((coordinate) => coordinate.pointer === addition.declaration.pointer)!)).toEqual(addition.coordinate);
      }
      expect(document(row)).toEqual(bytes);
      count += actual.length;
    }
    expect(count).toBe(33);
  });

  test("closed approval rejects wider selectors and any changed historical identity", () => {
    const ajv = new Ajv({ strict: true });
    for (const row of input.documents) {
      const approved = expected[row.id]!, validate = ajv.compile({ const: approved });
      expect(validate(approved)).toBe(true);
      const first = row.energy[0].declaration;
      const variants = [
        { ...approved, sha256: "0".repeat(64) },
        { ...approved, schemaVersion: 1 },
        { ...approved, path: "🔣️neighbor.json" },
        { ...approved, coordinates: approved.coordinates.slice(1) },
        { ...approved, coordinates: [...approved.coordinates, first] },
        { ...approved, coordinates: [{ ...first, pointer: first.pointer.replace(/\/[0-9]+\//u, "/*/") }] },
        { ...approved, coordinates: [...approved.coordinates, { ...first, pointer: "/neighbor" }] },
        { ...approved, coordinates: approved.coordinates.map((coordinate) => "representation" in coordinate ? { ...coordinate, recordedRepositoryRoot: "/recorded/foreign" } : { ...coordinate, representation: "recorded-repository-absolute", recordedRepositoryRoot: "/recorded/foreign" }) },
      ];
      for (const variant of variants) expect(validate(variant), row.id).toBe(false);
    }
  });

  test("neighboring undeclared Energy values do not gain coordinate authority", () => {
    for (const neighbor of input.undeclaredNeighbors) {
      const row = input.documents.find((entry: { id: string }) => entry.id === neighbor.contractId)!;
      const bytes = document(row), actual = frozenCoordinateEvidenceCoordinates(row.registration.path, bytes, contracts)!;
      const { contractId, ...coordinate } = neighbor;
      expect(valuePin(oracle(bytes.toString(), neighbor.pointer))).toEqual({ ...coordinate, kind: "source" });
      expect(actual.some((coordinate) => coordinate.pointer === neighbor.pointer || coordinate.start === neighbor.start)).toBe(false);
    }
    const scenario = input.duplicateValueCase, source = input.documents.find((row: { id: string }) => row.id === scenario.valueFrom.contractId)!;
    const value = oracle(document(source).toString(), scenario.valueFrom.pointer).value;
    const content = JSON.stringify({ selected: value, neighbor: value }), bytes = Buffer.from(content);
    const contract = { path: "🔣️history.json", sha256: sha(bytes), schemaVersion: null, coordinates: scenario.coordinates };
    const selected = oracle(content, scenario.selectedPointer), neighbor = oracle(content, scenario.unownedPointer);
    expect(selected.value).toBe(neighbor.value);
    expect(selected.start).not.toBe(neighbor.start);
    expect(frozenCoordinateEvidenceCoordinates(contract.path, bytes, { exact: contract })).toEqual([selected]);
    expect(frozenCoordinateEvidenceCoordinates("🔣️neighbor.json", bytes, { exact: contract })).toBeNull();
  });

  for (const row of input.negativeCases) test("rejects " + row.id, () => {
    const bytes = Buffer.from(row.document), contract = { path: "🔣️history.json", sha256: sha(row.registeredDocument ?? bytes), schemaVersion: row.schemaVersion, coordinates: row.coordinates };
    const registry: Record<string, FrozenCoordinateEvidenceContract> = { exact: contract };
    if (row.duplicateOwner) registry.duplicate = contract;
    expect(() => frozenCoordinateEvidenceCoordinates(contract.path, bytes, registry)).toThrow(row.error);
  });
});
