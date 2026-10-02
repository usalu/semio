/** ▶️ The run player: the sheet's tasks in order, one at a time, with overall progress, per-task completeness and a
 * submission that is only possible once every task is complete, asks for confirmation, shows its progress and can be
 * cancelled. Results are never shown here — they exist only after submission; the way out of a run is the navbar's
 * (answers are saved as they are given). The run and its current task are two
 * cards; the confirmation is a modal dialog over the client. What the others answered stays out of a run until it is
 * submitted, so nobody is led by the crowd — unless the learner asks to see it now (or chose to always see it): then
 * the task's figures show below its interaction, where nothing above them moves, until the learner hides them again or
 * the run ends. The submit button stays focusable while tasks are open, so its explanation can be reached and heard
 * from it.
 */

import { useEffect, useId, useRef, useState, type ReactElement } from "react";
import type { IconName } from "@semio-tech/ui-react/chrome";
import { answerComplete, thinkingScope, type Answer, type Id, type SheetTask, type Icon, type TaskKind } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLabelKey, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { ClassificationTaskView } from "../🗂️classification/🟦️.tsx";
import { SortingTaskView } from "../↕️sorting/🟦️.tsx";
import { MatchingTaskView } from "../🃏️matching/🟦️.tsx";
import { failureProblem, thrownProblem } from "../🪪️identity/🟦️.tsx";
import { CardAction, CardIcon, Dialog, Glyph, IconLabel, Mark, ProblemNote, QuizCard, cn, type Problem } from "../🪟️chrome/🟦️.tsx";
import type { QuizSession, QuizState, SubmissionPhase } from "../🧭️session/🟦️.ts";
import { PRESENCE_ANCHORS, usePresenceTask, usePresenceView } from "../👥️presence/🟦️.tsx";
import { CrowdDoor, TaskFigures, crowdGate, crowdShown, type OthersChoice } from "../🗳️crowd/🟦️.tsx";

/** 🔣️ The icon of each task kind in its card's title chip. */
export const TASK_KIND_ICONS: { readonly [K in SheetTask["kind"]]: IconName } = { classification: "layout-grid", sorting: "list", matching: "link" };

/** 🖼️ The icon of a task in its title chip: the task's own emoji playing its microanimation, else the icon of its kind. */
export function TaskGlyph(props: { readonly task: { readonly kind: TaskKind; readonly icon?: Icon } }): ReactElement {
  const { kind, icon } = props.task;
  return icon === undefined ? <CardIcon icon={TASK_KIND_ICONS[kind]} /> : <Glyph emoji={icon.emoji} motion={icon.motion} className="text-sm" />;
}

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

/** 📣️ What a phase of the submission is called when it begins — without the counts that change while it lasts. */
const PHASE_SPOKEN: { readonly [P in SubmissionPhase["phase"]]: QuizLabelKey } = { saving: "quiz.run.savingAnswers", submitting: "quiz.run.submitting", results: "quiz.run.loadingResults" };

type Submission = { readonly stage: "confirming" } | { readonly stage: "working"; readonly controller: AbortController; readonly phase: SubmissionPhase };

/** ▶️ The run screen of `run` for a learner who chose when the `others` show (once submitted, unless said otherwise). */
export function RunScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale; readonly others?: OthersChoice }): ReactElement | null {
  const { session, state, run, text, locale } = props;
  const scope = useId();
  const [current, setCurrent] = useState(0);
  const [submission, setSubmission] = useState<Submission | undefined>(undefined);
  const [problem, setProblem] = useState<Problem | undefined>(undefined);
  const taskHeading = useRef<HTMLHeadingElement>(null);
  const moved = useRef(false);
  const working = submission?.stage === "working" ? submission.controller : undefined;
  useEffect(() => () => working?.abort(), [working]);
  useEffect(() => {
    if (moved.current) taskHeading.current?.focus();
    moved.current = false;
  }, [current]);

  const view = state.runs[run];
  usePresenceTask(view?.sheet.tasks[Math.min(current, view.sheet.tasks.length - 1)]?.id);
  const { thinking } = usePresenceView();
  if (view === undefined) return null;
  const gate = crowdGate(props.others ?? "submitted", "run", { asked: state.asked.includes(view.quiz), submitted: false });
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
  const close = (): void => setSubmission(undefined);
  const submit = (): void => {
    const controller = new AbortController();
    setProblem(undefined);
    setSubmission({ stage: "working", controller, phase: { phase: "saving", done: 0, total: 0 } });
    session
      .submit(run, controller.signal, (phase) => setSubmission((present) => (present?.stage === "working" && present.controller === controller ? { ...present, phase } : present)))
      .then((failure) => {
        if (failure === undefined) return;
        setProblem(failureProblem(failure, text));
        close();
      })
      .catch((thrown: unknown) => {
        setProblem(thrownProblem(thrown, controller.signal, text));
        close();
      });
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
        footerRight={
          <CardAction primary aria-disabled={!ready} aria-describedby={ready ? undefined : `${scope}-hint`} className="aria-disabled:cursor-not-allowed aria-disabled:opacity-50" onClick={() => ready && setSubmission({ stage: "confirming" })}>
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
          <ol role="list" className="m-0 flex list-none flex-wrap gap-single p-0">
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
                    <IconLabel icon={entry.icon} order={position}>
                      {position + 1}. {localized(entry.title, locale)}
                    </IconLabel>
                  </span>
                  <span className="text-xs" data-complete={complete[position] ? "" : undefined}>
                    <Mark symbol={complete[position] ? "✓" : "○"} />
                    {complete[position] ? text("quiz.run.complete") : text("quiz.run.incomplete")}
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
        {problem === undefined ? null : <ProblemNote problem={problem} text={text} />}
      </QuizCard>
      {task === undefined ? null : (
        <QuizCard
          id={`${scope}-task`}
          card="task"
          anchor={PRESENCE_ANCHORS.task(task.id)}
          headingRef={taskHeading}
          focusableHeading
          icon={<TaskGlyph task={task} />}
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
          <TaskView key={task.id} task={task} answer={view.answers[task.id]} onAnswer={(answer) => session.answer(run, task.id, answer)} text={text} locale={locale} />
          <CrowdDoor gate={gate} onAsk={() => session.askCrowd(view.quiz)} onUnask={() => session.unaskCrowd(view.quiz)} text={text} />
          {crowdShown(gate) ? (
            <TaskFigures task={task} title={localized(task.title, locale)} crowd={state.crowds[view.quiz]} thinking={state.catalog === undefined ? [] : thinking.get(thinkingScope(state.catalog.id, view.quiz))} answer={view.answers[task.id]} text={text} locale={locale} />
          ) : null}
        </QuizCard>
      )}
      {submission === undefined ? null : (
        <Dialog
          id={`${scope}-dialog`}
          role="alertdialog"
          icon={<CardIcon icon="info" />}
          title={text(submission.stage === "confirming" ? "quiz.run.confirmTitle" : "quiz.run.submitting")}
          describedBy={`${scope}-dialog-body`}
          focusKey={submission.stage}
          onEscape={() => (submission.stage === "working" ? submission.controller.abort() : close())}
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
            <>
              <div id={`${scope}-dialog-body`} aria-live="off" className="flex flex-wrap items-center gap-double text-sm">
                {submission.phase.phase === "saving" && submission.phase.total > 0 ? (
                  <progress max={submission.phase.total} value={submission.phase.done} aria-label={text("quiz.run.progressLabel")} className="quiz-progress" />
                ) : (
                  <progress aria-label={text("quiz.run.progressLabel")} className="quiz-progress" />
                )}
                <span>{phaseText(submission.phase, text)}</span>
              </div>
              <p role="status" className="sr-only">
                {text(PHASE_SPOKEN[submission.phase.phase])}
              </p>
            </>
          )}
        </Dialog>
      )}
    </div>
  );
}
