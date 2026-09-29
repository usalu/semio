/** 🎛️ Persisted local-only display preferences: language, colour theme, text size, and whether others' cursors and what
 * others think show (a learner may want to think alone).
 *
 * The language is stored only once the learner chooses it; until then it follows the browser. The theme is the design
 * system's surface appearance (system, light or dark) and the text size scales the root font size, so every
 * rem-based token of the design system grows with it.
 */

import { useId, type ReactElement } from "react";
import { QUIZ_LOCALES, isQuizLocale, type QuizLabelKey, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { isRecord, type LocalStore } from "../💾️persistence/🟦️.ts";
import { CardAction, CardIcon, PageFrame, QuizCard, Segments, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { HOME_PAGES } from "../🧭️session/🟦️.ts";

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
  readonly showCursors: boolean;
  readonly showAnswers: boolean;
}

/** 💾️ The stored preferences, falling back to the system theme, normal text, and others' cursors and answers shown. */
export function readPreferences(store: LocalStore): QuizPreferences {
  const stored = store.read("preferences");
  const record = isRecord(stored) ? stored : {};
  return {
    locale: isQuizLocale(record.locale) ? record.locale : undefined,
    theme: THEME_CHOICES.some((choice) => choice.value === record.theme) ? (record.theme as ThemeChoice) : "system",
    textSize: TEXT_SIZES.some((size) => size.value === record.textSize) ? (record.textSize as TextSize) : "normal",
    showCursors: record.showCursors !== false,
    showAnswers: record.showAnswers !== false,
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

/** 🗣️ The language switch: one pressed button per language, each named in its own language. */
export function LanguageSwitch(props: { readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (locale: QuizLocale) => void }): ReactElement {
  const { locale, text, onChange } = props;
  return <Segments label={text("quiz.preferences.language")} options={QUIZ_LOCALES.map((option) => ({ value: option, label: text(LOCALE_NAMES[option]), lang: option }))} value={locale} onChange={onChange} />;
}

/** 🎛️ Language, theme and text size as three labelled segmented choices; others' cursors and answers as checkboxes. */
export function PreferencesPanel(props: { readonly preferences: QuizPreferences; readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void }): ReactElement {
  const { preferences, locale, text, onChange } = props;
  const row = "flex flex-wrap items-center gap-x-double gap-y-single";
  const name = "min-w-[6em] text-xs text-muted-foreground";
  return (
    <div className="flex flex-col gap-double">
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("quiz.preferences.language")}
        </span>
        <LanguageSwitch locale={locale} text={text} onChange={(chosen) => onChange({ ...preferences, locale: chosen })} />
      </div>
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("quiz.preferences.theme")}
        </span>
        <Segments label={text("quiz.preferences.theme")} options={THEME_CHOICES.map((choice) => ({ value: choice.value, label: text(choice.label) }))} value={preferences.theme} onChange={(theme) => onChange({ ...preferences, theme })} />
      </div>
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("quiz.preferences.textSize")}
        </span>
        <Segments label={text("quiz.preferences.textSize")} options={TEXT_SIZES.map((size) => ({ value: size.value, label: text(size.label) }))} value={preferences.textSize} onChange={(textSize) => onChange({ ...preferences, textSize })} />
      </div>
      <label className="quiz-target flex w-fit cursor-pointer items-center gap-single text-sm">
        <input type="checkbox" className="quiz-check" checked={preferences.showCursors} onChange={(event) => onChange({ ...preferences, showCursors: event.target.checked })} />
        {text("quiz.preferences.cursors")}
      </label>
      <label className="quiz-target flex w-fit cursor-pointer items-center gap-single text-sm">
        <input type="checkbox" className="quiz-check" checked={preferences.showAnswers} onChange={(event) => onChange({ ...preferences, showAnswers: event.target.checked })} />
        {text("quiz.preferences.answers")}
      </label>
    </div>
  );
}

/** 🎛️ Language, theme, text size and cursors as a card: beside the first visit and on the preferences page. */
export function PreferencesPanelCard(props: { readonly preferences: QuizPreferences; readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void }): ReactElement {
  const id = useId();
  return (
    <QuizCard id={id} card="preferences" icon={<CardIcon icon="settings" />} title={props.text("quiz.preferences.title")}>
      <PreferencesPanel {...props} />
    </QuizCard>
  );
}

/** 🃏️ The preferences card of the overview: what can be set; its heading and "Open" open the page (`#prefs`). */
export function PreferencesCard(props: { readonly text: QuizText; readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { text, revealed, onOpen } = props;
  const id = useId();
  return (
    <QuizCard
      id={id}
      card="preferences"
      anchor={PRESENCE_ANCHORS.home("preferences")}
      href={`#${HOME_PAGES.preferences}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<CardIcon icon="settings" />}
      title={text("quiz.preferences.title")}
      footerRight={
        <CardAction primary onClick={onOpen}>
          {text("quiz.home.openPage")}
        </CardAction>
      }
    >
      <p className="m-0 text-xs leading-normal text-muted-foreground">{text("quiz.preferences.summary")}</p>
    </QuizCard>
  );
}

/** 🎛️ The preferences page behind its card: personal, so presence shares no cursor on it. */
export function PreferencesPage(props: { readonly preferences: QuizPreferences; readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void; readonly view: PaneView }): ReactElement {
  const { view: _view, ...panel } = props;
  return (
    <PageFrame page={HOME_PAGES.preferences}>
      <PreferencesPanelCard {...panel} />
    </PageFrame>
  );
}
