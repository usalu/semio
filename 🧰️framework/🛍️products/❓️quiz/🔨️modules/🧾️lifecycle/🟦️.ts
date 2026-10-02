/** 🧾️ The learner lifecycle as pure deciders: the handle streams that register pseudonyms and names exactly once, and the runs, answers, submissions and badges of one learner.
 *
 * `decide*` turns a command into events or a rejection at the decision time `now`; `evolve*` folds one event into the
 * state. The proctor wraps both in framework deciders; idempotency by command id is the framework's. Every decision
 * first holds the command to its id and slug shapes (`id-invalid`), so no malformed id reaches an event, and to the
 * caps of {@link Limits}, so no stream grows without bound. A claimed handle is recalled by a read, never by a
 * command: recalling writes nothing.
 *
 * @see ../../README.md — the run lifecycle table, the handle policy and the caps
 * @see ./🦀️.rs — the Rust twin
 */
import type { Answer, BadgeAward, Catalog, Event, IdentifyLearnerCommand, Identity, Limits, Quiz, Rejection, RunResult, RunStatus, Command, Timestamp } from "../../🧬️schema/🟦️.ts";
import { earnedBadges } from "../🏅️badges/🟦️.ts";
import { runSeed } from "../🎲️randomness/🟦️.ts";
import { sheetOf } from "../🃏️sheet/🟦️.ts";
import { scoreRun } from "../📏️scoring/🟦️.ts";
import { answerComplete, answerRejection, commandRejection, normalizeHandle } from "../✅️validation/🟦️.ts";

/** ⚖️ The outcome of a decision: the events to append, or why the command is rejected. */
export type Decision = { readonly events: readonly Event[] } | { readonly rejection: Rejection };

/** 🗂️ One handle key and the learner holding it, once claimed. */
export type HandleState = { readonly key: string; readonly holder?: string };

/** 📦️ A quiz as the proctor loaded it, with the content hash of its file. */
export type LoadedQuiz = { readonly quiz: Quiz; readonly revision: string };

/** 🧭️ What a learner decision reads besides the state: the decision time, the catalog, its quizzes by id and the caps. */
export type LearnerContext = { readonly now: Timestamp; readonly catalog: Catalog; readonly quizzes: Readonly<Record<string, LoadedQuiz>>; readonly limits: Limits };

/** 🏃️ One run of a learner as the learner stream built it; `recorded` counts every answer recorded, also the replaced ones. */
export type RunState = {
  readonly run: string;
  readonly quiz: string;
  readonly revision: string;
  readonly seed: number;
  readonly status: RunStatus;
  readonly answers: Readonly<Record<string, Answer>>;
  readonly recorded: number;
  readonly result?: RunResult;
  readonly startedAt: Timestamp;
  readonly submittedAt?: Timestamp;
};

/** 🧑‍🎓️ One learner: identity once registered, runs in start order and badges in award order. */
export type LearnerState = {
  readonly learner: string;
  readonly identity?: Identity;
  readonly runs: readonly RunState[];
  readonly badges: readonly BadgeAward[];
};

/** 🌱️ A handle key nobody claimed yet. */
export function emptyHandleState(key: string): HandleState {
  return { key };
}

/** 🚧️ `roster-full` once a proctor holds its cap of learners: the check a registration passes before it is decided. */
export function registrationRejection(learners: number, limits: Limits): Rejection | undefined {
  return learners >= limits.learners ? "roster-full" : undefined;
}

/** 🙋️ Registers the command's learner under a free handle; refuses a handle outside the policy or of another key (`handle-invalid`), an anonymous identity (it has no handle) and a claimed handle (`handle-claimed`). */
export function decideHandle(state: HandleState, command: IdentifyLearnerCommand, now: Timestamp): Decision {
  const malformed = commandRejection(command);
  if (malformed) return { rejection: malformed };
  if (command.identity.kind === "anonymous") return { rejection: "handle-invalid" };
  const handle = normalizeHandle(command.identity.handle);
  if (!handle || handle.key !== state.key) return { rejection: "handle-invalid" };
  if (state.holder !== undefined) return { rejection: "handle-claimed" };
  return { events: [{ type: "learner-registered", learner: command.learner, identity: { kind: command.identity.kind, handle: handle.display }, at: now }] };
}

/** 📇️ The learner registered under a pseudonym or name holds the handle from then on. */
export function evolveHandle(state: HandleState, event: Event): HandleState {
  return event.type === "learner-registered" && event.identity.kind !== "anonymous" ? { ...state, holder: event.learner } : state;
}

/** 🐣️ A learner before any event. */
export function emptyLearnerState(learner: string): LearnerState {
  return { learner, runs: [], badges: [] };
}

/** 🔎️ The run with the given id. */
function runOf(state: LearnerState, run: string): RunState | undefined {
  return state.runs.find((candidate) => candidate.run === run);
}

/** 🕰️ Whether a run was started against another revision than the loaded one (or its quiz is gone). */
function stale(run: RunState, context: LearnerContext): boolean {
  return !Object.hasOwn(context.quizzes, run.quiz) || context.quizzes[run.quiz]!.revision !== run.revision;
}

/** 🎫️ Registers an anonymous learner once; a pseudonym or name is registered by its handle stream, never here. */
function register(state: LearnerState, command: IdentifyLearnerCommand, context: LearnerContext): Decision {
  if (command.identity.kind !== "anonymous") return { rejection: "handle-invalid" };
  if (state.identity !== undefined) return { rejection: "learner-exists" };
  return { events: [{ type: "learner-registered", learner: state.learner, identity: { kind: "anonymous" }, at: context.now }] };
}

/** ▶️ Starts a run, voiding an open run of the same quiz whose revision is stale; refused once the learner submitted its cap of runs. */
function startRun(state: LearnerState, command: Extract<Command, { type: "start-run" }>, context: LearnerContext): Decision {
  if (state.identity === undefined) return { rejection: "unknown-learner" };
  if (!Object.hasOwn(context.quizzes, command.quiz)) return { rejection: "unknown-quiz" };
  const existing = runOf(state, command.run);
  if (existing) return { rejection: existing.status === "open" ? "run-open" : "run-closed" };
  const open = state.runs.find((run) => run.quiz === command.quiz && run.status === "open");
  if (open && !stale(open, context)) return { rejection: "run-open" };
  const submitted = state.runs.filter((run) => run.status === "submitted");
  if (submitted.length >= context.limits.runs || submitted.filter((run) => run.quiz === command.quiz).length >= context.limits.runsPerQuiz) return { rejection: "runs-exhausted" };
  const started: Event = { type: "run-started", learner: state.learner, run: command.run, quiz: command.quiz, revision: context.quizzes[command.quiz]!.revision, seed: runSeed(command.run), at: context.now };
  return { events: open ? [{ type: "run-voided", learner: state.learner, run: open.run, at: context.now }, started] : [started] };
}

/** ✍️ Records the latest answer to a task of an open, current run; refused once the run recorded its cap of answers. */
function recordAnswer(state: LearnerState, command: Extract<Command, { type: "record-answer" }>, context: LearnerContext): Decision {
  const run = runOf(state, command.run);
  if (!run) return { rejection: "unknown-run" };
  if (run.status !== "open") return { rejection: "run-closed" };
  if (stale(run, context)) return { rejection: "quiz-revised" };
  if (run.recorded >= context.limits.answersPerRun) return { rejection: "answers-exhausted" };
  const sheetTask = sheetOf(context.quizzes[run.quiz]!.quiz, run.seed).tasks.find((task) => task.id === command.task);
  if (!sheetTask) return { rejection: "unknown-task" };
  const rejection = answerRejection(sheetTask, command.answer);
  if (rejection) return { rejection };
  return { events: [{ type: "answer-recorded", learner: state.learner, run: run.run, task: command.task, answer: command.answer, at: context.now }] };
}

/** 📨️ Scores a complete open run and awards the badges it newly earns; a stale run is voided instead. */
function submitRun(state: LearnerState, command: Extract<Command, { type: "submit-run" }>, context: LearnerContext): Decision {
  const run = runOf(state, command.run);
  if (!run) return { rejection: "unknown-run" };
  if (run.status !== "open") return { rejection: "run-closed" };
  if (stale(run, context)) return { events: [{ type: "run-voided", learner: state.learner, run: run.run, at: context.now }] };
  const quiz = context.quizzes[run.quiz]!.quiz;
  const sheet = sheetOf(quiz, run.seed);
  if (!sheet.tasks.every((task) => answerComplete(task, Object.hasOwn(run.answers, task.id) ? run.answers[task.id] : undefined))) return { rejection: "run-incomplete" };
  const result = scoreRun(quiz, sheet, run.answers);
  if (!result) return { rejection: "run-incomplete" };
  const results = [...state.runs.flatMap((candidate) => (candidate.result ? [candidate.result] : [])), result];
  const quizzes = Object.values(context.quizzes).map((loaded) => loaded.quiz);
  const badges = earnedBadges(
    context.catalog.badges,
    quizzes,
    results,
    state.badges.map((award) => award.badge),
  );
  return {
    events: [{ type: "run-submitted", learner: state.learner, run: run.run, result, at: context.now }, ...badges.map((badge): Event => ({ type: "badge-awarded", learner: state.learner, badge, run: run.run, at: context.now }))],
  };
}

/** 🧑‍⚖️ Decides a command of this learner: the registration of an anonymous learner, start-run, record-answer or submit-run. A command of malformed ids is `id-invalid`, a command of another learner `unknown-learner`. */
export function decideLearner(state: LearnerState, command: Command, context: LearnerContext): Decision {
  const malformed = commandRejection(command);
  if (malformed) return { rejection: malformed };
  if (command.learner !== state.learner) return { rejection: "unknown-learner" };
  switch (command.type) {
    case "identify-learner":
      return register(state, command, context);
    case "start-run":
      return startRun(state, command, context);
    case "record-answer":
      return recordAnswer(state, command, context);
    case "submit-run":
      return submitRun(state, command, context);
  }
}

/** 🔧️ Replaces the run with the given id. */
function withRun(state: LearnerState, run: string, change: (run: RunState) => RunState): readonly RunState[] {
  return state.runs.map((candidate) => (candidate.run === run ? change(candidate) : candidate));
}

/** 🧬️ Folds one event of this learner into the state; events of other learners leave it unchanged. */
export function evolveLearner(state: LearnerState, event: Event): LearnerState {
  if (event.learner !== state.learner) return state;
  switch (event.type) {
    case "learner-registered":
      return { ...state, identity: event.identity };
    case "run-started":
      return { ...state, runs: [...state.runs, { run: event.run, quiz: event.quiz, revision: event.revision, seed: event.seed, status: "open", answers: {}, recorded: 0, startedAt: event.at }] };
    case "run-voided":
      return { ...state, runs: withRun(state, event.run, (run) => ({ ...run, status: "voided" })) };
    case "answer-recorded":
      return { ...state, runs: withRun(state, event.run, (run) => ({ ...run, answers: { ...run.answers, [event.task]: event.answer }, recorded: run.recorded + 1 })) };
    case "run-submitted":
      return { ...state, runs: withRun(state, event.run, (run) => ({ ...run, status: "submitted", result: event.result, submittedAt: event.at })) };
    case "badge-awarded":
      return { ...state, badges: [...state.badges, { badge: event.badge, run: event.run, at: event.at }] };
  }
}
