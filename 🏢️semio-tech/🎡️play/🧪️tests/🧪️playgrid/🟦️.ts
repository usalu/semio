import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any): Promise<void> {
  const { PLAY_PANES, resolvePlayLocale, PLAY_TERMINOLOGY } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🧪️PlayGridTests
  describe("PLAY_PANES", () => {
    it("retains native terminology and theme with a user-owned language", () => {
      expect(PLAY_TERMINOLOGY).toBe("native");
      for (const pane of PLAY_PANES) expect(pane.brand.locks).toEqual(fixture.locks);
    });

    it("resolves an explicit supported locale or requires a bilingual choice", () => {
      for (const row of fixture.cases) {
        expect(resolvePlayLocale(row.stored, row.requested)).toBe(row.expected);
        expect(new Ajv().compile({const:row.expected})(resolvePlayLocale(row.stored, row.requested))).toBe(true);
        const oracle = fixture.languages.includes(row.stored ?? "") ? row.stored : row.requested.map((language:string) => new Intl.Locale(language).language).find((language:string) => fixture.languages.includes(language)) ?? null;
        expect(resolvePlayLocale(row.stored, row.requested)).toBe(oracle);
      }
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
