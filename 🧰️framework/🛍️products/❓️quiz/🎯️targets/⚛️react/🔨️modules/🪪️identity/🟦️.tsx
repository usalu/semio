/** 🪪️ The identity step: anonymous, pseudonym or name — never a password.
 *
 * A pseudonym or name that is already known recalls its learner and therefore its progress, on any device; the form
 * says so, and says that anyone entering the same handle continues that progress. Handles are sent NFC-normalised and
 * checked against the proctor's own rule (1…64 characters after trimming) before they leave the device.
 */

import { useEffect, useId, useRef, useState, type FormEvent, type ReactElement } from "react";
import { normalizeHandle, type Identity } from "@semio-tech/quiz";
import { REJECTION_LABELS, type QuizLabelKey, type QuizText } from "../🌐️i18n/🟦️.ts";
import { BodyButton, CardAction, CardIcon, QuizCard } from "../🪟️chrome/🟦️.tsx";
import type { QuizSession, SessionFailure } from "../🧭️session/🟦️.ts";

/** 🏷️ How a learner is named in public: their handle, or the localized "Anonymous" with their tag — never with any
 * part of their id, which acts as their credential. */
export function learnerName(identity: Identity | undefined, tag: string, text: QuizText): string {
  return identity === undefined || identity.kind === "anonymous" ? `${text("quiz.leaderboard.anonymous")} #${tag}` : identity.handle;
}

/** 🚫️ The message of a failed interactive command. */
export function failureMessage(failure: SessionFailure, text: QuizText): string {
  return failure.kind === "rejected" ? text(REJECTION_LABELS[failure.rejection]) : text("quiz.rejection.refused", { detail: failure.detail });
}

/** ✋️ The message of an interactive command that threw: cancelled by the learner, or refused for `error`. */
export function thrownMessage(error: unknown, signal: AbortSignal, text: QuizText): string {
  return signal.aborted ? text("quiz.run.cancelled") : text("quiz.rejection.refused", { detail: error instanceof Error ? error.message : String(error) });
}

const KINDS: readonly { readonly kind: Identity["kind"]; readonly label: QuizLabelKey; readonly hint: QuizLabelKey }[] = [
  { kind: "anonymous", label: "quiz.identity.anonymous", hint: "quiz.identity.anonymousHint" },
  { kind: "pseudonym", label: "quiz.identity.pseudonym", hint: "quiz.identity.pseudonymHint" },
  { kind: "name", label: "quiz.identity.name", hint: "quiz.identity.nameHint" },
];

/** 🪪️ The identity form. */
export function IdentityScreen(props: { readonly session: QuizSession; readonly text: QuizText }): ReactElement {
  const { session, text } = props;
  const scope = useId();
  const [kind, setKind] = useState<Identity["kind"]>("anonymous");
  const [handle, setHandle] = useState("");
  const [working, setWorking] = useState<AbortController | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);
  const input = useRef<HTMLInputElement>(null);
  useEffect(() => () => working?.abort(), [working]);

  const submit = (event: FormEvent<HTMLFormElement>): void => {
    event.preventDefault();
    if (working !== undefined) return;
    let identity: Identity = { kind: "anonymous" };
    if (kind !== "anonymous") {
      const normalized = normalizeHandle(handle.normalize("NFC"));
      if (normalized === undefined) {
        setError(text("quiz.identity.handleInvalid"));
        input.current?.focus();
        return;
      }
      identity = { kind, handle: normalized.display };
    }
    const controller = new AbortController();
    setError(undefined);
    setWorking(controller);
    session
      .identify(identity, controller.signal)
      .then((failure) => failure !== undefined && setError(failureMessage(failure, text)))
      .catch((thrown: unknown) => setError(thrownMessage(thrown, controller.signal, text)))
      .finally(() => setWorking((current) => (current === controller ? undefined : current)));
  };

  const errorId = `${scope}-error`;
  const explanationId = `${scope}-explanation`;
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
                onChange={() => setKind(option.kind)}
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
              maxLength={64}
              autoComplete={kind === "name" ? "name" : "nickname"}
              spellCheck={false}
              aria-invalid={error !== undefined}
              aria-describedby={error === undefined ? explanationId : `${errorId} ${explanationId}`}
              onChange={(event) => setHandle(event.target.value)}
              className="quiz-input"
            />
          </div>
        )}
        <p id={explanationId} className="quiz-note m-0 border-l-2 px-double py-single text-xs leading-normal">
          {text("quiz.identity.noPassword")}
        </p>
        {error === undefined ? null : (
          <p id={errorId} role="alert" className="quiz-alert m-0 border border-normal px-double py-single text-sm font-semibold">
            {error}
          </p>
        )}
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
