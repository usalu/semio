/** 🌐️ Explicit locale vectors validate the canonical labels through the owned resolver and i18next. */
import { expect, test } from "vitest";

import { getValueByPointer } from "fast-json-patch";
import { createShellI18nInstance, disposeShellI18nInstance, resolveUiLabel, uiChromeTranslationBundles } from "../../../../🎯️targets/⚛️react/🌐️i18n/🟦️.ts";
import fixture from "../../🧫️fixtures/🌐️translation-totality/🔣️.json";
import outcomeCodes from "../../../../../📡️replication/🎮️mutation/🧫️fixtures/🧫️outcome-code/🔣️.json";

test("every frozen outcome code has distinct en and de labels", () => {
  const keys = [...outcomeCodes.codes.map((row) => row.code.slice("mutation.".length).replace(/-([a-z0-9])/g, (_, letter: string) => letter.toUpperCase())), "apply"];
  for (const locale of ["en", "de"] as const) {
    const instance = createShellI18nInstance(locale);
    try {
      for (const key of keys) {
        for (const tier of ["normal", "beginner"] as const) {
          const label = String(instance.t(`ui.mutation.code.${key}.label.${tier}`));
          expect(label, `${locale}:${key}:${tier}`).not.toBe(`ui.mutation.code.${key}.label.${tier}`);
          expect(label.trim().length, `${locale}:${key}:${tier}`).toBeGreaterThan(0);
        }
      }
    } finally {
      disposeShellI18nInstance(instance);
    }
  }
  for (const key of keys) expect(resolveUiLabel(getValueByPointer(uiChromeTranslationBundles, `/de/translation/ui/mutation/code/${key}`), "normal"), key).not.toBe(resolveUiLabel(getValueByPointer(uiChromeTranslationBundles, `/en/translation/ui/mutation/code/${key}`), "normal"));
});

test("canonical labels are total for both explicit shell locales", () => {
  for (const hostile of [
    { ...fixture, foreign: true },
    { cases: [{ ...fixture.cases[0], locale: "unknown" }, ...fixture.cases.slice(1)] },
    { cases: [{ ...fixture.cases[0], key: "ui.windowFault.undeclared" }, ...fixture.cases.slice(1)] },
  ])
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
