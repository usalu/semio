/** 🧪️ Ajv (third-party oracle) and the hand-written `dragBlocks` parser agree on the committed witness and on every hostile variant. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import schema from "../../../../../🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🧬️schema/🔣️.json";
import mutation from "../../../../../🧱️block/🧫️fixtures/🧬️mutations/🤏️drag-blocks/🤏️nudges/🦠️mutation/🔣️.json";
import { parseDragBlocks } from "../../../../../🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🟦️.ts";

test("the schema and the parser admit exactly the same dragBlocks records", () => {
  const validate = semioSchemaAjvV1({ allErrors: true }).compile(schema);
  expect(validate(mutation)).toBe(true);
  const { mutation: _tag, ...payload } = mutation;
  expect(parseDragBlocks(mutation)).toEqual(payload);
  const [first] = mutation.ids;
  for (const candidate of [{ ...mutation, ids: [] }, { ...mutation, ids: [first, first] }, { ...mutation, ids: [7] }, { ...mutation, ids: first }, { ...mutation, dx: "10" }, { ...mutation, dy: null }, { ...mutation, extra: true }, { ...mutation, mutation: "moveBlock" }, { ids: mutation.ids, dx: 1, dy: 2 }, { ...mutation, dx: 0, dy: 0 }, { ...mutation, ids: [first] }, { ...mutation, dx: -2.5 }] as unknown[]) {
    const accepted = validate(candidate);
    if (accepted) expect(parseDragBlocks(candidate)).toEqual(Object.fromEntries(Object.entries(candidate as Record<string, unknown>).filter(([key]) => key !== "mutation")));
    else expect(() => parseDragBlocks(candidate), JSON.stringify(candidate)).toThrow();
  }
});
