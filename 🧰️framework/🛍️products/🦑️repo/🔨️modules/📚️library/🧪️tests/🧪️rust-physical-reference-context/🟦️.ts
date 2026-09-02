//#region Imports
import { expect, test } from "bun:test";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readlinkSync, symlinkSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, parse, relative, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import { join as oraclePathJoin } from "pathe";
import ts from "typescript";
import * as rustDiscovery from "../../🔍️discovery/🟦️.ts";
import { inspectRustAssertionMessageSpans, inspectRustJoinArgumentSpans, inspectRustManifestPathReferences, inspectRustModuleGraph } from "../../🔍️discovery/🟦️.ts";
import { applyTaxonomyPlan, canonicalJson, inventoryTaxonomy, planTaxonomy } from "../../🧹️normalization/🟦️.ts";
//#endregion Imports

//#region Authority
const root = resolve(import.meta.dir, "../../../../../../../");
const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const vectorPath = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔣️rust-physical-reference-context.json");
const golden = JSON.parse(readFileSync(vectorPath, "utf8"));
const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const schemaBytes = readFileSync(join(root, schemaPath));
const retainedRunParent = join(ticket, ...golden.joinArguments.retention.parentSegments);
const retainedRuns = new Map<string, { dev: number; ino: number; reportHash: string }>();

/** 🛡️ Validates every ancestor of the exact retained run parent without following links. */
function verifyRetainedParent(create: boolean): void {
  let current = parse(retainedRunParent).root;
  for (const segment of relative(current, retainedRunParent).split(/[\\/]/u)) {
    current = join(current, segment);
    let stat;
    try { stat = lstatSync(current); }
    catch (error) {
      const withinTicket = relative(ticket, current);
      if (!create || (error as NodeJS.ErrnoException).code !== "ENOENT" || withinTicket === "" || withinTicket.startsWith("..") || resolve(ticket, withinTicket) !== current) throw error;
      mkdirSync(current);
      stat = lstatSync(current);
    }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Rust retained run ancestor is not a no-follow directory: ${current}`);
  }
}

/** 🧪️ Allocates one exact no-follow run owner without discarding authored or recovery evidence. */
function retainedRun(name: string): string {
  verifyRetainedParent(true);
  const directory = mkdtempSync(join(retainedRunParent, `🧪️${name}-`)), stat = lstatSync(directory);
  const report = `# Rust Physical Reference Run\n\nCase: ${name}.\n\nDisposition: retain all authored inputs, generated outputs and active or failed recovery evidence until exact review.\n\nThe run is allocated; the enclosing gate outcome is recorded in the parent report.\n`;
  writeFileSync(join(directory, "📝️.md"), report, { flag: "wx" });
  retainedRuns.set(directory, { dev: stat.dev, ino: stat.ino, reportHash: createHash("sha256").update(report).digest("hex") });
  return directory;
}

/** 📓️ Records a terminal assertion outcome only in the unchanged uniquely owned run. */
function retainRun(directory: string, passed: boolean): void {
  verifyRetainedParent(false);
  const ownership = retainedRuns.get(directory), stat = lstatSync(directory), reportPath = join(directory, "📝️.md"), reportStat = lstatSync(reportPath);
  if (!ownership || dirname(directory) !== retainedRunParent || !stat.isDirectory() || stat.isSymbolicLink() || stat.dev !== ownership.dev || stat.ino !== ownership.ino || !reportStat.isFile() || reportStat.isSymbolicLink()) throw new Error("Rust retained run ownership changed");
  const report = readFileSync(reportPath, "utf8");
  if (createHash("sha256").update(report).digest("hex") !== ownership.reportHash) throw new Error("Rust retained run report changed");
  writeFileSync(reportPath, `${report}\nAssertion outcome: ${passed ? "passed" : "failed or interrupted"}. No files were deleted.\n`);
}

/** 🧫️ Isolates Cargo ownership, a misleading sibling, and the exact normalized target. */
function fixture(kind = "mounted") {
  const directory = retainedRun(`path-${kind}`), vector = golden.transaction;
  const put = (path: string, content: string | Buffer) => { mkdirSync(dirname(join(directory, path)), { recursive: true }); writeFileSync(join(directory, path), content); };
  const schema = JSON.parse(schemaBytes.toString());
  delete schema.generatorContracts["plugin-registry"].inputDiscovery;
  put(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
  put(`${vector.scope}/${vector.source}`, vector.bytes);
  put(vector.lookalike, "wrong sibling\n");
  const manifest = '[package]\nname = "fixture"\nversion = "0.0.0"\nedition = "2021"\n[workspace]\n[lib]\npath = "entry.rs"\n[[bin]]\nname = "reader"\npath = "main.rs"\n';
  if (kind !== "symlink") put(vector.manifest, kind === "missing" ? '[package]\nname = "fixture"\n' : manifest);
  const reader = `pub fn read() { let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../${vector.scope}"); for path in ["${vector.source}"] { let source = std::fs::read_to_string(root.join(path)).unwrap(); println!("READ:{}", source.trim()); } }\n`;
  put(vector.consumer, kind === "unproven-base" ? reader.replace('std::path::Path::new(env!("CARGO_MANIFEST_DIR"))', "std::env::current_dir().unwrap()") : kind === "mutable-base" ? reader.replace("let root =", "let mut root =") : reader);
  put("pkg/main.rs", "fn main() { fixture::read(); }\n");
  if (kind === "ambiguous") put("second/Cargo.toml", '[package]\nname = "second"\n[lib]\npath = "../pkg/entry.rs"\n');
  if (kind === "symlink") { put("actual/Cargo.toml", manifest); symlinkSync("../actual/Cargo.toml", join(directory, vector.manifest)); }
  const git = (args: string[]) => { const result = Bun.spawnSync(["git", ...args], { cwd: directory, stdout: "pipe", stderr: "pipe" }); if (result.exitCode !== 0) throw new Error(result.stderr.toString()); return result.stdout.toString().trim(); };
  git(["init", "--quiet", "--object-format=sha1"]);
  put(".git/info/exclude", `${schemaPath}\n🧪️build/\npkg/Cargo.lock\n`);
  git(["-c", "user.name=Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "--quiet", "--allow-empty", "-m", "fixture"]);
  const baselineCommit = git(["rev-parse", "HEAD"]), ticketDir = join(directory, "🧪️transaction");
  return { directory, baselineCommit, ticketDir, put, options: { repoRoot: directory, scope: vector.scope, ticketDir, workers: 1 }, plan() { return planTaxonomy(inventoryTaxonomy(this.options), { baselineCommit, excludedTreeDigests: [] }); } };
}
//#endregion Authority

//#region Cases
test("string collection joins require exact standard receiver provenance", () => {
  const oracle = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(dirname(vectorPath), "🧬️join-provenance/🔣️.json"), "utf8")));
  expect(oracle(golden.joinArguments)).toBe(true);
  for (const changed of [{ ...golden.joinArguments, ownership: "guessed" }, { ...golden.joinArguments, extra: true }, { ...golden.joinArguments, cases: [] }]) expect(oracle(changed)).toBe(false);
  expect(new Set(golden.joinArguments.cases.map((row: { id: string }) => row.id)).size).toBe(golden.joinArguments.cases.length);
  for (const row of golden.joinArguments.cases) {
    const arguments_ = inspectRustJoinArgumentSpans(row.source);
    expect(arguments_.map(({ value }) => value), row.id).toEqual(row.expected);
    for (const argument of arguments_) expect(row.source.slice(argument.start, argument.end)).toBe(argument.value);
  }
});

test("literal predicates keep identifier tokens reachable under strict TypeScript narrowing", () => {
  const path = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
  const types = source.statements.filter((node) => (ts.isTypeAliasDeclaration(node) || ts.isInterfaceDeclaration(node)) && ["RustTokenKind", "RustToken"].includes(node.name.text)).map((node) => node.getText(source)).join("\n");
  for (const name of golden.tokenNarrowing.functions) {
    const owner = source.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name)!;
    const declarations: ts.VariableDeclaration[] = [];
    const visit = (node: ts.Node): void => { if (ts.isVariableDeclaration(node) && node.name.getText(source) === "literal") declarations.push(node); ts.forEachChild(node, visit); };
    visit(owner);
    expect(declarations).toHaveLength(1);
    const code = `${types}\nfunction probe(token: RustToken | undefined) { const ${declarations[0]!.getText(source)}; if (literal(token)) return "literal"; if (token?.kind === "identifier") return token.text; return null; }`;
    const virtualPath = join(ticket, `🟦️${name}.ts`), options: ts.CompilerOptions = { strict: true, noEmit: true, types: [], lib: ["lib.es5.d.ts", "lib.es2015.core.d.ts"], target: ts.ScriptTarget.ES2022, skipLibCheck: true };
    const host = ts.createCompilerHost(options), getSourceFile = host.getSourceFile.bind(host);
    host.getSourceFile = (path, languageVersion, onError, shouldCreateNewSourceFile) => path === virtualPath ? ts.createSourceFile(path, code, languageVersion, true) : getSourceFile(path, languageVersion, onError, shouldCreateNewSourceFile);
    const program = ts.createProgram([virtualPath], options, host);
    expect(ts.getPreEmitDiagnostics(program).map((diagnostic) => ({ code: diagnostic.code, message: ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n") }))).toEqual([]);
    for (const compiled of [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
      const probe = new Function(`${compiled}\nreturn probe;`)();
      for (const row of golden.tokenNarrowing.cases) expect(probe(row.token === null ? undefined : { ...row.token, start: 0, end: row.token.text.length })).toEqual(row.expected);
    }
  }
  console.log("[DEBUG] strict literal-token narrowing and independent runtime parity passed");
});

test("registers the physical Rust reference gate through Nx and both launch catalogs", () => {
  const expected = golden.execution;
  const project = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(root, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
  }
});

test("manifest-relative joins require immutable lexical bindings and exact loop ownership", () => {
  expect(golden.manifestPaths.roots).toEqual(['std::path::Path::new(env!("CARGO_MANIFEST_DIR"))', 'std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))']);
  expect(golden.manifestPaths.binding).toBe("immutable-lexical-local-only");
  expect(golden.manifestPaths.literal).toBe("unescaped-normal-string-only");
  expect(golden.manifestPaths.loop).toBe("literal-array-variable-used-only-as-one-proven-join-argument");
  expect(new Set(golden.manifestPaths.cases.map((row: { id: string }) => row.id)).size).toBe(golden.manifestPaths.cases.length);
  for (const row of golden.manifestPaths.cases) {
    const references = inspectRustManifestPathReferences(row.source);
    expect({ id: row.id, references: references.map(({ value, base }) => ({ value, base })) }).toEqual({ id: row.id, references: row.expected });
    for (const reference of references) expect(row.source.slice(reference.start, reference.end)).toBe(reference.value);
  }
});

test("implicit format captures cannot widen the single-use manifest path law", () => {
  const row = golden.manifestPaths.cases.find((row: { id: string }) => row.id === "path-loop-implicit-label");
  expect(row.expected).toEqual([]);
  expect(inspectRustManifestPathReferences(row.source)).toEqual([]);
});

test("finite manifest candidates prove complete correlated targets without editable loop authority", () => {
  const contract = golden.manifestCandidates;
  expect(contract.contract).toBe("rust-finite-manifest-path-candidates-v1");
  expect(contract.authority).toBe("candidate-only-never-editable");
  expect(contract.missingFact).toBe("unproven-never-complete-empty");
  expect(contract.maxExpandedIterations).toBe(256);
  expect(contract.maxTargetsPerSpan).toBe(256);
  expect(contract.tupleCorrelation).toBe("per-row-environment");
  expect(contract.coordinates).toBe("relative-component-chains-only");
  expect(contract.environment).toBe("standard-env-macro-with-no-shadow-or-foreign-glob");
  expect(contract.literal).toBe("unescaped-normal-string-only");
  expect(contract.grammar).toEqual(["exact-standard-manifest-root", "immutable-literal-string-or-tuple-array", "literal-number-boolean-and-full-slice-metadata", "lexical-destructured-for", "exact-array-iter-enumerate", "immutable-join-chain", "known-standard-read-only-macro-use"]);
  expect(new Set(contract.cases.map((row: { id: string }) => row.id)).size).toBe(contract.cases.length);
  const inspect = rustDiscovery.inspectRustManifestPathCandidates;
  expect(typeof inspect).toBe("function");
  for (const row of contract.cases) {
    const candidates = inspect(row.source).filter((candidate) => candidate.value === contract.selectedValue);
    expect(candidates.map(({ value, targets }) => ({ value, targets })), row.id).toEqual(row.expected);
    for (const candidate of candidates) expect(row.source.slice(candidate.start, candidate.end), row.id).toBe(candidate.value);
    const files = { [contract.manifestPath]: '[package]\nname="candidate"\n[lib]\npath="lib.rs"\n', [contract.consumerPath]: row.source };
    const graph = inspectRustModuleGraph(Object.keys(files), (path) => files[path], { strictManifests: true });
    const manifests = [...new Set((graph.contexts.get(contract.consumerPath) ?? []).map((context) => context.manifestPath).filter(Boolean))];
    expect(manifests).toEqual([contract.manifestPath]);
    const targets = [...new Set(candidates.flatMap((candidate) => candidate.targets.map((parts) => oraclePathJoin("pkg", ...parts))))].sort();
    expect(targets, row.id).toEqual(row.physicalTargets);
    const relevance = candidates.length === 0 ? "unproven" : targets.some((target) => target === contract.affectedRoot || target.startsWith(contract.affectedRoot + "/")) ? "intersects" : "disjoint";
    expect(relevance, row.id).toBe(row.relevance);
    expect(inspectRustManifestPathReferences(row.source).filter((reference) => reference.value === contract.selectedValue), row.id).toEqual([]);
  }
  const row = contract.cases[0], files = {
    [contract.manifestPath]: '[package]\nname="one"\n[lib]\npath="lib.rs"\n',
    "second/Cargo.toml": '[package]\nname="two"\n[lib]\npath="../pkg/lib.rs"\n',
    [contract.consumerPath]: row.source,
  };
  const graph = inspectRustModuleGraph(Object.keys(files), (path) => files[path], { strictManifests: true });
  expect(new Set((graph.contexts.get(contract.consumerPath) ?? []).map((context) => context.manifestPath)).size).toBe(2);
});

test("Cargo manifest ownership requires an actual unique module-mount proof", () => {
  for (const row of golden.manifestPaths.moduleGraphCases) {
    const graph = inspectRustModuleGraph(Object.keys(row.files), (path) => row.files[path], { strictManifests: true });
    const manifests = [...new Set((graph.contexts.get(row.target) ?? []).map((context) => context.manifestPath).filter(Boolean))].sort();
    expect({ id: row.id, manifests }).toEqual({ id: row.id, manifests: row.expectedManifests });
  }
});

test("scoped joins bind the real target instead of a sibling and survive rollback, retry, runtime and an empty replan", () => {
  const row = fixture(), vector = golden.transaction, plan = row.plan();
  let passed = false;
  try {
  expect(plan.unresolved).toEqual([]);
  expect(plan.moves.map((move) => [move.sourcePath, move.destinationPath])).toEqual([[`${vector.scope}/${vector.source}`, `${vector.scope}/${vector.destination}`]]);
  expect(plan.edits.map((edit) => [edit.path, edit.oldValue, edit.newValue])).toEqual([[vector.consumer, vector.source, vector.destination]]);
  expect(plan.edits[0]!.structuredLocation.startsWith("rust-path-join:")).toBe(true);
  const runtime = () => {
    const result = Bun.spawnSync(["cargo", "run", "--quiet", "--offline", "--manifest-path", join(row.directory, vector.manifest), "--target-dir", join(row.directory, "🧪️build"), "--bin", "reader"], { cwd: row.directory, env: { ...process.env, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe" });
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    expect(result.stdout.toString().trim()).toBe(vector.runtimeOutput);
  };
  runtime();
  const planPath = join(row.ticketDir, "🧾️plan/🔣️.json");
  row.put("🧪️transaction/🧾️plan/🔣️.json", `${canonicalJson(plan)}\n`);
  const options = { ...row.options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest, planArtifactPath: planPath };
  const rollback = applyTaxonomyPlan(plan, { ...options, injectFailureAt: "after-edits" });
  expect(rollback.state).toBe("rolled-back");
  expect(readFileSync(join(row.directory, vector.scope, vector.source), "utf8")).toBe(vector.bytes);
  expect(existsSync(join(row.directory, vector.scope, vector.destination))).toBe(false);
  runtime();
  expect(applyTaxonomyPlan(plan, options).state).toBe("committed");
  runtime();
  expect(readFileSync(join(row.directory, vector.lookalike), "utf8")).toBe("wrong sibling\n");
  expect(lstatSync(join(row.directory, vector.scope, vector.destination)).mode & 0o777).toBe(0o644);
  const empty = row.plan();
  expect([empty.moves.length, empty.edits.length, empty.unresolved.length]).toEqual([0, 0, 0]);
  passed = true;
  } finally {
    retainRun(row.directory, passed);
  }
}, 120_000);

for (const kind of ["missing", "ambiguous"]) test(`unproven ${kind} Cargo ownership fails closed`, () => {
  const row = fixture(kind), plan = row.plan();
  expect(plan.edits.filter((edit) => edit.path === golden.transaction.consumer)).toEqual([]);
  expect(plan.unresolved.some((problem) => problem.code === "reference-syntax-unsupported" && problem.message.includes("proven Cargo owner"))).toBe(true);
});

for (const kind of ["unproven-base", "mutable-base"]) test(`an external ${kind} path join is not silently dropped`, () => {
  const row = fixture(kind), plan = row.plan();
  expect(plan.edits.filter((edit) => edit.path === golden.transaction.consumer)).toEqual([]);
  expect(plan.unresolved.some((problem) => problem.code === "reference-syntax-unsupported" && problem.message.includes("immutable"))).toBe(true);
});

test("a symlinked Cargo ownership input is rejected without following it", () => {
  const row = fixture("symlink");
  expect(() => row.plan()).toThrow("symlink ancestor");
});

test("an unrelated unsafe Cargo candidate cannot become ownership authority or block the actual owner", () => {
  const row = fixture(), vector = golden.transaction, manifest = golden.referenceUniverse.unsafeUnrelatedManifest;
  row.put("unrelated-declaration/⚙️.toml", '[package]\nname = "unrelated"\n[lib]\npath = "entry.rs"\n');
  row.put("unrelated/entry.rs", "pub fn unrelated() {}\n");
  symlinkSync("../unrelated-declaration/⚙️.toml", join(row.directory, manifest));
  const plan = row.plan();
  expect(plan.unresolved).toEqual([]);
  expect(plan.edits.map((edit) => [edit.path, edit.oldValue, edit.newValue])).toEqual([[vector.consumer, vector.source, vector.destination]]);
  expect(readlinkSync(join(row.directory, manifest))).toBe("../unrelated-declaration/⚙️.toml");
});

for (const scenario of golden.referenceUniverse.cases) test(`Cargo reference ownership respects ${scenario.id} coordinates`, () => {
  const row = fixture(), vector = golden.transaction, independent = golden.referenceUniverse.independentRoot;
  const manifest = `${independent}/pkg/Cargo.toml`, consumer = `${independent}/pkg/entry.rs`;
  row.put(`${independent}/actual/⚙️.toml`, '[package]\nname = "independent"\n[lib]\npath = "entry.rs"\n');
  row.put(consumer, `pub fn read() { let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("${scenario.base}"); root.join("${vector.source}"); }\n`);
  row.put(`${independent}/${vector.scope}/${vector.source}`, "pub const LOCAL: bool = true;\n");
  symlinkSync("../actual/⚙️.toml", join(row.directory, manifest));
  const git = Bun.spawnSync(["git", "init", "--quiet", "--object-format=sha1"], { cwd: join(row.directory, independent), stdout: "pipe", stderr: "pipe" });
  expect(git.exitCode, git.stderr.toString()).toBe(0);
  const target = oraclePathJoin(independent, "pkg", scenario.base, vector.source);
  expect(target).toBe(scenario.physicalTarget);
  expect(target === `${vector.scope}/${vector.source}`).toBe(scenario.affectsParent);
  if (scenario.affectsParent) expect(() => row.plan()).toThrow("symlink ancestor");
  else {
    const plan = row.plan();
    expect(plan.unresolved).toEqual([]);
    expect(plan.edits.map((edit) => [edit.path, edit.oldValue, edit.newValue])).toEqual([[vector.consumer, vector.source, vector.destination]]);
    const localBytes = readFileSync(join(row.directory, consumer));
    const options = { ...row.options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest };
    expect(applyTaxonomyPlan(plan, { ...options, injectFailureAt: "after-edits" }).state).toBe("rolled-back");
    expect(readFileSync(join(row.directory, consumer))).toEqual(localBytes);
    expect(applyTaxonomyPlan(plan, options).state).toBe("committed");
    expect(readFileSync(join(row.directory, consumer))).toEqual(localBytes);
    const empty = row.plan();
    expect([empty.moves.length, empty.edits.length, empty.unresolved.length]).toEqual([0, 0, 0]);
  }
  expect(readlinkSync(join(row.directory, manifest))).toBe("../actual/⚙️.toml");
});

test("a newly relevant unsafe Cargo consumer rejects the frozen plan without overwriting its source", () => {
  const row = fixture(), vector = golden.transaction, plan = row.plan();
  row.put("new/entry.rs", readFileSync(join(row.directory, vector.consumer)));
  row.put("declaration/⚙️.toml", readFileSync(join(row.directory, vector.manifest)));
  symlinkSync("../declaration/⚙️.toml", join(row.directory, "new/Cargo.toml"));
  const bytes = readFileSync(join(row.directory, "new/entry.rs"));
  expect(() => applyTaxonomyPlan(plan, { ...row.options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow("symlink ancestor");
  expect(readFileSync(join(row.directory, "new/entry.rs"))).toEqual(bytes);
  expect(readFileSync(join(row.directory, vector.scope, vector.source), "utf8")).toBe(vector.bytes);
  expect(existsSync(join(row.directory, vector.scope, vector.destination))).toBe(false);
  expect(readlinkSync(join(row.directory, "new/Cargo.toml"))).toBe("../declaration/⚙️.toml");
});

for (const kind of ["consumer-drift", "manifest-drift", "new-incoming"]) test(`frozen scoped join authority rejects ${kind} without overwriting it`, () => {
  const row = fixture(), plan = row.plan(), vector = golden.transaction;
  expect(row.plan().planDigest).toBe(plan.planDigest);
  const consumer = readFileSync(join(row.directory, vector.consumer), "utf8"), manifest = readFileSync(join(row.directory, vector.manifest), "utf8");
  if (kind === "consumer-drift") row.put(vector.consumer, `${consumer}// concurrent source edit\n`);
  if (kind === "manifest-drift") row.put(vector.manifest, `${manifest}# concurrent manifest edit\n`);
  if (kind === "new-incoming") { row.put("new/Cargo.toml", manifest.replace('name = "fixture"', 'name = "new"')); row.put("new/entry.rs", consumer); }
  const bytes = readFileSync(join(row.directory, kind === "manifest-drift" ? vector.manifest : kind === "new-incoming" ? "new/entry.rs" : vector.consumer));
  expect(() => applyTaxonomyPlan(plan, { ...row.options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow();
  expect(readFileSync(join(row.directory, kind === "manifest-drift" ? vector.manifest : kind === "new-incoming" ? "new/entry.rs" : vector.consumer))).toEqual(bytes);
  expect(readFileSync(join(row.directory, vector.scope, vector.source), "utf8")).toBe(vector.bytes);
  expect(existsSync(join(row.directory, vector.scope, vector.destination))).toBe(false);
});

test("cancellation during Cargo-context planning remains read-only", () => {
  const row = fixture(), cancelFile = "🔣️cancel.json";
  expect(() => planTaxonomy(inventoryTaxonomy(row.options), { baselineCommit: row.baselineCommit, excludedTreeDigests: [], cancelFile, progress(event) { if (event.phase === "incoming-parse" && event.path === golden.transaction.consumer) row.put(cancelFile, "{}"); } })).toThrow(/cancel/iu);
  expect(readFileSync(join(row.directory, golden.transaction.scope, golden.transaction.source), "utf8")).toBe(golden.transaction.bytes);
  expect(existsSync(join(row.directory, golden.transaction.scope, golden.transaction.destination))).toBe(false);
});

test("Rust diagnostic references require exact unescaped assertion-message arguments", () => {
  const oracle = new Ajv().compile({ type: "object", required: ["schemaVersion", "contract", "assertionMessages"], properties: { schemaVersion: { const: 1 }, contract: { const: "rust-physical-reference-context-v1" }, assertionMessages: { type: "object", required: ["macros", "literal", "context", "cases"] } } });
  expect(oracle(golden)).toBe(true);
  for (const row of golden.assertionMessages.cases) {
    const messages = inspectRustAssertionMessageSpans(row.source);
    expect(messages.map(({ macroName, value }) => ({ macroName, value }))).toEqual(row.expected);
    for (const message of messages) expect(row.source.slice(message.start, message.end)).toBe(message.value);
  }
});

test("independent syn parsing reproduces assertion-message, manifest-path and join-provenance facts", async () => {
  const directory = retainedRun("syn-oracle"), target = join(directory, "🧪️target");
  writeFileSync(join(directory, "Cargo.toml"), readFileSync(join(root, golden.oracle.manifestInput)));
  writeFileSync(join(directory, "🦀️.rs"), readFileSync(join(root, golden.oracle.sourceInput)));
  let passed = false;
  try {
    const result = Bun.spawn(["cargo", "run", "--offline", "--quiet", "--manifest-path", join(directory, "Cargo.toml"), "--target-dir", target, "--", vectorPath], { cwd: directory, env: { ...process.env, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, exitCode] = await Promise.all([new Response(result.stdout).text(), new Response(result.stderr).text(), result.exited]);
    expect({ exitCode, stderr }).toEqual({ exitCode: 0, stderr: "" });
    expect(JSON.parse(stdout)).toEqual({
      assertionMessages: golden.assertionMessages.cases.map((row: any) => ({ id: row.id, messages: inspectRustAssertionMessageSpans(row.source).map(({ macroName, value }) => ({ macroName, value })) })),
      manifestPaths: golden.manifestPaths.cases.map((row: any) => ({ id: row.id, references: inspectRustManifestPathReferences(row.source).map(({ value, base }) => ({ value, base })) })),
      manifestCandidates: [...golden.manifestCandidates.cases, ...golden.manifestCandidates.adversarial.cases].map((row: any) => ({ id: row.id, candidates: rustDiscovery.inspectRustManifestPathCandidates(row.source).filter((candidate) => candidate.value === golden.manifestCandidates.selectedValue) })),
      joinArguments: golden.joinArguments.cases.map((row: any) => ({ id: row.id, candidates: inspectRustJoinArgumentSpans(row.source).map(({ value }) => value), allArguments: row.allArguments })),
    });
    passed = true;
  } finally {
    retainRun(directory, passed);
  }
}, 120_000);

test("rustc independently confirms delimiter strings and actual custom or standard path joins", () => {
  const directory = retainedRun("compiler-oracle");
  let passed = false;
  try {
    for (const row of golden.joinArguments.cases.filter((row: { compiler?: string }) => row.compiler)) {
      const owner = join(directory, `🧪️${row.id}`), input = join(owner, "🦀️.rs"), executable = join(owner, process.platform === "win32" ? "🧪️.exe" : "🧪️.bin");
      mkdirSync(owner);
      writeFileSync(input, `${row.source}\n${row.compiler}\n`, { flag: "wx" });
      const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "join_oracle", input, "-o", executable], { cwd: owner, env: { ...process.env, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
      expect(compiled.exitCode, `${row.id}: ${compiled.stderr.toString()}`).toBe(0);
      const runtime = Bun.spawnSync([executable], { cwd: owner, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 5_000 });
      expect(runtime.exitCode, `${row.id}: ${runtime.stderr.toString()}`).toBe(0);
      expect(runtime.stdout.toString().trim(), row.id).toBe(row.compilerOutput);
    }
    passed = true;
    console.log("Rust join compiler oracle confirmed six string/path runtime cases.");
  } finally { retainRun(directory, passed); }
}, 120_000);

test("finite candidate helper compiles independently under strict TypeScript", () => {
  const input = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"), source = ts.createSourceFile(input, readFileSync(input, "utf8"), ts.ScriptTarget.Latest, true);
  const names = new Set(["RustTokenKind", "RustToken", "RustManifestPathCandidate", "inspectRustManifestPathCandidates"]);
  const declarations = source.statements.filter((node) => (ts.isTypeAliasDeclaration(node) || ts.isInterfaceDeclaration(node) || ts.isFunctionDeclaration(node)) && node.name && names.has(node.name.text));
  expect(declarations).toHaveLength(4);
  const text = declarations.map((node) => node.getText(source)).join("\n") + '\ndeclare function rustTokens(source: string): RustToken[]; declare function rustTokenPairs(tokens: readonly RustToken[]): Map<number, number>; declare function rustTokenSegments(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number, delimiter: string): [number, number][]; declare function rustFindTopLevel(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number, values: ReadonlySet<string>): number; declare function rustRepoRootAncestorWalkHelperNames(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): ReadonlySet<string>;\n';
  const virtualPath = join(ticket, "📓️energy-rust-reference-diagnostics/🧭️manifest-pathbuf/🟦️typescript.ts"), options: ts.CompilerOptions = { strict: true, noEmit: true, types: [], target: ts.ScriptTarget.ES2022, skipLibCheck: true }, host = ts.createCompilerHost(options), original = host.getSourceFile.bind(host);
  host.getSourceFile = (path, language, onError, create) => path === virtualPath ? ts.createSourceFile(path, text, language, true) : original(path, language, onError, create);
  expect(ts.getPreEmitDiagnostics(ts.createProgram([virtualPath], options, host)).map((diagnostic) => ({ code: diagnostic.code, message: ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n") }))).toEqual([]);
});

test("rustc confirms correlated finite receiver targets and missing-target rejection", () => {
  const directory = retainedRun("finite-candidate-compiler");
  let passed = false;
  try {
    const cases = golden.manifestCandidates.cases.filter((row: { runtime?: boolean }) => row.runtime);
    expect(cases).toHaveLength(5);
    for (const row of cases) for (const missing of row.id === "tuple-row-correlation" ? [false, true] : [false]) {
      const owner = join(directory, row.id + (missing ? "-missing" : "-present")), manifestDirectory = join(owner, "pkg"), input = join(owner, "🦀️.rs"), executable = join(owner, process.platform === "win32" ? "🧪️.exe" : "🧪️.bin");
      mkdirSync(manifestDirectory, { recursive: true });
      for (const path of row.physicalTargets.slice(missing ? 1 : 0)) { mkdirSync(dirname(join(owner, path)), { recursive: true }); writeFileSync(join(owner, path), "exact finite target\n", { flag: "wx" }); }
      writeFileSync(input, row.source + '\nfn main() { inspect(); println!("finite-targets-confirmed"); }\n', { flag: "wx" });
      const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "finite_candidate_oracle", input, "-o", executable], { cwd: owner, env: { ...process.env, CARGO_MANIFEST_DIR: manifestDirectory, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
      expect(compiled.exitCode, row.id + ": " + compiled.stderr.toString()).toBe(0);
      const runtime = Bun.spawnSync([executable], { cwd: owner, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 5_000 });
      if (missing) { expect(runtime.exitCode).not.toBe(0); expect(runtime.stderr.toString()).toContain("leaf.rs"); }
      else { expect(runtime.exitCode, row.id + ": " + runtime.stderr.toString()).toBe(0); expect(runtime.stdout.toString().trim()).toBe("finite-targets-confirmed"); }
    }
    passed = true;
    console.log("Rust finite receiver compiler oracle confirmed five exact-target cases and one missing-target rejection.");
  } finally { retainRun(directory, passed); }
}, 120_000);

for (const row of golden.manifestCandidates.adversarial.cases) test(`finite candidate adversarial runtime: ${row.id}`, () => {
  const contract = golden.manifestCandidates.adversarial;
  expect(contract.contract).toBe("rust-finite-candidate-adversarial-v1");
  expect(contract.unknownControlFlow).toBe("captured-bindings-remain-unproven");
  expect(contract.namespace).toBe("standard-type-and-macro-identity-required");
  const directory = retainedRun("finite-adversarial-" + row.id), manifestDirectory = join(directory, "pkg"), input = join(directory, "🦀️.rs"), executable = join(directory, process.platform === "win32" ? "🧪️.exe" : "🧪️.bin");
  let passed = false;
  try {
    mkdirSync(manifestDirectory);
    for (const [path, bytes] of Object.entries(row.runtimeProof.files ?? {})) writeFileSync(join(directory, path), bytes as string, { flag: "wx" });
    writeFileSync(input, row.source + `\nfn main() { ${row.runtimeProof.call}; }\n`, { flag: "wx" });
    const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "candidate_adversarial", input, "-o", executable], { cwd: directory, env: { ...process.env, CARGO_MANIFEST_DIR: manifestDirectory, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
    expect(compiled.exitCode, compiled.stderr.toString()).toBe(0);
    const runtime = Bun.spawnSync([executable], { cwd: directory, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 5_000 });
    expect(runtime.exitCode, runtime.stderr.toString()).toBe(0);
    const observed = runtime.stdout.toString().trim().split(/\r?\n/u).map((path) => relative(directory, path).replaceAll("\\", "/"));
    expect(observed).toEqual(row.runtimeProof.targets);
    console.log(`Rust adversarial ${row.id}: ${JSON.stringify(observed)}`);
    const candidates = rustDiscovery.inspectRustManifestPathCandidates(row.source).filter((candidate) => candidate.value === golden.manifestCandidates.selectedValue);
    expect(candidates.map(({ value, targets }) => ({ value, targets }))).toEqual(row.expected);
    expect(inspectRustManifestPathReferences(row.source).filter((reference) => reference.value === golden.manifestCandidates.selectedValue)).toEqual([]);
    passed = true;
  } finally { retainRun(directory, passed); }
}, 120_000);

test("finite candidate generic std namespace and expanded env ambiguities are rejected by Rust", () => {
  const directory = retainedRun("finite-generic-namespace");
  let passed = false;
  try {
    for (const row of golden.manifestCandidates.adversarial.namespaceReviews) {
      const owner = join(directory, row.id), input = join(owner, "🦀️.rs");
      mkdirSync(owner);
      for (const [path, bytes] of Object.entries(row.files ?? {})) writeFileSync(join(owner, path), bytes as string, { flag: "wx" });
      writeFileSync(input, row.source, { flag: "wx" });
      const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "namespace_review", input, "-o", join(owner, "🧪️.bin")], { cwd: owner, env: { ...process.env, CARGO_MANIFEST_DIR: owner, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
      expect(compiled.exitCode).not.toBe(0);
      expect(compiled.stderr.toString()).toContain(row.errorCode);
    }
    passed = true;
  } finally { retainRun(directory, passed); }
}, 120_000);

test("rustc independently confirms manifest-root PathBuf construction and format capture behavior", () => {
  const directory = retainedRun("manifest-pathbuf-compiler");
  let passed = false;
  try {
    const cases = golden.manifestPaths.cases.filter((row: { compiler?: string }) => row.compiler);
    expect(cases).toHaveLength(5);
    for (const row of cases) {
      const owner = join(directory, `🧪️${row.id}`), input = join(owner, "🦀️.rs"), executable = join(owner, process.platform === "win32" ? "🧪️.exe" : "🧪️.bin");
      mkdirSync(owner);
      writeFileSync(input, `${row.source}\n${row.compiler}\n`, { flag: "wx" });
      const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "manifest_pathbuf_oracle", input, "-o", executable], { cwd: owner, env: { ...process.env, CARGO_MANIFEST_DIR: owner, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
      expect(compiled.exitCode, `${row.id}: ${compiled.stderr.toString()}`).toBe(0);
      const runtime = Bun.spawnSync([executable], { cwd: owner, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 5_000 });
      expect(runtime.exitCode, `${row.id}: ${runtime.stderr.toString()}`).toBe(0);
      expect(runtime.stdout.toString().trim().replaceAll("\r\n", "\n"), row.id).toBe(row.compilerOutput);
    }
    passed = true;
    console.log("Rust manifest PathBuf compiler oracle confirmed five constructor/capture runtime cases.");
  } finally { retainRun(directory, passed); }
}, 120_000);
//#endregion Cases
