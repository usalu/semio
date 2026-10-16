/** 🧮️ The pure client state of the quiz session: steps and trail, learner, notices, boards, held leaderboards, the events that move them and the questions asked of a state. */

import {
  acted,
  type Answer,
  type CatalogView,
  type Challenge,
  type CrowdView,
  type Hint,
  type Id,
  type Identity,
  type Leaderboard,
  type LeaderboardPeriod,
  type LearnerView,
  type Rejection,
  type RunResult,
  type RunSummary,
  type RunView,
  type Slug,
} from "@semio-tech/quiz";

//#region 🧭️State
/** 📄️ The pages of home besides the quizzes' own pages (a quiz's page is its id): each is a card of the overview and
 * the page behind it, addressed by hash (`#board`). */
export const HOME_PAGES = { learner: "learner", introduction: "intro", leaderboard: "board", badges: "badges", preferences: "prefs" } as const;

/** 🪧️ The screen the learner is on (ephemeral local-only); on home, `page` is the page opened over the overview — one
 * of {@link HOME_PAGES} or a quiz id. */
export type QuizStep = { readonly screen: "introduction" } | { readonly screen: "identity" } | { readonly screen: "home"; readonly page?: string } | { readonly screen: "run"; readonly run: Id } | { readonly screen: "results"; readonly run: Id };

/** 🧵️ The trail around the step in front (ephemeral local-only, like the step): the steps the learner came through
 * (`back`) and those a way back left ahead (`forward`), the nearest last in both. */
export interface QuizTrail {
  readonly back: readonly QuizStep[];
  readonly forward: readonly QuizStep[];
}

/** 🪡️ The trail of a learner who has not gone anywhere yet. */
export const EMPTY_TRAIL: QuizTrail = { back: [], forward: [] };

/** 📏️ How many steps a trail keeps behind the one in front; older ones are forgotten. */
export const TRAIL_LIMIT = 50;

/** 🧑‍🎓️ The learner this device acts as; the identity is known once registered here or loaded from the proctor. */
export interface QuizLearner {
  readonly id: Id;
  readonly identity?: Identity;
}

/** 📣️ A message the learner should read once: a rejection, a refusal, a voided run, or — `recalled` — that the proctor
 * already knew the pseudonym or name this device registered by itself, so the learner continues as its holder. */
export type QuizNotice = { readonly kind: "rejection"; readonly rejection: Rejection } | { readonly kind: "refused"; readonly detail: string } | { readonly kind: "voided" } | { readonly kind: "recalled"; readonly handle: string };

/** 🗂️ Which of the leaderboards the learner looks at: its period, and the one quiz it counts when it names one. */
export interface BoardChoice {
  readonly period: LeaderboardPeriod;
  readonly quiz?: Slug;
}

/** 🌐️ The leaderboard a learner sees first: every run of every quiz. */
export const DEFAULT_BOARD: BoardChoice = { period: "all-time" };

/** 🔑️ What names one leaderboard among those the client holds: its period, then its quiz. */
export function boardKey(board: BoardChoice): string {
  return board.quiz === undefined ? board.period : `${board.period}/${board.quiz}`;
}

/** 🏆️ One leaderboard as the client holds it: the last answer of the proctor and when it last changed here — or,
 * `local`, the standing of this device alone, made up while the proctor could not be asked. */
export interface HeldLeaderboard {
  readonly board: Leaderboard;
  readonly at: number;
  readonly local?: true;
}

/** 🖼️ Everything the client renders. `board` is the leaderboard the learner chose to look at (ephemeral local-only);
 * `leaderboards` holds, by {@link boardKey}, the last standings the proctor sent for every leaderboard looked at so far
 * and when they last changed here — so a leaderboard chosen again shows at once while it is asked for anew, and an
 * answer equal to the one held changes nothing, so polling renders nothing while the standings stand. `submissions`
 * is how many runs the last answer said were submitted in the whole catalog, whatever leaderboard it was: it moves
 * with every submission, which is when what the others answered is worth asking for again.
 * `asked` names the quizzes whose crowd the learner asked to see before submitting; the ask ends with the run. */
export interface QuizState {
  readonly step: QuizStep;
  readonly trail: QuizTrail;
  readonly introduced: boolean;
  readonly learner?: QuizLearner;
  readonly catalog?: CatalogView;
  readonly learnerView?: LearnerView;
  readonly runs: Readonly<Record<Id, RunView>>;
  readonly awards: Readonly<Record<Id, readonly Slug[]>>;
  readonly board: BoardChoice;
  readonly leaderboards: Readonly<Record<string, HeldLeaderboard>>;
  readonly submissions?: number;
  readonly crowds: Readonly<Record<Slug, CrowdView>>;
  readonly asked: readonly Slug[];
  readonly notice?: QuizNotice;
}

/** ⚡️ A fact of the client session. A step opened `instead` takes the place of the one in front and leaves nothing
 * behind; a step retraced is the one the trail leads `to` on that side. A learner `recalled` is the same person under
 * the id the proctor knows them by: everything held moves `from` the one id `to` the other. A leaderboard loaded
 * `local` is the device's own standing, not an answer of the proctor. */
export type QuizClientEvent =
  | { readonly type: "introduction-read" }
  | { readonly type: "step-opened"; readonly step: QuizStep; readonly instead?: boolean }
  | { readonly type: "step-retraced"; readonly to: keyof QuizTrail }
  | { readonly type: "catalog-loaded"; readonly catalog: CatalogView }
  | { readonly type: "learner-identified"; readonly learner: Id; readonly identity?: Identity }
  | { readonly type: "learner-recalled"; readonly from: Id; readonly to: Id; readonly identity: Identity }
  | { readonly type: "learner-forgotten" }
  | { readonly type: "learner-loaded"; readonly view: LearnerView }
  | { readonly type: "run-loaded"; readonly view: RunView }
  | { readonly type: "answer-given"; readonly run: Id; readonly task: Slug; readonly answer: Answer }
  | { readonly type: "task-opened"; readonly run: Id; readonly task: Slug; readonly at: number }
  | { readonly type: "run-hinted"; readonly run: Id; readonly hints: Readonly<Record<Slug, readonly Hint[]>> }
  | { readonly type: "run-submitted"; readonly run: Id; readonly result: RunResult; readonly badges: readonly Slug[]; readonly at: number }
  | { readonly type: "run-voided"; readonly run: Id }
  | { readonly type: "board-chosen"; readonly board: BoardChoice }
  | { readonly type: "leaderboard-loaded"; readonly leaderboard: Leaderboard; readonly at: number; readonly local?: true }
  | { readonly type: "crowd-loaded"; readonly crowd: CrowdView }
  | { readonly type: "crowd-asked"; readonly quiz: Slug }
  | { readonly type: "crowd-unasked"; readonly quiz: Slug }
  | { readonly type: "notice-raised"; readonly notice: QuizNotice }
  | { readonly type: "notice-dismissed" };

/** 🌱️ The state a device starts with, from its persisted local-only slices. */
export function initialQuizState(persisted: Pick<QuizState, "introduced" | "learner" | "catalog" | "learnerView" | "runs">): QuizState {
  const step: QuizStep = !persisted.introduced ? { screen: "introduction" } : persisted.learner === undefined ? { screen: "identity" } : { screen: "home" };
  return { ...persisted, step, trail: EMPTY_TRAIL, awards: {}, board: DEFAULT_BOARD, leaderboards: {}, crowds: {}, asked: [] };
}

/** 👁️ The leaderboard the learner looks at, as far as the proctor has answered it. */
export function shownLeaderboard(state: Pick<QuizState, "board" | "leaderboards">): HeldLeaderboard | undefined {
  return state.leaderboards[boardKey(state.board)];
}

/** 🥇️ The all-time leaderboard of every quiz — where a learner's rank is its rank overall — as far as the proctor has
 * answered it. */
export function overallLeaderboard(state: Pick<QuizState, "leaderboards">): HeldLeaderboard | undefined {
  return state.leaderboards[boardKey(DEFAULT_BOARD)];
}

/** 🚧️ `board` as far as `catalog` allows it: a quiz the catalog does not list is no category to look at. */
function allowedBoard(board: BoardChoice, catalog: CatalogView | undefined): BoardChoice {
  return board.quiz === undefined || catalog === undefined || catalog.quizzes.some((quiz) => quiz.id === board.quiz) ? board : { period: board.period };
}

/** 🟰️ Whether two steps show the same place. */
export function sameStep(one: QuizStep, other: QuizStep): boolean {
  if (one.screen === "home") return other.screen === "home" && one.page === other.page;
  if (one.screen === "run" || one.screen === "results") return other.screen === one.screen && one.run === other.run;
  return one.screen === other.screen;
}

/** 📍️ Whether `step` still stands, so the trail may lead to it: the overview and its pages always, a run while it is
 * open and results once their run is submitted — either one also while its view has not arrived yet. */
function stands(state: Pick<QuizState, "runs">, step: QuizStep): boolean {
  if (step.screen === "home") return true;
  if (step.screen !== "run" && step.screen !== "results") return false;
  const status = state.runs[step.run]?.status;
  return status === undefined || status === (step.screen === "run" ? "open" : "submitted");
}

/** 🚶️ The state once `step` is in front: the step left stays behind when it still stands and the steps ahead are
 * forgotten — unless it shows `instead` of the one in front, which then leaves no trace. The step already in front
 * changes nothing. */
function opened(state: QuizState, step: QuizStep, instead = false): QuizState {
  if (sameStep(state.step, step)) return state;
  if (instead) return { ...state, step };
  const back = stands(state, state.step) ? [...state.trail.back, state.step].slice(-TRAIL_LIMIT) : state.trail.back;
  return { ...state, step, trail: { back, forward: [] } };
}

/** 🔙️ The state once the trail was followed `to` one side: in front is the nearest step there that still stands and is
 * not the one in front already, the step left lies on the other side, and the steps passed over are forgotten. Nothing
 * when no step of that side stands. */
function retraced(state: QuizState, to: keyof QuizTrail): QuizState | undefined {
  const steps = state.trail[to];
  const index = steps.findLastIndex((step) => stands(state, step) && !sameStep(step, state.step));
  if (index < 0) return undefined;
  const others = state.trail[to === "back" ? "forward" : "back"];
  const left = stands(state, state.step) ? [...others, state.step] : others;
  const kept = steps.slice(0, index);
  return { ...state, step: steps[index]!, trail: to === "back" ? { back: kept, forward: left } : { back: left, forward: kept } };
}

/** ⏪️ The step a way back along the trail leads to, if any still stands. */
export function stepBefore(state: QuizState): QuizStep | undefined {
  return retraced(state, "back")?.step;
}

/** ⏩️ The step a way forward again leads to, if any still stands. */
export function stepAfter(state: QuizState): QuizStep | undefined {
  return retraced(state, "forward")?.step;
}

/** ⏫️ The place above the step in front: the overview above its pages, the page of its quiz above a run and its
 * results (the overview when the catalog does not list that quiz); nothing above the overview and before it. */
export function stepAbove(state: Pick<QuizState, "step" | "runs" | "catalog">): QuizStep | undefined {
  const { step } = state;
  if (step.screen === "home") return step.page === undefined ? undefined : { screen: "home" };
  if (step.screen !== "run" && step.screen !== "results") return undefined;
  const quiz = state.runs[step.run]?.quiz;
  return quiz !== undefined && state.catalog?.quizzes.some((candidate) => candidate.id === quiz) ? { screen: "home", page: quiz } : { screen: "home" };
}

/** ✂️ `view` without the hints of the tasks `dropped` names (every task when it names none): a hint tells about the
 * answer it was given for and only while the run is open, never about another answer. */
export function unhinted(view: RunView, dropped: (task: Slug) => boolean = () => true): RunView {
  if (view.hints === undefined || !Object.keys(view.hints).some(dropped)) return view;
  const { hints, ...rest } = view;
  const kept = Object.entries(hints).filter(([task]) => !dropped(task));
  return kept.length === 0 ? rest : { ...rest, hints: Object.fromEntries(kept) };
}

/** 🆕️ A proctor's read `loaded` of an open run with the answers — and their hints — of the tasks `fresh` names taken
 * from the `held` view instead: answers the read cannot hold, since the device gave them after it was asked for. */
export function keptFresh(loaded: RunView, held: RunView, fresh: (task: Slug) => boolean): RunView {
  if (loaded.status !== "open" || held.status !== "open" || ![...Object.keys(loaded.answers), ...Object.keys(held.answers)].some(fresh)) return loaded;
  const merged = <T>(read: Readonly<Record<Slug, T>> | undefined, local: Readonly<Record<Slug, T>> | undefined): Record<Slug, T> => ({
    ...Object.fromEntries(Object.entries(read ?? {}).filter(([task]) => !fresh(task))),
    ...Object.fromEntries(Object.entries(local ?? {}).filter(([task]) => fresh(task))),
  });
  const { hints: _, ...rest } = loaded;
  const hints = merged(loaded.hints, held.hints);
  return { ...rest, answers: merged(loaded.answers, held.answers), ...(Object.keys(hints).length === 0 ? {} : { hints }) };
}

/** ⌛️ Whether an answer to `task` of `view` at the instant `at` comes after the task's time is up: its clock started
 * at its opening and allows its sheet task's seconds, as the deciders count them (`time-up`), the device deciding at
 * its own clock (`now = at`). An unopened task is not
 * overdue here — another device may have opened it — and the deciders judge it. */
export function overdue(view: RunView, task: Slug, at: number): boolean {
  const opened = view.opened?.[task];
  const seconds = view.sheet.tasks.find((candidate) => candidate.id === task)?.seconds;
  return opened !== undefined && seconds !== undefined && acted(at, opened, at) - opened > seconds * 1000;
}

/** 🔒️ A cached open run the learner view lists as closed (e.g. submitted on another device): closed here too, without
 * hints; its result arrives with the next run view. */
function closedRun(view: RunView, summary: RunSummary): RunView {
  return { ...unhinted(view), status: summary.status, ...(summary.submittedAt === undefined ? {} : { submittedAt: summary.submittedAt }) };
}

function withRun(state: QuizState, run: Id, change: (view: RunView) => RunView): QuizState {
  const view = state.runs[run];
  return view === undefined ? state : { ...state, runs: { ...state.runs, [run]: change(view) } };
}

/** 🧹️ The asks without the one for the quiz of `run`: a run that closed takes its ask along, so the next run of the
 * quiz starts with the others hidden again. */
function unasked(state: QuizState, run: Id): readonly Slug[] {
  const quiz = state.runs[run]?.quiz;
  return quiz === undefined || !state.asked.includes(quiz) ? state.asked : state.asked.filter((asked) => asked !== quiz);
}

/** 🧮️ Folds one client event into the state. Pure. */
export function evolveQuizState(state: QuizState, event: QuizClientEvent): QuizState {
  switch (event.type) {
    case "introduction-read":
      return { ...state, introduced: true, step: state.learner === undefined ? { screen: "identity" } : { screen: "home" }, trail: EMPTY_TRAIL };
    case "step-opened":
      return opened(state, event.step, event.instead);
    case "step-retraced":
      return retraced(state, event.to) ?? state;
    case "catalog-loaded":
      return { ...state, catalog: event.catalog, board: allowedBoard(state.board, event.catalog) };
    case "learner-identified": {
      const same = state.learner?.id === event.learner;
      return {
        ...state,
        learner: { id: event.learner, identity: event.identity ?? (same ? state.learner?.identity : undefined) },
        learnerView: same ? state.learnerView : undefined,
        runs: same ? state.runs : {},
        awards: same ? state.awards : {},
        asked: same ? state.asked : [],
        leaderboards: same ? state.leaderboards : {},
        step: { screen: "home" },
        trail: EMPTY_TRAIL,
      };
    }
    case "learner-recalled": {
      if (state.learner?.id !== event.from) return state;
      const runs = Object.fromEntries(Object.entries(state.runs).map(([run, view]) => [run, { ...view, learner: event.to }]));
      const learnerView = state.learnerView === undefined ? undefined : { ...state.learnerView, learner: event.to, identity: event.identity };
      return { ...state, learner: { id: event.to, identity: event.identity }, learnerView, runs, leaderboards: {} };
    }
    case "learner-forgotten":
      return { ...state, learner: undefined, learnerView: undefined, runs: {}, awards: {}, asked: [], leaderboards: {}, step: { screen: "identity" }, trail: EMPTY_TRAIL };
    case "learner-loaded": {
      if (state.learner?.id !== event.view.learner) return state;
      const closed = event.view.runs.filter((summary) => summary.status !== "open" && state.runs[summary.run]?.status === "open");
      const runs = closed.length === 0 ? state.runs : { ...state.runs, ...Object.fromEntries(closed.map((summary) => [summary.run, closedRun(state.runs[summary.run]!, summary)])) };
      return { ...state, learnerView: event.view, learner: { id: event.view.learner, identity: event.view.identity }, runs };
    }
    case "run-loaded":
      if (state.learner?.id !== event.view.learner) return state;
      return { ...state, runs: { ...state.runs, [event.view.run]: event.view }, asked: event.view.status === "open" || state.runs[event.view.run]?.status !== "open" ? state.asked : unasked(state, event.view.run) };
    case "answer-given":
      return withRun(state, event.run, (view) => unhinted({ ...view, answers: { ...view.answers, [event.task]: event.answer } }, (task) => task === event.task));
    case "task-opened":
      return withRun(state, event.run, (view) => (view.opened !== undefined && Object.hasOwn(view.opened, event.task) ? view : { ...view, opened: { ...view.opened, [event.task]: event.at } }));
    case "run-hinted":
      return withRun(state, event.run, (view) => (Object.keys(event.hints).length === 0 ? unhinted(view) : { ...view, hints: event.hints }));
    case "run-submitted": {
      const submitted = withRun(state, event.run, (view) => ({ ...unhinted(view), status: "submitted", result: event.result, submittedAt: event.at }));
      return opened({ ...submitted, awards: { ...state.awards, [event.run]: event.badges }, asked: unasked(state, event.run) }, { screen: "results", run: event.run });
    }
    case "run-voided": {
      const voided = { ...withRun(state, event.run, (view) => ({ ...unhinted(view), status: "voided" })), asked: unasked(state, event.run) };
      const onRun = (state.step.screen === "run" || state.step.screen === "results") && state.step.run === event.run;
      return onRun ? opened(voided, { screen: "home" }) : voided;
    }
    case "board-chosen": {
      const board = allowedBoard(event.board, state.catalog);
      return boardKey(board) === boardKey(state.board) ? state : { ...state, board };
    }
    case "leaderboard-loaded": {
      const key = boardKey(event.leaderboard);
      const held = state.leaderboards[key];
      if (event.local && held !== undefined && !held.local) return state;
      const same = held !== undefined && held.local === event.local && JSON.stringify(held.board) === JSON.stringify(event.leaderboard);
      const submissions = event.local ? state.submissions : event.leaderboard.submissions;
      if (same && state.submissions === submissions) return state;
      return { ...state, submissions, leaderboards: same ? state.leaderboards : { ...state.leaderboards, [key]: { board: event.leaderboard, at: event.at, ...(event.local ? { local: true as const } : {}) } } };
    }
    case "crowd-loaded":
      return { ...state, crowds: { ...state.crowds, [event.crowd.quiz]: event.crowd } };
    case "crowd-asked":
      return state.asked.includes(event.quiz) ? state : { ...state, asked: [...state.asked, event.quiz] };
    case "crowd-unasked":
      return state.asked.includes(event.quiz) ? { ...state, asked: state.asked.filter((asked) => asked !== event.quiz) } : state;
    case "notice-raised":
      return { ...state, notice: event.notice };
    case "notice-dismissed":
      return { ...state, notice: undefined };
  }
}

/** 🏅️ The badges `run` earned: announced by its submission or listed in the learner view. */
export function runAwards(state: QuizState, run: Id): readonly Slug[] {
  const listed = (state.learnerView?.badges ?? []).filter((award) => award.run === run).map((award) => award.badge);
  return [...new Set([...(state.awards[run] ?? []), ...listed])];
}

/** 🏃️ The open run of `quiz`, if the learner has one; a run this device already saw closed is not open, whatever a
 * lagging learner view still says. */
export function openRunOf(state: QuizState, quiz: Slug): Id | undefined {
  const closedHere = (run: Id): boolean => state.runs[run] !== undefined && state.runs[run].status !== "open";
  const listed = state.learnerView?.runs.find((summary) => summary.quiz === quiz && summary.status === "open" && !closedHere(summary.run))?.run;
  return listed ?? Object.values(state.runs).find((view) => view.quiz === quiz && view.status === "open" && view.learner === state.learner?.id)?.run;
}

/** 🧗️ The challenge of the open run of `quiz`, if the learner has one: from its held view, else from its listing. */
export function openChallengeOf(state: QuizState, quiz: Slug): Challenge | undefined {
  const open = openRunOf(state, quiz);
  return open === undefined ? undefined : (state.runs[open]?.sheet.challenge ?? state.learnerView?.runs.find((summary) => summary.run === open)?.challenge);
}

/** 🏁️ The latest submitted run of `quiz`, from the learner view or from what this device submitted since. */
export function lastSubmittedRunOf(state: QuizState, quiz: Slug): Id | undefined {
  const listed = state.learnerView?.runs.find((summary) => summary.quiz === quiz && summary.status === "submitted");
  const here = Object.values(state.runs)
    .filter((view) => view.quiz === quiz && view.status === "submitted" && view.learner === state.learner?.id)
    .sort((left, right) => (right.submittedAt ?? 0) - (left.submittedAt ?? 0))[0];
  if (here === undefined) return listed?.run;
  return listed === undefined || (here.submittedAt ?? 0) >= (listed.submittedAt ?? 0) ? here.run : listed.run;
}

/** 🔀️ One run from this tab's copy and another tab's: a closed status (with its result) wins over an open one; while
 * both are open the answers are their union, the other tab's — written last — winning per task, each task keeping the
 * hints of the copy whose answer it keeps, and every task opened in either copy counts as opened at its earliest
 * instant. */
export function mergeRunViews(mine: RunView | undefined, theirs: RunView): RunView {
  if (mine === undefined || theirs.status !== "open") return theirs;
  if (mine.status !== "open") return mine;
  const { hints: _, opened: __, ...rest } = theirs;
  const answers = { ...mine.answers, ...theirs.answers };
  const hints = Object.keys(answers).flatMap((task) => {
    const hinted = (Object.hasOwn(theirs.answers, task) ? theirs : mine).hints?.[task];
    return hinted === undefined ? [] : [[task, hinted] as const];
  });
  const openings = [...Object.entries(mine.opened ?? {}), ...Object.entries(theirs.opened ?? {})].sort(([, left], [, right]) => right - left);
  return {
    ...rest,
    answers,
    ...(mine.opened === undefined && theirs.opened === undefined ? {} : { opened: Object.fromEntries(openings) }),
    ...(hints.length === 0 ? {} : { hints: Object.fromEntries(hints) }),
  };
}
//#endregion 🧭️State
