//#region Imports
import { expect, test } from "bun:test";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readlinkSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import { join as oraclePathJoin } from "pathe";
import ts from "typescript";
import { inspectRustAssertionMessageSpans, inspectRustManifestPathReferences, inspectRustModuleGraph } from "../../🔍️discovery/🟦️component.ts";
import { applyTaxonomyPlan, canonicalJson, inventoryTaxonomy, planTaxonomy } from "../../🧹️normalization/🟦️.ts";
//#endregion Imports

//#region Authority
const root = resolve(import.meta.dir, "../../../../../../../");
const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const vectorPath = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️rust-physical-reference-context/🔣️.json");
const golden = JSON.parse(readFileSync(vectorPath, "utf8"));
const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const schemaBytes = readFileSync(join(root, schemaPath));

/** 🧫️ Isolates Cargo ownership, a misleading sibling, and the exact normalized target. */
function fixture(kind = "mounted") {
  const directory = mkdtempSync(join(ticket, "🧪️rust-path-")), vector = golden.transaction;
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
test("literal predicates keep identifier tokens reachable under strict TypeScript narrowing", () => {
  const path = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts");
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
  const types = source.statements.filter((node) => (ts.isTypeAliasDeclaration(node) || ts.isInterfaceDeclaration(node)) && ["RustTokenKind", "RustToken"].includes(node.name.text)).map((node) => node.getText(source)).join("\n");
  for (const name of golden.tokenNarrowing.functions) {
    const owner = source.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name)!;
    const declarations: ts.VariableDeclaration[] = [];
    const visit = (node: ts.Node): void => { if (ts.isVariableDeclaration(node) && node.name.getText(source) === "literal") declarations.push(node); ts.forEachChild(node, visit); };
    visit(owner);
    expect(declarations).toHaveLength(1);
    const code = `${types}\nfunction probe(token: RustToken | undefined) { const ${declarations[0]!.getText(source)}; if (literal(token)) return "literal"; if (token?.kind === "identifier") return token.text; return null; }`;
    const virtualPath = join(ticket, `🧪️${name}/🟦️.ts`), options: ts.CompilerOptions = { strict: true, noEmit: true, types: [], lib: ["lib.es5.d.ts", "lib.es2015.core.d.ts"], target: ts.ScriptTarget.ES2022, skipLibCheck: true };
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
  for (const row of golden.manifestPaths.cases) {
    const references = inspectRustManifestPathReferences(row.source);
    expect({ id: row.id, references: references.map(({ value, base }) => ({ value, base })) }).toEqual({ id: row.id, references: row.expected });
    for (const reference of references) expect(row.source.slice(reference.start, reference.end)).toBe(reference.value);
  }
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
  } finally {
    rmSync(join(row.directory, "🧪️build"), { recursive: true, force: true });
    rmSync(join(row.directory, "pkg/Cargo.lock"), { force: true });
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
  const row = fixture(), cancelFile = "🧪️cancel/🔣️.json";
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

test("independent syn parsing reproduces assertion-message and manifest-path ownership facts", async () => {
  const directory = mkdtempSync(join(ticket, "🧪️rust-syn-oracle-")), target = join(directory, "🧪️target");
  writeFileSync(join(directory, "Cargo.toml"), readFileSync(join(root, golden.oracle.manifestInput)));
  writeFileSync(join(directory, "🦀️.rs"), readFileSync(join(root, golden.oracle.sourceInput)));
  try {
    const result = Bun.spawn(["cargo", "run", "--offline", "--quiet", "--manifest-path", join(directory, "Cargo.toml"), "--target-dir", target, "--", vectorPath], { cwd: directory, env: { ...process.env, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, exitCode] = await Promise.all([new Response(result.stdout).text(), new Response(result.stderr).text(), result.exited]);
    expect({ exitCode, stderr }).toEqual({ exitCode: 0, stderr: "" });
    expect(JSON.parse(stdout)).toEqual({
      assertionMessages: golden.assertionMessages.cases.map((row: any) => ({ id: row.id, messages: inspectRustAssertionMessageSpans(row.source).map(({ macroName, value }) => ({ macroName, value })) })),
      manifestPaths: golden.manifestPaths.cases.map((row: any) => ({ id: row.id, references: inspectRustManifestPathReferences(row.source).map(({ value, base }) => ({ value, base })) })),
    });
  } finally {
    rmSync(target, { recursive: true, force: true });
    rmSync(join(directory, "Cargo.lock"), { force: true });
  }
}, 120_000);
//#endregion Cases
