/** 🏁️ Results of a submitted run: the score, and per task the learner's answer beside the solution — true values
 * with their units, the correct categories, positions and explanations — plus the badges the run newly earned.
 * Verdicts are spelled out in words and symbols, never by colour alone.
 */

import { useId, type ReactElement } from "react";
import type { Id, MatchingTaskResult, SheetClassificationTask, SheetMatchingTask, SheetSortingTask, SheetTask, Slug, SortingTaskResult, TaskResult, Text, ClassificationTaskResult } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { formatQuantity, formatScore } from "../📏️quantity/🟦️.ts";
import { runAwards, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";

function verdict(credit: number, text: QuizText): string {
  if (credit >= 1) return `✓ ${text("quiz.results.correct")}`;
  return credit > 0 ? `◐ ${text("quiz.results.partial")}` : `✗ ${text("quiz.results.wrong")}`;
}

function labelOf(items: readonly { readonly id: Slug; readonly label: Text }[], id: Slug, locale: QuizLocale): string {
  return localized(items.find((item) => item.id === id)?.label ?? { en: id, de: id }, locale);
}

function Explanation(props: { readonly explanation: Text | undefined; readonly locale: QuizLocale }): ReactElement {
  return <>{props.explanation === undefined ? "" : localized(props.explanation, props.locale)}</>;
}

function ClassificationResult(props: { readonly task: SheetClassificationTask; readonly result: ClassificationTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, text, locale } = props;
  return (
    <div className="quiz-table-scroll">
      <table className="quiz-table">
        <thead>
          <tr>
            <th scope="col">{text("quiz.results.item")}</th>
            <th scope="col">{text("quiz.results.yourAnswer")}</th>
            <th scope="col">{text("quiz.results.solution")}</th>
            <th scope="col">{text("quiz.results.credit")}</th>
            <th scope="col">{text("quiz.results.explanation")}</th>
          </tr>
        </thead>
        <tbody>
          {result.items.map((item) => (
            <tr key={item.item}>
              <th scope="row">{labelOf(task.items, item.item, locale)}</th>
              <td>{labelOf(task.categories, item.assigned, locale)}</td>
              <td>{labelOf(task.categories, item.correct, locale)}</td>
              <td>
                {verdict(item.credit, text)} ({formatScore(item.credit, locale)})
              </td>
              <td>
                <Explanation explanation={item.explanation} locale={locale} />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function SortingResult(props: { readonly task: SheetSortingTask; readonly result: SortingTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, text, locale } = props;
  const value = (amount: number): string => formatQuantity(amount, task.quantity, locale);
  return (
    <>
      <div className="quiz-table-scroll">
        <table className="quiz-table">
          <thead>
            <tr>
              <th scope="col">{text("quiz.results.position")}</th>
              <th scope="col">{text("quiz.results.item")}</th>
              <th scope="col">{text("quiz.results.value")}</th>
              <th scope="col">{text("quiz.results.rank")}</th>
              <th scope="col">{text("quiz.results.explanation")}</th>
            </tr>
          </thead>
          <tbody>
            {result.items.map((item) => (
              <tr key={item.item}>
                <td>{item.position + 1}</td>
                <th scope="row">{labelOf(task.items, item.item, locale)}</th>
                <td>{value(item.value)}</td>
                <td>
                  {item.rank + 1} {item.rank === item.position ? `✓ ${text("quiz.results.correct")}` : `✗ ${text("quiz.results.wrong")}`}
                </td>
                <td>
                  <Explanation explanation={item.explanation} locale={locale} />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <h4>{text("quiz.results.trueOrder")}</h4>
      <ol className="quiz-true-order">
        {[...result.items]
          .sort((left, right) => left.rank - right.rank)
          .map((item) => (
            <li key={item.item}>
              {labelOf(task.items, item.item, locale)} — {value(item.value)}
            </li>
          ))}
      </ol>
    </>
  );
}

function MatchingResult(props: { readonly task: SheetMatchingTask; readonly result: MatchingTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, result, text, locale } = props;
  return (
    <>
      {result.dimensions.map((dimension) => {
        const sheet = task.dimensions.find((candidate) => candidate.id === dimension.dimension);
        const quantity = sheet === undefined ? dimension.dimension : localized(sheet.quantity.label, locale);
        const value = (amount: number): string => (sheet === undefined ? String(amount) : formatQuantity(amount, sheet.quantity, locale));
        return (
          <section key={dimension.dimension} className="quiz-result-dimension">
            <h4>{text("quiz.results.dimension", { quantity, score: formatScore(dimension.score, locale) })}</h4>
            <div className="quiz-table-scroll">
              <table className="quiz-table">
                <thead>
                  <tr>
                    <th scope="col">{text("quiz.results.item")}</th>
                    <th scope="col">{text("quiz.results.yourAnswer")}</th>
                    <th scope="col">{text("quiz.results.solution")}</th>
                    <th scope="col">{text("quiz.results.explanation")}</th>
                  </tr>
                </thead>
                <tbody>
                  {dimension.items.map((item) => (
                    <tr key={item.item}>
                      <th scope="row">{labelOf(task.items, item.item, locale)}</th>
                      <td>
                        {value(item.assigned)} {item.assigned === item.correct ? `✓ ${text("quiz.results.correct")}` : `✗ ${text("quiz.results.wrong")}`}
                      </td>
                      <td>{value(item.correct)}</td>
                      <td>
                        <Explanation explanation={item.explanation} locale={locale} />
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
        );
      })}
    </>
  );
}

/** 🧾️ The result of one task beside its presented form. */
export function TaskResultView(props: { readonly task: SheetTask; readonly result: TaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { task, result, text, locale } = props;
  if (task.kind === "classification" && result.kind === "classification") return <ClassificationResult task={task} result={result} text={text} locale={locale} />;
  if (task.kind === "sorting" && result.kind === "sorting") return <SortingResult task={task} result={result} text={text} locale={locale} />;
  if (task.kind === "matching" && result.kind === "matching") return <MatchingResult task={task} result={result} text={text} locale={locale} />;
  return null;
}

/** 🏁️ The results screen of `run`. */
export function ResultsScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { session, state, run, text, locale } = props;
  const scope = useId();
  const view = state.runs[run];
  const result = view?.status === "submitted" ? view.result : undefined;
  if (view === undefined) return null;
  if (result === undefined) {
    return (
      <section className="quiz-panel">
        <h1 tabIndex={-1}>{text("quiz.results.title", { quiz: localized(view.sheet.title, locale) })}</h1>
        <p role="status">{text("quiz.results.missing")}</p>
      </section>
    );
  }
  const badges = runAwards(state, run).flatMap((id) => state.catalog?.badges.filter((badge) => badge.id === id) ?? []);
  return (
    <section className="quiz-results">
      <h1 tabIndex={-1}>{text("quiz.results.title", { quiz: localized(view.sheet.title, locale) })}</h1>
      <p className="quiz-score">
        <strong>{text("quiz.results.score", { score: formatScore(result.score, locale) })}</strong>
      </p>
      {badges.length === 0 ? null : (
        <section className="quiz-new-badges" aria-labelledby={`${scope}-badges`}>
          <h2 id={`${scope}-badges`}>{text("quiz.results.newBadges")}</h2>
          <ul className="quiz-badges">
            {badges.map((badge) => (
              <li key={badge.id} className="quiz-badge" data-earned="">
                <span className="quiz-badge-emoji" aria-hidden="true">
                  {badge.emoji}
                </span>
                <div>
                  <h3>{localized(badge.label, locale)}</h3>
                  <p>{localized(badge.description, locale)}</p>
                </div>
              </li>
            ))}
          </ul>
        </section>
      )}
      {result.tasks.map((taskResult, index) => {
        const task = view.sheet.tasks.find((candidate) => candidate.id === taskResult.task);
        if (task === undefined) return null;
        return (
          <section key={taskResult.task} className="quiz-result-task" aria-labelledby={`${scope}-${taskResult.task}`}>
            <h2 id={`${scope}-${taskResult.task}`}>
              {index + 1}. {localized(task.title, locale)}
            </h2>
            <p className="quiz-muted">
              {text(TASK_KIND_LABELS[task.kind])} · {text("quiz.results.taskScore", { score: formatScore(taskResult.score, locale) })}
            </p>
            <TaskResultView task={task} result={taskResult} text={text} locale={locale} />
          </section>
        );
      })}
      <div className="quiz-actions">
        <button type="button" className="quiz-button quiz-button-primary" onClick={() => session.open({ screen: "home" })}>
          {text("quiz.results.home")}
        </button>
        <button type="button" className="quiz-button" onClick={() => session.open({ screen: "leaderboard" })}>
          {text("quiz.results.leaderboard")}
        </button>
      </div>
    </section>
  );
}
