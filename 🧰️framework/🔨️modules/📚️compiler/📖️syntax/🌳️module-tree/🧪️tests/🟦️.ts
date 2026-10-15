/** 🧪️ Validates module tree reading against the installed Rust grammar, the Rust compiler and the TypeScript parser on closed fixtures. */
import { afterAll, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import Parser from "web-tree-sitter";
import ts from "typescript";

const owner = resolve(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(resolve(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
const scratch = mkdtempSync(join(tmpdir(), "semio-module-tree-"));
afterAll(() => rmSync(scratch, { recursive: true, force: true }));
const api = async () => await import(resolve(owner, "🟦️.ts"));

function materialize(id: string, files: Record<string, string>): string {
  const base = join(scratch, id);
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(dirname(join(base, path)), { recursive: true });
    writeFileSync(join(base, path), text);
  }
  return base;
}

const joined = (base: string, paths: readonly string[]) => paths.map(path => readFileSync(join(base, path), "utf8")).map(text => text.endsWith("\n") ? text : text + "\n").join("");

test("fixture identities are unique", () => {
  const ids = [...fixture.mounts, ...fixture.trees, ...fixture.refusals].map((row: { id: string }) => row.id);
  expect(new Set(ids).size).toBe(ids.length);
});

test("own path mounts match the independent installed Rust grammar", async () => {
  const module = await api();
  await Parser.init({ locateFile: () => resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  const pathOf = (node: Parser.SyntaxNode): string | undefined => {
    const attributes: string[] = [];
    for (let sibling = node.previousSibling; sibling && ["attribute_item", "line_comment", "block_comment"].includes(sibling.type); sibling = sibling.previousSibling) if (sibling.type === "attribute_item") attributes.push(sibling.text);
    return attributes.map(text => /^#\[\s*path\s*=\s*"((?:[^"\\]|\\.)*)"\s*\]$/u.exec(text)?.[1]).find(value => value !== undefined);
  };
  try {
    for (const row of fixture.mounts) {
      const tree = parser.parse(row.source);
      try {
        expect(tree.rootNode.hasError()).toBe(false);
        const independent = tree.rootNode.descendantsOfType("mod_item").flatMap(node => {
          if (node.childForFieldName("body")) return [];
          const mount = pathOf(node);
          if (mount === undefined) return [];
          const modules: string[] = [];
          for (let ancestor = node.parent; ancestor && ancestor.type !== "source_file"; ancestor = ancestor.parent) {
            if (ancestor.type === "declaration_list") continue;
            if (ancestor.type !== "mod_item") return [];
            modules.unshift(pathOf(ancestor) ?? ancestor.childForFieldName("name")!.text);
          }
          return [{ path: mount, modules, offset: node.startIndex }];
        }).sort((a, b) => a.offset - b.offset).map(({ path, modules }) => ({ path, modules }));
        expect(independent).toEqual(row.expected);
        expect(module.inspectRustPathMounts(row.source)).toEqual(independent);
      } finally { tree.delete(); }
    }
  } finally { parser.delete(); }
});

test("rust trees list children in declaration order and agree with the Rust compiler", async () => {
  const module = await api();
  for (const row of fixture.trees.filter((tree: { kind: string }) => tree.kind === "rust")) {
    const base = materialize(row.id, row.files), root = join(base, row.root);
    expect(module.listModuleTree(root).map((file: string) => file.slice(base.length + 1))).toEqual(row.production);
    expect(module.listModuleTree(pathToFileURL(root), { tests: true }).map((file: string) => file.slice(base.length + 1))).toEqual(row.withTests);
    expect(module.readModuleTreeSource(root)).toBe(joined(base, row.production));
    expect(module.readModuleTreeSource(root, { tests: true })).toBe(joined(base, row.withTests));
    for (const slice of row.slices ?? []) expect(module.sliceModuleTreeSource(root, slice.from, slice.to, { tests: slice.tests }), slice.id).toBe(slice.expected);
    if (row.oracle !== "rustc") continue;
    const dependencies = join(base, "oracle.d"), run = Bun.spawnSync(["rustc", "--edition=2021", "--crate-type=lib", "--crate-name=module_tree_oracle", "--test", "--emit=dep-info=" + dependencies, "-A", "warnings", row.root], { cwd: base });
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    const compiler = readFileSync(dependencies, "utf8").split("\n").flatMap(line => /^(.+):$/u.exec(line)?.[1] ?? []).sort();
    expect(compiler).toEqual([...row.withTests].sort());
  }
});

test("ecma trees follow relative child imports below the declaring directory and agree with the TypeScript parser", async () => {
  const module = await api();
  for (const row of fixture.trees.filter((tree: { kind: string }) => tree.kind === "ecma")) {
    const base = materialize(row.id, row.files), root = join(base, row.root);
    expect(module.listModuleTree(root).map((file: string) => file.slice(base.length + 1))).toEqual(row.production);
    expect(module.listModuleTree(root, { tests: true }).map((file: string) => file.slice(base.length + 1))).toEqual(row.withTests);
    expect(module.readModuleTreeSource(root)).toBe(joined(base, row.production));
    for (const [path, text] of Object.entries(row.files as Record<string, string>).filter(([path]) => /\.tsx?$/u.test(path))) {
      const source = ts.createSourceFile(path, text, ts.ScriptTarget.ES2022, true, path.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
      const independent = source.statements.flatMap(statement => (ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) && statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier) && /^\.\.?\//u.test(statement.moduleSpecifier.text) ? [statement.moduleSpecifier.text] : []);
      expect(module.inspectEcmaRelativeImports(text)).toEqual(independent);
    }
  }
});

test("an injected reader serves every file of the tree", async () => {
  const module = await api();
  const row = fixture.trees.find((tree: { id: string }) => tree.id === "rust-nested-children-in-declaration-order");
  const base = materialize("injected-reader", row.files), seen: string[] = [];
  const text = module.readModuleTreeSource(join(base, row.root), { read: (path: string) => { seen.push(path.slice(base.length + 1)); return "// served\n" + readFileSync(path, "utf8"); } });
  expect([...new Set(seen)].sort()).toEqual([...row.production].sort());
  expect(text.split("// served\n").length - 1).toBe(row.production.length);
});

test("module tree reading fails closed on missing mounts, ambiguity and foreign roots", async () => {
  const module = await api();
  for (const row of fixture.refusals) {
    const base = materialize(row.id, row.files);
    expect(() => row.slice ? module.sliceModuleTreeSource(join(base, row.root), row.slice.from, row.slice.to) : module.readModuleTreeSource(join(base, row.root))).toThrow(row.error);
  }
});

test("module tree reader owns its types and has a strict neutral source graph", () => {
  const path = resolve(owner, "🟦️.ts");
  const program = ts.createProgram([path], { noEmit: true, strict: true, skipLibCheck: true, types: ["node"], target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true });
  expect(ts.getPreEmitDiagnostics(program).map(value => ts.flattenDiagnosticMessageText(value.messageText, "\n"))).toEqual([]);
}, 30_000);
