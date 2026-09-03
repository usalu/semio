import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import MarkdownIt from "markdown-it";
import ts from "typescript";

type Span = Readonly<{ start: number; end: number; value: string }>;
type Vector = Readonly<{
  schemaVersion: 1;
  contract: "gherkin-description-inline-code-v1";
  cases: readonly Readonly<{ id: string; source: string; oracle: boolean; expected: readonly Span[] }>[];
  corpus: readonly Readonly<{ path: string; values: readonly string[] }>[];
}>;

const library = resolve(import.meta.dir, "../..");
const root = resolve(library, "../../../../..");
const normalizerPath = join(library, "🧹️normalization/🟦️.ts");
const vectorPath = join(import.meta.dir, "../🧪️🐬️gherkin-description-inline-code/🔣️.json"), schemaPath = join(import.meta.dir, "../🧪️🐬️gherkin-description-inline-code/🧬️schema/🔣️.json");
const vector: Vector = JSON.parse(readFileSync(vectorPath, "utf8"));
const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
const normalizerText = readFileSync(normalizerPath, "utf8");
const syntax = ts.createSourceFile("../🧪️🐬️gherkin-description-inline-code/🟦️.ts", normalizerText, ts.ScriptTarget.Latest, true);

const spanDeclaration = syntax.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "gherkinDescriptionInlineCodeSpans");
if (!spanDeclaration) throw new Error("Missing actual gherkinDescriptionInlineCodeSpans implementation");
const spanSource = spanDeclaration.getText(syntax);

const gherkinTokensDeclaration = syntax.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "gherkinTokens");
if (!gherkinTokensDeclaration) throw new Error("Missing actual gherkinTokens implementation");

const compilers = [
  { name: "Bun", compile: (source: string): string => new Bun.Transpiler({ loader: "ts" }).transformSync(source) },
  { name: "TypeScript", compile: (source: string): string => ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText },
];

/** 🔒️ Rejects any process/import escape capability in the exact extracted production function — it has zero declared dependencies, so nothing beyond bare syntax should compile. */
function assertPureClosure(): void {
  const forbidden: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) || ts.isImportEqualsDeclaration(node) || node.kind === ts.SyntaxKind.ImportKeyword) forbidden.push(node.getText(syntax));
    if (ts.isIdentifier(node) && ["process", "Bun", "globalThis", "require", "eval", "Function", "constructor"].includes(node.text)) forbidden.push(node.text);
    ts.forEachChild(node, visit);
  };
  visit(spanDeclaration!);
  expect(forbidden).toEqual([]);
}

function compiledSpans(compiler: typeof compilers[number]): (content: string) => Span[] {
  return new Function(compiler.compile(spanSource) + "\nreturn gherkinDescriptionInlineCodeSpans;")() as (content: string) => Span[];
}

/** 🥒️ Slices the raw code-unit span the actual production function reports, without duplicating its scanning logic. */
function actualValues(spans: Span[], content: string): Span[] {
  return spans.map((span) => ({ start: span.start, end: span.end, value: content.slice(span.start, span.end) }));
}

function codeInlineValues(source: string): string[] {
  const md = new MarkdownIt(), env = {};
  const state = new md.core.State(source, md, env);
  md.core.process(state);
  const values: string[] = [];
  const visit = (tokens: readonly import("markdown-it/lib/token.mjs").default[]): void => {
    for (const token of tokens) {
      if (token.type === "code_inline") values.push(token.content);
      if (token.children) visit(token.children);
    }
  };
  visit(state.tokens);
  return values;
}

test("the vector is a closed, self-consistent contract with a real independent third-party oracle", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  const bad = [
    { ...vector, extra: true },
    { ...vector, schemaVersion: 2 },
    { ...vector, cases: vector.cases.slice(1) },
    { ...vector, cases: vector.cases.map((row, index) => (index ? row : { ...row, extra: true })) },
    { ...vector, corpus: vector.corpus.map((row, index) => (index ? row : { ...row, values: [] })) },
  ];
  for (const row of bad) expect(validate(row)).toBe(false);
  expect(new Set(vector.cases.map((row) => row.id)).size).toBe(vector.cases.length);
  for (const row of vector.cases) for (const span of row.expected) {
    expect(row.source.slice(span.start, span.end), row.id).toBe(span.value);
    expect(span.end, row.id).toBe(span.start + span.value.length);
  }
  const installed = JSON.parse(readFileSync(join(root, "node_modules/markdown-it/package.json"), "utf8"));
  const packageJson = JSON.parse(readFileSync(join(library, "📦️packages/🟦️typescript/package.json"), "utf8"));
  expect(packageJson.devDependencies?.["markdown-it"]).toBeDefined();
  expect(packageJson.dependencies?.["markdown-it"]).toBeUndefined();
  expect(typeof installed.version).toBe("string");
});

for (const compiler of compilers) test(compiler.name + " actual extraction matches every declared case, including cross-line and blank-line reset discipline", () => {
  assertPureClosure();
  const spans = compiledSpans(compiler);
  for (const row of vector.cases) expect(actualValues(spans(row.source), row.source), row.id).toEqual(row.expected);
});

test("both real compiler closures agree with each other on the full case set", () => {
  const [bun, typescript] = compilers.map(compiledSpans);
  for (const row of vector.cases) expect(actualValues(bun(row.source), row.source)).toEqual(actualValues(typescript(row.source), row.source));
});

test("an independent markdown-it oracle recognises the same ordered code-span values on the common, non-edge-case subset", () => {
  const spans = compiledSpans(compilers[0]!);
  for (const row of vector.cases.filter((candidate) => candidate.oracle)) {
    const oracleValues = codeInlineValues(row.source);
    const actual = actualValues(spans(row.source), row.source).map((span) => span.value);
    expect(actual, row.id).toEqual(row.expected.map((span) => span.value));
    expect(oracleValues, row.id).toEqual(row.expected.map((span) => span.value));
  }
});

test("real committed .feature fixtures under 🖼️assets recognise every documented asset-path description reference", () => {
  const spans = compiledSpans(compilers[0]!);
  for (const row of vector.corpus) {
    const content = readFileSync(join(root, row.path), "utf8");
    const found = new Set(actualValues(spans(content), content).map((span) => span.value));
    for (const value of row.values) expect(found.has(value), row.path + " -> " + value).toBe(true);
  }
});

test("gherkinTokens wires the description scanner into a plain gherkin reference token, so removing the call site would leave these targets unresolved again", () => {
  const source = gherkinTokensDeclaration!.getText(syntax);
  expect(source).toContain("gherkinDescriptionInlineCodeSpans(content)");
  expect(source).toContain('"gherkin-description-inline-code"');
  expect(source).toContain('adapter: "gherkin"');
  const calls: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "gherkinDescriptionInlineCodeSpans") calls.push(node.getText(syntax));
    ts.forEachChild(node, visit);
  };
  visit(gherkinTokensDeclaration!);
  expect(calls).toHaveLength(1);
});

test("registers the gherkin description inline-code gate through Nx and both launch catalogs", () => {
  const project = JSON.parse(readFileSync(join(library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets["test-gherkin-description-inline-code"]?.options.command).toBe("bun ./📜️script.ts test gherkin-description-inline-code");
  const router = readFileSync(join(library, "📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  expect(router.match(/segments\[0\] === "gherkin-description-inline-code"/gu)).toHaveLength(1);
  expect(router).toContain("🧪️tests/🟦️gherkin-description-inline-code.ts");
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const content = readFileSync(join(root, path), "utf8");
    expect(content, path).toContain("test-gherkin-description-inline-code");
  }
});
