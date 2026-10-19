/** 🧪️ Validates module tree reading against the installed Rust grammar, the Rust compiler and the TypeScript parser on closed fixtures. */
import { afterAll, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, normalize, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import fastGlob from "fast-glob";
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
  const ids = [...fixture.mounts, ...fixture.includes, ...fixture.trees, ...fixture.refusals].map((row: { id: string }) => row.id);
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

test("own include items match the independent installed Rust grammar", async () => {
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
    for (const row of fixture.includes) {
      const tree = parser.parse(row.source);
      try {
        expect(tree.rootNode.hasError()).toBe(false);
        const nesting = (node: Parser.SyntaxNode): string[] | undefined => {
          const modules: string[] = [];
          for (let ancestor = node.parent; ancestor && ancestor.type !== "source_file"; ancestor = ancestor.parent) {
            if (ancestor.type === "declaration_list" || ancestor.type === "expression_statement") continue;
            if (ancestor.type !== "mod_item") return undefined;
            modules.unshift(pathOf(ancestor) ?? ancestor.childForFieldName("name")!.text);
          }
          return modules;
        };
        const mounts = tree.rootNode.descendantsOfType("mod_item").flatMap(node => {
          const mount = node.childForFieldName("body") ? undefined : pathOf(node), modules = mount === undefined ? undefined : nesting(node);
          return mount === undefined || modules === undefined ? [] : [{ path: mount, modules, via: "path", offset: node.startIndex }];
        });
        const includes = tree.rootNode.descendantsOfType("macro_invocation").flatMap(node => {
          const literal = node.namedChildren.find(child => child.type === "token_tree")?.namedChildren ?? [], modules = nesting(node);
          if (node.childForFieldName("macro")?.text !== "include" || literal.length !== 1 || literal[0]!.type !== "string_literal" || modules === undefined || node.parent?.type === "let_declaration") return [];
          return [{ path: literal[0]!.text.slice(1, -1), modules, via: "include", offset: node.startIndex }];
        });
        const independent = [...mounts, ...includes].sort((a, b) => a.offset - b.offset).map(({ path, modules, via }) => ({ path, modules, via }));
        expect(independent).toEqual(row.expected);
        expect(module.inspectRustModuleChildren(row.source)).toEqual(independent);
        expect(module.inspectRustPathMounts(row.source)).toEqual(independent.filter(child => child.via === "path").map(({ path, modules }) => ({ path, modules })));
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
    const compiler = readFileSync(dependencies, "utf8").split("\n").flatMap(line => /^(.+):$/u.exec(line)?.[1] ?? []).map(path => normalize(path)).sort();
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

test("facades, hubs and roots of the renderer partition pull in the crates they name, and crate members never do", async () => {
  const module = await api();
  await Parser.init({ locateFile: () => resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  const crate = (name: string) => `[package]\nname = "semio-framework-os-renderer-wgpu-${name}"\n`;
  const files = {
    "engine/🎯️targets/🧊️wgpu/🔧️alpha/📦️packages/🦀️rust/Cargo.toml": crate("alpha"),
    "engine/🎯️targets/🧊️wgpu/🔧️alpha/🦀️.rs": '#[path = "../../../🧱️elements/A/🎯️targets/🧊️wgpu/🦀️.rs"]\npub mod member_a;\nextern crate semio_framework_os_renderer_wgpu_beta as beta;\n',
    "engine/🎯️targets/🧊️wgpu/🔨️beta/📦️packages/🦀️rust/Cargo.toml": crate("beta"),
    "engine/🎯️targets/🧊️wgpu/🔨️beta/🦀️.rs": '#[path = "../../../🧱️elements/B/🎯️targets/🧊️wgpu/🦀️.rs"]\npub mod member_b;\n',
    "engine/🧱️elements/A/🎯️targets/🧊️wgpu/🦀️.rs": "use semio_framework_os_renderer_wgpu_beta::member_b;\n",
    "engine/🧱️elements/B/🎯️targets/🧊️wgpu/🦀️.rs": "pub fn member_b() {}\n",
    "engine/🧱️elements/F/🎯️targets/🧊️wgpu/🦀️.rs": 'use semio_framework_os_renderer_wgpu_alpha::member_a;\npub use ::semio_framework_os_renderer_wgpu_beta::member_b as other;\nuse std::fmt;\nmod inline { use semio_framework_os_renderer_wgpu_alpha::x; }\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n',
    "engine/🧱️elements/F/🎯️targets/🧊️wgpu/🧪️tests/🦀️.rs": "// tests\n",
  };
  const base = materialize("renderer-partition-crates", files), facade = join(base, "engine/🧱️elements/F/🎯️targets/🧊️wgpu/🦀️.rs");
  const rel = (list: readonly string[]) => list.map(file => file.slice(base.length + 1));
  const source = files["engine/🧱️elements/F/🎯️targets/🧊️wgpu/🦀️.rs"];
  expect(module.inspectRustCrateReferences(source)).toEqual(["semio_framework_os_renderer_wgpu_alpha", "semio_framework_os_renderer_wgpu_beta", "std"]);
  const tree = parser.parse(source);
  try {
    const independent = tree.rootNode.children.flatMap(node => node.type === "use_declaration" ? [/(?:::)?([A-Za-z_][A-Za-z0-9_]*)/u.exec(node.childForFieldName("argument")!.text)![1]!] : []);
    expect(independent).toEqual(module.inspectRustCrateReferences(source));
  } finally { tree.delete(); parser.delete(); }
  expect(rel(module.listModuleTree(facade))).toEqual([
    "engine/🧱️elements/F/🎯️targets/🧊️wgpu/🦀️.rs",
    "engine/🎯️targets/🧊️wgpu/🔧️alpha/🦀️.rs",
    "engine/🧱️elements/A/🎯️targets/🧊️wgpu/🦀️.rs",
    "engine/🎯️targets/🧊️wgpu/🔨️beta/🦀️.rs",
    "engine/🧱️elements/B/🎯️targets/🧊️wgpu/🦀️.rs",
  ]);
  expect(rel(module.listModuleTree(facade, { crates: false }))).toEqual(["engine/🧱️elements/F/🎯️targets/🧊️wgpu/🦀️.rs"]);
  expect(rel(module.listModuleTree(join(base, "engine/🧱️elements/A/🎯️targets/🧊️wgpu/🦀️.rs")))).toEqual(["engine/🧱️elements/A/🎯️targets/🧊️wgpu/🦀️.rs"]);
  expect(rel(module.listModuleTree(join(base, "engine/🎯️targets/🧊️wgpu/🔧️alpha/🦀️.rs")))).toEqual(["engine/🎯️targets/🧊️wgpu/🔧️alpha/🦀️.rs", "engine/🧱️elements/A/🎯️targets/🧊️wgpu/🦀️.rs"]);
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

test("source family reading lists every authored Rust file below a directory except canonical test folders, as an independent glob does", async () => {
  const module = await api();
  const base = materialize("family", { "a/🦀️.rs": "a", "a/b/🦀️.rs": "b\n", "a/b/c.txt": "x", "a/🧪️tests/🦀️.rs": "t", "a/b/🧪️testing/🦀️.rs": "u", "a/node_modules/🦀️.rs": "n", "z/🦀️.rs": "z" });
  const oracle = fastGlob.sync("**/*.rs", { cwd: join(base, "a"), absolute: true, ignore: ["**/🧪️tests/**", "**/🧪️testing/**", "**/node_modules/**"] }).map(path => resolve(path)).sort();
  const listed: string[] = [...module.listSourceFamily(join(base, "a"))];
  expect([...listed].sort()).toEqual(oracle);
  expect(listed).toHaveLength(2);
  expect(module.readSourceFamilySource(join(base, "a"))).toBe(listed.map(path => readFileSync(path, "utf8")).map(text => text.endsWith("\n") ? text : text + "\n").join(""));
  expect(module.readSourceFamilySource(pathToFileURL(join(base, "a") + "/"))).toBe(module.readSourceFamilySource(join(base, "a")));
});
