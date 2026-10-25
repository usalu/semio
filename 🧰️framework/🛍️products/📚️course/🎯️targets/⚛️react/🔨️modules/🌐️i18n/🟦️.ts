/** 🌐️ Every string the course client itself shows, in English and German, registered on the repo's shared i18n port — each
 * language a chunk of its own that {@link loadCourseLocales} fetches, the chosen language alone or, while none is chosen, all.
 *
 * Learner content (titles, prompts, labels, explanations) is data and arrives as `{ en, de }` {@link Text}; only the
 * client's own chrome lives here. There is no default language: the locale is the learner's explicit, persisted choice,
 * else the first language of the browser's list the client speaks — a preselection the language switch shows and
 * changes — else none, and the client asks in every language at once ({@link COURSE_LOCALES} order: English, then German).
 * Counts are written as "Label: n", so no sentence needs a plural form.
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx — `UiI18nPort`, `UiLabelValue`
 * @see ../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts — `@semio-tech/ui-react/i18n`: `registerUiTranslationBundles`, `uiI18n`
 */

import { LANGUAGES, type Rejection, type ShortText, type TaskKind, type Text } from "@semio-tech/course";
import { registerUiTranslationBundle, resolveUiLabel, setUiLocale, uiI18n, type DeepUiTranslationKeys, type UiRegisteredTranslationKey } from "@semio-tech/ui-react/i18n";

//#region 🗣️Locale
/** 🗣️ A language the quiz client speaks; the same set every learner {@link Text} carries. */

export type CourseLocale = (typeof LANGUAGES)[number];

/** 🔃️ Every {@link CourseLocale} in the order they are offered: English, then German. */
export const COURSE_LOCALES: readonly CourseLocale[] = LANGUAGES;

/** ❓️ Whether `value` names a {@link CourseLocale}. */
export function isCourseLocale(value: unknown): value is CourseLocale {
  return typeof value === "string" && (COURSE_LOCALES as readonly string[]).includes(value);
}

/** 🧭️ The first of the browser's `languages` (BCP 47 tags, preference order) the client speaks; none when the list names
 * no offered language — then nothing is assumed and the learner is asked. */
export function preferredLocale(languages: readonly string[]): CourseLocale | undefined {
  for (const language of languages) {
    const primary = language.toLowerCase().split("-")[0];
    if (isCourseLocale(primary)) return primary;
  }
  return undefined;
}

/** 🌍️ The learner-visible text of `text` in `locale`. */
export function localized(text: Text, locale: CourseLocale): string {
  return text[locale];
}

/** 🏷️ The full and short learner-visible names of something that carries both. */
export function titled(source: { readonly title: Text; readonly short?: ShortText }, locale: CourseLocale): { readonly full: string; readonly short: string } {
  const full = localized(source.title, locale);
  return { full, short: source.short === undefined ? full : localized(source.short, locale) };
}

/** 🔁️ Makes `locale` the language of the shared i18n port and of the document; without a locale the document names no
 * language, and every part of it names its own. */
export function applyLocale(locale: CourseLocale | undefined): Promise<unknown> {
  if (locale !== undefined) return setUiLocale(locale);
  if (typeof document !== "undefined") document.documentElement.removeAttribute("lang");
  return Promise.resolve();
}
//#endregion 🗣️Locale

//#region 📚️Bundles
/** 📚️ The shape of the chrome of every language: the English bundle's, which the other languages must repeat key for key. */
type CourseBundle = typeof import("./🔠️english-bundle/🟦️.ts").COURSE_BUNDLE_EN;

/** 🔑️ Every key of the quiz chrome as a dot path, e.g. `course.run.submit`. */
export type CourseLabelKey = DeepUiTranslationKeys<CourseBundle>;

const courseLabel = (key: CourseLabelKey): UiRegisteredTranslationKey => key as UiRegisteredTranslationKey;

const bundleLoaders: { readonly [L in CourseLocale]: () => Promise<CourseBundle> } = {
  en: () => import("./🔠️english-bundle/🟦️.ts").then((module) => module.COURSE_BUNDLE_EN),
  de: () => import("./🔡️german-bundle/🟦️.ts").then((module) => module.COURSE_BUNDLE_DE),
};
const loadedLocales = new Set<CourseLocale>();
const loadingLocales = new Map<CourseLocale, Promise<void>>();

function fetchLocale(locale: CourseLocale): Promise<void> {
  if (loadedLocales.has(locale)) return Promise.resolve();
  let pending = loadingLocales.get(locale);
  if (pending === undefined) {
    pending = bundleLoaders[locale]()
      .then((bundle) => {
        registerUiTranslationBundle(locale, bundle);
        loadedLocales.add(locale);
      })
      .finally(() => loadingLocales.delete(locale));
    loadingLocales.set(locale, pending);
  }
  return pending;
}

/** 📥️ Whether the chrome texts of `locale` have arrived: until then nothing of the client resolves in it. */
export function courseLocaleLoaded(locale: CourseLocale): boolean {
  return loadedLocales.has(locale);
}

/** 📦️ Fetches the chrome texts of `locales` — each language is a chunk of its own, so a learner downloads only the
 * language the learner speaks — and settles once they are registered on the shared i18n port; every offered language
 * by default, which is what the language question needs while none is chosen. */
export function loadCourseLocales(locales: readonly CourseLocale[] = COURSE_LOCALES): Promise<void> {
  return Promise.all(locales.map(fetchLocale)).then(() => undefined);
}

/** 🚫️ The chrome key explaining each proctor {@link Rejection}. */
export const REJECTION_LABELS: { readonly [R in Rejection]: CourseLabelKey } = {
  "unknown-learner": "course.rejection.unknownLearner",
  "unknown-quiz": "course.rejection.unknownQuiz",
  "unknown-run": "course.rejection.unknownRun",
  "unknown-task": "course.rejection.unknownTask",
  "run-open": "course.rejection.runOpen",
  "run-closed": "course.rejection.runClosed",
  "run-incomplete": "course.rejection.runIncomplete",
  "answer-invalid": "course.rejection.answerInvalid",
  "quiz-revised": "course.rejection.quizRevised",
  "handle-invalid": "course.rejection.handleInvalid",
  "handle-claimed": "course.rejection.handleClaimed",
  "id-invalid": "course.rejection.idInvalid",
  "learner-exists": "course.rejection.learnerExists",
  "roster-full": "course.rejection.rosterFull",
  "runs-exhausted": "course.rejection.runsExhausted",
  "answers-exhausted": "course.rejection.answersExhausted",
  "run-untimed": "course.rejection.runUntimed",
  "task-unopened": "course.rejection.taskUnopened",
  "time-up": "course.rejection.timeUp",
  "already-opened": "course.rejection.alreadyOpened",
};

/** 🧩️ The chrome key naming each {@link TaskKind}. */
export const TASK_KIND_LABELS: { readonly [K in TaskKind]: CourseLabelKey } = { classification: "course.task.classification", sorting: "course.task.sorting", matching: "course.task.matching" };
//#endregion 📚️Bundles

//#region 🔤️Text
/** 🔤️ Resolves one chrome key in the bound locale, interpolating `{{name}}` placeholders from `values`. */
export type CourseText = (key: CourseLabelKey, values?: Readonly<Record<string, string | number>>) => string;

/** 📖️ A {@link CourseText} reading the shared i18n port in `locale`, independent of the port's current language. */
export function courseText(locale: CourseLocale): CourseText {
  return (key, values) => resolveUiLabel(uiI18n.t(courseLabel(key) as never, { ...values, lng: locale }), "normal") ?? key;
}
//#endregion 🔤️Text
