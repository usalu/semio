/** 🏷️ The app labels overlay over `🧫️fixtures/🏷️app-labels-overlay.json` — TypeScript twin of the Rust law beside this file (`AppLabelsOverlay::resolve`). */
import { describe, expect, test } from "bun:test";
import fixture from "../🧫️fixtures/🏷️app-labels-overlay.json";
import { normalizeAppLabelsOverlay } from "../../🗣️contributions-and-examples/🟦️.ts";

const MAP_OF_KIND = { example: "exampleLabels", action: "actionLabels", dialog: "dialogLabels" } as const;

describe("🏷️ app labels overlay", () => {
  const overlay = normalizeAppLabelsOverlay(fixture.overlay);
  for (const row of fixture.cases) {
    test(row.name, () => {
      const map = overlay[MAP_OF_KIND[row.kind as keyof typeof MAP_OF_KIND]];
      expect(map[row.id] ?? row.fallback).toBe(row.expected);
    });
  }
  test("a missing map normalizes to empty", () => {
    expect(overlay.groupLabels).toEqual({});
  });
});
