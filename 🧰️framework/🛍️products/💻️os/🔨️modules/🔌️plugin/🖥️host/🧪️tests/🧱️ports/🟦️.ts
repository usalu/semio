import { test } from "bun:test";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import Parser from "web-tree-sitter";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

const owner = resolve(import.meta.dir, "../..");

/** 🧱️ Reads direct third-party type paths from public Rust signatures with the independent grammar. */
function foreignTypes(parser: Parser, source: string): string[] {
  const tree = parser.parse(source);
  assert(tree && !tree.rootNode.namedChildren.some(node => node.type === "ERROR"), "complete Rust port declarations");
  try {
    const publicItem = (node: Parser.SyntaxNode) => node.namedChildren.some(child => child.type === "visibility_modifier" && child.text === "pub");
    const aliases = new Map<string, string>();
    const imports = (node: Parser.SyntaxNode, prefix = "", target = aliases) => {
      if (node.type === "scoped_use_list") { const path = node.childForFieldName("path"), list = node.childForFieldName("list"); if (path && list) imports(list, prefix + path.text + "::", target); }
      else if (node.type === "use_list") for (const child of node.namedChildren) imports(child, prefix, target);
      else if (node.type === "use_as_clause") { const path = node.childForFieldName("path"), alias = node.childForFieldName("alias"); if (path && alias) target.set(alias.text, prefix + path.text); }
      else target.set(node.text.split("::").at(-1)!, prefix + node.text);
    };
    for (const item of tree.rootNode.namedChildren.filter(node => node.type === "use_declaration")) { const argument = item.childForFieldName("argument"); if (argument) imports(argument); }
    const exported = tree.rootNode.namedChildren.filter(publicItem), names = new Set(exported.map(node => node.childForFieldName("name")?.text));
    const methods = tree.rootNode.namedChildren.filter(node => node.type === "impl_item" && names.has(node.childForFieldName("type")?.text)).flatMap(node => node.childForFieldName("body")?.namedChildren.filter(publicItem) ?? []);
    const paths = new Set<string>();
    const resolveAlias = (path: string) => { const split = path.indexOf("::"), prefix = split < 0 ? path : path.slice(0, split); return aliases.get(path) ?? (aliases.has(prefix) ? aliases.get(prefix)! + (split < 0 ? "" : path.slice(split)) : path); };
    for (const item of [...exported, ...methods]) {
      if (item.type === "use_declaration") { const argument = item.childForFieldName("argument"), visible = new Map<string, string>(); if (argument) imports(argument, "", visible); for (const value of visible.values()) { const path = resolveAlias(value); if (/^(?:tokio|wasmtime|wasmtime_wasi)::/u.test(path)) paths.add(path); } continue; }
      const body = item.childForFieldName("body");
      const signatures = item.type === "struct_item" ? (body?.namedChildren.filter(publicItem) ?? []) : item.namedChildren.filter(child => child.id !== body?.id || item.type === "trait_item" || item.type === "enum_item");
      assert(signatures.every(node => !node.hasError()), "complete public Rust signature syntax");
      for (const node of signatures.flatMap(child => [child, ...child.descendantsOfType(["scoped_type_identifier", "type_identifier"])])) {
        const path = resolveAlias(node.text);
        if (/^(?:tokio|wasmtime|wasmtime_wasi)::/u.test(path)) paths.add(path);
      }
    }
    return [...paths].sort();
  } finally { tree.delete(); }
}

/** 🛂️ Public runtime ports keep backend library types inside their owning implementation. */
test("native host uses one called runtime with first-party guest ports", async () => {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as { version: number; ports: { source: string; foreign: string[] }[] };
  assert(new Ajv2020({ strict: true }).validate(JSON.parse(readFileSync(join(import.meta.dir, "🧬️.json"), "utf8")), fixture));
  await Parser.init();
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", import.meta.dir)), "out/tree-sitter-rust.wasm")));
  const db = new Database(":memory:");
  try {
    for (const row of fixture.ports) {
      const actual = foreignTypes(parser, row.source);
      assert.deepEqual(actual, row.foreign);
      assert.equal((db.query("select count(*) as count from json_each(?)").get(JSON.stringify(actual)) as { count: number }).count, row.foreign.length);
    }
    const source = readModuleTreeSource(join(owner, "🎭️guest-runtime", "🦀️.rs")), tree = parser.parse(source);
    assert(tree);
    try {
      const port = tree.rootNode.descendantsOfType("trait_item").find(node => node.childForFieldName("name")?.text === "GuestRuntime");
      assert(port && !port.hasError(), "actual called GuestRuntime port");
      assert.deepEqual(foreignTypes(parser, port.text), []);
      assert.deepEqual(foreignTypes(parser, source), [], "public host facade owns its backend implementation types");
      const obsolete = tree.rootNode.descendantsOfType("mod_item").find(node => node.childForFieldName("name")?.text === "runtime");
      assert.equal(obsolete, undefined, "uncalled alternate backend is not in the public host closure");
    } finally { tree.delete(); }
    assert(!existsSync(join(owner, "⏳️runtime", "🦀️.rs")), "retired alternate implementation has no source continuation");
  } finally { db.close(); parser.delete(); }
});
