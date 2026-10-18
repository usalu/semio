/**
 * 🧬️ The `ShellState` god object is fifteen component structs and a composer, and nothing else.
 *
 * Language-agnostic oracle: `🧪️tests/🧫️fixtures/🧬️shell-components/🔣️.json` (the component table of the renderer partition
 * plan, 7.3). Two independent readers of the Rust source must agree with it and with each other: the shipped
 * tree-sitter Rust grammar (third-party) and the repository's own Rust tokenizer. The law reads the whole Shell
 * module tree through `readModuleTreeSource`, so it keeps holding after the components move into their crates.
 */

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Parser from "web-tree-sitter";
import { describe, expect, it } from "vitest";
import { readModuleTreeSource } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { rustFindTopLevel, rustTokenPairs, rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(resolve(here, "../🧫️fixtures/🧬️shell-components/🔣️.json"), "utf8")) as {
  readonly composerFields: readonly { readonly field: string; readonly struct: string }[];
  readonly components: readonly { readonly struct: string; readonly fields: readonly string[] }[];
};
const source = readModuleTreeSource(resolve(here, "../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"));

type Fields = ReadonlyMap<string, readonly string[]>;

async function treeSitterStructs(text: string, names: ReadonlySet<string>): Promise<Fields> {
  await Parser.init({ locateFile: () => resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
  const tree = parser.parse(text), out = new Map<string, readonly string[]>();
  const visit = (node: Parser.SyntaxNode): void => {
    if (node.type === "struct_item") {
      const name = node.childForFieldName("name")?.text ?? "";
      const body = node.childForFieldName("body");
      if (names.has(name) && body?.type === "field_declaration_list") out.set(name, body.namedChildren.filter(child => child.type === "field_declaration").map(child => child.childForFieldName("name")!.text));
    }
    for (const child of node.children) visit(child);
  };
  visit(tree.rootNode);
  return out;
}

function tokenizerStructs(text: string, names: ReadonlySet<string>): Fields {
  const tokens = rustTokens(text), pairs = rustTokenPairs(tokens), out = new Map<string, readonly string[]>();
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]!.text !== "struct" || !names.has(tokens[index + 1]?.text ?? "") || tokens[index + 2]?.text !== "{") continue;
    const close = pairs.get(index + 2)!, fields: string[] = [];
    for (let at = index + 3; at < close;) {
      if (tokens[at]!.text === "#" && tokens[at + 1]?.text === "[") { at = pairs.get(at + 1)! + 1; continue; }
      if (tokens[at]!.text === "pub") { at += tokens[at + 1]?.text === "(" ? pairs.get(at + 1)! - at + 1 : 1; continue; }
      if (tokens[at + 1]?.text === ":") fields.push(tokens[at]!.text);
      const end = rustFindTopLevel(tokens, pairs, at, close, new Set([","]));
      at = end < 0 ? close : end + 1;
    }
    out.set(tokens[index + 1]!.text, fields);
  }
  return out;
}

describe("🧬️ ShellState is fifteen components and a composer", () => {
  const names = new Set([...fixture.components.map(component => component.struct), "ShellState"]);
  it("the composer has exactly the fifteen component fields, in table order, in both readers", async () => {
    const expected = fixture.composerFields.map(row => row.field);
    expect((await treeSitterStructs(source, names)).get("ShellState")).toEqual(expected);
    expect(tokenizerStructs(source, names).get("ShellState")).toEqual(expected);
  });
  it("every component struct carries exactly its fields of the table, and the 248 former fields appear once", async () => {
    const sitter = await treeSitterStructs(source, names), ours = tokenizerStructs(source, names), seen = new Map<string, number>();
    for (const component of fixture.components) {
      expect(sitter.get(component.struct), component.struct).toEqual(component.fields);
      expect(ours.get(component.struct), component.struct).toEqual(component.fields);
      for (const field of component.fields) seen.set(field, (seen.get(field) ?? 0) + 1);
    }
    expect(seen.size).toBe(248);
    expect([...seen.values()].every(count => count === 1)).toBe(true);
  });
  it("no inherent `impl ShellState` member reaches a former field directly", async () => {
    const legacy = new Set(fixture.components.flatMap(component => component.fields)), composer = new Set(fixture.composerFields.map(row => row.field));
    await Parser.init({ locateFile: () => resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))), "tree-sitter.wasm") });
    const parser = new Parser();
    parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))), "out/tree-sitter-rust.wasm")));
    const offenders: string[] = [];
    const visit = (node: Parser.SyntaxNode, inState: boolean): void => {
      const state = node.type === "impl_item" ? node.childForFieldName("type")?.text === "ShellState" && node.childForFieldName("trait") === null : inState;
      if (state && node.type === "field_expression" && node.childForFieldName("value")?.text === "self") {
        const field = node.childForFieldName("field")?.text ?? "";
        if (legacy.has(field) && !composer.has(field)) offenders.push(field);
      }
      for (const child of node.children) visit(child, state);
    };
    visit(parser.parse(source).rootNode, false);
    expect(offenders).toEqual([]);
  });
});
