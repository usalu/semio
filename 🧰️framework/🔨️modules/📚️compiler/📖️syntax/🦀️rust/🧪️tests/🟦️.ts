import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import ts from "typescript";

const owner = resolve(import.meta.dir, "..");
const contract = JSON.parse(readFileSync(resolve(owner, "🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(owner, "🧬️schema/🔣️.json"), "utf8"));
async function api() { const module = await import(resolve(owner, "🟦️.ts")); expect(typeof module.inspectRustCompileReferences).toBe("function"); return module; }

test("closed language-neutral original syntax corpus has independent AJV admission", () => {
  const validate = new Ajv({ strict: true }).addSchema(contract).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  for (const rows of [fixture.cases, fixture.unsupported, fixture.oracle]) expect(new Set(rows.map((row: { id: string }) => row.id)).size).toBe(rows.length);
});

test("canonical neutral owner preserves all original syntax outputs and fail-closed refusals", async () => {
  const module = await api();
  const validate = new Ajv({ strict: true }).addSchema(contract).compile({ type: "array", items: { $ref: contract.$id + "#/$defs/reference" } });
  for (const row of fixture.cases) {
    const actual = module.inspectRustCompileReferences(row.source);
    expect(actual).toEqual(row.references);
    expect(validate(actual)).toBe(true);
  }
  for (const row of fixture.unsupported) expect(() => module.inspectRustCompileReferences(row.source)).toThrow("Unsupported Rust compile");
});

test("literal include syntax agrees with independent installed tree-sitter Rust grammar", async () => {
  const module = await api();
  await Parser.init({ locateFile: () => resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  try {
    for (const row of fixture.oracle) {
      const tree = parser.parse(row.source);
      try {
        expect(tree.rootNode.hasError()).toBe(false);
        const independent = tree.rootNode.descendantsOfType("macro_invocation").flatMap(node => {
          const kind = node.childForFieldName("macro")?.text;
          if (!["include", "include_str", "include_bytes"].includes(kind ?? "")) return [];
          const argument = node.namedChildren.find(child => child.type === "token_tree")!.namedChildren[0]!;
          const text = argument.text;
          const path = argument.type === "raw_string_literal" ? text.slice(text.indexOf('"') + 1, text.lastIndexOf('"')) : JSON.parse(text);
          return [{ kind, path, line: node.startPosition.row + 1 }];
        });
        expect(independent).toEqual(row.references);
        expect(module.inspectRustCompileReferences(row.source)).toEqual(independent);
      } finally { tree.delete(); }
    }
  } finally { parser.delete(); }
});

test("neutral syntax public API typechecks without any product or foreign runtime", async () => {
  await api();
  const path = resolve(owner, "🟦️.ts"), source = readFileSync(path, "utf8");
  const parsed = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
  const imports = parsed.statements.filter(ts.isImportDeclaration).map(node => (node.moduleSpecifier as ts.StringLiteral).text);
  expect(imports.every(value => value === "node:path" || value.startsWith(".") && !value.includes("🛍️products") && !value.includes("🌎️hub") && !value.includes("✏️s"))).toBe(true);
  const program = ts.createProgram([path], { noEmit: true, strict: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true });
  expect(ts.getPreEmitDiagnostics(program).map(value => ts.flattenDiagnosticMessageText(value.messageText, "\n"))).toEqual([]);
});

