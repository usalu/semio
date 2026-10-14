/** 🏛️ Defining Actor source and test owners survive a product resolution denial. */
import { test, expect } from "bun:test";
import { readFileSync, readdirSync, realpathSync } from "node:fs";
import { resolve, join, relative } from "node:path";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";

/** 🧭️ Independent compiler syntax and SQLite agree on the closed first-party owner boundary. */
test("General Actor source and test mounts reject product references", async () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, sourceOwners: fixture.sourceOwners.slice(1) })).toBe(false);
  expect(validate({ ...fixture, entrypoints: [...fixture.entrypoints, "unexpected"] })).toBe(false);
  const owner = resolve(import.meta.dir, "../.."), db = new Database(":memory:"), ownedSources: string[] = [];
  const collect = (directory: string): void => { for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) { if (!fixture.sourceScope.excludedDirectories.includes(entry.name)) collect(path); }
    else if (entry.isFile() && fixture.sourceScope.extensions.some(extension => path.endsWith(extension))) ownedSources.push(relative(owner, path).replaceAll("\\", "/"));
  } };
  collect(owner);
  for (const source of fixture.sourceOwners) expect(ownedSources).toContain(source);
  try {
    for (const row of fixture.references) expect(db.query("SELECT instr(?,?)=0 AS allowed").get(row.path, fixture.forbiddenOwner)).toEqual({ allowed: Number(row.allowed) });
    for (const path of ownedSources) {
      const source = readFileSync(join(owner, path), "utf8"), syntax = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      expect((syntax as unknown as { parseDiagnostics: unknown[] }).parseDiagnostics, path).toEqual([]);
      const references: string[] = [];
      const visit = (node: ts.Node): void => {
        if (ts.isStringLiteralLike(node)) {
          references.push(node.text);
          const parent = node.parent, imported = ts.isImportDeclaration(parent) || ts.isExportDeclaration(parent) || (ts.isCallExpression(parent) && parent.expression.kind === ts.SyntaxKind.ImportKeyword) || (ts.isLiteralTypeNode(parent) && ts.isImportTypeNode(parent.parent));
          if (imported && node.text.startsWith("@semio-tech/")) expect(realpathSync(Bun.resolveSync(node.text, join(owner, path))), path).not.toContain("🛍️products");
        }
        ts.forEachChild(node, visit);
      };
      visit(syntax);
      for (const reference of references) expect(reference !== "🛍️products" && reference.includes("🛍️products"), path + ": " + reference).toBe(false);
    }
  } finally { db.close(); }
  const observed = new Set<string>();
  const built = await Bun.build({ entrypoints: fixture.entrypoints.map(path => join(owner, path)), target: "node", packages: "external", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "deny-product-owner", setup(build) {
    build.onResolve({ filter: /🛍️products/u }, args => { throw Error("General Actor cannot resolve product owner: " + args.path); });
    build.onLoad({ filter: /.*/u }, args => { observed.add(args.path); return undefined; });
  } }] });
  expect(built.success, built.logs.map(String).join("\n")).toBe(true);
  const denied: string[] = [], deniedPath = join(resolve(owner, "../../.."), fixture.forbiddenOwner, "probe.ts");
  await expect(Bun.build({ entrypoints: [join(owner, fixture.entrypoints[0]!)], target: "node", plugins: [{ name: "product-denial-probe", setup(build) {
    build.onResolve({ filter: /🛍️products/u }, args => { denied.push(args.path); throw Error("General Actor cannot resolve product owner: " + args.path); });
    build.onLoad({ filter: /.*/u }, () => ({ contents: "import " + JSON.stringify(deniedPath) + ";", loader: "ts" }));
  } }] })).rejects.toThrow("Bundle failed");
  expect(denied).toEqual([deniedPath]);
  expect(built.outputs.length).toBe(fixture.entrypoints.length);
  expect(observed.size).toBeGreaterThan(fixture.entrypoints.length);
  expect([...observed].every(path => !relative(owner, path).replaceAll("\\", "/").includes("🛍️products"))).toBe(true);
  console.log("[DEBUG] Closed Ajv/SQLite owner corpus, independent TypeScript AST and actual Bun production bundler denied every product resolution", { requiredOwners: fixture.sourceOwners.length, definingSources: ownedSources.length, modules: observed.size, entries: built.outputs.length });
});
