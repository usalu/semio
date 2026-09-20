export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any): Promise<void> {
  const { playGridDimensions, playGridRowSpan, playOccupiedColumnRange, playPaneGridCell, playNextWarmBootPane, playPanesOverBudget, schedulePlayIdle, PLAY_PANES, PLAY_LOCALE, PLAY_TERMINOLOGY } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🧪️PlayGridTests
  describe("playGridDimensions", () => {
    it("leaves no empty cell whenever a near-square shape fits exactly", () => {
      expect(playGridDimensions(1)).toEqual({ columns: 1, rows: 1 });
      expect(playGridDimensions(6)).toEqual({ columns: 3, rows: 2 });
      expect(playGridDimensions(9)).toEqual({ columns: 3, rows: 3 });
      expect(playGridDimensions(56)).toEqual({ columns: 8, rows: 7 });
      expect(playGridDimensions(64)).toEqual({ columns: 8, rows: 8 });
    });

    it("lays eight panes out exactly like the demonstrator's own gapless 4×2", () => {
      expect(playGridDimensions(8)).toEqual({ columns: 4, rows: 2 });
    });

    it("prefers the shape with the fewest empty cells, then the squarest", () => {
      expect(playGridDimensions(58)).toEqual({ columns: 9, rows: 7 });
      expect(playGridDimensions(61)).toEqual({ columns: 9, rows: 7 });
      expect(playGridDimensions(63)).toEqual({ columns: 9, rows: 7 });
      expect(playGridDimensions(5)).toEqual({ columns: 3, rows: 2 });
    });

    it("never grows wider than two columns past square", () => {
      for (let count = 1; count <= 200; count += 1) {
        const { columns, rows } = playGridDimensions(count);
        expect(columns * rows).toBeGreaterThanOrEqual(count);
        expect(columns).toBeGreaterThanOrEqual(rows);
        expect(columns - rows).toBeLessThanOrEqual(2);
        expect((columns - 1) * rows).toBeLessThan(count);
      }
    });

    it("refuses an empty grid", () => {
      expect(() => playGridDimensions(0)).toThrow(/at least one pane/);
    });
  });

  describe("playGridRowSpan", () => {
    it("fills every row but the last edge to edge", () => {
      expect(playGridRowSpan(0, 58)).toEqual({ first: 0, last: 8 });
      expect(playGridRowSpan(5, 58)).toEqual({ first: 0, last: 8 });
    });

    it("centres a short trailing row", () => {
      expect(playGridRowSpan(6, 58)).toEqual({ first: 2, last: 5 });
    });

    it("leaves a full trailing row edge to edge", () => {
      expect(playGridRowSpan(6, 63)).toEqual({ first: 0, last: 8 });
    });

    it("refuses a row outside the grid", () => {
      expect(() => playGridRowSpan(7, 58)).toThrow(/outside the grid/);
      expect(() => playGridRowSpan(-1, 58)).toThrow(/outside the grid/);
    });
  });

  describe("playOccupiedColumnRange", () => {
    it("clamps a pan resting on the short trailing row to its occupied columns", () => {
      expect(playOccupiedColumnRange(6, 58)).toEqual({ first: 2, last: 5 });
    });

    it("opens the full range while a full row fills most of the viewport", () => {
      expect(playOccupiedColumnRange(0, 58)).toEqual({ first: 0, last: 8 });
      expect(playOccupiedColumnRange(5.4, 58)).toEqual({ first: 0, last: 8 });
    });

    it("clamps as soon as the short row takes the majority of the viewport", () => {
      expect(playOccupiedColumnRange(5.6, 58)).toEqual({ first: 2, last: 5 });
      expect(playOccupiedColumnRange(5.98, 58)).toEqual({ first: 2, last: 5 });
    });

    it("never lets a clamped pan leave the grid", () => {
      expect(playOccupiedColumnRange(-3, 58)).toEqual({ first: 0, last: 8 });
      expect(playOccupiedColumnRange(99, 58)).toEqual({ first: 2, last: 5 });
    });

    it("keeps every pane of the live grid reachable by its own pan offset", () => {
      const count = PLAY_PANES.length;
      for (let index = 0; index < count; index += 1) {
        const cell = playPaneGridCell(index, count);
        const range = playOccupiedColumnRange(cell.row, count);
        expect(cell.column).toBeGreaterThanOrEqual(range.first);
        expect(cell.column).toBeLessThanOrEqual(range.last);
      }
    });
  });

  describe("playPaneGridCell", () => {
    it("keeps every full row row-major", () => {
      expect(playPaneGridCell(0, 58)).toEqual({ column: 0, row: 0 });
      expect(playPaneGridCell(8, 58)).toEqual({ column: 8, row: 0 });
      expect(playPaneGridCell(9, 58)).toEqual({ column: 0, row: 1 });
      expect(playPaneGridCell(53, 58)).toEqual({ column: 8, row: 5 });
    });

    it("centres a short trailing row", () => {
      expect(playPaneGridCell(54, 58)).toEqual({ column: 2, row: 6 });
      expect(playPaneGridCell(57, 58)).toEqual({ column: 5, row: 6 });
    });

    it("leaves a full trailing row untouched", () => {
      expect(playPaneGridCell(63, 64)).toEqual({ column: 7, row: 7 });
    });

    it("refuses an index outside the grid", () => {
      expect(() => playPaneGridCell(58, 58)).toThrow(/outside the grid/);
      expect(() => playPaneGridCell(-1, 58)).toThrow(/outside the grid/);
    });

    it("never places two panes in the same cell and never leaves the grid", () => {
      const { columns, rows } = playGridDimensions(PLAY_PANES.length);
      const cells = PLAY_PANES.map((_: unknown, index: number) => playPaneGridCell(index, PLAY_PANES.length));
      expect(new Set(cells.map((cell: { column: number; row: number }) => `${cell.column}/${cell.row}`)).size).toBe(PLAY_PANES.length);
      for (const cell of cells) {
        expect(cell.column).toBeLessThan(columns);
        expect(cell.row).toBeLessThan(rows);
      }
    });
  });

  describe("playNextWarmBootPane", () => {
    it("warms the first never-booted pane in grid order", () => {
      expect(playNextWarmBootPane(["a", "b", "c"], new Set(["a"]), 1, null, 4)).toEqual({ kind: "boot", id: "b" });
    });

    it("never warms while a pane is focused full screen", () => {
      expect(playNextWarmBootPane(["a", "b"], new Set(), 0, "a", 4)).toEqual({ kind: "hold", reason: "focused" });
    });

    it("stops once the live budget is full so it can never evict a touched pane", () => {
      expect(playNextWarmBootPane(["a", "b", "c", "d", "e"], new Set(["a", "b", "c", "d"]), 4, null, 4)).toEqual({ kind: "hold", reason: "budget" });
    });

    it("resumes as soon as suspension frees a slot", () => {
      expect(playNextWarmBootPane(["a", "b", "c", "d", "e"], new Set(["a", "b", "c", "d"]), 3, null, 4)).toEqual({ kind: "boot", id: "e" });
    });

    it("holds when every pane has been booted once", () => {
      expect(playNextWarmBootPane(["a", "b"], new Set(["a", "b"]), 1, null, 4)).toEqual({ kind: "hold", reason: "complete" });
    });
  });

  describe("playPanesOverBudget", () => {
    it("suspends the least recently used pristine panes first", () => {
      expect(playPanesOverBudget(["a", "b", "c", "d"], new Set(), null, 2)).toEqual(["a", "b"]);
    });

    it("never suspends the focused or an interacted pane", () => {
      expect(playPanesOverBudget(["a", "b", "c"], new Set(["a"]), "b", 1)).toEqual(["c"]);
    });

    it("suspends nothing within budget", () => {
      expect(playPanesOverBudget(["a"], new Set(), null, 4)).toEqual([]);
    });
  });

  describe("schedulePlayIdle", () => {
    it("never enters the idle queue before the minimum delay", () => {
      const delayed: (() => void)[] = [];
      let calls = 0;
      schedulePlayIdle(() => { calls += 1; }, 1_500, { setTimeout: (callback: () => void) => { delayed.push(callback); return 1; }, clearTimeout: () => undefined, requestIdleCallback: (callback: () => void) => { callback(); return 2; } });
      expect(calls).toBe(0);
      delayed.shift()?.();
      expect(calls).toBe(1);
    });
  });

  describe("PLAY_PANES", () => {
    it("locks every pane to English and native terminology", () => {
      expect(PLAY_LOCALE).toBe("en");
      expect(PLAY_TERMINOLOGY).toBe("native");
      for (const pane of PLAY_PANES) expect(pane.brand.locks).toEqual({ locale: "en", terminology: "native", themeId: "semio" });
    });

    it("gives every pane a unique id and brand", () => {
      expect(new Set(PLAY_PANES.map((pane: any) => pane.id)).size).toBe(PLAY_PANES.length);
      expect(new Set(PLAY_PANES.map((pane: any) => pane.brand.id)).size).toBe(PLAY_PANES.length);
    });

    it("never carries German partner terminology", () => {
      const text = JSON.stringify(PLAY_PANES);
      expect(text).not.toMatch(/entwerfen|bestand|reuse|wiederverw/i);
    });
  });
  //#endregion 🧪️PlayGridTests
}
