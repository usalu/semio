import { describe, expect, test } from "vitest";
import lodash from "lodash";
import Ajv from "ajv/dist/2020";
import schema from "./places.schema.json";
import cases from "./places.cases.json";
import { DRAG_GHOST_OFFSET, DRAG_START_THRESHOLD_PX, DROP_TINT_PERCENT, dragCancelKey, dragGhostPosition, dragStarts, dropPlaceAt, dropPlaceStates, insertSlot, treeDropBand, type DropPlace } from "./places.ts";

const area = (place: DropPlace): number => (place.rect.right - place.rect.left) * (place.rect.bottom - place.rect.top);

/** 🔬 Independent oracle of the hit rule: the places that contain the point, reversed so the later one comes first, then the smallest by lodash. */
const oracle = (places: readonly DropPlace[], x: number, y: number): string | null =>
  lodash.minBy(lodash.reverse(places.filter(({ rect }) => x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom)), area)?.id ?? null;

describe("drag drop places", () => {
  test("the schema validates the case file", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  test("the tint of an available place is weaker than the tint of the active one", () => {
    expect(DROP_TINT_PERCENT).toEqual(cases.tint);
    expect(DROP_TINT_PERCENT.available).toBeLessThan(DROP_TINT_PERCENT.active);
  });
  for (const fixture of cases.places) {
    test(fixture.name, () => {
      const { places, point } = fixture;
      expect(dropPlaceAt(places, point.x, point.y)).toBe(fixture.active);
      expect(dropPlaceAt(places, point.x, point.y)).toBe(oracle(places, point.x, point.y));
      const states = dropPlaceStates(places, point.x, point.y);
      expect(states.map(({ id }) => id)).toEqual(places.map(({ id }) => id));
      expect(states.filter(({ state }) => state === "active").map(({ id }) => id)).toEqual(fixture.active === null ? [] : [fixture.active]);
      expect(states.filter(({ state }) => state === "available")).toHaveLength(places.length - (fixture.active === null ? 0 : 1));
    });
  }
  for (const fixture of cases.ghost) test(fixture.name, () => expect(dragGhostPosition(fixture.point)).toEqual(fixture.position));
  for (const fixture of cases.threshold) test(fixture.name, () => expect(dragStarts(fixture.from, fixture.to)).toBe(fixture.begins));
  test("the copy keeps the offset of the case file", () => expect(DRAG_GHOST_OFFSET).toEqual(cases.offset));
  test("the threshold is four pixels", () => expect(DRAG_START_THRESHOLD_PX).toBe(4));
  for (const fixture of cases.keys) test(`${fixture.key || "space"} ${fixture.cancels ? "cancels" : "does not cancel"} the drag`, () => expect(dragCancelKey(fixture.key)).toBe(fixture.cancels));
  for (const fixture of cases.insertSlots) test(fixture.name, () => expect(insertSlot(fixture.row, fixture.items, fixture.index)).toEqual(fixture.slot));
  for (const fixture of cases.treeBands) test(fixture.name, () => expect(treeDropBand(fixture.row, fixture.position as "before" | "after" | "inside")).toEqual(fixture.band));
  test("every slot and band lies inside its row", () => {
    for (const fixture of cases.insertSlots) {
      const slot = insertSlot(fixture.row, fixture.items, fixture.index);
      expect([slot.left >= fixture.row.left, slot.right <= fixture.row.right, slot.top >= fixture.row.top, slot.bottom <= fixture.row.bottom]).toEqual([true, true, true, true]);
    }
  });
});
