import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import schema from "./🧬️.json";
import law from "./🔣️.json";

test("normal process return requires actual empty while blocked lifetime retains fatal custody", () => {
  expect(new Ajv({ strict: true }).validate(schema, law)).toBe(true);
  const db = new Database(":memory:");
  try {
    for (const row of law.cases) {
      const result = db.query("select case when ? and not ? then 0 else 1 end code")
        .get(row.sourceEmpty, row.heldBorrow) as { code: number };
      expect(result.code).toBe(row.exitCode);
    }
  } finally {
    db.close();
  }
});

test("actual process owner and stdio/http mounts parse without a fallback global arena", async () => {
  await Parser.init();
  const parser = new Parser();
  const grammar = dirname(Bun.resolveSync("tree-sitter-wasms/package.json", process.cwd()));
  const trees: Parser.Tree[] = [];
  try {
    parser.setLanguage(await Parser.Language.load(resolve(grammar, "out/tree-sitter-rust.wasm")));
    const parse = (path: string, wholeFile = true) => {
      const tree = parser.parse(readFileSync(new URL(path, import.meta.url), "utf8"))!;
      trees.push(tree);
      if (wholeFile) expect(tree.rootNode.hasError()).toBe(false);
      return tree.rootNode;
    };
    const owner = parse("../🦀️.rs");
    const mounts = parse("../../🦀️.rs");
    const bootstrap = parse("../../🏗️bootstrap/🦀️.rs", false);
    for (const root of [owner, mounts, bootstrap]) {
      expect(root.descendantsOfType("static_item").some(node =>
        /\b(ProcessUiArena|UiArenaLifetime)\b/.test(node.childForFieldName("type")?.text ?? ""),
      )).toBe(false);
    }
    for (const name of ["run_stdio", "run_http"]) {
      const mount = mounts.descendantsOfType("function_item")
        .find(node => node.childForFieldName("name")?.text === name);
      expect(mount?.descendantsOfType("call_expression").some(call =>
        call.childForFieldName("function")?.text === "semio_os_mcp_process_arena::serve",
      )).toBe(true);
    }
    const entry = bootstrap.descendantsOfType("function_item")
      .find(node => node.childForFieldName("name")?.text === "run_mcp_entrypoint");
    expect(entry?.hasError()).toBe(false);
    const serving = entry?.descendantsOfType("call_expression")
      .find(call => call.childForFieldName("function")?.text === "serve_on_worker_stack");
    const refused = serving?.descendantsOfType("if_expression").find(node => {
      const condition = node.childForFieldName("condition");
      const pattern = condition?.childForFieldName("pattern");
      return condition?.type === "let_condition"
        && condition.childForFieldName("value")?.text === "result"
        && pattern?.childForFieldName("type")?.text === "Err"
        && pattern.namedChildren.at(-1)?.text === "error";
    });
    expect(refused?.childForFieldName("consequence")?.descendantsOfType("call_expression").some(call => {
      const method = call.childForFieldName("function");
      return method?.type === "field_expression"
        && method.childForFieldName("value")?.text === "error"
        && method.childForFieldName("field")?.text === "exit";
    })).toBe(true);
  } finally {
    for (const tree of trees) tree.delete();
    parser.delete();
  }
});
