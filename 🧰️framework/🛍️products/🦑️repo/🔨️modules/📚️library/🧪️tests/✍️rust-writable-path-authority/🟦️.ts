import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, parse, posix, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import { parse as parseToml } from "@iarna/toml";
import { join as oracleJoin, normalize as oracleNormalize } from "pathe";
import ts from "typescript";
import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, inspectRustNonRepoJoinBaseSpans } from "../../🔍️discovery/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../"), ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../✍️rust-writable-path-authority/🔣️.json"), "utf8"));
const priorPath = join(root, vector.semantics.preservedFiniteCheckpoint.path), prior = readFileSync(priorPath, "utf8"), priorSyntax = ts.createSourceFile(priorPath, prior, ts.ScriptTarget.Latest, true);
const sourcePath = resolve(import.meta.dir, "../../🧹️normalization/🟦️.ts"), source = readFileSync(sourcePath, "utf8"), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
type Row = { id: string; source: string; targets: string[]; affected: string[]; condition: string; expected: string };
type Token = { start: number; end: number; value: string; rewriteKind?: string; physicalTargets?: string[]; physicalInterpretation?: string; unsupportedReason?: string };
const compilers = [
  { name: "Bun", compile: (text: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(text) },
  { name: "TypeScript", compile: (text: string) => ts.transpileModule(text, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];

/** 🧫️ Reuses the exact released fixture and private-pipeline harness without importing or modifying its tests. */
function harness(compiler: typeof compilers[number], inspectors: Partial<{ inspectRustManifestPathReferences: typeof inspectRustManifestPathReferences; inspectRustManifestPathCandidates: typeof inspectRustManifestPathCandidates }> = {}) {
  const initializer = (name: string): string => {
    for (const node of priorSyntax.statements) if (ts.isVariableStatement(node)) for (const declaration of node.declarationList.declarations) if (declaration.name.getText(priorSyntax) === name && declaration.initializer) return declaration.initializer.getText(priorSyntax);
    throw new Error("Missing released harness initializer: " + name);
  };
  const functions = new Function("return " + initializer("functions"))() as Set<string>, constants = new Function("return " + initializer("constants"))() as Set<string>;
  const extracted = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? functions.has(node.name?.text ?? "") : ts.isClassDeclaration(node) ? node.name?.text === "TaxonomyCancellationError" : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  const names = new Set(["newRun", "fixture", "implementation", "cargoOracle", "leafSpan"]);
  const helpers = priorSyntax.statements.filter((node) => ts.isFunctionDeclaration(node) && names.has(node.name?.text ?? "")).map((node) => node.getText(priorSyntax)).join("\n");
  const reads: string[] = [];
  const dependencies = { root, ticket, vector, runParent: join(ticket, ...vector.retention.parentSegments), extracted,
    createHash, existsSync, lstatSync, mkdirSync, mkdtempSync, symlinkSync, writeFileSync,
    readFileSync: (...args: Parameters<typeof readFileSync>) => { reads.push(String(args[0])); return (readFileSync as any)(...args); },
    basename, dirname, isAbsolute, join, parse, posix, relative, resolve, sep, parseToml, oracleJoin, oracleNormalize,
    inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, inspectRustNonRepoJoinBaseSpans, ...inspectors };
  const actual = new Function(...Object.keys(dependencies), compiler.compile(helpers) + "\nreturn { fixture, implementation, cargoOracle, leafSpan };")(...Object.values(dependencies));
  return { ...actual, reads };
}

test("exact writable route, package, and both launch registrations preserve the canonical semantic leaf", () => {
  const registration = vector.registration, project = JSON.parse(readFileSync(join(root, registration.projectPath), "utf8"));
  expect(project.targets[registration.target]).toEqual({ executor: "nx:run-commands", options: { cwd: dirname(registration.projectPath), command: registration.command } });
  const routerText = readFileSync(join(root, registration.routerPath), "utf8"), router = ts.createSourceFile(registration.routerPath, routerText, ts.ScriptTarget.Latest, true), branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node) => { if (ts.isIfStatement(node) && node.expression.getText(router) === 'segments[0] === "' + registration.route + '"') branches.push(node); ts.forEachChild(node, visit); };
  visit(router);
  expect(branches).toHaveLength(1);
  expect(branches[0]!.thenStatement.getText(router)).toContain(JSON.stringify(registration.testPath));
  expect(branches[0]!.thenStatement.getText(router)).toContain('runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot })');
  const manifest = JSON.parse(readFileSync(join(root, registration.packagePath), "utf8"));
  expect(manifest.name).toBe("@semio-tech/repo-lib");
  expect(manifest.scripts[registration.target]).toBe(registration.packageCommand);
  for (const path of registration.launchPaths) {
    const configurations = parseJsonc(readFileSync(join(root, path), "utf8")).configurations.filter((item: { name: string }) => item.name === registration.launchName);
    expect(configurations).toHaveLength(1);
    expect(configurations[0]).toEqual({ name: registration.launchName, type: "node-terminal", request: "launch", command: registration.launchCommand, cwd: "$" + "{workspaceFolder}", presentation: { group: "4_gate", order: registration.launchOrder } });
  }
});

test("closed writable authority contract preserves the released finite checkpoint exactly", () => {
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...vector, semantics: { ...vector.semantics, failure: "writable-without-proof" } })).toBe(false);
  expect(createHash("sha256").update(prior).digest("hex")).toBe(vector.semantics.preservedFiniteCheckpoint.sha256);
  expect(new Set(vector.cases.map((row: Row) => row.id)).size).toBe(vector.cases.length);
});

for (const compiler of compilers) for (const row of vector.cases as Row[]) test(compiler.name + " writable authority: " + row.id, () => {
  const h = harness(compiler), f = h.fixture(row), actual = h.implementation(compiler, f.directory);
  if (row.condition === "symlink-leaf" || row.condition === "symlink-ancestor") expect(lstatSync(join(f.directory, row.condition === "symlink-leaf" ? "foreign/item.json" : "foreign")).isSymbolicLink()).toBe(true);
  if (row.condition === "missing-writable-target") { f.known.add("foreign"); f.known.add("foreign/item.json"); }
  const index = actual.index(f.known, f.directory, f.coordinateRoots, f.known, undefined, new Set(row.affected));
  if (row.condition.startsWith("changed-")) {
    actual.graph(f.consumer, index);
    const changed = row.condition === "changed-consumer" ? f.consumer : row.condition === "changed-chain" ? f.entry : f.manifest;
    f.put(changed, readFileSync(join(f.directory, changed), "utf8") + "\n" + (changed.endsWith(".rs") ? "pub const SNAPSHOT_CHANGE: u8 = 1;\n" : 'description = "changed snapshot"\n'));
  }
  const span = h.leafSpan(row.source), tokens = actual.tokens(f.consumer, row.source, index) as Token[], rows = tokens.filter((token) => token.start === span.start && token.end === span.end);
  expect(inspectRustManifestPathReferences(row.source).some((reference) => reference.start === span.start && reference.end === span.end)).toBe(true);
  expect(rows).toHaveLength(1);
  if (row.expected === "writable") {
    expect(h.cargoOracle(f)).toHaveLength(1);
    expect(rows[0]!.rewriteKind).toBe("rust-path-join");
    expect(rows[0]!.unsupportedReason).toBeUndefined();
    expect(rows[0]!.physicalTargets).toEqual([oracleNormalize(oracleJoin(dirname(f.manifest), "../foreign", "item.json"))]);
    expect((actual.all(f.consumer, row.source, index) as Token[]).filter((token) => token.start === span.start && token.end === span.end)).toHaveLength(1);
  } else {
    expect(rows[0]!.rewriteKind).toBeUndefined();
    expect(rows[0]!.physicalInterpretation).toBeUndefined();
    expect(rows[0]!.unsupportedReason).toBeTruthy();
    expect(rows[0]!.physicalTargets?.some((target) => row.affected.includes(target))).toBe(true);
    expect((actual.all(f.consumer, row.source, index) as Token[]).some((token) => token.start === span.start && token.end === span.end && !token.rewriteKind && !token.physicalInterpretation)).toBe(true);
  }
  expect(actual.accesses.some((path: string) => ["compose", "temp/compose"].some((opaque) => path === opaque || path.startsWith(opaque + "/")))).toBe(false);
});

for (const compiler of compilers) test(compiler.name + " writable and finite paths share one source snapshot per tokenization", () => {
  const row = { ...vector.cases[0], id: "batched-source-proof", source: vector.batch.source, targets: vector.batch.targets, affected: vector.batch.targets }, h = harness(compiler), f = h.fixture(row), actual = h.implementation(compiler, f.directory);
  const index = actual.index(f.known, f.directory, [], f.known, undefined, new Set(row.affected));
  actual.graph(f.consumer, index);
  h.reads.length = 0;
  const tokens = actual.tokens(f.consumer, row.source, index) as Token[];
  for (const value of ["alpha.json", "beta.json"]) {
    const span = h.leafSpan(row.source, value), token = tokens.find((item) => item.start === span.start && item.end === span.end);
    expect(token?.rewriteKind).toBe("rust-path-join");
  }
  const finite = h.leafSpan(row.source, "gamma.json"), token = tokens.find((item) => item.start === finite.start && item.end === finite.end);
  expect(token?.physicalInterpretation).toBe("rust-finite-manifest-targets");
  for (const path of [f.manifest, f.entry, f.consumer]) expect(h.reads.filter((candidate: string) => candidate === join(f.directory, path))).toHaveLength(1);
});

for (const compiler of compilers) for (const scenario of vector.spanIntegrity as string[]) test(compiler.name + " writable proof rejects colliding source span: " + scenario, () => {
  const row = vector.cases[0] as Row, sourceReferences = inspectRustManifestPathReferences(row.source), leaf = sourceReferences.find((reference) => reference.value === "item.json")!;
  expect(leaf).toBeDefined();
  const changed = scenario.includes("immutable") ? leaf : { ...leaf, base: ["../absent"] };
  const extra = scenario === "overlapping-immutable-span" ? { ...leaf, start: leaf.start + 1, value: leaf.value.slice(1) } : { ...leaf, base: ["../absent"] };
  const references = sourceReferences.map((reference) => reference === leaf ? changed : reference);
  if (scenario.includes("immutable")) references.push(extra);
  const candidate = { start: leaf.start, end: leaf.end, value: leaf.value, targets: [["../foreign", "item.json"]] };
  if (scenario === "same-key-shorter-finite-candidate") { candidate.end--; candidate.value = candidate.value.slice(0, -1); }
  if (scenario === "covering-finite-candidate") { candidate.start--; candidate.value = row.source.slice(candidate.start, candidate.end); }
  const h = harness(compiler, { inspectRustManifestPathReferences: () => references, inspectRustManifestPathCandidates: () => scenario.includes("immutable") ? [] : [candidate] }), f = h.fixture({ ...row, id: scenario }), actual = h.implementation(compiler, f.directory);
  const tokens = actual.tokens(f.consumer, row.source, actual.index(f.known, f.directory)) as Token[];
  const overlapping = tokens.filter((token) => token.start < leaf.end && leaf.start < token.end);
  expect(overlapping.length).toBeGreaterThan(0);
  expect(overlapping.every((token) => !token.rewriteKind && !token.physicalInterpretation && Boolean(token.unsupportedReason))).toBe(true);
  expect(overlapping.some((token) => token.physicalTargets?.includes("foreign/item.json"))).toBe(true);
});

for (const compiler of compilers) test(compiler.name + " writable proof observes cancellation and changed input bytes", () => {
  const h = harness(compiler), row = vector.cases[0], f = h.fixture(row), actual = h.implementation(compiler, f.directory);
  expect(() => actual.tokens(f.consumer, row.source + "\n", actual.index(f.known, f.directory))).toThrow("source changed");
  f.put("cancel", "cancel\n");
  expect(() => actual.tokens(f.consumer, row.source, actual.index(f.known, f.directory, [], f.known, "cancel"))).toThrow("cancelled");
});

for (const scenario of [
  { id: "cancelled-module-edge", entry: '#[path = "../pkg/entry.rs"] mod owner;\nfn main() { println!("{}", owner::origin()); }\n', expected: "actual physical module\n" },
  { id: "cancelled-lib-edge", entry: '#[path = "../pkg/../alias/../pkg/entry.rs"] mod owner;\nfn main() { println!("{}", owner::ACTUAL_CRATE); }\n', expected: "true\n" },
  { id: "inherited-env-macro", entry: '#[path = "../pkg/entry.rs"] mod owner;\nfn main() { owner::run(); }\n', expected: "TARGET:actual macro target\n" },
]) test("actual rustc independently proves writable counterexample: " + scenario.id, () => {
  const h = harness(compilers[0]!), row = vector.cases.find((item: Row) => item.id === scenario.id), f = h.fixture(row);
  f.put("🧾️native/🦀️.rs", scenario.entry);
  const binary = join(f.directory, "🧾️native", process.platform === "win32" ? "🔣️.exe" : "../✍️rust-writable-path-authority/🔣️.json");
  const compile = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "writable_path_authority", join(f.directory, "🧾️native/🦀️.rs"), "-o", binary], { cwd: f.directory, env: { ...process.env, CARGO_MANIFEST_DIR: join(f.directory, "pkg") }, stdout: "pipe", stderr: "pipe" });
  expect(compile.exitCode, compile.stderr.toString()).toBe(0);
  const runtime = Bun.spawnSync([binary], { cwd: f.directory, stdout: "pipe", stderr: "pipe" });
  expect(runtime.exitCode, runtime.stderr.toString()).toBe(0);
  expect(runtime.stdout.toString()).toBe(scenario.expected);
  writeFileSync(join(f.directory, "🧾️native/📝️.md"), "# Writable Counterexample Native Oracle\n\nCase: " + scenario.id + ".\n\nRuntime stdout: " + JSON.stringify(runtime.stdout.toString()) + ".\n\nThe exact new input compiled and executed; this is not full product compilation.\n");
});
