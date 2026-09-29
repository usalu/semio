/** ▶️ The run player: the sheet's tasks in order, one at a time, with overall progress, per-task completeness and a
 * submission that is only possible once every task is complete, asks for confirmation, shows its progress and can be
 * cancelled. Results are never shown here — they exist only after submission.
 */

import { useEffect, useId, useRef, useState, type KeyboardEvent, type ReactElement } from "react";
import { answerComplete, type Answer, type Id, type SheetTask } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { ClassificationTaskView } from "../🗂️classification/🟦️.tsx";
import { SortingTaskView } from "../↕️sorting/🟦️.tsx";
import { MatchingTaskView } from "../🃏️matching/🟦️.tsx";
import { failureMessage, thrownMessage } from "../🪪️identity/🟦️.tsx";
import type { QuizSession, QuizState, SubmissionPhase } from "../🧭️session/🟦️.ts";

/** 🧩️ The interaction of one presented task, whatever its kind. */
export function TaskView(props: { readonly task: SheetTask; readonly answer: Answer | undefined; readonly onAnswer: (answer: Answer) => void; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { task, answer, onAnswer, text, locale } = props;
  switch (task.kind) {
    case "classification":
      return <ClassificationTaskView task={task} answer={answer?.kind === "classification" ? answer : undefined} onAnswer={onAnswer} text={text} locale={locale} />;
    case "sorting":
      return <SortingTaskView task={task} answer={answer?.kind === "sorting" ? answer : undefined} onAnswer={onAnswer} text={text} locale={locale} />;
    case "matching":
      return <MatchingTaskView task={task} answer={answer?.kind === "matching" ? answer : undefined} onAnswer={onAnswer} text={text} locale={locale} />;
  }
}

function phaseText(phase: SubmissionPhase, text: QuizText): string {
  switch (phase.phase) {
    case "saving":
      return text("quiz.run.saving", { done: phase.done, total: phase.total });
    case "submitting":
      return text("quiz.run.submitting");
    case "results":
      return text("quiz.run.loadingResults");
  }
}

type Submission = { readonly stage: "confirming" } | { readonly stage: "working"; readonly controller: AbortController; readonly phase: SubmissionPhase };

/** ▶️ The run screen of `run`. */
export function RunScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {
  const { session, state, run, text, locale } = props;
  const scope = useId();
  const [current, setCurrent] = useState(0);
  const [submission, setSubmission] = useState<Submission | undefined>(undefined);
  const [message, setMessage] = useState<string | undefined>(undefined);
  const taskHeading = useRef<HTMLHeadingElement>(null);
  const submitButton = useRef<HTMLButtonElement>(null);
  const dialog = useRef<HTMLDivElement>(null);
  const moved = useRef(false);
  const working = submission?.stage === "working" ? submission.controller : undefined;
  useEffect(() => () => working?.abort(), [working]);
  useEffect(() => {
    if (moved.current) taskHeading.current?.focus();
    moved.current = false;
  }, [current]);
  useEffect(() => {
    if (submission !== undefined) dialog.current?.querySelector<HTMLElement>("[data-autofocus]")?.focus();
  }, [submission?.stage]);

  const view = state.runs[run];
  if (view === undefined) return null;
  const tasks = view.sheet.tasks;
  const complete = tasks.map((task) => answerComplete(task, view.answers[task.id]));
  const done = complete.filter(Boolean).length;
  const ready = done === tasks.length && view.status === "open";
  const index = Math.min(current, tasks.length - 1);
  const task = tasks[index];
  const go = (target: number): void => {
    moved.current = true;
    setCurrent(Math.max(0, Math.min(tasks.length - 1, target)));
  };
  const close = (): void => {
    setSubmission(undefined);
    submitButton.current?.focus();
  };
  const submit = (): void => {
    const controller = new AbortController();
    setMessage(undefined);
    setSubmission({ stage: "working", controller, phase: { phase: "saving", done: 0, total: 0 } });
    session
      .submit(run, controller.signal, (phase) => setSubmission((present) => (present?.stage === "working" && present.controller === controller ? { ...present, phase } : present)))
      .then((failure) => {
        if (failure === undefined) return;
        setMessage(failureMessage(failure, text));
        close();
      })
      .catch((thrown: unknown) => {
        setMessage(thrownMessage(thrown, controller.signal, text));
        close();
      });
  };
  const trap = (event: KeyboardEvent<HTMLDivElement>): void => {
    if (event.key === "Escape") {
      event.preventDefault();
      if (submission?.stage === "working") submission.controller.abort();
      else close();
      return;
    }
    if (event.key !== "Tab") return;
    const focusable = [...(dialog.current?.querySelectorAll<HTMLElement>("button:not([disabled])") ?? [])];
    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last?.focus();
    } else if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first?.focus();
    }
  };

  return (
    <section className="quiz-run" aria-labelledby={`${scope}-title`}>
      <h1 id={`${scope}-title`} tabIndex={-1}>
        {localized(view.sheet.title, locale)}
      </h1>
      <p className="quiz-muted">{localized(view.sheet.description, locale)}</p>
      <div className="quiz-progress">
        <progress max={tasks.length} value={done} aria-labelledby={`${scope}-progress`} />
        <span id={`${scope}-progress`}>{text("quiz.run.progress", { done, total: tasks.length })}</span>
      </div>
      <div className="quiz-run-layout">
        <nav className="quiz-task-nav" aria-label={text("quiz.run.tasks")}>
          <ol>
            {tasks.map((entry, position) => (
              <li key={entry.id}>
                <button type="button" aria-current={position === index ? "step" : undefined} onClick={() => go(position)}>
                  <span className="quiz-task-nav-title">
                    {position + 1}. {localized(entry.title, locale)}
                  </span>
                  <span className="quiz-state" data-complete={complete[position] ? "" : undefined}>
                    {complete[position] ? `✓ ${text("quiz.run.complete")}` : `○ ${text("quiz.run.incomplete")}`}
                  </span>
                </button>
              </li>
            ))}
          </ol>
        </nav>
        {task === undefined ? null : (
          <article className="quiz-task" aria-labelledby={`${scope}-task`}>
            <p className="quiz-muted">
              {text("quiz.run.task", { index: index + 1, total: tasks.length })} · {text(TASK_KIND_LABELS[task.kind])} · {complete[index] ? text("quiz.run.complete") : text("quiz.run.incomplete")}
            </p>
            <h2 id={`${scope}-task`} ref={taskHeading} tabIndex={-1}>
              {localized(task.title, locale)}
            </h2>
            <p className="quiz-prompt">{localized(task.prompt, locale)}</p>
            <TaskView key={task.id} task={task} answer={view.answers[task.id]} onAnswer={(answer) => session.answer(run, task.id, answer)} text={text} locale={locale} />
            <div className="quiz-task-footer">
              <button type="button" className="quiz-button" disabled={index === 0} onClick={() => go(index - 1)}>
                ← {text("quiz.run.previous")}
              </button>
              <button type="button" className="quiz-button" disabled={index === tasks.length - 1} onClick={() => go(index + 1)}>
                {text("quiz.run.next")} →
              </button>
            </div>
          </article>
        )}
      </div>
      <div className="quiz-submit">
        {ready ? null : (
          <p id={`${scope}-hint`} className="quiz-muted">
            {text("quiz.run.submitHint")}
          </p>
        )}
        {message === undefined ? null : (
          <p className="quiz-error" role="alert">
            {message}
          </p>
        )}
        <button ref={submitButton} type="button" className="quiz-button quiz-button-primary" disabled={!ready} aria-describedby={ready ? undefined : `${scope}-hint`} onClick={() => setSubmission({ stage: "confirming" })}>
          {text("quiz.run.submit")}
        </button>
      </div>
      {submission === undefined ? null : (
        <div className="quiz-dialog-backdrop">
          <div ref={dialog} className="quiz-dialog" role={submission.stage === "confirming" ? "alertdialog" : "dialog"} aria-modal="true" aria-labelledby={`${scope}-dialog`} aria-describedby={`${scope}-dialog-body`} onKeyDown={trap}>
            <h2 id={`${scope}-dialog`}>{text("quiz.run.confirmTitle")}</h2>
            {submission.stage === "confirming" ? (
              <>
                <p id={`${scope}-dialog-body`}>{text("quiz.run.confirmBody")}</p>
                <div className="quiz-actions">
                  <button type="button" className="quiz-button quiz-button-primary" data-autofocus="" onClick={submit}>
                    {text("quiz.run.confirm")}
                  </button>
                  <button type="button" className="quiz-button" onClick={close}>
                    {text("quiz.run.keepWorking")}
                  </button>
                </div>
              </>
            ) : (
              <>
                <div id={`${scope}-dialog-body`} role="status" className="quiz-progress">
                  {submission.phase.phase === "saving" && submission.phase.total > 0 ? (
                    <progress max={submission.phase.total} value={submission.phase.done} aria-label={text("quiz.run.progressLabel")} />
                  ) : (
                    <progress aria-label={text("quiz.run.progressLabel")} />
                  )}
                  <span>{phaseText(submission.phase, text)}</span>
                </div>
                <div className="quiz-actions">
                  <button type="button" className="quiz-button" data-autofocus="" onClick={() => submission.controller.abort()}>
                    {text("quiz.run.cancel")}
                  </button>
                </div>
              </>
            )}
          </div>
        </div>
      )}
    </section>
  );
}
