import { expect, test } from "bun:test";
import { createHash, randomUUID } from "node:crypto";
import { execFileSync, spawnSync } from "node:child_process";
import { lstatSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, parse, posix, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc, visit as jsoncVisit } from "jsonc-parser";
import stringify from "fast-json-stable-stringify";
import { join as oracleJoin } from "pathe";
import ts from "typescript";
import * as discovery from "../../🔍️discovery/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../");
const path = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
const source = readFileSync(path, "utf8"), syntax = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧪️🏔️preflight-reference-basis/🔣️.json"), "utf8"));
const compilers = [
  { name: "Bun", compile: (value: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(value) },
  { name: "TypeScript", compile: (value: string) => ts.transpileModule(value, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];
const compiledFragments = new Map<string, string>();
const compile = (compiler: typeof compilers[number], code: string) => {
  const key = compiler.name + "\0" + code;
  let compiled = compiledFragments.get(key);
  if (compiled === undefined) { compiled = compiler.compile(code); compiledFragments.set(key, compiled); }
  return compiled;
};
type Node = { kind: "file" | "directory" | "symlink"; content?: string; raw?: Uint8Array; target?: string; mode: number; ino: number; gitRoot?: boolean };
type Query = { id: string; targets: string[]; ignored: string[]; expected: string[] };
type Basis = { candidates: string[]; coordinateRoots: string[]; observed: Map<string, unknown> };
type Production = {
  capturePreflightReferenceBasis: (...args: any[]) => Basis;
  validatePreflightReferenceBasis: (...args: any[]) => void;
  lexicalTargetIncomingReferences: (...args: any[]) => string[];
};

/** 🧬️ Extracts the actual private production declarations without importing or dispatching workspace code. */
function declarations(names: string[]): string {
  return names.map((name) => {
    const rows = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? node.name?.text === name : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => declaration.name.getText(syntax) === name));
    if (rows.length !== 1) throw new Error("Missing exact production declaration: " + name);
    return rows[0]!.getText(syntax).replace(/^export /u, "");
  }).join("\n");
}

/** 🧫️ Models explicit filesystem/Git reads and forbids all opaque access before any node observation. */
function environment(compiler: typeof compilers[number]) {
  const repoRoot = resolve("/virtual-preflight"), nodes = new Map<string, Node>(), tracked = new Set<string>(), untracked = new Set<string>(), ticketPaths = new Set<string>();
  let sequence = 1, cancelled = false, repositoryCensuses = 0, planCensuses = 0, parsed = 0;
  const accesses: string[] = [], reads: string[] = [], projections: string[] = [], frozen: string[] = [], progressRows: unknown[] = [];
  const normalize = (value: string) => isAbsolute(value) ? relative(repoRoot, value).replaceAll("\\", "/") : value;
  const opaque = (value: string) => vector.semantics.opaqueRoots.some((part: string) => value === part || value.startsWith(part + "/"));
  const access = (value: string) => { const name = normalize(value); if (opaque(name)) throw new Error("Forbidden opaque access: " + name); accesses.push(name); return name; };
  const put = (name: string, content: string, admit = true) => {
    for (let parent = posix.dirname(name); parent !== "."; parent = posix.dirname(parent)) if (!nodes.has(parent)) nodes.set(parent, { kind: "directory", mode: 0o755, ino: sequence++ });
    nodes.set(name, { kind: "file", content, mode: 0o644, ino: sequence++ });
    if (admit) tracked.add(name);
  };
  for (const [name, content] of Object.entries(vector.fixture.files)) put(name, content as string);
  nodes.set(".git", { kind: "directory", mode: 0o755, ino: sequence++, gitRoot: true });
  nodes.set("nested/.git", { kind: "directory", mode: 0o755, ino: sequence++, gitRoot: true });
  tracked.add("compose/opaque.json");
  untracked.add("temp/compose/opaque.json");
  const stat = (value: string) => {
    const name = access(value), node = nodes.get(name);
    if (!node) throw Object.assign(new Error("absent: " + name), { code: "ENOENT" });
    return { mode: node.mode, size: node.raw?.byteLength ?? Buffer.byteLength(node.content ?? node.target ?? ""), dev: 1, ino: node.ino, isFile: () => node.kind === "file", isDirectory: () => node.kind === "directory", isSymbolicLink: () => node.kind === "symlink" };
  };
  const read = (value: string, encoding?: string) => {
    const name = access(value), node = nodes.get(name);
    if (!node || node.kind !== "file") throw new Error("Non-file read: " + name);
    reads.push(name);
    const bytes = node.raw ? Buffer.from(node.raw) : Buffer.from(node.content!);
    return encoding ? bytes.toString("utf8") : bytes;
  };
  const dependencies: Record<string, unknown> = {
    Buffer, posix, resolve, relative, sep, join, dirname, basename, isAbsolute, createHash,
    lstatSync: stat, readFileSync: read,
    readlinkSync: (value: string) => { const name = access(value), node = nodes.get(name); if (node?.kind !== "symlink") throw new Error("Non-link read: " + name); return node.target!; },
    execFileSync: (_command: string, args: string[]) => { expect(args).toEqual(["rev-parse", "--absolute-git-dir"]); return repoRoot + "/.git\n"; },
    spawnSync: (_command: string, args: string[], options: { cwd: string }) => { expect(args).toEqual(["rev-parse", "--show-toplevel"]); const name = normalize(options.cwd), marker = nodes.get(name + "/.git"); return { status: marker?.gitRoot ? 0 : 128, stdout: marker?.gitRoot ? options.cwd + "\n" : "" }; },
    checkCancellation: () => { if (cancelled) throw new Error("Taxonomy operation cancelled"); },
    gitRows: () => { repositoryCensuses++; return [...tracked].map((name) => ({ path: name })); },
    untrackedGitPaths: () => [...untracked],
    explicitTicketRows: () => [...ticketPaths].map((name) => ({ path: name })),
    planVerificationCandidatePaths: () => { planCensuses++; return [...ticketPaths]; },
    validateObservedFrozenEvidenceNodes: () => {},
    frozenEvidenceCoordinateAuthority: (name: string) => { frozen.push(name); },
    isFrozenSourceCoordinateToken: (name: string) => vector.fixture.frozenPaths.includes(name),
    repositoryLocalSymlinkTargetPath: (_root: string, target: string) => normalize(target),
    report: (...args: unknown[]) => { progressRows.push(args); const callback = args[0]; if (typeof callback === "function") callback({ operation: args[1], phase: args[2], completed: args[3], total: args[4], path: args[5] }); },
    readdirSync: (value: string) => { const name = access(value); return [...nodes.keys()].filter((key) => posix.dirname(key) === name).map((key) => posix.basename(key)); },
  };
  const parserSupport = declarations(["normalizeRelative", "sourceRelative", "emojiFold", "graphemes", "isEmojiGrapheme", "splitLeadingEmoji", "lineLocation", "embeddedArgumentTokens", "artifactRootForPath", "mutationStructuralPaths", "canonicalProjectionSuffix", "projectionKey", "projectedStructuralValue", "structuralProjectionToken", "structuralTokensInFragment", "SEGMENTER", "indexedLineContent", "indexedLineStarts", "OLD_MUTATION_TEST_PREFIX_SOURCE", "OLD_MUTATION_STRUCTURE_SOURCE", "jsonTokens"]);
  const jsonTokens = new Function("posix", compile(compiler, parserSupport) + "\nreturn jsonTokens;")(posix);
  dependencies.referenceTokensIncludingUnsupported = (name: string, content: string) => { parsed++; return /\.json$/u.test(name) ? jsonTokens(name, content, "json") : []; };
  const code = declarations(["sourceRelative", "normalizeRelative", "assertNoFollowAncestors", "assertLexicalInputOutsideOpaque", "LEXICAL_OPAQUE_ROOTS", "isExcluded", "lstatOrNull", "generatorPathCompare", "sha256", "canonicalArrayKey", "canonicalValue", "canonicalJson", "absolutePath", "HISTORICAL_TICKET_ROOT_PATTERN", "HISTORICAL_PROMPT_LOG_ROOT_PATTERN", "packageRootManifestBasenames", "historicalEvidenceBoundaryOwns", "historicalDocumentEvidence", "repositoryReferenceCandidatePaths", "referenceCoordinateRoots", "ancestorReferenceCoordinateRoot", "incomingReferenceLexicalAdmission", "referenceCandidatesWithProgress", "textualPath", "splitTokenSuffix", "addUniqueIndex", "referencePathIndex", "resolveReferencePath", "resolveReferenceTokenPath", "preflightReferenceNodeWitness", "observePreflightReferenceNode", "capturePreflightReferenceBasis", "validatePreflightReferenceBasis", "lexicalTargetIncomingReferences"]);
  const api: Production = new Function(...Object.keys(dependencies), compile(compiler, code) + "\nreturn { capturePreflightReferenceBasis, validatePreflightReferenceBasis, lexicalTargetIncomingReferences };")(...Object.values(dependencies));
  const taxonomy = { exclusions: vector.semantics.opaqueRoots.map((name: string) => ({ path: name })), discoverySchema: { historicalDocumentEvidencePopulations: discovery.loadCatalogTaxonomy().historicalDocumentEvidencePopulations, fixedFilenameContracts: {} }, pathMatcher: discovery.createTaxonomyPathMatcher() }, plan = { id: "frozen-plan" };
  const capture = () => api.capturePreflightReferenceBasis(repoRoot, taxonomy, vector.fixture.ticketDir, vector.fixture.transactionRoot, plan);
  const query = (row: Query, basis?: Basis, project?: (name: string, bytes: Buffer, mode: number) => { path: string; bytes: Buffer }, progress?: (value: any) => void) => api.lexicalTargetIncomingReferences(repoRoot, new Set(row.targets), row.ignored, taxonomy, vector.fixture.ticketDir, { path: vector.fixture.planAuthorityPath, bytes: Buffer.from(vector.fixture.files[vector.fixture.planAuthorityPath]) }, vector.fixture.transactionRoot, plan, undefined, progress, project, basis);
  return { api, repoRoot, nodes, tracked, untracked, ticketPaths, put, capture, query, taxonomy, plan, accesses, reads, frozen, projections, progressRows, counters: () => ({ repositoryCensuses, planCensuses, parsed }), cancel: () => { cancelled = true; }, nextInode: () => sequence++ };
}

/** 🔬️ Derives exact JSON string references using an independent parser and path implementation. */
function oracle(row: Query): string[] {
  const result = new Set<string>();
  for (const [name, content] of Object.entries(vector.fixture.files)) {
    if (!name.endsWith(".json") || row.ignored.some((owner) => name === owner || name.startsWith(owner + "/")) || vector.fixture.frozenPaths.includes(name) || name === vector.fixture.planAuthorityPath) continue;
    jsoncVisit(content as string, { onLiteralValue: (value) => {
      if (typeof value !== "string") return;
      const candidates = /^\.\.?\//u.test(value) ? [oracleJoin(posix.dirname(name), value)] : [oracleJoin(name.startsWith("nested/") ? "nested" : "", value), oracleJoin(posix.dirname(name), value)];
      const target = candidates.find((candidate) => row.targets.includes(candidate));
      if (target) result.add("text\0" + name + "\0" + target);
    } });
  }
  return [...result].sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
}

test("neutral contract requires isolated queries and a fresh final authority witness", () => {
  expect(vector.schemaVersion).toBe(1);
  expect(vector.semantics.lifetime).toBe("one-fresh-apply-preflight");
  expect(vector.semantics.finalBoundary).toBe("after-canonical-attempt-publication-before-staging");
  const validate = new Ajv().compile(vector.caseSchema);
  for (const row of vector.queries) expect(validate(row), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...vector.queries[0], undeclared: true })).toBe(false);
  expect(validate({ ...vector.queries[0], targets: [] })).toBe(false);
  for (const row of vector.queries) expect(oracle(row)).toEqual(row.expected);
});

for (const compiler of compilers) test(compiler.name + " preserves independent ordered query results while reusing only the preflight basis", () => {
  const actual = environment(compiler), basis = actual.capture(), original = stringify(vector.fixture);
  for (const row of vector.queries) expect(actual.query(row, basis)).toEqual(oracle(row));
  expect(actual.counters().repositoryCensuses).toBe(1);
  expect(actual.counters().planCensuses).toBe(1);
  expect(basis.coordinateRoots).toEqual(vector.fixture.coordinateRoots);
  actual.api.validatePreflightReferenceBasis(basis);
  expect(actual.counters().repositoryCensuses).toBe(2);
  expect(actual.counters().planCensuses).toBe(2);
  expect(actual.reads).not.toContain("binary.bin");
  expect(actual.frozen).toContain("frozen.json");
  expect(stringify(vector.fixture)).toBe(original);
  const fresh = environment(compiler);
  for (const row of vector.queries) expect(fresh.query(row)).toEqual(actual.query(row, basis));
  expect(fresh.counters().repositoryCensuses).toBe(vector.queries.length);
});

for (const row of vector.mutations) test("fresh witness rejects only unowned authority drift: " + row.id, () => {
  for (const compiler of compilers) {
    const actual = environment(compiler);
    if (row.kind === "marker-file-replace") {
      actual.put("nested/.git", "gitdir: ../.git\n", false);
    }
    const basis = actual.capture();
    actual.query(vector.queries[0], basis);
    const node = actual.nodes.get(row.path);
    if (row.kind === "add") actual.put(row.path, row.content);
    if (row.kind === "remove") { actual.nodes.delete(row.path); actual.tracked.delete(row.path); }
    if (row.kind === "physical-remove") actual.nodes.delete(row.path);
    if (row.kind === "content") node!.content = row.content;
    if (row.kind === "mode") node!.mode = row.mode;
    if (row.kind === "symlink") actual.nodes.set(row.path, { kind: "symlink", mode: 0o777, ino: actual.nextInode(), target: row.target });
    if (row.kind === "marker") actual.put(row.path, row.content, false);
    if (row.kind === "marker-replace" || row.kind === "marker-file-replace") node!.ino = actual.nextInode();
    if (row.error) expect(() => actual.api.validatePreflightReferenceBasis(basis)).toThrow(row.error);
    else expect(() => actual.api.validatePreflightReferenceBasis(basis)).not.toThrow();
  }
});

test("late temporary content changes stay rejected even when restored before final validation", () => {
  for (const compiler of compilers) {
    const actual = environment(compiler), basis = actual.capture(), node = actual.nodes.get("consumer.json")!, original = node.content!;
    actual.query(vector.queries[0], basis);
    node.content = "[]\n";
    actual.query(vector.queries[1], basis);
    node.content = original;
    expect(() => actual.api.validatePreflightReferenceBasis(basis)).toThrow("node changed: consumer.json");
  }
});

test("raw marker bytes cannot drift behind identical UTF-8 replacement characters", () => {
  for (const compiler of compilers) for (const row of vector.markerBytesCases) {
    const actual = environment(compiler), initial = Buffer.from(row.initial), changed = Buffer.from(row.changed);
    expect(initial.toString("utf8") === changed.toString("utf8")).toBe(row.decodedEqual);
    expect(createHash("sha256").update(initial).digest("hex")).not.toBe(createHash("sha256").update(changed).digest("hex"));
    actual.put("pkg/.git", "", false);
    actual.nodes.get("pkg/.git")!.raw = initial;
    const basis = actual.capture();
    actual.nodes.get("pkg/.git")!.raw = changed;
    expect(() => actual.api.validatePreflightReferenceBasis(basis)).toThrow(row.error);
  }
});

test("marker checks retain exact skipped paths, fake-marker behavior and symlink rejection", () => {
  for (const compiler of compilers) {
    const actual = environment(compiler);
    actual.put("nested/deep/consumer.json", "[]");
    actual.nodes.set("nested/deep/.git", { kind: "symlink", target: "/outside", ino: actual.nextInode(), mode: 0o777 });
    actual.put("pkg/.git", "not a Git pointer\n", false);
    expect(actual.capture().coordinateRoots).toEqual(["nested"]);
    expect(actual.accesses).not.toContain("nested/deep/.git");
    actual.nodes.set("pkg/.git", { kind: "symlink", target: "/outside", ino: actual.nextInode(), mode: 0o777 });
    expect(() => actual.capture()).toThrow("Reference repository marker is a symlink: pkg/.git");
    expect(actual.accesses).not.toContain("/outside");
  }
});

test("freshness does not share projected bytes, target indexes or frozen checks between queries", () => {
  for (const compiler of compilers) {
    const actual = environment(compiler), basis = actual.capture(), reads: string[] = [];
    const project = (name: string, bytes: Buffer, mode: number) => { expect(mode).toBe(0o644); reads.push(name); return { path: name, bytes: name === "consumer.json" ? Buffer.from("[]") : bytes }; };
    expect(actual.query(vector.queries[0], basis, project)).toEqual(["text\0nested/consumer.json\0pkg/alpha.ts"]);
    expect(actual.query(vector.queries[0], basis)).toEqual(vector.queries[0].expected);
    expect(reads).toContain("consumer.json");
    expect(actual.frozen.filter((name) => name === "consumer.json")).toHaveLength(2);
    expect(() => actual.api.validatePreflightReferenceBasis(basis)).not.toThrow();
  }
});

test("cancellation is observed during capture, every query and final validation", () => {
  for (const compiler of compilers) for (const phase of ["capture", "query", "final"]) {
    const actual = environment(compiler), basis = phase === "capture" ? undefined : actual.capture();
    actual.cancel();
    expect(() => phase === "capture" ? actual.capture() : phase === "query" ? actual.query(vector.queries[0], basis) : actual.api.validatePreflightReferenceBasis(basis)).toThrow("cancelled");
  }
});

test("explicit ignored-ticket and untracked consumers remain in the shared authority", () => {
  for (const compiler of compilers) {
    const actual = environment(compiler);
    actual.put("ticket/ignored.json", '["pkg/alpha.ts"]', false);
    actual.ticketPaths.add("ticket/ignored.json");
    actual.put("untracked.json", '["pkg/alpha.ts"]', false);
    actual.untracked.add("untracked.json");
    const basis = actual.capture();
    expect(actual.query(vector.queries[0], basis)).toEqual([...vector.queries[0].expected, "text\0ticket/ignored.json\0pkg/alpha.ts", "text\0untracked.json\0pkg/alpha.ts"]);
    actual.nodes.get("ticket/ignored.json")!.content = "[]";
    expect(() => actual.api.validatePreflightReferenceBasis(basis)).toThrow("node changed: ticket/ignored.json");
  }
});

test("query callbacks can cancel without bypassing the final or next-candidate check", () => {
  for (const compiler of compilers) for (const phase of ["query", "final"]) {
    const actual = environment(compiler), basis = actual.capture();
    actual.query(vector.queries[0], basis);
    const progress = (event: { completed: number }) => { if (event.completed === 0) actual.cancel(); };
    expect(() => phase === "query" ? actual.query(vector.queries[0], basis, undefined, progress) : actual.api.validatePreflightReferenceBasis(basis, undefined, progress)).toThrow("cancelled");
  }
});

test("basis identity cannot cross a repository, taxonomy object, plan or preflight invocation", () => {
  for (const compiler of compilers) {
    const first = environment(compiler), second = environment(compiler), basis = first.capture();
    expect(() => second.query(vector.queries[0], basis)).toThrow("basis context changed");
    first.query(vector.queries[0], basis);
    first.put("new.json", "[]");
    expect(() => first.api.validatePreflightReferenceBasis(basis)).toThrow("candidate membership changed");
    expect(() => first.api.validatePreflightReferenceBasis(first.capture())).not.toThrow();
  }
});

test("apply validates freshness after its publication callback and before source staging", () => {
  const apply = syntax.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "applyTaxonomyPlan")!.getText(syntax);
  const publication = apply.indexOf('transactionProbe("transaction-attempt-canonical-published"');
  const staging = apply.indexOf('journal.state = "staging"');
  const validation = apply.indexOf("validatePreflightReferenceBasis(", publication);
  expect(publication).toBeGreaterThan(0);
  expect(validation).toBeGreaterThan(publication);
  expect(validation).toBeLessThan(staging);
  expect(apply.slice(publication, staging)).toContain("if (preflightReferenceBasis)");
});

/** 🧾️ Allocates one new semantic owner; no existing run is removed or reused. */
function physicalRun(name: string): string {
  const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
  const report = join(ticket, "📓️preflight-reference-basis"), parent = join(report, "🧾️runs");
  let current = parse(parent).root;
  for (const part of relative(current, parent).split(sep)) {
    current = join(current, part);
    let stat;
    try { stat = lstatSync(current); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT" || current !== parent) throw error; mkdirSync(current); stat = lstatSync(current); }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Invalid preflight retained run ancestor: " + current);
  }
  const owner = join(parent, "🔖️" + randomUUID());
  mkdirSync(owner);
  writeFileSync(join(owner, "📝️.md"), `# Preflight Reference Basis Run\n\nCase: ${name}.\n\nRetain all inputs and active/failed recovery outputs until coordinator review. No cleanup is performed by this gate.\n`, { flag: "wx" });
  return owner;
}

/** 🧩️ Reuses only the authored embedded fixture function, never importing or executing the 62-case suite. */
function physicalFixture(name: string) {
  const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", schemaRelative = library + "/🔣️taxonomy.json", suitePath = join(root, library, "🧪️tests/🟦️transaction-v2.ts");
  const suite = readFileSync(suitePath, "utf8"), tree = ts.createSourceFile(suitePath, suite, ts.ScriptTarget.Latest, true);
  const helpers = tree.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === "embeddedFixture");
  expect(helpers).toHaveLength(1);
  const owner = physicalRun(name), repoRoot = join(owner, "🧪️fixture");
  mkdirSync(repoRoot);
  const put = (path: string, value: string | Buffer) => { const target = join(repoRoot, path); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, value); };
  const git = (...args: string[]) => { const result = Bun.spawnSync(["git", ...args], { cwd: repoRoot, stdout: "pipe", stderr: "pipe" }); if (result.exitCode !== 0) throw new Error(result.stderr.toString()); return result.stdout.toString().trim(); };
  const fixture = (_name: string, files: Record<string, string>, configure: (row: any) => void) => {
    const scope = "🧪️tests/🧪️fixture", workspace = join(repoRoot, scope), ticketDir = join(repoRoot, "🧪️tests"), schema = JSON.parse(readFileSync(join(root, schemaRelative), "utf8"));
    delete schema.generatorContracts["plugin-registry"].inputDiscovery;
    put(schemaRelative, JSON.stringify(schema, null, 2) + "\n");
    for (const [path, bytes] of Object.entries(files)) put(scope + "/" + path, bytes);
    put("external.json", "[]\n");
    const row = { repoRoot, root: repoRoot, scope, workspace, ticketDir, options: { repoRoot, scope, ticketDir, workers: 1 } };
    configure(row);
    git("init", "--quiet", "--object-format=sha1");
    put(".git/info/exclude", schemaRelative + "\n");
    git("add", "--all", "--", ".");
    git("-c", "user.name=Preflight Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "--quiet", "-m", "preflight fixture");
    return { ...row, baselineCommit: git("rev-parse", "HEAD") };
  };
  const original = helpers[0]!.getText(tree), compiled = new Bun.Transpiler({ loader: "ts" }).transformSync(original);
  const build = new Function("fixture", "join", "mkdirSync", "dirname", "writeFileSync", "readFileSync", "relative", "SCHEMA_REL", compiled + "\nreturn embeddedFixture;")(fixture, join, mkdirSync, dirname, writeFileSync, readFileSync, relative, schemaRelative);
  const row = build(name);
  put(".git/info/exclude", schemaRelative + "\n");
  return { ...row, owner, put, suite, suitePath, fixtureSourceHash: createHash("sha256").update(original).digest("hex") };
}

test("actual Git markers preserve nested ownership, fake markers and skipped symlink descendants", () => {
  const owner = physicalRun("actual-git-markers"), repoRoot = join(owner, "🧪️fixture");
  mkdirSync(repoRoot);
  const put = (path: string, bytes: string) => { const target = join(repoRoot, path); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, bytes); };
  const git = (cwd: string) => { const result = Bun.spawnSync(["git", "init", "--quiet", "--object-format=sha1"], { cwd, stdout: "pipe", stderr: "pipe" }); expect(result.exitCode).toBe(0); };
  put("nested/deep/consumer.json", "[]");
  put("fake/.git", "not a repository\n");
  put("fake/consumer.json", "[]");
  git(repoRoot);
  git(join(repoRoot, "nested"));
  symlinkSync("missing", join(repoRoot, "nested/deep/.git"), "file");
  const dependencies = { posix, resolve, relative, sep, join, dirname, isAbsolute, Buffer, lstatSync, readFileSync, execFileSync, spawnSync, checkCancellation: () => {} };
  const code = declarations(["sourceRelative", "normalizeRelative", "assertNoFollowAncestors", "assertLexicalInputOutsideOpaque", "LEXICAL_OPAQUE_ROOTS", "lstatOrNull", "isExcluded", "generatorPathCompare", "report", "ancestorReferenceCoordinateRoot", "referenceCoordinateRoots"]);
  const paths = ["nested/deep/consumer.json", "fake/consumer.json"], taxonomy = { exclusions: vector.semantics.opaqueRoots.map((path: string) => ({ path })) };
  for (const compiler of compilers) {
    const discover = new Function(...Object.keys(dependencies), compile(compiler, code) + "\nreturn referenceCoordinateRoots;")(...Object.values(dependencies));
    const markers: string[] = [];
    expect(discover(repoRoot, paths, taxonomy, undefined, (path: string) => markers.push(path))).toEqual(["nested"]);
    expect(markers).toContain("fake/.git");
    expect(markers).not.toContain("nested/deep/.git");
  }
  put("broken/consumer.json", "[]");
  symlinkSync("../fake/.git", join(repoRoot, "broken/.git"), "file");
  const discover = new Function(...Object.keys(dependencies), compile(compilers[0]!, code) + "\nreturn referenceCoordinateRoots;")(...Object.values(dependencies));
  expect(() => discover(repoRoot, [...paths, "broken/consumer.json"], taxonomy)).toThrow("Reference repository marker is a symlink: broken/.git");
  console.log("[DEBUG] Actual preflight Git marker proof", JSON.stringify({ owner, roots: ["nested"], skippedSymlink: true, directSymlinkRejected: true }));
});

for (const row of vector.physicalCases) test("physical preflight publication boundary: " + row.id, async () => {
  const api = await import("../../🧹️normalization/🟦️.ts"), fixture = physicalFixture(row.id);
  let evidence: Record<string, unknown> = { caseId: row.id, fixtureSourceHash: fixture.fixtureSourceHash };
  try {
    if (row.mutation === "marker") { fixture.put("outside-marker/consumer.json", "[]\n"); fixture.put("outside-marker/.git", "first fake marker\n"); }
    const plan = api.planTaxonomy(api.inventoryTaxonomy(fixture.options), { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [] });
    expect(plan.unresolved).toEqual([]);
    expect(plan.embeddedTicketRoots).toHaveLength(2);
    expect(plan.embeddedTicketRootRelocations).toHaveLength(3);
    expect(plan.evidenceRemovals).toHaveLength(1);
    const sources = [...plan.embeddedTicketRootRelocations, ...plan.evidenceRemovals].map((entry) => ({ path: entry.sourcePath, bytes: readFileSync(join(fixture.repoRoot, entry.sourcePath)) }));
    let published = 0;
    const result = api.applyTaxonomyPlan(plan, { ...fixture.options, expectedBaselineCommit: fixture.baselineCommit, injectFailureAt: "after-embedded-root-staging", progress: (event) => {
      if (event.phase !== "transaction-attempt-canonical-published") return;
      published++;
      const target = plan.embeddedTicketRoots[0]!.sourceTicketRoot;
      if (row.mutation === "add") fixture.put("late.json", JSON.stringify([target]) + "\n");
      if (row.mutation === "content") fixture.put("external.json", JSON.stringify([target]) + "\n");
      if (row.mutation === "marker") fixture.put("outside-marker/.git", "later fake marker\n");
    } });
    const journal = JSON.parse(readFileSync(result.journalPath, "utf8"));
    evidence = { ...evidence, result, journalError: journal.error, published, preparedEmbeddedRootIds: journal.preparedEmbeddedRootIds, stagedEmbeddedRootIds: journal.stagedEmbeddedRootIds };
    expect(published).toBe(1);
    expect(result.state).toBe("rolled-back");
    expect(journal.error).toContain(row.error);
    if (!row.staged) {
      expect(journal.preparedEmbeddedRootIds).toEqual([]);
      expect(journal.preparedEmbeddedRelocationIds).toEqual([]);
      expect(journal.preparedEvidenceRemovalIds).toEqual([]);
    }
    for (const entry of sources) expect(readFileSync(join(fixture.repoRoot, entry.path))).toEqual(entry.bytes);
    expect(readFileSync(fixture.suitePath, "utf8")).toBe(fixture.suite);
    evidence.passed = true;
    console.log("[DEBUG] Preflight publication boundary", JSON.stringify({ caseId: row.id, owner: fixture.owner, result: result.state, error: journal.error, sourcesUnchanged: sources.length }));
  } finally {
    const output = join(fixture.owner, "📊️outcome");
    mkdirSync(output);
    writeFileSync(join(output, "../🧪️🏔️preflight-reference-basis/🔣️.json"), JSON.stringify(evidence, null, 2) + "\n", { flag: "wx" });
  }
}, 15_000);

test("the dedicated preflight gate is registered through Nx and both ordered launch catalogs", () => {
  const expected = vector.execution, library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
  const project = JSON.parse(readFileSync(join(root, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  const router = readFileSync(join(root, library, "📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  expect(router.match(/segments\[0\] === "preflight-reference-basis"/gu)).toHaveLength(1);
  expect(router).toContain("🧪️tests/🟦️preflight-reference-basis.ts");
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const rows = parseJsonc(readFileSync(join(root, path), "utf8")).configurations;
    const matches = rows.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(matches).toHaveLength(1);
    expect(matches[0].command).toBe(expected.launchCommand);
    expect(matches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
    expect(rows.filter((entry: any) => entry.presentation?.group === expected.launchGroup && entry.presentation?.order === expected.launchOrder)).toHaveLength(1);
  }
});

test("the exercised production slices remain stable across the dedicated gate", () => {
  const names = ["referenceCoordinateRoots", "preflightReferenceNodeWitness", "observePreflightReferenceNode", "capturePreflightReferenceBasis", "validatePreflightReferenceBasis", "lexicalTargetIncomingReferences", "applyTaxonomyPlan"];
  const current = readFileSync(path, "utf8"), tree = ts.createSourceFile(path, current, ts.ScriptTarget.Latest, true);
  const actual = names.map((name) => tree.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name)!.getText(tree).replace(/^export /u, "")).join("\n");
  expect(actual).toBe(declarations(names));
  const hash = (value: string) => createHash("sha256").update(value).digest("hex");
  console.log("[DEBUG] Preflight source identity", JSON.stringify({ sourceBefore: hash(source), sourceAfter: hash(current), exercisedSlices: hash(actual), stable: true }));
});
