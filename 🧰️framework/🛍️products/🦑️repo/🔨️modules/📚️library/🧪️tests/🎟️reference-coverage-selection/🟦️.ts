import { afterAll, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import Ajv from "ajv";
import differenceWith from "lodash/differenceWith.js";
import { parse, type ParseError } from "jsonc-parser";
import ts from "typescript";

type Token = Readonly<{ id: string; adapter: string; start: number; end: number; value: string; rewriteKind?: string; physicalInterpretation?: string; physicalTargets?: readonly string[]; unsupportedReason?: string }>;
type Scenario = Readonly<{ id: string; supported: readonly string[]; unsupported: readonly string[]; retained: readonly string[] }>;
const library = resolve(import.meta.dir, "../.."), root = resolve(library, "../../../../..");
const inputBytes = readFileSync(join(import.meta.dir, "../🎟️reference-coverage-selection/🔣️.json")), vector = JSON.parse(inputBytes.toString("utf8"));
const normalizerPath = join(library, "🧹️normalization/🟦️.ts"), normalizerBytes = readFileSync(normalizerPath);
const tree = ts.createSourceFile(normalizerPath, normalizerBytes.toString("utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const declaration = (name: string): string => {
  const declarations = tree.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === name);
  expect(declarations, name).toHaveLength(1);
  return declarations[0]!.getText(tree);
};
const names = ["rustReferenceInterpretationCovers", "referenceTokensIncludingUnsupported", "referenceAdapter"];
const bodies = Object.fromEntries(names.map((name) => [name, declaration(name)]));
const compilers = [
  { id: "Bun", compile: (code: string): string => new Bun.Transpiler({ loader: "ts" }).transformSync(code) },
  { id: "TypeScript", compile: (code: string): string => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText },
];
const token = (id: string, adapter?: string): Token => Object.freeze({ id, ...vector.tokens[id], ...(adapter === undefined ? {} : { adapter }) });

/** 🔬️ Observes coverage comparisons only inside the actual extracted selector closure. */
function select(compiler: typeof compilers[number], path: string, supported: readonly Token[], unsupported: readonly Token[]) {
  let coverageCalls = 0, sourceCalls = 0, unsupportedCalls = 0;
  const covers = new Function(compiler.compile(bodies.rustReferenceInterpretationCovers!) + "\nreturn rustReferenceInterpretationCovers;")() as (left: Token, right: Token) => boolean;
  const input = Object.freeze({ path }), content = "exact controlled tokenizer boundary";
  const operation = new Function("basename", "referenceTokens", "unsupportedReferenceTokens", "rustReferenceInterpretationCovers", compiler.compile(bodies.referenceAdapter! + "\n" + bodies.referenceTokensIncludingUnsupported!) + "\nreturn referenceTokensIncludingUnsupported;")(
    basename,
    (actualPath: string, actualContent: string, actualInput: unknown) => { sourceCalls++; expect([actualPath, actualContent, actualInput]).toEqual([path, content, input]); return supported; },
    (actualContent: string, adapter: string) => { unsupportedCalls++; expect(actualContent).toBe(content); expect(unsupported.every((row) => row.adapter === adapter)).toBe(true); return unsupported; },
    (left: Token, right: Token) => { coverageCalls++; return covers(left, right); },
  );
  const output = operation(path, content, input) as readonly Token[];
  expect([sourceCalls, unsupportedCalls]).toEqual([1, 1]);
  expect(output).not.toBe(supported);
  expect(output.slice(0, supported.length)).toEqual(supported);
  for (const row of output) expect(supported.includes(row) || unsupported.includes(row)).toBe(true);
  return { output, coverageCalls };
}

/** 🧮️ Uses third-party stable difference with a separately stated interval/exact-target law. */
function oracle(supported: readonly Token[], unsupported: readonly Token[]): readonly Token[] {
  const remaining = differenceWith(unsupported, supported, (candidate: Token, owner: Token) => {
    if (candidate.adapter !== "rust" || owner.adapter !== "rust") return false;
    if (owner.rewriteKind === "rust-path-join") return Math.min(owner.start, candidate.start) === owner.start && Math.max(owner.end, candidate.end) === owner.end;
    return [owner.physicalInterpretation === "rust-finite-manifest-targets", owner.rewriteKind === undefined, Boolean(owner.unsupportedReason), Array.isArray(owner.physicalTargets) && owner.physicalTargets.length > 0, owner.start === candidate.start, owner.end === candidate.end, owner.value === candidate.value].every(Boolean);
  });
  return [...supported, ...remaining];
}

test("reference coverage selection has a closed neutral contract and every adapter", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const bad of [{ ...vector, extra: true }, { ...vector, schemaVersion: 2 }, { ...vector, scale: { ...vector.scale, expectedCoverageCalls: 1 } }]) expect(validate(bad)).toBe(false);
  const errors: ParseError[] = [];
  expect(parse(inputBytes.toString("utf8"), errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(errors).toEqual([]);
  const aliases = tree.statements.filter((node): node is ts.TypeAliasDeclaration => ts.isTypeAliasDeclaration(node) && node.name.text === "TaxonomyReferenceAdapter");
  expect(aliases).toHaveLength(1);
  expect(ts.isUnionTypeNode(aliases[0]!.type)).toBe(true);
  const adapters = (aliases[0]!.type as ts.UnionTypeNode).types.map((node) => (node as ts.LiteralTypeNode).literal.getText(tree).slice(1, -1));
  expect([...vector.nonRustAdapters.map((row: { adapter: string }) => row.adapter), "rust"].sort()).toEqual(adapters.sort());
  for (const scenario of vector.rustCases as Scenario[]) for (const id of [...scenario.supported, ...scenario.unsupported, ...scenario.retained]) expect(vector.tokens[id], id).toBeDefined();
});

test("actual Rust coverage retains exact finite and writable laws through both compilers", () => {
  for (const compiler of compilers) for (const scenario of vector.rustCases as Scenario[]) {
    const supported = Object.freeze(scenario.supported.map((id) => token(id))), unsupported = Object.freeze(scenario.unsupported.map((id) => token(id)));
    const expected = [...scenario.supported, ...scenario.retained];
    expect(oracle(supported, unsupported).map((row) => row.id), scenario.id).toEqual(expected);
    expect(select(compiler, "🧪️case/🦀️.rs", supported, unsupported).output.map((row) => row.id), compiler.id + ":" + scenario.id).toEqual(expected);
  }
});

test("non-Rust candidates preserve duplicates and order without Rust coverage comparisons", () => {
  const counts: number[] = [];
  for (const compiler of compilers) for (const row of vector.nonRustAdapters) {
    const supported = Object.freeze(vector.nonRustSupported.map((id: string) => token(id))), unsupported = Object.freeze(vector.nonRustUnsupported.map((id: string) => token(id, row.adapter)));
    const actual = select(compiler, row.path, supported, unsupported), expected = oracle(supported, unsupported);
    expect(actual.output).toEqual(expected);
    expect(actual.output.map((entry) => entry.id)).toEqual([...vector.nonRustSupported, ...vector.nonRustUnsupported]);
    console.info("[DEBUG] Adapter-local coverage " + JSON.stringify({ compiler: compiler.id, adapter: row.adapter, coverageCalls: actual.coverageCalls }));
    counts.push(actual.coverageCalls);
  }
  expect(counts).toEqual(Array(compilers.length * vector.nonRustAdapters.length).fill(0));
});

test("large non-Rust token sets never perform a quadratic Rust coverage join", () => {
  const counts: number[] = [];
  for (const compiler of compilers) {
    const supported = Object.freeze(Array.from({ length: vector.scale.supported }, (_, index) => Object.freeze({ ...token("writable"), id: "supported-" + index })));
    const unsupported = Object.freeze(Array.from({ length: vector.scale.unsupported }, (_, index) => Object.freeze({ ...token("interior", "json"), id: "unsupported-" + index })));
    const actual = select(compiler, "🧪️case/🔣️.json", supported, unsupported);
    expect(actual.output).toEqual(oracle(supported, unsupported));
    expect(actual.output).toHaveLength(vector.scale.supported + vector.scale.unsupported);
    console.info("[DEBUG] Bounded coverage comparison count " + JSON.stringify({ compiler: compiler.id, supported: supported.length, unsupported: unsupported.length, comparisons: actual.coverageCalls }));
    counts.push(actual.coverageCalls);
  }
  expect(counts).toEqual(compilers.map(() => vector.scale.expectedCoverageCalls));
});

test("reference coverage gate is registered through its exact default-budget route", async () => {
  const expected = vector.execution, pkg = join(library, "📦️packages/🟦️typescript"), project = JSON.parse(readFileSync(join(pkg, "📋️project.json"), "utf8")), packageJson = JSON.parse(readFileSync(join(pkg, "package.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe("bun ./📜️script.ts test " + expected.route);
  expect(packageJson.scripts[expected.target]).toBe("nx run @semio-tech/repo-lib:" + expected.target);
  const router = readFileSync(join(pkg, "📜️script.ts"), "utf8");
  expect(router).toContain('if (segments[0] === "' + expected.route + '")');
  expect(router).toContain('🧪️tests/🎟️reference-coverage-selection/🟦️.ts');
  const routerTree = ts.createSourceFile("📜️script.ts", router, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => { if (ts.isIfStatement(node) && node.expression.getText(routerTree) === 'segments[0] === "' + expected.route + '"') branches.push(node); ts.forEachChild(node, visit); };
  visit(routerTree);
  expect(branches).toHaveLength(1);
  const body = branches[0]!.thenStatement.getText(routerTree);
  for (const compiler of compilers) {
    const calls: unknown[][] = [], operation = new Function("join", "process", "runTestBudgeted", compiler.compile("async function route(segments: string[]) " + body) + "\nreturn route;")(join, { execPath: "exact-bun" }, (...args: unknown[]) => { calls.push(args); });
    await operation.call({ repoRoot: root }, [expected.route, "--test-name-pattern", "retained-selector"]);
    expect(calls).toEqual([["exact-bun", ["test", join(import.meta.dir, "../🎟️reference-coverage-selection/🟦️.ts"), "--test-name-pattern", "retained-selector"], { cwd: root }]]);
  }
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const errors: ParseError[] = [], document = parse(readFileSync(join(root, path), "utf8"), errors, { allowTrailingComma: true });
    expect(errors).toEqual([]);
    const rows = document.configurations.filter((row: { name: string }) => row.name === expected.launchName);
    expect(rows).toHaveLength(1);
    expect(rows[0]).toEqual({ name: expected.launchName, type: "node-terminal", request: "launch", command: "bun nx run @semio-tech/repo-lib:" + expected.target + " --skip-nx-cache", cwd: "${workspaceFolder}", presentation: { group: "4_gate", order: expected.launchOrder } });
  }
});

afterAll(() => {
  const current = ts.createSourceFile(normalizerPath, readFileSync(normalizerPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  for (const name of names) {
    const node = current.statements.find((entry) => ts.isFunctionDeclaration(entry) && entry.name?.text === name);
    expect(node?.getText(current), name + " changed during the gate").toBe(bodies[name]);
  }
});
