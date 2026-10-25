/** 📝 Contract: a package source imported as text through an import attribute is delivered to the compiler as text, whatever loader its load hook is told. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { packageSourceLoaderV1 } from "../../🟦️.ts";

test("the loader table names .sql as text and leaves every other reported loader alone", () => {
  expect(packageSourceLoaderV1("/w/🗄️.sql", "js")).toBe("text");
  for (const [path, reported] of [["/w/a.ts", "ts"], ["/w/a.tsx", "tsx"], ["/w/a.json", "json"], ["/w/a.js", "js"]] as const) expect(packageSourceLoaderV1(path, reported)).toBe(reported);
});

test("the real compiler embeds the text of an attribute import only through the named loader", async () => {
  const root = mkdtempSync(join(tmpdir(), "text-loader-")), sql = "CREATE TABLE witness (id INTEGER PRIMARY KEY);\n";
  try {
    mkdirSync(join(root, "🕸️mesh")); writeFileSync(join(root, "🕸️mesh/🗄️.sql"), sql);
    writeFileSync(join(root, "entry.ts"), 'import text from"./🕸️mesh/🗄️.sql"with{type:"text"};\nexport default text;\n');
    const build = (named: boolean) => Bun.build({ entrypoints: [join(root, "entry.ts")], target: "bun", format: "esm", throw: false, plugins: [{ name: "witness", setup(builder) { builder.onLoad({ filter: /.*/, namespace: "file" }, args => ({ contents: readFileSync(args.path), loader: (named ? packageSourceLoaderV1(args.path, args.loader) : args.loader) as typeof args.loader })); } }] });
    const named = await build(true), unnamed = await build(false);
    expect(named.success).toBe(true);
    expect(await named.outputs[0]!.text()).toContain("CREATE TABLE witness");
    expect(unnamed.success).toBe(false);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
