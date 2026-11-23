import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./bands.cases.json";
import schema from "./bands.schema.json";
import { chipGapReadsAsSliver, snapCardHeight } from "./bands.ts";

describe("card stack chip seams and snapping", () => {
  test("neutral bands schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const snap of cases.snaps) test(`snap ${snap.height} @ ${snap.pixel}`, () => expect(snapCardHeight(snap.height, snap.pixel)).toBeCloseTo(snap.expected, 9));
  for (const fixture of cases.gaps) test(`gap ${fixture.gap}`, () => expect(chipGapReadsAsSliver(fixture.gap)).toBe(fixture.sliver));
});
