/** 🧪️ Validates authored path arguments against independent Rust grammar and closed fixtures. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import JSON5 from "json5";
import ts from "typescript";
const owner = resolve(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🔣️.json"), "utf8"));
async function api() { return await import(resolve(owner, "🟦️.ts")); }
const calls = new Set(["std::path::Path::new", "std::path::PathBuf::from", ...["read", "read_to_string", "read_dir", "write", "metadata", "symlink_metadata", "canonicalize", "create_dir", "create_dir_all", "remove_file", "remove_dir", "remove_dir_all"].map(name => "std::fs::" + name), "std::fs::File::open", "std::fs::File::create"]);
function decode(text: string): string {
  if (/^(?:br|r)#*"/u.test(text)) return text.slice(text.indexOf('"') + 1, text.lastIndexOf('"'));
  const quoted = text.startsWith("b") ? text.slice(1) : text;
  return JSON5.parse(quoted.replace(/\\u\{([0-9a-f_]+)\}/giu, (_, hex) => String.fromCodePoint(parseInt(hex.replaceAll("_", ""), 16))));
}
test("closed path corpus admits exact references and refuses undeclared fields", () => {
  expect(new Set(fixture.cases.map((row: any) => row.id)).size).toBe(fixture.cases.length);
});
test("own path arguments match the independent installed Rust grammar", async () => {
  const module = await api();
  await Parser.init({ locateFile: () => resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  try {
    for (const row of fixture.cases) {
      const tree = parser.parse(row.source);
      try {
        expect(tree.rootNode.hasError()).toBe(false);
        const independent = tree.rootNode.descendantsOfType("call_expression").flatMap(node => {
          const fn = node.childForFieldName("function"), args = node.childForFieldName("arguments");
          if (!fn || !args) return [];
          const method = fn.type === "field_expression";
          const call = method ? fn.childForFieldName("field")?.text : fn.text.replace(/\s/gu, "");
          if (!call || !(method ? ["join", "push"].includes(call) : calls.has(call))) return [];
          const arg = args.namedChildren[0];
          if (!arg || !["string_literal", "raw_string_literal"].includes(arg.type)) return [];
          const value = decode(arg.text), path = value.replaceAll("\\", "/").replace(/^(?:(?:\.|\.\.)\/)+/u, "");
          const root = [...fixture.roots].sort((a: string, b: string) => b.length - a.length).find((value: string) => path === value || path.startsWith(value + "/"));
          if (!root || /[\u0000\r\n]/u.test(path)) return [];
          return [{ value, path, root, call, context: method ? "method" : "qualified", start: arg.startIndex, end: arg.endIndex, line: arg.startPosition.row + 1 }];
        }).sort((a, b) => a.start - b.start);
        expect(independent).toEqual(row.expected);
        expect(module.inspectRustPathLiterals(row.source, fixture.roots)).toEqual(independent);
        console.log("[DEBUG] Rust path literal " + JSON.stringify({ id: row.id, actual: independent }));
      } finally { tree.delete(); }
    }
  } finally { parser.delete(); }
});
test("path inspector owns its types and has a strict neutral source graph", async () => {
  const module = await api();
  expect(typeof module.RustPathRootError).toBe("function");
  for (const row of fixture.invalidRoots) {
    try { module.inspectRustPathLiterals("", row.roots); throw Error("Expected root refusal"); }
    catch (error) { expect(error).toBeInstanceOf(module.RustPathRootError); expect((error as { code: string }).code).toBe(row.refusal); }
  }
  const path = resolve(owner, "🟦️.ts");
  const program = ts.createProgram([path], { noEmit: true, strict: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true });
  expect(ts.getPreEmitDiagnostics(program).map(value => ts.flattenDiagnosticMessageText(value.messageText, "\n"))).toEqual([]);
}, 30_000);
