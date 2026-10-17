import { test } from "bun:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import { build } from "esbuild";
import ts from "typescript";
import type { NumericIndex, NumericIndexRetirement } from "../../📦️packages/🟦️typescript/🗂️ordered/🔢️numeric/🟦️.ts";

const owner = resolve(import.meta.dir, "../.."), workspace = resolve(owner, "../../..");
const ownerPackage = join(owner, "📦️packages", "🟦️typescript"), sdkPackage = join(workspace, "🧰️framework", "📦️packages", "🟦️typescript");
type Manifest = { name: string; exports: Record<string, string> };
type Fixture = { version: number; ownerName: string; sdkName: string; ownerExport: string; sdkExport: string; runtime: string[]; types: string[]; removedRoots: string[]; grant: { maxItems: number; maxBytes: number }; commands: ({ kind: "set"; id: number; value: string } | { kind: "remove"; id: number })[]; expected: [number, string][] };
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as Fixture;

/** ♻️ Drains only caller-admitted numeric retirement work and verifies its terminal owner. */
function close(owner: NumericIndexRetirement<string>): void {
  for (let turns = 0; turns < 100_000; turns++) {
    const step = owner.advance(fixture.grant);
    assert(step.items <= fixture.grant.maxItems && step.bytes <= fixture.grant.maxBytes);
    if (step.kind === "complete") { assert(owner.terminalIsEmpty()); return; }
    assert.notEqual(step.kind, "blocked");
  }
  assert.fail("public numeric retirement did not finish under finite caller turns");
}

/** 📤️ The authored Value and SDK exports preserve the same original bounded numeric ownership. */
test("published numeric value facets match SQLite and survive removal of specifics", async () => {
  assert(new Ajv2020({ strict: true }).validate(JSON.parse(readFileSync(join(import.meta.dir, "🧬️.json"), "utf8")), fixture));
  const manifest = JSON.parse(readFileSync(join(ownerPackage, "package.json"), "utf8")) as Manifest, sdkManifest = JSON.parse(readFileSync(join(sdkPackage, "package.json"), "utf8")) as Manifest;
  assert.equal(manifest.name, fixture.ownerName); assert.equal(sdkManifest.name, fixture.sdkName);
  assert.equal(typeof manifest.exports[fixture.ownerExport], "string");
  assert(Object.keys(manifest.exports).every(key => !key.includes("*")), "published owner facets have explicit keys");
  const paths = [join(ownerPackage, manifest.exports[fixture.ownerExport]!), join(sdkPackage, sdkManifest.exports[fixture.sdkExport]!)];
  for (const path of paths) {
    const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
    const exported: string[] = [];
    for (const statement of source.statements) if (ts.isExportDeclaration(statement)) { assert(statement.exportClause && ts.isNamedExports(statement.exportClause), "explicit names are required"); exported.push(...statement.exportClause.elements.map(item => item.name.text)); }
    assert.deepEqual(exported.sort(), [...fixture.runtime, ...fixture.types].sort());
  }
  const ownerApi = await import(pathToFileURL(paths[0]!).href) as typeof import("../../📦️packages/🟦️typescript/🗂️ordered/🔢️numeric/🟦️.ts"), sdkApi = await import(pathToFileURL(paths[1]!).href) as typeof ownerApi;
  assert.deepEqual(Object.keys(ownerApi).sort(), fixture.runtime);
  for (const key of fixture.runtime) assert.equal(Reflect.get(ownerApi, key), Reflect.get(sdkApi, key), "SDK preserves the exact first-party constructor");
  const db = new Database(":memory:");
  let index: NumericIndex<string> = sdkApi.NumericIndex.empty<string>();
  try {
    db.run("create table original (id integer primary key, ordinal integer unique, value text not null)");
    let ordinal = 0;
    for (const command of fixture.commands) {
      const edit = command.kind === "set" ? index.beginSet(command.id, command.value) : index.beginRemove(command.id);
      assert.equal(edit.advance({ maxItems: 0, maxBytes: fixture.grant.maxBytes }).kind, "blocked");
      let next: NumericIndex<string> | null = null;
      for (let turn = 0; turn < 100_000; turn++) { const step = edit.advance(fixture.grant); assert(step.items <= fixture.grant.maxItems && step.bytes <= fixture.grant.maxBytes); if (step.kind === "ready") { next = edit.takeResult(); break; } assert.notEqual(step.kind, "blocked"); }
      assert(next, "published edit completes under finite original grants");
      close(edit.beginClose()); assert(edit.terminalIsEmpty()); close(index.beginClose()); index = next;
      if (command.kind === "set") db.query("insert into original values (?, ?, ?) on conflict(id) do update set value=excluded.value").run(command.id, ordinal++, command.value);
      else db.query("delete from original where id=?").run(command.id);
      const expected = db.query("select id,value from original order by ordinal").all() as { id: number; value: string }[];
      assert.deepEqual([...index], expected.map(row => [row.id, row.value]));
    }
    assert.deepEqual([...index], fixture.expected);
  } finally { close(index.beginClose()); db.close(); }
  const closure = await build({ entryPoints: [paths[1]!], absWorkingDir: workspace, bundle: true, write: false, metafile: true, platform: "neutral", format: "esm", plugins: [{ name: "specific-deletion", setup(builder) { builder.onLoad({ filter: /./ }, args => { const path = relative(workspace, args.path).split("\\").join("/"); if (fixture.removedRoots.some(root => path === root || path.startsWith(root + "/"))) throw Error("removed specific cannot be loaded: " + path); return undefined; }); } }] });
  assert(Object.keys(closure.metafile!.inputs).length >= 3);
  assert(Object.keys(closure.metafile!.inputs).every(path => path.startsWith("🧰️framework/")));
  for (const [directory, specifier] of [[ownerPackage, fixture.ownerName + fixture.ownerExport.slice(1)], [sdkPackage, fixture.sdkName + fixture.sdkExport.slice(1)]]) {
    const published = await build({ stdin: { contents: `export { ${fixture.runtime.join(", ")} } from ${JSON.stringify(specifier)};`, resolveDir: directory!, loader: "ts" }, bundle: true, write: false, platform: "neutral", format: "esm" });
    assert(published.outputFiles.length === 1 && published.outputFiles[0]!.text.includes("NumericIndex"), "actual package self-reference resolves its published export");
  }
  const program = ts.createProgram(paths, { declaration: true, emitDeclarationOnly: true, allowImportingTsExtensions: true, strict: true, skipLibCheck: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, types: [] });
  assert.equal(ts.getPreEmitDiagnostics(program).length, 0, "actual owner and SDK declarations have no unresolved public types");
  const declarations = new Map<string, string>();
  assert(!program.emit(undefined, (path, source) => declarations.set(path, source)).emitSkipped);
  for (const path of paths) assert(declarations.get(path.slice(0, -3) + ".d.ts")?.includes("NumericIndexGrant"), "published declaration retains its first-party grant type");
  assert([...declarations.values()].every(source => !/from\s+["'](?:✏️s|🌎️hub|@semio-tech\/s|@semio-tech\/hub)/u.test(source)));
});
