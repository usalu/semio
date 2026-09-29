/** 🪪️ The identity step: anonymous, pseudonym or name — never a password.
 *
 * A pseudonym or name that is already known recalls its learner and therefore its progress, on any device; the form
 * says so, and says that anyone entering the same handle continues that progress. Handles are sent NFC-normalised and
 * checked against the proctor's own rule (1…64 characters after trimming) before they leave the device.
 */

import { useEffect, useId, useRef, useState, type FormEvent, type ReactElement } from "react";
import { normalizeHandle, type Identity } from "@semio-tech/quiz";
import { REJECTION_LABELS, type QuizLabelKey, type QuizText } from "../🌐️i18n/🟦️.ts";
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
    <form className="quiz-identity quiz-panel" onSubmit={submit} noValidate aria-busy={working !== undefined}>
      <h1 tabIndex={-1}>{text("quiz.identity.title")}</h1>
      <p>{text("quiz.identity.lead")}</p>
      <fieldset>
        <legend>{text("quiz.identity.kind")}</legend>
        {KINDS.map((option) => (
          <div key={option.kind} className="quiz-choice-row">
            <input id={`${scope}-${option.kind}`} type="radio" name={`${scope}-kind`} value={option.kind} checked={kind === option.kind} onChange={() => setKind(option.kind)} aria-describedby={`${scope}-${option.kind}-hint`} />
            <label htmlFor={`${scope}-${option.kind}`}>{text(option.label)}</label>
            <p id={`${scope}-${option.kind}-hint`} className="quiz-muted">
              {text(option.hint)}
            </p>
          </div>
        ))}
      </fieldset>
      {kind === "anonymous" ? null : (
        <div className="quiz-field">
          <label htmlFor={`${scope}-handle`}>{text(kind === "name" ? "quiz.identity.handleName" : "quiz.identity.handlePseudonym")}</label>
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
          />
        </div>
      )}
      <p id={explanationId} className="quiz-note">
        {text("quiz.identity.noPassword")}
      </p>
      {error === undefined ? null : (
        <p id={errorId} className="quiz-error" role="alert">
          {error}
        </p>
      )}
      <div className="quiz-actions">
        <button type="submit" className="quiz-button quiz-button-primary" disabled={working !== undefined}>
          {text("quiz.identity.submit")}
        </button>
        {working === undefined ? null : (
          <span className="quiz-working" role="status">
            {text("quiz.identity.working")}
            <button type="button" className="quiz-button" onClick={() => working.abort()}>
              {text("quiz.run.cancel")}
            </button>
          </span>
        )}
      </div>
    </form>
  );
}
