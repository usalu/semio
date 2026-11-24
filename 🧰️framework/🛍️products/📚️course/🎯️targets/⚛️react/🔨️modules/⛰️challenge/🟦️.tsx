/** ⛰️ The challenge of a run in the client: its name, what it asks with the most points it gives, the chooser a quiz's
 * page offers before a run starts, and points at a challenge as text. The chooser is a group of real radios in the
 * identity step's pattern, each described by its line, so arrow keys move the choice and a screen reader says what each
 * one asks; it changes nothing but the choice, which the device remembers for the next run of every quiz.
 *
 * @see ../../../../🔨️modules/⛰️challenge/🟦️.ts — `CHALLENGE_RULES`, `points`
 * @see ../🪪️identity/🟦️.tsx — the fieldset pattern
 */

import { useId, type ReactElement } from "react";
import { CHALLENGES, challengeRules, type Challenge } from "@semio-tech/course";
import type { CourseLocale, CourseText } from "../🌐️i18n/🟦️.ts";
import { formatPoints } from "../📏️quantity/🟦️.ts";
import { ROW, RadioChoice } from "../🪟️chrome/🟦️.tsx";

import { CHALLENGE_HINTS, CHALLENGE_ICONS, CHALLENGE_LABELS } from "./🏷️labels/🟦️.ts";
export { CHALLENGE_HINTS, CHALLENGE_ICONS, CHALLENGE_LABELS } from "./🏷️labels/🟦️.ts";

/** 💰️ `points` earned at `challenge` as text: its name and the points of its most, "Hard · Points: 261 of 300". */
export function challengeScored(challenge: Challenge, points: number, text: CourseText, locale: CourseLocale): string {
  return text("course.challenge.scored", { challenge: text(CHALLENGE_LABELS[challenge]), points: formatPoints(points, locale), par: formatPoints(challengeRules(challenge).par, locale) });
}

/** 🧗️ The challenge chooser of a quiz's page: four radios, each with what it asks and its most points. It is the content of a flush card that the
 * card's title chip names (`labelledBy`): every choice is a cell, the choices are separated by the card's single lines, and the chosen one is a filled toggle cell with its icon. */
export function ChallengeChooser(props: { readonly challenge: Challenge; readonly labelledBy: string; readonly onChange: (challenge: Challenge) => void; readonly text: CourseText; readonly locale: CourseLocale }): ReactElement {
  const { challenge, onChange, text, locale } = props;
  const scope = useId();
  return (
    <div role="radiogroup" aria-labelledby={props.labelledBy} data-options={CHALLENGES.length} {...ROW} className="course-kinds min-w-0">
      {CHALLENGES.map((option) => (
        <RadioChoice key={option} id={`${scope}-${option}`} icon={CHALLENGE_ICONS[option]} name={`${scope}-challenge`} value={option} checked={challenge === option} onChange={() => onChange(option)} label={text(CHALLENGE_LABELS[option])} hint={text(CHALLENGE_HINTS[option], { par: formatPoints(challengeRules(option).par, locale) })} />
      ))}
    </div>
  );
}
