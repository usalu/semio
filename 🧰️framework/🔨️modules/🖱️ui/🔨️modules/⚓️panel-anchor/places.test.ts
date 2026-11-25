import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020";
import schema from "./places.schema.json";
import cases from "./places.cases.json";
import { ANCHORS, type Anchor, anchorPlaceRect, anchorPlaces, nearestAnchor } from "./🟦️.ts";
import { dropPlaceAt } from "../🫳️pointer-drag/places.ts";

describe("pane anchor places", () => {
  test("the schema validates the case file", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const anchor of ANCHORS) test(`${anchor} is its cell of the three by three grid over the host`, () => expect(anchorPlaceRect(anchor, cases.host)).toEqual((cases.places as Record<string, unknown>)[anchor]));
  test("the eight places lie inside the host and cover no more than the host", () => {
    const area = anchorPlaces(cases.host).reduce((sum, { rect }) => sum + (rect.right - rect.left) * (rect.bottom - rect.top), 0);
    expect(area).toBe(cases.host.width * cases.host.height * (8 / 9));
  });
  for (const fixture of cases.pointers) {
    test(fixture.name, () => {
      expect(nearestAnchor(fixture.point.x, fixture.point.y, cases.host)).toBe(fixture.anchor);
      const places = anchorPlaces(cases.host);
      const under = dropPlaceAt(places, fixture.point.x, fixture.point.y);
      if (under !== null) expect(under).toBe(fixture.anchor);
    });
  }
  test("every anchor has exactly one place", () => expect(anchorPlaces(cases.host).map(({ id }) => id as Anchor).sort()).toEqual([...ANCHORS].sort()));
});
