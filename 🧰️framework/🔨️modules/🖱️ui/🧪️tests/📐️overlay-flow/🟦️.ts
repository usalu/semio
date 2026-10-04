/** 📐️ Admits the shared Overlay/Absolute fixture with independently validated symbolic spacing. */
import { expect, it } from "vitest";
import { semioSchemaAjvV1 } from "../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import fixture from "../../🧫️fixtures/📐️overlay-flow/🔣️.json";
import schema from "../../🧬️schema/📐️overlay-flow/🔣️.json";

it("admits the closed Overlay/Absolute fixture and refuses foreign geometry metadata", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, foreign: true })).toBe(false);
  expect(validate({ ...fixture, overlay: { ...fixture.overlay, expectedRect: { ...fixture.overlay.expectedRect, foreign: true } } })).toBe(false);
});
