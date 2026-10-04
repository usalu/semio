import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { parseTree, type Node as OracleNode, type ParseError } from "jsonc-parser";
import ts from "typescript";

const root = resolve(import.meta.dir, "..");
const contract = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
const operation = () => ({ maximumBytes: 1000000, maximumNodes: 10000, maximumDepth: 256, chunk: 128, cancelled: () => false, progress: (_: unknown) => {}, yield: async () => { await new Promise<void>(resolve => setTimeout(resolve, 0)); } });
async function api() { const owner = await import(resolve(root, "🟦️.ts")); expect(typeof owner.decodeJsonSyntax).toBe("function"); return owner; }

function oracle(node: OracleNode, source: string, policy: string): unknown {
  if (node.type === "number") return { kind: "number", text: source.slice(node.offset, node.offset + node.length) };
  if (node.type === "null") return { kind: "null" };
  if (node.type === "string" || node.type === "boolean") return { kind: node.type, value: node.value };
  if (node.type === "array") return { kind: "array", items: (node.children ?? []).map(child => oracle(child, source, policy)) };
  const members: { name: string; value: unknown }[] = [], index = new Map<string, number>();
  for (const child of node.children ?? []) {
    const name: string = child.children![0]!.value;
    const value = oracle(child.children![1]!, source, policy);
    const previous = index.get(name);
    if (previous !== undefined) {
      if (policy === "Reject") throw Error("duplicate-member");
      members[previous] = { name, value };
    } else { index.set(name, members.length); members.push({ name, value }); }
  }
  return { kind: "object", members };
}

test("language-neutral lossless syntax agrees with independent jsonc-parser positions and AJV", () => {
  const ajv = new Ajv({ strict: true }).addSchema(contract);
  const validate = ajv.compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  expect(new Set(fixture.cases.map((row: { id: string }) => row.id)).size).toBe(fixture.cases.length);
  for (const row of fixture.cases) {
    const errors: ParseError[] = [], tree = parseTree(row.source, errors, { disallowComments: true, allowTrailingComma: false });
    if ("error" in row.expected) {
      if (row.expected.error === "invalid-unicode") continue;
      if (row.expected.error === "duplicate-member") expect(() => oracle(tree!, row.source, row.policy)).toThrow("duplicate-member");
      else expect(errors.length).toBeGreaterThan(0);
    } else { expect(errors).toEqual([]); expect(oracle(tree!, row.source, row.policy)).toEqual(row.expected); expect(ajv.validate(contract.$id, row.expected)).toBe(true); }
  }
});

test("actual reader retains every numeric lexeme, string code unit, order, and duplicate decision", async () => {
  const owner = await api();
  for (const row of fixture.cases) {
    if ("error" in row.expected) await expect(owner.decodeJsonSyntax(row.source, row.policy, operation())).rejects.toMatchObject({ code: row.expected.error });
    else expect(await owner.decodeJsonSyntax(row.source, row.policy, operation())).toEqual(row.expected);
  }
});

test("actual byte accounting, depth, nodes and asynchronous cancellation have finite authority", async () => {
  const owner = await api();
  const events: { bytes: number; nodes: number }[] = [];
  const source = '["ä😀",{"a":1}]';
  await owner.decodeJsonSyntax(source, "Reject", { ...operation(), progress: (value: { bytes: number; nodes: number }) => events.push({ ...value }) });
  expect(events.at(-1)!.bytes).toBe(new TextEncoder().encode(source).length);
  for (let index = 1; index < events.length; index++) { expect(events[index]!.bytes).toBeGreaterThanOrEqual(events[index - 1]!.bytes); expect(events[index]!.nodes).toBeGreaterThanOrEqual(events[index - 1]!.nodes); }
  for (const [name, code] of [["maximumBytes", "byte-budget"], ["maximumNodes", "node-budget"], ["maximumDepth", "depth-budget"]] as const) await expect(owner.decodeJsonSyntax("[[true]]", "Reject", { ...operation(), [name]: 1 })).rejects.toMatchObject({ code });
  await expect(owner.decodeJsonSyntax("true", "Reject", { ...operation(), cancelled: () => true })).rejects.toMatchObject({ code: "cancelled" });
  let cancelled = false, yields = 0;
  await expect(owner.decodeJsonSyntax('"' + "x".repeat(20000) + '"', "Reject", { ...operation(), chunk: 32, cancelled: () => cancelled, yield: async () => {
    yields++; setTimeout(() => { cancelled = true; }, 0); await new Promise<void>(resolve => setTimeout(resolve, 1));
  } })).rejects.toMatchObject({ code: "cancelled" });
  expect(yields).toBe(1);
  await expect(owner.decodeJsonSyntax("true", undefined, operation())).rejects.toMatchObject({ code: "invalid-policy" });
});

test("strict own API needs no external runtime or product dependency", async () => {
  await api();
  const path = resolve(root, "🟦️.ts"), source = readFileSync(path, "utf8");
  const ast = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
  const imports = ast.statements.filter(ts.isImportDeclaration).map(node => (node.moduleSpecifier as ts.StringLiteral).text);
  expect(imports.every(value => value.startsWith(".") && !value.includes("🛍️products") && !value.includes("🌎️hub") && !value.includes("🖥️s"))).toBe(true);
  const program = ts.createProgram([path], { noEmit: true, strict: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, lib: ["lib.es2022.d.ts", "lib.dom.d.ts"] });
  expect(ts.getPreEmitDiagnostics(program).map(value => ts.flattenDiagnosticMessageText(value.messageText, "\n"))).toEqual([]);
});

test("operation limits and callbacks are captured once without invoking accessors", async () => {
  const owner = await api();
  const original = operation();
  original.chunk = 1;
  let mutated = false, oldCallbacks = 0;
  original.progress = () => { oldCallbacks++; };
  original.yield = async () => { if (!mutated) { mutated = true; original.maximumBytes = 1; original.progress = () => { throw Error("uncaptured callback"); }; } };
  expect(await owner.decodeJsonSyntax("[true]", "Reject", original)).toEqual({ kind: "array", items: [{ kind: "boolean", value: true }] });
  expect(mutated).toBe(true);
  expect(oldCallbacks).toBeGreaterThan(0);
  let reads = 0;
  const accessor = operation();
  Object.defineProperty(accessor, "maximumBytes", { enumerable: true, get() { reads++; return 100; } });
  await expect(owner.decodeJsonSyntax("true", "Reject", accessor)).rejects.toMatchObject({ code: "invalid-control" });
  expect(reads).toBe(0);
});
