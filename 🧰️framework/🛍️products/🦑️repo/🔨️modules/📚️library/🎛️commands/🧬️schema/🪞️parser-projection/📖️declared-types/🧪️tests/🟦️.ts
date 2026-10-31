import { afterAll, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Parser from "web-tree-sitter";
import { declares } from "../🟦️.ts";

const corpus = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { cases: { id: string; source: string; name: string; declared: boolean }[] };
await Parser.init();
const language = await Parser.Language.load(require.resolve("tree-sitter-wasms/out/tree-sitter-rust.wasm"));
const parser = new Parser();
parser.setLanguage(language);
afterAll(() => parser.delete());
for (const row of corpus.cases) test("projector Rust declaration: " + row.id, () => {
  const tree = parser.parse(row.source);
  try {
    const independent = tree.rootNode.namedChildren.some(node => {
      if (node.namedChildren.find(child => child.type === "visibility_modifier")?.text !== "pub") return false;
      if (["struct_item", "enum_item", "type_item"].includes(node.type)) return node.childForFieldName("name")?.text === row.name;
      if (node.type !== "use_declaration") return false;
      const argument = node.childForFieldName("argument");
      if (argument?.type === "scoped_identifier") return argument.childForFieldName("name")?.text === row.name;
      return argument?.type === "use_as_clause" && argument.childForFieldName("alias")?.text === row.name;
    });
    expect(independent).toBe(row.declared);
    expect(declares("🦀️rust", row.source, row.name)).toBe(row.declared);
  } finally {
    tree.delete();
  }
});
