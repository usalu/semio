import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { basename, dirname, join, posix, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const sourcePath = join(root, library, "🧹️normalization/🟦️.ts"), source = readFileSync(sourcePath, "utf8");
const syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧪️🧬️🐼️typescript-path-collection/🔣️.json"), "utf8"));
type Span = { value: string; start: number; end: number; physicalTargets: string[] };
type Token = Span & { adapter: string; structuredLocation: string; unsupportedReason?: string };
const compilers = [
  { name: "Bun", compile: (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code) },
  { name: "TypeScript", compile: (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];
const digest = (value: string) => createHash("sha256").update(value).digest("hex");

/** 🔬️ Uses independent TypeScript AST symbols, lexical owners, and immutable-use checks without executing input code. */
function oracle(content: string): Span[] {
  const name = "/virtual/input.ts", tree = ts.createSourceFile(name, content, ts.ScriptTarget.Latest, true);
  if ((tree as ts.SourceFile & { parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics.length) return [];
  const host: ts.CompilerHost = { getSourceFile: (path) => path === name ? tree : undefined, getDefaultLibFileName: () => "", writeFile: () => { throw new Error("Oracle cannot write"); }, getCurrentDirectory: () => "/virtual", getDirectories: () => [], fileExists: (path) => path === name, readFile: (path) => path === name ? content : undefined, getCanonicalFileName: (path) => path, useCaseSensitiveFileNames: () => true, getNewLine: () => "\n" };
  const checker = ts.createProgram([name], { noLib: true, noResolve: true, target: ts.ScriptTarget.ES2022 }, host).getTypeChecker();
  const identifiers: ts.Identifier[] = [], arrays: ts.VariableDeclaration[] = [], loops: ts.ForOfStatement[] = [], calls: ts.CallExpression[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isIdentifier(node)) identifiers.push(node);
    if (ts.isVariableDeclaration(node)) arrays.push(node);
    if (ts.isForOfStatement(node)) loops.push(node);
    if (ts.isCallExpression(node)) calls.push(node);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  if (identifiers.some((node) => node.getText(tree).includes("\\"))) return [];
  const symbol = (node: ts.Node) => checker.getSymbolAtLocation(node);
  const references = (node: ts.Identifier) => identifiers.filter((entry) => symbol(entry) === symbol(node));
  const contains = (outer: ts.Node, inner: ts.Node) => outer.pos <= inner.pos && outer.end >= inner.end;
  const assigned = (node: ts.Identifier): boolean => {
    for (let parent = node.parent; parent && !ts.isStatement(parent); parent = parent.parent) {
      if (ts.isBinaryExpression(parent) && parent.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && parent.operatorToken.kind <= ts.SyntaxKind.LastAssignment && contains(parent.left, node)) return true;
      if ((ts.isPrefixUnaryExpression(parent) || ts.isPostfixUnaryExpression(parent)) && [ts.SyntaxKind.PlusPlusToken, ts.SyntaxKind.MinusMinusToken].includes(parent.operator)) return true;
    }
    return false;
  };
  const declaration = (node: ts.Identifier): ts.VariableDeclaration | null => {
    const declarations = symbol(node)?.declarations;
    if (declarations?.length !== 1 || !ts.isVariableDeclaration(declarations[0]!) || !ts.isIdentifier(declarations[0]!.name)) return null;
    const row = declarations[0]!;
    return ts.isVariableDeclarationList(row.parent) && (row.parent.flags & ts.NodeFlags.Const) !== 0 && !references(row.name as ts.Identifier).some(assigned) ? row : null;
  };
  const exactImport = (node: ts.Expression, module: string, imported: string): boolean => {
    if (!ts.isIdentifier(node)) return false;
    const declarations = symbol(node)?.declarations;
    if (declarations?.length !== 1 || !ts.isImportSpecifier(declarations[0]!)) return false;
    const specifier = declarations[0]!, clause = specifier.parent.parent, owner = clause.parent;
    return !specifier.isTypeOnly && !clause.isTypeOnly && ts.isImportDeclaration(owner) && ts.isStringLiteral(owner.moduleSpecifier) && owner.moduleSpecifier.text === module && (specifier.propertyName ?? specifier.name).text === imported && !references(specifier.name).some(assigned);
  };
  const block = (node: ts.Node): ts.Node => { let current = node.parent; while (current && !ts.isBlock(current) && !ts.isSourceFile(current)) current = current.parent; return current; };
  const unwrap = (node: ts.Expression): ts.Expression => ts.isAsExpression(node) && node.type.getText(tree) === "const" ? node.expression : node;
  const plain = (node: ts.StringLiteralLike) => content.slice(node.getStart(tree) + 1, node.end - 1) === node.text && !node.getText(tree).includes("\\");
  const staticString = (node: ts.Expression, seen = new Set<ts.Symbol>()): string | null => {
    if (ts.isStringLiteralLike(node)) return plain(node) ? node.text : null;
    if (ts.isIdentifier(node)) {
      const owner = declaration(node), key = symbol(node);
      if (!owner?.initializer || !key || seen.has(key) || owner.getStart(tree) >= node.getStart(tree)) return null;
      return staticString(owner.initializer, new Set([...seen, key]));
    }
    if (!ts.isTemplateExpression(node) || node.getText(tree).includes("\\")) return null;
    let value = node.head.text;
    for (const span of node.templateSpans) { const part = staticString(span.expression, seen); if (part === null) return null; value += part + span.literal.text; }
    return value;
  };
  const rootValue = (node: ts.Expression): boolean => {
    if (!ts.isIdentifier(node)) return false;
    const owner = declaration(node), call = owner?.initializer;
    if (!call || owner!.getStart(tree) >= node.getStart(tree) || !ts.isCallExpression(call) || call.arguments.length || !ts.isPropertyAccessExpression(call.expression) || call.expression.name.text !== "cwd" || !ts.isIdentifier(call.expression.expression) || call.expression.expression.text !== "process" || symbol(call.expression.expression)) return false;
    return !identifiers.some((entry) => entry.text === "process" && !symbol(entry) && assigned(entry));
  };
  const relativeLeaf = (value: string): boolean => Boolean(value) && !value.startsWith("/") && !/^[A-Za-z]:/u.test(value) && !/[\\\u0000]/u.test(value) && !value.split("/").some((part) => !part || part === "." || part === "..");
  const result: Span[] = [];
  for (const owner of arrays) {
    if (!ts.isIdentifier(owner.name) || declaration(owner.name) !== owner || !owner.initializer) continue;
    if (ts.isVariableStatement(owner.parent.parent) && owner.parent.parent.modifiers?.some((node) => node.kind === ts.SyntaxKind.ExportKeyword)) continue;
    const array = unwrap(owner.initializer);
    if (!ts.isArrayLiteralExpression(array) || !array.elements.length || array.elements.some((element) => !ts.isExpression(element) || ts.isSpreadElement(element) || staticString(element) === null || !relativeLeaf(staticString(element)!))) continue;
    const uses = references(owner.name);
    if (uses.some((entry) => entry !== owner.name && !(ts.isForOfStatement(entry.parent) && entry.parent.expression === entry) && !(ts.isSpreadElement(entry.parent) && ts.isArrayLiteralExpression(entry.parent.parent)))) continue;
    const reader = loops.some((loop) => {
      if (!ts.isIdentifier(loop.expression) || symbol(loop.expression) !== symbol(owner.name) || loop.parent !== block(owner) || owner.getStart(tree) >= loop.getStart(tree) || loop.awaitModifier || !ts.isBlock(loop.statement) || !ts.isVariableDeclarationList(loop.initializer) || (loop.initializer.flags & ts.NodeFlags.Const) === 0 || loop.initializer.declarations.length !== 1) return false;
      const item = loop.initializer.declarations[0]!;
      if (!ts.isIdentifier(item.name) || declaration(item.name) !== item) return false;
      const joined = (node: ts.Expression): boolean => ts.isCallExpression(node) && exactImport(node.expression, "node:path", "join") && node.arguments.length === 2 && rootValue(node.arguments[0]!) && ts.isIdentifier(node.arguments[1]!) && symbol(node.arguments[1]!) === symbol(item.name);
      return calls.some((call) => {
        if (!contains(loop.statement, call) || block(call) !== loop.statement || !exactImport(call.expression, "node:fs", "readFileSync") || !call.arguments.length) return false;
        for (let parent = call.parent; parent !== loop.statement; parent = parent.parent) if (ts.isFunctionLike(parent)) return false;
        const argument = call.arguments[0]!;
        if (joined(argument)) return true;
        if (!ts.isIdentifier(argument)) return false;
        const alias = declaration(argument);
        return Boolean(alias?.initializer && block(alias) === loop.statement && alias.getStart(tree) < call.getStart(tree) && joined(alias.initializer));
      });
    });
    if (!reader) continue;
    for (const element of array.elements) if (ts.isStringLiteral(element) && plain(element)) result.push({ value: element.text, start: element.getStart(tree) + 1, end: element.end - 1, physicalTargets: [element.text] });
  }
  return result.sort((left, right) => left.start - right.start);
}

/** 🧬️ Extracts actual repository-owned declarations without importing workspace dispatch or reading any input consumer. */
function implementation(compiler: typeof compilers[number]) {
  const names = new Set(["lineLocation", "indexedLineContent", "indexedLineStarts", "regexTokens", "typescriptTokens", "ticketImportantProseReferenceAuthority", "typescriptLeadingDocumentationReferenceAuthority", "typescriptCommentPathReferenceAuthority", "dependencyCruiserBoundaryReferenceAuthority", "normalizeRelative", "sourceRelative", "splitTokenSuffix", "addUniqueIndex", "referencePathIndex", "ancestorReferenceCoordinateRoot", "resolveReferencePath", "resolveReferenceTokenPath", "rewriteReferenceValue"]);
  const declarations = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? names.has(node.name?.text ?? "") || /^typescriptCollection|^typescriptPathCollectionReferenceAuthority$/u.test(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((entry) => names.has(entry.name.getText(syntax))));
  if (!declarations.some((node) => ts.isFunctionDeclaration(node) && node.name?.text === "typescriptPathCollectionReferenceAuthority")) throw new Error("Missing immutable for-of path-collection authority");
  const code = declarations.map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  return new Function("posix", "dirname", "basename", compiler.compile(code) + "\nreturn { authority: typescriptPathCollectionReferenceAuthority, hasForOf: typescriptCollectionHasForOf, parse: typescriptTokens, index: referencePathIndex, resolve: resolveReferenceTokenPath, rewrite: rewriteReferenceValue };")(posix, dirname, basename);
}

test("ordered-word precheck stays bounded on large neutral source buffers", () => {
  const validate = new Ajv().compile(vector.scanBoundarySchema);
  const implementations = compilers.map((compiler) => ({ name: compiler.name, actual: implementation(compiler) }));
  for (const row of vector.scanBoundaryCases) {
    expect(validate(row), JSON.stringify(validate.errors)).toBe(true);
    const content = row.prefix + row.repeat.repeat(row.count) + row.suffix;
    const tree = ts.createSourceFile(row.id + ".ts", content, ts.ScriptTarget.Latest, true);
    let expected = false;
    const visit = (node: ts.Node): void => { if (ts.isForOfStatement(node)) expected = true; ts.forEachChild(node, visit); };
    visit(tree);
    expect((tree as ts.SourceFile & { parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics).toHaveLength(0);
    expect(expected, row.id).toBe(row.expected);
    for (const { name, actual } of implementations) {
      const before = digest(content), started = performance.now();
      const result = actual.hasForOf(content), milliseconds = performance.now() - started;
      expect(result, name + ":" + row.id).toBe(row.expected);
      expect(milliseconds, name + ":" + row.id).toBeLessThan(row.maxMilliseconds);
      expect(digest(content)).toBe(before);
      console.log("[DEBUG] Bounded for-of precheck", JSON.stringify({ compiler: name, case: row.id, bytes: Buffer.byteLength(content), milliseconds, result }));
    }
  }
});

test("neutral immutable-reader vectors agree with an independent TypeScript binding oracle", () => {
  expect(vector.contract).toBe("typescript-immutable-path-collection-for-of-v1");
  const validate = new Ajv().compile(vector.caseSchema);
  for (const row of vector.cases) { expect(validate(row), JSON.stringify(validate.errors)).toBe(true); expect(oracle(row.source), row.id).toEqual(row.expected); }
  expect(validate({ ...vector.cases[0], undeclared: true })).toBe(false);
  console.log("[DEBUG] Independent immutable collection oracle", JSON.stringify({ cases: vector.cases.length, positives: vector.cases.filter((row: any) => row.expected.length).length }));
});

test("opaque expressions and typed shadow bindings cannot authorize collection leaves", () => {
  const rows = vector.cases.filter((entry: any) => /^(?:template-expression|typed-function|mixed-default-named)/u.test(entry.id));
  for (const compiler of compilers) {
    const actual = implementation(compiler);
    expect(rows.map((row: any) => ({ id: row.id, spans: actual.authority(row.source).length }))).toEqual(rows.map((row: any) => ({ id: row.id, spans: 0 })));
  }
});

for (const compiler of compilers) test(compiler.name + " proves exact immutable reader spans without mutating source buffers", () => {
  const actual = implementation(compiler);
  for (const row of vector.cases) {
    const before = digest(row.source), tokens: Token[] = actual.authority(row.source);
    expect(tokens.map(({ value, start, end, physicalTargets }) => ({ value, start, end, physicalTargets })), row.id).toEqual(row.expected);
    for (const token of tokens) { expect(row.source.slice(token.start, token.end)).toBe(token.value); expect(token.adapter).toBe("typescript"); expect(token.structuredLocation).toStartWith("path-collection-for-of:"); }
    expect(digest(row.source)).toBe(before);
  }
});

test("the exact live mixed-template collection is admitted without reading or executing its inputs", () => {
  const path = join(root, vector.liveInput), bytes = readFileSync(path), content = bytes.toString("utf8"), expected = oracle(content);
  expect(expected).toHaveLength(2);
  expect(expected.some((row) => row.value.endsWith("/🧬️mutations/🦀️.rs"))).toBe(true);
  for (const compiler of compilers) expect(implementation(compiler).authority(content).map(({ value, start, end, physicalTargets }: Token) => ({ value, start, end, physicalTargets }))).toEqual(expected);
  expect(readFileSync(path).equals(bytes)).toBe(true);
});

test("rejected for-of proof cannot fall through the separate weak distant-map authority", () => {
  const rows = vector.cases.filter((entry: any) => entry.id.startsWith("rejected-for-of"));
  for (const compiler of compilers) for (const row of rows) expect(implementation(compiler).parse("reader.ts", row.source).filter((token: Token) => token.value === "🟦️targetsold.ts"), row.id).toEqual([]);
});

test("exact physical resolution and leaf rewriting remain separate from reader proof", () => {
  const row = vector.cases[0], oldTarget = row.expected[0].value, destination = "🟦️targets.ts";
  for (const compiler of compilers) {
    const actual = implementation(compiler), token = actual.authority(row.source)[0];
    expect(actual.resolve("consumerreader.ts", token, actual.index([]))).toBeNull();
    expect(actual.resolve("consumerreader.ts", token, actual.index(["🧪️consumer/" + oldTarget]))).toBeNull();
    expect(actual.resolve("consumerreader.ts", token, actual.index([oldTarget]))).toBe(oldTarget);
    const rewritten = actual.rewrite("consumerreader.ts", token.value, oldTarget, destination);
    expect(rewritten).toBe(destination);
    const changed = row.source.slice(0, token.start) + rewritten + row.source.slice(token.end);
    expect(oracle(changed).map((entry) => entry.value)).toEqual([destination]);
    expect(changed.slice(0, token.start)).toBe(row.source.slice(0, token.start));
    expect(changed.slice(token.start + rewritten.length)).toBe(row.source.slice(token.end));
  }
});

test("neutral map-only boundaries match independent AST for-of detection without fallback", () => {
  const validate = new Ajv().compile({ type: "object", required: ["id", "source", "expected"], additionalProperties: false, properties: { id: { type: "string" }, source: { type: "string" }, expected: { type: "array", items: { type: "string" } } } });
  for (const row of vector.mapBoundaryCases) {
    expect(validate(row)).toBe(true);
    const tree = ts.createSourceFile("boundary.ts", row.source, ts.ScriptTarget.Latest, true), literals: ts.StringLiteral[] = [];
    let forOf = false;
    const visit = (node: ts.Node): void => { if (ts.isForOfStatement(node)) forOf = true; if (ts.isStringLiteral(node) && node.text === "🟦️targetsold.ts") literals.push(node); ts.forEachChild(node, visit); };
    visit(tree);
    expect(forOf ? [] : literals.map((node) => node.text), row.id).toEqual(row.expected);
    for (const compiler of compilers) expect(implementation(compiler).parse("reader.ts", row.source).filter((token: Token) => token.value === "🟦️targetsold.ts").map((token: Token) => token.value), row.id).toEqual(row.expected);
  }
});

test("registers the kind-only canonical test through the package router and both launch catalogs", () => {
  const expected = vector.execution, project = JSON.parse(readFileSync(join(root, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  const router = readFileSync(join(root, library, "📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  expect(router.match(/segments\[0\] === "typescript-path-collection"/gu)).toHaveLength(1);
  expect(router).toContain(expected.source);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const configurations = parseJsonc(readFileSync(join(root, path), "utf8")).configurations;
    const rows = configurations.filter((row: any) => row.name === expected.launchName);
    expect(rows).toHaveLength(1);
    expect(rows[0].command).toBe(expected.launchCommand);
    expect(rows[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
    expect(configurations.filter((row: any) => row.presentation?.group === expected.launchGroup && row.presentation?.order === expected.launchOrder)).toHaveLength(1);
  }
});
