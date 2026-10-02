/** 🪪️ The identity step: anonymous, pseudonym or name — never a password — and what that means, said plainly before
 * the choice.
 *
 * A pseudonym or name is public (the leaderboard, the others online) and is the whole key to its learner: whoever
 * enters it continues as that learner, on any device. An anonymous learner's key is this browser alone. The quizzes
 * and the leaderboard are for fun, and the others see what one answers. A handle is checked with the core's own
 * {@link normalizeHandle} before it leaves the device, and a refusal is explained by asking that same function about
 * the handle's parts — its length, each of its characters — so the explanation can never disagree with the proctor.
 *
 * @see ../../../../🔨️modules/✅️validation/🟦️.ts — `normalizeHandle`, the handle policy of both cores
 */

import { useEffect, useId, useRef, useState, type FormEvent, type ReactElement } from "react";
import { HANDLE_INPUT_MAX, HANDLE_MAX, normalizeHandle, type Identity, type IdentityClaim, type Rejection } from "@semio-tech/quiz";
import { REJECTION_LABELS, type QuizLabelKey, type QuizText } from "../🌐️i18n/🟦️.ts";
import { BodyButton, CardAction, CardIcon, ProblemNote, QuizCard, type Problem } from "../🪟️chrome/🟦️.tsx";
import type { QuizNotice, QuizSession, SessionFailure } from "../🧭️session/🟦️.ts";

/** 🏷️ How a learner is named in public: their handle, or the localized "Anonymous" with their tag — never with any
 * part of their id, which acts as their credential. */
export function learnerName(identity: Identity | undefined, tag: string, text: QuizText): string {
  return identity === undefined || identity.kind === "anonymous" ? `${text("quiz.leaderboard.anonymous")} #${tag}` : identity.handle;
}

/** 🚫️ A failed interactive command as a {@link Problem}: the rejection in the learner's language, the refusal with the
 * proctor's own words apart, or — for a spent allowance — what happened, that it passes by itself and when to try
 * again: the wait in whole seconds, rounded up and at least one, or "in a moment" when the proctor named none. */
export function failureProblem(failure: SessionFailure, text: QuizText): Problem {
  switch (failure.kind) {
    case "rejected":
      return { message: text(REJECTION_LABELS[failure.rejection]) };
    case "refused":
      return { message: text("quiz.rejection.refused"), detail: failure.detail };
    case "waiting":
      return { message: failure.retryAfterMs === undefined ? text("quiz.identity.signUpsSpentSoon") : text("quiz.identity.signUpsSpent", { seconds: Math.max(1, Math.ceil(failure.retryAfterMs / 1000)) }) };
  }
}

/** ✋️ An interactive command that threw as a {@link Problem}: cancelled by the learner, or refused for `error`. */
export function thrownProblem(error: unknown, signal: AbortSignal, text: QuizText): Problem {
  return signal.aborted ? { message: text("quiz.run.cancelled") } : { message: text("quiz.rejection.refused"), detail: error instanceof Error ? error.message : String(error) };
}

/** 📣️ A notice of the session as a {@link Problem}. */
export function noticeProblem(notice: QuizNotice, text: QuizText): Problem {
  switch (notice.kind) {
    case "rejection":
      return { message: text(REJECTION_LABELS[notice.rejection]) };
    case "refused":
      return { message: text("quiz.rejection.refused"), detail: notice.detail };
    case "voided":
      return { message: text("quiz.run.voided") };
    case "recalled":
      return { message: text("quiz.identity.recalled", { handle: notice.handle }) };
  }
}

/** 🔬️ Why {@link normalizeHandle} refuses a handle: nothing to keep, too long (with its length in characters),
 * characters outside the policy (each once, in typed order), or no letter or digit. */
export type HandleFault = { readonly kind: "empty" } | { readonly kind: "long"; readonly length: number } | { readonly kind: "characters"; readonly characters: readonly string[] } | { readonly kind: "letter" };

const PROBE = "a";

/** 🔬️ The fault of a typed handle, `undefined` when the core accepts it. Every answer comes from {@link normalizeHandle}
 * itself: a character is white space when the core collapses it between two letters and refused when the core refuses
 * it beside a letter; the length is that of the handle as the core would keep it (as typed when even that is too long
 * to be read). */
export function handleFault(typed: string): HandleFault | undefined {
  const handle = typed.normalize("NFC");
  if (normalizeHandle(handle) !== undefined) return undefined;
  const characters = [...handle];
  const spaces = new Set(characters.filter((character) => normalizeHandle(`${PROBE}${character}${PROBE}`)?.display === `${PROBE} ${PROBE}`));
  const refused = [...new Set(characters.filter((character) => !spaces.has(character) && normalizeHandle(`${PROBE}${character}`) === undefined))];
  if (refused.length > 0) return { kind: "characters", characters: refused };
  const words = characters
    .map((character) => (spaces.has(character) ? " " : character))
    .join("")
    .split(" ")
    .filter((word) => word !== "");
  if (words.length === 0) return { kind: "empty" };
  const length = [...words.join(" ")].length;
  if (characters.length > HANDLE_INPUT_MAX) return { kind: "long", length: characters.length };
  return length > HANDLE_MAX ? { kind: "long", length } : { kind: "letter" };
}

const INVISIBLE = /^[\p{C}\p{Z}\p{M}]$/u;
const SHOWN_CHARACTERS = 8;

function shownCharacter(character: string): string {
  return INVISIBLE.test(character) ? `U+${character.codePointAt(0)!.toString(16).toUpperCase().padStart(4, "0")}` : `“${character}”`;
}

/** 🔬️ A {@link HandleFault} in words: what exactly is wrong and what is allowed instead. */
export function handleFaultMessage(fault: HandleFault, text: QuizText): string {
  switch (fault.kind) {
    case "empty":
      return text("quiz.identity.handleEmpty");
    case "long":
      return text("quiz.identity.handleLong", { length: fault.length });
    case "characters":
      return text("quiz.identity.handleCharacters", { characters: `${fault.characters.slice(0, SHOWN_CHARACTERS).map(shownCharacter).join(" ")}${fault.characters.length > SHOWN_CHARACTERS ? " …" : ""}` });
    case "letter":
      return text("quiz.identity.handleLetter");
  }
}

const KINDS: readonly { readonly kind: Identity["kind"]; readonly label: QuizLabelKey; readonly hint: QuizLabelKey }[] = [
  { kind: "anonymous", label: "quiz.identity.anonymous", hint: "quiz.identity.anonymousHint" },
  { kind: "pseudonym", label: "quiz.identity.pseudonym", hint: "quiz.identity.pseudonymHint" },
  { kind: "name", label: "quiz.identity.name", hint: "quiz.identity.nameHint" },
];

/** 🖊️ The rejections that are about the handle a learner typed; every other failure leaves the field valid. */
const HANDLE_REJECTIONS: ReadonlySet<Rejection> = new Set<Rejection>(["handle-invalid", "handle-claimed"]);

/** 🪪️ The identity form. A problem is announced as an alert; the handle field is marked invalid and described by it
 * only when the problem is about the handle — not when the proctor refuses the sign-up for a reason of its own. */
export function IdentityScreen(props: { readonly session: QuizSession; readonly text: QuizText }): ReactElement {
  const { session, text } = props;
  const scope = useId();
  const [kind, setKind] = useState<Identity["kind"]>("anonymous");
  const [handle, setHandle] = useState("");
  const [working, setWorking] = useState<AbortController | undefined>(undefined);
  const [problem, setProblem] = useState<Problem | undefined>(undefined);
  const [faulty, setFaulty] = useState(false);
  const input = useRef<HTMLInputElement>(null);
  useEffect(() => () => working?.abort(), [working]);
  const say = (said: Problem | undefined, typed = false): void => {
    setProblem(said);
    setFaulty(typed);
  };

  const submit = (event: FormEvent<HTMLFormElement>): void => {
    event.preventDefault();
    if (working !== undefined) return;
    let identity: IdentityClaim = { kind: "anonymous" };
    if (kind !== "anonymous") {
      const typed = handle.normalize("NFC");
      const fault = handleFault(typed);
      if (fault !== undefined) {
        say({ message: handleFaultMessage(fault, text) }, true);
        input.current?.focus();
        return;
      }
      identity = { kind, handle: typed };
    }
    const controller = new AbortController();
    say(undefined);
    setWorking(controller);
    session
      .identify(identity, controller.signal)
      .then((failure) => failure !== undefined && say(failureProblem(failure, text), failure.kind === "rejected" && HANDLE_REJECTIONS.has(failure.rejection)))
      .catch((thrown: unknown) => say(thrownProblem(thrown, controller.signal, text)))
      .finally(() => setWorking((current) => (current === controller ? undefined : current)));
  };

  const errorId = `${scope}-error`;
  const ruleId = `${scope}-rule`;
  const publicId = `${scope}-public`;
  return (
    <form onSubmit={submit} noValidate aria-busy={working !== undefined} className="min-w-0">
      <QuizCard
        id={`${scope}-title`}
        card="identity"
        headingLevel={1}
        focusableHeading
        icon={<CardIcon icon="user" />}
        title={text("quiz.identity.title")}
        footerRight={
          <CardAction primary type="submit" disabled={working !== undefined}>
            {text("quiz.identity.submit")}
          </CardAction>
        }
      >
        <p className="m-0 text-sm leading-normal">{text("quiz.identity.lead")}</p>
        <fieldset className="m-0 flex flex-col gap-single border border-normal p-double">
          <legend className="px-single text-sm font-semibold">{text("quiz.identity.kind")}</legend>
          {KINDS.map((option) => (
            <div key={option.kind} className="grid grid-cols-[auto_minmax(0,1fr)] items-center gap-x-single">
              <input
                id={`${scope}-${option.kind}`}
                type="radio"
                name={`${scope}-kind`}
                value={option.kind}
                checked={kind === option.kind}
                onChange={() => {
                  setKind(option.kind);
                  say(undefined);
                }}
                aria-describedby={`${scope}-${option.kind}-hint`}
                className="quiz-radio"
              />
              <label htmlFor={`${scope}-${option.kind}`} className="quiz-target flex cursor-pointer items-center text-sm font-semibold">
                {text(option.label)}
              </label>
              <p id={`${scope}-${option.kind}-hint`} className="col-start-2 m-0 text-xs leading-normal text-muted-foreground">
                {text(option.hint)}
              </p>
            </div>
          ))}
        </fieldset>
        {kind === "anonymous" ? null : (
          <div className="flex max-w-md flex-col gap-single">
            <label htmlFor={`${scope}-handle`} className="text-sm font-semibold">
              {text(kind === "name" ? "quiz.identity.handleName" : "quiz.identity.handlePseudonym")}
            </label>
            <input
              ref={input}
              id={`${scope}-handle`}
              type="text"
              value={handle}
              autoComplete={kind === "name" ? "name" : "nickname"}
              spellCheck={false}
              aria-invalid={faulty}
              aria-describedby={faulty ? `${errorId} ${ruleId} ${publicId}` : `${ruleId} ${publicId}`}
              onChange={(event) => setHandle(event.target.value)}
              className="quiz-input"
            />
            <p id={ruleId} className="m-0 text-xs leading-normal text-muted-foreground">
              {text("quiz.identity.handleRule")}
            </p>
          </div>
        )}
        <div className="quiz-note flex flex-col gap-single border-l-2 px-double py-single text-xs leading-normal">
          <p id={publicId} className="m-0">
            {text("quiz.identity.public")}
          </p>
          <p className="m-0">{text("quiz.identity.forFun")}</p>
        </div>
        {problem === undefined ? null : <ProblemNote id={errorId} problem={problem} text={text} />}
        {working === undefined ? null : (
          <p role="status" className="m-0 flex flex-wrap items-center gap-double text-sm">
            {text("quiz.identity.working")}
            <BodyButton onClick={() => working.abort()}>{text("quiz.run.cancel")}</BodyButton>
          </p>
        )}
      </QuizCard>
    </form>
  );
}
