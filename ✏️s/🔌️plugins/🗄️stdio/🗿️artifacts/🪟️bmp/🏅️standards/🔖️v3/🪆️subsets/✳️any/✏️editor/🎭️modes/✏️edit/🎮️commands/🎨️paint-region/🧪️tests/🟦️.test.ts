import { expect, test } from "bun:test";
import { runBmpPaintRegionFixtureChecks } from "../🟦️.ts";

test("BMP paint-region fixtures agree with JSON Schema", () => {
  expect(runBmpPaintRegionFixtureChecks()).toBe(4);
});
