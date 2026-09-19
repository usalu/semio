export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any): Promise<void> {
  const { playGridDimensions, playPanesOverBudget, schedulePlayIdle, PLAY_PANES, PLAY_LOCALE, PLAY_TERMINOLOGY } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🧪️PlayGridTests
  describe("playGridDimensions", () => {
    it("builds the most square grid that holds every pane", () => {
      expect(playGridDimensions(1)).toEqual({ columns: 1, rows: 1 });
      expect(playGridDimensions(8)).toEqual({ columns: 3, rows: 3 });
      expect(playGridDimensions(58)).toEqual({ columns: 8, rows: 8 });
      expect(playGridDimensions(64)).toEqual({ columns: 8, rows: 8 });
      expect(playGridDimensions(65)).toEqual({ columns: 9, rows: 8 });
    });

    it("refuses an empty grid", () => {
      expect(() => playGridDimensions(0)).toThrow(/at least one pane/);
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
