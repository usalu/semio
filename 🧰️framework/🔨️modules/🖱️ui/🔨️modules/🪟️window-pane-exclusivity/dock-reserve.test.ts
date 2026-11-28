import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./dock-reserve.cases.json";
import schema from "./dock-reserve.schema.json";
import { windowBottomDockReservePx, type WindowRectV1 } from "./🟦️.ts";

const oracle = (body: WindowRectV1, docks: readonly WindowRectV1[]) => {
  let reserve = 0;
  for (let y = Math.ceil(body.top); y < body.bottom; y++) {
    const lowerHalf = y >= body.top + (body.bottom - body.top) / 2;
    if (lowerHalf && docks.some((dock) => dock.top === y && dock.bottom > dock.top && dock.right > body.left && dock.left < body.right)) reserve = Math.max(reserve, body.bottom - y);
  }
  return reserve;
};

describe("window bottom dock reserve", () => {
  test("neutral dock reserve schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const entry of cases.cases) test(entry.name, () => {
    expect(windowBottomDockReservePx(entry.body, entry.docks)).toBe(entry.reservePx);
    expect(oracle(entry.body, entry.docks)).toBe(entry.reservePx);
  });
});
