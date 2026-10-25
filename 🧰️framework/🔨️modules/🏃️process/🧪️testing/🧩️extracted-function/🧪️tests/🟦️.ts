import { expect, test } from "bun:test";
import { evaluateExtractedFunctionV1 } from "../🟦️.ts";

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
