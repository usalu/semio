import { expect, test } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
import { inspectSchemaValidationReads, inspectSchemaValidationReadIndex } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import examples from "../🧫️fixtures/🔣️.json";

const sha = (text: string) => createHash("sha256").update(text).digest("hex");

const files = new Map(Object.entries(examples.files).map(([path, value]) => [path, JSON.stringify(value)])), admit = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/SchemaValidationReadReport`)!;
for (const row of examples.cases) test(`schema read original binding provenance: ${row.id}`, () => {
    const report = inspectSchemaValidationReads("/reader.ts", row.source, path => files.get(path));
    expect(admit(report), row.id + JSON.stringify(admit.errors)).toBe(true);
    expect(report.readerHash, row.id).toBe(sha(row.source));
    expect(report.reads.length, row.id).toBe(row.expected.reads);
    expect(report.unresolved.length > 0, row.id).toBe(row.expected.unresolved);
    expect(report.completeSyntax, row.id).toBe("completeSyntax" in row.expected ? row.expected.completeSyntax : true);
    const file = ts.createSourceFile("reader.ts", row.source, ts.ScriptTarget.Latest, true), callRanges: { start: number; end: number }[] = [];
    const visit = (node: ts.Node): void => { if (ts.isCallExpression(node)) callRanges.push({ start: node.getStart(file), end: node.end }); ts.forEachChild(node, visit); };
    visit(file);
    for (const read of report.reads) {
      expect(callRanges, row.id).toContainEqual({ start: read.callStart, end: read.callEnd });
      expect(read.resolution, row.id).toBe(row.expected.resolution);
      expect(read.schema.hash, row.id).toBe(sha(read.schema.path ? files.get(read.schema.path)! : row.source));
      if (read.value) expect(read.value.hash, row.id).toBe(sha(files.get(read.value.path!)!));
      if (row.id === "loop-row") expect(read.value!.selector).toEqual(["cases", "*", "value"]);
      if (row.id === "selected-schema-ref") expect(read.schema.selector).toEqual(["$defs", "Grant"]);
    }
    if (row.id === "shadowed-validator") {
      const options = { noLib: true, noResolve: true }, host = ts.createCompilerHost(options); host.getSourceFile = path => path === "reader.ts" ? file : undefined;
      const checker = ts.createProgram(["reader.ts"], options, host).getTypeChecker(), imports: ts.Identifier[] = [], constructors: ts.Identifier[] = [];
      const symbols = (node: ts.Node): void => { if (ts.isImportClause(node) && node.name) imports.push(node.name); if (ts.isNewExpression(node) && ts.isIdentifier(node.expression)) constructors.push(node.expression); ts.forEachChild(node, symbols); };
      symbols(file);
      expect(checker.getSymbolAtLocation(constructors[0]!)).not.toBe(checker.getSymbolAtLocation(imports[0]!));
    }
  console.log(`[DEBUG] original schema reader evidence verified: ${row.id}`);
});

test("schema read inspection honors cancellation without creating file copies", () => {
  const marker = new Error("cancelled"); let turns = 0;
  expect(() => inspectSchemaValidationReads("/reader.ts", examples.cases[0]!.source, () => undefined, () => { if (++turns === 2) throw marker; })).toThrow(marker);
  expect(turns).toBe(2);
});


test("schema read production implementation and formats satisfy strict independent TypeScript", () => {
  const program = ts.createProgram([fileURLToPath(new URL("../🟦️.ts", import.meta.url))], { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.Preserve, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noUncheckedIndexedAccess: true, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: ["bun"] });
  expect(ts.getPreEmitDiagnostics(program).map(row => ({ code: row.code, start: row.start, path: row.file?.fileName, message: ts.flattenDiagnosticMessageText(row.messageText, "\n") }))).toEqual([]);
  console.log("[DEBUG] actual schema-read implementation and defining formats compile under strict independent TypeScript");
});

test("schema reader index inventories supplied original sources once with explicit unavailable evidence", async () => {
  const sources = new Map(Object.entries(examples.index.sources)), reads = new Map<string, number>(), progress: { completed: number; total: number | null; phase: string }[] = [];
  const index = await inspectSchemaValidationReadIndex(examples.index.paths, path => { reads.set(path, (reads.get(path) ?? 0) + 1); return sources.get(path) ?? files.get(path); }, { onProgress: row => progress.push(row) });
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/SchemaValidationReadIndex`)!;
  expect(validate(index), JSON.stringify(validate.errors)).toBe(true);
  expect(index.reports.map(row => row.readerPath)).toEqual(examples.index.expectedPaths);
  expect(index.reports.map(row => row.completeSyntax)).toEqual(examples.index.expectedComplete);
  expect(index.unavailablePaths).toEqual(examples.index.expectedUnavailable);
  expect(index.reports.find(row => row.readerPath === "/missing.ts")!.readerHash).toBeNull();
  expect(index.reports.find(row => row.readerPath === "/missing.ts")!.unresolved).toEqual([{ start: 0, end: 0, reason: "source-unavailable" }]);
  expect(reads.get("/z.ts")).toBe(1);
  expect(progress.map(row => row.completed)).toEqual([0, 1, 2, 3]);
  for (const row of progress) expect(row.total).toBe(3);
  const oracle = examples.index.paths.filter((path, index, paths) => /\.(?:[cm]?[jt]s|[jt]sx)$/u.test(path) && paths.indexOf(path) === index).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  expect(index.reports.map(row => row.readerPath)).toEqual(oracle);
  console.log("[DEBUG] original source index preserves unique byte order, unavailable hashes, closed syntax and progressive inspection");
});

test("schema reader index cancels before the next source and propagates its original reason", async () => {
  const marker = new Error("index-cancelled"); let stopped = false; const reads: string[] = [];
  await expect(inspectSchemaValidationReadIndex(["/b.ts", "/a.ts"], path => { reads.push(path); return "const value = 1;"; }, { checkCancellation: () => { if (stopped) throw marker; }, onProgress: row => { if (row.completed === 1) stopped = true; } })).rejects.toBe(marker);
  expect(reads).toEqual(["/a.ts"]);
});


test("defining discovery adapter reads handcrafted specimens in their original walk and refuses vanished sources", async () => {
  const { mkdtempSync, mkdirSync, writeFileSync, rmSync } = await import("node:fs"), { join } = await import("node:path"), discovery = await import("../../🟦️.ts");
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("Original source inspection law requires caller-owned test output");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "ri-"));
  try {
    writeFileSync(join(root, "a.ts"), 'import Ajv from "ajv";import schema from "./schema.json";import value from "./examples.json";new Ajv().compile(schema)(value);');
    writeFileSync(join(root, "schema.json"), '{"type":"object"}'); writeFileSync(join(root, "examples.json"), '{"value":1}');
    writeFileSync(join(root, "b.ts"), "const value = 1;"); mkdirSync(join(root, "vanished")); writeFileSync(join(root, "vanished", "c.ts"), "const value = 2;");
    const progress: string[] = [], index = await discovery.inventorySchemaValidationReads(root, discovery.loadCatalogTaxonomy(), { onProgress: row => {
      progress.push(row.phase);
      if (row.phase === "walk" && row.path === "vanished") rmSync(join(root, "vanished"), { recursive: true });
      if (row.phase === "inspect" && row.completed === 0) rmSync(join(root, "b.ts"));
    } });
    expect(index.reports.map(row => row.readerPath)).toEqual(["a.ts", "b.ts"]);
    expect(index.reports[0]!.reads[0]!.schema.path).toBe("schema.json");
    expect(index.reports[0]!.reads[0]!.value!.path).toBe("examples.json");
    expect(index.reports[1]!.readerHash).toBeNull();
    expect(index.unavailablePaths).toEqual(["b.ts", "vanished"]);
    expect(progress).toContain("walk"); expect(progress).toContain("inspect");
    console.log("[DEBUG] defining discovery adapter inspected original handed specimens and retained both vanished directory and source evidence");
  } finally { rmSync(root, { recursive: true, force: true }); }
});

for (const row of examples.cases) if ("runtime" in row) test(`schema reader semantics agree with independent TypeScript and Ajv execution: ${row.id}`, async () => {
  const { runInNewContext } = await import("node:vm"), mutationKeys: string[] = [], schemaCalls: unknown[] = [], validations: unknown[] = [], documents = new Map(Object.entries(examples.files).map(([path, value]) => [path.slice(1), JSON.parse(JSON.stringify(value))]));
  const fixture = new Proxy(documents.get("examples.json")!, { set: (target, key, value) => { mutationKeys.push(String(key)); target[key] = value; return true; } });
  class ObservedAjv extends Ajv {
    compile(value: any): any { schemaCalls.push(value); const validate = super.compile(value); return (input: unknown) => { validations.push(input); return validate(input); }; }
  }
  const opaque = (value?: unknown): unknown => { if (value instanceof ObservedAjv) value.compile = () => () => true; return {}; };
  const javascript = ts.transpileModule(row.source, { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 }, reportDiagnostics: true });
  expect(javascript.diagnostics).toEqual([]);
  let error = false;
  try { runInNewContext(javascript.outputText, { exports: {}, opaque, Mutator: function(input: Record<string, unknown>) { input.x = 1; }, require: (name: string) => {
    if (name === "ajv") return { default: ObservedAjv };
    const path = name.replace(/^\.\//u, "");
    if (!documents.has(path)) throw new Error("Unexpected oracle module " + name);
    return { default: path === "examples.json" ? fixture : documents.get(path) };
  } }, { timeout: 1000 }); } catch { error = true; }
  expect({ compileSchema: !schemaCalls.length ? "absent" : schemaCalls[0] === undefined ? "undefined" : schemaCalls[0] === documents.get("schema.json") ? "schema.json" : "other", validations: validations.length, mutationKeys, error }).toEqual(row.runtime);
  console.log(`[DEBUG] independent TypeScript lowering, actual JavaScript effects and Ajv witness agree: ${row.id}`);
});

test("schema reader index yields to timer-driven cancellation before finishing expensive source inventories", async () => {
  const marker = new Error("timer-cancelled"), paths = Array.from({ length: 80 }, (_, index) => `/source-${String(index).padStart(3, "0")}.ts`); let cancelled = false, readCount = 0;
  await expect(inspectSchemaValidationReadIndex(paths, () => { readCount++; return "const value = 1;"; }, { checkCancellation: () => { if (cancelled) throw marker; }, onProgress: row => { if (row.completed === 1) setImmediate(() => { cancelled = true; }); } })).rejects.toBe(marker);
  expect(readCount).toBeGreaterThan(0); expect(readCount).toBeLessThan(paths.length);
});

test("schema reader index shares first original document content and literal Unicode byte order", async () => {
  const source = 'import Ajv from "ajv";import schema from "./schema.json";import value from "./examples.json";new Ajv().compile(schema)(value);', reads = new Map<string, number>();
  const index = await inspectSchemaValidationReadIndex(["/💡.ts", "/ä.ts", "/z.ts", "/a.ts"], path => { reads.set(path, (reads.get(path) ?? 0) + 1); return path.endsWith(".ts") ? source : files.get(path); });
  expect(index.reports.map(row => row.readerPath)).toEqual(["/a.ts", "/z.ts", "/ä.ts", "/💡.ts"]);
  expect(reads.get("/schema.json")).toBe(1); expect(reads.get("/examples.json")).toBe(1);
  for (const report of index.reports) expect(report.reads[0]!.schema.hash).toBe(sha(files.get("/schema.json")!));
});
