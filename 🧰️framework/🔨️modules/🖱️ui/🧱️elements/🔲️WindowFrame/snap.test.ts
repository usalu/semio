import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./snap.cases.json";
import schema from "./snap.schema.json";
import { createSharedBorderSnap } from "./shared-border.ts";

describe("shared border edge snap", () => {
  test("neutral snap schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const entry of cases.cases) test(entry.name, () => {
    const snap = createSharedBorderSnap(entry.xs, entry.ys);
    for (const query of entry.queries) expect(snap(query.axis as "x" | "y", query.value)).toBe(query.expected);
  });
});
