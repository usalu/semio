import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./inset.cases.json";
import schema from "./inset.schema.json";
import { windowViewportInsetV1 } from "./🟦️.ts";

const oracle = (width: number, panes: readonly { leftPx: number; rightPx: number }[], min: number) => {
  const free = Array.from({ length: width }, (_, x) => !panes.some((pane) => x >= pane.leftPx && x < pane.rightPx));
  let best = [0, 0], from = -1;
  free.concat(false).forEach((open, x) => {
    if (open && from < 0) from = x;
    if (!open && from >= 0) { if (x - from > best[1] - best[0]) best = [from, x]; from = -1; }
  });
  return panes.length === 0 || best[1] - best[0] < min ? { leftPx: 0, rightPx: 0 } : { leftPx: best[0], rightPx: width - best[1] };
};

describe("window viewport inset", () => {
  test("neutral inset schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const entry of cases.cases) test(entry.name, () => {
    expect(windowViewportInsetV1(entry.bodyWidthPx, entry.panes, cases.minViewportPx)).toEqual(entry.inset);
    expect(oracle(entry.bodyWidthPx, entry.panes, cases.minViewportPx)).toEqual(entry.inset);
  });
});
