/** ⛰️ The challenge of a run in the client: its name, what it asks with the most points it gives, the chooser a quiz's
 * page offers before a run starts, and points at a challenge as text. The chooser is a group of real radios in the
 * identity step's pattern, each described by its line, so arrow keys move the choice and a screen reader says what each
 * one asks; it changes nothing but the choice, which the device remembers for the next run of every quiz.
 *
 * @see ../../../../🔨️modules/⛰️challenge/🟦️.ts — `CHALLENGE_RULES`, `points`
 * @see ../🪪️identity/🟦️.tsx — the fieldset pattern
 */

import { useId, type ReactElement } from "react";
import { CHALLENGES, challengeRules, type Challenge } from "@semio-tech/quiz";
import type { QuizLabelKey, QuizLocale, QuizText } from "../🌐️i18n/🟦️.ts";
import { formatPoints } from "../📏️quantity/🟦️.ts";

/** 🏷️ The name of every {@link Challenge}. */
export const CHALLENGE_LABELS: { readonly [C in Challenge]: QuizLabelKey } = { easy: "quiz.challenge.easy", medium: "quiz.challenge.medium", hard: "quiz.challenge.hard", expert: "quiz.challenge.expert" };

/** 📝️ The one line saying what every {@link Challenge} asks and its most points (placeholder `par`). */
export const CHALLENGE_HINTS: { readonly [C in Challenge]: QuizLabelKey } = { easy: "quiz.challenge.easyHint", medium: "quiz.challenge.mediumHint", hard: "quiz.challenge.hardHint", expert: "quiz.challenge.expertHint" };

/** 💰️ `points` earned at `challenge` as text: its name and the points of its most, "Hard · Points: 261 of 300". */
export function challengeScored(challenge: Challenge, points: number, text: QuizText, locale: QuizLocale): string {
  return text("quiz.challenge.scored", { challenge: text(CHALLENGE_LABELS[challenge]), points: formatPoints(points, locale), par: formatPoints(challengeRules(challenge).par, locale) });
}

/** 🧗️ The challenge chooser of a quiz's page: four radios, each with what it asks and its most points. */
export function ChallengeChooser(props: { readonly challenge: Challenge; readonly onChange: (challenge: Challenge) => void; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { challenge, onChange, text, locale } = props;
  const scope = useId();
  return (
    <fieldset data-options={CHALLENGES.length} className="quiz-kinds m-0 min-w-0 border border-normal p-double">
      <legend className="px-single text-sm font-semibold">{text("quiz.challenge.legend")}</legend>
      {CHALLENGES.map((option) => (
        <div key={option} className="grid grid-cols-[auto_minmax(0,1fr)] items-center gap-x-single">
          <input
            id={`${scope}-${option}`}
            type="radio"
            name={`${scope}-challenge`}
            value={option}
            checked={challenge === option}
            onChange={() => onChange(option)}
            aria-describedby={`${scope}-${option}-hint`}
            className="quiz-radio"
          />
          <label htmlFor={`${scope}-${option}`} className="quiz-target flex cursor-pointer items-center text-sm font-semibold">
            {text(CHALLENGE_LABELS[option])}
          </label>
          <p id={`${scope}-${option}-hint`} className="col-start-2 m-0 text-xs leading-normal text-muted-foreground">
            {text(CHALLENGE_HINTS[option], { par: formatPoints(challengeRules(option).par, locale) })}
          </p>
        </div>
      ))}
    </fieldset>
  );
}
