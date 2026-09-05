import { afterAll, expect, test } from "bun:test";
import { createHash, randomUUID } from "node:crypto";
import { spawn, spawnSync } from "node:child_process";
import { chmodSync, closeSync, constants, fstatSync, lstatSync, mkdirSync, openSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, posix, relative, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseJson, type ParseError } from "jsonc-parser";
import ts from "typescript";

const library = resolve(import.meta.dir, "../.."), root = resolve(library, "../../../../..");
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../👀️readme-reviewed-fixture-inputs/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../👀️readme-reviewed-fixture-inputs/🧬️schema/🔣️.json"), "utf8"));
const sha = (bytes: Uint8Array | string): string => createHash("sha256").update(bytes).digest("hex");
const observations = new Map<string, { bytes: Buffer; sha256: string; size: number; mode: number }>(), outcomes: any[] = [];
let owner: string | undefined;

/** 🛡️ Rejects unsafe and opaque coordinates before observing any node. */
function safe(path: string): string {
  if (!path || path !== path.normalize("NFC") || /[\\:*?"<>|\u0000-\u001f]/u.test(path) || Buffer.from(path).toString("utf8") !== path || path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error("Unsafe reviewed fixture coordinate");
  return path;
}

/** 📚️ Reads exact regular bytes with no-follow ancestry and descriptor checks. */
function input(repo: string, path: string) {
  const parts = safe(path).split("/");
  let current = repo;
  const anchor = lstatSync(repo), ancestors = [{ path: repo, dev: anchor.dev, ino: anchor.ino }];
  if (!anchor.isDirectory() || anchor.isSymbolicLink()) throw new Error("Unsafe reviewed fixture root");
  for (const [index, part] of parts.entries()) {
    current = join(current, part);
    const node = lstatSync(current);
    if (node.isSymbolicLink() || (index < parts.length - 1 ? !node.isDirectory() : !node.isFile())) throw new Error("Nonregular reviewed fixture input");
    if (index < parts.length - 1) ancestors.push({ path: current, dev: node.dev, ino: node.ino });
  }
  const before = lstatSync(current), fd = openSync(current, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    const opened = fstatSync(fd);
    if (!opened.isFile() || opened.dev !== before.dev || opened.ino !== before.ino || opened.mode !== before.mode || opened.size !== before.size || opened.mtimeMs !== before.mtimeMs || opened.ctimeMs !== before.ctimeMs) throw new Error("Reviewed fixture input changed during open");
    const bytes = readFileSync(fd), after = fstatSync(fd), named = lstatSync(current);
    if (opened.size !== after.size || opened.mtimeMs !== after.mtimeMs || opened.ctimeMs !== after.ctimeMs || opened.mode !== after.mode || named.dev !== after.dev || named.ino !== after.ino || named.mode !== after.mode || named.size !== after.size || named.mtimeMs !== after.mtimeMs || named.ctimeMs !== after.ctimeMs || !named.isFile() || named.isSymbolicLink()) throw new Error("Reviewed fixture input changed during read");
    for (const ancestor of ancestors) {
      const node = lstatSync(ancestor.path);
      if (!node.isDirectory() || node.isSymbolicLink() || node.dev !== ancestor.dev || node.ino !== ancestor.ino) throw new Error("Reviewed fixture ancestry changed during read");
    }
    return { bytes, sha256: sha(bytes), size: bytes.length, mode: opened.mode & 0o7777 };
  } finally { closeSync(fd); }
}

/** 🔏️ Retains each observed production input identity without reading logical historical paths. */
function capture(path: string, records = observations) {
  const value = input(root, path);
  const before = records.get(path);
  if (before && (before.sha256 !== value.sha256 || before.size !== value.size || before.mode !== value.mode || !before.bytes.equals(value.bytes))) throw new Error("Repeated reviewed input drift: " + path);
  if (!before) records.set(path, { ...value, bytes: Buffer.from(value.bytes) });
  return value;
}

const manifestInput = capture(vector.fixtureAuthority), manifest = JSON.parse(manifestInput.bytes.toString("utf8"));
const manifestSchemaPath = posix.dirname(vector.fixtureAuthority) + "../👀️readme-reviewed-fixture-inputs/🧬️schema/🔣️.json";
const manifestSchema = JSON.parse(capture(manifestSchemaPath).bytes.toString("utf8"));
const catalogInput = capture(manifest.catalog.path), catalog = JSON.parse(catalogInput.bytes.toString("utf8"));
const revisionVector = JSON.parse(capture(manifest.revision.path).bytes.toString("utf8")), revision = revisionVector.revisions[manifest.revision.id];
const declaredOwner = catalog.cases[manifest.catalog.caseIndex];

/** 📁️ Allocates only exact new directories beneath one retained semantic run owner. */
function directories(base: string, path: string): string {
  let current = base;
  for (const part of safe(path).split("/")) {
    current = join(current, part);
    try {
      const node = lstatSync(current);
      if (!node.isDirectory() || node.isSymbolicLink()) throw new Error("Unsafe reviewed fixture directory");
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
      mkdirSync(current);
    }
  }
  return current;
}

/** 🔖️ Allocates a unique child beneath the declared existing ticket-owned run parent. */
function runOwner(): string {
  if (owner) return owner;
  if (vector.runParent !== ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️readme-current-plan-readiness/📓️fixture-inputs/🧾️runs") throw new Error("Unregistered reviewed fixture run parent");
  let parent = root;
  for (const part of safe(vector.runParent).split("/")) {
    parent = join(parent, part);
    const node = lstatSync(parent);
    if (!node.isDirectory() || node.isSymbolicLink()) throw new Error("Reviewed fixture parent must already exist");
  }
  owner = join(parent, "🔖️" + randomUUID());
  mkdirSync(owner);
  writeFileSync(join(owner, "📝️.md"), "# Self-Contained Reviewed Fixture Proof\n\nExact authored inputs and active or failed results are retained. No cleanup, production moves, producer execution or Git mutation occurs here.\n", { flag: "wx" });
  console.log("[DEBUG] README reviewed fixture owner", owner);
  return owner;
}

/** 🧾️ Writes only a fresh leaf beneath the validated isolated owner. */
function put(repo: string, path: string, bytes: string | Uint8Array, mode = 0o644): void {
  safe(path);
  if (dirname(path) !== ".") directories(repo, dirname(path));
  writeFileSync(join(repo, path), bytes, { flag: "wx", mode });
  chmodSync(join(repo, path), mode);
}

/** ✅️ Requires the complete declared fixture preimage, not merely parseable content. */
function validated(repo: string, row: any) {
  const value = input(repo, row.path);
  if (value.sha256 !== row.preimage.sha256 || value.size !== row.preimage.size || value.mode !== row.preimage.mode) throw new Error("Reviewed fixture preimage drift");
  return value;
}

/** 📭️ Checks an exact absent path without following a possible symbolic node. */
function absent(repo: string, path: string): boolean {
  try { lstatSync(join(repo, safe(path))); return false; }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; return true; }
}

/** 🛑️ Terminates only the freshly owned process tree, with a direct-PID fallback when its group is absent. */
function stopTree(pid: number): void {
  if (process.platform === "win32") {
    const result = spawnSync("taskkill", ["/pid", String(pid), "/T", "/F"], { stdio: "ignore", timeout: 1_000 });
    if (result.status === 0) return;
  } else {
    try { process.kill(-pid, "SIGKILL"); return; } catch {}
  }
  try { process.kill(pid, "SIGKILL"); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") throw error; }
}

/** 📭️ Proves the owned PID or process group has reached its terminal state. */
function processAbsent(pid: number, group = false): boolean {
  try { process.kill(group && process.platform !== "win32" ? -pid : pid, 0); return false; }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") throw error; return true; }
}

/** ⏱️ Captures bounded output and terminal evidence under the unchanged child ceiling. */
async function ownedRun(args: readonly string[], cwd: string, budgetMs = 4_500, maxBytes = 1024 * 1024) {
  if (!Number.isInteger(budgetMs) || budgetMs < 1 || budgetMs > 4_500 || !Number.isInteger(maxBytes) || maxBytes < 1 || maxBytes > 1024 * 1024) throw new Error("Invalid reviewed child bounds");
  const started = performance.now(), child = spawn(process.execPath, [...args], { cwd, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32" });
  const chunks: Record<"stdout" | "stderr", Buffer[]> = { stdout: [], stderr: [] };
  let size = 0, capturedBytes = 0, reason: string | null = null;
  const errors: string[] = [];
  const stop = (cause: string): void => {
    reason ??= cause;
    if (child.pid) try { stopTree(child.pid); } catch (error) { errors.push(String(error)); }
  };
  const receive = (channel: "stdout" | "stderr", chunk: Buffer): void => {
    try {
      size += chunk.length;
      if (size > maxBytes) { stop("output-limit"); return; }
      chunks[channel].push(Buffer.from(chunk));
      capturedBytes += chunk.length;
    } catch (error) { errors.push(String(error)); stop("capture-error"); }
  };
  child.stdout.on("data", (chunk) => receive("stdout", chunk));
  child.stderr.on("data", (chunk) => receive("stderr", chunk));
  child.stdout.on("error", (error) => { errors.push(String(error)); stop("capture-error"); });
  child.stderr.on("error", (error) => { errors.push(String(error)); stop("capture-error"); });
  const timer = setTimeout(() => stop("timeout"), budgetMs);
  const abort = () => stop("controller-cancelled");
  process.once("SIGINT", abort); process.once("SIGTERM", abort);
  try {
    const terminal = await new Promise<{ exitCode: number | null; signal: NodeJS.Signals | null }>((complete) => {
      child.once("error", (error) => { errors.push(String(error)); stop("spawn-error"); });
      child.once("close", (exitCode, signal) => complete({ exitCode, signal }));
    });
    if (child.pid && (!processAbsent(child.pid) || !processAbsent(child.pid, true))) stop("surviving-owned-tree");
    return { pid: child.pid ?? null, ...terminal, stdout: Buffer.concat(chunks.stdout).toString("utf8"), stderr: Buffer.concat(chunks.stderr).toString("utf8"), receivedBytes: size, capturedBytes, reason, errors, terminal: true, pidAbsent: child.pid ? processAbsent(child.pid) : true, groupAbsent: child.pid ? processAbsent(child.pid, true) : true, milliseconds: performance.now() - started };
  } finally {
    clearTimeout(timer);
    process.removeListener("SIGINT", abort); process.removeListener("SIGTERM", abort);
    if (child.pid && child.exitCode === null && child.signalCode === null) stop("controller-finalization");
  }
}

test("reviewed fixture manifest agrees with independent JSON and immutable catalog lineage", () => {
  for (const [bytes, value, grammar] of [[readFileSync(join(import.meta.dir, "../👀️readme-reviewed-fixture-inputs/🔣️.json")), vector, schema], [manifestInput.bytes, manifest, manifestSchema]] as const) {
    const errors: ParseError[] = [], validate = new Ajv({ allErrors: true }).compile(grammar);
    expect(parseJson(bytes.toString("utf8"), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(value);
    expect(errors).toEqual([]);
    expect(validate(value), JSON.stringify(validate.errors)).toBe(true);
  }
  expect(manifestInput.sha256).toBe(vector.fixtureAuthoritySha256);
  expect(catalogInput.sha256).toBe(manifest.catalog.sha256);
  expect(declaredOwner.preimage).toEqual({ ...manifest.provenance.baselinePreimage, mode: "0644" });
  expect({ commit: revision.baselineCommit, blob: revision.baselineBlob }).toEqual({ commit: manifest.provenance.baselineCommit, blob: manifest.provenance.baselineBlob });
  expect(revision.sourcePath).toBe(declaredOwner.sourcePath);
  const execute = (args: string[]) => {
    const result = Bun.spawnSync(["git", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    return Buffer.from(result.stdout);
  };
  expect(execute(["rev-parse", manifest.provenance.baselineCommit + ":" + safe(declaredOwner.sourcePath)]).toString().trim()).toBe(manifest.provenance.baselineBlob);
  const bytes = execute(["cat-file", "blob", manifest.provenance.baselineBlob]);
  expect({ sha256: sha(bytes), size: bytes.length }).toEqual({ sha256: manifest.provenance.baselinePreimage.sha256, size: manifest.provenance.baselinePreimage.size });
  expect(new Bun.CryptoHasher("sha1").update(Buffer.concat([Buffer.from("blob " + bytes.length + "\0"), bytes])).digest("hex")).toBe(manifest.provenance.baselineBlob);
});

test("new permanent reviewed fixture directories use current registered canonical kinds", async () => {
  const taxonomy = JSON.parse(capture("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json").bytes.toString("utf8"));
  const discovery = await import("../../🔍️discovery/🟦️.ts"), normalization = await import("../../🧹️normalization/🟦️.ts");
  for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts"]) capture(path);
  const cases = [...vector.directoryCases, ...manifest.inputs.map((row: any) => ({ path: posix.dirname(row.path), kind: "fixture-case", parent: "fixtures" }))];
  for (const row of cases) expect(discovery.semanticDirectoryKindId(posix.basename(row.path), taxonomy, { parentKindId: row.parent }), row.path).toBe(row.kind);
  const inventory = normalization.inventoryTaxonomy({ repoRoot: root, scope: relative(root, import.meta.dir).replaceAll("\\", "/"), workers: 1 });
  for (const row of cases) {
    const entry = inventory.entries.find((entry) => entry.sourcePath === row.path);
    expect(entry, row.path).toBeDefined();
    expect(entry!.normalizedPath, row.path).toBe(row.path);
    expect(inventory.violations.filter((problem) => problem.path === row.path && problem.severity === "error"), row.path).toEqual([]);
  }
});

test("reviewed fixture inputs retain exact approved source and expectation bytes", () => {
  for (const row of manifest.inputs) {
    const value = validated(root, row);
    capture(row.path);
    expect(new Bun.CryptoHasher("sha256").update(value.bytes).digest("hex")).toBe(row.preimage.sha256);
    expect(Buffer.from(value.bytes.toString("utf8"), "utf8")).toEqual(value.bytes);
    if (row.role === "source") expect({ ...row.preimage, mode: "0644" }).toEqual(revision.currentPreimage);
    else {
      expect(row.preimage.sha256).toBe(revision.expectationsSha256);
      const errors: ParseError[] = [], document = parseJson(value.bytes.toString("utf8"), errors, { disallowComments: true, allowTrailingComma: false });
      expect(errors).toEqual([]);
      expect(document).toEqual(JSON.parse(value.bytes.toString("utf8")));
      expect(document.documents.readme).toBe(declaredOwner.sourcePath);
    }
  }
});

test("reviewed fixture input identity rejects changed missing and symbolic nodes", () => {
  const row = manifest.inputs.find((entry: any) => entry.role === "source"), original = validated(root, row);
  for (const scenario of vector.inputCases) {
    const repo = directories(runOwner(), "🧪️" + scenario.id), local = { ...row, path: "🧫️fixtures/📃️source/📝️.md" };
    if (scenario.state === "symlink" || scenario.state === "parent-symlink") {
      put(repo, "🧫️fixtures/📃️target/📝️.md", original.bytes);
      const link = scenario.state === "symlink" ? local.path : posix.dirname(local.path), target = scenario.state === "symlink" ? "🧫️fixtures/📃️target/📝️.md" : "🧫️fixtures/📃️target";
      directories(repo, posix.dirname(link));
      symlinkSync(posix.relative(posix.dirname(link), target), join(repo, link), scenario.state === "symlink" ? "file" : "dir");
    } else if (scenario.state !== "missing") put(repo, local.path, scenario.state === "changed" ? Buffer.concat([original.bytes, Buffer.from("\n")]) : original.bytes, scenario.state === "mode" ? 0o600 : 0o644);
    if (scenario.accepted) expect(validated(repo, local).sha256, scenario.id).toBe(row.preimage.sha256);
    else expect(() => validated(repo, local), scenario.id).toThrow(/Reviewed fixture preimage drift|Nonregular reviewed fixture input|ENOENT/u);
    outcomes.push({ id: scenario.id, accepted: scenario.accepted });
  }
});

test("repeated reviewed input capture rejects a deliberate same-path change", () => {
  const repo = directories(runOwner(), "🧪️repeated-capture"), path = "🧫️fixtures/📝️.md";
  put(repo, path, vector.repeatedCapture.first);
  const physical = relative(root, join(repo, path)).replaceAll("\\", "/"), records = new Map();
  const first = capture(physical, records);
  writeFileSync(join(repo, path), vector.repeatedCapture.changed);
  expect(() => capture(physical, records)).toThrow(/Repeated reviewed input drift/u);
  expect(records.get(physical)).toMatchObject({ sha256: first.sha256, size: first.size, mode: first.mode });
});

test("owned reviewed child timeout and output-limit terminate their descendants", async () => {
  for (const row of vector.childSafety.probes) {
    const repo = directories(runOwner(), "🧪️child-" + row.id);
    const code = 'const {spawn}=require("node:child_process");const child=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"inherit"});console.log(JSON.stringify({descendant:child.pid}));' + (row.reason === "output-limit" ? 'setTimeout(()=>process.stdout.write("x".repeat(8192)),50);' : '') + 'setInterval(()=>{},1000);';
    const result = await ownedRun(["-e", code], repo, row.budgetMs, row.maxBytes);
    put(repo, "📊️outcome/🔣️.json", JSON.stringify(result, null, 2) + "\n");
    const descendant = JSON.parse(result.stdout.split("\n")[0]).descendant;
    expect(result.reason).toBe(row.reason);
    expect(result.errors).toEqual([]);
    expect({ terminal: result.terminal, pidAbsent: result.pidAbsent, groupAbsent: result.groupAbsent }).toEqual({ terminal: true, pidAbsent: true, groupAbsent: true });
    expect(processAbsent(descendant)).toBe(true);
    outcomes.push({ id: row.id, ...result, descendant, descendantAbsent: true });
  }
  const row = vector.childSafety.utf8Probe, repo = directories(runOwner(), "🧪️child-utf8");
  const code = 'process.stdout.write(Buffer.from(' + JSON.stringify(row.segmentsHex[0]) + ',"hex"));setTimeout(()=>process.stdout.write(Buffer.from(' + JSON.stringify(row.segmentsHex[1]) + ',"hex")),50);';
  const result = await ownedRun(["-e", code], repo, row.budgetMs, row.maxBytes);
  put(repo, "📊️outcome/🔣️.json", JSON.stringify(result, null, 2) + "\n");
  expect({ exitCode: result.exitCode, stdout: result.stdout, reason: result.reason }).toEqual({ exitCode: 0, stdout: row.expected, reason: null });
  expect({ pidAbsent: result.pidAbsent, groupAbsent: result.groupAbsent }).toEqual({ pidAbsent: true, groupAbsent: true });
  outcomes.push({ id: "utf8-chunk-preservation", ...result });
});

/** 🧪️ Captures unchanged gate inputs into a repository with no live historical source or expectation. */
function isolatedRepository(id: string) {
  const repo = directories(runOwner(), "🧪️" + id + "/🧪️repository"), paths = [...vector.copies, vector.fixtureAuthority, manifestSchemaPath, manifest.catalog.path, ...manifest.inputs.map((row: any) => row.path)];
  expect(new Set(paths).size).toBe(paths.length);
  for (const path of paths) { const value = capture(path); put(repo, path, value.bytes, value.mode); }
  put(repo, declaredOwner.destinationPath, vector.isolation.canonicalBytes);
  expect(absent(repo, declaredOwner.sourcePath)).toBe(true);
  expect(absent(repo, revision.expectationsPath)).toBe(true);
  const canonical = input(repo, declaredOwner.destinationPath);
  expect(canonical.sha256).not.toBe(revision.currentPreimage.sha256);
  return { repo, canonical };
}

test("the unchanged copied revision gate runs with absent raw source and edited canonical source", async () => {
  const { repo, canonical } = isolatedRepository("independent-revision"), copiedTest = input(repo, vector.isolation.gateSource);
  expect(copiedTest.sha256).toBe(input(root, vector.isolation.gateSource).sha256);
  try {
    const execution = await ownedRun(["test", join(repo, vector.isolation.gateSource)], repo, vector.childSafety.budgetMs, vector.childSafety.maxBytes);
    const result = { id: "absent-raw-edited-canonical", ...execution, copiedTestSha256: copiedTest.sha256, canonicalSha256: canonical.sha256 };
    put(runOwner(), "📊️execution/🔣️.json", JSON.stringify(result, null, 2) + "\n");
    outcomes.push(result);
    console.log("[DEBUG] README isolated revision result", JSON.stringify({ exitCode: result.exitCode, reason: result.reason, milliseconds: result.milliseconds }));
    expect({ reason: result.reason, errors: result.errors, pidAbsent: result.pidAbsent, groupAbsent: result.groupAbsent }).toEqual({ reason: null, errors: [], pidAbsent: true, groupAbsent: true });
    expect(result.exitCode, result.stdout + result.stderr).toBe(0);
    expect(result.stdout + result.stderr).toMatch(new RegExp("\\b" + vector.isolation.expectedPasses + " pass\\b", "u"));
    expect(result.stdout + result.stderr).toMatch(new RegExp("\\b" + vector.isolation.expectedFailures + " fail\\b", "u"));
  } finally {
    expect(absent(repo, declaredOwner.sourcePath)).toBe(true);
    expect(absent(repo, revision.expectationsPath)).toBe(true);
    expect(input(repo, declaredOwner.destinationPath).sha256).toBe(canonical.sha256);
  }
});

test("the unchanged activation module validates a schema snapshot without live historical inputs", async () => {
  const { repo, canonical } = isolatedRepository("independent-activation"), source = vector.isolation.activationGateSource;
  expect(input(repo, source).sha256).toBe(input(root, source).sha256);
  const result = await ownedRun(["test", join(repo, source), "-t", vector.isolation.activationTestPattern], repo, vector.childSafety.budgetMs, vector.childSafety.maxBytes);
  put(runOwner(), "📊️activation-execution/🔣️.json", JSON.stringify(result, null, 2) + "\n");
  outcomes.push({ id: "activation-without-live-historical-inputs", ...result });
  expect({ exitCode: result.exitCode, reason: result.reason, errors: result.errors, pidAbsent: result.pidAbsent, groupAbsent: result.groupAbsent }, result.stdout + result.stderr).toEqual({ exitCode: 0, reason: null, errors: [], pidAbsent: true, groupAbsent: true });
  expect(result.stdout + result.stderr).toMatch(/\b1 pass\b/u);
  expect(result.stdout + result.stderr).toMatch(/\b0 fail\b/u);
  expect(absent(repo, declaredOwner.sourcePath)).toBe(true);
  expect(absent(repo, revision.expectationsPath)).toBe(true);
  expect(input(repo, declaredOwner.destinationPath).sha256).toBe(canonical.sha256);
});

test("reviewed fixture gate registration matches its package route and both launch catalogs", () => {
  const expected = vector.execution, packagePath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript";
  const parsed = (path: string, jsonc = false): any => {
    const bytes = capture(path).bytes, errors: ParseError[] = [], value = parseJson(bytes.toString("utf8"), errors, { disallowComments: !jsonc, allowTrailingComma: jsonc });
    expect(errors, path).toEqual([]);
    if (!jsonc) expect(value, path).toEqual(JSON.parse(bytes.toString("utf8")));
    return value;
  };
  const project = parsed(packagePath + "/📋️project.json"), packageManifest = parsed(packagePath + "/package.json");
  const router = capture(packagePath + "/📜️script.ts").bytes.toString("utf8"), tree = ts.createSourceFile("router.ts", router, ts.ScriptTarget.Latest, true), branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node) && ts.isBinaryExpression(node.expression) && node.expression.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken && node.expression.left.getText(tree) === "segments[0]" && ts.isStringLiteral(node.expression.right) && node.expression.right.text === expected.route) branches.push(node);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  const launches = [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"].map((path) => {
    const configurations = parsed(path, true).configurations;
    return { path, rows: configurations.filter((row: any) => row.name === expected.launchName), orderRows: configurations.filter((row: any) => row.presentation?.group === expected.launchGroup && row.presentation?.order === expected.launchOrder).length };
  });
  expect({ packageName: packageManifest.name, packageCommand: packageManifest.scripts?.[expected.target], target: project.targets[expected.target], branches: branches.length, launches }).toEqual({ packageName: expected.packageName, packageCommand: expected.packageCommand, target: { executor: "nx:run-commands", options: { cwd: packagePath, command: expected.command } }, branches: 1, launches: launches.map(({ path }) => ({ path, rows: [{ name: expected.launchName, type: "node-terminal", request: "launch", command: expected.launchCommand, cwd: "${workspaceFolder}", presentation: { group: expected.launchGroup, order: expected.launchOrder } }], orderRows: 1 })) });
  expect(branches[0]!.thenStatement.getText(tree)).toContain("join(this.repoRoot, " + JSON.stringify(expected.source) + ")");
  expect(branches[0]!.thenStatement.getText(tree)).toContain('await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });');
});

afterAll(() => {
  const identities = [...observations].map(([path, before]) => {
    const value = input(root, path), after = { sha256: value.sha256, size: value.size, mode: value.mode };
    expect(value.bytes, path).toEqual(before.bytes);
    return { path, before: { sha256: before.sha256, size: before.size, mode: before.mode }, after };
  });
  if (owner) put(owner, "📊️summary/🔣️.json", JSON.stringify({ schemaVersion: 1, contract: vector.contract, outcomes, identities }, null, 2) + "\n");
  for (const row of identities) expect(row.after, row.path).toEqual(row.before);
});
