import { describe, expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./exclusivity.cases.json";
import schema from "./exclusivity.schema.json";
import { resolveWindowPaneExpansion, windowQuickActionsCovered, type WindowPaneId } from "./🟦️.ts";

describe("window pane exclusivity", () => {
  test("neutral exclusivity schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  for (const entry of cases.cases) test(entry.name, () => {
    const expanded = resolveWindowPaneExpansion(entry.open, entry.last as WindowPaneId | null, entry.exclusive);
    expect(expanded).toEqual(entry.expanded);
    expect(windowQuickActionsCovered(expanded, entry.exclusive)).toBe(entry.quickActionsCovered);
  });
});
