/** 🏠️ Home: the catalog's quizzes with the learner's best score and open run, and every badge, earned or not yet. */

import { useEffect, useId, useState, type ReactElement } from "react";
import { learnerTag } from "@semio-tech/quiz";
import { localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatDate, formatPoints, formatScore } from "../📏️quantity/🟦️.ts";
import { failureMessage, learnerName, thrownMessage } from "../🪪️identity/🟦️.tsx";
import { lastSubmittedRunOf, openRunOf, type QuizSession, type QuizState, type SessionFailure } from "../🧭️session/🟦️.ts";

/** 🏠️ The home screen. */
export function HomeScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { session, state, text, locale } = props;
  const scope = useId();
  const [pending, setPending] = useState<AbortController | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);
  useEffect(() => () => pending?.abort(), [pending]);
  const { catalog, learnerView, learner } = state;
  if (catalog === undefined) return null;

  const act = (run: (signal: AbortSignal) => Promise<SessionFailure | undefined>): void => {
    if (pending !== undefined) return;
    const controller = new AbortController();
    setError(undefined);
    setPending(controller);
    run(controller.signal)
      .then((failure) => failure !== undefined && setError(failureMessage(failure, text)))
      .catch((thrown: unknown) => setError(thrownMessage(thrown, controller.signal, text)))
      .finally(() => setPending((current) => (current === controller ? undefined : current)));
  };

  return (
    <section className="quiz-home">
      <div className="quiz-home-header">
        <h1 tabIndex={-1}>{text("quiz.home.title")}</h1>
        {learner === undefined ? null : (
          <p className="quiz-home-learner">
            <span>{text("quiz.home.welcome", { name: learnerName(learner.identity, learnerTag(learner.id), text) })}</span>
            {learnerView === undefined ? null : <span>{text("quiz.home.total", { points: formatPoints(learnerView.total, locale) })}</span>}
            <button type="button" className="quiz-button quiz-button-quiet" onClick={() => session.forgetLearner()}>
              {text("quiz.identity.switch")}
            </button>
          </p>
        )}
      </div>
      {error === undefined ? null : (
        <p className="quiz-error" role="alert">
          {error}
        </p>
      )}
      {pending === undefined ? null : (
        <div className="quiz-working" role="status">
          <progress aria-label={text("quiz.home.starting")} />
          <span>{text("quiz.home.starting")}</span>
          <button type="button" className="quiz-button" onClick={() => pending.abort()}>
            {text("quiz.run.cancel")}
          </button>
        </div>
      )}
      <ul className="quiz-quizzes">
        {catalog.quizzes.map((quiz) => {
          const open = openRunOf(state, quiz.id);
          const best = learnerView?.best[quiz.id];
          const last = lastSubmittedRunOf(state, quiz.id);
          const headingId = `${scope}-${quiz.id}`;
          return (
            <li key={quiz.id}>
              <article className="quiz-quiz" aria-labelledby={headingId}>
                <h2 id={headingId}>{localized(quiz.title, locale)}</h2>
                <p>{localized(quiz.description, locale)}</p>
                <ul className="quiz-facts">
                  <li>{text("quiz.home.tasks", { amount: quiz.tasks.length })}</li>
                  <li>{best === undefined ? text("quiz.home.notYet") : text("quiz.home.best", { score: formatScore(best, locale) })}</li>
                  {open === undefined ? null : <li>{text("quiz.home.open")}</li>}
                </ul>
                <div className="quiz-actions">
                  {open === undefined ? (
                    <button type="button" className="quiz-button quiz-button-primary" disabled={pending !== undefined} onClick={() => act((signal) => session.startRun(quiz.id, signal))}>
                      {text(best === undefined ? "quiz.home.start" : "quiz.home.again")}
                    </button>
                  ) : (
                    <button type="button" className="quiz-button quiz-button-primary" disabled={pending !== undefined} onClick={() => act((signal) => session.resumeRun(open, signal))}>
                      {text("quiz.home.resume")}
                    </button>
                  )}
                  {last === undefined ? null : (
                    <button type="button" className="quiz-button" onClick={() => session.open({ screen: "results", run: last })}>
                      {text("quiz.home.lastResult")}
                    </button>
                  )}
                </div>
              </article>
            </li>
          );
        })}
      </ul>
      <section aria-labelledby={`${scope}-badges`}>
        <h2 id={`${scope}-badges`}>{text("quiz.home.badges")}</h2>
        <ul className="quiz-badges">
          {catalog.badges.map((badge) => {
            const award = learnerView?.badges.find((held) => held.badge === badge.id);
            return (
              <li key={badge.id} className="quiz-badge" data-earned={award === undefined ? undefined : ""}>
                <span className="quiz-badge-emoji" aria-hidden="true">
                  {badge.emoji}
                </span>
                <div>
                  <h3>{localized(badge.label, locale)}</h3>
                  <p className="quiz-badge-state">{award === undefined ? text("quiz.home.locked") : text("quiz.home.earnedAt", { date: formatDate(award.at, locale) })}</p>
                  <p>{localized(badge.description, locale)}</p>
                </div>
              </li>
            );
          })}
        </ul>
      </section>
    </section>
  );
}
