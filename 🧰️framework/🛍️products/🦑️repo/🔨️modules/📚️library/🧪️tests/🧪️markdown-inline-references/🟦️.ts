import { afterAll, expect, test } from "bun:test";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, constants, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, writeFileSync, type Stats } from "node:fs";
import { createRequire } from "node:module";
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { parse, type ParseError } from "jsonc-parser";
import ts from "typescript";

type Token = Readonly<{ adapter: "markdown"; structuredLocation: string; start: number; end: number; value: string }>;
type Scenario = Readonly<{ id: string; path: string; source: string; oracleScope: "common-subset" | "scanner-only"; reason: string; expected: readonly Token[] }>;
type Stress = Readonly<{ id: string; unit: string; repeat: number; suffix: string; codeUnits: number; bytes: number; expected: readonly Token[] }>;
type Vector = Readonly<{ extraction: { required: readonly string[]; optional: readonly string[]; globals: readonly string[] }; oracle: { package: string; version: string; dependencyKind: "devDependencies" }; limits: { childMilliseconds: number; childOutputBytes: number; caseMilliseconds: number; childDescendants: number }; cases: readonly Scenario[]; cacheSequence: readonly string[]; stress: readonly Stress[] }>;
type Operation = (path: string, content: string) => readonly Token[];
type NativeToken = { type: string; attrGet(name: string): string | null; children?: NativeToken[] | null };
type Destination = Readonly<{ ok: boolean; str: string; pos: number }>;
type NativeHelpers = { parseLinkDestination(source: string, start: number, max: number): Destination; [name: string]: unknown };
type NativeParser = { helpers: NativeHelpers; inline: { parse(source: string, parser: NativeParser, environment: object, tokens: NativeToken[]): void }; normalizeLink(value: string): string };
type NativeSpan = Readonly<{ value: string; start: number; end: number; raw: string }>;
const library = resolve(import.meta.dir, "../.."), root = resolve(library, "../../../../..");
const oracleRequire = createRequire(import.meta.url), MarkdownIt = oracleRequire("markdown-it") as new () => NativeParser;
const normalizerPath = join(library, "🧹️normalization/🟦️.ts");
const vectorPath = join(import.meta.dir, "🔣️.json"), schemaPath = join(import.meta.dir, "🧬️schema/🔣️.json");
const packagePath = join(library, "📦️packages/🟦️typescript/package.json"), oraclePath = fileURLToPath(import.meta.resolve("markdown-it/package.json")), oracleEntryPath = oracleRequire.resolve("markdown-it");
const vectorBytes = snapshot(vectorPath), vector: Vector = JSON.parse(vectorBytes.toString("utf8"));
const inputBytes = new Map([vectorPath, schemaPath, join(import.meta.dir, "🟦️.ts"), packagePath, oraclePath, oracleEntryPath, ...["index.mjs", "parser_inline.mjs", "helpers/parse_link_destination.mjs", "rules_inline/link.mjs", "rules_inline/image.mjs", "common/utils.mjs"].map((path) => join(dirname(oraclePath), "lib", path))].map((path) => [path, snapshot(path)]));
const sha = (bytes: string | Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const compilers = [
  { id: "bun", compile: (code: string): string => new Bun.Transpiler({ loader: "ts" }).transformSync(code) },
  { id: "typescript", compile: (code: string): string => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext }, reportDiagnostics: true }).outputText },
];

/** 🛡️ Rechecks named workspace ancestors without following a symlink or entering an opaque path. */
function ancestry(path: string): string {
  const local = relative(root, path), normalized = local.split(sep).join("/");
  if (resolve(path) !== path || !local || isAbsolute(local) || normalized === ".." || normalized.startsWith("../") || ["compose", "temp/compose"].some((prefix) => normalized === prefix || normalized.startsWith(prefix + "/"))) throw new Error("Unsafe named input: " + path);
  const names = relative(root, dirname(path)).split(sep).filter(Boolean), identities: unknown[] = [];
  let current = root;
  for (const name of ["", ...names]) {
    current = name ? join(current, name) : current;
    const stat = lstatSync(current);
    if (stat.isSymbolicLink() || !stat.isDirectory()) throw new Error("Unsafe input ancestor: " + current);
    identities.push([current, stat.dev, stat.ino, stat.mode]);
  }
  return JSON.stringify(identities);
}

function fileIdentity(stat: Stats): string {
  return JSON.stringify([stat.dev, stat.ino, stat.mode, stat.size, stat.mtimeMs, stat.ctimeMs]);
}

/** 📷️ Reads an exact regular input through a checked descriptor and revalidates its ancestry and identity. */
function snapshot(path: string): Buffer {
  const parents = ancestry(path), before = lstatSync(path);
  if (!before.isFile() || before.isSymbolicLink() || ancestry(path) !== parents) throw new Error("Unsafe named input: " + path);
  const descriptor = openSync(path, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    const opened = fstatSync(descriptor);
    if (!opened.isFile() || fileIdentity(opened) !== fileIdentity(before) || ancestry(path) !== parents) throw new Error("Input changed before read: " + path);
    const bytes = readFileSync(descriptor), after = fstatSync(descriptor);
    if (ancestry(path) !== parents || fileIdentity(after) !== fileIdentity(before) || fileIdentity(lstatSync(path)) !== fileIdentity(before) || bytes.length !== before.size) throw new Error("Input changed during read: " + path);
    return bytes;
  } finally { closeSync(descriptor); }
}

/** 📝️ Creates only a new run leaf, with fresh no-follow parent checks before writing. */
function writeNew(path: string, content: string): void {
  const parents = ancestry(path), descriptor = openSync(path, constants.O_WRONLY | constants.O_CREAT | constants.O_EXCL | (constants.O_NOFOLLOW ?? 0), 0o600);
  try {
    if (!fstatSync(descriptor).isFile() || ancestry(path) !== parents) throw new Error("Unsafe output parent: " + path);
    writeFileSync(descriptor, content);
    if (ancestry(path) !== parents) throw new Error("Output parent changed: " + path);
  } finally { closeSync(descriptor); }
  if (!snapshot(path).equals(Buffer.from(content))) throw new Error("Output changed after write: " + path);
}

function declarations(content: string): Readonly<Record<string, string | null>> {
  const source = ts.createSourceFile(normalizerPath, content, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), rows: Record<string, string | null> = {};
  for (const name of [...vector.extraction.required, ...vector.extraction.optional]) {
    const matches = source.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name);
    expect(matches.length, name).toBe(vector.extraction.required.includes(name) ? 1 : Math.min(1, matches.length));
    rows[name] = matches[0]?.getText(source) ?? null;
  }
  for (const name of vector.extraction.globals) {
    const matches = source.statements.filter((node): node is ts.VariableStatement => ts.isVariableStatement(node) && node.declarationList.declarations.some((row) => ts.isIdentifier(row.name) && row.name.text === name));
    expect(matches, name).toHaveLength(1);
    expect(matches[0]!.declarationList.declarations).toHaveLength(1);
    rows[name] = matches[0]!.getText(source);
  }
  return rows;
}

const bodies = declarations(snapshot(normalizerPath).toString("utf8"));
const closure = [...vector.extraction.globals, ...vector.extraction.required, ...vector.extraction.optional].map((name) => bodies[name]).filter((body): body is string => body !== null).join("\n");
const compiled = (compiler: typeof compilers[number]): Operation => new Function("basename", compiler.compile(closure) + "\nreturn referenceTokens;")(basename) as Operation;

/** 🧭️ Counts authored UTF-16 positions independently from the production line-index cache. */
function location(source: string, start: number, label = "markdown-link"): string {
  const lines = source.slice(0, start).split("\n");
  return label + ":" + lines.length + ":" + (lines.at(-1)!.length + 1) + "@" + start;
}

/** 🔬️ Joins only native published destinations with unambiguous distinct raw helper spans. */
function native(content: string) {
  const parser = new MarkdownIt() as unknown as NativeParser, originalHelpers = parser.helpers, original = originalHelpers.parseLinkDestination;
  const other = new MarkdownIt() as unknown as NativeParser, otherBinding = other.helpers.parseLinkDestination, observations: NativeSpan[] = [], tokens: NativeToken[] = [];
  expect(other.helpers).not.toBe(originalHelpers);
  parser.helpers = { ...originalHelpers, parseLinkDestination(source, start, max) {
    const result = original(source, start, max);
    if (result.ok && source === content) observations.push({ value: result.str, start, end: result.pos, raw: source.slice(start, result.pos) });
    return result;
  } };
  try { parser.inline.parse(content, parser, {}, tokens); }
  finally { parser.helpers = originalHelpers; }
  expect(parser.helpers).toBe(originalHelpers);
  expect(parser.helpers.parseLinkDestination).toBe(original);
  expect(other.helpers.parseLinkDestination).toBe(otherBinding);
  const published: string[] = [];
  const visit = (entries: readonly NativeToken[]): void => {
    for (const row of entries) {
      if (row.type === "link_open" || row.type === "image") {
        const value = row.attrGet(row.type === "image" ? "src" : "href");
        expect(value).not.toBeNull();
        published.push(value!);
      }
      if (row.children && row.type !== "image") visit(row.children);
    }
  };
  visit(tokens);
  const normalize = (value: string): string => parser.normalizeLink(value);
  const joined = (): Token[] => {
    const candidates = [...new Map(observations.filter((row) => published.includes(normalize(row.value))).map((row) => [row.start + "\0" + row.end + "\0" + row.raw, row])).values()].sort((a, b) => a.start - b.start);
    if (candidates.length !== published.length || candidates.some((row, index) => normalize(row.value) !== published[index])) throw new Error("Ambiguous native destination span join");
    return candidates.map((row) => ({ adapter: "markdown", structuredLocation: location(content, row.start), start: row.start, end: row.end, value: row.raw }));
  };
  return { published, observations, normalize, joined };
}

/** 🔒️ Rejects process/import escape capabilities in the exact extracted production declarations. */
function assertPureClosure(): void {
  const source = ts.createSourceFile("closure.ts", closure, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), forbidden: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) || ts.isImportEqualsDeclaration(node) || ts.isExportDeclaration(node) || node.kind === ts.SyntaxKind.ImportKeyword) forbidden.push(node.getText(source));
    if (ts.isIdentifier(node) && ["process", "Bun", "globalThis", "require", "eval", "Function", "constructor"].includes(node.text)) forbidden.push(node.text);
    ts.forEachChild(node, visit);
  };
  visit(source);
  expect(forbidden).toEqual([]);
  expect(vector.limits.childDescendants).toBe(0);
}

const driver = [
  'import { basename } from "node:path";',
  "const payload = await Bun.stdin.json();",
  'const operation = new Function("basename", payload.code + "\\nreturn referenceTokens;")(basename);',
  "const started = performance.now();",
  "const rows = payload.inputs.map((row) => operation(row.path, row.source));",
  "console.log(JSON.stringify({ pid: process.pid, elapsedMs: performance.now() - started, rows }));",
].join("\n");

/** ⏱️ Gives one pure compiler child one monotonic deadline and one combined stdout/stderr cap. */
async function stress(compiler: typeof compilers[number]) {
  assertPureClosure();
  const owner = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️markdown-inline-reference-tests");
  snapshot(join(owner, "📝️.md"));
  const runs = join(owner, "🧾️runs"), parent = ancestry(runs);
  try { lstatSync(runs); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; mkdirSync(runs); }
  if (ancestry(runs) !== parent) throw new Error("Ticket owner changed before run");
  const parents = ancestry(join(runs, "📜️script.ts")), run = mkdtempSync(join(runs, "🔖️")), driverPath = join(run, "📜️script.ts"), code = compiler.compile(closure);
  if (ancestry(join(runs, "📜️script.ts")) !== parents) throw new Error("Run container changed during allocation");
  const inputs = vector.stress.map((row) => ({ path: "📝️.md", source: row.unit.repeat(row.repeat) + row.suffix }));
  writeNew(driverPath, driver);
  writeNew(join(run, "🔣️.json"), JSON.stringify({ compiler: compiler.id, declarationSha256: sha(closure), sourceDeclarations: closure, limits: vector.limits, stress: vector.stress }, null, 2) + "\n");
  const started = performance.now(), child = spawn(process.execPath, [driverPath], { cwd: run, stdio: ["pipe", "pipe", "pipe"] });
  let bytes = 0, timedOut = false, overflow = false, spawnError: string | null = null;
  const stdout: Buffer[] = [], stderr: Buffer[] = [];
  const collect = (chunks: Buffer[], chunk: Buffer): void => {
    const remaining = Math.max(0, vector.limits.childOutputBytes - bytes);
    bytes += chunk.length;
    if (remaining) chunks.push(chunk.subarray(0, remaining));
    if (bytes > vector.limits.childOutputBytes) { overflow = true; child.kill("SIGKILL"); }
  };
  child.stdout.on("data", (chunk: Buffer) => collect(stdout, chunk));
  child.stderr.on("data", (chunk: Buffer) => collect(stderr, chunk));
  child.stdin.on("error", (error: NodeJS.ErrnoException) => { if (error.code !== "EPIPE") spawnError = error.message; });
  const timeout = setTimeout(() => { timedOut = true; child.kill("SIGKILL"); }, vector.limits.childMilliseconds);
  const terminal = await new Promise<{ code: number | null; signal: string | null }>((done) => {
    child.once("error", (error) => { spawnError = error.message; });
    child.once("close", (code, signal) => { clearTimeout(timeout); done({ code, signal }); });
    child.stdin.end(JSON.stringify({ code, inputs }));
  });
  const elapsedMs = performance.now() - started, out = Buffer.concat(stdout).toString("utf8"), err = Buffer.concat(stderr).toString("utf8");
  let alive = false;
  if (child.pid !== undefined) try { process.kill(child.pid, 0); alive = true; } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") throw error; }
  const outcome = { compiler: compiler.id, declarationSha256: sha(closure), compiledSha256: sha(code), driverSha256: sha(driver), pid: child.pid, ...terminal, timedOut, overflow, spawnError, bytes, elapsedMs, alive, stdout: out, stderr: err };
  const fence = String.fromCharCode(96).repeat(3);
  writeNew(join(run, "📝️.md"), "# Markdown Inline Compiler Child\n\nNew run, not reconstructed historical evidence. The driver imports only node:path and the extracted production closure has no process/import escape capability. No child descendants are created by this closed execution path.\n\n" + fence + "json\n" + JSON.stringify(outcome, null, 2) + "\n" + fence + "\n");
  console.info("[DEBUG] Markdown inline bounded child " + JSON.stringify({ ...outcome, stdout: undefined, stderr: undefined, report: join(run, "📝️.md") }));
  expect(alive).toBe(false);
  expect({ timedOut, overflow, spawnError, code: terminal.code, signal: terminal.signal }, err).toEqual({ timedOut: false, overflow: false, spawnError: null, code: 0, signal: null });
  expect(bytes).toBeLessThanOrEqual(vector.limits.childOutputBytes);
  const result = JSON.parse(out);
  expect(result.pid).toBe(child.pid);
  expect(result.rows).toEqual(vector.stress.map((row) => row.expected));
  expect(result.elapsedMs).toBeLessThan(vector.limits.childMilliseconds);
}

test("Markdown inline references have closed neutral vectors and exact UTF-16 spans", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(inputBytes.get(schemaPath)!.toString("utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  const bad = [
    { ...vector, extra: true }, { ...vector, schemaVersion: 2 },
    { ...vector, limits: { ...vector.limits, childMilliseconds: 2001 } },
    { ...vector, limits: { ...vector.limits, childOutputBytes: 65537 } },
    { ...vector, extraction: { ...vector.extraction, optional: ["missingFallback"] } },
    { ...vector, cases: vector.cases.map((row, index) => index ? row : { ...row, extra: true }) },
    { ...vector, stress: vector.stress.map((row, index) => index ? row : { ...row, repeat: row.repeat - 1 }) },
  ];
  for (const row of bad) expect(validate(row)).toBe(false);
  const errors: ParseError[] = [];
  expect(parse(vectorBytes.toString("utf8"), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(errors).toEqual([]);
  expect(new Set(vector.cases.map((row) => row.id)).size).toBe(vector.cases.length);
  for (const row of vector.cases) for (const token of row.expected) {
    expect(row.source.slice(token.start, token.end), row.id).toBe(token.value);
    expect(token.end, row.id).toBe(token.start + token.value.length);
    expect(token.structuredLocation, row.id).toBe(location(row.source, token.start, token.structuredLocation.startsWith("html-") ? "html-attribute" : "markdown-link"));
  }
  for (const row of vector.stress) {
    const content = row.unit.repeat(row.repeat) + row.suffix;
    expect(content.length, row.id).toBe(row.codeUnits);
    expect(Buffer.byteLength(content), row.id).toBe(row.bytes);
    for (const token of row.expected) {
      expect(content.slice(token.start, token.end), row.id).toBe(token.value);
      expect(token.structuredLocation, row.id).toBe(location(content, token.start));
    }
  }
});

for (const compiler of compilers) test(compiler.id + " actual Markdown extraction preserves all authored tokens and intentional destination offsets", () => {
  const operation = compiled(compiler);
  for (const row of vector.cases) expect(operation(row.path, row.source), row.id).toEqual(row.expected);
});

test("both real compiler closures invalidate line caches and leave unused adapters unimplemented", () => {
  assertPureClosure();
  for (const compiler of compilers) {
    const operation = compiled(compiler);
    for (const id of vector.cacheSequence) {
      const row = vector.cases.find((candidate) => candidate.id === id);
      expect(row, id).toBeDefined();
      expect(operation(row!.path, row!.source), compiler.id + ":" + id).toEqual(row!.expected);
    }
    for (const path of ["source.rs", "source.json", "source.ts"]) expect(() => operation(path, "unused adapter must not be stubbed"), path).toThrow(ReferenceError);
  }
});

test("independent markdown-it published tokens prove only the declared common subset and reject ambiguous spans", () => {
  for (const row of vector.cases.filter((candidate) => candidate.oracleScope === "common-subset")) {
    const actual = native(row.source);
    expect(actual.published, row.id).toEqual(row.expected.map((token) => actual.normalize(token.value)));
    expect(actual.joined(), row.id).toEqual(row.expected);
  }
  const row = vector.cases.find((candidate) => candidate.id === "speculative-equal-destination")!;
  const ambiguous = native(row.source);
  expect(ambiguous.published).toEqual(row.expected.map((token) => ambiguous.normalize(token.value)));
  expect(new Set(ambiguous.observations.filter((span) => span.value === row.expected[0]!.value).map((span) => span.start)).size).toBeGreaterThan(1);
  expect(() => ambiguous.joined()).toThrow("Ambiguous native destination span join");
});

test("Markdown parser oracle is an exact direct test-only dependency", () => {
  const packageJson = JSON.parse(inputBytes.get(packagePath)!.toString("utf8"));
  const installed = JSON.parse(inputBytes.get(oraclePath)!.toString("utf8"));
  expect(installed.version).toBe(vector.oracle.version);
  expect(packageJson[vector.oracle.dependencyKind]?.[vector.oracle.package]).toBe(vector.oracle.version);
  expect(packageJson.dependencies?.[vector.oracle.package]).toBeUndefined();
});

for (const compiler of compilers) test(compiler.id + " failed-label and failed-title scans finish inside one bounded pure child", () => stress(compiler));

afterAll(() => {
  expect(declarations(snapshot(normalizerPath).toString("utf8"))).toEqual(bodies);
  for (const [path, bytes] of inputBytes) expect(snapshot(path), path + " changed during the packet").toEqual(bytes);
  console.info("[DEBUG] Markdown inline declaration closure " + JSON.stringify({ sha256: sha(closure), declarations: Object.fromEntries(Object.entries(bodies).map(([name, body]) => [name, body === null ? null : sha(body)])), inputSha256: Object.fromEntries([...inputBytes].map(([path, bytes]) => [path, sha(bytes)])) }));
});
