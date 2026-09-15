/** 🔀️ The table sort-header law: a sortable header click requests the next direction for its column, and
 * only the active column announces `aria-sort`. The wgpu table target answers to the same transitions
 * (`🎞️Scenes/🧪️tests/🔬️wgpu-table`).
 *
 * 🧫️ Read from `🖱️ui/🧫️fixtures/📊️table-sort-header/🔣️.json`.
 */
import { describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/📊️table-sort-header/🔣️.json" with { type: "json" };
import { tableSortAriaV1, tableSortNextDirectionV1, type SortDirection } from "../../🧱️elements/📊️Table/🟦️.tsx";

describe("🔀️ table sort header", () => {
  it("declares the contract the header answers to", () => {
    expect(Object.values(fixture.contract).every(Boolean)).toBe(true);
  });

  for (const scenario of fixture.transitions) {
    it(scenario.name, () => {
      const sortColumn = scenario.sortColumn ?? undefined;
      const sortDirection = (scenario.sortDirection ?? undefined) as SortDirection | undefined;
      expect(tableSortNextDirectionV1(scenario.columnId, sortColumn, sortDirection)).toBe(scenario.next);
      expect(tableSortAriaV1(scenario.columnId, sortColumn, sortDirection)).toBe(scenario.aria ?? undefined);
    });
  }
});
