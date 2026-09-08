type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { ADMIN_I18N, de, en } = dependencies;

  const { describe, expect, it } = vitest;

  describe("admin i18n", () => {
    it("has an identical key set in en and de", () => {
      const enKeys = Object.keys(ADMIN_I18N.en).sort();
      const deKeys = Object.keys(ADMIN_I18N.de).sort();
      expect(deKeys).toEqual(enKeys);
    });

    it("covers every admin.* namespace the app renders", () => {
      const namespaces = ["nav", "session", "overview", "spaces", "users", "connections", "documents", "events"];
      for (const namespace of namespaces) {
        expect(Object.keys(ADMIN_I18N.en).some((key) => key.startsWith(`admin.${namespace}.`))).toBe(true);
      }
    });

    it("substitutes {placeholder} vars", () => {
      expect(ADMIN_I18N.en["admin.overview.rebuildSuccess"].replace("{count}", "3")).toBe("Replayed 3 events.");
    });
  });

}
