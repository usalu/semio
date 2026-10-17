/** 📣️ Closed first-party Actor facets expose exact runtime and type names without implementation imports. */
import { test, expect } from "bun:test";
import { readFileSync, existsSync, realpathSync } from "node:fs";
import { resolve, join, dirname } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import { Database } from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";

/** 🧾️ Independent syntax, schema and relational rosters qualify the actual authored package exports. */
test("Actor public facets publish only their defining explicit symbol and schema rosters", async () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, facets: fixture.facets.slice(1) })).toBe(false);
  const owner = resolve(import.meta.dir, "../.."), packageRoot = join(owner, "📦️packages/🟦️typescript");
  const manifest = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8")), db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE public_symbol(facet TEXT,name TEXT,kind TEXT,PRIMARY KEY(facet,name))");
    for (const row of fixture.facets) {
      expect(manifest.exports[row.subpath], row.subpath).toBe("./" + row.public);
      const path = join(packageRoot, row.public);
      expect(existsSync(path), row.subpath).toBe(true);
      const text = readFileSync(path, "utf8"), syntax = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      expect((syntax as unknown as { parseDiagnostics: unknown[] }).parseDiagnostics).toEqual([]);
      const exports = syntax.statements.filter(ts.isExportDeclaration);
      expect(exports.length).toBeGreaterThan(0);
      for (const declaration of exports) {
        expect(declaration.exportClause && ts.isNamedExports(declaration.exportClause), row.subpath).toBe(true);
        if (!declaration.exportClause || !ts.isNamedExports(declaration.exportClause)) throw Error("Named first-party exports required");
        if (!declaration.moduleSpecifier || !ts.isStringLiteral(declaration.moduleSpecifier)) throw Error("Defining source reexports required");
        const schemaExport = declaration.exportClause.elements.some(element => element.name.text === row.schemaExport);
        expect(resolve(dirname(path), declaration.moduleSpecifier.text)).toBe(join(owner, schemaExport ? row.schema! : row.implementation));
        for (const element of declaration.exportClause.elements) db.query("INSERT INTO public_symbol VALUES(?,?,?)").run(row.subpath, element.name.text, declaration.isTypeOnly || element.isTypeOnly ? "type" : "runtime");
      }
      expect(db.query("SELECT name FROM public_symbol WHERE facet=? AND kind='runtime' ORDER BY name").all(row.subpath)).toEqual([...row.runtime].sort().map(name => ({ name })));
      expect(db.query("SELECT name FROM public_symbol WHERE facet=? AND kind='type' ORDER BY name").all(row.subpath)).toEqual([...row.types].sort().map(name => ({ name })));
      const runtime = await import(pathToFileURL(path).href);
      expect(Object.keys(runtime).sort()).toEqual([...row.runtime].sort());
      const packageSpecifier = fixture.package + (row.subpath === "." ? "" : row.subpath.slice(1));
      expect(realpathSync(Bun.resolveSync(packageSpecifier, owner))).toBe(realpathSync(path));
      const published = await import(packageSpecifier);
      expect(Object.keys(published).sort()).toEqual([...row.runtime].sort());
      for (const name of row.runtime) expect(published[name]).toBe(runtime[name]);
      if (row.schema && row.schemaExport) expect(runtime[row.schemaExport]).toEqual(JSON.parse(readFileSync(join(owner, row.schema), "utf8")));
    }
  } finally { db.close(); }
});

