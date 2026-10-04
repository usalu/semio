import assert from "node:assert/strict";
import { dirname, join, relative, resolve } from "node:path";
import { readFileSync } from "node:fs";
import { parse as parseToml } from "@iarna/toml";
import Parser from "web-tree-sitter";

type Names = { file: string; names: string[] };
export type ConsumerProbe =
 | ({ kind: "public-export" | "private-import"; namespace: string } & Names)
 | ({ kind: "retired-public" | "retired-reexport" | "retired-laws" } & Names)
 | { kind: "qualified-use"; file: string; name: string; expected: string[]; caller: string; modules: string[] }
 | { kind: "normal-dependency"; file: string; key: string; package: string; manifest: string };
export type ConsumerFixture = { version: 1; owner: string; cases: ConsumerProbe[] };

/** 🧩️ Validates source-owned consumer declarations against closed first-party probes. */
export async function inspectCommandIngressConsumer(repository: string, fixture: ConsumerFixture, read: (path: string) => string = path => readFileSync(join(repository, path), "utf8")): Promise<{ owner: string; cases: number }> {
 await Parser.init();
 const parser = new Parser(), trees: Parser.Tree[] = [];
 parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", repository)), "out/tree-sitter-rust.wasm")));
 const nodes = (node: Parser.SyntaxNode): Parser.SyntaxNode[] => [node, ...node.children.flatMap(nodes)], ast = (path: string, complete: boolean) => { const tree = parser.parse(read(path)); trees.push(tree); if (complete) assert.equal(tree.rootNode.hasError(), false, `Rust parser refused ${path}`); return tree.rootNode; }, publicNode = (node: Parser.SyntaxNode) => node.namedChildren.some(child => child.type === "visibility_modifier" && child.text === "pub");
 const tokens = (node: Parser.SyntaxNode): string[] => ["line_comment", "block_comment"].includes(node.type) ? [] : node.children.length ? node.children.flatMap(tokens) : [node.text];
 try {
  for (const probe of fixture.cases) {
   if (probe.kind === "normal-dependency") {
    const source = read(probe.file), first = parseToml(source), second = Bun.TOML.parse(source); assert.deepEqual(first, second);
    const dependencies = first.dependencies as Record<string, { package?: string; path?: string }>, edge = dependencies[probe.key]; assert.ok(edge); assert.equal(edge.package, probe.package); assert.equal(typeof edge.path, "string"); assert.equal(relative(repository, resolve(repository, dirname(probe.file), edge.path!, "Cargo.toml")).split("\\").join("/"), probe.manifest);
    continue;
   }
   const root = ast(probe.file, probe.kind !== "qualified-use"), all = nodes(root);
   switch (probe.kind) {
    case "public-export":
    case "private-import": {
     const prefix = [...(probe.kind === "public-export" ? ["pub"] : []), "use", ...probe.namespace.split("::").flatMap((name, index) => [...(index ? ["::"] : []), ...(name ? [name] : [])]), "::", "{"], declarations = all.filter(node => node.type === "use_declaration" && prefix.every((token, index) => tokens(node)[index] === token)); assert.equal(declarations.length, 1);
     const actual = tokens(declarations[0]!).slice(prefix.length); if (actual.at(-3) === ",") actual.splice(-3, 1); assert.deepEqual(actual, [...probe.names.flatMap((name, index) => [...(index ? [","] : []), name]), "}", ";"]);
     break;
    }
    case "retired-public": {
     const names = root.namedChildren.filter(node => ["struct_item", "enum_item", "type_item", "const_item"].includes(node.type) && publicNode(node)).map(node => node.childForFieldName("name")!.text), exported = all.filter(node => node.type === "use_declaration" && publicNode(node)).flatMap(tokens); assert.deepEqual([...names, ...exported].filter(name => probe.names.includes(name)), []); break;
    }
    case "retired-reexport": {
     const words = all.filter(node => node.type === "use_declaration" && publicNode(node)).flatMap(tokens); assert.deepEqual(words.filter(name => probe.names.includes(name)), []); break;
    }
    case "retired-laws": {
     const names = all.filter(node => node.type === "function_item").map(node => node.childForFieldName("name")!.text); assert.deepEqual(names.filter(name => probe.names.includes(name)), []); break;
    }
    case "qualified-use": {
     const callers = all.filter(node => node.type === "function_item" && node.childForFieldName("name")?.text === probe.caller); assert.equal(callers.length, 1); const caller = callers[0]!, modules: Parser.SyntaxNode[] = []; let parent = caller.parent; while (parent) { if (parent.type === "mod_item") modules.unshift(parent); parent = parent.parent; } assert.deepEqual(modules.map(node => node.childForFieldName("name")!.text), probe.modules);
     for (const scope of [caller, ...modules]) { const body = scope.childForFieldName("body"); assert.ok(body); for (const child of scope.children.filter(node => node.id !== body.id)) assert.equal(child.hasError() || child.isMissing(), false); }
     const errors = all.filter(node => node.type === "ERROR" || node.isMissing()), uses = all.filter(node => ["scoped_identifier", "scoped_type_identifier"].includes(node.type) && node.childForFieldName("name")?.text === probe.name); assert.deepEqual(uses.map(node => tokens(node).join("")), probe.expected);
     for (const use of uses) { assert.equal(use.hasError() || use.isMissing(), false); assert.equal(errors.some(error => error.startIndex < use.endIndex && error.endIndex >= use.startIndex), false); let enclosing = use.parent; while (enclosing && enclosing.type !== "function_item") enclosing = enclosing.parent; assert.equal(enclosing?.id, caller.id); }
     break;
    }
   }
  }
  return { owner: fixture.owner, cases: fixture.cases.length };
 } finally { for (const tree of trees) tree.delete(); parser.delete(); }
}
