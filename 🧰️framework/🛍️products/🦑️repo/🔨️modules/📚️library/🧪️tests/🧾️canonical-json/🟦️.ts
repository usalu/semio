import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv/dist/2020.js";
import { applyEdits, findNodeAtLocation, modify, parseTree, type ParseError } from "jsonc-parser";
import ts from "typescript";

type RuntimeKind = "object-undefined" | "array-undefined" | "undefined" | "function" | "symbol";
type Corpus = Readonly<{ schemaVersion: 1; contractId: string; cases: readonly Readonly<{ id: string; input: unknown; oracle: unknown; canonical: string }>[]; runtimeCases: readonly Readonly<{ id: string; kind: RuntimeKind; canonical?: string; error?: string }>[] }>;
const bytes = readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧾️canonical-json/🔣️.json"), "utf8");
const corpus = JSON.parse(bytes) as Corpus;
const oracle = (value: unknown): string => applyEdits("", modify("", [], value, {}));

/** 🔬️ Executes the owned declaration closure with the runtime and two independent compilers. */
async function providers(): Promise<readonly Readonly<{ name: string; canonicalJson: (value: unknown) => string }>[]> {
  const actual = await import("../../🧾️serialization/🔣️json/🟦️.ts");
  const source = readFileSync(join(import.meta.dir, "../../🧾️serialization/🔣️json/🟦️.ts"), "utf8").replace(/^export /gmu, "");
  const compilers = [
    { name: "Bun", javascript: new Bun.Transpiler({ loader: "ts" }).transformSync(source) },
    { name: "TypeScript", javascript: ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
  ];
  return [{ name: "runtime", canonicalJson: actual.canonicalJson }, ...compilers.map(({ name, javascript }) => ({ name, canonicalJson: new Function("Buffer", javascript + "\nreturn canonicalJson;")(Buffer) as (value: unknown) => string }))];
}

test("repository JSON has a closed language-neutral corpus and independent serialization witnesses", () => {
  const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🧾️canonical-json/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...corpus, extra: true })).toBe(false);
  expect(validate({ ...corpus, cases: corpus.cases.map((row, index) => index === 0 ? { ...row, extra: true } : row) })).toBe(false);
  for (const extra of [{ error: "Repository JSON requires a serializable top-level value" }, { extra: true }]) expect(validate({ ...corpus, runtimeCases: corpus.runtimeCases.map((row, index) => index === 0 ? { ...row, ...extra } : row) })).toBe(false);
  expect(validate({ ...corpus, runtimeCases: corpus.runtimeCases.map((row, index) => index === 2 ? { ...row, canonical: "null" } : row) })).toBe(false);
  const errors: ParseError[] = [];
  const tree = parseTree(bytes, errors, { disallowComments: true, allowTrailingComma: false });
  expect(tree?.type).toBe("object");
  if (!tree) throw new Error("Canonical JSON corpus must parse to an owned object");
  expect(findNodeAtLocation(tree, ["contractId"])?.value).toBe(corpus.contractId);
  expect(findNodeAtLocation(tree, ["cases", 9, "input", "__proto__", "value"])?.value).toBe(1);
  expect(errors).toEqual([]);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(new Set(corpus.runtimeCases.map((row) => row.kind)).size).toBe(corpus.runtimeCases.length);
  for (const row of corpus.cases) expect(oracle(row.oracle), row.id).toBe(row.canonical);
});

test("canonical repository records preserve exact JSON bytes and leave input records untouched", async () => {
  for (const { name, canonicalJson } of await providers()) for (const row of corpus.cases) {
    const before = JSON.stringify(row.input);
    expect(canonicalJson(row.input), name + ": " + row.id).toBe(row.canonical);
    expect(canonicalJson(row.input), row.id).toBe(oracle(row.oracle));
    expect(JSON.stringify(row.input), row.id).toBe(before);
    expect(canonicalJson(JSON.parse(row.canonical)), row.id).toBe(row.canonical);
  }
  console.log("[DEBUG] canonical JSON oracle; cases=" + corpus.cases.length + "; provider=owned; oracle=jsonc-parser");
});

test("canonical repository JSON omits undefined object values and refuses nonserializable roots", async () => {
  const values: Readonly<Record<RuntimeKind, unknown>> = { "object-undefined": { z: undefined, a: 1 }, "array-undefined": [undefined, 1], undefined, function: () => {}, symbol: Symbol("root") };
  for (const { name, canonicalJson } of await providers()) for (const row of corpus.runtimeCases) {
    if (row.error) expect(() => canonicalJson(values[row.kind]), name + ": " + row.id).toThrow(row.error);
    else {
      if (row.canonical === undefined) throw new Error("Canonical JSON runtime case requires expected bytes");
      expect(canonicalJson(values[row.kind]), row.id).toBe(row.canonical);
      expect(canonicalJson(values[row.kind]), row.id).toBe(oracle(JSON.parse(row.canonical)));
    }
  }
  console.log("[DEBUG] canonical JSON runtime contract; cases=" + corpus.runtimeCases.length);
});
