import { test } from "bun:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import { build } from "esbuild";
import { read as ieeeRead, write as ieeeWrite } from "ieee754";
import ts from "typescript";
import type * as Refusal from "../../../⚠️refusal/🟦️.ts";
import type * as Ieee from "../../../🔢️ieee754/🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };
import schema from "./🧬️.json" with { type: "json" };

const owner = resolve(import.meta.dir, "../../.."), workspace = resolve(owner, "../../..");
const ownerPackage = join(owner, "📦️packages", "🟦️typescript"), sdkPackage = join(workspace, "🧰️framework", "📦️packages", "🟦️typescript");
type Manifest = { name: string; exports: Record<string, string> };

/** 📤️ Resolves exact authored exports and validates their original first-party declaration and closure. */
async function publicFacet(facet: typeof fixture.facets[number]): Promise<Record<string, unknown>> {
  const paths: string[] = [], apis: Record<string, unknown>[] = [];
  for (const [directory, name, key] of [[ownerPackage, fixture.ownerName, facet.ownerExport], [sdkPackage, fixture.sdkName, facet.sdkExport]]) {
    const manifest = JSON.parse(readFileSync(join(directory!, "package.json"), "utf8")) as Manifest;
    assert.equal(manifest.name, name); assert.equal(typeof manifest.exports[key!], "string");
    assert(Object.keys(manifest.exports).every(exported => !exported.includes("*")), "only explicit authored public facets");
    const path = join(directory!, manifest.exports[key!]!); paths.push(path);
    const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
    const names: string[] = [];
    for (const statement of source.statements) if (ts.isExportDeclaration(statement)) { assert(statement.exportClause && ts.isNamedExports(statement.exportClause)); names.push(...statement.exportClause.elements.map(item => item.name.text)); }
    assert.deepEqual(names.sort(), [...facet.runtime, ...facet.types].sort());
    const api = await import(pathToFileURL(path).href) as Record<string, unknown>; apis.push(api);
    assert.deepEqual(Object.keys(api).sort(), facet.runtime);
    const closure = await build({ entryPoints: [path], absWorkingDir: workspace, bundle: true, write: false, metafile: true, platform: "neutral", format: "esm", plugins: [{ name: "specific-deletion", setup(builder) { builder.onLoad({ filter: /./ }, args => { const input = relative(workspace, args.path).split("\\").join("/"); if (fixture.removedRoots.some(root => input === root || input.startsWith(root + "/"))) throw Error("removed specific cannot be loaded: " + input); return undefined; }); } }] });
    assert(Object.keys(closure.metafile!.inputs).every(input => input.startsWith("🧰️framework/")));
    const published = await build({ stdin: { contents: `export { ${facet.runtime.join(", ")} } from ${JSON.stringify(name! + key!.slice(1))};`, resolveDir: directory!, loader: "ts" }, bundle: true, write: false, platform: "neutral", format: "esm" });
    assert.equal(published.outputFiles.length, 1);
  }
  for (const name of facet.runtime) assert.equal(apis[0]![name], apis[1]![name], "SDK retains the exact original value authority");
  const program = ts.createProgram(paths, { declaration: true, emitDeclarationOnly: true, allowImportingTsExtensions: true, strict: true, skipLibCheck: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, types: [] });
  assert.deepEqual(ts.getPreEmitDiagnostics(program).map(row => ts.flattenDiagnosticMessageText(row.messageText, "\n")), []);
  const declarations = new Map<string, string>(); assert(!program.emit(undefined, (path, source) => declarations.set(path, source)).emitSkipped);
  for (const path of paths) for (const name of facet.types) assert(declarations.get(path.slice(0, -3) + ".d.ts")?.includes(name));
  return apis[1]!;
}

/** 🧾️ The exact public refusal preserves semantic identity, original effects and independent path projection. */
test("public refusal facets preserve original typed receipt and survive specific removal", async () => {
  assert(new Ajv2020({ strict: true, allowUnionTypes: true }).validate(schema, fixture));
  const api = await publicFacet(fixture.facets[0]!) as unknown as typeof Refusal;
  const db = new Database(":memory:");
  try {
    db.run("create table path (position integer primary key, segment text not null)");
    fixture.refusal.path.forEach((segment, index) => db.run("insert into path values (?,?)", [index, String(segment)]));
    const row = db.query("select group_concat(segment,'.') || '.' || ? as display from (select segment from path order by position)").get(fixture.refusal.message) as { display: string };
    for (const kind of fixture.refusal.kinds) {
      assert.equal(api.valueRefusalKindFromWire(kind), kind);
      const original = new api.ValueError(kind as Refusal.ValueRefusalKind, fixture.refusal.message).withRetainedProgress(fixture.refusal.receipt);
      const decorated = fixture.refusal.path.reduceRight((error, segment) => error.under(segment), original);
      assert.equal(decorated.kind, kind); assert.equal(decorated.retainedProgress, original.retainedProgress);
      assert.equal(decorated.toString(), row.display); assert.equal(decorated.intoMessage(), fixture.refusal.display);
      assert.equal(original.kind, kind); assert.equal(original.message, fixture.refusal.message);
    }
    for (const value of fixture.refusal.invalid) assert.equal(api.valueRefusalKindFromWire(value), undefined);
  } finally { db.close(); }
  console.log("[DEBUG] public refusal facets originalKind=true originalPhysicalReceipt=true independentSQLite=true explicitExports=true actualPackageResolution=true declarations=true specificDeletion=true");
});

/** 🔢️ Public exact IEEE words match an independent width-specific oracle, including signed zeros and NaN payloads. */
test("public IEEE facets preserve native words against ieee754 and survive specific removal", async () => {
  const api = await publicFacet(fixture.facets[1]!) as unknown as typeof Ieee;
  for (const row of fixture.ieee) {
    assert.equal(row.bits.length * 4, row.width);
    const bytes = Buffer.from(row.bits, "hex"), expected = ieeeRead(bytes, 0, false, row.width === 32 ? 23 : 52, row.width / 8);
    const bits = BigInt("0x" + row.bits);
    const word = row.width === 32 ? api.parseBinary32({ bits: Number(bits) }) : api.parseBinary64({ bits });
    const actual = row.width === 32 ? api.binary32Value(word as Ieee.Binary32) : api.binary64Value(word as Ieee.Binary64);
    assert(Object.is(actual, expected), row.bits); assert.equal(BigInt(word.bits), bits);
    if (row.value === "positiveZero") assert(Object.is(actual, 0));
    else if (row.value === "negativeZero") assert(Object.is(actual, -0));
    else if (row.value === "nan") assert(Number.isNaN(actual));
    else if (row.value === "positiveInfinity") assert.equal(actual, Infinity);
    else if (row.value === "negativeInfinity") assert.equal(actual, -Infinity);
    else assert(Number.isFinite(actual) && actual !== 0);
  }
  for (const value of fixture.capture) for (const width of [32, 64]) {
    const bytes = Buffer.alloc(width / 8); ieeeWrite(bytes, value, 0, false, width === 32 ? 23 : 52, width / 8);
    assert.equal(BigInt((width === 32 ? api.binary32(value) : api.binary64(value)).bits), BigInt("0x" + bytes.toString("hex")));
  }
  for (const bits of fixture.invalidWords) { assert.throws(() => api.parseBinary32({ bits })); assert.throws(() => api.parseBinary64({ bits })); }
  console.log("[DEBUG] public IEEE facets exactWords=true signedZero=true nanPayload=true independentIeee754=true explicitExports=true actualPackageResolution=true declarations=true specificDeletion=true");
});
