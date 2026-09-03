import { afterAll, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, posix, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import { dirname as oracleDirname } from "pathe";
import ts from "typescript";

const owner = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️reference-coordinate-progress";
const root = resolve(import.meta.dir, ...owner.split("/").map(() => ".."));
const library = owner.slice(0, owner.lastIndexOf("/🧪️tests"));
const packageRoot = library + "/📦️packages/🟦️typescript";
const sourcePath = library + "/🧹️normalization/🟦️.ts";
const observations = new Map<string, Buffer>();

/** 🛡️ Reads only declared nonopaque inputs beneath no-follow ancestors. */
function input(path: string): Buffer {
  if (path !== posix.normalize(path) || isAbsolute(path) || path.includes("\\") || path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error("Unsafe test input");
  let current = root;
  const workspace = lstatSync(current);
  if (!workspace.isDirectory() || workspace.isSymbolicLink()) throw new Error("Unsafe test workspace");
  const parts = path.split("/");
  parts.forEach((part, index) => {
    current = join(current, part);
    const node = lstatSync(current);
    if (node.isSymbolicLink() || (index === parts.length - 1 ? !node.isFile() : !node.isDirectory())) throw new Error("Unsafe test input ancestry");
  });
  const bytes = readFileSync(current), prior = observations.get(path);
  if (prior && !prior.equals(bytes)) throw new Error("Test input drift: " + path);
  if (!prior) observations.set(path, bytes);
  return bytes;
}

if (join(root, owner) !== import.meta.dir) throw new Error("Wrong test owner");
input(owner + "../🧪️🍊️reference-coordinate-progress/🟦️.ts");
const vector = JSON.parse(input(owner + "../🧪️🍊️reference-coordinate-progress/🔣️.json").toString("utf8"));
const grammar = JSON.parse(input(owner + "../🧪️🍊️reference-coordinate-progress/🧬️schema/🔣️.json").toString("utf8"));
const source = input(sourcePath).toString("utf8");
input(library + "/🔍️discovery/🟦️.ts");
input(library + "/🔣️taxonomy.json");
const syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const compilers = [
  { name: "Bun", compile: (text: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(text) },
  { name: "TypeScript", compile: (text: string) => ts.transpileModule(text, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];
type Progress = { operation: string; phase: string; current: number; total: number; path?: string };
type VirtualNode = { kind: "file" | "directory" | "symlink"; content?: string; gitRoot?: boolean };
type Case = {
  id: string; operation: "plan" | "apply"; paths: string[]; nodes: Record<string, VirtualNode>;
  cancel?: { when: "before-start" | "pending" | "empty" | "observer"; path?: string };
  expected: { directories: string[]; visited: string[]; roots: string[] | null; markerProbes: string[]; observers: string[]; error: string | null };
};

/** 🧬️ Extracts one unchanged production declaration through the independent TypeScript AST. */
function declaration(name: string): ts.Statement {
  const found = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? node.name?.text === name : ts.isVariableStatement(node) && node.declarationList.declarations.some((item) => item.name.getText(syntax) === name));
  if (found.length !== 1) throw new Error("Missing production declaration: " + name);
  return found[0]!;
}

const names = ["LEXICAL_OPAQUE_ROOTS", "sourceRelative", "normalizeRelative", "assertNoFollowAncestors", "assertLexicalInputOutsideOpaque", "isExcluded", "lstatOrNull", "generatorPathCompare", "report", "ancestorReferenceCoordinateRoot", "referenceCoordinateRoots"];
const extracted = names.map((name) => declaration(name).getText(syntax).replace(/^export /u, "")).join("\n");
const compiled = compilers.map((compiler) => ({ name: compiler.name, code: compiler.compile(extracted) }));

/** 🔬️ Uses independent path parsing and explicit neutral outcomes to derive completed-prefix events. */
function oracle(row: Case): Progress[] {
  const directories = new Set<string>();
  for (const path of row.paths) for (let parent = oracleDirname(path); parent && parent !== "."; parent = oracleDirname(parent)) directories.add(parent);
  const ordered = [...directories].sort((left, right) => left.split("/").length - right.split("/").length || Buffer.compare(Buffer.from(left), Buffer.from(right)));
  expect(ordered).toEqual(row.expected.directories);
  expect(row.expected.visited).toEqual(ordered.slice(0, row.expected.visited.length));
  if (row.cancel?.when === "before-start") return [];
  const base = { operation: row.operation, phase: vector.semantics.phase };
  if (!ordered.length) return [{ ...base, current: 0, total: 0 }];
  const pending = row.expected.visited.map((path, current) => ({ ...base, current, total: ordered.length, path }));
  return row.expected.error ? pending : [...pending, { ...base, current: ordered.length, total: ordered.length }];
}

/** 🧫️ Executes actual production code against an explicit in-memory filesystem and Git model. */
function execute(compiler: typeof compiled[number], row: Case, withProgress = true) {
  const repoRoot = resolve(root, "virtual-reference-coordinate-progress"), nodes = new Map<string, VirtualNode>();
  const markerProbes: string[] = [], observers: string[] = [], io: string[] = [], events: Progress[] = [], ioEventCounts: number[] = [];
  let cancelled = row.cancel?.when === "before-start";
  const addParents = (path: string) => { for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) nodes.set(parent, { kind: "directory" }); };
  for (const path of row.paths) addParents(path);
  for (const path of Object.keys(row.nodes)) addParents(path);
  nodes.set("", { kind: "directory" }); nodes.set(".git", { kind: "directory" });
  for (const [path, node] of Object.entries(row.nodes)) nodes.set(path, node);
  const local = (path: string) => relative(repoRoot, path).replaceAll("\\", "/");
  const access = (path: string) => {
    const name = local(path);
    if (name === ".." || name.startsWith("../") || vector.semantics.opaqueRoots.some((opaque: string) => name === opaque || name.startsWith(opaque + "/"))) throw new Error("Forbidden virtual access: " + name);
    io.push(name); ioEventCounts.push(events.length);
    return name;
  };
  const deps = {
    Buffer, posix, resolve, relative, dirname, join, isAbsolute, sep,
    lstatSync(path: string) {
      const name = access(path), node = nodes.get(name);
      if (name.endsWith("/.git")) markerProbes.push(name);
      if (!node) throw Object.assign(new Error("absent"), { code: "ENOENT" });
      return { isFile: () => node.kind === "file", isDirectory: () => node.kind === "directory", isSymbolicLink: () => node.kind === "symlink" };
    },
    readFileSync(path: string) {
      const name = access(path), node = nodes.get(name);
      if (node?.kind !== "file") throw new Error("Invalid virtual file read");
      return Buffer.from(node.content!);
    },
    execFileSync(command: string, args: string[]) {
      expect(command).toBe("git"); expect(args).toEqual(["rev-parse", "--absolute-git-dir"]);
      io.push("git:absolute-git-dir");
      return join(repoRoot, ".git") + "\n";
    },
    spawnSync(command: string, args: string[], options: { cwd: string }) {
      expect(command).toBe("git"); expect(args).toEqual(["rev-parse", "--show-toplevel"]);
      const name = local(options.cwd); io.push("git:owner:" + name);
      return { status: nodes.get(name + "/.git")?.gitRoot ? 0 : 128, stdout: nodes.get(name + "/.git")?.gitRoot ? options.cwd + "\n" : "" };
    },
    checkCancellation() { if (cancelled) throw new Error("Taxonomy operation cancelled"); },
  };
  const api = new Function(...Object.keys(deps), compiler.code + "\nreturn referenceCoordinateRoots;")(...Object.values(deps));
  const observe = (path: string) => { observers.push(path); if (row.cancel?.when === "observer" && row.cancel.path === path) cancelled = true; };
  const progress = (event: Progress) => {
    events.push(JSON.parse(JSON.stringify(event)));
    if (row.cancel?.when === "pending" && event.path === row.cancel.path || row.cancel?.when === "empty" && event.total === 0) cancelled = true;
  };
  let roots: string[] | null = null, error: string | null = null;
  try { roots = api(repoRoot, row.paths, { exclusions: vector.semantics.opaqueRoots.map((path: string) => ({ path })) }, "cancel", observe, withProgress ? progress : undefined, row.operation); }
  catch (caught) { error = caught instanceof Error ? caught.message : String(caught); }
  return { roots, error, events, markerProbes, observers, io, ioEventCounts };
}

test("neutral coordinate progress contract has independent schema and directory-order parity", () => {
  const validate = new Ajv({ strict: false, allErrors: true }).compile(grammar);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  expect(parseJsonc(input(owner + "../🧪️🍊️reference-coordinate-progress/🔣️.json").toString("utf8"))).toEqual(vector);
  expect(new Set(vector.cases.map((row: Case) => row.id)).size).toBe(vector.cases.length);
  expect(validate({ ...vector, unknown: true })).toBe(false);
  expect(validate({ ...vector, semantics: { ...vector.semantics, current: "attempted-candidates" } })).toBe(false);
  expect(validate({ ...vector, registration: { ...vector.registration, order: 999 } })).toBe(false);
  for (const row of vector.cases) oracle(row);
});

for (const row of vector.cases as Case[]) test("actual coordinate progress: " + row.id, () => {
  const results = compiled.map((compiler) => {
    const actual = execute(compiler, row);
    expect(actual.events, compiler.name).toEqual(oracle(row));
    expect(actual.roots, compiler.name).toEqual(row.expected.roots);
    expect(actual.error, compiler.name).toBe(row.expected.error);
    expect(actual.markerProbes, compiler.name).toEqual(row.expected.markerProbes);
    expect(actual.observers, compiler.name).toEqual(row.expected.observers);
    expect(actual.ioEventCounts.every((count) => count > 0), compiler.name).toBe(true);
    if (row.expected.error && row.expected.directories.length) expect(actual.events.some((event) => event.current === event.total && !event.path)).toBe(false);
    if (!row.cancel) {
      const silent = execute(compiler, row, false);
      expect({ ...silent, events: [], ioEventCounts: [] }).toEqual({ ...actual, events: [], ioEventCounts: [] });
    }
    return actual;
  });
  expect(results[0]).toEqual(results[1]);
});

/** 🧭️ Finds only actual calls inside a selected production function. */
function calls(ownerName: string, callee: string): ts.CallExpression[] {
  const result: ts.CallExpression[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === callee) result.push(node);
    ts.forEachChild(node, visit);
  };
  visit(declaration(ownerName));
  return result;
}

test("actual plan and apply callers forward progress without changing the marker observer position", () => {
  const resolver = declaration("referenceCoordinateRoots") as ts.FunctionDeclaration;
  expect(resolver.parameters.map((node) => node.name.getText(syntax))).toEqual(["repoRoot", "paths", "taxonomy", "cancelFile", "observe", "progress", "operation"]);
  expect(resolver.parameters[6]?.initializer?.getText(syntax).replaceAll(/["']/gu, "")).toBe("plan");
  for (const [caller, callback, operation] of [["incomingReferenceSnapshot", "options.progress", "plan"], ["capturePreflightReferenceBasis", "progress", "apply"], ["lexicalTargetIncomingReferences", "progress", "apply"]]) {
    const sites = calls(caller!, "referenceCoordinateRoots");
    expect(sites).toHaveLength(1);
    expect(sites[0]!.arguments[5]?.getText(syntax)).toBe(callback);
    expect(sites[0]!.arguments[6]?.getText(syntax).replaceAll(/["']/gu, "")).toBe(operation);
  }
  const capture = declaration("capturePreflightReferenceBasis") as ts.FunctionDeclaration;
  expect(capture.parameters[6]?.name.getText(syntax)).toBe("progress");
  expect(calls("validatePreflightReferenceBasis", "capturePreflightReferenceBasis")[0]?.arguments[6]?.getText(syntax)).toBe("progress");
  const apply = calls("applyTaxonomyPlan", "capturePreflightReferenceBasis");
  expect(apply).toHaveLength(2);
  expect(apply.map((node) => node.arguments[6]?.getText(syntax))).toEqual(["options.progress", "options.progress"]);
  expect(ts.isArrowFunction(calls("capturePreflightReferenceBasis", "referenceCoordinateRoots")[0]!.arguments[4]!)).toBe(true);
});

test("registration only: coordinate progress has one exact route package and launch owner", () => {
  const registration = vector.registration;
  const project = JSON.parse(input(packageRoot + "/📋️project.json").toString("utf8"));
  const packageJson = JSON.parse(input(packageRoot + "/package.json").toString("utf8"));
  const router = input(packageRoot + "/📜️script.ts").toString("utf8");
  const launches = [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"].map((path) => parseJsonc(input(path).toString("utf8")));
  expect(project.targets[registration.target]?.options?.command).toBe("bun ./📜️script.ts test " + registration.command);
  expect(packageJson.scripts?.[registration.target]).toBe("nx run @semio-tech/repo-lib:" + registration.target);
  expect(router).toContain('"' + registration.command + '"');
  expect(router).toContain(registration.testPath);
  for (const data of launches) {
    const rows = data.configurations.filter((row: any) => row.name === registration.launchName);
    expect(rows).toHaveLength(1);
    expect(rows[0]?.command).toBe(registration.nxCommand);
    expect(rows[0]?.presentation).toEqual({ group: registration.group, order: registration.order });
  }
});

afterAll(() => {
  const identities = [...observations].map(([path, before]) => ({ path, sha256: createHash("sha256").update(before).digest("hex"), size: before.length, stable: before.equals(input(path)) }));
  expect(identities.every((entry) => entry.stable)).toBe(true);
  console.log("[DEBUG] reference-coordinate-progress inputs " + JSON.stringify(identities));
});
