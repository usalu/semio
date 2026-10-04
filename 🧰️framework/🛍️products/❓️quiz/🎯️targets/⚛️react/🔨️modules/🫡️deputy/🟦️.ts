/** 🫡️ The proctor's deputy on the device: with the material a site hands in — the catalog and its quizzes, solutions
 * included — the quiz core's own deciders run here, so a learner registers, starts runs, submits them and earns badges
 * while the proctor cannot be reached.
 *
 * The deputy keeps nothing. The learner it decides for is derived from the views the client holds anyway (the learner
 * view and the run views, {@link Deputy.state}), a decision is the core's ({@link Deputy.decide}), and its events fold
 * back into views ({@link Deputy.learnerView}, {@link Deputy.runView}). What it decides is provisional: the command
 * still goes to the proctor, which decides it again with the same deciders and whose views replace the deputy's.
 * Where the views hold less than the proctor's stream does, the deputy decides with what there is: a run known only
 * from the learner view counts at its challenge with its score, its points and no task results, and a handle is free
 * unless the device itself holds it — the proctor settles both when it hears of them. A run's challenge and the openings
 * of its tasks come from its run view, and its held sheet must be the one the material deals at that challenge; the
 * session never has the deputy open a task of or submit a run it knows from its listing alone. An answer is never the
 * deputy's to decide — the session records it and the proctor decides it at delivery — so the count of recorded answers
 * (the proctor's cap) is rebuilt only as far as the views tell it, the answers held, and decides nothing here.
 *
 * @see ../../../../🔨️modules/🧾️lifecycle/🟦️.ts — `decideLearner`, `decideHandle`, `evolveLearner`
 * @see ../../../../🔨️modules/👁️views/🟦️.ts — `learnerView`, `runView`, `leaderboard`
 * @see ../🧭️session/🟦️.ts — where a command goes to the deputy
 */

import {
  DEFAULT_LIMITS,
  catalogView,
  decideHandle,
  decideLearner,
  emptyHandleState,
  fnv1a32,
  leaderboard,
  learnerView,
  normalizeHandle,
  points,
  runSeed,
  runView,
  sheetOf,
  transcript,
  type Catalog,
  type CatalogView,
  type Command,
  type Decision,
  type HandleView,
  type Hint,
  type Id,
  type Identity,
  type Leaderboard,
  type LeaderboardPeriod,
  type LearnerState,
  type LearnerView,
  type Limits,
  type LoadedQuiz,
  type Quiz,
  type RunState,
  type RunSummary,
  type RunView,
  type Slug,
  type Timestamp,
} from "@semio-tech/quiz";

/** 📚️ What a site hands the client so that the device can decide by itself: the catalog and the quizzes it names, in
 * catalog order, with their solutions. */
export interface QuizMaterial {
  readonly catalog: Catalog;
  readonly quizzes: readonly Quiz[];
}

/** 🗃️ What the client holds of its learner: who it is, and the views of it and of its runs. */
export interface HeldLearner {
  readonly learner: Id;
  readonly identity?: Identity;
  readonly view?: LearnerView;
  readonly runs: Readonly<Record<Id, RunView>>;
}

/** 🕰️ The revision of a run whose held sheet is not the one the material deals for its seed: the quiz was revised since. */
export const REVISED = "revised";

/** 🔖️ The deputy's name for a quiz as the material holds it: FNV-1a of its JSON as 8 lowercase hex digits. It never
 * leaves the device — the proctor names its own revisions. */
export function materialRevision(quiz: Quiz): string {
  return fnv1a32(JSON.stringify(quiz)).toString(16).padStart(8, "0");
}

/** 🟰️ Whether two JSON values say the same, whatever the order of their keys: a view the proctor sent and one the core
 * built here differ in nothing else. */
function alike(left: unknown, right: unknown): boolean {
  if (left === right) return true;
  if (typeof left !== "object" || typeof right !== "object" || left === null || right === null || Array.isArray(left) !== Array.isArray(right)) return false;
  const [one, other] = [left as Readonly<Record<string, unknown>>, right as Readonly<Record<string, unknown>>];
  const keys = Object.keys(one);
  return keys.length === Object.keys(other).length && keys.every((key) => Object.hasOwn(other, key) && alike(one[key], other[key]));
}

/** 🫡️ The deciders and views of the quiz core over one site's material. */
export class Deputy {
  /** 📖️ The solution-free catalog of the material: what the client shows until the proctor's own arrives. */
  readonly catalog: CatalogView;
  private readonly material: Catalog;
  private readonly quizzes: Readonly<Record<Slug, LoadedQuiz>>;
  private readonly limits: Limits;

  constructor(material: QuizMaterial, limits: Limits = DEFAULT_LIMITS) {
    this.material = material.catalog;
    this.catalog = catalogView(material.catalog, material.quizzes);
    this.quizzes = Object.fromEntries(material.quizzes.map((quiz) => [quiz.id, { quiz, revision: materialRevision(quiz) }]));
    this.limits = limits;
  }

  /** 🧑‍🎓️ The learner as the held views describe it: every run the learner view lists or a run view holds, in start
   * order — a closed status wins over an open one, a run without a run view counts with the score of its listing —
   * the badges and the identity of the learner view, else the identity the device knows. */
  state(held: HeldLearner): LearnerState {
    const listed = held.view?.learner === held.learner ? held.view : undefined;
    const summaries = new Map((listed?.runs ?? []).map((summary) => [summary.run, summary]));
    const views = Object.values(held.runs).filter((view) => view.learner === held.learner);
    const runs = [...new Set([...summaries.keys(), ...views.map((view) => view.run)])]
      .map((run) => this.run(run, summaries.get(run), views.find((view) => view.run === run)))
      .sort((left, right) => left.startedAt - right.startedAt || (left.run < right.run ? -1 : 1));
    const identity = listed?.identity ?? held.identity;
    return { learner: held.learner, ...(identity === undefined ? {} : { identity }), runs, badges: listed?.badges ?? [] };
  }

  /** ⚖️ Decides `command` at `now` as the proctor would: a registration under a pseudonym or name against a handle
   * nobody holds, everything else against `state`. */
  decide(state: LearnerState, command: Command, now: Timestamp): Decision {
    if (command.type !== "identify-learner" || command.identity.kind === "anonymous") return decideLearner(state, command, { now, catalog: this.material, quizzes: this.quizzes, limits: this.limits });
    const handle = normalizeHandle(command.identity.handle);
    return handle === undefined ? { rejection: "handle-invalid" } : decideHandle(emptyHandleState(handle.key), command, now);
  }

  /** 👤️ The learner view of `state`; nothing before the learner is registered. */
  learnerView(state: LearnerState): LearnerView | undefined {
    return learnerView(state, this.catalog);
  }

  /** 🏃️ The view of one run of `state` with the sheet the material deals for its seed. */
  runView(state: LearnerState, run: Id): RunView | undefined {
    return runView(state, run, this.quizzes);
  }

  /** 🔦️ What the device can say about `handle`: its normalized display (the handle as typed when it is outside the
   * policy, which the registration then refuses) and no holder. */
  handle(handle: string): HandleView {
    return { display: normalizeHandle(handle)?.display ?? handle };
  }

  /** 🏆️ The leaderboard of `board` at `now` as far as this device knows: the standing of `state` alone. */
  leaderboard(state: LearnerState, board: { readonly period: LeaderboardPeriod; readonly quiz?: Slug }, now: Timestamp): Leaderboard {
    const own = transcript(state);
    return leaderboard(own === undefined ? [] : [own], this.catalog, board, now, state.learner);
  }

  /** 💡️ The hints of `run` of `state` as the proctor would give them (none on a closed run or a challenge that does
   * not hint); nothing when the material does not hold its quiz or its held sheet is not the one the material deals. */
  hints(state: LearnerState, run: Id): Readonly<Record<Slug, readonly Hint[]>> | undefined {
    const found = state.runs.find((candidate) => candidate.run === run);
    if (found === undefined || found.revision === REVISED) return undefined;
    return runView(state, run, this.quizzes)?.hints ?? {};
  }

  private run(run: Id, summary: RunSummary | undefined, view: RunView | undefined): RunState {
    const known = (view ?? summary)!;
    const challenge = view?.sheet.challenge ?? summary!.challenge;
    const status = view === undefined || (view.status === "open" && summary !== undefined) ? summary!.status : view.status;
    const seed = view?.sheet.seed ?? runSeed(run);
    const loaded = Object.hasOwn(this.quizzes, known.quiz) ? this.quizzes[known.quiz] : undefined;
    const current = loaded !== undefined && (view === undefined || status !== "open" || alike(view.sheet, sheetOf(loaded.quiz, seed, challenge)));
    const result = view?.result ?? (summary?.score === undefined ? undefined : { quiz: known.quiz, challenge, score: summary.score, points: summary.points ?? points(summary.score, challenge), tasks: [] });
    const submittedAt = view?.submittedAt ?? summary?.submittedAt;
    const answers = view?.answers ?? {};
    return {
      run,
      quiz: known.quiz,
      challenge,
      revision: current ? loaded.revision : REVISED,
      seed,
      status,
      answers,
      recorded: Object.keys(answers).length,
      opened: view?.opened ?? {},
      ...(result === undefined ? {} : { result }),
      startedAt: known.startedAt,
      ...(submittedAt === undefined ? {} : { submittedAt }),
    };
  }
}
