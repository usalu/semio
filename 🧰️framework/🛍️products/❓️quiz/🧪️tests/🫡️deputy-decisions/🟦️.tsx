/** 🫡️ The deputy decides like the proctor from nothing but the views a client holds: every committed sequence of the
 * learner-lifecycle vectors (written by the Python reference) is replayed twice — through the core's deciders over the
 * exact learner stream, as a proctor does, and through the {@link Deputy} over the learner view and the run views alone
 * — and both give the same verdicts, the same events and the same views after every step. A registration under a
 * pseudonym or name is the one command the two decide apart, each rightly: a learner stream refuses it (it belongs to
 * the stream of its handle), the deputy stands in for that stream too. Where a view holds less than the stream the
 * deputy is told apart on purpose: a revised quiz shows in a held sheet (also one dealt at another challenge than the
 * run's), a run without a run view counts at the challenge and with the score and points of its listing, a handle is
 * free unless the proctor says otherwise. The challenge of a run and the openings of its tasks come from its run view,
 * so the deputy opens tasks and keeps the clock like the proctor, and it tells an easy run's hints as the proctor's view
 * does.
 *
 * @see ../../🧫️fixtures/🧾️learner-lifecycle/🔣️.json — the vectors
 * @see ../../🎯️targets/⚛️react/🔨️modules/🫡️deputy/🟦️.ts
 */

import { describe, expect, it } from "vitest";
import {
  CHALLENGES,
  catalogView,
  decideLearner,
  emptyLearnerState,
  evolveLearner,
  learnerView,
  runView,
  type Catalog,
  type Command,
  type Decision,
  type Event,
  type LearnerState,
  type Limits,
  type LoadedQuiz,
  type Quiz,
  type RunView,
} from "@semio-tech/quiz";
import { Deputy, REVISED, materialRevision, type HeldLearner } from "@semio-tech/quiz-react";
import vectors from "../../🧫️fixtures/🧾️learner-lifecycle/🔣️.json";

interface Step {
  readonly command: Command;
  readonly now: number;
  readonly revisions?: Readonly<Record<string, string>>;
}

interface Sequence {
  readonly id: string;
  readonly learner: string;
  readonly given: readonly Event[];
  readonly limits?: Limits;
  readonly steps: readonly Step[];
}

const CATALOG = vectors.catalog as unknown as Catalog;
const QUIZZES = vectors.quizzes as unknown as readonly Quiz[];
const LOADED: Readonly<Record<string, LoadedQuiz>> = Object.fromEntries(QUIZZES.map((quiz) => [quiz.id, { quiz, revision: (vectors.revisions as Readonly<Record<string, string>>)[quiz.id]! }]));
const SEQUENCES = vectors.learners as unknown as readonly Sequence[];

/** 🧪️ The sequences a client can follow from its views: those that revise no quiz mid-way (a proctor's matter) and
 * decide under the default caps (the cap of recorded answers counts the replaced ones, which no view carries). */
const FOLLOWED = SEQUENCES.filter((sequence) => sequence.limits === undefined && sequence.steps.every((step) => step.revisions === undefined));

/** 🏷️ A decision with the revisions of its events left out: the deputy names its material itself. */
function unrevised(decision: Decision): unknown {
  return "rejection" in decision ? decision : { events: decision.events.map((event) => (event.type === "run-started" ? { ...event, revision: "" } : event)) };
}

/** 🗃️ What a client holds of `state`: the learner view and the view of every run. */
function heldOf(state: LearnerState): HeldLearner {
  const view = learnerView(state, catalogView(CATALOG, QUIZZES));
  const runs = Object.fromEntries(state.runs.flatMap((run) => [runView(state, run.run, LOADED)].flatMap((found) => (found === undefined ? [] : [[run.run, found] as const]))));
  return { learner: state.learner, ...(state.identity === undefined ? {} : { identity: state.identity }), ...(view === undefined ? {} : { view }), runs };
}

describe("🫡️ deputy decisions", () => {
  it("follows the sequences of the lifecycle vectors that a client can follow", () => {
    expect(FOLLOWED.map((sequence) => sequence.id)).toEqual(expect.arrayContaining(["happy-path", "unknown-learner", "rejections-and-latest-answer", "every-badge", "anonymous-registration", "hints-on-easy", "points-by-challenge", "the-limit-of-a-sorting-and-a-matching", "clock-ahead-late", "clock-behind-late"]));    const kinds = new Set(FOLLOWED.flatMap((sequence) => sequence.steps.map((step) => step.command.type)));
    expect([...kinds].sort()).toEqual(["identify-learner", "open-task", "record-answer", "start-run", "submit-run"]);
    expect(new Set(FOLLOWED.flatMap((sequence) => sequence.steps.flatMap((step) => (step.command.type === "start-run" ? [step.command.challenge] : []))))).toEqual(new Set(CHALLENGES));
  });

  for (const sequence of FOLLOWED) {
    it(`decides ${sequence.id} from the held views as the core decides it from the stream`, () => {
      const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
      let exact = sequence.given.reduce(evolveLearner, emptyLearnerState(sequence.learner));
      let held = heldOf(exact);
      for (const [index, step] of sequence.steps.entries()) {
        if (step.command.type === "identify-learner" && step.command.identity.kind !== "anonymous") continue;
        const expected = decideLearner(exact, step.command, { now: step.now, catalog: CATALOG, quizzes: LOADED, limits: vectors.limits });
        const before = deputy.state(held);
        const decided = deputy.decide(before, step.command, step.now);
        expect(unrevised(decided), `step ${index}: ${step.command.type}`).toEqual(unrevised(expected));
        if ("rejection" in expected || "rejection" in decided) continue;
        exact = expected.events.reduce(evolveLearner, exact);
        const after = decided.events.reduce(evolveLearner, before);
        const runs: Record<string, RunView> = { ...held.runs };
        for (const run of after.runs) runs[run.run] = deputy.runView(after, run.run)!;
        const view = deputy.learnerView(after);
        held = { learner: sequence.learner, ...(after.identity === undefined ? {} : { identity: after.identity }), ...(view === undefined ? {} : { view }), runs };
        expect(held, `step ${index}: the views`).toEqual(heldOf(exact));
      }
    });
  }

  it("shows the material's catalog without solutions, paths or badge rules", () => {
    const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
    expect(deputy.catalog).toEqual(catalogView(CATALOG, QUIZZES));
    for (const withheld of ['"rule"', '"value"', '"values"', '"category"', '"items"']) expect(JSON.stringify(deputy.catalog)).not.toContain(withheld);
  });

  it("sees a revised quiz in a held sheet, whatever the order of its keys, and voids the run instead of scoring it", () => {
    const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
    const learner = "a".repeat(32);
    const run = "b".repeat(32);
    const quiz = QUIZZES[0]!;
    const registered = evolveLearner(emptyLearnerState(learner), { type: "learner-registered", learner, identity: { kind: "anonymous" }, at: 1 });
    const started = deputy.decide(registered, { type: "start-run", id: "c".repeat(32), learner, run, quiz: quiz.id, challenge: "medium", at: 2 }, 2);
    if ("rejection" in started) throw new Error(started.rejection);
    expect(started.events).toMatchObject([{ type: "run-started", revision: materialRevision(quiz) }]);
    const held = heldOf(started.events.reduce(evolveLearner, registered));
    const view = deputy.runView(deputy.state(held), run)!;
    const reordered = JSON.parse(JSON.stringify(view, (_, value: unknown) => (typeof value === "object" && value !== null && !Array.isArray(value) ? Object.fromEntries(Object.entries(value).reverse()) : value))) as RunView;
    expect(Object.keys(reordered)).toEqual(Object.keys(view).reverse());
    expect(deputy.state({ ...held, runs: { [run]: reordered } }).runs[0]?.revision).toBe(materialRevision(quiz));
    const revised: RunView = { ...view, sheet: { ...view.sheet, tasks: view.sheet.tasks.slice(1) } };
    const stale = deputy.state({ ...held, runs: { [run]: revised } });
    expect(stale.runs[0]?.revision).toBe(REVISED);
    expect(deputy.decide(stale, { type: "submit-run", id: "d".repeat(32), learner, run }, 3)).toEqual({ events: [{ type: "run-voided", learner, run, at: 3 }] });
  });

  it("counts a run it holds no view of with the score of its listing, and a closed listing over an open view", () => {
    const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
    const sequence = SEQUENCES.find((candidate) => candidate.id === "happy-path")!;
    let exact = sequence.given.reduce(evolveLearner, emptyLearnerState(sequence.learner));
    for (const step of sequence.steps) {
      const decision = decideLearner(exact, step.command, { now: step.now, catalog: CATALOG, quizzes: LOADED, limits: vectors.limits });
      if ("events" in decision) exact = decision.events.reduce(evolveLearner, exact);
    }
    const held = heldOf(exact);
    const listed = deputy.state({ ...held, runs: {} });
    expect(deputy.learnerView(listed)).toEqual(held.view);
    expect(listed.runs.every((run) => run.result?.tasks.length === 0)).toBe(true);
    const submitted = exact.runs.find((run) => run.status === "submitted")!;
    const open: RunView = { ...held.runs[submitted.run]!, status: "open" };
    delete (open as { result?: unknown }).result;
    expect(deputy.state({ ...held, runs: { [submitted.run]: open } }).runs.find((run) => run.run === submitted.run)).toMatchObject({ status: "submitted", result: { score: submitted.result!.score } });
  });

  it("takes a run's challenge and the openings of its tasks from its run view and sees a sheet dealt at another challenge as revised", () => {
    const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
    const learner = "a".repeat(32);
    const run = "b".repeat(32);
    const quiz = QUIZZES[0]!;
    const registered = evolveLearner(emptyLearnerState(learner), { type: "learner-registered", learner, identity: { kind: "anonymous" }, at: 1 });
    const started = deputy.decide(registered, { type: "start-run", id: "c".repeat(32), learner, run, quiz: quiz.id, challenge: "expert", at: 1_000 }, 1_000);
    if ("rejection" in started) throw new Error(started.rejection);
    expect(started.events).toMatchObject([{ type: "run-started", challenge: "expert" }]);
    const begun = started.events.reduce(evolveLearner, registered);
    const task = deputy.runView(begun, run)!.sheet.tasks[0]!;
    expect(task.seconds).toBeGreaterThan(0);
    const opened = deputy.decide(deputy.state(heldOf(begun)), { type: "open-task", id: "d".repeat(32), learner, run, task: task.id, at: 500 }, 5_000);
    expect(opened).toEqual({ events: [{ type: "task-opened", learner, run, task: task.id, at: 1_000 }] });
    if ("rejection" in opened) throw new Error(opened.rejection);
    const held = heldOf(opened.events.reduce(evolveLearner, begun));
    expect(held.runs[run]?.opened).toEqual({ [task.id]: 1_000 });
    const rebuilt = deputy.state(held).runs[0]!;
    expect(rebuilt).toMatchObject({ challenge: "expert", opened: { [task.id]: 1_000 }, revision: materialRevision(quiz) });
    expect(deputy.decide(deputy.state(held), { type: "open-task", id: "e".repeat(32), learner, run, task: task.id, at: 6_000 }, 6_000)).toEqual({ rejection: "already-opened" });
    const late = task.seconds! * 1000 + 1_001;
    expect(deputy.decide(deputy.state(held), { type: "record-answer", id: "f".repeat(32), learner, run, task: task.id, answer: { kind: task.kind } as never, at: late + 1 }, late + 10)).toEqual({ rejection: "time-up" });
    const unopened = deputy.state({ ...held, runs: { [run]: { ...held.runs[run]!, opened: {} } } });
    expect(deputy.decide(unopened, { type: "record-answer", id: "f".repeat(32), learner, run, task: task.id, answer: { kind: task.kind } as never, at: 2_000 }, 2_000)).toEqual({ rejection: "task-unopened" });
    const other = deputy.runView(deputy.state(held), run)!;
    const medium = deputy.decide(registered, { type: "start-run", id: "c".repeat(32), learner, run, quiz: quiz.id, challenge: "medium", at: 1_000 }, 1_000);
    if ("rejection" in medium) throw new Error(medium.rejection);
    const mediumSheet = deputy.runView(medium.events.reduce(evolveLearner, registered), run)!.sheet;
    expect(deputy.state({ ...held, runs: { [run]: { ...other, sheet: { ...mediumSheet, challenge: "expert" } } } }).runs[0]?.revision).toBe(REVISED);
    expect(deputy.decide(deputy.state(heldOf(medium.events.reduce(evolveLearner, registered))), { type: "open-task", id: "9".repeat(32), learner, run, task: task.id, at: 2_000 }, 2_000)).toEqual({ rejection: "run-untimed" });
  });

  it("counts a run it holds no view of at the challenge and with the points of its listing", () => {
    const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
    const sequence = SEQUENCES.find((candidate) => candidate.id === "points-by-challenge")!;
    let exact = sequence.given.reduce(evolveLearner, emptyLearnerState(sequence.learner));
    for (const step of sequence.steps) {
      const decision = decideLearner(exact, step.command, { now: step.now, catalog: CATALOG, quizzes: LOADED, limits: vectors.limits });
      if ("events" in decision) exact = decision.events.reduce(evolveLearner, exact);
    }
    const held = heldOf(exact);
    const listed = deputy.state({ ...held, runs: {} });
    for (const run of exact.runs) {
      const found = listed.runs.find((candidate) => candidate.run === run.run)!;
      expect(found.challenge, run.run).toBe(run.challenge);
      expect(found.opened).toEqual({});
      if (run.result !== undefined) expect(found.result).toEqual({ quiz: run.quiz, challenge: run.challenge, score: run.result.score, points: run.result.points, tasks: [] });
    }
    expect(new Set(exact.runs.filter((run) => run.status === "submitted").map((run) => run.challenge)).size).toBeGreaterThan(1);
    expect(deputy.learnerView(listed)).toEqual(held.view);
  });

  it("tells the hints of an easy run as the proctor's view tells them, none for a run of another challenge, nothing for a revised one", () => {
    const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
    const sequence = SEQUENCES.find((candidate) => candidate.id === "hints-on-easy")!;
    let exact = sequence.given.reduce(evolveLearner, emptyLearnerState(sequence.learner));
    let seen = 0;
    for (const step of sequence.steps) {
      const decision = decideLearner(exact, step.command, { now: step.now, catalog: CATALOG, quizzes: LOADED, limits: vectors.limits });
      if ("events" in decision) exact = decision.events.reduce(evolveLearner, exact);
      const held = heldOf(exact);
      const state = deputy.state(held);
      for (const run of exact.runs) {
        expect(deputy.hints(state, run.run) ?? {}, `${run.run} after ${step.command.type}`).toEqual(held.runs[run.run]?.hints ?? {});
        if (run.challenge !== "easy" || run.status !== "open") expect(deputy.hints(state, run.run)).toEqual({});
        seen += Object.keys(held.runs[run.run]?.hints ?? {}).length;
      }
    }
    expect(seen).toBeGreaterThan(0);
    const easy = exact.runs.find((run) => run.challenge === "easy")!;
    const held = heldOf(exact);
    const view = held.runs[easy.run]!;
    const { view: _, ...unlisted } = held;
    expect(deputy.hints(deputy.state({ ...unlisted, runs: { [easy.run]: { ...view, status: "open", sheet: { ...view.sheet, tasks: view.sheet.tasks.slice(1) } } } }), easy.run)).toBeUndefined();
    expect(deputy.hints(deputy.state({ ...unlisted, runs: { [easy.run]: { ...view, status: "open" } } }), easy.run)).toEqual(runView({ ...deputy.state({ ...unlisted, runs: { [easy.run]: { ...view, status: "open" } } }) }, easy.run, LOADED)?.hints ?? {});
    expect(deputy.hints(deputy.state(held), "0".repeat(32))).toBeUndefined();
  });

  it("registers a pseudonym against a handle nobody holds and refuses one outside the policy", () => {
    const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
    const learner = "a".repeat(32);
    const claim = (handle: string): Command => ({ type: "identify-learner", id: "c".repeat(32), learner, identity: { kind: "pseudonym", handle } });
    expect(deputy.decide(emptyLearnerState(learner), claim("  Ada   L. "), 7)).toEqual({ events: [{ type: "learner-registered", learner, identity: { kind: "pseudonym", handle: "Ada L." }, at: 7 }] });
    expect(deputy.decide(emptyLearnerState(learner), claim("§"), 7)).toEqual({ rejection: "handle-invalid" });
    expect(deputy.handle("  Ada   L. ")).toEqual({ display: "Ada L." });
    expect(deputy.handle("§")).toEqual({ display: "§" });
  });

  it("ranks the device's own standing alone and nobody before a run is submitted", () => {
    const deputy = new Deputy({ catalog: CATALOG, quizzes: QUIZZES });
    const sequence = SEQUENCES.find((candidate) => candidate.id === "every-badge")!;
    const registered = sequence.given.reduce(evolveLearner, emptyLearnerState(sequence.learner));
    expect(deputy.leaderboard(registered, { period: "all-time" }, 1)).toMatchObject({ rows: [], learners: 0, submissions: 0 });
    let exact = registered;
    for (const step of sequence.steps) {
      const decision = decideLearner(exact, step.command, { now: step.now, catalog: CATALOG, quizzes: LOADED, limits: vectors.limits });
      if ("events" in decision) exact = decision.events.reduce(evolveLearner, exact);
    }
    const last = sequence.steps.at(-1)!.now;
    const board = deputy.leaderboard(deputy.state(heldOf(exact)), { period: "all-time" }, last);
    expect(board).toMatchObject({ learners: 1, rows: [{ rank: 1, total: learnerView(exact, deputy.catalog)!.total }] });
    expect(board.own).toEqual(board.rows[0]);
    expect(board.rows[0]?.badges).toEqual(exact.badges.map((award) => award.badge));
  });
});
