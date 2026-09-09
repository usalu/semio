import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc, visit as visitJsonc } from "jsonc-parser";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🔎️json-reference-owner-lookup/🔣️.json"), "utf8"));
const text = readFileSync(join(root, library, "🧹️normalization/🟦️.ts"), "utf8");
const syntax = ts.createSourceFile("../🔎️json-reference-owner-lookup/🟦️.ts", text, ts.ScriptTarget.Latest, true);
type Token = { adapter: string; structuredLocation: string; start: number; end: number; value: string; [name: string]: unknown };
type Parser = (path: string, content: string, adapter: "json" | "jsonc") => Token[];
const helpers = new Set(["lineLocation", "embeddedArgumentTokens"]);
const constants = new Set(["indexedLineContent", "indexedLineStarts"]);
const support = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? helpers.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax)).join("\n");
const declaration = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === "jsonTokens");
if (declaration.length !== 1) throw new Error("Expected one actual jsonTokens implementation");
const parserSource = declaration[0].getText(syntax);
const compilers = [
  { name: "Bun", compile: (source: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(source) },
  { name: "TypeScript", compile: (source: string) => ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];

/** 🧬️ Executes the actual private parser and helpers through independent compilers. */
function implementation(compiler: typeof compilers[number]) {
  const parse: Parser = new Function(compiler.compile(`${support}\n${parserSource}`) + "\nreturn jsonTokens;")();
  return { parse };
}

/** 🔬️ Uses independent JSONC visitor spans, retaining duplicate properties and exact UTF-16 string coordinates. */
function jsoncOracle(content: string, adapter: string): Token[] {
  const rows: Token[] = [];
  const add = (value: string, offset: number, length: number, key: boolean): void => {
    const start = offset + 1, end = offset + length - 1;
    if (content.slice(start, end) === value) rows.push({ adapter, structuredLocation: "/@" + (key ? "key" : "value") + "[" + rows.length + "]@" + start, start, end, value });
  };
  visitJsonc(content, { onObjectProperty: (value, offset, length) => add(value, offset, length, true), onLiteralValue: (value, offset, length) => { if (typeof value === "string") add(value, offset, length, false); } }, { allowTrailingComma: true });
  return rows;
}

/** 🧾️ Uses the TypeScript JSON AST as a second independent source-coordinate oracle. */
function typescriptOracle(content: string, adapter: string): Token[] {
  const document = ts.parseJsonText("../../🧫️fixtures/🔎️json-reference-owner-lookup/🔣️.json", content), rows: Token[] = [];
  const visit = (node: ts.Node, parent?: ts.Node): void => {
    if (ts.isStringLiteral(node)) {
      const start = node.getStart(document) + 1, end = node.getEnd() - 1;
      if (content.slice(start, end) === node.text) {
        const key = parent && ts.isPropertyAssignment(parent) && parent.name === node;
        rows.push({ adapter, structuredLocation: "/@" + (key ? "key" : "value") + "[" + rows.length + "]@" + start, start, end, value: node.text });
      }
    }
    ts.forEachChild(node, (child) => visit(child, node));
  };
  visit(document);
  return rows;
}

test("the language-neutral contract fixes call-local, lazy, null-preserving ownership semantics", () => {
  const validate = new Ajv().compile({ type: "object", required: ["schemaVersion", "contract", "semantics", "cases", "projections", "corpus", "execution"], properties: { schemaVersion: { const: 1 }, contract: { const: "json-reference-owner-lookup-v1" }, semantics: { const: { scope: "one-jsonTokens-call", admission: "first-non-key-unescaped-json-string-value", absentOwner: "cache-null", errorTiming: "first-admitted-value", output: "identical-token-values-utf16-offsets-order-and-metadata" } }, cases: { type: "array", minItems: 1, items: { type: "object", required: ["id", "path", "source", "adapter", "owner", "lookups", "oracle"], properties: { lookups: { enum: [0, 1] }, adapter: { enum: ["json", "jsonc"] }, owner: { type: ["string", "null"] } } } } } });
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
});

for (const compiler of compilers) test(compiler.name + " parses JSON references without mutation projection owner state", () => {
  const actual = implementation(compiler);
  for (const row of vector.cases.filter((entry: { error?: string }) => !entry.error)) for (const token of actual.parse(row.path, row.source, row.adapter)) expect(row.source.slice(token.start, token.end)).toBe(token.value);
});

test("unescaped JSON and JSONC token bytes, offsets, ordinals and ordering match two independent parsers", () => {
  for (const compiler of compilers) {
    const actual = implementation(compiler);
    for (const row of vector.cases.filter((row: { oracle: boolean; error?: string }) => row.oracle && !row.error)) {
      const expected = jsoncOracle(row.source, row.adapter);
      expect(typescriptOracle(row.source, row.adapter)).toEqual(expected);
      const tokens = actual.parse(row.path, row.source, row.adapter).filter((token) => /^\/@(?:key|value)\[\d+\]@\d+$/u.test(token.structuredLocation));
      expect(tokens).toEqual(expected);
    }
  }
});

test("legacy mutation projection strings remain ordinary JSON values", () => {
  for (const compiler of compilers) for (const row of vector.projections) {
    const value = row.prefix + row.source, content = JSON.stringify([value]);
    expect(implementation(compiler).parse(row.path, content, "json")).toEqual(jsoncOracle(content, "json"));
  }
});

test("separate parser calls retain independent coordinates", () => {
  for (const compiler of compilers) {
    const actual = implementation(compiler);
    const rows = [vector.cases[0], vector.cases[1], vector.cases[0], vector.cases[2], vector.cases[1]];
    for (const row of rows) expect(actual.parse(row.path, '["first","second"]', "json")).toEqual(jsoncOracle('["first","second"]', "json"));
  }
});

test("actual JSON and JSONC corpus tokens retain complete metadata across independent implementations", () => {
  const parsers = compilers.map(implementation);
  for (const path of vector.corpus) {
    const content = readFileSync(join(root, path), "utf8"), adapter = path.endsWith(".jsonc") ? "jsonc" : "json";
    const results = parsers.map((actual) => actual.parse(path, content, adapter));
    expect(results[0]).toEqual(results[1]);
    for (const tokens of results) expect(tokens.every((token) => content.slice(token.start, token.end) === token.value)).toBe(true);
    expect(readFileSync(join(root, path), "utf8")).toBe(content);
  }
});

test("registers the JSON owner lookup gate through Nx and both launch catalogs", () => {
  const expected = vector.execution;
  const project = JSON.parse(readFileSync(join(root, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  const router = readFileSync(join(root, library, "📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  expect(router.match(/segments\[0\] === "json-reference-owner-lookup"/gu)).toHaveLength(1);
  expect(router).toContain("🧪️tests/🟦️json-reference-owner-lookup.ts");
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(root, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
  }
});
