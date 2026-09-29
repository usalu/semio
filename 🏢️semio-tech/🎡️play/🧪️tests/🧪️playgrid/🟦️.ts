export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any): Promise<void> {
  const { PLAY_PANES, PLAY_LOCALE, PLAY_TERMINOLOGY } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🧪️PlayGridTests
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
