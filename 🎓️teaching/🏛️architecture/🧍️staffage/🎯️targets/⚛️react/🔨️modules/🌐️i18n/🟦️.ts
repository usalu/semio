/** 🌐️ Every string the archive app itself shows, in English and German; there is no default language.
 *
 * The reader's language is the stored explicit choice, else the first language of the browser's list the app speaks (a preselection the switch shows and changes), else none:
 * then the app asks in every language at once. Catalog texts are data and arrive as `{ en, de }`; only the app's own chrome lives in the bundles.
 * Counts are written as "Label: n", so no sentence needs a plural form.
 *
 * @see ./🔠️english/🟦️.ts — the key set every language repeats
 * @see ./🔡️german/🟦️.ts — German, checked key for key against English by the compiler
 */
import { LANGUAGES } from "@teaching/staffage";
import type { Language, Text } from "@teaching/staffage";
import { ENGLISH } from "./🔠️english/🟦️.ts";
import { GERMAN } from "./🔡️german/🟦️.ts";

/** 🔑️ Every key of the app chrome. */
export type Key = keyof typeof ENGLISH;

/** 🔤️ Values that fill `{name}` placeholders of a bundle string. */
export type Params = Readonly<Record<string, string | number>>;

/** 🗣️ A translator bound to one language. */
export type Translate = (key: Key, params?: Params) => string;

/** 🔃️ The offered languages in order: English, then German. */
export const OFFERED: readonly Language[] = LANGUAGES;

/** 🏷️ Each language's own name, shown in its own language. */
export const LANGUAGE_NAMES: Readonly<Record<Language, string>> = { en: "English", de: "Deutsch" };

const BUNDLES: Readonly<Record<Language, Readonly<Record<Key, string>>>> = { en: ENGLISH, de: GERMAN };

/** ❓️ Whether `value` names an offered language. */
export function isLanguage(value: unknown): value is Language {
  return typeof value === "string" && (OFFERED as readonly string[]).includes(value);
}

/** 🧭️ The first of the browser's BCP 47 tags (preference order) the app speaks; none when no tag matches, so nothing is assumed. */
export function preferredLanguage(languages: readonly string[]): Language | undefined {
  for (const tag of languages) {
    const primary = tag.toLowerCase().split("-")[0];
    if (isLanguage(primary)) return primary;
  }
  return undefined;
}

/** 🧭️ How the reader's language was found. */
export type LanguageResolution = { readonly language: Language | undefined; readonly source: "stored" | "browser" | "none" };

/** 🧭️ Stored choice first, then the browser's list, else none. */
export function resolveLanguage(stored: unknown, browser: readonly string[]): LanguageResolution {
  if (isLanguage(stored)) return { language: stored, source: "stored" };
  const preselected = preferredLanguage(browser);
  return preselected === undefined ? { language: undefined, source: "none" } : { language: preselected, source: "browser" };
}

/** 🗣️ The translator of a language: `{name}` placeholders are replaced from `params`. */
export function translator(language: Language): Translate {
  const bundle = BUNDLES[language];
  return (key, params) => (params === undefined ? bundle[key] : bundle[key].replace(/\{(\w+)\}/gu, (whole, name: string) => (name in params ? String(params[name]) : whole)));
}

/** 🌈️ The same key in every offered language, English first, for the question no language is assumed for. */
export function inEveryLanguage(key: Key): readonly { readonly language: Language; readonly text: string }[] {
  return OFFERED.map((language) => ({ language, text: BUNDLES[language][key] }));
}

/** 🌍️ A catalog text in a language. */
export const localized = (text: Text, language: Language): string => text[language];
