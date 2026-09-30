/** 🌐️ Explicit locale vectors validate the canonical labels through the owned resolver and i18next. */
import { expect, test } from "vitest";
import Ajv from "ajv";
import { getValueByPointer } from "fast-json-patch";
import { createShellI18nInstance, disposeShellI18nInstance, resolveUiLabel, uiChromeTranslationBundles } from "../../../../🎯️targets/⚛️react/🌐️i18n/🟦️.ts";
import fixture from "../../🧫️fixtures/🌐️translation-totality/🔣️.json";
import schema from "../../../../🌐️i18n/🧬️schema/🔣️.json";

test("canonical labels are total for both explicit shell locales", () => {
  const validate = new Ajv({ strict: true }).addKeyword("x-semio-formats").compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const vector of fixture.cases) {
    if (vector.locale !== "en" && vector.locale !== "de") throw new Error("Unsupported fixture locale");
    const instance = createShellI18nInstance(vector.locale);
    try {
      const leaf = getValueByPointer(uiChromeTranslationBundles, `/${vector.locale}/translation/${vector.key.replaceAll(".", "/")}`);
      const translated = instance.t(vector.key, { ...vector.options, returnObjects: true });
      for (const tier of ["normal", "beginner"] as const) {
        expect(resolveUiLabel(translated, tier), `${vector.locale}:${vector.key}:${tier}`).toBe(vector[tier]);
        const template = resolveUiLabel(leaf, tier);
        expect(template).toBeDefined();
        expect(instance.t(`${vector.key}.label.${tier}`, vector.options)).toBe(vector[tier]);
      }
    } finally {
      disposeShellI18nInstance(instance);
    }
  }
});
