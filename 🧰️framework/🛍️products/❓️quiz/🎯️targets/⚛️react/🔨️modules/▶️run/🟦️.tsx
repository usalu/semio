/** ▶️ The run player: the sheet's tasks in order, one at a time, with overall progress, per-task completeness and a
 * submission that is only possible once every task is complete, asks for confirmation, shows its progress and can be
 * cancelled. Results are never shown here — they exist only after submission. The run and its current task are two
 * cards; the confirmation is a card at dialog level over the page. Unless the learner wants to think alone, each item
 * shows what the others think: live while anyone thinks along, else what the submitted runs answered.
 */

import { useEffect, useId, useRef, useState, type KeyboardEvent, type ReactElement } from "react";
import type { IconName } from "@semio-tech/ui-react/chrome";
import { answerComplete, type Answer, type Id, type SheetTask } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { ClassificationTaskView } from "../🗂️classification/🟦️.tsx";
import { SortingTaskView } from "../↕️sorting/🟦️.tsx";
import { MatchingTaskView } from "../🃏️matching/🟦️.tsx";
import { failureMessage, thrownMessage } from "../🪪️identity/🟦️.tsx";
import { CardAction, CardIcon, Glyph, QuizCard, cn } from "../🪟️chrome/🟦️.tsx";
import type { QuizSession, QuizState, SubmissionPhase } from "../🧭️session/🟦️.ts";
import { PRESENCE_ANCHORS, usePresenceTask, usePresenceView } from "../👥️presence/🟦️.tsx";
import { CrowdProvider, CrowdSource, chooseCrowd } from "../🗳️crowd/🟦️.tsx";
import { thinkingScope } from "@semio-tech/quiz";

/** 🔣️ The icon of each task kind in its card's title chip. */
export const TASK_KIND_ICONS: { readonly [K in SheetTask["kind"]]: IconName } = { classification: "layout-grid", sorting: "list", matching: "link" };

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
export function RunScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale; readonly showAnswers?: boolean }): ReactElement | null {
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
  usePresenceTask(view?.sheet.tasks[Math.min(current, view.sheet.tasks.length - 1)]?.id);
  const { thinking, colours } = usePresenceView();
  if (view === undefined) return null;
  const others = state.catalog === undefined ? [] : (thinking.get(thinkingScope(state.catalog.id, view.quiz)) ?? []);
  const crowd = props.showAnswers === false ? undefined : chooseCrowd(others, state.crowds[view.quiz]);
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
  const emoji = state.catalog?.quizzes.find((quiz) => quiz.id === view.quiz)?.emoji;

  return (
    <div className="quiz-run flex flex-col gap-double">
      <QuizCard
        id={`${scope}-title`}
        card="run"
        anchor={PRESENCE_ANCHORS.run}
        headingLevel={1}
        focusableHeading
        icon={emoji === undefined ? <CardIcon icon="list" /> : <Glyph emoji={emoji} className="text-sm" />}
        title={localized(view.sheet.title, locale)}
        footerLeft={<CardAction onClick={() => session.open({ screen: "home" })}>{text("quiz.nav.home")}</CardAction>}
        footerRight={
          <CardAction ref={submitButton} primary disabled={!ready} aria-describedby={ready ? undefined : `${scope}-hint`} onClick={() => setSubmission({ stage: "confirming" })}>
            {text("quiz.run.submit")}
          </CardAction>
        }
      >
        <p className="m-0 text-sm leading-normal text-muted-foreground">{localized(view.sheet.description, locale)}</p>
        <div className="flex flex-wrap items-center gap-double text-sm">
          <progress max={tasks.length} value={done} aria-labelledby={`${scope}-progress`} className="quiz-progress" />
          <span id={`${scope}-progress`}>{text("quiz.run.progress", { done, total: tasks.length })}</span>
        </div>
        <nav aria-label={text("quiz.run.tasks")}>
          <ol className="m-0 flex list-none flex-wrap gap-single p-0">
            {tasks.map((entry, position) => (
              <li key={entry.id} className="min-w-0">
                <button
                  type="button"
                  aria-current={position === index ? "step" : undefined}
                  onClick={() => go(position)}
                  className={cn(
                    "quiz-target flex min-w-0 cursor-pointer flex-col items-start border border-normal px-double py-single text-left text-sm transition-colors",
                    position === index ? "bg-active-base text-active-foreground" : "text-foreground hover:bg-hover-interactive-fill",
                  )}
                >
                  <span className="font-semibold">
                    {position + 1}. {localized(entry.title, locale)}
                  </span>
                  <span className="text-xs" data-complete={complete[position] ? "" : undefined}>
                    {complete[position] ? `✓ ${text("quiz.run.complete")}` : `○ ${text("quiz.run.incomplete")}`}
                  </span>
                </button>
              </li>
            ))}
          </ol>
        </nav>
        {ready ? null : (
          <p id={`${scope}-hint`} className="m-0 text-xs text-muted-foreground">
            {text("quiz.run.submitHint")}
          </p>
        )}
        {message === undefined ? null : (
          <p role="alert" className="quiz-alert m-0 border border-normal px-double py-single text-sm font-semibold">
            {message}
          </p>
        )}
      </QuizCard>
      {task === undefined ? null : (
        <QuizCard
          id={`${scope}-task`}
          card="task"
          anchor={PRESENCE_ANCHORS.task(task.id)}
          headingRef={taskHeading}
          focusableHeading
          icon={<CardIcon icon={TASK_KIND_ICONS[task.kind]} />}
          title={localized(task.title, locale)}
          footerLeft={
            <CardAction disabled={index === 0} onClick={() => go(index - 1)}>
              {text("quiz.run.previous")}
            </CardAction>
          }
          footerRight={
            <CardAction primary disabled={index === tasks.length - 1} onClick={() => go(index + 1)}>
              {text("quiz.run.next")}
            </CardAction>
          }
        >
          <p className="m-0 text-xs text-muted-foreground">
            {text("quiz.run.task", { index: index + 1, total: tasks.length })} · {text(TASK_KIND_LABELS[task.kind])} · {complete[index] ? text("quiz.run.complete") : text("quiz.run.incomplete")}
          </p>
          <p className="m-0 text-sm leading-normal">{localized(task.prompt, locale)}</p>
          {crowd === undefined ? null : <CrowdSource crowd={crowd} text={text} />}
          <CrowdProvider crowd={crowd} colours={colours}>
            <TaskView key={task.id} task={task} answer={view.answers[task.id]} onAnswer={(answer) => session.answer(run, task.id, answer)} text={text} locale={locale} />
          </CrowdProvider>
        </QuizCard>
      )}
      {submission === undefined ? null : (
        <div className="quiz-backdrop fixed inset-0 z-50 grid place-items-center p-double">
          <div ref={dialog} role={submission.stage === "confirming" ? "alertdialog" : "dialog"} aria-modal="true" aria-labelledby={`${scope}-dialog`} aria-describedby={`${scope}-dialog-body`} onKeyDown={trap} className="w-full max-w-md">
            <QuizCard
              id={`${scope}-dialog`}
              card="dialog"
              icon={<CardIcon icon="info" />}
              title={text("quiz.run.confirmTitle")}
              footerLeft={
                submission.stage === "confirming" ? (
                  <CardAction onClick={close}>{text("quiz.run.keepWorking")}</CardAction>
                ) : (
                  <CardAction data-autofocus="" onClick={() => submission.controller.abort()}>
                    {text("quiz.run.cancel")}
                  </CardAction>
                )
              }
              footerRight={
                submission.stage === "confirming" ? (
                  <CardAction primary data-autofocus="" onClick={submit}>
                    {text("quiz.run.confirm")}
                  </CardAction>
                ) : undefined
              }
            >
              {submission.stage === "confirming" ? (
                <p id={`${scope}-dialog-body`} className="m-0 text-sm leading-normal">
                  {text("quiz.run.confirmBody")}
                </p>
              ) : (
                <div id={`${scope}-dialog-body`} role="status" className="flex flex-wrap items-center gap-double text-sm">
                  {submission.phase.phase === "saving" && submission.phase.total > 0 ? (
                    <progress max={submission.phase.total} value={submission.phase.done} aria-label={text("quiz.run.progressLabel")} className="quiz-progress" />
                  ) : (
                    <progress aria-label={text("quiz.run.progressLabel")} className="quiz-progress" />
                  )}
                  <span>{phaseText(submission.phase, text)}</span>
                </div>
              )}
            </QuizCard>
          </div>
        </div>
      )}
    </div>
  );
}
