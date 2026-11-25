import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./bands.cases.json";
import schema from "./bands.schema.json";
import { snapCardHeight } from "./bands.ts";

describe("card stack height snapping", () => {
  test("neutral bands schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const snap of cases.snaps) test(`snap ${snap.height} @ ${snap.pixel}`, () => expect(snapCardHeight(snap.height, snap.pixel)).toBeCloseTo(snap.expected, 9));
});
