import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, parse, posix, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import { parse as parseToml } from "@iarna/toml";
import { join as oracleJoin, normalize as oracleNormalize } from "pathe";
import ts from "typescript";
import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, inspectRustNonRepoJoinBaseSpans, rustTokens as rustSyntaxTokens, rustTokenPairs } from "../../🔍️discovery/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../");
const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const sourcePath = resolve(import.meta.dir, "../../🧹️normalization/🟦️.ts");
const source = readFileSync(sourcePath, "utf8"), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const marker = "rust-finite-manifest-targets";
type Token = { start: number; end: number; value: string; structuredLocation: string; adapter: string; physicalTargets?: string[]; physicalInterpretation?: string; rewriteKind?: string; unsupportedReason?: string };
type Row = { id: string; source: string; targets: string[]; expected: string; affected: string[]; condition: string };
const functions = new Set(["sha256", "canonicalArrayKey", "canonicalValue", "canonicalJson", "generatorPathCompare", "sourceRelative", "normalizeRelative", "assertNoFollowAncestors", "assertLexicalInputOutsideOpaque", "lstatOrNull", "checkCancellation", "ancestorReferenceCoordinateRoot", "lineLocation", "regexTokens", "rustTokens", "rustCodeOnlyTextForMacroTrust", "referenceTokens", "referenceAdapter", "unsupportedReferenceTokens", "addUniqueIndex", "referencePathIndex", "rustContextFiles", "unprovenRustReferenceTargets", "rustReferenceNeedsOwnership", "rustReferenceGraph", "rustFiniteManifestTargets", "rustManifestReferenceTokens", "rustReferenceInterpretationCovers", "referenceTokensIncludingUnsupported", "splitTokenSuffix", "resolveReferencePath", "resolveReferenceTokenPath"]);
const constants = new Set(["LEXICAL_OPAQUE_ROOTS", "RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS", "RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS", "RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS", "RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES", "RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS", "RUST_RESERVED_KEYWORDS", "indexedLineContent", "indexedLineStarts", "rustReferenceGraphs", "rustUnprovenReferenceTargets", "rustReferenceContextFiles"]);
const extracted = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? functions.has(node.name?.text ?? "") : ts.isClassDeclaration(node) ? node.name?.text === "TaxonomyCancellationError" : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
const compilers = [
  { name: "Bun", compile: (text: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(text) },
  { name: "TypeScript", compile: (text: string) => ts.transpileModule(text, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];
const runParent = join(ticket, ...vector.retention.parentSegments);

/** 🛡️ Allocates a fresh ticket-owned run after validating every ancestor without following links. */
function newRun(name: string): string {
  let current = parse(runParent).root;
  for (const segment of relative(current, runParent).split(sep)) {
    current = join(current, segment);
    let stat;
    try { stat = lstatSync(current); }
    catch (error) {
      const local = relative(ticket, current);
      if ((error as NodeJS.ErrnoException).code !== "ENOENT" || !local || local.startsWith("..") || isAbsolute(local)) throw error;
      mkdirSync(current);
      stat = lstatSync(current);
    }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Unsafe finite-consumer run parent: " + current);
  }
  const directory = mkdtempSync(join(runParent, vector.retention.runPrefix));
  writeFileSync(join(directory, "📝️.md"), "# Finite Target Consumer Run\n\nCase: " + name + ".\n\nNew isolated inputs; no production inventory, generators, Git mutation, source moves, or cleanup.\n\nThe enclosing gate records the assertion result. These inputs are not reconstructed historical evidence.\n", { flag: "wx" });
  return directory;
}

/** 🧫️ Materializes only an explicit language-neutral source/Cargo/target graph. */
function fixture(row: Row) {
  const directory = newRun(row.id), known = new Set<string>(), prefix = row.condition === "nested-coordinate" ? "nested/" : "", pkg = prefix + "pkg";
  const consumer = pkg + "/reader.rs", manifest = pkg + "/Cargo.toml", entry = pkg + "/entry.rs";
  const put = (path: string, content: string): void => {
    mkdirSync(dirname(join(directory, path)), { recursive: true });
    writeFileSync(join(directory, path), content);
    for (let current = path; current && current !== "."; current = posix.dirname(current)) known.add(current);
  };
  const manifestBytes = '[package]\nname = "finite_consumer"\nversion = "0.0.0"\nedition = "2021"\n[workspace]\n[lib]\npath = "entry.rs"\n';
  if (row.condition !== "no-owner") put(manifest, manifestBytes);
  if (row.condition !== "missing-chain") put(entry, '#[path = "reader.rs"] mod reader;\n');
  else known.add(entry);
  put(consumer, row.source);
  for (const path of row.affected) put(path, "affected sibling\n");
  for (const path of row.targets) {
    if (row.condition === "symlink-leaf" || row.condition === "symlink-ancestor") continue;
    put(path, "physical target\n");
  }
  if (row.condition === "two-owners") put("second/Cargo.toml", '[package]\nname = "second"\nversion = "0.0.0"\n[workspace]\n[lib]\npath = "../pkg/entry.rs"\n');
  if (row.condition === "symlink-leaf") {
    put("actual/item.json", "target behind link\n");
    mkdirSync(join(directory, "foreign"), { recursive: true });
    symlinkSync("../actual/item.json", join(directory, "foreign/item.json"));
    known.add("foreign"); known.add("foreign/item.json");
  }
  if (row.condition === "symlink-ancestor") {
    put("actual/item.json", "target behind ancestor\n");
    symlinkSync("actual", join(directory, "foreign"), process.platform === "win32" ? "junction" : "dir");
    known.add("foreign"); known.add("foreign/item.json");
  }
  if (row.condition === "unadmitted") known.delete("foreign/item.json");
  if (row.id === "missing-target") { known.add("absent"); known.add("absent/item.json"); }
  if (["cancelled-symlink", "cancelled-module-edge", "cancelled-manifest-edge"].includes(row.condition)) {
    put("elsewhere/deep/placeholder", "directory owner\n");
    put("elsewhere/foreign/item.json", "actual joined target\n");
    symlinkSync("elsewhere/deep", join(directory, "alias"), process.platform === "win32" ? "junction" : "dir");
    known.add("alias");
  }
  if (row.condition === "cancelled-module-edge") {
    put(entry, '#[path = "../alias/../pkg/reader.rs"] mod reader;\npub fn origin() -> &\'static str { reader::origin() }\n');
    put("elsewhere/pkg/reader.rs", 'pub fn origin() -> &\'static str { "actual physical module" }\n');
  }
  if (row.condition === "cancelled-manifest-edge") {
    put(manifest, manifestBytes.replace('path = "entry.rs"', 'path = "../alias/../pkg/entry.rs"'));
    put("elsewhere/pkg/entry.rs", "pub const ACTUAL_CRATE: bool = true;\n");
  }
  if (row.condition === "cancelled-file") put("not-directory", "not a directory\n");
  if (row.condition === "parent-env-macro") {
    put("shadow/deep/placeholder", "macro root\n");
    put("shadow/foreign/item.json", "actual macro target\n");
    put(entry, 'macro_rules! env { ("CARGO_MANIFEST_DIR") => { ' + JSON.stringify(join(directory, "shadow/deep")) + ' }; }\n#[path = "reader.rs"] mod reader;\npub fn run() { reader::read(); }\n');
  }
  if (row.condition === "parent-doc-comment") put(entry, '//! 🧭️ Crate root docs mention `!important` and a `#heading` in prose — neither is code.\n//!\n#[path = "reader.rs"] mod reader;\n');
  if (row.condition === "parent-known-macro") put(entry, '#[path = "reader.rs"] mod reader;\nsemio_framework_plugin::plugin_exports!(plugin::plugin, plugin::TestApps);\n');
  if (row.condition === "parent-cfg-test-mod") {
    put(entry, '#[path = "reader.rs"] mod reader;\n#[cfg(test)]\n#[path = "tests_x.rs"]\nmod tests_x;\n');
    put("pkg/tests_x.rs", 'mod tests { #[test] fn it_renames() {} }\n');
    known.add("pkg/tests_x.rs");
  }
  if (row.condition === "parent-glob-reexport") put(entry, '#[path = "reader.rs"] mod reader;\npub use reader::*;\n');
  if (row.condition === "parent-known-attribute-path") put(entry, '#[path = "reader.rs"] mod reader;\n#[semio_framework_async_macros::async_test]\nasync fn placeholder_test() {}\n');
  if (row.condition === "parent-crate-local-macro-and-std-expression-macros") put(entry, '#[path = "reader.rs"] mod reader;\nmacro_rules! impl_serde_op_codec { ($t:ty) => { impl $t {} }; }\nimpl_serde_op_codec!(Placeholder);\npub fn run() -> String { if true { format!("ok") } else { unreachable!() } }\n');
  if (row.condition === "opaque") known.add(row.source.includes("../temp/compose") ? "temp/compose/item.json" : "compose/item.json");
  const coordinateRoots = row.condition === "nested-coordinate" ? ["nested"] : row.condition === "foreign-coordinate" ? ["foreign"] : [];
  return { directory, known, consumer, manifest, entry, coordinateRoots, put };
}

/** 🔬️ Runs the actual private token pipeline with independent compilers and observable physical reads. */
function implementation(compiler: typeof compilers[number], directory: string) {
  const accesses: string[] = [];
  const observe = (path: string): void => {
    const local = relative(directory, String(path)).split(sep).join("/");
    accesses.push(local);
    if (["compose", "temp/compose"].some((opaque) => local === opaque || local.startsWith(opaque + "/"))) throw new Error("Opaque filesystem access: " + local);
    if (local === ".." || local.startsWith("../") || isAbsolute(local)) throw new Error("Foreign fixture filesystem access: " + local);
  };
  const dependencies = { createHash, posix, basename, dirname, join, resolve, relative, isAbsolute, sep,
    lstatSync: (path: string) => { observe(path); return lstatSync(path); },
    readFileSync: (...args: Parameters<typeof readFileSync>) => { observe(String(args[0])); return (readFileSync as any)(...args); },
    existsSync: (path: string) => { observe(path); return existsSync(path); },
    inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, inspectRustNonRepoJoinBaseSpans, rustSyntaxTokens, rustTokenPairs };
  const actual = new Function(...Object.keys(dependencies), compiler.compile(extracted) + "\nreturn { index: referencePathIndex, graph: rustReferenceGraph, tokens: rustManifestReferenceTokens, all: referenceTokensIncludingUnsupported, unsupported: unsupportedReferenceTokens, resolve: resolveReferenceTokenPath, finite: typeof rustFiniteManifestTargets === 'undefined' ? undefined : rustFiniteManifestTargets, covers: typeof rustReferenceInterpretationCovers === 'undefined' ? undefined : rustReferenceInterpretationCovers };")(...Object.values(dependencies));
  return { ...actual, accesses };
}

/** 🧭️ Resolves explicit Cargo roots through an independent TOML parser and cross-platform path implementation. */
function cargoOracle(f: ReturnType<typeof fixture>) {
  return [...f.known].filter((path) => path.endsWith("/Cargo.toml")).map((path) => {
    const document = parseToml(readFileSync(join(f.directory, path), "utf8")) as any;
    const entry = oracleNormalize(oracleJoin(dirname(path), document.lib?.path ?? "src/lib.rs"));
    const entrySource = existsSync(join(f.directory, entry)) ? readFileSync(join(f.directory, entry), "utf8") : "";
    const child = entrySource.match(/#\[path = "([^"]+)"\] mod reader;/u)?.[1];
    return { manifest: path, entry, consumer: child ? oracleNormalize(oracleJoin(dirname(entry), child)) : null };
  }).filter((row) => row.consumer === f.consumer);
}

/** 🧾️ Locates only the exact authored leaf literal, independently of production token offsets. */
function leafSpan(content: string, value = "item.json") {
  const start = content.indexOf('"' + value + '"') + 1;
  if (start <= 0 || content.indexOf('"' + value + '"', start + value.length + 1) >= 0) throw new Error("Expected one authored leaf");
  return { start, end: start + value.length, value };
}

test("new finite interpretation declarations satisfy strict TypeScript without ambient any callbacks", () => {
  const declarations = new Set(["ReferenceToken", "ReferencePathIndex", "RustReferenceGraphView", "rustReferenceNeedsOwnership", "rustFiniteManifestTargets", "rustManifestReferenceTokens", "rustReferenceInterpretationCovers"]);
  const body = syntax.statements.filter((node) => (ts.isFunctionDeclaration(node) || ts.isInterfaceDeclaration(node)) && declarations.has(node.name?.text ?? "")).map((node) => node.getText(syntax)).join("\n");
  expect(body.includes("function rustFiniteManifestTargets")).toBe(true);
  const contracts = [
    'type TaxonomyReferenceAdapter = string;',
    'interface Reference { readonly start: number; readonly end: number; readonly value: string; readonly base: readonly string[] }',
    'interface Candidate { readonly start: number; readonly end: number; readonly value: string; readonly targets: readonly (readonly string[])[] }',
    'interface RustModuleContext { readonly crateRoot: string; readonly manifestPath: string | null; readonly modulePath: readonly string[]; readonly sourceScope: readonly string[]; readonly moduleBase: string; readonly sourceChain: readonly string[] }',
    'interface RustModuleGraph { readonly targets: ReadonlyMap<string, string>; readonly contexts: ReadonlyMap<string, readonly RustModuleContext[]> }',
    'interface ModuleFact { readonly name: string; readonly modulePath: readonly string[]; readonly inline: boolean; readonly pathTarget: string | null }',
    'interface Stat { readonly mode: number; readonly size: number; readonly mtimeMs: number; isFile(): boolean; isDirectory(): boolean; }',
    'interface Bytes { readonly byteLength: number; toString(encoding: "utf8"): string }',
    'declare const posix: { dirname(path: string): string; join(...parts: string[]): string; isAbsolute(path: string): boolean };',
    'declare function inspectRustManifestPathReferences(source: string): readonly Reference[];',
    'declare function inspectRustManifestPathCandidates(source: string): readonly Candidate[];',
    'declare function inspectRustJoinArgumentSpans(source: string): readonly Pick<Reference, "start" | "end" | "value">[];',
    'declare function inspectRustNonRepoJoinBaseSpans(source: string): ReadonlySet<number>;',
    'declare function inspectRustCargoManifest(source: string, strict: boolean): { readonly valid: boolean; readonly libPath: string | null; readonly dependencies: readonly string[] };',
    'declare function inspectRustModuleGraphFacts(source: string): { readonly modules: readonly ModuleFact[]; readonly uses: readonly { readonly specifier: string }[] };',
    'declare function rustCodeOnlyTextForMacroTrust(source: string): string;',
    'declare function sha256(source: string | Bytes): string;',
    'declare function canonicalJson(value: unknown): string;',
    'declare function checkCancellation(root: string, path?: string): void;',
    'declare function normalizeRelative(path: string): string;',
    'declare function ancestorReferenceCoordinateRoot(path: string, roots: ReadonlySet<string>): string | undefined;',
    'declare function assertLexicalInputOutsideOpaque(root: string, path: string, label: string, leaf: boolean): string;',
    'declare function lstatOrNull(path: string): Stat | null;',
    'declare function lstatSync(path: string): Stat;',
    'declare function readFileSync(path: string): Bytes;',
    'declare function generatorPathCompare(left: string, right: string): number;',
    'declare function dirname(path: string): string;',
    'declare function basename(path: string): string;',
    'declare function lineLocation(source: string, start: number, label: string): string;',
    'declare function rustContextFiles(path: string, index: ReferencePathIndex): readonly string[];',
    'declare function unprovenRustReferenceTargets(path: string, value: string, index: ReferencePathIndex): readonly string[];',
    'declare function rustReferenceGraph(path: string, index: ReferencePathIndex): RustReferenceGraphView | null;',
    'declare class TaxonomyCancellationError extends Error {};',
  ].join("\n");
  const file = join(import.meta.dir, "🧾️strict/🟦️.ts"), input = contracts + "\n" + body, options = { strict: true, noEmit: true, skipLibCheck: true, target: ts.ScriptTarget.ES2022, types: [] as string[] };
  const host = ts.createCompilerHost(options), read = host.readFile.bind(host), exists = host.fileExists.bind(host), get = host.getSourceFile.bind(host);
  host.readFile = (path) => path === file ? input : read(path);
  host.fileExists = (path) => path === file || exists(path);
  host.getSourceFile = (path, language, onError, fresh) => path === file ? ts.createSourceFile(path, input, language, true) : get(path, language, onError, fresh);
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([file], options, host));
  expect(diagnostics.map((item) => ts.flattenDiagnosticMessageText(item.messageText, "\n"))).toEqual([]);
});


test("exact finite consumer route and launch registration preserve the canonical semantic leaf", () => {
  const registration = vector.registration, project = JSON.parse(readFileSync(join(root, registration.projectPath), "utf8"));
  expect(project.targets[registration.target]).toEqual({ executor: "nx:run-commands", options: { cwd: dirname(registration.projectPath), command: registration.command } });
  const routerText = readFileSync(join(root, registration.routerPath), "utf8"), router = ts.createSourceFile(registration.routerPath, routerText, ts.ScriptTarget.Latest, true);
  const branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node) => { if (ts.isIfStatement(node) && node.expression.getText(router) === 'segments[0] === "' + registration.route + '"') branches.push(node); ts.forEachChild(node, visit); };
  visit(router);
  expect(branches).toHaveLength(1);
  expect(branches[0]!.thenStatement.getText(router)).toContain(JSON.stringify(registration.testPath));
  expect(branches[0]!.thenStatement.getText(router)).toContain('runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot })');
  const launch = parseJsonc(readFileSync(join(root, ".vscode/launch.json"), "utf8")).configurations.filter((item: any) => item.name === registration.launchName);
  expect(launch).toHaveLength(1);
  expect(launch[0]).toEqual({ name: registration.launchName, type: "node-terminal", request: "launch", command: registration.launchCommand, cwd: "$" + "{workspaceFolder}", presentation: { group: "4_gate", order: registration.launchOrder } });
});


test("language-neutral finite consumer contract is closed and retains all physical proof obligations", () => {
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const changed of [{ ...vector, unknown: true }, { ...vector, semantics: { ...vector.semantics, failure: "empty-is-disjoint" } }, { ...vector, cases: [] }]) expect(validate(changed)).toBe(false);
  expect(new Set(vector.cases.map((row: Row) => row.id)).size).toBe(vector.cases.length);
});

for (const compiler of compilers) for (const row of vector.cases as Row[]) test(compiler.name + " physical finite targets: " + row.id, () => {
  const f = fixture(row), actual = implementation(compiler, f.directory);
  const index = actual.index(f.known, f.directory, f.coordinateRoots, f.known, undefined, new Set(row.affected));
  if (row.condition.startsWith("changed-")) {
    actual.graph(f.consumer, index);
    const changed = row.condition === "changed-consumer" ? f.consumer : row.condition === "changed-chain" ? f.entry : f.manifest;
    f.put(changed, readFileSync(join(f.directory, changed), "utf8") + "\n" + (changed.endsWith(".rs") ? "pub const SNAPSHOT_CHANGE: u8 = 1;\n" : 'description = "changed snapshot"\n'));
  }
  const span = leafSpan(row.source), tokens = actual.tokens(f.consumer, row.source, index) as Token[], same = tokens.filter((token) => token.start === span.start && token.end === span.end);
  if (row.expected === "finite") {
    expect(cargoOracle(f)).toHaveLength(1);
    const candidate = inspectRustManifestPathCandidates(row.source).find((item) => item.start === span.start && item.end === span.end)!;
    const expected = [...new Set(candidate.targets.map((parts) => oracleNormalize(oracleJoin(dirname(f.manifest), ...parts))))].sort();
    expect(expected).toEqual([...row.targets].sort());
    expect(same).toHaveLength(1);
    expect(same[0]!.physicalInterpretation).toBe(marker);
    expect(same[0]!.physicalTargets).toEqual(expected);
    expect(same[0]!.rewriteKind).toBeUndefined();
    expect(same[0]!.unsupportedReason).toBeTruthy();
    const all = actual.all(f.consumer, row.source, index) as Token[];
    expect(all.filter((token) => token.start === span.start && token.end === span.end)).toHaveLength(1);
    expect(same[0]!.physicalTargets!.filter((path) => row.affected.includes(path))).toEqual(expected.filter((path) => row.affected.includes(path)));
    if (row.id === "foreign-disjoint") expect(all.some((token) => actual.resolve(f.consumer, token, index) === "pkg/item.json")).toBe(false);
  } else {
    expect(same.some((token) => token.physicalInterpretation === marker)).toBe(false);
    const all = actual.all(f.consumer, row.source, index) as Token[];
    expect(all.some((token) => token.start === span.start && token.end === span.end && (token.physicalTargets?.some((path) => row.affected.includes(path)) || row.affected.includes(actual.resolve(f.consumer, token, index))))).toBe(true);
  }
  expect(actual.accesses.some((path: string) => ["compose", "temp/compose"].some((opaque) => path === opaque || path.startsWith(opaque + "/")))).toBe(false);
});

for (const compiler of compilers) test(compiler.name + " terminal exact-target index keeps the complete foreign physical interpretation", () => {
  const row = vector.cases[0] as Row, f = fixture(row), actual = implementation(compiler, f.directory), span = leafSpan(row.source);
  const index = actual.index(row.affected, f.directory, [], f.known);
  const tokens = actual.all(f.consumer, row.source, index) as Token[];
  const same = tokens.filter((token) => token.start === span.start && token.end === span.end);
  expect(same).toHaveLength(1);
  expect(same[0]!.physicalTargets).toEqual(["foreign/item.json"]);
  expect(same[0]!.physicalInterpretation).toBe(marker);
  expect(tokens.some((token) => row.affected.includes(actual.resolve(f.consumer, token, index)))).toBe(false);
});

for (const compiler of compilers) test(compiler.name + " tuple row correlation does not manufacture cross-paired physical targets", () => {
  const row = { ...vector.cases[0], id: "tuple-correlation", source: vector.correlated.source, targets: vector.correlated.targets.map((item: any) => item.target), affected: ["affected/beta.json", "pkg/alpha.json", "pkg/beta.json"] };
  const f = fixture(row), actual = implementation(compiler, f.directory), index = actual.index(f.known, f.directory, [], f.known, undefined, new Set(row.affected));
  const tokens = actual.all(f.consumer, row.source, index) as Token[];
  for (const item of vector.correlated.targets) {
    const span = leafSpan(row.source, item.value), same = tokens.filter((token) => token.start === span.start && token.end === span.end);
    expect(same).toHaveLength(1);
    expect(same[0]!.physicalInterpretation).toBe(marker);
    expect(same[0]!.physicalTargets).toEqual([item.target]);
    expect(same[0]!.rewriteKind).toBeUndefined();
  }
});

for (const compiler of compilers) test(compiler.name + " writable proof keeps precedence while finite suppression requires exact span identity", () => {
  const row = { ...vector.cases[0], id: "writable-precedence", source: vector.writable.source }, f = fixture(row), actual = implementation(compiler, f.directory), index = actual.index(f.known, f.directory);
  const span = leafSpan(row.source), rows = (actual.all(f.consumer, row.source, index) as Token[]).filter((token) => token.start === span.start && token.end === span.end);
  expect(rows).toHaveLength(1);
  expect(rows[0]!.rewriteKind).toBe("rust-path-join");
  expect(rows[0]!.physicalInterpretation).toBeUndefined();
  expect(rows[0]!.physicalTargets).toEqual([vector.writable.target]);
  expect(actual.covers).toBeFunction();
  const interpreted = { ...span, adapter: "rust", physicalTargets: ["foreign/item.json"], physicalInterpretation: marker, unsupportedReason: "candidate-only" };
  expect(actual.covers(interpreted, { ...span, adapter: "rust" })).toBe(true);
  for (const changed of [{ ...span, start: span.start + 1 }, { ...span, end: span.end - 1 }, { ...span, value: "other.json" }, { ...span, start: span.start + 100, end: span.end + 100 }]) expect(actual.covers(interpreted, { ...changed, adapter: "rust" })).toBe(false);
  expect(actual.covers({ ...interpreted, physicalInterpretation: undefined }, { ...span, adapter: "rust" })).toBe(false);
});

for (const compiler of compilers) test(compiler.name + " changed source bytes and cancellation cannot obtain finite authority", () => {
  const row = vector.cases[0] as Row, f = fixture(row), actual = implementation(compiler, f.directory), index = actual.index(f.known, f.directory);
  expect(() => actual.tokens(f.consumer, row.source + "\n", index)).toThrow("source changed");
  f.put("cancel", "cancel\n");
  expect(() => actual.tokens(f.consumer, row.source, actual.index(f.known, f.directory, [], f.known, "cancel"))).toThrow("cancelled");
});

for (const compiler of compilers) test(compiler.name + " neighboring equal-value literals remain conservative and incomplete candidate facts never mean disjoint", () => {
  const initial = vector.cases[0] as Row, row = { ...initial, id: "neighbor-span", source: initial.source + '\nconst UNRELATED: &str = "item.json";\n' };
  const f = fixture(row), actual = implementation(compiler, f.directory), index = actual.index(f.known, f.directory, [], f.known, undefined, new Set(row.affected)), graph = actual.graph(f.consumer, index);
  const first = leafSpan(initial.source), neighbor = row.source.lastIndexOf('"item.json"') + 1;
  const tokens = actual.all(f.consumer, row.source, index) as Token[];
  expect(tokens.filter((token) => token.start === first.start && token.physicalInterpretation === marker)).toHaveLength(1);
  expect(tokens.some((token) => token.start === neighbor && actual.resolve(f.consumer, token, index) === "pkg/item.json")).toBe(true);
  const candidate = inspectRustManifestPathCandidates(row.source).find((item) => item.start === first.start)!;
  for (const changed of [{ ...candidate, start: candidate.start + 1 }, { ...candidate, end: candidate.end - 1 }, { ...candidate, value: "other.json" }, { ...candidate, targets: [] }, { ...candidate, targets: Array.from({ length: 257 }, () => candidate.targets[0]) }]) expect(actual.finite(f.consumer, row.source, [changed], index, graph).size).toBe(0);
});

test("actual rustc proves cancelled symlink steps target different bytes from normalized lexical paths", () => {
  const row = vector.cases.find((item: Row) => item.id === "cancelled-symlink-ancestor") as Row, f = fixture(row);
  const nativeSource = 'fn main() { let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")); let actual = root.join("../alias/../foreign").join("item.json"); println!("{}", std::fs::read_to_string(actual).unwrap().trim()); }\n';
  f.put("🧾️native/🦀️.rs", nativeSource);
  const binary = join(f.directory, "🧾️native", process.platform === "win32" ? "🔣️.exe" : "🔣️");
  const compile = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "finite_path_identity", join(f.directory, "🧾️native/🦀️.rs"), "-o", binary], { cwd: f.directory, env: { ...process.env, CARGO_MANIFEST_DIR: join(f.directory, "pkg") }, stdout: "pipe", stderr: "pipe" });
  expect(compile.exitCode, compile.stderr.toString()).toBe(0);
  const execution = Bun.spawnSync([binary], { cwd: f.directory, stdout: "pipe", stderr: "pipe" });
  expect(execution.exitCode, execution.stderr.toString()).toBe(0);
  expect(execution.stdout.toString()).toBe("actual joined target\n");
  expect(readFileSync(join(f.directory, "foreign/item.json"), "utf8")).toBe("physical target\n");
  writeFileSync(join(f.directory, "🧾️native/📝️.md"), "# Native Physical Identity Oracle\n\nrustc compiled the exact new isolated input. Runtime stdout was actual joined target; the normalized lexical target contained physical target. The cancelled symlink segment therefore changes physical identity and must not receive finite authority.\n");
});

test("actual rustc resolves raw module ownership paths before lexical cancellation", () => {
  const row = vector.cases.find((item: Row) => item.id === "cancelled-module-ownership-edge") as Row, f = fixture(row);
  f.put("🧾️native/🦀️.rs", '#[path = "../pkg/entry.rs"] mod owner;\nfn main() { println!("{}", owner::origin()); }\n');
  const binary = join(f.directory, "🧾️native", process.platform === "win32" ? "🔣️.exe" : "🔣️");
  const compile = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "finite_source_identity", join(f.directory, "🧾️native/🦀️.rs"), "-o", binary], { cwd: f.directory, env: { ...process.env, CARGO_MANIFEST_DIR: join(f.directory, "pkg") }, stdout: "pipe", stderr: "pipe" });
  expect(compile.exitCode, compile.stderr.toString()).toBe(0);
  const execution = Bun.spawnSync([binary], { cwd: f.directory, stdout: "pipe", stderr: "pipe" });
  expect(execution.exitCode, execution.stderr.toString()).toBe(0);
  expect(execution.stdout.toString()).toBe("actual physical module\n");
  expect(readFileSync(join(f.directory, f.consumer), "utf8")).toBe(row.source);
});


test("actual rustc proves inherited env macro provenance is part of physical source authority", () => {
  const row = vector.cases.find((item: Row) => item.id === "inherited-env-macro") as Row, f = fixture(row);
  f.put("🧾️native/🦀️.rs", '#[path = "../pkg/entry.rs"] mod owner;\nfn main() { owner::run(); }\n');
  const binary = join(f.directory, "🧾️native", process.platform === "win32" ? "🔣️.exe" : "🔣️");
  const compile = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "finite_macro_identity", join(f.directory, "🧾️native/🦀️.rs"), "-o", binary], { cwd: f.directory, env: { ...process.env, CARGO_MANIFEST_DIR: join(f.directory, "pkg") }, stdout: "pipe", stderr: "pipe" });
  expect(compile.exitCode, compile.stderr.toString()).toBe(0);
  const execution = Bun.spawnSync([binary], { cwd: f.directory, stdout: "pipe", stderr: "pipe" });
  expect(execution.exitCode, execution.stderr.toString()).toBe(0);
  expect(execution.stdout.toString()).toBe("TARGET:actual macro target\nitem.json\n");
  expect(readFileSync(join(f.directory, "foreign/item.json"), "utf8")).toBe("physical target\n");
});


test("actual Cargo metadata independently agrees with explicit lib-root and module ownership inputs", () => {
  const f = fixture({ ...vector.cases[0], id: "cargo-metadata-oracle" });
  const result = Bun.spawnSync(["cargo", "metadata", "--offline", "--no-deps", "--format-version", "1", "--manifest-path", join(f.directory, f.manifest)], { cwd: f.directory, env: { ...process.env, CARGO_TARGET_DIR: join(f.directory, "🧾️cargo-target") }, stdout: "pipe", stderr: "pipe" });
  expect(result.exitCode, result.stderr.toString()).toBe(0);
  const metadata = JSON.parse(result.stdout.toString()), library = metadata.packages[0].targets.find((target: any) => target.kind.includes("lib"));
  expect(metadata.packages).toHaveLength(1);
  expect(relative(f.directory, library.src_path).split(sep).join("/")).toBe(f.entry);
  expect(cargoOracle(f)).toEqual([{ manifest: f.manifest, entry: f.entry, consumer: f.consumer }]);
});


test("incoming, planning unsupported pass, and terminal verification share one exact-span suppression predicate", () => {
  const declaration = (name: string) => syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === name) as ts.FunctionDeclaration | undefined;
  const calls = (name: string, target: string) => {
    const owner = declaration(name); if (!owner) throw new Error("Missing actual caller: " + name);
    const found: ts.CallExpression[] = [];
    const visit = (node: ts.Node) => { if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === target) found.push(node); ts.forEachChild(node, visit); };
    visit(owner); return found;
  };
  expect(calls("referenceTokensIncludingUnsupported", "rustReferenceInterpretationCovers")).toHaveLength(1);
  expect(calls("buildReferenceEdits", "rustReferenceInterpretationCovers")).toHaveLength(1);
  expect(calls("incomingReferenceSnapshot", "referenceTokensIncludingUnsupported")).toHaveLength(1);
  expect(calls("lexicalTargetIncomingReferences", "referenceTokensIncludingUnsupported")).toHaveLength(1);
  const planner = declaration("buildReferenceEdits")!.getText(syntax);
  expect(planner).toContain('if (token.unsupportedReason && token.physicalTargets !== undefined');
  expect(planner).toContain('token.physicalTargets.some((target) => destinationBySource.has(target))');
  expect(planner).toContain('unresolved.push(violation("reference-syntax-unsupported"');
  expect(source).toBe(readFileSync(sourcePath, "utf8"));
});
