/** 🧪️ Ajv (third-party oracle) and the hand-written `rotateLayers` parser agree on the committed fixture and on every hostile variant. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import mutation from "../../../../../🧫️fixtures/🧬️mutations/🧭️rotate-layers/🧭️quarter-turn/🦠️mutation/🔣️.json";
import { parseRotateLayers } from "../../🦠️mutation/🟦️.ts";

test("the schema and the parser admit exactly the same rotateLayers records", () => {
  const validate = semioSchemaAjvV1({ allErrors: true }).compile(schema);
  expect(validate(mutation)).toBe(true);
  const { mutation: _tag, ...payload } = mutation;
  expect(parseRotateLayers(mutation)).toEqual(payload);
  for (const candidate of [{ ...mutation, targets: [] }, { ...mutation, targets: ["shape-a", "shape-a"] }, { ...mutation, angle: "90" }, { ...mutation, pivotX: null }, { ...mutation, extra: true }, { ...mutation, mutation: "scaleLayers" }, { ...mutation, angle: 0 }, { ...mutation, angle: -3.5 }] as unknown[]) {
    const accepted = validate(candidate);
    if (accepted) expect(parseRotateLayers(candidate)).toEqual(Object.fromEntries(Object.entries(candidate as Record<string, unknown>).filter(([key]) => key !== "mutation")));
    else expect(() => parseRotateLayers(candidate), JSON.stringify(candidate)).toThrow();
  }
});
