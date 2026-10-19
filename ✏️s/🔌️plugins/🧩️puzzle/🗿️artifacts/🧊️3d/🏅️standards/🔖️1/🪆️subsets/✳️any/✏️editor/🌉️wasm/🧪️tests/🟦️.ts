/** 🔤️ Original Rust parser bodies and real story consumers survive obsolete VCS removal. */
import { expect, test } from "bun:test";
import ts from "typescript";
import fixture from "./🧫️fixtures/🔣️.json";

test("browser parser exports retain original bodies and actual typed consumers", async () => {
  const artifacts = new URL("../../../../../../../../", import.meta.url);
  for (const row of fixture.parsers) {
    const root = new URL(`${row.artifact}/`, artifacts);
    const source = (await Bun.file(new URL("🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs", root)).text()).replace(/\r\n/g, "\n");
    const body = source.match(/#\[wasm_bindgen(?:::prelude::wasm_bindgen)?\(js_name = puzzle(?:3|5)dParseDslJson\)\]\npub fn .*?\n\}/s)?.[0];
    expect(body).toBeDefined();
    expect(new Bun.CryptoHasher("sha256").update(body!).digest("hex")).toBe(row.parserSha256);
    expect(source).not.toContain("ArtifactVcs");
    expect(source).not.toContain("funded_turn");
    expect(source).not.toContain("StepContext::new");
    const story = await Bun.file(new URL(`📖️stories/${row.story}/🧪️.story.tsx`, root)).text();
    const tree = ts.createSourceFile("story.tsx", story, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
    const calls: string[] = [], members: string[] = [];
    const visit = (node: ts.Node): void => {
      if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression)) calls.push(node.expression.name.text);
      if (ts.isPropertySignature(node) && ts.isIdentifier(node.name)) members.push(node.name.text);
      ts.forEachChild(node, visit);
    };
    visit(tree);
    expect(calls).toContain(row.jsExport);
    expect(members).toContain(row.jsExport);
    expect(calls).not.toContain("pollEnvelopeLoad");
    expect(calls).not.toContain("closeStep");
  }
  console.log("[DEBUG] Puzzle 3D/5D Rust parser bodies match original SHA256; independent TypeScript AST confirms actual parser-only story consumers; native parser execution separate");
});
