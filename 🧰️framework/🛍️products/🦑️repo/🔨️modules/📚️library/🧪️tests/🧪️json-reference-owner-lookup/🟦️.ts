import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, posix, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc, visit as visitJsonc } from "jsonc-parser";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧪️🍀️json-reference-owner-lookup/🔣️.json"), "utf8"));
const text = readFileSync(join(root, library, "🧹️normalization/🟦️.ts"), "utf8");
const syntax = ts.createSourceFile("../🧪️🍀️json-reference-owner-lookup/🟦️.ts", text, ts.ScriptTarget.Latest, true);
type Token = { adapter: string; structuredLocation: string; start: number; end: number; value: string; [name: string]: unknown };
type Parser = (path: string, content: string, adapter: "json" | "jsonc") => Token[];
const helpers = new Set(["normalizeRelative", "sourceRelative", "emojiFold", "graphemes", "isEmojiGrapheme", "splitLeadingEmoji", "lineLocation", "embeddedArgumentTokens", "artifactRootForPath", "mutationStructuralPaths", "canonicalProjectionSuffix", "projectionKey", "projectedStructuralValue", "structuralProjectionToken", "structuralTokensInFragment"]);
const constants = new Set(["SEGMENTER", "indexedLineContent", "indexedLineStarts", "OLD_MUTATION_TEST_PREFIX_SOURCE", "OLD_MUTATION_STRUCTURE_SOURCE"]);
const support = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? helpers.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax)).join("\n");
const declaration = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === "jsonTokens");
if (declaration.length !== 1) throw new Error("Expected one actual jsonTokens implementation");
const parserSource = declaration[0].getText(syntax);
const compilers = [
  { name: "Bun", compile: (source: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(source) },
  { name: "TypeScript", compile: (source: string) => ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];

const projectionScope = JSON.parse(readFileSync(join(import.meta.dir, "📍️projection-scope/🔣️.json"), "utf8"));

/** 🧭️ Executes the actual planner activation helper without scanning a workspace. */
function projectionScopeImplementation(compiler: typeof compilers[number]) {
  const helper = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "mutationReferenceProjectionState");
  if (!helper) throw new Error("Missing actual structural scope authority");
  const actual = new Function(compiler.compile(helper.getText(syntax)) + "\nreturn mutationReferenceProjectionState;")();
  return (row: any) => actual({ rewriteData: { artifactRoot: row.owner, projectionProfile: row.profile }, ...(row.hasTarget ? { targetValues: ["unresolved-physical-source"] } : {}) }, row.target, new Set(projectionScope.activeKeys), row.scope);
}

test("the planner invokes structural scope authority and retains its unresolved branch", () => {
  const planner = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "buildReferenceEdits");
  if (!planner) throw new Error("Missing actual reference planner");
  const calls: string[][] = [], guards: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "mutationReferenceProjectionState") calls.push(node.arguments.map((argument) => argument.getText(syntax)));
    if (ts.isIfStatement(node) && node.expression.getText(syntax) === 'projectionState === "unproven"') guards.push(node.thenStatement.getText(syntax));
    ts.forEachChild(node, visit);
  };
  visit(planner);
  expect(calls).toEqual([["token", "oldTarget", "activeProjectionKeys", "inventory.scope"]]);
  expect(guards).toHaveLength(1);
  expect(guards[0]).toContain('unresolved.push(violation("reference-syntax-unsupported"');
  expect(guards[0]).toContain("continue;");
  expect(planner.getText(syntax)).not.toContain("activeProjectionProfiles");
});

test("language-neutral projection scope decisions match an independent JSON Schema oracle", () => {
  const validate = new Ajv({ strict: false }).compile({
    type: "object", required: ["schemaVersion", "contract", "activeKeys", "semantics", "oracle", "cases"],
    properties: {
      schemaVersion: { const: 1 }, contract: { const: "mutation-reference-projection-scope-v1" },
      cases: { type: "array", minItems: 17, items: { type: "object", required: ["id", "owner", "profile", "scope", "target", "hasTarget", "state"], properties: { state: { enum: ["active", "inactive", "unproven"] }, target: { type: ["string", "null"] }, hasTarget: { type: "boolean" } } } },
    },
  });
  expect(validate(projectionScope), JSON.stringify(validate.errors)).toBe(true);
  const active = new Ajv({ strict: false }).compile(projectionScope.oracle.active), unproven = new Ajv({ strict: false }).compile(projectionScope.oracle.unproven);
  for (const row of projectionScope.cases) {
    expect(active(row) && unproven(row), row.id).toBe(false);
    expect(active(row) ? "active" : unproven(row) ? "unproven" : "inactive", row.id).toBe(row.state);
  }
});

for (const compiler of compilers) test(compiler.name + " scopes structural fallback to an exact complete artifact owner and leaves unproved owned references unresolved", () => {
  const actual = projectionScopeImplementation(compiler);
  expect(projectionScope.cases.map((row: any) => ({ id: row.id, state: actual(row) }))).toEqual(projectionScope.cases.map((row: any) => ({ id: row.id, state: row.state })));
});

/** 🧬️ Executes the actual private parser and helpers through independent compilers with an observable owner lookup. */
function implementation(compiler: typeof compilers[number]) {
  const dependencies = new Function("posix", compiler.compile(support) + "\nreturn { artifactRootForPath, structuralTokensInFragment, mutationStructuralPaths, embeddedArgumentTokens };")(posix);
  const owners: (string | null)[] = [], paths: string[] = [];
  const lookup = (path: string): string | null => { paths.push(path); const owner = dependencies.artifactRootForPath(path); owners.push(owner); return owner; };
  const parse: Parser = new Function("artifactRootForPath", "structuralTokensInFragment", "mutationStructuralPaths", "embeddedArgumentTokens", compiler.compile(parserSource) + "\nreturn jsonTokens;")(lookup, dependencies.structuralTokensInFragment, dependencies.mutationStructuralPaths, dependencies.embeddedArgumentTokens);
  return { parse, paths, owners };
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
  const document = ts.parseJsonText("../🧪️🍀️json-reference-owner-lookup/🔣️.json", content), rows: Token[] = [];
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

for (const compiler of compilers) test(compiler.name + " performs at most one owner lookup per call, including null and deferred errors", () => {
  const actual = implementation(compiler);
  for (const row of vector.cases) {
    const before = actual.paths.length, owners = actual.owners.length;
    if (row.error) expect(() => actual.parse(row.path, row.source, row.adapter)).toThrow(row.error);
    else {
      const tokens = actual.parse(row.path, row.source, row.adapter);
      for (const token of tokens) expect(row.source.slice(token.start, token.end)).toBe(token.value);
      expect(actual.owners.slice(owners)).toEqual(row.lookups ? [row.owner] : []);
    }
    expect({ id: row.id, lookups: actual.paths.length - before }).toEqual({ id: row.id, lookups: row.lookups });
  }
  actual.parse(vector.cases[0].path, vector.cases[0].source, "json");
  expect(actual.owners.at(-1)).toBeNull();
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

test("owner-dependent structural projection metadata remains byte-for-byte stable for both implementations", () => {
  for (const compiler of compilers) for (const row of vector.projections) {
    const actual = implementation(compiler), value = row.prefix + row.source, content = JSON.stringify([value]);
    const base = jsoncOracle(content, "json");
    expect(actual.parse(row.path, content, "json")).toEqual([...base, {
      adapter: "json", structuredLocation: "/@value[0]/prose@2", start: 2, end: 2 + value.length, value,
      targetValues: row.owner ? [row.owner + "/" + row.source] : undefined,
      rewriteKind: row.prefix === "asset://" ? "artifact-uri" : "projection-prose",
      rewriteData: { newValue: row.prefix + row.destination, projectionKey: row.owner ? row.owner + "\u00001\u0000any" : "", projectionProfile: "1\u0000any", artifactRoot: row.owner ?? "" },
    }]);
    expect(actual.paths).toEqual([row.path]);
  }
});

test("separate calls never share an artifact owner or a null lookup result", () => {
  for (const compiler of compilers) {
    const actual = implementation(compiler);
    const rows = [vector.cases[0], vector.cases[1], vector.cases[0], vector.cases[2], vector.cases[1]];
    for (const row of rows) actual.parse(row.path, '["first","second"]', "json");
    expect(actual.paths).toEqual(rows.map((row: { path: string }) => row.path));
    expect(actual.owners).toEqual(rows.map((row: { owner: string | null }) => row.owner));
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
