/** 🎛️ Persisted local-only display preferences: language, colour theme and text size.
 *
 * The language is stored only once the learner chooses it; until then it follows the browser. The theme follows the
 * system unless set, and the text size scales every size of the client through one CSS custom property.
 */

import { useEffect, useId, useState, type ReactElement } from "react";
import { QUIZ_LOCALES, isQuizLocale, type QuizLabelKey, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { isRecord, type LocalStore } from "../💾️persistence/🟦️.ts";

/** 🌓️ A colour theme choice. */
export type ThemeChoice = "system" | "light" | "dark";

/** 🔠️ A text size choice. */
export type TextSize = "normal" | "large" | "larger" | "largest";

/** 🌓️ Every {@link ThemeChoice} with its label. */
export const THEME_CHOICES: readonly { readonly value: ThemeChoice; readonly label: QuizLabelKey }[] = [
  { value: "system", label: "quiz.preferences.themeSystem" },
  { value: "light", label: "quiz.preferences.themeLight" },
  { value: "dark", label: "quiz.preferences.themeDark" },
];

/** 🔠️ Every {@link TextSize} with its label and the factor it scales text by. */
export const TEXT_SIZES: readonly { readonly value: TextSize; readonly label: QuizLabelKey; readonly scale: number }[] = [
  { value: "normal", label: "quiz.preferences.textNormal", scale: 1 },
  { value: "large", label: "quiz.preferences.textLarge", scale: 1.125 },
  { value: "larger", label: "quiz.preferences.textLarger", scale: 1.25 },
  { value: "largest", label: "quiz.preferences.textLargest", scale: 1.5 },
];

/** 🗣️ The autonym of every {@link QuizLocale}, shown in its own language. */
export const LOCALE_NAMES: { readonly [L in QuizLocale]: QuizLabelKey } = { en: "quiz.preferences.english", de: "quiz.preferences.german" };

/** 🎛️ The display preferences of one device. */
export interface QuizPreferences {
  readonly locale?: QuizLocale;
  readonly theme: ThemeChoice;
  readonly textSize: TextSize;
}

/** 💾️ The stored preferences, falling back to the system theme and normal text. */
export function readPreferences(store: LocalStore): QuizPreferences {
  const stored = store.read("preferences");
  const record = isRecord(stored) ? stored : {};
  return {
    locale: isQuizLocale(record.locale) ? record.locale : undefined,
    theme: THEME_CHOICES.some((choice) => choice.value === record.theme) ? (record.theme as ThemeChoice) : "system",
    textSize: TEXT_SIZES.some((size) => size.value === record.textSize) ? (record.textSize as TextSize) : "normal",
  };
}

/** 💾️ Stores the preferences. */
export function writePreferences(store: LocalStore, preferences: QuizPreferences): void {
  store.write("preferences", preferences);
}

/** 🔠️ The factor text is scaled by. */
export function textScale(size: TextSize): number {
  return TEXT_SIZES.find((entry) => entry.value === size)?.scale ?? 1;
}

/** 🌗️ Whether the system prefers a dark colour scheme, following changes. */
export function useSystemDark(): boolean {
  const [query] = useState(() => (typeof window !== "undefined" && typeof window.matchMedia === "function" ? window.matchMedia("(prefers-color-scheme: dark)") : undefined));
  const [dark, setDark] = useState(query?.matches ?? false);
  useEffect(() => {
    if (query === undefined) return;
    const change = (): void => setDark(query.matches);
    query.addEventListener("change", change);
    return () => query.removeEventListener("change", change);
  }, [query]);
  return dark;
}

/** 🌓️ The theme to render for a choice. */
export function resolvedTheme(choice: ThemeChoice, systemDark: boolean): "light" | "dark" {
  return choice === "system" ? (systemDark ? "dark" : "light") : choice;
}

/** 🗣️ The language switch: one pressed button per language, each named in its own language. */
export function LanguageSwitch(props: { readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (locale: QuizLocale) => void }): ReactElement {
  const { locale, text, onChange } = props;
  return (
    <div className="quiz-language" role="group" aria-label={text("quiz.preferences.language")}>
      {QUIZ_LOCALES.map((option) => (
        <button key={option} type="button" lang={option} aria-pressed={option === locale} onClick={() => onChange(option)}>
          {text(LOCALE_NAMES[option])}
        </button>
      ))}
    </div>
  );
}

/** 🎛️ Theme and text size as two radio groups. */
export function PreferencesPanel(props: { readonly preferences: QuizPreferences; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void }): ReactElement {
  const { preferences, text, onChange } = props;
  const name = useId();
  return (
    <div className="quiz-preferences">
      <fieldset>
        <legend>{text("quiz.preferences.theme")}</legend>
        {THEME_CHOICES.map((choice) => (
          <label key={choice.value} className="quiz-choice">
            <input type="radio" name={`${name}-theme`} value={choice.value} checked={preferences.theme === choice.value} onChange={() => onChange({ ...preferences, theme: choice.value })} />
            {text(choice.label)}
          </label>
        ))}
      </fieldset>
      <fieldset>
        <legend>{text("quiz.preferences.textSize")}</legend>
        {TEXT_SIZES.map((size) => (
          <label key={size.value} className="quiz-choice">
            <input type="radio" name={`${name}-text`} value={size.value} checked={preferences.textSize === size.value} onChange={() => onChange({ ...preferences, textSize: size.value })} />
            {text(size.label)}
          </label>
        ))}
      </fieldset>
    </div>
  );
}
