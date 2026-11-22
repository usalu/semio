/** 📊️ The table model law in TypeScript: the same vectors the Rust table model answers.
 *
 * 🧫️ Read from `🖱️ui/🧪️tests/🧫️fixtures/📊️table-column-model/🔣️.json`.
 */
import { describe, expect, it } from "vitest";
import fixture from "../🧫️fixtures/📊️table-column-model/🔣️.json" with { type: "json" };
import { tableWindowColumnTemplate, tableWindowNextColumnV1, tableWindowNextRowV1, tableWindowScrollLeftForColumnV1, tableWindowScrollTopForRowV1 } from "../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/📊️table-window/🟦️.ts";
import { tableSortAriaV1, tableSortNextDirectionV1, type SortDirection } from "../../🧱️elements/📊️Table/🟦️.tsx";

describe("📊️ table column model", () => {
  for (const c of fixture.templates) {
    it(`template ${c.columns}/${c.actions}/${c.leading}/${c.trailing}`, () => {
      expect(tableWindowColumnTemplate(c.columns, c.actions, c.leading, c.trailing)).toBe(c.template);
    });
  }
  for (const c of fixture.scrollLeftForColumn) {
    it(`scrollLeft column ${c.index} from ${c.scrollLeft}`, () => {
      expect(tableWindowScrollLeftForColumnV1(c.index, c.columnPx, c.scrollLeft, c.width)).toBeCloseTo(c.expected, 3);
    });
  }
  for (const c of fixture.scrollTopForRow) {
    it(`scrollTop row ${c.index} from ${c.scrollTop}`, () => {
      expect(tableWindowScrollTopForRowV1(c.index, c.rowPx, c.scrollTop, c.height)).toBeCloseTo(c.expected, 3);
    });
  }
  for (const c of fixture.nextRow) {
    it(`row ${c.key} ${c.current}/${c.total}`, () => {
      expect(tableWindowNextRowV1(c.key, c.current, c.total, c.page)).toBe(c.expected);
    });
  }
  for (const c of fixture.nextColumn) {
    it(`column ${c.key} ${c.current}/${c.total}`, () => {
      expect(tableWindowNextColumnV1(c.key, c.current, c.total)).toBe(c.expected);
    });
  }
  for (const c of fixture.sort) {
    it(`sort ${c.columnId} after ${c.sortColumn}`, () => {
      const column = c.sortColumn ?? undefined;
      const direction = (c.sortDirection ?? undefined) as SortDirection | undefined;
      expect(tableSortNextDirectionV1(c.columnId, column, direction)).toBe(c.next);
      expect(tableSortAriaV1(c.columnId, column, direction)).toBe(c.aria ?? undefined);
    });
  }
});
