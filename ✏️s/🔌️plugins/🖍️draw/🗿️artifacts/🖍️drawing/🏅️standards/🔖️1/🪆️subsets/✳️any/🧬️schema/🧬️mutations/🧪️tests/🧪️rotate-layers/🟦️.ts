/** 🧪️ Ajv (third-party oracle) and the hand-written `rotateLayers` parser agree on the committed fixture and on every hostile variant. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import schema from "../../../../../🔀️transform/🧬️schema/🧬️mutations/🧭️rotate-layers/🧬️schema/🔣️.json";
import mutation from "../../../../../🔀️transform/🧪️tests/🧫️fixtures/🧬️mutations/🧭️rotate-layers/🧭️quarter-turn/🦠️mutation/🔣️.json";
import { parseRotateLayers } from "../../../../../🔀️transform/🧬️schema/🧬️mutations/🧭️rotate-layers/🦠️mutation/🟦️.ts";

test("the schema and the parser admit exactly the same rotateLayers records", () => {
  const validate = semioSchemaAjvV1({ allErrors: true }).compile<ReturnType<typeof parseRotateLayers> & { mutation: string }>(schema);
  expect(validate(mutation)).toBe(true);
  const { mutation: _tag, ...payload } = mutation;
  expect(parseRotateLayers(mutation)).toEqual(payload);
  for (const candidate of [{ ...mutation, targets: [] }, { ...mutation, targets: ["shape-a", "shape-a"] }, { ...mutation, angle: "90" }, { ...mutation, pivotX: null }, { ...mutation, extra: true }, { ...mutation, mutation: "scaleLayers" }, { ...mutation, angle: 0 }, { ...mutation, angle: -3.5 }] as unknown[]) {
    if (validate(candidate)) {
      const { mutation: _tag, ...expected } = candidate;
      expect(parseRotateLayers(candidate)).toEqual(expected);
    } else expect(() => parseRotateLayers(candidate), JSON.stringify(candidate)).toThrow();
  }
});
