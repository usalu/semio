/** 🎛️ Persisted local-only display preferences: language, colour theme, text size, the device (desktop or tablet, on a
 * window that is not a phone), whether others' cursors show, when
 * the others' answers show (never, once the learner submitted, or always), and whether the task icons move.
 *
 * The language is stored only once the learner chooses it; until then the browser's list preselects one of the offered
 * languages, and when it names none the client asks with {@link LanguageChoice}, which speaks every offered language at
 * once. The theme is the design system's surface appearance (system, light or dark) and the text size scales the root
 * font size, so every rem-based token of the design system grows with it. The pets of `../🐾️pets/🟦️.tsx` are off,
 * motionless, slightly active or lively, and lively until the learner chooses otherwise; the liveliness they had is remembered
 * while they are off, so the switch on every screen brings them back as they were. That the learner chose is a fact of
 * its own (`petsChosen`), because the preferences are stored as a whole on every change: only the pets' choice and the
 * pets' switch set it. A device that asks for reduced motion does not turn the pets or the icons down: both start as
 * vivid as they can, and what the learner then chooses holds. Whether calm and lively pets answer clicks and can be picked up (`petsPlay`) and whether
 * they may play with the page (`petsMischief`) are two more of the learner's yes-or-no, both yes until the learner says
 * no; under the pets' row the settings also play with the pets on stage without a pointer. The challenge the learner
 * chose last on a quiz's page is remembered per quiz as well — the next run of that quiz, from its page or its card,
 * starts at it, while a quiz never chosen on starts at medium, so an expert run on one quiz never makes another one
 * expert by a click — but it is chosen on that page, not in the settings.
 */

import { Fragment, useId, type ReactElement } from "react";
import { UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, elementsSurfaceDeviceForLayout, elementsSurfaceDeviceForMatches, useMediaQuery, type IconName } from "@semio-tech/ui-react/chrome";
import { isSlug, roomScope, type Challenge, type Slug } from "@semio-tech/course";
import { COURSE_LOCALES, isCourseLocale, courseText, type CourseLabelKey, type CourseLocale, type CourseText } from "../🌐️i18n/🟦️.ts";
import { isChallenge, isRecord, type LocalStore } from "../💾️persistence/🟦️.ts";
import { BodyButton, CardAction, CardIcon, PageFrame, CourseCard, Segments, cn, type PaneView } from "../🪟️chrome/🟦️.tsx";
import { PRESENCE_ANCHORS } from "../👥️presence/👁️view/🟦️.ts";
import { PanePeers } from "../👥️presence/🟦️.tsx";
import { OTHERS_CHOICES, type OthersChoice } from "../🗳️crowd/🟦️.tsx";
import { PET_CHOICES, PetsPlay, effectivePetMode, usePetCast, usePetsForced, usePetsReduced, type PetChoice, type PetLiveliness } from "../🐾️pets/🟦️.tsx";
import { HOME_PAGES } from "../🧭️session/🧮️state/🟦️.ts";

/** 🌓️ A colour theme choice. */
export type ThemeChoice = "system" | "light" | "dark";

/** 🔠️ A text size choice. */
export type TextSize = "normal" | "large" | "larger" | "largest";

/** 📱️ Desktop or tablet, the two layouts a window that is not a phone can pin. */
export type CourseDevice = "desktop" | "tablet";

/** 🖥️ Every {@link CourseDevice} with its label and icon. */
export const COURSE_DEVICES: readonly { readonly value: CourseDevice; readonly label: CourseLabelKey; readonly icon: IconName }[] = [
  { value: "desktop", label: "course.preferences.deviceDesktop", icon: "monitor" },
  { value: "tablet", label: "course.preferences.deviceTablet", icon: "tablet" },
];

/** 🎨️ Every {@link ThemeChoice} with its label. */
export const THEME_CHOICES: readonly { readonly value: ThemeChoice; readonly label: CourseLabelKey; readonly icon: IconName }[] = [
  { value: "system", label: "course.preferences.themeSystem", icon: "monitor" },
  { value: "light", label: "course.preferences.themeLight", icon: "sun" },
  { value: "dark", label: "course.preferences.themeDark", icon: "moon" },
];

/** 🔡️ Every {@link TextSize} with its label and the factor it scales text by. */
export const TEXT_SIZES: readonly { readonly value: TextSize; readonly label: CourseLabelKey; readonly scale: number; readonly icon: IconName }[] = [
  { value: "normal", label: "course.preferences.textNormal", scale: 1, icon: "type" },
  { value: "large", label: "course.preferences.textLarge", scale: 1.125, icon: "type" },
  { value: "larger", label: "course.preferences.textLarger", scale: 1.25, icon: "typography" },
  { value: "largest", label: "course.preferences.textLargest", scale: 1.5, icon: "typography" },
];

/** 🗣️ The autonym of every {@link CourseLocale}, shown in its own language. */
export const LOCALE_NAMES: { readonly [L in CourseLocale]: CourseLabelKey } = { en: "course.preferences.english", de: "course.preferences.german" };

/** 🌐️ The icon of every {@link CourseLocale}: the same globe the language question already uses, beside the autonym. */
export const LOCALE_ICONS: { readonly [L in CourseLocale]: IconName } = { en: "globe", de: "globe" };

/** 🐾️ The label of every {@link PetChoice}. */
export const PET_CHOICE_LABELS: { readonly [C in PetChoice]: CourseLabelKey } = { off: "course.preferences.petsOff", still: "course.preferences.petsStill", calm: "course.preferences.petsCalm", lively: "course.preferences.petsLively" };

/** 🐈️ The icon of every {@link PetChoice}. */
export const PET_CHOICE_ICONS: { readonly [C in PetChoice]: IconName } = { off: "eye-off", still: "pause", calm: "smile", lively: "sparkles" };

/** 👥️ The label of every {@link OthersChoice}. */
export const OTHERS_CHOICE_LABELS: { readonly [C in OthersChoice]: CourseLabelKey } = { never: "course.preferences.othersNever", submitted: "course.preferences.othersSubmitted", always: "course.preferences.othersAlways" };

/** 👤️ The icon of every {@link OthersChoice}. */
export const OTHERS_CHOICE_ICONS: { readonly [C in OthersChoice]: IconName } = { never: "eye-off", submitted: "check", always: "users" };

/** ⚙️ The display preferences of one device. */
export interface CoursePreferences {
  readonly locale?: CourseLocale;
  readonly theme: ThemeChoice;
  readonly textSize: TextSize;
  readonly device?: CourseDevice;
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
 * once the learner submitted, animated icons and lively pets — both of which count as chosen only when the store says
 * so —, pets that answer clicks and can be picked up and that may play with the page (each on unless the store says
 * `false`; neither is a choice of liveliness, so neither marks one), and the challenge last chosen per quiz — only the
 * entries whose quiz is a slug and whose challenge is one of the four. */
export function readPreferences(store: LocalStore): CoursePreferences {
  const stored = store.read("preferences");
  const record = isRecord(stored) ? stored : {};
  const pets = PET_CHOICES.some((choice) => choice === record.pets) ? (record.pets as PetChoice) : "lively";
  return {
    locale: isCourseLocale(record.locale) ? record.locale : undefined,
    theme: THEME_CHOICES.some((choice) => choice.value === record.theme) ? (record.theme as ThemeChoice) : "system",
    textSize: TEXT_SIZES.some((size) => size.value === record.textSize) ? (record.textSize as TextSize) : "normal",
    ...(record.device === "desktop" || record.device === "tablet" ? { device: record.device } : {}),
    showCursors: record.showCursors !== false,
    others: OTHERS_CHOICES.some((choice) => choice === record.others) ? (record.others as OthersChoice) : "submitted",
    animateIcons: record.animateIcons !== false,
    iconsChosen: record.iconsChosen === true,
    pets,
    petsLiveliness: pets !== "off" ? pets : PET_CHOICES.some((choice) => choice !== "off" && choice === record.petsLiveliness) ? (record.petsLiveliness as PetLiveliness) : "lively",
    petsChosen: record.petsChosen === true,
    petsPlay: record.petsPlay !== false,
    petsMischief: record.petsMischief !== false,
    challenges: isRecord(record.challenges) ? Object.fromEntries(Object.entries(record.challenges).filter((entry): entry is [Slug, Challenge] => isSlug(entry[0]) && isChallenge(entry[1]))) : {},
  };
}

/** ⛰️ The challenge a run of `quiz` starts at: the one last chosen for it, medium until one is. */
export function challengeOf(preferences: CoursePreferences, quiz: Slug): Challenge {
  return Object.hasOwn(preferences.challenges, quiz) ? preferences.challenges[quiz]! : "medium";
}

/** 🧗️ The preferences with `challenge` chosen for `quiz`, every other quiz keeping its own. */
export function withChallenge(preferences: CoursePreferences, quiz: Slug, challenge: Challenge): CoursePreferences {
  return { ...preferences, challenges: { ...preferences.challenges, [quiz]: challenge } };
}

/** 🎞️ Whether the icons play their microanimations: what the learner chose, and until they choose, on — a device
 * that asks for reduced motion does not hold them still. */
export function effectiveIconMotion(animate: boolean, chosen: boolean): boolean {
  return chosen ? animate : true;
}

/** ✨️ The preferences with a choice the learner made for the icons: it is marked as chosen, so it holds whatever the
 * device asks for. */
export function withIcons(preferences: CoursePreferences, animate: boolean): CoursePreferences {
  return { ...preferences, animateIcons: animate, iconsChosen: true };
}

/** 🐕️ The preferences with a choice the learner made for the pets, in the preferences or with the switch on every
 * screen: it is marked as chosen, so it holds whatever the device asks for, and a choice that shows the pets is also
 * the liveliness they come back with after they were switched off. Nothing else marks a choice as chosen. */
export function withPets(preferences: CoursePreferences, pets: PetChoice): CoursePreferences {
  return { ...preferences, pets, petsLiveliness: pets === "off" ? preferences.petsLiveliness : pets, petsChosen: true };
}

/** 💽️ Stores the preferences. */
export function writePreferences(store: LocalStore, preferences: CoursePreferences): void {
  store.write("preferences", preferences);
}

/** 🔍️ The factor text is scaled by. */
export function textScale(size: TextSize): number {
  return TEXT_SIZES.find((entry) => entry.value === size)?.scale ?? 1;
}

/** 🔀️ The language switch: one pressed button per language, each a globe and its name in its own language; a `compact`
 * one — the navbar's — shows the language codes (EN, DE) below tablet width, still beside the globe. */
export function LanguageSwitch(props: { readonly locale: CourseLocale; readonly text: CourseText; readonly onChange: (locale: CourseLocale) => void; readonly compact?: boolean }): ReactElement {
  const { locale, text, onChange, compact } = props;
  return <Segments label={text("course.preferences.language")} options={COURSE_LOCALES.map((option) => ({ value: option, label: text(LOCALE_NAMES[option]), lang: option, icon: LOCALE_ICONS[option], ...(compact ? { short: option.toUpperCase() } : {}) }))} value={locale} onChange={onChange} />;
}

/** 🌈️ One text in every offered language, each part marked with its language, joined by a middle dot. */
export function EveryLanguage(props: { readonly label: CourseLabelKey }): ReactElement {
  return (
    <>
      {COURSE_LOCALES.map((locale, index) => (
        <Fragment key={locale}>
          {index === 0 ? null : " · "}
          <span lang={locale}>{courseText(locale)(props.label)}</span>
        </Fragment>
      ))}
    </>
  );
}

/** 🧵️ The same text as {@link EveryLanguage} as one string, for where markup cannot go (the document title). */
export function everyLanguage(label: CourseLabelKey): string {
  return COURSE_LOCALES.map((locale) => courseText(locale)(label)).join(" · ");
}

/** 🌍️ The question no language is assumed for: which language? Asked in every offered language at once, in the order
 * they are offered, each invitation and each button in its own language and marked as such. */
export function LanguageChoice(props: { readonly onChoose: (locale: CourseLocale) => void }): ReactElement {
  const id = useId();
  return (
    <CourseCard id={id} card="language" headingLevel={1} focusableHeading icon={<CardIcon icon="globe" />} title={<EveryLanguage label="course.preferences.language" />}>
      <ul role="list" className="m-0 flex list-none flex-col gap-double p-0">
        {COURSE_LOCALES.map((locale) => (
          <li key={locale} lang={locale} className="flex flex-wrap items-center gap-double">
            <BodyButton onClick={() => props.onChoose(locale)}>{courseText(locale)(LOCALE_NAMES[locale])}</BodyButton>
            <span className="text-sm">{courseText(locale)("course.preferences.choose")}</span>
          </li>
        ))}
      </ul>
    </CourseCard>
  );
}

/** 🎚️ Language, theme, text size, pets and when the others' answers show as segmented choices, each an icon and its label — the pets'
 * choice is the one stored, lively until the learner chooses otherwise; under it the names of those
 * on stage right now and, when the device forces its own colours, the word that this is why none show; below it whether the pets
 * answer clicks and can be picked up and whether they may play with the page, as checkboxes that are off and say why
 * while the pets are off or still, and while they may play, the play with the pets on stage ({@link PetsPlay}, drawn
 * by the half of the pets' glue that came with them); others' cursors and the animated icons as checkboxes. Icons move
 * until the learner turns them off. In a narrow card the name of each choice
 * stands above it; in a wider one the names are a column on the left and every control starts at the same place right
 * of it (`.course-settings`). */
export function PreferencesPanel(props: { readonly preferences: CoursePreferences; readonly locale: CourseLocale; readonly text: CourseText; readonly onChange: (preferences: CoursePreferences) => void }): ReactElement {
  const { preferences, locale, text, onChange } = props;
  const measured = elementsSurfaceDeviceForMatches({ mobile: useMediaQuery(UI_MOBILE_MEDIA_QUERY), tablet: useMediaQuery(UI_TABLET_MEDIA_QUERY) });
  const device = elementsSurfaceDeviceForLayout(measured, preferences.device);
  const cast = usePetCast();
  const pets = effectivePetMode(preferences.pets);
  const forced = usePetsForced() && pets !== "off";
  const resting = pets === "off" || pets === "still";
  const restingId = useId();
  const icons = effectiveIconMotion(preferences.animateIcons, preferences.iconsChosen);
  const row = "course-setting";
  const name = "course-setting-name text-xs text-muted-foreground";
  const allowance = cn("course-target flex w-fit items-center gap-single text-sm", resting ? "cursor-not-allowed text-muted-foreground" : "cursor-pointer");
  return (
    <div className="course-settings">
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("course.preferences.language")}
        </span>
        <LanguageSwitch locale={locale} text={text} onChange={(chosen) => onChange({ ...preferences, locale: chosen })} />
      </div>
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("course.preferences.theme")}
        </span>
        <Segments label={text("course.preferences.theme")} options={THEME_CHOICES.map((choice) => ({ value: choice.value, label: text(choice.label), icon: choice.icon }))} value={preferences.theme} onChange={(theme) => onChange({ ...preferences, theme })} />
      </div>
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("course.preferences.textSize")}
        </span>
        <Segments label={text("course.preferences.textSize")} options={TEXT_SIZES.map((size) => ({ value: size.value, label: text(size.label), icon: size.icon }))} value={preferences.textSize} onChange={(textSize) => onChange({ ...preferences, textSize })} />
      </div>
      {device === "mobile" ? null : (
        <div className={row}>
          <span className={name} aria-hidden="true">
            {text("course.preferences.device")}
          </span>
          <Segments label={text("course.preferences.device")} options={COURSE_DEVICES.map((choice) => ({ value: choice.value, label: text(choice.label), icon: choice.icon }))} value={device} onChange={(chosen) => onChange({ ...preferences, device: chosen })} />
        </div>
      )}
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("course.preferences.pets")}
        </span>
        <Segments label={text("course.preferences.pets")} options={PET_CHOICES.map((choice) => ({ value: choice, label: text(PET_CHOICE_LABELS[choice]), icon: PET_CHOICE_ICONS[choice] }))} value={pets} onChange={(choice) => onChange(withPets(preferences, choice))} />
        {forced ? <p className="m-0 text-xs text-muted-foreground">{text("course.preferences.petsForced")}</p> : null}
        {cast.length === 0 ? null : <p className="m-0 text-xs text-muted-foreground">{text("course.preferences.petsCast", { names: cast.join(" · ") })}</p>}
      </div>
      <label className={allowance} data-pets-allow="play">
        <input type="checkbox" className="course-check" checked={!resting && preferences.petsPlay} disabled={resting} aria-describedby={resting ? restingId : undefined} onChange={(event) => onChange({ ...preferences, petsPlay: event.target.checked })} />
        {text("course.preferences.petsPlay")}
      </label>
      <label className={allowance} data-pets-allow="mischief">
        <input type="checkbox" className="course-check" checked={!resting && preferences.petsMischief} disabled={resting} aria-describedby={resting ? restingId : undefined} onChange={(event) => onChange({ ...preferences, petsMischief: event.target.checked })} />
        {text("course.preferences.petsMischief")}
      </label>
      {resting ? (
        <p id={restingId} className="m-0 text-xs text-muted-foreground">
          {text("course.preferences.petsResting")}
        </p>
      ) : null}
      <PetsPlay text={text} row={row} name={name} />
      <div className={row}>
        <span className={name} aria-hidden="true">
          {text("course.preferences.others")}
        </span>
        <Segments label={text("course.preferences.others")} options={OTHERS_CHOICES.map((choice) => ({ value: choice, label: text(OTHERS_CHOICE_LABELS[choice]), icon: OTHERS_CHOICE_ICONS[choice] }))} value={preferences.others} onChange={(others) => onChange({ ...preferences, others })} />
      </div>
      <label className="course-target flex w-fit cursor-pointer items-center gap-single text-sm">
        <input type="checkbox" className="course-check" checked={preferences.showCursors} onChange={(event) => onChange({ ...preferences, showCursors: event.target.checked })} />
        {text("course.preferences.cursors")}
      </label>
      <label className="course-target flex w-fit cursor-pointer items-center gap-single text-sm">
        <input type="checkbox" className="course-check" checked={icons} onChange={(event) => onChange(withIcons(preferences, event.target.checked))} />
        {text("course.preferences.motion")}
      </label>
    </div>
  );
}

/** 🧰️ Language, theme, text size and cursors as a card: beside the first visit and on the preferences page. */
export function PreferencesPanelCard(props: { readonly preferences: CoursePreferences; readonly locale: CourseLocale; readonly text: CourseText; readonly onChange: (preferences: CoursePreferences) => void }): ReactElement {
  const id = useId();
  return (
    <CourseCard id={id} card="preferences" icon={<CardIcon icon="settings" />} title={props.text("course.preferences.title")}>
      <PreferencesPanel {...props} />
    </CourseCard>
  );
}

/** 🃏️ The preferences card of the overview: what can be set; its heading and "Open" open the page (`#prefs`). */
export function PreferencesCard(props: { readonly text: CourseText; readonly revealed: boolean; readonly onOpen: () => void }): ReactElement {
  const { text, revealed, onOpen } = props;
  const id = useId();
  return (
    <CourseCard
      id={id}
      card="preferences"
      anchor={PRESENCE_ANCHORS.home("preferences")}
      href={`#${HOME_PAGES.preferences}`}
      onOpen={onOpen}
      revealed={revealed}
      icon={<CardIcon icon="settings" />}
      title={text("course.preferences.title")}
      footerRight={
        <CardAction primary onClick={onOpen}>
          {text("course.home.openPage")}
        </CardAction>
      }
    >
      <p className="m-0 text-xs leading-normal text-muted-foreground">{text("course.preferences.summary")}</p>
    </CourseCard>
  );
}

/** 📃️ The preferences page behind its card with shared cursor activity. */
export function PreferencesPage(props: { readonly course?: string; readonly preferences: CoursePreferences; readonly locale: CourseLocale; readonly text: CourseText; readonly onChange: (preferences: CoursePreferences) => void; readonly view: PaneView }): ReactElement {
  const { view, course, ...panel } = props;
  return (
    <PageFrame page={HOME_PAGES.preferences} overlay={<PanePeers scope={course === undefined ? undefined : roomScope(course, { screen: "preferences" })} opened={view.opened} />}>
      <PreferencesPanelCard {...panel} />
    </PageFrame>
  );
}
