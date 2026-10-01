/** 🧪️ Ajv (third-party oracle) and the hand-written `dragPathPoints` parser agree on the committed fixture and on every hostile variant. */
import { expect, test } from "bun:test";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import mutation from "../../../../../🧫️fixtures/🧬️mutations/📍️drag-path-points/📍️drags/🦠️mutation/🔣️.json";
import { parseDragPathPoints } from "../../🦠️mutation/🟦️.ts";

test("the schema and the parser admit exactly the same dragPathPoints records", () => {
  const validate = semioSchemaAjvV1({ allErrors: true }).compile(schema);
  expect(validate(mutation)).toBe(true);
  const { mutation: _tag, ...payload } = mutation;
  expect(parseDragPathPoints(mutation)).toEqual(payload);
  for (const candidate of [{ ...mutation, targets: [] }, { ...mutation, targets: [{ layerId: "path-a", index: -1, point: "anchor" }] }, { ...mutation, targets: [{ layerId: "path-a", index: 1.5, point: "anchor" }] }, { ...mutation, targets: [{ layerId: "path-a", index: 0, point: "tangent" }] }, { ...mutation, targets: [{ layerId: "path-a", index: 0, point: "control1" }] }, { ...mutation, targets: [{ layerId: "path-a", index: 0, point: "anchor", extra: 1 }] }, { ...mutation, targets: [{ index: 0, point: "anchor" }] }, { ...mutation, dx: "1" }, { ...mutation, extra: true }] as unknown[]) {
    const accepted = validate(candidate);
    if (accepted) expect(parseDragPathPoints(candidate)).toEqual(Object.fromEntries(Object.entries(candidate as Record<string, unknown>).filter(([key]) => key !== "mutation")));
    else expect(() => parseDragPathPoints(candidate), JSON.stringify(candidate)).toThrow();
  }
});
