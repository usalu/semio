import { expect, test } from "bun:test";
import { runBmpPaintRegionChecks } from "./🟦️.ts";

test("BMP paint-region fixtures agree with JSON Schema", () => {
  expect(runBmpPaintRegionChecks()).toBe(4);
});
