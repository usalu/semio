/** 🧪️ Ajv (third-party oracle) and the hand-written `dragLayers` parser agree on the committed fixture and on every hostile variant. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import mutation from "../../../../../🧫️fixtures/🧬️mutations/✋️drag-layers/✋️drags-a-child/🦠️mutation/🔣️.json";
import { parseDragLayers } from "../../🦠️mutation/🟦️.ts";

test("the schema and the parser admit exactly the same dragLayers records", () => {
  const validate = semioSchemaAjvV1({ allErrors: true }).compile(schema);
  expect(validate(mutation)).toBe(true);
  const { mutation: _tag, ...payload } = mutation;
  expect(parseDragLayers(mutation)).toEqual(payload);
  for (const candidate of [{ ...mutation, targets: [] }, { ...mutation, targets: ["shape-a", "shape-a"] }, { ...mutation, targets: [7] }, { ...mutation, dx: "20" }, { ...mutation, dy: null }, { ...mutation, extra: true }, { ...mutation, mutation: "rotateLayers" }, { targets: mutation.targets, dx: 1, dy: 2 }, { ...mutation, dx: 0, dy: 0 }] as unknown[]) {
    const accepted = validate(candidate);
    if (accepted) expect(parseDragLayers(candidate)).toEqual(Object.fromEntries(Object.entries(candidate as Record<string, unknown>).filter(([key]) => key !== "mutation")));
    else expect(() => parseDragLayers(candidate), JSON.stringify(candidate)).toThrow();
  }
});
