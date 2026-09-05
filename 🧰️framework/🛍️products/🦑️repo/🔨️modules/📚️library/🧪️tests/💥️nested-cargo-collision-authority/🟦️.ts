import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, posix, resolve } from "node:path";
import Ajv from "ajv";
import stringify from "fast-json-stable-stringify";
import { escape, minimatch } from "minimatch";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";
import { semanticPackageProjectionAuthority, type SemanticPackageProjectionCatalog, type Taxonomy } from "../../🔍️discovery/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../💥️nested-cargo-collision-authority/🔣️.json"), "utf8"));
const taxonomy = JSON.parse(readFileSync(join(repoRoot, library, "🔣️taxonomy.json"), "utf8")) as Taxonomy;
const catalogBytes = readFileSync(join(repoRoot, taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"]!.authorityCatalogPath));
const catalog = JSON.parse(catalogBytes.toString()) as SemanticPackageProjectionCatalog;
const prefix = "Nested Cargo destination collision: ";
const fold = (path: string) => path.normalize("NFC").toLocaleLowerCase("und").replaceAll("\ufe0f", "");
const sha = (value: string | Buffer) => createHash("sha256").update(value).digest("hex");
type CollisionInput = { mappings: string[]; adapters: string[]; derived: string[]; allowedDirectories: string[]; occupiedPaths?: string[] };
type Candidate = { sourcePath: string };

/** 🧬️ Evaluates only the two reviewed production expressions, without exposing a testing runtime API. */
function productionExpressions() {
  const source = (path: string, name: string) => {
    const tree = ts.createSourceFile(path, readFileSync(join(repoRoot, library, path), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const rows = tree.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name);
    expect(rows).toHaveLength(1);
    return { tree, node: rows[0]! };
  };
  const discovery = source("🔍️discovery/🟦️.ts", "semanticPackageProjectionAuthority");
  const blocks = discovery.node.body!.statements.filter((node): node is ts.IfStatement => ts.isIfStatement(node) && node.expression.getText(discovery.tree) === "!destination");
  expect(blocks).toHaveLength(1);
  const collisionBody = blocks[0]!.getText(discovery.tree);
  const collision = new Function(ts.transpileModule(`function run(row, adapters, derived, allowedDirectories, facts) { const problems = []; const destination = false; ${collisionBody} return problems; }`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText + "\nreturn run;")() as (row: object, adapters: Map<string, null>, derived: Map<string, null>, allowedDirectories: Set<string>, facts: object) => string[];
  const normalization = source("🧹️normalization/🟦️.ts", "projectNestedCargoPackages"), initializers: ts.Expression[] = [];
  const visit = (node: ts.Node) => { if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.name.text === "occupiedPaths" && node.initializer) initializers.push(node.initializer); ts.forEachChild(node, visit); };
  visit(normalization.node);
  expect(initializers).toHaveLength(1);
  const membershipBody = initializers[0]!.getText(normalization.tree);
  const membership = new Function(ts.transpileModule(`function run(entries, candidates, admitted) { return ${membershipBody}; }`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText + "\nreturn run;")() as (entries: Map<string, null>, candidates: Candidate[], admitted: Set<string>) => string[];
  return {
    collisions: (input: CollisionInput) => collision({ mappings: input.mappings.map((destinationPath) => ({ destinationPath })) }, new Map(input.adapters.map((path) => [path, null])), new Map(input.derived.map((path) => [path, null])), new Set(input.allowedDirectories), { occupiedPaths: input.occupiedPaths }),
    membership,
    collisionHash: sha(collisionBody),
    membershipHash: sha(membershipBody),
  };
}

const production = productionExpressions();

/** 🧾️ Retains the prior algorithm solely as a bounded test oracle, never as production fallback. */
function priorCollisions(input: CollisionInput): string[] {
  const outputs = new Set([...input.mappings, ...input.adapters, ...input.derived]), allowed = new Set(input.allowedDirectories), problems: string[] = [];
  for (const path of input.occupiedPaths ?? []) if ([...outputs].some((output) => fold(path) === fold(output) || fold(output).startsWith(fold(path) + "/")) && !allowed.has(path)) problems.push(prefix + path);
  return problems;
}

/** 🔬️ Uses minimatch's literal escaping and ancestor matching as the independent relation oracle. */
function independentCollisions(input: CollisionInput): string[] {
  const options = { dot: true, nonegate: true, nocomment: true, noext: true, nobrace: true, platform: "linux" as const };
  const outputs = [...new Set([...input.mappings, ...input.adapters, ...input.derived])].map(fold);
  return (input.occupiedPaths ?? []).filter((path) => !input.allowedDirectories.includes(path) && outputs.some((output) => minimatch(output, escape(fold(path)), options) || minimatch(output, escape(fold(path)) + "/**", options))).map((path) => prefix + path);
}

/** 📏️ Counts ICU-bound fold calls synchronously and restores the exact native descriptor. */
function foldedCalls<T>(run: () => T): { value: T; calls: Map<string, number> } {
  const descriptor = Object.getOwnPropertyDescriptor(String.prototype, "toLocaleLowerCase")!, native = descriptor.value, calls = new Map<string, number>();
  Object.defineProperty(String.prototype, "toLocaleLowerCase", { ...descriptor, value: function(this: string, locales?: Intl.LocalesArgument) {
    const path = String(this);
    calls.set(path, (calls.get(path) ?? 0) + 1);
    return native.call(this, locales);
  } });
  try { return { value: run(), calls }; } finally { Object.defineProperty(String.prototype, "toLocaleLowerCase", descriptor); }
}

test("neutral collision schema pins exact Unicode order and diagnostic multiplicity", () => {
  expect(vector.schemaVersion).toBe(1);
  expect(vector.contract.fold).toEqual(["NFC", "toLocaleLowerCase:und", "remove:U+FE0F"]);
  expect(vector.contract.allowedDirectories).toBe("exact-unfolded-identity");
  expect(vector.contract.diagnostics).toBe("occupied-input-order-with-duplicates");
  expect(vector.contract.reuse).toBe("invocation-local");
  const valid = new Ajv().compile(vector.caseSchema);
  for (const row of vector.cases) expect(valid(row), JSON.stringify(valid.errors)).toBe(true);
  expect(valid({ ...vector.cases[0], undocumented: true })).toBe(false);
  expect(valid({ id: "missing-input", collisions: [] })).toBe(false);
});

for (const row of vector.cases) test("source collision authority: " + row.id, () => {
  const input = { ...vector.fixture, occupiedPaths: row.occupiedPaths }, before = stringify(input), expected = row.collisions.map((path: string) => prefix + path);
  expect(independentCollisions(input)).toEqual(expected);
  expect(priorCollisions(input)).toEqual(expected);
  expect(production.collisions(input)).toEqual(expected);
  expect(stringify(input)).toBe(before);
});

test("source collision reuse remains invocation-local and preserves missing occupied input", () => {
  expect(production.collisions(vector.fixture)).toEqual([]);
  const input = { ...structuredClone(vector.fixture), occupiedPaths: ["fresh/leaf"] };
  expect(production.collisions(input)).toEqual([]);
  input.derived.push("fresh/leaf");
  expect(production.collisions(input)).toEqual([prefix + "fresh/leaf"]);
  input.allowedDirectories.push("fresh/leaf");
  expect(production.collisions(input)).toEqual([]);
  input.allowedDirectories.pop();
  input.occupiedPaths.push("fresh/leaf");
  expect(production.collisions(input)).toEqual([prefix + "fresh/leaf", prefix + "fresh/leaf"]);
});

test("source collision fold work is bounded by distinct outputs plus occupied occurrences", () => {
  const occupiedPaths = Array.from({ length: vector.performance.noisePaths }, (_, index) => `unrelated-${index}/leaf`), input = { ...vector.fixture, occupiedPaths };
  const actual = foldedCalls(() => production.collisions(input)), prior = foldedCalls(() => priorCollisions(input));
  expect(actual.value).toEqual(prior.value);
  expect(actual.value).toEqual([]);
  const outputCount = new Set<string>([...input.mappings, ...input.adapters, ...input.derived]).size;
  expect([...prior.calls.values()].reduce((sum, value) => sum + value, 0)).toBe(4 * outputCount * occupiedPaths.length);
  expect([...actual.calls.values()].reduce((sum, value) => sum + value, 0)).toBe(outputCount + occupiedPaths.length);
  for (const path of occupiedPaths) expect(actual.calls.get(path)).toBe(vector.contract.maxFoldsPerOccupiedPath);
});

for (const row of vector.membershipCases) test("occupied membership retains exact identity and order: " + row.id, () => {
  const entries = new Map<string, null>(row.entries.map((path: string) => [path, null])), candidates: Candidate[] = row.candidates.map((sourcePath: string) => ({ sourcePath })), admitted = new Set<string>(row.candidates);
  expect([...entries.keys()].filter((path) => !candidates.some((entry) => entry.sourcePath === path))).toEqual(row.occupiedPaths);
  expect(production.membership(entries, candidates, admitted)).toEqual(row.occupiedPaths);
});

test("occupied membership reuses the existing admission set without rescanning candidates", () => {
  let reads = 0;
  const paths = Array.from({ length: 32 }, (_, index) => `owner/${index}`);
  const candidates = paths.map((path) => ({ get sourcePath() { reads++; return path; } }));
  const admitted = new Set(paths), entries = new Map<string, null>([...paths, ...paths.map((path) => "outside/" + path)].map((path) => [path, null]));
  expect(production.membership(entries, candidates, admitted)).toEqual(paths.map((path) => "outside/" + path));
  expect(reads).toBe(vector.contract.maxCandidateReadsDuringOccupiedSelection);
});

test("full current package diagnostics retain complete ordered parity without reading live sources", () => {
  expect(sha(catalogBytes)).toBe(taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"]!.authorityCatalogSha256);
  const records = [];
  for (const row of catalog.packages) {
    const allowed = new Set<string>();
    for (const mapping of row.mappings) for (let path = posix.dirname(mapping.sourcePath); path !== "."; path = posix.dirname(path)) allowed.add(path);
    const input: CollisionInput = { mappings: row.mappings.map((mapping) => mapping.destinationPath), adapters: row.adapters.map((adapter) => adapter.path), derived: row.derivedLeaves.map((leaf) => leaf.path), allowedDirectories: [...allowed] };
    input.occupiedPaths = [row.mappings[0]!.destinationPath, row.destinationRoot, row.semanticOwnerRoot, ...allowed, row.mappings[0]!.destinationPath.replaceAll("\ufe0f", ""), row.mappings[0]!.destinationPath + "/child", row.mappings[0]!.destinationPath, ...Array.from({ length: 32 }, (_, index) => `unrelated-${index}/leaf`)];
    const base = semanticPackageProjectionAuthority({ packageId: row.id, nodes: [] }, catalog, taxonomy), collisions = independentCollisions(input), expectedProblems = [...base.problems];
    const insertion = expectedProblems.indexOf("Nested Cargo package name is not the exact registered identity");
    expect(insertion).toBeGreaterThanOrEqual(0);
    expectedProblems.splice(insertion, 0, ...collisions);
    const facts = { packageId: row.id, nodes: [], occupiedPaths: input.occupiedPaths };
    const actual = semanticPackageProjectionAuthority(facts, catalog, taxonomy);
    expect(production.collisions(input)).toEqual(collisions);
    expect(priorCollisions(input)).toEqual(collisions);
    expect(actual).toEqual({ ...base, problems: expectedProblems });
    expect(actual.problems.some((problem) => problem.startsWith("Missing nested Cargo regular leaf: "))).toBe(true);
    expect(actual.mappings).toEqual([]);
    const destination = semanticPackageProjectionAuthority({ ...facts, layout: "destination" }, catalog, taxonomy);
    expect(destination.problems.filter((problem) => problem.startsWith(prefix))).toEqual([]);
    records.push({ packageId: row.id, problems: actual.problems.length, collisions: collisions.length, digest: sha(stringify(actual)) });
  }
  console.log("[DEBUG] Complete collision diagnostic parity", JSON.stringify({ catalogSha256: sha(catalogBytes), collisionBodySha256: production.collisionHash, membershipBodySha256: production.membershipHash, records }));
});

test("bounded alternating collision timings preserve identical outputs", () => {
  const input = { ...vector.fixture, occupiedPaths: Array.from({ length: vector.performance.timingNoisePaths }, (_, index) => `timing-${index}/leaf`) };
  const results: { priorMs: number; currentMs: number }[] = [];
  const measure = (run: () => string[]) => { const start = performance.now(), result = run(); expect(result).toEqual([]); return performance.now() - start; };
  priorCollisions({ ...input, occupiedPaths: input.occupiedPaths.slice(0, 4) });
  production.collisions({ ...input, occupiedPaths: input.occupiedPaths.slice(0, 4) });
  for (let round = 0; round < vector.performance.timingRounds; round++) {
    const row = { priorMs: 0, currentMs: 0 };
    for (const variant of round % 2 ? ["currentMs", "priorMs"] as const : ["priorMs", "currentMs"] as const) row[variant] = measure(() => variant === "priorMs" ? priorCollisions(input) : production.collisions(input));
    results.push(row);
  }
  console.log("[DEBUG] Bounded collision timing samples", JSON.stringify({ occupiedPaths: input.occupiedPaths.length, outputPaths: new Set([...input.mappings, ...input.adapters, ...input.derived]).size, results }));
});

test("collision gate is registered through Nx and both ordered launch catalogs", () => {
  const expected = vector.execution, project = JSON.parse(readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(repoRoot, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.group, order: expected.order });
  }
});
