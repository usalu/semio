import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import ceiling from "./🔣️.json" with { type: "json" };
import schema from "./schema.json" with { type: "json" };
import { evaluateExtractedFunctionV1, loadProductFunctionsSyncV1, loadProductFunctionsV1 } from "../🟦️.ts";

const source = "async function law(a, b) { const sum = add(a, b); const wrapped = await phase('x', () => sum * factor); return { wrapped, missing: unknownPort(1), length: Math.max(a, b) }; }";

test("explicit ports win, globals resolve, and an unknown dependency is a pass-through stub", async () => {
  const law = evaluateExtractedFunctionV1(source, "law", { add: (left: number, right: number) => left + right, factor: 10, phase: async (_label: string, body: () => number) => body() });
  expect(await law(1, 2)).toEqual({ wrapped: 30, missing: undefined, length: 2 });
});

test("a stub passes the last function argument through, so an unported wrapper still runs its body", async () => {
  const law = evaluateExtractedFunctionV1("function run() { return wrapUnknown('label', () => 7); }", "run", {});
  expect(law()).toBe(7);
});

test("the named function must be defined by the evaluated code", () => {
  expect(() => evaluateExtractedFunctionV1("const other = 1;", "law", {})).toThrow();
});

test("import.meta inside the evaluated code resolves to the supplied meta object", () => {
  const law = evaluateExtractedFunctionV1("function where() { return import.meta.dirname; }", "where", { __importMeta: { dirname: "/here" } });
  expect(law()).toBe("/here");
});

test("a loaded product function reaches its module-level helpers and imports transitively, and only ported names are replaced", async () => {
  const loaded = await loadProductFunctionsV1(new URL("../🟦️.ts", import.meta.url), ["evaluateExtractedFunctionV1"], {});
  expect(loaded.evaluateExtractedFunctionV1("function f() { return 3; }", "f", {})()).toBe(3);
  const ported = await loadProductFunctionsV1(new URL("../🟦️.ts", import.meta.url), ["evaluateExtractedFunctionV1"], { refuseProcessV1: () => new Error("ported") });
  expect(() => ported.evaluateExtractedFunctionV1("const other = 1;", "f", {})).toThrow("ported");
});

test("a function missing from the product module is refused at load", async () => {
  await expect(loadProductFunctionsV1(new URL("../🟦️.ts", import.meta.url), ["movedAwayV1"], {})).rejects.toThrow();
});

test("the synchronous loader resolves the same helpers and imports as the asynchronous one", async () => {
  const sync = loadProductFunctionsSyncV1(new URL("../🟦️.ts", import.meta.url), ["evaluateExtractedFunctionV1"], {});
  expect(sync.evaluateExtractedFunctionV1("function f() { return 5; }", "f", {})()).toBe(5);
  expect(() => loadProductFunctionsSyncV1(new URL("../🟦️.ts", import.meta.url), ["movedAwayV1"], {})).toThrow();
});

/** 📉️ Ratchet over every test below the framework root that evaluates product function text (`evaluateExtractedFunctionV1` or `new Function(`): each file has a ceiling of such uses in `🔣️.json`, a new file or a higher count is refused, and a lower count must lower the ceiling, so the list only shrinks; the replacement is `loadProductFunctionsV1`. */
function textExtractionUsesV1(): Record<string, number> {
  const root = fileURLToPath(new URL("../../../../../", import.meta.url));
  const skipped = new Set(["node_modules", "target", ".cache", "🗑️generated", "dist"]);
  const found: Record<string, number> = {};
  const walk = (folder: string, relative: string): void => {
    for (const entry of readdirSync(folder, { withFileTypes: true })) {
      const next = relative === "" ? entry.name : relative + "/" + entry.name;
      if (entry.isDirectory()) { if (!skipped.has(entry.name) && entry.name !== "🧩️extracted-function") walk(folder + "/" + entry.name, next); continue; }
      if (!/\.(ts|mjs|js)$/u.test(entry.name) || !/🧪️tests|🧪️\.ts|\.test\./u.test(next)) continue;
      const count = (readFileSync(folder + "/" + entry.name, "utf8").match(/evaluateExtractedFunctionV1|new Function\(/gu) ?? []).length;
      if (count > 0) found[next] = count;
    }
  };
  walk(root.replace(/\/$/u, ""), "");
  return found;
}

test("the text-extraction ceiling file matches its schema under Ajv and the repository validator", () => {
  expect(new Ajv({ strict: true }).validate(schema, ceiling)).toBe(true);
  expect(validateJsonSchemaSubset(schema, ceiling)).toEqual([]);
});

test("no test adds product function text evaluation and every migrated file lowers its ceiling", () => {
  const actual = textExtractionUsesV1(), allowed = ceiling.ceilings as Record<string, number>;
  const added = Object.keys(actual).filter(path => !(path in allowed) || actual[path]! > allowed[path]!);
  const slack = Object.keys(allowed).filter(path => (actual[path] ?? 0) < allowed[path]!).map(path => path + " -> " + (actual[path] ?? 0));
  expect({ added, slack }).toEqual({ added: [], slack: [] });
});

test("a ported name that shares one declaration statement with a needed name is replaced without a duplicate declaration", async () => {
  const loaded = await loadProductFunctionsV1(new URL("./🧫️fixtures/📜️multi-declarator.ts", import.meta.url), ["sumV1"], { second: 2000 });
  expect(loaded.sumV1()).toBe(2301);
  expect((await loadProductFunctionsV1(new URL("./🧫️fixtures/📜️multi-declarator.ts", import.meta.url), ["sumV1"], {})).sumV1()).toBe(321);
});

test("a ported name inside a destructuring declaration is replaced while its siblings stay real", async () => {
  const url = new URL("./🧫️fixtures/📜️multi-declarator.ts", import.meta.url);
  expect((await loadProductFunctionsV1(url, ["pairV1"], { alpha: 7 })).pairV1()).toBe(72);
  expect((await loadProductFunctionsV1(url, ["pairV1"], {})).pairV1()).toBe(12);
});
