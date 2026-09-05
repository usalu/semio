import { afterAll, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { spawn as nodeSpawn, spawnSync as nodeSpawnSync } from "node:child_process";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, parse, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";

type Candidate = { start: number; end: number; value: string; targets: string[][] };
type Capture = { name: string; start: number; end: number };
type Callback = { start: number; end: number; bodyStart: number; bodyEnd: number; parameter: Capture; macroPath: string; captures: Capture[]; freeVariables: string[] };
type Row = { id: string; source: string; reason: string; selectedValues: string[]; expectedCandidates: Candidate[]; expectedCallbacks: Callback[]; native: string | null };
const root = resolve(import.meta.dir, "../../../../../../../"), ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
const vectorPath = join(import.meta.dir, "../↪️rust-divergence-callback/🔣️.json"), vector = JSON.parse(readFileSync(vectorPath, "utf8"));
const rows = vector.cases as Row[], hash = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const inputs = [vectorPath, join(import.meta.dir, "../↪️rust-divergence-callback/🧬️schema/🔣️.json"), join(import.meta.dir, "../↪️rust-divergence-callback/🟦️.ts")];
const identities = inputs.map((path) => ({ path, sha256: hash(readFileSync(path)) }));
const runParent = join(ticket, ...vector.retention.parentSegments);
let helpers: Promise<typeof import("../../🔍️discovery/🟦️.ts")> | undefined;

/** 🔬️ Loads and pins discovery only for a phase that actually exercises its source helpers. */
async function sourceHelpers() {
  if (!helpers) {
    const path = resolve(import.meta.dir, "../../🔍️discovery/🟦️.ts");
    identities.push({ path, sha256: hash(readFileSync(path)) });
    helpers = import("../../🔍️discovery/🟦️.ts");
  }
  return helpers;
}

/** 🧫️ Allocates one no-follow ticket run, retaining each compiler input and review evidence. */
function runOwner(name: string): string {
  let current = parse(runParent).root;
  for (const segment of relative(current, runParent).split(/[\\/]/u)) {
    current = join(current, segment);
    try { lstatSync(current); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT" || current === ticket || !current.startsWith(ticket + sep)) throw error;
      mkdirSync(current);
    }
    const stat = lstatSync(current);
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Callback run ancestor is not a no-follow directory");
  }
  const owner = mkdtempSync(join(runParent, "🔖️" + name + "-"));
  writeFileSync(join(owner, "📝️.md"), "# Divergence Callback Oracle Run\n\nCase: " + name + ".\n\nNew isolated inputs; no historical run reconstruction. Retain during the initial RED review. No production changes or live reference authority are implied.\n", { flag: "wx" });
  return owner;
}

/** 📓️ Records exact process outcomes in the current uniquely owned run report. */
function record(owner: string, result: { command: string[]; exitCode: number | null; stdout: string; stderr: string }): void {
  const path = join(owner, "📝️.md");
  writeFileSync(path, readFileSync(path, "utf8") + "\n## Process\n\nCommand: " + result.command.join(" ") + ".\n\nExit: " + result.exitCode + ".\n\nOutput:\n\n" + result.stdout + result.stderr + "\n");
}

/** 🧬️ Records exact inputs separately from compiler output and never reuses an older run. */
function evidence(owner: string, label: string, value: unknown): void {
  const path = join(owner, "📝️.md");
  writeFileSync(path, readFileSync(path, "utf8") + "\n## " + label + "\n\n" + JSON.stringify(value, null, 2) + "\n");
}

/** 🌳️ Observes only the owned compiler group or its explicitly discovered Windows descendants. */
function compilerPids(pid: number, observed: Set<number>): number[] {
  const command = process.platform === "win32"
    ? ["powershell", "-NoProfile", "-NonInteractive", "-Command", "Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId | ConvertTo-Json -Compress"]
    : ["ps", "-axo", "pid=,ppid=,pgid="];
  const result = nodeSpawnSync(command[0]!, command.slice(1), { encoding: "utf8", timeout: 1_000 });
  if (result.status !== 0) throw new Error("Compiler PID observation failed: " + result.stderr);
  const rows: { pid: number; parent: number; group?: number }[] = process.platform === "win32"
    ? [JSON.parse(result.stdout)].flat().map((row) => ({ pid: row.ProcessId, parent: row.ParentProcessId }))
    : result.stdout.trim().split(/\r?\n/u).filter(Boolean).map((line) => { const [child, parent, group] = line.trim().split(/\s+/u).map(Number); return { pid: child!, parent: parent!, group }; });
  if (process.platform === "win32") {
    let changed = true;
    while (changed) { changed = false; for (const row of rows) if (observed.has(row.parent) && !observed.has(row.pid)) { observed.add(row.pid); changed = true; } }
  }
  const live = rows.filter((row) => process.platform === "win32" ? observed.has(row.pid) : row.group === pid).map((row) => row.pid);
  for (const child of live) observed.add(child);
  return live;
}

/** ⏱️ Uses one declared compiler deadline, with owned-tree termination and observable terminal closure. */
async function coldCompiler(owner: string, command: string[], budgetMs: number) {
  const started = performance.now(), child = nodeSpawn(command[0]!, command.slice(1), { cwd: owner, env: { ...process.env, RUSTC_WRAPPER: "" }, detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] });
  if (!child.pid) throw new Error("Compiler has no PID");
  const pid = child.pid, observed = new Set([pid]);
  let stdout = "", stderr = "", timedOut = false, observationError: string | null = null;
  child.stdout.setEncoding("utf8"); child.stderr.setEncoding("utf8");
  child.stdout.on("data", (text: string) => { stdout += text; }); child.stderr.on("data", (text: string) => { stderr += text; });
  const stop = () => {
    if (process.platform === "win32") nodeSpawnSync("taskkill", ["/pid", String(pid), "/T", "/F"], { stdio: "ignore", timeout: 1_000 });
    else try { process.kill(-pid, "SIGKILL"); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") observationError = String(error); }
  };
  const timer = setTimeout(() => { timedOut = true; stop(); }, Math.max(1, budgetMs));
  const sample = setInterval(() => { try { compilerPids(pid, observed); } catch (error) { observationError = String(error); stop(); } }, 250);
  console.log("[DEBUG] cold syn compiler PID " + pid + ", explicit remaining compiler budget " + Math.round(budgetMs) + "ms");
  const outcome = await new Promise<{ exitCode: number | null; signal: NodeJS.Signals | null }>((accept, reject) => { child.once("error", reject); child.once("close", (exitCode, signal) => accept({ exitCode, signal })); }).finally(() => { clearTimeout(timer); clearInterval(sample); });
  const survivors = compilerPids(pid, observed);
  const result = { command, pid, observedPids: [...observed].sort((a, b) => a - b), ...outcome, stdout, stderr, timedOut, observationError, survivors, elapsedMs: performance.now() - started };
  record(owner, result); evidence(owner, "Compiler Process Closure", { ...result, stdout: undefined, stderr: undefined });
  return result;
}

afterAll(() => {
  for (const input of identities) expect(hash(readFileSync(input.path)), input.path + " changed during callback packet").toBe(input.sha256);
});

test("closed divergence callback contract preserves candidate-only and physical-proof separation", () => {
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  expect(rows).toHaveLength(38);
  expect(new Set(rows.map((row) => row.id)).size).toBe(rows.length);
  expect(rows.filter((row) => row.reason === "closed")).toHaveLength(4);
  expect(vector.registration.execution.map((row: { budgetMs: number }) => row.budgetMs)).toEqual([15_000, 15_000, 120_000]);
  const labels = ["closed divergence contract", "shared callback labels", "candidate-only callback: builtin attributed closure remains conservative", ...rows.map((row) => "candidate-only callback: " + row.id), "actual rustc oracle", ...vector.attributeCompilerCases.map((row: { id: string }) => "actual rustc attributed callback validity: " + row.id), "independent syn callback oracle", ...vector.registration.execution.map((row: { phase: string }) => "closed divergence registration: " + row.phase)];
  for (const label of labels) expect(vector.registration.execution.filter((row: { pattern: string }) => new RegExp(row.pattern, "u").test(label))).toHaveLength(1);
  for (const changed of [
    { ...vector, compatibility: true },
    { ...vector, semantics: { ...vector.semantics, authority: "editable" } },
    { ...vector, semantics: { ...vector.semantics, maxExpandedIterations: 257 } },
    { ...vector, scope: { ...vector.scope, liveEnergyRowsRemovedClaim: 9 } },
    { ...vector, cases: [rows[0]] },
    { ...vector, cases: rows.map((row, index) => index ? row : { ...row, guessedRoot: true }) },
  ]) expect(validate(changed)).toBe(false);
  for (const row of rows) {
    expect(row.expectedCallbacks.length > 0).toBe(row.reason === "closed");
    for (const candidate of row.expectedCandidates) expect(row.source.slice(candidate.start, candidate.end)).toBe(candidate.value);
    for (const callback of row.expectedCallbacks) {
      expect(row.source.slice(callback.parameter.start, callback.parameter.end)).toBe(callback.parameter.name);
      expect(row.source.slice(callback.start, callback.end)).toContain("|" + callback.parameter.name + "|");
      expect(row.source.slice(callback.bodyStart, callback.bodyEnd)).toStartWith(callback.macroPath + "!");
      for (const capture of callback.captures) expect(row.source.slice(capture.start, capture.end)).toBe(capture.name);
      expect([...new Set(callback.captures.map((capture) => capture.name).filter((name) => name !== callback.parameter.name))].sort()).toEqual(callback.freeVariables);
    }
  }
});

for (const registration of vector.registration.execution) test("closed divergence registration: " + registration.phase, () => {
  const contract = vector.registration, project = JSON.parse(readFileSync(join(root, contract.projectPath), "utf8"));
  expect(project.targets[registration.target]).toEqual({ executor: "nx:run-commands", options: { cwd: dirname(contract.projectPath), command: registration.command } });
  const manifest = JSON.parse(readFileSync(join(root, contract.packagePath), "utf8"));
  expect(manifest.scripts[registration.target]).toBe(registration.packageCommand);
  const router = ts.createSourceFile(contract.routerPath, readFileSync(join(root, contract.routerPath), "utf8"), ts.ScriptTarget.Latest, true), branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => { if (ts.isIfStatement(node) && node.expression.getText(router) === 'segments[0] === "' + registration.route + '"') branches.push(node); ts.forEachChild(node, visit); };
  visit(router); expect(branches).toHaveLength(1);
  expect(branches[0]!.thenStatement.getText(router)).toContain(JSON.stringify(contract.testPath));
  const calls: ts.CallExpression[] = [];
  const callVisit = (node: ts.Node): void => { if (ts.isCallExpression(node) && node.expression.getText(router) === "runTestBudgeted") calls.push(node); ts.forEachChild(node, callVisit); };
  callVisit(branches[0]!.thenStatement); expect(calls).toHaveLength(1);
  expect(calls[0]!.arguments[0]!.getText(router)).toBe("process.execPath");
  const arguments_ = calls[0]!.arguments[1]; expect(arguments_ && ts.isArrayLiteralExpression(arguments_)).toBe(true);
  const expressions = (arguments_ as ts.ArrayLiteralExpression).elements;
  expect(expressions.map((node) => ts.isStringLiteral(node) ? node.text : node.getText(router))).toEqual(["test", "source", "--test-name-pattern", registration.pattern, "...segments.slice(1)"]);
  const options = calls[0]!.arguments[2]; expect(options && ts.isObjectLiteralExpression(options)).toBe(true);
  expect(Object.fromEntries((options as ts.ObjectLiteralExpression).properties.map((property) => { expect(ts.isPropertyAssignment(property)).toBe(true); const assignment = property as ts.PropertyAssignment; return [assignment.name.getText(router), assignment.initializer.getText(router)]; }))).toEqual(registration.phase === "syn" ? { cwd: "this.repoRoot", budgetMs: "120_000" } : { cwd: "this.repoRoot" });
  for (const path of contract.launchPaths) {
    const configurations = parseJsonc(readFileSync(join(root, path), "utf8")).configurations.filter((row: { name: string }) => row.name === registration.launchName);
    expect(configurations).toHaveLength(1);
    expect(configurations[0]).toEqual({ name: registration.launchName, type: "node-terminal", request: "launch", command: registration.launchCommand, cwd: "$" + "{workspaceFolder}", presentation: { group: "4_gate", order: registration.launchOrder } });
  }
});

test("shared callback and assertion labels never gain writable source proof", async () => {
  const { inspectRustManifestPathReferences } = await sourceHelpers();
  for (const row of rows) expect(inspectRustManifestPathReferences(row.source).filter((reference) => row.selectedValues.includes(reference.value)), row.id).toEqual([]);
});

test("candidate-only callback: builtin attributed closure remains conservative", async () => {
  const { inspectRustManifestPathCandidates, inspectRustManifestPathReferences } = await sourceHelpers();
  const source = vector.attributeCompilerCases.find((row: { id: string }) => row.id === "builtin-allow-expression-attribute").source;
  const values = new Set(["facet.json", "snapshot.rs"]);
  expect(inspectRustManifestPathReferences(source).filter((row) => values.has(row.value))).toEqual([]);
  expect(inspectRustManifestPathCandidates(source).filter((row) => values.has(row.value))).toEqual([]);
});

for (const row of rows) test("candidate-only callback: " + row.id, async () => {
  const { inspectRustManifestPathCandidates, inspectRustManifestPathReferences } = await sourceHelpers();
  const selected = new Set(row.selectedValues);
  const actual = inspectRustManifestPathCandidates(row.source).filter((candidate) => selected.has(candidate.value));
  expect(actual, row.reason).toEqual(row.expectedCandidates);
  expect(inspectRustManifestPathReferences(row.source).filter((reference) => selected.has(reference.value))).toEqual([]);
  for (const candidate of actual) expect(row.source.slice(candidate.start, candidate.end)).toBe(candidate.value);
});

test("independent syn callback AST, spans, free variables and target tuples match the neutral contract", async () => {
  const started = performance.now(), owner = runOwner("syn-120s-cold"), manifest = join(owner, "Cargo.toml"), source = join(owner, "../↪️rust-divergence-callback/🦀️.rs"), target = join(owner, "🏗️target");
  expect(existsSync(target)).toBe(false);
  writeFileSync(manifest, readFileSync(join(root, vector.oracle.manifestInput)), { flag: "wx" });
  writeFileSync(source, readFileSync(join(root, vector.oracle.sourceInput)), { flag: "wx" });
  const retained = [join(owner, "🧬️inputs/🔣️vector.json"), join(owner, "🧬️inputs/🧪️vector/🧬️schema/🔣️.json"), join(owner, "🧬️inputs/🟦️harness.ts")];
  for (let index = 0; index < inputs.length; index++) {
    mkdirSync(dirname(retained[index]!), { recursive: true });
    writeFileSync(retained[index]!, readFileSync(inputs[index]!), { flag: "wx" });
  }
  const phaseInputs = [...inputs, join(root, vector.oracle.manifestInput), join(root, vector.oracle.sourceInput), manifest, source, ...retained];
  const before = phaseInputs.map((path) => ({ path, sha256: hash(readFileSync(path)) }));
  evidence(owner, "Cold Compiler Authority", { budgetMs: 120_000, targetInitiallyAbsent: true, inputs: before });
  const command = ["cargo", "run", "--offline", "--manifest-path", manifest, "--target-dir", target, "--", retained[0]!];
  const result = await coldCompiler(owner, command, 118_000 - (performance.now() - started));
  const after = before.map((input) => ({ path: input.path, sha256: hash(readFileSync(input.path)) }));
  evidence(owner, "Input Readback", { inputs: after, stable: JSON.stringify(before) === JSON.stringify(after), elapsedMs: performance.now() - started });
  expect(after).toEqual(before);
  expect(result.timedOut).toBe(false); expect(result.observationError).toBeNull(); expect(result.survivors).toEqual([]);
  expect(result.exitCode, result.stderr).toBe(0);
  expect(JSON.parse(result.stdout)).toEqual({ rows: rows.map((row) => ({ id: row.id, parseable: true, candidates: row.expectedCandidates, callbacks: row.expectedCallbacks })) });
  console.log("[DEBUG] independent syn callback AST/span/free-variable oracle matched 38 closed vectors");
}, 120_000);

for (const row of vector.attributeCompilerCases) test("actual rustc attributed callback validity: " + row.id, () => {
  const owner = runOwner("attribute-" + row.id), manifestRoot = join(owner, "pkg"), source = join(owner, "../↪️rust-divergence-callback/🦀️.rs"), binary = join(owner, process.platform === "win32" ? "🔣️.exe" : "../↪️rust-divergence-callback/🔣️.json");
  mkdirSync(manifestRoot);
  writeFileSync(source, row.source + "\nfn main() { inspect(); }\n", { flag: "wx" });
  const command = ["rustc", "--edition=2021", "--crate-name", "attributed_callback", source, "-o", binary];
  const compile = Bun.spawnSync(command, { cwd: owner, env: { ...process.env, CARGO_MANIFEST_DIR: manifestRoot, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe", timeout: 4_000 });
  record(owner, { command, exitCode: compile.exitCode, stdout: compile.stdout.toString(), stderr: compile.stderr.toString() });
  expect(compile.exitCode, compile.stderr.toString()).toBe(row.compileExit);
  if (row.runtimeExit === null) expect(compile.stderr.toString()).toContain("unknown");
  else {
    const runtime = Bun.spawnSync([binary], { cwd: owner, stdout: "pipe", stderr: "pipe", timeout: 2_000 });
    record(owner, { command: [binary], exitCode: runtime.exitCode, stdout: runtime.stdout.toString(), stderr: runtime.stderr.toString() });
    expect(runtime.exitCode).toBe(row.runtimeExit); expect(runtime.stderr.toString()).toContain("facet.json: ");
  }
  console.log("[DEBUG] attributed callback " + row.id + " compile=" + compile.exitCode + " runtime=" + row.runtimeExit);
});

test("actual rustc executes ordinary and divergent error paths and rejects shadowed authority", async () => {
  const { inspectRustManifestPathCandidates } = await sourceHelpers();
  const owner = runOwner("native"), positive = rows.find((row) => row.native === "ordinary-and-error")!;
  const compile = (name: string, row: Row, ordinary: boolean) => {
    const directory = join(owner, name), manifestRoot = join(directory, "pkg");
    mkdirSync(manifestRoot, { recursive: true });
    if (ordinary) for (const value of row.selectedValues) {
      const path = join(directory, "foreign", value);
      mkdirSync(dirname(path), { recursive: true });
      writeFileSync(path, value, { flag: "wx" });
    }
    const input = join(directory, "../↪️rust-divergence-callback/🦀️.rs"), binary = join(directory, process.platform === "win32" ? "🔣️.exe" : "../↪️rust-divergence-callback/🔣️.json");
    writeFileSync(input, row.source + '\nfn main() { inspect(); println!("ordinary-finished"); }\n', { flag: "wx" });
    const command = ["rustc", "--edition=2021", "--crate-name", "divergence_callback", input, "-o", binary];
    const result = Bun.spawnSync(command, { cwd: directory, env: { ...process.env, CARGO_MANIFEST_DIR: manifestRoot, RUSTC_WRAPPER: "" }, stdout: "pipe", stderr: "pipe", timeout: 4_000 });
    record(owner, { command, exitCode: result.exitCode, stdout: result.stdout.toString(), stderr: result.stderr.toString() });
    return { result, binary, directory };
  };
  for (const ordinary of [true, false]) {
    const compiled = compile(ordinary ? "ordinary" : "error", positive, ordinary);
    expect(compiled.result.exitCode, compiled.result.stderr.toString()).toBe(0);
    const result = Bun.spawnSync([compiled.binary], { cwd: compiled.directory, stdout: "pipe", stderr: "pipe", timeout: 2_000 });
    record(owner, { command: [compiled.binary], exitCode: result.exitCode, stdout: result.stdout.toString(), stderr: result.stderr.toString() });
    expect(result.exitCode).toBe(ordinary ? 0 : 101);
    if (ordinary) expect(result.stdout.toString()).toBe("ordinary-finished\n");
    else { expect(result.stdout.toString()).toBe(""); expect(result.stderr.toString()).toContain("facet.json: "); }
  }
  const shadow = rows.find((row) => row.native === "nondivergent-shadow")!, compiled = compile("shadowed-panic", shadow, false);
  expect(compiled.result.exitCode, compiled.result.stderr.toString()).toBe(0);
  const runtime = Bun.spawnSync([compiled.binary], { cwd: compiled.directory, stdout: "pipe", stderr: "pipe", timeout: 2_000 });
  record(owner, { command: [compiled.binary], exitCode: runtime.exitCode, stdout: runtime.stdout.toString(), stderr: runtime.stderr.toString() });
  expect(runtime.exitCode).toBe(0);
  expect(runtime.stdout.toString()).toBe("ordinary-finished\n");
  expect(inspectRustManifestPathCandidates(shadow.source).filter((row) => shadow.selectedValues.includes(row.value))).toEqual([]);
  const rejected = compile("generic-std", rows.find((row) => row.native === "compiler-rejection")!, true);
  expect(rejected.result.exitCode).not.toBe(0);
  expect(rejected.result.stderr.toString()).toMatch(/error\[E[0-9]+\]/u);
  console.log("[DEBUG] rustc confirmed ordinary reads, actual standard panic, nondivergent shadowed panic and generic-std semantic rejection");
}, 15_000);
