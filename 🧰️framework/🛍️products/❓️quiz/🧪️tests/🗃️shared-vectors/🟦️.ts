import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  Mt19937,
  answerComplete,
  answerRejection,
  catalogIssues,
  catalogView,
  decideLearner,
  decideRoster,
  earnedBadges,
  emptyLearnerState,
  emptyRosterState,
  evolveLearner,
  evolveRoster,
  fnv1a32,
  leaderboard,
  learnerView,
  normalizeHandle,
  quizIssues,
  runSeed,
  runView,
  scoreTask,
  sheetOf,
  shuffle,
  uniformIndex,
  crowdView,
  cursorProblem,
  presenceProblem,
  roomScope,
  rosterScope,
  thinkingProblem,
  thinkingScope,
  type Answer,
  type Place,
  type Badge,
  type Catalog,
  type Command,
  type Event,
  type IdentifyLearnerCommand,
  type LearnerState,
  type LoadedQuiz,
  type Quiz,
  type RunResult,
  type Sheet,
  type SheetTask,
  type Task,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

const FIXTURES = resolve(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures");
const TOLERANCE = 1e-12;
const STRUCTURAL = new Set(["type-invalid", "required", "property-unknown", "value-invalid", "slug-invalid", "length-invalid", "integer-invalid", "below-minimum", "items-too-few", "properties-too-few", "duplicate-path"]);

/** 🧫️ The shared vectors of one case, as generated from the Python reference. */
function vectors<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(FIXTURES, name, "🔣️.json"), "utf8")) as T;
}

/** ⚖️ Structural equality with the §6 parity tolerance on numbers: same keys, same order of list entries. */
function expectClose(produced: unknown, committed: unknown, path = ""): void {
  if (typeof produced === "number" && typeof committed === "number") {
    if (Math.abs(produced - committed) > TOLERANCE) throw new Error(`${path}: produced ${produced}, committed ${committed}`);
    return;
  }
  if (Array.isArray(produced) && Array.isArray(committed)) {
    if (produced.length !== committed.length) throw new Error(`${path}: produced ${produced.length} entries, committed ${committed.length}`);
    produced.forEach((entry, index) => expectClose(entry, committed[index], `${path}/${index}`));
    return;
  }
  if (produced && committed && typeof produced === "object" && typeof committed === "object") {
    const left = Object.keys(produced).sort();
    const right = Object.keys(committed).sort();
    if (left.join("\u0000") !== right.join("\u0000")) throw new Error(`${path}: produced keys ${left.join(",")}, committed keys ${right.join(",")}`);
    for (const key of left) expectClose((produced as Record<string, unknown>)[key], (committed as Record<string, unknown>)[key], `${path}/${key}`);
    return;
  }
  if (produced !== committed) throw new Error(`${path}: produced ${JSON.stringify(produced)}, committed ${JSON.stringify(committed)}`);
}

describe("🎲️seeded-randomness", () => {
  const fixture = vectors<{
    hashes: { id: string; text: string; hash: number }[];
    runSeeds: { id: string; run: string; seed: number }[];
    rawOutputs: { id: string; seed: number; skip: number; count: number; outputs: number[] }[];
    uniformDraws: { id: string; seed: number; bounds: number[]; expected: { draws: number[]; next: number } }[];
    shuffles: { id: string; seed: number; length: number; expected: { permutation: number[]; next: number } }[];
  }>("🎲️seeded-randomness");

  it("hashes and seeds every committed text and run id", () => {
    for (const vector of fixture.hashes) expect(fnv1a32(vector.text), vector.id).toBe(vector.hash);
    for (const vector of fixture.runSeeds) expect(runSeed(vector.run), vector.id).toBe(vector.seed);
  });

  it("reproduces every committed output window, draw sequence and shuffle", () => {
    for (const vector of fixture.rawOutputs) {
      const random = new Mt19937(vector.seed);
      for (let i = 0; i < vector.skip; i++) random.next();
      expect(Array.from({ length: vector.count }, () => random.next()), vector.id).toEqual(vector.outputs);
    }
    for (const vector of fixture.uniformDraws) {
      const random = new Mt19937(vector.seed);
      expect(vector.bounds.map((bound) => uniformIndex(random, bound)), vector.id).toEqual(vector.expected.draws);
      expect(random.next(), vector.id).toBe(vector.expected.next);
    }
    for (const vector of fixture.shuffles) {
      const random = new Mt19937(vector.seed);
      expect(shuffle(random, Array.from({ length: vector.length }, (_, index) => index)), vector.id).toEqual(vector.expected.permutation);
      expect(random.next(), vector.id).toBe(vector.expected.next);
    }
  });
});

describe("🃏️sheet-assembly", () => {
  const fixture = vectors<{ quizzes: Quiz[]; sheets: { id: string; quiz: string; seed: number; sheet: Sheet }[] }>("🃏️sheet-assembly");

  it("assembles every committed sheet", () => {
    expect(fixture.sheets.length).toBeGreaterThan(0);
    for (const vector of fixture.sheets) expect(sheetOf(fixture.quizzes.find((quiz) => quiz.id === vector.quiz)!, vector.seed), vector.id).toEqual(vector.sheet);
  });

  it("finds no issue in the committed quizzes", () => {
    for (const quiz of fixture.quizzes) expect(quizIssues(quiz), quiz.id).toEqual([]);
  });
});

describe("✅️answer-validation", () => {
  const fixture = vectors<{ sheetTasks: SheetTask[]; vectors: { id: string; sheetTask: string; answer?: Answer; expected: { rejection: string | null; complete?: boolean } }[] }>("✅️answer-validation");

  it("rejects and completes every committed answer as committed, an absent answer being incomplete", () => {
    for (const vector of fixture.vectors) {
      const sheetTask = fixture.sheetTasks.find((task) => task.id === vector.sheetTask)!;
      if (vector.answer !== undefined) expect(answerRejection(sheetTask, vector.answer) ?? null, vector.id).toBe(vector.expected.rejection);
      if (vector.expected.complete !== undefined) expect(answerComplete(sheetTask, vector.answer), vector.id).toBe(vector.expected.complete);
    }
  });
});

describe("📏️sorting-concordance, 🔀️matching-concordance, 🕸️profile-similarity", () => {
  for (const name of ["📏️sorting-concordance", "🔀️matching-concordance", "🕸️profile-similarity"]) {
    it(`scores every committed ${name} vector within the parity tolerance`, () => {
      const fixture = vectors<{ tasks: Task[]; vectors: { id: string; task: string; sheetTask: SheetTask; answer: Answer; expected: unknown }[] }>(name);
      expect(fixture.vectors.length).toBeGreaterThan(0);
      for (const vector of fixture.vectors) expectClose(scoreTask(fixture.tasks.find((task) => task.id === vector.task)!, vector.sheetTask, vector.answer), vector.expected, vector.id);
      for (const task of fixture.tasks) expect(quizIssues({ schema: "semio.quiz/v1", id: "vectors", emoji: "🧪", title: { en: "V", de: "V" }, description: { en: "V", de: "V" }, tasks: [task] }), task.id).toEqual([]);
    });
  }
});

describe("🏅️badge-rules", () => {
  const fixture = vectors<{ quizzes: Quiz[]; badges: Badge[]; vectors: { id: string; results: RunResult[]; held: string[]; expected: string[] }[] }>("🏅️badge-rules");

  it("earns exactly the committed badges", () => {
    for (const vector of fixture.vectors) expect(earnedBadges(fixture.badges, fixture.quizzes, vector.results, vector.held), vector.id).toEqual(vector.expected);
  });
});

describe("🏆️leaderboard", () => {
  const fixture = vectors<{ catalog: Catalog; quizzes: Quiz[]; vectors: { id: string; learners: { learner: string; events: Event[] }[]; expected: { learnerViews: Record<string, unknown>; leaderboard: unknown } }[] }>("🏆️leaderboard");
  const view = catalogView(fixture.catalog, fixture.quizzes);

  it("reproduces every committed learner view and leaderboard", () => {
    for (const vector of fixture.vectors) {
      const states = vector.learners.map((learner) => learner.events.reduce(evolveLearner, emptyLearnerState(learner.learner)));
      for (const state of states) expectClose(learnerView(state, view), vector.expected.learnerViews[state.learner], `${vector.id}/${state.learner}`);
      expectClose(leaderboard(states, view), vector.expected.leaderboard, vector.id);
    }
  });

  it("finds no issue in the committed catalog", () => {
    expect(catalogIssues(fixture.catalog, fixture.quizzes)).toEqual([]);
  });
});

describe("🧬️schema-conformance", () => {
  const fixture = vectors<{ rejected: { id: string; definition: "Quiz" | "Catalog"; violates: string; document: unknown }[] }>("🧬️schema-conformance");

  it("reports a structural issue for every committed schema violation", () => {
    for (const vector of fixture.rejected) {
      const issues = vector.definition === "Quiz" ? quizIssues(vector.document) : catalogIssues(vector.document, []);
      expect(
        issues.some((issue) => STRUCTURAL.has(issue.code)),
        `${vector.id}: ${JSON.stringify(issues)}`,
      ).toBe(true);
    }
  });
});

describe("📊️crowd-view", () => {
  const fixture = vectors<{ quizzes: Quiz[]; vectors: { id: string; quiz: string; results: RunResult[]; expected: unknown }[] }>("📊️crowd-view");

  it("aggregates every committed set of results into the committed crowd view within the parity tolerance", () => {
    expect(fixture.vectors.length).toBeGreaterThan(0);
    for (const vector of fixture.vectors) expectClose(crowdView(fixture.quizzes.find((quiz) => quiz.id === vector.quiz)!, vector.results), vector.expected, vector.id);
  });
});

describe("👥️shared-presence", () => {
  const fixture = vectors<{
    scopes: { id: string; catalog: string; place: Place; expected: { roster: string; room: string | null } }[];
    presence: { id: string; state: unknown; expected: boolean }[];
    cursors: { id: string; state: unknown; expected: boolean }[];
    thinkingScopes: { id: string; catalog: string; quiz: string; expected: string }[];
    thinking: { id: string; state: unknown; expected: boolean }[];
  }>("👥️shared-presence");

  it("names every committed thinking room and admits and refuses every committed thinking state as committed", () => {
    expect(fixture.thinkingScopes.length + fixture.thinking.length).toBeGreaterThan(0);
    for (const vector of fixture.thinkingScopes) expect(thinkingScope(vector.catalog, vector.quiz), vector.id).toBe(vector.expected);
    for (const vector of fixture.thinking) expect(thinkingProblem(vector.state) === undefined, `${vector.id}: ${JSON.stringify(thinkingProblem(vector.state))}`).toBe(vector.expected);
  });

  it("maps every committed place to the committed roster and room scope", () => {
    expect(fixture.scopes.length).toBeGreaterThan(0);
    for (const vector of fixture.scopes) expect({ roster: rosterScope(vector.catalog), room: roomScope(vector.catalog, vector.place) ?? null }, vector.id).toEqual(vector.expected);
  });

  it("admits and refuses every committed presence and cursor state as committed", () => {
    expect(fixture.presence.length + fixture.cursors.length).toBeGreaterThan(0);
    for (const vector of fixture.presence) expect(presenceProblem(vector.state) === undefined, `${vector.id}: ${JSON.stringify(presenceProblem(vector.state))}`).toBe(vector.expected);
    for (const vector of fixture.cursors) expect(cursorProblem(vector.state) === undefined, `${vector.id}: ${JSON.stringify(cursorProblem(vector.state))}`).toBe(vector.expected);
  });
});

describe("🧾️learner-lifecycle", () => {
  type Step = { command: Command; now: number; revisions?: Record<string, string>; expected: unknown };
  const fixture = vectors<{
    catalog: Catalog;
    quizzes: Quiz[];
    revisions: Record<string, string>;
    handles: { id: string; handle: string; expected: { display: string; key: string } | null }[];
    roster: { id: string; steps: Step[] }[];
    learners: { id: string; learner: string; given: Event[]; steps: Step[]; views?: { runs: string[]; expected: { learner: unknown; runs: Record<string, unknown> } } }[];
  }>("🧾️learner-lifecycle");
  const loaded = (revisions: Record<string, string>): Record<string, LoadedQuiz> => Object.fromEntries(fixture.quizzes.map((quiz) => [quiz.id, { quiz, revision: revisions[quiz.id]! }]));

  it("normalises every committed handle", () => {
    for (const vector of fixture.handles) expect(normalizeHandle(vector.handle) ?? null, vector.id).toEqual(vector.expected);
  });

  it("decides every committed roster step", () => {
    for (const vector of fixture.roster) {
      let state = emptyRosterState();
      for (const [index, step] of vector.steps.entries()) {
        const decision = decideRoster(state, step.command as IdentifyLearnerCommand, step.now);
        expectClose(decision, step.expected, `${vector.id}/${index}`);
        if ("events" in decision) state = decision.events.reduce(evolveRoster, state);
      }
    }
  });

  it("decides every committed learner step and serves the committed views", () => {
    const view = catalogView(fixture.catalog, fixture.quizzes);
    for (const vector of fixture.learners) {
      let state: LearnerState = vector.given.reduce(evolveLearner, emptyLearnerState(vector.learner));
      let revisions = { ...fixture.revisions };
      for (const [index, step] of vector.steps.entries()) {
        revisions = { ...revisions, ...step.revisions };
        const decision = decideLearner(state, step.command as Exclude<Command, IdentifyLearnerCommand>, { now: step.now, catalog: fixture.catalog, quizzes: loaded(revisions) });
        expectClose(decision, step.expected, `${vector.id}/${index}`);
        if ("events" in decision) state = decision.events.reduce(evolveLearner, state);
      }
      if (!vector.views) continue;
      expectClose(learnerView(state, view), vector.views.expected.learner, `${vector.id}/learner`);
      for (const run of vector.views.runs) expectClose(runView(state, run, loaded(revisions)), vector.views.expected.runs[run], `${vector.id}/${run}`);
    }
  });

  it("flags exactly the badges no catalog task can earn and finds no issue in the quizzes", () => {
    const unreachable = fixture.catalog.badges.flatMap((badge, index) => {
      const rule = badge.rule;
      if (rule.kind !== "perfect-tasks") return [];
      const reachable = fixture.quizzes.some((quiz) => (rule.quiz === undefined || quiz.id === rule.quiz) && quiz.tasks.some((task) => rule.taskKind === undefined || task.kind === rule.taskKind));
      return reachable ? [] : [{ path: `/badges/${index}/rule`, code: "badge-unreachable" }];
    });
    expect(catalogIssues(fixture.catalog, fixture.quizzes)).toEqual(unreachable);
    for (const quiz of fixture.quizzes) expect(quizIssues(quiz), quiz.id).toEqual([]);
  });
});
