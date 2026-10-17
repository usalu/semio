/** 🧪️ Ajv (third-party oracle) and the hand-written `scaleLayers` parser agree on the committed fixture and on every hostile variant. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import schema from "../../../../../🔀️transform/🧬️schema/🧬️mutations/📐️scale-layers/🧬️schema/🔣️.json";
import mutation from "../../../../../🔀️transform/🧪️tests/🧫️fixtures/🧬️mutations/📐️scale-layers/📐️doubles/🦠️mutation/🔣️.json";
import { parseScaleLayers } from "../../../../../🔀️transform/🧬️schema/🧬️mutations/📐️scale-layers/🦠️mutation/🟦️.ts";

test("the schema and the parser admit exactly the same scaleLayers records", () => {
  const validate = semioSchemaAjvV1({ allErrors: true }).compile<ReturnType<typeof parseScaleLayers> & { mutation: string }>(schema);
  expect(validate(mutation)).toBe(true);
  const { mutation: _tag, ...payload } = mutation;
  expect(parseScaleLayers(mutation)).toEqual(payload);
  for (const candidate of [{ ...mutation, targets: [] }, { ...mutation, scaleX: 0 }, { ...mutation, scaleY: 0 }, { ...mutation, scaleX: -1 }, { ...mutation, scaleY: "2" }, { ...mutation, extra: true }, { ...mutation, mutation: "dragLayers" }, { ...mutation, scaleX: 1, scaleY: 1 }] as unknown[]) {
    if (validate(candidate)) {
      const { mutation: _tag, ...expected } = candidate;
      expect(parseScaleLayers(candidate)).toEqual(expected);
    } else expect(() => parseScaleLayers(candidate), JSON.stringify(candidate)).toThrow();
  }
});
