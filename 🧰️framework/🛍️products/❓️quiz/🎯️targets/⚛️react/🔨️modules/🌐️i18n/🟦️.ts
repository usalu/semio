/** 🌐️ Every string the quiz client itself shows, in English and German, registered on the repo's shared i18n port.
 *
 * Learner content (titles, prompts, labels, explanations) is data and arrives as `{ en, de }` {@link Text}; only the
 * client's own chrome lives here. There is no default language: the locale is the learner's explicit, persisted choice,
 * else the first language of the browser's list the client speaks — a preselection the language switch shows and
 * changes — else none, and the client asks in every language at once ({@link QUIZ_LOCALES} order: English, then German).
 * Counts are written as "Label: n", so no sentence needs a plural form.
 *
 * @see ../../../../../../🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx — `UiI18nPort`, `UiLabelValue`
 * @see ../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts — `@semio-tech/ui-react/i18n`: `registerUiTranslationBundles`, `uiI18n`
 */

import { LANGUAGES, type Rejection, type ShortText, type TaskKind, type Text } from "@semio-tech/quiz";
import { registerUiTranslationBundles, resolveUiLabel, setUiLocale, uiI18n, type DeepUiTranslationKeys } from "@semio-tech/ui-react/i18n";
import { QUIZ_BUNDLE_EN } from "./🔠️english-bundle/🟦️.ts";
import { QUIZ_BUNDLE_DE } from "./🔡️german-bundle/🟦️.ts";

//#region 🗣️Locale
/** 🗣️ A language the quiz client speaks; the same set every learner {@link Text} carries. */

export type QuizLocale = (typeof LANGUAGES)[number];

/** 🗣️ Every {@link QuizLocale} in the order they are offered: English, then German. */
export const QUIZ_LOCALES: readonly QuizLocale[] = LANGUAGES;

/** 🗣️ Whether `value` names a {@link QuizLocale}. */
export function isQuizLocale(value: unknown): value is QuizLocale {
  return typeof value === "string" && (QUIZ_LOCALES as readonly string[]).includes(value);
}

/** 🧭️ The first of the browser's `languages` (BCP 47 tags, preference order) the client speaks; none when the list names
 * no offered language — then nothing is assumed and the learner is asked. */
export function preferredLocale(languages: readonly string[]): QuizLocale | undefined {
  for (const language of languages) {
    const primary = language.toLowerCase().split("-")[0];
    if (isQuizLocale(primary)) return primary;
  }
  return undefined;
}

/** 🌍️ The learner-visible text of `text` in `locale`. */
export function localized(text: Text, locale: QuizLocale): string {
  return text[locale];
}

/** 🏷️ The full and short learner-visible names of something that carries both. */
export function titled(source: { readonly title: Text; readonly short?: ShortText }, locale: QuizLocale): { readonly full: string; readonly short: string } {
  const full = localized(source.title, locale);
  return { full, short: source.short === undefined ? full : localized(source.short, locale) };
}

/** 🔁️ Makes `locale` the language of the shared i18n port and of the document; without a locale the document names no
 * language, and every part of it names its own. */
export function applyLocale(locale: QuizLocale | undefined): Promise<unknown> {
  if (locale !== undefined) return setUiLocale(locale);
  if (typeof document !== "undefined") document.documentElement.removeAttribute("lang");
  return Promise.resolve();
}
//#endregion 🗣️Locale

//#region 📚️Bundles
/** 🔑️ Every key of the quiz chrome as a dot path, e.g. `quiz.run.submit`. */
export type QuizLabelKey = DeepUiTranslationKeys<typeof QUIZ_BUNDLE_EN>;

const quizLabel = registerUiTranslationBundles({ en: { translation: QUIZ_BUNDLE_EN }, de: { translation: QUIZ_BUNDLE_DE } });

/** 🚫️ The chrome key explaining each proctor {@link Rejection}. */
export const REJECTION_LABELS: { readonly [R in Rejection]: QuizLabelKey } = {
  "unknown-learner": "quiz.rejection.unknownLearner",
  "unknown-quiz": "quiz.rejection.unknownQuiz",
  "unknown-run": "quiz.rejection.unknownRun",
  "unknown-task": "quiz.rejection.unknownTask",
  "run-open": "quiz.rejection.runOpen",
  "run-closed": "quiz.rejection.runClosed",
  "run-incomplete": "quiz.rejection.runIncomplete",
  "answer-invalid": "quiz.rejection.answerInvalid",
  "quiz-revised": "quiz.rejection.quizRevised",
  "handle-invalid": "quiz.rejection.handleInvalid",
  "handle-claimed": "quiz.rejection.handleClaimed",
  "id-invalid": "quiz.rejection.idInvalid",
  "learner-exists": "quiz.rejection.learnerExists",
  "roster-full": "quiz.rejection.rosterFull",
  "runs-exhausted": "quiz.rejection.runsExhausted",
  "answers-exhausted": "quiz.rejection.answersExhausted",
  "run-untimed": "quiz.rejection.runUntimed",
  "task-unopened": "quiz.rejection.taskUnopened",
  "time-up": "quiz.rejection.timeUp",
  "already-opened": "quiz.rejection.alreadyOpened",
};

/** 🧩️ The chrome key naming each {@link TaskKind}. */
export const TASK_KIND_LABELS: { readonly [K in TaskKind]: QuizLabelKey } = { classification: "quiz.task.classification", sorting: "quiz.task.sorting", matching: "quiz.task.matching" };
//#endregion 📚️Bundles

//#region 🔤️Text
/** 🔤️ Resolves one chrome key in the bound locale, interpolating `{{name}}` placeholders from `values`. */
export type QuizText = (key: QuizLabelKey, values?: Readonly<Record<string, string | number>>) => string;

/** 🔤️ A {@link QuizText} reading the shared i18n port in `locale`, independent of the port's current language. */
export function quizText(locale: QuizLocale): QuizText {
  return (key, values) => resolveUiLabel(uiI18n.t(quizLabel(key) as never, { ...values, lng: locale }), "normal") ?? key;
}
//#endregion 🔤️Text
