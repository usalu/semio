import { expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import { fileURLToPath } from "node:url";
import { ecmaIdentifierValue, ecmaProgram, ecmaStringValue, ecmaTokens, type EcmaToken } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import examples from "../🧫️fixtures/🔣️.json";

const admit = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/EcmaToken`)!;

test("ECMA original source spans and decoded literal examples agree with independent TypeScript syntax", () => {
  for (const row of examples.strings) {
    const value = ecmaStringValue({ kind: "string", text: row.source, start: 0, end: row.source.length });
    expect(value, row.source).toBe(row.expected);
    if (row.expected === null) continue;
    const parsed = ts.createSourceFile("literal.ts", `const value = ${row.source};`, ts.ScriptTarget.Latest, true);
    const declaration = (parsed.statements[0] as ts.VariableStatement).declarationList.declarations[0]!;
    expect(ts.isStringLiteral(declaration.initializer!), row.source).toBe(true);
    expect((declaration.initializer as ts.StringLiteral).text, row.source).toBe(value);
  }
  for (const row of examples.sources) {
    const parsed = ts.createSourceFile("reader.ts", row.source, ts.ScriptTarget.Latest, true), expected: { kind: string; text: string; start: number; end: number; value?: string }[] = [];
    const visit = (node: ts.Node): void => {
      if (ts.isStringLiteral(node) || ts.isRegularExpressionLiteral(node)) expected.push({ kind: ts.isStringLiteral(node) ? "string" : "regex", text: node.getText(parsed), start: node.getStart(parsed), end: node.end, ...(ts.isStringLiteral(node) ? { value: node.text } : {}) });
      ts.forEachChild(node, visit);
    };
    visit(parsed);
    const found: typeof expected = [];
    const inspect = (tokens: readonly EcmaToken[]): void => {
      for (const token of tokens) {
        expect(admit(token), JSON.stringify(admit.errors)).toBe(true);
        expect(row.source.slice(token.start, token.end), row.id).toBe(token.text);
        if (token.kind === "string" || token.kind === "regex") found.push({ kind: token.kind, text: token.text, start: token.start, end: token.end, ...(token.kind === "string" ? { value: ecmaStringValue(token)! } : {}) });
        for (const span of token.expressions ?? []) {
          expect(row.source.slice(span.start, span.end), row.id).toBe(span.text);
          inspect(ecmaTokens(span.text, span.start));
        }
      }
    };
    inspect(ecmaTokens(row.source));
    expect(found.sort((a, b) => a.start - b.start), row.id).toEqual(expected.sort((a, b) => a.start - b.start));
    expect(found.filter(row => row.kind === "string").map(row => row.value), row.id).toEqual(row.literalValues);
  }
  console.log("[DEBUG] original ECMA UTF16 spans, escaped literals, regex custody and nested template expressions match independent TypeScript");
});

test("ECMA malformed input retains original offending spans and terminal offset", () => {
  for (const source of examples.invalidSources) {
    const tokens = ecmaTokens(source);
    expect(tokens.some(token => token.kind === "invalid"), source).toBe(true);
    expect(tokens.at(-1)).toEqual({ kind: "eof", text: "", start: source.length, end: source.length });
    for (const token of tokens) expect(source.slice(token.start, token.end)).toBe(token.text);
  }
});

test("ECMA original atomic source owners observe cancellation during expensive scans", () => {
  for (const row of examples.cancellationAtoms) {
    const source = row.prefix + row.unit.repeat(row.repeat) + row.suffix, marker = new Error(row.id);
    const parsed = ts.createSourceFile("atom.ts", source, ts.ScriptTarget.Latest, true);
    expect(parsed.end).toBe(source.length);
    let turns = 0;
    expect(() => ecmaTokens(source, 0, () => { if (++turns === 3) throw marker; })).toThrow(marker);
    expect(turns, row.id).toBe(3);
  }
  console.log("[DEBUG] original ECMA atomic literal, identifier and comment scans honor caller cancellation");
});


test("ECMA binding identities and original syntax-tree call ranges match independent TypeScript", () => {
  for (const row of examples.identifiers) {
    expect(ecmaIdentifierValue({ kind: "identifier", text: row.source, start: 0, end: row.source.length }), row.source).toBe(row.expected);
    if (row.expected === null) continue;
    const file = ts.createSourceFile("binding.ts", `const ${row.source} = 1;`, ts.ScriptTarget.Latest, true);
    expect(((file.statements[0] as ts.VariableStatement).declarationList.declarations[0]!.name as ts.Identifier).text, row.source).toBe(row.expected);
  }
  const admitStatement = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/EcmaStatement`)!;
  for (const row of examples.programs) {
    const file = ts.createSourceFile("reader.ts", row.source, ts.ScriptTarget.Latest, true), expected: { text: string; start: number; end: number }[] = [];
    const inspectOracle = (node: ts.Node): void => { if (ts.isCallExpression(node)) expected.push({ text: node.getText(file), start: node.getStart(file), end: node.end }); ts.forEachChild(node, inspectOracle); };
    inspectOracle(file);
    expect((file as ts.SourceFile & { parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics, row.id).toEqual([]);
    const program = ecmaProgram(row.source), actual: typeof expected = [];
    expect(program, row.id).not.toBeNull();
    const inspect = (value: unknown): void => {
      if (!value || typeof value !== "object") return;
      if (Array.isArray(value)) { value.forEach(inspect); return; }
      const node = value as { kind?: string; start?: number; end?: number };
      if (node.kind === "call") actual.push({ text: row.source.slice(node.start!, node.end!), start: node.start!, end: node.end! });
      Object.values(value).forEach(inspect);
    };
    for (const statement of program!) expect(admitStatement(statement), row.id + JSON.stringify(admitStatement.errors)).toBe(true);
    inspect(program);
    if (row.id === "escaped-template-binding") {
      const template = program![1]!.declarations![0]!.initializer, call = template.expressions![0]!;
      expect(call.callee!.name).toBe(((file.statements[1] as ts.VariableStatement).declarationList.declarations[0]!.initializer as ts.TemplateExpression).templateSpans[0]!.expression.getText(file).startsWith("\\u0072ead") ? "read" : "unresolved");
    }
    if (row.id === "async-test-callback") expect(program![0]!.expression!.arguments![1]!.async).toBe(true);
    if (row.id === "selected-local-ref") {
      const call = program![0]!.expression!.callee!.arguments![0]!.properties![0]!.value;
      expect(row.source.slice(call.start, call.end)).toBe(call.value!);
    }
    expect(actual.sort((a, b) => a.start - b.start || b.end - a.end), row.id).toEqual(expected.sort((a, b) => a.start - b.start || b.end - a.end));
  }
  console.log("[DEBUG] original ECMA syntax-tree call spans and decoded escaped binding identities match independent TypeScript");
});


test("ECMA first-party parser and defining syntax formats satisfy strict independent TypeScript", () => {
  const source = fileURLToPath(new URL("../🟦️.ts", import.meta.url));
  const program = ts.createProgram([source], { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.Preserve, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noUncheckedIndexedAccess: true, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: [], lib: ["lib.esnext.d.ts"] });
  const diagnostics = ts.getPreEmitDiagnostics(program);
  expect(diagnostics.map(row => ({ code: row.code, path: row.file?.fileName, start: row.start, message: ts.flattenDiagnosticMessageText(row.messageText, "\n") }))).toEqual([]);
  console.log("[DEBUG] actual first-party ECMA implementation and defining formats compile under strict independent TypeScript");
});


test("ECMA parser preserves original statement boundaries and refuses malformed bindings", () => {
  for (const row of examples.syntaxBoundaries) {
    const file = ts.createSourceFile("boundary.ts", row.source, ts.ScriptTarget.Latest, true), diagnostics = (file as ts.SourceFile & { parseDiagnostics: readonly ts.Diagnostic[] }).parseDiagnostics, program = ecmaProgram(row.source);
    const options = { noLib: true, noResolve: true, noEmit: true }, host = ts.createCompilerHost(options);
    host.getSourceFile = path => path === "boundary.ts" ? file : undefined;
    const oracle = ts.createProgram(["boundary.ts"], options, host), grammar = oracle.getSemanticDiagnostics(file).filter(diagnostic => diagnostic.code === 1142);
    expect(diagnostics.length + grammar.length === 0, row.id).toBe(row.valid);
    if (!row.valid) { expect(program, row.id).toBeNull(); continue; }
    expect(program, row.id).not.toBeNull();
    if (row.id === "return-line") {
      expect(program![0]!.body!.map(node => node.kind)).toEqual(["return", "expression"]);
      expect(program![0]!.body![0]!.expression).toBeUndefined();
      expect(program![0]!.body![0]!.end).toBe((file.statements[0] as ts.FunctionDeclaration).body!.statements[0]!.end);
    }
    if (row.id === "spread-range") {
      const spread = program![0]!.expression!.arguments![0]!, oracle = (file.statements[0] as ts.ExpressionStatement).expression as ts.CallExpression;
      expect({ start: spread.start, end: spread.end }).toEqual({ start: oracle.arguments[0]!.getStart(file), end: oracle.arguments[0]!.end });
    }
    if (row.id === "empty-statements") expect(program![0]!.start).toBe(file.statements[2]!.getStart(file));
  }
  console.log("[DEBUG] original ECMA statement, spread and line-terminator boundaries agree with independent TypeScript; malformed bindings remain unresolved");
});
