/** 🧾️ The learner lifecycle as pure deciders: handles and the roster that recalls them, runs, answers, submissions and badges of one learner.
 *
 * `decide*` turns a command into events or a rejection at the decision time `now`; `evolve*` folds one event into the
 * state. The proctor wraps both in framework deciders; idempotency by command id is the framework's.
 *
 * @see ../../README.md — the run lifecycle table
 * @see ./🦀️.rs — the Rust twin
 */
import type { Answer, BadgeAward, Catalog, Event, IdentifyLearnerCommand, Identity, Quiz, Rejection, RunResult, RunStatus, Command, Timestamp } from "../../🧬️schema/🟦️.ts";
import { earnedBadges } from "../🏅️badges/🟦️.ts";
import { runSeed } from "../🎲️randomness/🟦️.ts";
import { sheetOf } from "../🃏️sheet/🟦️.ts";
import { scoreRun } from "../📏️scoring/🟦️.ts";
import { answerComplete, answerRejection, normalizeHandle } from "../✅️validation/🟦️.ts";

/** ⚖️ The outcome of a decision: the events to append, or why the command is rejected. */
export type Decision = { readonly events: readonly Event[] } | { readonly rejection: Rejection };

/** 🗂️ The roster: which learner holds each handle key. */
export type RosterState = { readonly handles: Readonly<Record<string, string>> };

/** 📦️ A quiz as the proctor loaded it, with the content hash of its file. */
export type LoadedQuiz = { readonly quiz: Quiz; readonly revision: string };

/** 🧭️ What a learner decision reads besides the state: the decision time, the catalog and its quizzes by id. */
export type LearnerContext = { readonly now: Timestamp; readonly catalog: Catalog; readonly quizzes: Readonly<Record<string, LoadedQuiz>> };

/** 🏃️ One run of a learner as the learner stream built it. */
export type RunState = {
  readonly run: string;
  readonly quiz: string;
  readonly revision: string;
  readonly seed: number;
  readonly status: RunStatus;
  readonly answers: Readonly<Record<string, Answer>>;
  readonly result?: RunResult;
  readonly startedAt: Timestamp;
  readonly submittedAt?: Timestamp;
};

/** 🧑‍🎓️ One learner: identity once registered, runs in start order, badges in award order and the time of the latest event. */
export type LearnerState = {
  readonly learner: string;
  readonly identity?: Identity;
  readonly runs: readonly RunState[];
  readonly badges: readonly BadgeAward[];
  readonly lastActivity?: Timestamp;
};

/** 🌱️ The roster before any learner registered. */
export function emptyRosterState(): RosterState {
  return { handles: {} };
}

/** 🙋️ Registers anonymous learners and unclaimed handles, recalls claimed ones, rejects invalid handles. */
export function decideRoster(state: RosterState, command: IdentifyLearnerCommand, now: Timestamp): Decision {
  if (command.identity.kind === "anonymous") return { events: [{ type: "learner-registered", learner: command.learner, identity: { kind: "anonymous" }, at: now }] };
  const handle = normalizeHandle(command.identity.handle);
  if (!handle) return { rejection: "handle-invalid" };
  const holder = Object.hasOwn(state.handles, handle.key) ? state.handles[handle.key] : undefined;
  if (holder !== undefined) return { events: [{ type: "learner-recalled", learner: holder, at: now }] };
  return { events: [{ type: "learner-registered", learner: command.learner, identity: { kind: command.identity.kind, handle: handle.display }, at: now }] };
}

/** 📇️ Claims the handle key of every learner registered under a pseudonym or name. */
export function evolveRoster(state: RosterState, event: Event): RosterState {
  if (event.type !== "learner-registered" || event.identity.kind === "anonymous") return state;
  const handle = normalizeHandle(event.identity.handle);
  return handle ? { handles: { ...state.handles, [handle.key]: event.learner } } : state;
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

/** ▶️ Starts a run, voiding an open run of the same quiz whose revision is stale. */
function startRun(state: LearnerState, command: Extract<Command, { type: "start-run" }>, context: LearnerContext): Decision {
  if (state.identity === undefined || command.learner !== state.learner) return { rejection: "unknown-learner" };
  if (!Object.hasOwn(context.quizzes, command.quiz)) return { rejection: "unknown-quiz" };
  const existing = runOf(state, command.run);
  if (existing) return { rejection: existing.status === "open" ? "run-open" : "run-closed" };
  const open = state.runs.find((run) => run.quiz === command.quiz && run.status === "open");
  if (open && !stale(open, context)) return { rejection: "run-open" };
  const started: Event = { type: "run-started", learner: state.learner, run: command.run, quiz: command.quiz, revision: context.quizzes[command.quiz]!.revision, seed: runSeed(command.run), at: context.now };
  return { events: open ? [{ type: "run-voided", learner: state.learner, run: open.run, at: context.now }, started] : [started] };
}

/** ✍️ Records the latest answer to a task of an open, current run. */
function recordAnswer(state: LearnerState, command: Extract<Command, { type: "record-answer" }>, context: LearnerContext): Decision {
  const run = runOf(state, command.run);
  if (!run) return { rejection: "unknown-run" };
  if (run.status !== "open") return { rejection: "run-closed" };
  if (stale(run, context)) return { rejection: "quiz-revised" };
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
  const results =[...state.runs.flatMap((candidate) => (candidate.result ? [candidate.result] : [])), result];
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

/** 🧑‍⚖️ Decides a start-run, record-answer or submit-run command of this learner. */
export function decideLearner(state: LearnerState, command: Exclude<Command, IdentifyLearnerCommand>, context: LearnerContext): Decision {
  switch (command.type) {
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
  const touched = { ...state, lastActivity: Math.max(state.lastActivity ?? event.at, event.at) };
  switch (event.type) {
    case "learner-registered":
      return { ...touched, identity: event.identity };
    case "learner-recalled":
      return touched;
    case "run-started":
      return { ...touched, runs: [...state.runs, { run: event.run, quiz: event.quiz, revision: event.revision, seed: event.seed, status: "open", answers: {}, startedAt: event.at }] };
    case "run-voided":
      return { ...touched, runs: withRun(state, event.run, (run) => ({ ...run, status: "voided" })) };
    case "answer-recorded":
      return { ...touched, runs: withRun(state, event.run, (run) => ({ ...run, answers: { ...run.answers, [event.task]: event.answer } })) };
    case "run-submitted":
      return { ...touched, runs: withRun(state, event.run, (run) => ({ ...run, status: "submitted", result: event.result, submittedAt: event.at })) };
    case "badge-awarded":
      return { ...touched, badges: [...state.badges, { badge: event.badge, run: event.run, at: event.at }] };
  }
}
