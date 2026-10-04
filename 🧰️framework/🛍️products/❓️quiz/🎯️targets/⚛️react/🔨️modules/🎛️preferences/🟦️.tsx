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
 * every device afterwards. Whether calm and lively pets answer clicks and can be picked up (`petsPlay`) and whether
 * they may play with the page (`petsMischief`) are two more of the learner's yes-or-no, both yes until the learner says
 * no; under the pets' row the settings also play with the pets on stage without a pointer. The challenge the learner
 * chose last on a quiz's page is remembered per quiz as well — the next run of that quiz, from its page or its card,
 * starts at it, while a quiz never chosen on starts at medium, so an expert run on one quiz never makes another one
 * expert by a click — but it is chosen on that page, not in the settings.
 */

import { Fragment, useId, type ReactElement } from "react";
import { useMediaQuery } from "@semio-tech/ui-react/chrome";
import { isSlug, type Challenge, type Slug } from "@semio-tech/quiz";
import { QUIZ_LOCALES, isQuizLocale, quizText, type QuizLabelKey, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { isChallenge, isRecord, type LocalStore } from "../💾️persistence/🟦️.ts";
import { BodyButton, CardAction, CardIcon, PageFrame, QuizCard, Segments, cn, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";
import { OTHERS_CHOICES, type OthersChoice } from "../🗳️crowd/🟦️.tsx";
import { PET_CHOICES, PetsPlay, effectivePetMode, usePetCast, usePetsForced, usePetsReduced, type PetChoice, type PetLiveliness } from "../🐾️pets/🟦️.tsx";
import { HOME_PAGES } from "../🧭️session/🟦️.ts";

/** 🌓️ A colour theme choice. */
export type ThemeChoice = "system" | "light" | "dark";

/** 🔠️ A text size choice. */
export type TextSize = "normal" | "large" | "larger" | "largest";

/** 🎨️ Every {@link ThemeChoice} with its label. */
export const THEME_CHOICES: readonly { readonly value: ThemeChoice; readonly label: QuizLabelKey }[] = [
  { value: "system", label: "quiz.preferences.themeSystem" },
  { value: "light", label: "quiz.preferences.themeLight" },
  { value: "dark", label: "quiz.preferences.themeDark" },
];

/** 🔡️ Every {@link TextSize} with its label and the factor it scales text by. */
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

/** 📱️ The display preferences of one device. */
export interface QuizPreferences {
  readonly locale?: QuizLocale;
  readonly theme: ThemeChoice;
  readonly textSize: TextSize;
  readonly showCursors: boolean;
  readonly others: OthersChoice;
  readonly animateIcons: boolean;
  readonly iconsChosen: boolean;
  readonly pets: PetChoice;
  readonly petsLiveliness: PetLiveliness;
  readonly petsChosen: boolean;
  readonly petsPlay: boolean;
  readonly petsMischief: boolean;
  readonly challenges: { readonly [quiz: Slug]: Challenge };
}

/** 💾️ The stored preferences, falling back to the system theme, normal text, others' cursors shown, others' answers
 * once the learner submitted, animated icons and calm pets — both of which count as chosen only when the store says
 * so —, pets that answer clicks and can be picked up and that may play with the page (each on unless the store says
 * `false`; neither is a choice of liveliness, so neither marks one), and the challenge last chosen per quiz — only the
 * entries whose quiz is a slug and whose challenge is one of the four. */
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
    iconsChosen: record.iconsChosen === true,
    pets,
    petsLiveliness: pets !== "off" ? pets : PET_CHOICES.some((choice) => choice !== "off" && choice === record.petsLiveliness) ? (record.petsLiveliness as PetLiveliness) : "calm",
    petsChosen: record.petsChosen === true,
    petsPlay: record.petsPlay !== false,
    petsMischief: record.petsMischief !== false,
    challenges: isRecord(record.challenges) ? Object.fromEntries(Object.entries(record.challenges).filter((entry): entry is [Slug, Challenge] => isSlug(entry[0]) && isChallenge(entry[1]))) : {},
  };
}

/** ⛰️ The challenge a run of `quiz` starts at: the one last chosen for it, medium until one is. */
export function challengeOf(preferences: QuizPreferences, quiz: Slug): Challenge {
  return Object.hasOwn(preferences.challenges, quiz) ? preferences.challenges[quiz]! : "medium";
}

/** 🧗️ The preferences with `challenge` chosen for `quiz`, every other quiz keeping its own. */
export function withChallenge(preferences: QuizPreferences, quiz: Slug, challenge: Challenge): QuizPreferences {
  return { ...preferences, challenges: { ...preferences.challenges, [quiz]: challenge } };
}

/** 🎞️ Whether the icons play their microanimations: what the learner chose, and until they choose, what the device
 * asks for — still on a device that asks for reduced motion, moving on every other. */
export function effectiveIconMotion(animate: boolean, chosen: boolean, reducedMotion: boolean): boolean {
  return chosen ? animate : !reducedMotion;
}

/** ✨️ The preferences with a choice the learner made for the icons: it is marked as chosen, so it holds whatever the
 * device asks for. */
export function withIcons(preferences: QuizPreferences, animate: boolean): QuizPreferences {
  return { ...preferences, animateIcons: animate, iconsChosen: true };
}

/** 🐕️ The preferences with a choice the learner made for the pets, in the preferences or with the switch on every
 * screen: it is marked as chosen, so it holds whatever the device asks for, and a choice that shows the pets is also
 * the liveliness they come back with after they were switched off. Nothing else marks a choice as chosen. */
export function withPets(preferences: QuizPreferences, pets: PetChoice): QuizPreferences {
  return { ...preferences, pets, petsLiveliness: pets === "off" ? preferences.petsLiveliness : pets, petsChosen: true };
}

/** 💽️ Stores the preferences. */
export function writePreferences(store: LocalStore, preferences: QuizPreferences): void {
  store.write("preferences", preferences);
}

/** 🔍️ The factor text is scaled by. */
export function textScale(size: TextSize): number {
  return TEXT_SIZES.find((entry) => entry.value === size)?.scale ?? 1;
}

/** 🔀️ The language switch: one pressed button per language, each named in its own language; a `compact` one — the
 * navbar's — shows the language codes (EN, DE) below tablet width. */
export function LanguageSwitch(props: { readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (locale: QuizLocale) => void; readonly compact?: boolean }): ReactElement {
  const { locale, text, onChange, compact } = props;
  return <Segments label={text("quiz.preferences.language")} options={QUIZ_LOCALES.map((option) => ({ value: option, label: text(LOCALE_NAMES[option]), lang: option, ...(compact ? { short: option.toUpperCase() } : {}) }))} value={locale} onChange={onChange} />;
}

/** 🌈️ One text in every offered language, each part marked with its language, joined by a middle dot. */
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

/** 🧵️ The same text as {@link EveryLanguage} as one string, for where markup cannot go (the document title). */
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

/** 🎚️ Language, theme, text size, pets and when the others' answers show as labelled segmented choices — the pets'
 * choice is the one in effect, so a learner who has not chosen sees the device's default; under it the names of those
 * on stage right now and, when the device forces its own colours, the word that this is why none show, or, when it asks
 * for reduced motion, the word that this is why they stay still until the learner chooses —; below it whether the pets
 * answer clicks and can be picked up and whether they may play with the page, as checkboxes that are off and say why
 * while the pets are off or still, and while they may play, the play with the pets on stage ({@link PetsPlay}, drawn
 * by the half of the pets' glue that came with them); others' cursors and the animated icons as checkboxes, the icons'
 * being the motion in effect with the same word under it on such a device. In a narrow card the name of each choice
 * stands above it; in a wider one the names are a column on the left and every control starts at the same place right
 * of it (`.quiz-settings`). */
export function PreferencesPanel(props: { readonly preferences: QuizPreferences; readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void }): ReactElement {
  const { preferences, locale, text, onChange } = props;
  const cast = usePetCast();
  const motionless = usePetsReduced();
  const pets = effectivePetMode(preferences.pets, preferences.petsChosen, motionless);
  const forced = usePetsForced() && pets !== "off";
  const reduced = motionless && !preferences.petsChosen && pets !== "off" && !forced;
  const resting = pets === "off" || pets === "still";
  const restingId = useId();
  const still = useMediaQuery("(prefers-reduced-motion: reduce)");
  const icons = effectiveIconMotion(preferences.animateIcons, preferences.iconsChosen, still);
  const row = "quiz-setting";
  const name = "quiz-setting-name text-xs text-muted-foreground";
  const allowance = cn("quiz-target flex w-fit items-center gap-single text-sm", resting ? "cursor-not-allowed text-muted-foreground" : "cursor-pointer");
  return (
    <div className="quiz-settings">
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
        {forced ? <p className="m-0 text-xs text-muted-foreground">{text("quiz.preferences.petsForced")}</p> : null}
        {reduced ? <p className="m-0 text-xs text-muted-foreground">{text("quiz.preferences.petsReduced")}</p> : null}
        {cast.length === 0 ? null : <p className="m-0 text-xs text-muted-foreground">{text("quiz.preferences.petsCast", { names: cast.join(" · ") })}</p>}
      </div>
      <label className={allowance} data-pets-allow="play">
        <input type="checkbox" className="quiz-check" checked={!resting && preferences.petsPlay} disabled={resting} aria-describedby={resting ? restingId : undefined} onChange={(event) => onChange({ ...preferences, petsPlay: event.target.checked })} />
        {text("quiz.preferences.petsPlay")}
      </label>
      <label className={allowance} data-pets-allow="mischief">
        <input type="checkbox" className="quiz-check" checked={!resting && preferences.petsMischief} disabled={resting} aria-describedby={resting ? restingId : undefined} onChange={(event) => onChange({ ...preferences, petsMischief: event.target.checked })} />
        {text("quiz.preferences.petsMischief")}
      </label>
      {resting ? (
        <p id={restingId} className="m-0 text-xs text-muted-foreground">
          {text("quiz.preferences.petsResting")}
        </p>
      ) : null}
      <PetsPlay text={text} row={row} name={name} />
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
        <input type="checkbox" className="quiz-check" checked={icons} onChange={(event) => onChange(withIcons(preferences, event.target.checked))} />
        {text("quiz.preferences.motion")}
      </label>
      {still && !preferences.iconsChosen ? <p className="m-0 text-xs text-muted-foreground">{text("quiz.preferences.motionReduced")}</p> : null}
    </div>
  );
}

/** 🧰️ Language, theme, text size and cursors as a card: beside the first visit and on the preferences page. */
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

/** 📃️ The preferences page behind its card: personal, so presence shares no cursor on it. */
export function PreferencesPage(props: { readonly preferences: QuizPreferences; readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void; readonly view: PaneView }): ReactElement {
  const { view: _view, ...panel } = props;
  return (
    <PageFrame page={HOME_PAGES.preferences}>
      <PreferencesPanelCard {...panel} />
    </PageFrame>
  );
}
