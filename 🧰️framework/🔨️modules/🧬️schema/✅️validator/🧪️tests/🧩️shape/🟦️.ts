import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import ts from "typescript";
import * as owner from "../../🟦️.ts";

type Guard = "record" | "array" | "string" | "literal" | "keys";
type Mode = "date" | "map" | "inherited" | "sparse" | "undefined" | "inherited-string" | "inherited-number" | "symbol" | "nonenumerable" | "null-prototype";
interface Observation { readonly admitted: boolean; readonly diagnostic: string | null }
interface Row { readonly id: string; readonly guard: Guard; readonly value?: unknown; readonly mode?: Mode; readonly label: string; readonly allowed?: readonly string[]; readonly keys?: readonly string[]; readonly expected: Observation }
interface Guards {
  requireRecord(value: unknown, label: string): Record<string, unknown>;
  requireStringArray(value: unknown, label: string): readonly string[];
  requireString(value: unknown, label: string): string;
  requireLiteral<T extends string>(value: unknown, label: string, allowed: readonly T[]): T;
  requireExactKeys(value: Record<string, unknown>, keys: readonly string[], label: string): void;
}
const corpus: { readonly schemaVersion: 1; readonly json: readonly Row[]; readonly runtime: readonly Row[] } = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧩️shape/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧩️shape/🔣️.json"), "utf8"));
const sourcePath = resolve(import.meta.dir, "../../🟦️.ts");
const allRows = [...corpus.json, ...corpus.runtime];
const names = ["requireRecord", "requireStringArray", "requireString", "requireLiteral", "requireExactKeys"] as const;

function runtimeValue(mode: Mode): unknown {
  if (mode === "date") return new Date(0);
  if (mode === "map") return new Map([["hidden", 1]]);
  if (mode === "inherited") return Object.assign(Object.create({ y: 2 }), { x: 1 });
  if (mode === "undefined") return [undefined];
  if (mode === "sparse") { const value = new Array(2); value[1] = "x"; return value; }
  if (mode === "inherited-string" || mode === "inherited-number") {
    const value = new Array(2); value[0] = "x";
    Object.setPrototypeOf(value, Object.assign(Object.create(Array.prototype), { 1: mode === "inherited-string" ? "yes" : 3 }));
    return value;
  }
  if (mode === "symbol") return Object.assign({ x: 1 }, { [Symbol("hidden")]: 2 });
  if (mode === "nonenumerable") return Object.defineProperty({ x: 1 }, "hidden", { value: 2 });
  return Object.assign(Object.create(null), { x: 1 });
}
function observe(api: Guards, row: Row, value: unknown): Observation {
  try {
    if (row.guard === "record") api.requireRecord(value, row.label);
    else if (row.guard === "array") api.requireStringArray(value, row.label);
    else if (row.guard === "string") api.requireString(value, row.label);
    else if (row.guard === "literal") api.requireLiteral(value, row.label, row.allowed!);
    else api.requireExactKeys(value as Record<string, unknown>, row.keys!, row.label);
    return { admitted: true, diagnostic: null };
  } catch (error) { return { admitted: false, diagnostic: error instanceof Error ? error.message : String(error) }; }
}
function actualGuards(): Guards {
  for (const name of names) expect(typeof Reflect.get(owner, name)).toBe("function");
  return owner;
}
function oracle(row: Row, value: unknown): boolean {
  const ajv = new Ajv({ strict: true, ownProperties: true });
  if (row.guard === "record") return Boolean(ajv.validate({ type: "object" }, value));
  if (row.guard === "string") return Boolean(ajv.validate({ type: "string", minLength: 1 }, value));
  if (row.guard === "array") {
    const occupied = Array.isArray(value) ? Array.from({ length: value.length }, (_, index) => index).filter(index => index in value).map(index => value[index]) : value;
    return Boolean(ajv.validate({ type: "array", items: { type: "string" } }, occupied));
  }
  if (row.guard === "literal") {
    const allowed = [...new Set(row.allowed!)];
    return allowed.length > 0 && Boolean(ajv.validate({ type: "string", minLength: 1, enum: allowed }, value));
  }
  const keys = row.keys!;
  const properties = keys.length === 0 ? false : { enum: [...new Set(keys)] };
  return Boolean(ajv.validate({ type: "object", minProperties: keys.length, maxProperties: keys.length, propertyNames: properties }, value));
}

test("closed portable JSON and runtime shapes agree with independent AJV admission", () => {
  const validate = new Ajv({ strict: true, ownProperties: true }).compile(schema);
  expect(validate(corpus)).toBe(true);
  expect(new Set(allRows.map(row => row.id)).size).toBe(allRows.length);
  expect(validate({ ...corpus, extra: true })).toBe(false);
  for (const row of allRows) expect(oracle(row, row.mode ? runtimeValue(row.mode) : row.value)).toBe(row.expected.admitted);
});

test("actual guards preserve identities, sparse slots, labels and exact own enumerable keys", () => {
  const api = actualGuards();
  for (const row of allRows) {
    const value = row.mode ? runtimeValue(row.mode) : row.value;
    const descriptors = value !== null && typeof value === "object" ? Object.getOwnPropertyDescriptors(value) : undefined;
    const keys = row.keys?.slice(), allowed = row.allowed?.slice();
    expect(observe(api, row, value)).toEqual(row.expected);
    if (row.expected.admitted && row.guard !== "keys") {
      const result = row.guard === "record" ? api.requireRecord(value, row.label) : row.guard === "array" ? api.requireStringArray(value, row.label) : row.guard === "string" ? api.requireString(value, row.label) : api.requireLiteral(value, row.label, row.allowed!);
      expect(result).toBe(value);
    }
    if (descriptors) expect(Object.getOwnPropertyDescriptors(value)).toEqual(descriptors);
    expect(row.keys).toEqual(keys);
    expect(row.allowed).toEqual(allowed);
  }
});

test("strict owned TypeScript API and both transpilers retain actual Node runtime outputs", () => {
  actualGuards();
  const ticket = process.env.SEMIO_TEST_ARTIFACT_DIR ?? resolve(import.meta.dir, "../../../../../..", ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-framework");
  const directory = mkdtempSync(resolve(ticket, "shape-"));
  const source = readFileSync(sourcePath, "utf8");
  writeFileSync(resolve(directory, "owner.ts"), source);
  writeFileSync(resolve(directory, "client.ts"), 'import { requireRecord, requireStringArray, requireString, requireLiteral, requireExactKeys, type UnknownRecord } from "./owner"; const record: UnknownRecord = requireRecord({}, "payload"); const strings: readonly string[] = requireStringArray([], "payload"); const text: string = requireString("x", "payload"); const literal: "on" = requireLiteral("on", "payload", ["on"] as const); const done: void = requireExactKeys(record, [], "payload"); void [strings, text, literal, done];');
  const program = ts.createProgram([resolve(directory, "owner.ts"), resolve(directory, "client.ts")], { strict: true, noEmit: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, skipLibCheck: true, types: [], lib: ["lib.es2022.d.ts"] });
  expect(ts.getPreEmitDiagnostics(program).map(diagnostic => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
  const sourceFile = ts.createSourceFile("test.ts", readFileSync(import.meta.path, "utf8"), ts.ScriptTarget.Latest, true);
  const helpers = sourceFile.statements.filter(node => ts.isFunctionDeclaration(node) && ["runtimeValue", "observe"].includes(node.name?.text ?? "")).map(node => node.getText(sourceFile)).join("\n");
  const runner = ts.transpileModule(helpers + '\nconst rows = JSON.parse(' + JSON.stringify(JSON.stringify(allRows)) + '); console.log(JSON.stringify(rows.map(row => observe(api, row, row.mode ? runtimeValue(row.mode) : row.value))));', { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText;
  const implementations = [new Bun.Transpiler({ loader: "ts", target: "node" }).transformSync(source), ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText];
  for (const [index, compiled] of implementations.entries()) {
    const ownerPath = resolve(directory, `owner-${index}.mjs`), runnerPath = resolve(directory, `runner-${index}.mjs`);
    writeFileSync(ownerPath, compiled);
    writeFileSync(runnerPath, `import * as api from "./owner-${index}.mjs";\n${runner}`);
    const result = spawnSync("node", [runnerPath], { encoding: "utf8", timeout: 4_000 });
    expect(result.status).toBe(0);
    expect(result.stderr).toBe("");
    expect(JSON.parse(result.stdout)).toEqual(allRows.map(row => row.expected));
  }
  expect(ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true).statements.some(node => ts.isImportDeclaration(node) || ts.isExportDeclaration(node))).toBe(false);
});
