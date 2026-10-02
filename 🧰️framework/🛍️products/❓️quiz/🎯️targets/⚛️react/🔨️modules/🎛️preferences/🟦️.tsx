/** 🎛️ Persisted local-only display preferences: language, colour theme, text size, whether others' cursors show, when
 * the others' answers show (never, once the learner submitted, or always), and whether the task icons move.
 *
 * The language is stored only once the learner chooses it; until then the browser's list preselects one of the offered
 * languages, and when it names none the client asks with {@link LanguageChoice}, which speaks every offered language at
 * once. The theme is the design system's surface appearance (system, light or dark) and the text size scales the root
 * font size, so every rem-based token of the design system grows with it. The pets of `../🐾️pets/🟦️.tsx` are off,
 * motionless, slightly active (unless the learner chose otherwise) or lively; the liveliness they had is remembered
 * while they are off, so the switch on every screen brings them back as they were. That the learner chose is a fact of
 * its own (`petsChosen`), because the preferences are stored as a whole on every change: only the pets' choice and the
 * pets' switch set it, a device that asks for reduced motion decides the default until then, and the choice holds on
 * every device afterwards.
 */

import { Fragment, useId, type ReactElement } from "react";
import { QUIZ_LOCALES, isQuizLocale, quizText, type QuizLabelKey, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { isRecord, type LocalStore } from "../💾️persistence/🟦️.ts";
import { BodyButton, CardAction, CardIcon, PageFrame, QuizCard, Segments, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { OTHERS_CHOICES, type OthersChoice } from "../🗳️crowd/🟦️.tsx";
import { PET_CHOICES, effectivePetMode, usePetCast, usePetsForced, usePetsReduced, type PetChoice, type PetLiveliness } from "../🐾️pets/🟦️.tsx";
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

/** 🐾️ The label of every {@link PetChoice}. */
export const PET_CHOICE_LABELS: { readonly [C in PetChoice]: QuizLabelKey } = { off: "quiz.preferences.petsOff", still: "quiz.preferences.petsStill", calm: "quiz.preferences.petsCalm", lively: "quiz.preferences.petsLively" };

/** 👥️ The label of every {@link OthersChoice}. */
export const OTHERS_CHOICE_LABELS: { readonly [C in OthersChoice]: QuizLabelKey } = { never: "quiz.preferences.othersNever", submitted: "quiz.preferences.othersSubmitted", always: "quiz.preferences.othersAlways" };

/** 🎛️ The display preferences of one device. */
export interface QuizPreferences {
  readonly locale?: QuizLocale;
  readonly theme: ThemeChoice;
  readonly textSize: TextSize;
  readonly showCursors: boolean;
  readonly others: OthersChoice;
  readonly animateIcons: boolean;
  readonly pets: PetChoice;
  readonly petsLiveliness: PetLiveliness;
  readonly petsChosen: boolean;
}

/** 💾️ The stored preferences, falling back to the system theme, normal text, others' cursors shown, others' answers
 * once the learner submitted, animated icons, and calm pets that count as
 * chosen only when the store says so. */
export function readPreferences(store: LocalStore): QuizPreferences {
  const stored = store.read("preferences");
  const record = isRecord(stored) ? stored : {};
  const pets = PET_CHOICES.some((choice) => choice === record.pets) ? (record.pets as PetChoice) : "calm";
  return {
    locale: isQuizLocale(record.locale) ? record.locale : undefined,
    theme: THEME_CHOICES.some((choice) => choice.value === record.theme) ? (record.theme as ThemeChoice) : "system",
    textSize: TEXT_SIZES.some((size) => size.value === record.textSize) ? (record.textSize as TextSize) : "normal",
    showCursors: record.showCursors !== false,
    others: OTHERS_CHOICES.some((choice) => choice === record.others) ? (record.others as OthersChoice) : "submitted",
    animateIcons: record.animateIcons !== false,
    pets,
    petsLiveliness: pets !== "off" ? pets : PET_CHOICES.some((choice) => choice !== "off" && choice === record.petsLiveliness) ? (record.petsLiveliness as PetLiveliness) : "calm",
    petsChosen: record.petsChosen === true,
  };
}

/** 🔁️ The preferences with a choice the learner made for the pets, in the preferences or with the switch on every
 * screen: it is marked as chosen, so it holds whatever the device asks for, and a choice that shows the pets is also
 * the liveliness they come back with after they were switched off. Nothing else marks a choice as chosen. */
export function withPets(preferences: QuizPreferences, pets: PetChoice): QuizPreferences {
  return { ...preferences, pets, petsLiveliness: pets === "off" ? preferences.petsLiveliness : pets, petsChosen: true };
}

/** 💾️ Stores the preferences. */
export function writePreferences(store: LocalStore, preferences: QuizPreferences): void {
  store.write("preferences", preferences);
}

/** 🔠️ The factor text is scaled by. */
export function textScale(size: TextSize): number {
  return TEXT_SIZES.find((entry) => entry.value === size)?.scale ?? 1;
}

/** 🗣️ The language switch: one pressed button per language, each named in its own language; a `compact` one — the
 * navbar's — shows the language codes (EN, DE) below tablet width. */
export function LanguageSwitch(props: { readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (locale: QuizLocale) => void; readonly compact?: boolean }): ReactElement {
  const { locale, text, onChange, compact } = props;
  return <Segments label={text("quiz.preferences.language")} options={QUIZ_LOCALES.map((option) => ({ value: option, label: text(LOCALE_NAMES[option]), lang: option, ...(compact ? { short: option.toUpperCase() } : {}) }))} value={locale} onChange={onChange} />;
}

/** 🗣️ One text in every offered language, each part marked with its language, joined by a middle dot. */
export function EveryLanguage(props: { readonly label: QuizLabelKey }): ReactElement {
  return (
    <>
      {QUIZ_LOCALES.map((locale, index) => (
        <Fragment key={locale}>
          {index === 0 ? null : " · "}
          <span lang={locale}>{quizText(locale)(props.label)}</span>
        </Fragment>
      ))}
    </>
  );
}

/** 🗣️ The same text as {@link EveryLanguage} as one string, for where markup cannot go (the document title). */
export function everyLanguage(label: QuizLabelKey): string {
  return QUIZ_LOCALES.map((locale) => quizText(locale)(label)).join(" · ");
}

/** 🌍️ The question no language is assumed for: which language? Asked in every offered language at once, in the order
 * they are offered, each invitation and each button in its own language and marked as such. */
export function LanguageChoice(props: { readonly onChoose: (locale: QuizLocale) => void }): ReactElement {
  const id = useId();
  return (
    <QuizCard id={id} card="language" headingLevel={1} focusableHeading icon={<CardIcon icon="globe" />} title={<EveryLanguage label="quiz.preferences.language" />}>
      <ul role="list" className="m-0 flex list-none flex-col gap-double p-0">
        {QUIZ_LOCALES.map((locale) => (
          <li key={locale} lang={locale} className="flex flex-wrap items-center gap-double">
            <BodyButton onClick={() => props.onChoose(locale)}>{quizText(locale)(LOCALE_NAMES[locale])}</BodyButton>
            <span className="text-sm">{quizText(locale)("quiz.preferences.choose")}</span>
          </li>
        ))}
      </ul>
    </QuizCard>
  );
}

/** 🎛️ Language, theme, text size, pets and when the others' answers show as labelled segmented choices — the pets'
 * choice is the one in effect, so a learner who has not chosen sees the device's default; under it the names of those
 * on stage right now and, when the device forces its own colours, the word that this is why none show, or, when it asks
 * for reduced motion, the word that this is why they stay still until the learner chooses —; others' cursors and the
 * animated icons as checkboxes. */
export function PreferencesPanel(props: { readonly preferences: QuizPreferences; readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void }): ReactElement {
  const { preferences, locale, text, onChange } = props;
  const cast = usePetCast();
  const motionless = usePetsReduced();
  const pets = effectivePetMode(preferences.pets, preferences.petsChosen, motionless);
  const forced = usePetsForced() && pets !== "off";
  const reduced = motionless && !preferences.petsChosen && pets !== "off" && !forced;
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
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("quiz.preferences.pets")}
        </span>
        <Segments label={text("quiz.preferences.pets")} options={PET_CHOICES.map((choice) => ({ value: choice, label: text(PET_CHOICE_LABELS[choice]) }))} value={pets} onChange={(choice) => onChange(withPets(preferences, choice))} />
        {forced ? <p className="m-0 basis-full text-xs text-muted-foreground">{text("quiz.preferences.petsForced")}</p> : null}
        {reduced ? <p className="m-0 basis-full text-xs text-muted-foreground">{text("quiz.preferences.petsReduced")}</p> : null}
        {cast.length === 0 ? null : <p className="m-0 basis-full text-xs text-muted-foreground">{text("quiz.preferences.petsCast", { names: cast.join(" · ") })}</p>}
      </div>
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("quiz.preferences.others")}
        </span>
        <Segments label={text("quiz.preferences.others")} options={OTHERS_CHOICES.map((choice) => ({ value: choice, label: text(OTHERS_CHOICE_LABELS[choice]) }))} value={preferences.others} onChange={(others) => onChange({ ...preferences, others })} />
      </div>
      <label className="quiz-target flex w-fit cursor-pointer items-center gap-single text-sm">
        <input type="checkbox" className="quiz-check" checked={preferences.showCursors} onChange={(event) => onChange({ ...preferences, showCursors: event.target.checked })} />
        {text("quiz.preferences.cursors")}
      </label>
      <label className="quiz-target flex w-fit cursor-pointer items-center gap-single text-sm">
        <input type="checkbox" className="quiz-check" checked={preferences.animateIcons} onChange={(event) => onChange({ ...preferences, animateIcons: event.target.checked })} />
        {text("quiz.preferences.motion")}
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
