/** ▶️ The run player: the sheet's tasks in order, one at a time, with overall progress, per-task completeness and a
 * submission that asks for confirmation, shows its progress and can be cancelled — on an untimed run only once every
 * task is complete, on a timed one at any time. Results are never shown here — they exist only after submission; the way out of a run is the navbar's
 * (answers are saved as they are given). The run and its current task are two
 * cards; the confirmation is a modal dialog over the client. What the others answered stays out of a run until it is
 * submitted, so nobody is led by the crowd — unless the learner asks to see it now (or chose to always see it): then
 * the task's figures show below its interaction, where nothing above them moves, until the learner hides them again or
 * the run ends. The submit button stays focusable while tasks are open, so its explanation can be reached and heard
 * from it. The head of the run and the task lay themselves out for the width of their cards: the tasks as a list, as
 * chips, or as chips beside the progress; the interaction in the tiers of its kind.
 *
 * The run card names the challenge. During an open run that hides the keys the others' answers are not offered, since
 * they would show the keys. On an easy run the task views show the run's hints. On a timed run (expert) a task shows
 * only its title, the time it allows and a button that starts its clock until it is opened; then its time left as
 * `m:ss` text that ticks every second — ticking digits are no motion, so they tick whatever the device says about
 * motion, and only the decorative bar under them follows it. The clock is a timer, no live region, whose text says the
 * time left in words, so it can be read at any moment; a separate polite status speaks when the clock starts, at 30 s,
 * at 10 s and when the time is up. Then the task turns read-only and says so — and whether an entry typed but not yet
 * committed was lost —, and focus stays where it was. Only the clock ticks: the task itself renders again once, when
 * its time is up. The time left is measured from the instant the task was opened by the session's clock, so a hidden
 * page loses nothing. Every timed task's step says whether its clock has not started, runs (with the time left) or is
 * up, and when the time of a task runs out while another is shown, the run's polite status says so once. A timed run
 * can be submitted at any time; the confirmation names the tasks still open and where their clocks stand.
 *
 * @see ../../🎨️.css — `.quiz-run-head`, `.quiz-task`, `.quiz-clock`
 * @see https://www.w3.org/WAI/WCAG22/Understanding/timing-adjustable.html
 */

import { memo, useCallback, useEffect, useId, useRef, useState, type CSSProperties, type ReactElement } from "react";
import { answerComplete, challengeRules, thinkingScope, type Answer, type Hint, type Id, type RunView, type SheetTask, type Slug } from "@semio-tech/quiz";
import { TASK_KIND_LABELS, localized, type QuizLabelKey, type QuizLocale, type QuizText } from "../🌐️i18n/🟦️.ts";
import { ClassificationTaskView } from "../🗂️classification/🟦️.tsx";
import { SortingTaskView } from "../↕️sorting/🟦️.tsx";
import { MatchingTaskView } from "../🃏️matching/🟦️.tsx";
import { failureProblem, thrownProblem } from "../🪪️identity/🟦️.tsx";
import { BodyButton, CardAction, CardIcon, Dialog, Glyph, IconLabel, LiveRegion, Mark, ProblemNote, QuizCard, TaskGlyph, cn, useAnnouncement, type Problem } from "../🪟️chrome/🟦️.tsx";
import type { QuizSession, QuizState, SessionFailure, SubmissionPhase } from "../🧭️session/🟦️.ts";
import { PRESENCE_ANCHORS, usePresenceTask, usePresenceView } from "../👥️presence/🟦️.tsx";
import { CrowdDoor, TaskFigures, crowdGate, crowdShown, keysHidden, type OthersChoice } from "../🗳️crowd/🟦️.tsx";
import { CHALLENGE_LABELS } from "../⛰️challenge/🟦️.tsx";
import { formatCountdown, formatDuration } from "../📏️quantity/🟦️.ts";
import { PetTopic, petProp } from "../🐾️pets/🟦️.tsx";

/** 🧩️ The interaction of one presented task, whatever its kind, with the `hints` of its recorded answer, `locked` once
 * its time is up and `onDropped` told when an entry not yet committed is lost to the lock. */
export function TaskView(props: {
  readonly task: SheetTask;
  readonly answer: Answer | undefined;
  readonly onAnswer: (answer: Answer) => void;
  readonly onDropped?: () => void;
  readonly hints?: readonly Hint[];
  readonly locked?: boolean;
  readonly text: QuizText;
  readonly locale: QuizLocale;
}): ReactElement {
  const { task, answer, onAnswer, onDropped, hints, locked, text, locale } = props;
  switch (task.kind) {
    case "classification":
      return <ClassificationTaskView task={task} answer={answer?.kind === "classification" ? answer : undefined} onAnswer={onAnswer} onDropped={onDropped} hints={hints} locked={locked} text={text} locale={locale} />;
    case "sorting":
      return <SortingTaskView task={task} answer={answer?.kind === "sorting" ? answer : undefined} onAnswer={onAnswer} onDropped={onDropped} hints={hints} locked={locked} text={text} locale={locale} />;
    case "matching":
      return <MatchingTaskView task={task} answer={answer?.kind === "matching" ? answer : undefined} onAnswer={onAnswer} onDropped={onDropped} hints={hints} locked={locked} text={text} locale={locale} />;
  }
}

/** 🪨️ {@link TaskView} rendered again only when its props change — not when the status beside its clock speaks. */
const SteadyTaskView = /* @__PURE__ */ memo(TaskView);

/** ⏳️ The milliseconds left until `deadline` by `now`, never below zero, rendered anew whenever the whole seconds left
 * change and when the page shows again; `undefined` without a deadline. Every timer ends with the component. */
export function useRemaining(deadline: number | undefined, now: () => number): number | undefined {
  const [, setTicks] = useState(0);
  useEffect(() => {
    if (deadline === undefined) return undefined;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const schedule = (): void => {
      clearTimeout(timer);
      const left = deadline - now();
      timer = left > 0 ? setTimeout(tick, left % 1000 || 1000) : undefined;
    };
    const tick = (): void => {
      setTicks((ticks) => ticks + 1);
      schedule();
    };
    schedule();
    document.addEventListener("visibilitychange", tick);
    return () => {
      clearTimeout(timer);
      document.removeEventListener("visibilitychange", tick);
    };
  }, [deadline, now]);
  return deadline === undefined ? undefined : Math.max(0, deadline - now());
}

/** ⌚️ Whether `deadline` has passed by `now` — rendered anew once, when it passes, and when the page shows again;
 * `false` without a deadline. Every timer ends with the component. */
export function usePassed(deadline: number | undefined, now: () => number): boolean {
  const [, setChecks] = useState(0);
  useEffect(() => {
    if (deadline === undefined) return undefined;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const check = (): void => {
      clearTimeout(timer);
      const left = deadline - now();
      timer = left > 0 ? setTimeout(check, left) : undefined;
      if (left <= 0) setChecks((checks) => checks + 1);
    };
    check();
    document.addEventListener("visibilitychange", check);
    return () => {
      clearTimeout(timer);
      document.removeEventListener("visibilitychange", check);
    };
  }, [deadline, now]);
  return deadline !== undefined && now() >= deadline;
}

/** 🚥️ Where a task's clock stands: not started, running, in its last 30 s, in its last 10 s, or up. */
export type ClockStage = "closed" | "running" | "thirty" | "ten" | "up";

/** 🚦️ The stage of a clock with `left` milliseconds (`undefined`: not started). */
export function clockStage(left: number | undefined): ClockStage {
  if (left === undefined) return "closed";
  return left <= 0 ? "up" : left <= 10_000 ? "ten" : left <= 30_000 ? "thirty" : "running";
}

/** 🔔️ What the clock's status says on entering a running stage: the start with the time left in words, the last 30 s
 * and the last 10 s. The task says itself when its time is up. */
const CLOCK_SPOKEN: { readonly [S in Exclude<ClockStage, "closed" | "up">]: QuizLabelKey } = { running: "quiz.run.clockStarted", thirty: "quiz.run.clockThirty", ten: "quiz.run.clockTen" };

/** ⌛️ The symbol of each stage, before the time left or what the stage says. */
const CLOCK_SYMBOLS: { readonly [S in ClockStage]: string } = { closed: "⏸", running: "⏱", thirty: "⏳", ten: "⚠", up: "⌛" };

/** ⏱️ The clock of a timed task allowing `seconds`: until it has a `deadline` (the task is not opened) what the task
 * asks, the time it allows and the button that starts it — with progress and a way to cancel while it starts — and
 * from then on the time left, ticking by `now` on its own. It is a timer whose text says the time left in words; it has
 * the task's polite status `announce` the start, the last 30 s and the last 10 s on entering them while it is shown. */
export function TaskClock(props: {
  readonly seconds: number;
  readonly deadline: number | undefined;
  readonly now: () => number;
  readonly onStart: (signal: AbortSignal) => Promise<SessionFailure | undefined>;
  readonly onStarted: () => void;
  readonly announce: (message: string) => void;
  readonly text: QuizText;
  readonly locale: QuizLocale;
}): ReactElement {
  const { seconds, deadline, now, onStart, onStarted, announce, text, locale } = props;
  const left = useRemaining(deadline, now);
  const stage = clockStage(left);
  const [starting, setStarting] = useState<AbortController | undefined>(undefined);
  const [problem, setProblem] = useState<Problem | undefined>(undefined);
  const previous = useRef<ClockStage | undefined>(undefined);
  useEffect(() => () => starting?.abort(), [starting]);
  useEffect(() => {
    const before = previous.current;
    previous.current = stage;
    if (before === undefined || before === stage || stage === "closed" || stage === "up") return;
    announce(text(CLOCK_SPOKEN[stage], { time: formatDuration(left ?? 0, locale) }));
  });

  const start = (): void => {
    if (starting !== undefined) return;
    const controller = new AbortController();
    setProblem(undefined);
    setStarting(controller);
    onStart(controller.signal)
      .then((failure) => {
        setStarting(undefined);
        if (failure === undefined) onStarted();
        else setProblem(failureProblem(failure, text));
      })
      .catch((thrown: unknown) => {
        setStarting(undefined);
        setProblem(thrownProblem(thrown, controller.signal, text));
      });
  };

  return (
    <>
      {stage === "closed" ? (
        <div className="quiz-clock flex flex-col gap-single" data-clock={stage}>
          <p className="quiz-prose m-0 text-sm leading-normal">{text("quiz.run.clockIntro")}</p>
          <p className="m-0 text-sm font-semibold tabular-nums">{text("quiz.run.clockAllowed", { time: formatCountdown(seconds * 1000) })}</p>
          <div className="flex flex-wrap items-center gap-double">
            <BodyButton primary onClick={start}>
              {text("quiz.run.clockStart")}
            </BodyButton>
            {starting === undefined ? null : (
              <>
                <progress aria-label={text("quiz.run.clockStarting")} className="quiz-progress" />
                <span className="text-sm">{text("quiz.run.clockStarting")}</span>
                <BodyButton onClick={() => starting.abort()}>{text("quiz.run.cancel")}</BodyButton>
              </>
            )}
          </div>
          {problem === undefined ? null : <ProblemNote problem={problem} text={text} />}
        </div>
      ) : (
        <p className="quiz-clock m-0 flex flex-col gap-single text-sm" data-clock={stage} role="timer">
          <span className="font-semibold tabular-nums">
            <Mark symbol={CLOCK_SYMBOLS[stage]} />
            {stage === "up" ? (
              text("quiz.run.clockUp")
            ) : (
              <>
                <span aria-hidden="true">{text("quiz.run.clockLeft", { time: formatCountdown(left ?? 0) })}</span>
                <span className="sr-only">{text("quiz.run.clockLeft", { time: formatDuration(left ?? 0, locale) })}</span>
              </>
            )}
          </span>
          <span className="quiz-clock-bar" aria-hidden="true" style={{ "--quiz-clock-share": String(Math.min(1, (left ?? 0) / (seconds * 1000))) } as CSSProperties} />
          {stage === "up" ? <span className="text-xs text-muted-foreground">{text("quiz.task.locked")}</span> : null}
        </p>
      )}
    </>
  );
}

/** ⏰️ Where the clock of a timed task allowing `seconds` and `opened` at an instant stands, in words after its symbol:
 * not started, the time left (`m:ss` shown, words read) or up — ticking on its own by `now`. */
function ClockState(props: { readonly seconds: number; readonly opened: number | undefined; readonly now: () => number; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { seconds, opened, now, text, locale } = props;
  const left = useRemaining(opened === undefined ? undefined : opened + seconds * 1000, now);
  const stage = clockStage(left);
  return (
    <span className="quiz-clock-state" data-clock-state={stage}>
      <Mark symbol={CLOCK_SYMBOLS[stage]} />
      {stage === "closed" ? (
        text("quiz.run.clockClosed")
      ) : stage === "up" ? (
        text("quiz.run.clockUp")
      ) : (
        <>
          <span aria-hidden="true">{text("quiz.run.clockLeft", { time: formatCountdown(left ?? 0) })}</span>
          <span className="sr-only">{text("quiz.run.clockLeft", { time: formatDuration(left ?? 0, locale) })}</span>
        </>
      )}
    </span>
  );
}

/** 🛎️ The polite status of a timed run that says once when the time of a task runs out while another one is shown;
 * the time of the `shown` task is said by its own clock, and tasks whose time ran out before this status was shown say
 * nothing. One timer waits for the next deadline; it ends with the component. */
function ClockWatch(props: { readonly tasks: readonly SheetTask[]; readonly opened: Readonly<Record<Slug, number>> | undefined; readonly shown: Slug | undefined; readonly now: () => number; readonly text: QuizText }): ReactElement {
  const { tasks, opened, now } = props;
  const { announcement, announce } = useAnnouncement();
  const told = useRef<Set<Slug> | undefined>(undefined);
  const deadlines = tasks.flatMap((task, index) => (task.seconds === undefined || opened?.[task.id] === undefined ? [] : [{ task: task.id, index, deadline: opened[task.id]! + task.seconds * 1000 }]));
  const latest = useRef({ deadlines, shown: props.shown, text: props.text });
  latest.current = { deadlines, shown: props.shown, text: props.text };
  const signature = deadlines.map((entry) => `${entry.task}@${entry.deadline}`).join("|");
  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const check = (): void => {
      clearTimeout(timer);
      const at = now();
      const { deadlines: current, shown, text } = latest.current;
      const quiet = told.current === undefined;
      const seen = told.current ?? new Set<Slug>();
      told.current = seen;
      const ended = current.filter((entry) => entry.deadline <= at && !seen.has(entry.task));
      for (const entry of ended) seen.add(entry.task);
      const spoken = ended.filter((entry) => entry.task !== shown).map((entry) => text("quiz.run.taskUp", { index: entry.index + 1 }));
      if (!quiet && spoken.length > 0) announce(spoken.join(" "));
      const next = Math.min(...current.filter((entry) => entry.deadline > at).map((entry) => entry.deadline));
      timer = Number.isFinite(next) ? setTimeout(check, next - at) : undefined;
    };
    check();
    document.addEventListener("visibilitychange", check);
    return () => {
      clearTimeout(timer);
      document.removeEventListener("visibilitychange", check);
    };
  }, [signature, now, announce]);
  return <LiveRegion announcement={announcement} />;
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

/** 📋️ The body of the task `task` of `view`: its prompt and its interaction with the run's hints — on a timed run behind
 * its clock, and locked once its time is up or the run is closed. It renders again only when the time is up, not with
 * every tick of its clock. An answer after the deadline by `now` is not sent but lost, like an entry that does not read
 * as an answer when the lock comes; the polite status says that the time is up — after the clock's own announcements —
 * and whether something was lost, and the loss also shows under the clock. The interaction lies in the pets' topic of
 * the task (`<quiz>/<task>`), so its items mark themselves for the pets of their topic. */
function TaskBody(props: { readonly session: QuizSession; readonly view: RunView; readonly task: SheetTask; readonly now: () => number; readonly onStarted: () => void; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {
  const { session, view, task, now, text, locale } = props;
  const opened = view.opened?.[task.id];
  const deadline = task.seconds === undefined || opened === undefined ? undefined : opened + task.seconds * 1000;
  const up = usePassed(deadline, now);
  const locked = view.status !== "open" || up;
  const { announcement, announce } = useAnnouncement();
  const lost = useRef(false);
  const [dropped, setDropped] = useState(false);
  const drop = useCallback(() => {
    lost.current = true;
    setDropped(true);
  }, []);
  const { status, run } = view;
  const answer = useCallback(
    (given: Answer): void => {
      if (status !== "open") return;
      if (deadline === undefined || now() < deadline) session.answer(run, task.id, given);
      else drop();
    },
    [status, run, deadline, now, session, task.id, drop],
  );
  const wasUp = useRef(up);
  useEffect(() => {
    const before = wasUp.current;
    wasUp.current = up;
    if (up && !before) announce(lost.current ? `${text("quiz.task.locked")} ${text("quiz.task.dropped")}` : text("quiz.task.locked"));
  }, [up, announce, text]);
  return (
    <>
      {task.seconds === undefined ? null : (
        <div className="flex flex-col gap-single">
          <TaskClock seconds={task.seconds} deadline={deadline} now={now} onStart={(signal) => session.openTask(view.run, task.id, signal)} onStarted={props.onStarted} announce={announce} text={text} locale={locale} />
          {dropped ? <p className="m-0 text-xs text-muted-foreground">{text("quiz.task.dropped")}</p> : null}
          <LiveRegion announcement={announcement} />
        </div>
      )}
      {task.seconds !== undefined && opened === undefined ? null : (
        <>
          <p className="quiz-prose m-0 text-sm leading-normal">{localized(task.prompt, locale)}</p>
          <div className="quiz-task">
            <PetTopic topic={petProp(view.quiz, task.id)}>
              <SteadyTaskView task={task} answer={view.answers[task.id]} onAnswer={answer} onDropped={drop} hints={view.hints?.[task.id]} locked={locked} text={text} locale={locale} />
            </PetTopic>
          </div>
        </>
      )}
    </>
  );
}

/** 🎬️ The run screen of `run` for a learner who chose when the `others` show (once submitted, unless said otherwise). */
export function RunScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale; readonly others?: OthersChoice }): ReactElement | null {
  const { session, state, run, text, locale } = props;
  const scope = useId();
  const now = useCallback(() => session.now(), [session]);
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
  const gate = crowdGate(props.others ?? "submitted", "run", { asked: state.asked.includes(view.quiz), submitted: false, hidden: keysHidden(view) });
  const tasks = view.sheet.tasks;
  const complete = tasks.map((task) => answerComplete(task, view.answers[task.id]));
  const done = complete.filter(Boolean).length;
  const timed = challengeRules(view.sheet.challenge).timed;
  const ready = view.status === "open" && (timed || done === tasks.length);
  const clockOf = (entry: SheetTask): ReactElement | null => (entry.seconds === undefined ? null : <ClockState seconds={entry.seconds} opened={view.opened?.[entry.id]} now={now} text={text} locale={locale} />);
  const open = tasks.flatMap((task, position) => (complete[position] ? [] : [{ task, position }]));
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
        <p className="quiz-prose m-0 text-sm leading-normal text-muted-foreground">{localized(view.sheet.description, locale)}</p>
        <p className="m-0 text-sm font-semibold">{text("quiz.run.challenge", { challenge: text(CHALLENGE_LABELS[view.sheet.challenge]) })}</p>
        <div className="quiz-run-head">
          <nav aria-label={text("quiz.run.tasks")} className="min-w-0">
            <ol role="list" className="quiz-steps m-0 list-none p-0">
              {tasks.map((entry, position) => (
                <li key={entry.id}>
                  <button
                    type="button"
                    aria-current={position === index ? "step" : undefined}
                    onClick={() => go(position)}
                    className={cn(
                      "quiz-step quiz-target min-w-0 cursor-pointer border border-normal px-double py-single text-left text-sm transition-colors",
                      position === index ? "bg-active-base text-active-foreground" : "text-foreground hover:bg-hover-interactive-fill",
                    )}
                  >
                    <span className="min-w-0 font-semibold">
                      <IconLabel icon={entry.icon} order={position}>
                        {position + 1}. {localized(entry.title, locale)}
                      </IconLabel>
                    </span>
                    <span className="quiz-step-state text-xs" data-complete={complete[position] ? "" : undefined}>
                      <Mark symbol={complete[position] ? "✓" : "○"} />
                      {complete[position] ? text("quiz.run.complete") : text("quiz.run.incomplete")}
                      {entry.seconds === undefined ? null : <> · {clockOf(entry)}</>}
                    </span>
                  </button>
                </li>
              ))}
            </ol>
          </nav>
          <div className="quiz-run-progress text-sm">
            <progress max={tasks.length} value={done} aria-labelledby={`${scope}-progress`} className="quiz-progress" />
            <span id={`${scope}-progress`}>{text("quiz.run.progress", { done, total: tasks.length })}</span>
          </div>
        </div>
        {ready ? null : (
          <p id={`${scope}-hint`} className="m-0 text-xs text-muted-foreground">
            {text("quiz.run.submitHint")}
          </p>
        )}
        {problem === undefined ? null : <ProblemNote problem={problem} text={text} />}
        {timed && view.status === "open" ? <ClockWatch tasks={tasks} opened={view.opened} shown={task?.id} now={now} text={text} /> : null}
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
          <TaskBody key={task.id} session={session} view={view} task={task} now={now} onStarted={() => taskHeading.current?.focus()} text={text} locale={locale} />
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
            <div id={`${scope}-dialog-body`} className="flex flex-col gap-single text-sm leading-normal">
              <p className="m-0">{text("quiz.run.confirmBody")}</p>
              {open.length === 0 ? null : (
                <>
                  <p className="m-0 font-semibold">{text("quiz.run.confirmOpen")}</p>
                  <ul role="list" className="m-0 flex list-none flex-col gap-single p-0">
                    {open.map(({ task: entry, position }) => (
                      <li key={entry.id}>
                        <Mark symbol="○" />
                        {position + 1}. {localized(entry.title, locale)}
                        {entry.seconds === undefined ? null : <> · {clockOf(entry)}</>}
                      </li>
                    ))}
                  </ul>
                </>
              )}
            </div>
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
