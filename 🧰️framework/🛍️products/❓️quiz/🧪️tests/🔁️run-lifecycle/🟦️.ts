import { describe, expect, it } from "vitest";
import {
  decideLearner,
  decideRoster,
  emptyLearnerState,
  emptyRosterState,
  evolveLearner,
  evolveRoster,
  normalizeHandle,
  runSeed,
  scoreRun,
  sheetOf,
  type Answer,
  type Catalog,
  type Command,
  type Decision,
  type Event,
  type IdentifyLearnerCommand,
  type LearnerContext,
  type LearnerState,
  type Quiz,
  type RosterState,
  type StartRunCommand,
  type Text,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

const T = (en: string): Text => ({ en, de: en });
const U = (...codes: number[]): string => String.fromCodePoint(...codes);
const id = (n: number) => n.toString(16).padStart(32, "0");
const ADA = id(1);
const BOB = id(2);
const RUN = id(100);
const REVISION = "a".repeat(64);
const REVISED = "b".repeat(64);

const QUIZ: Quiz = {
  schema: "semio.quiz/v1",
  id: "physics",
  emoji: "🧲",
  title: T("Physics"),
  description: T("Physics"),
  tasks: [
    { kind: "sorting", id: "power", title: T("Power"), prompt: T("Sort"), quantity: { label: T("Power"), unit: "W", scale: "logarithmic", prefixed: true }, items: [{ id: "bulb", label: T("Bulb"), value: 60 }, { id: "kettle", label: T("Kettle"), value: 2000 }, { id: "plant", label: T("Plant"), value: 1e9 }] },
    { kind: "classification", id: "units", title: T("Units"), prompt: T("Classify"), categories: [{ id: "power", label: T("Power") }, { id: "energy", label: T("Energy") }], items: [{ id: "watt", label: T("W"), category: "power" }, { id: "kwh", label: T("kWh"), category: "energy" }] },
  ],
};
const OTHER: Quiz = { ...QUIZ, id: "heating", title: T("Heating") };
const CATALOG: Catalog = {
  schema: "semio.quiz.catalog/v1",
  id: "architecture",
  title: T("Architecture"),
  introduction: { title: T("Welcome"), paragraphs: [T("Hello")] },
  quizzes: ["physics.json", "heating.json"],
  badges: [
    { id: "perfect-physics", emoji: "🧲", label: T("Perfect"), description: T("Perfect"), rule: { kind: "perfect-quiz", quiz: "physics" } },
    { id: "sorter", emoji: "📶", label: T("Sorter"), description: T("Sorter"), rule: { kind: "perfect-tasks", taskKind: "sorting", quiz: "physics" } },
    { id: "completionist", emoji: "🏁", label: T("Done"), description: T("Done"), rule: { kind: "completed-quizzes" } },
  ],
};

/** 🧭️ The decision context at `now` with the physics quiz at `revision`. */
function context(now: number, revision = REVISION): LearnerContext {
  return { now, catalog: CATALOG, quizzes: { physics: { quiz: QUIZ, revision }, heating: { quiz: OTHER, revision: REVISION } } };
}

/** ⚡️ The events of a decision, failing on a rejection. */
function events(decision: Decision): readonly Event[] {
  if ("rejection" in decision) throw new Error(`rejected: ${decision.rejection}`);
  return decision.events;
}

/** 🧬️ The learner state after folding the given events. */
function fold(state: LearnerState, folded: readonly Event[]): LearnerState {
  return folded.reduce(evolveLearner, state);
}

/** ▶️ Ada, registered, with the given commands decided and applied in order. */
function ada(commands: readonly [Exclude<Command, IdentifyLearnerCommand>, LearnerContext][]): LearnerState {
  let state = fold(emptyLearnerState(ADA), [{ type: "learner-registered", learner: ADA, identity: { kind: "pseudonym", handle: "Ada" }, at: 1 }]);
  for (const [command, decisionContext] of commands) state = fold(state, events(decideLearner(state, command, decisionContext)));
  return state;
}

const start = (run = RUN, quiz = "physics"): StartRunCommand => ({ type: "start-run", id: id(1000), learner: ADA, run, quiz });
const record = (task: string, answer: Answer, run = RUN): Exclude<Command, IdentifyLearnerCommand> => ({ type: "record-answer", id: id(2000), learner: ADA, run, task, answer });
const submit = (run = RUN): Exclude<Command, IdentifyLearnerCommand> => ({ type: "submit-run", id: id(3000), learner: ADA, run });
const PERFECT_SORTING: Answer = { kind: "sorting", order: ["bulb", "kettle", "plant"] };
const PERFECT_UNITS: Answer = { kind: "classification", assignments: { watt: "power", kwh: "energy" } };

describe("normalizeHandle", () => {
  it("trims, collapses inner whitespace runs and lowercases the key", () => {
    expect(normalizeHandle("  Ada \t  Lovelace ")).toEqual({ display: "Ada Lovelace", key: "ada lovelace" });
    expect(normalizeHandle(`${U(0x3000)}Ada${U(0xa0, 0x2003)}Lovelace${U(0x0a, 0x85)}`)).toEqual({ display: "Ada Lovelace", key: "ada lovelace" });
    expect(normalizeHandle("ÄRGER")).toEqual({ display: "ÄRGER", key: "ärger" });
    expect(normalizeHandle("ΟΔΟΣ")?.key).toBe("οδος");
  });

  it("treats exactly the Unicode White_Space characters as whitespace", () => {
    expect(normalizeHandle(`${U(0xfeff)}Ada`)?.display).toBe(`${U(0xfeff)}Ada`);
    expect(normalizeHandle(`Ada${U(0x200b)}Lovelace`)?.display).toBe(`Ada${U(0x200b)}Lovelace`);
  });

  it("accepts 1 to 64 code points", () => {
    expect(normalizeHandle("")).toBeUndefined();
    expect(normalizeHandle(" \t ")).toBeUndefined();
    expect(normalizeHandle("x")?.display).toBe("x");
    expect(normalizeHandle("a".repeat(64))?.display).toHaveLength(64);
    expect(normalizeHandle("a".repeat(65))).toBeUndefined();
    expect(normalizeHandle(` ${"🎲".repeat(64)} `)?.display).toBe("🎲".repeat(64));
    expect(normalizeHandle("🎲".repeat(65))).toBeUndefined();
  });
});

describe("roster", () => {
  const identify = (learner: string, identity: IdentifyLearnerCommand["identity"]): IdentifyLearnerCommand => ({ type: "identify-learner", id: id(9), learner, identity });

  it("registers anonymous learners, always as new", () => {
    const roster: RosterState = { handles: { ada: ADA } };
    expect(decideRoster(roster, identify(BOB, { kind: "anonymous" }), 5)).toEqual({ events: [{ type: "learner-registered", learner: BOB, identity: { kind: "anonymous" }, at: 5 }] });
    expect(evolveRoster(roster, { type: "learner-registered", learner: BOB, identity: { kind: "anonymous" }, at: 5 })).toBe(roster);
  });

  it("registers an unclaimed handle under its display form and recalls it case- and space-insensitively across kinds", () => {
    const registered = events(decideRoster(emptyRosterState(), identify(ADA, { kind: "pseudonym", handle: "  Ada   Lovelace " }), 7));
    expect(registered).toEqual([{ type: "learner-registered", learner: ADA, identity: { kind: "pseudonym", handle: "Ada Lovelace" }, at: 7 }]);
    const roster = registered.reduce(evolveRoster, emptyRosterState());
    expect(roster).toEqual({ handles: { "ada lovelace": ADA } });
    expect(decideRoster(roster, identify(BOB, { kind: "name", handle: "ADA LOVELACE" }), 8)).toEqual({ events: [{ type: "learner-recalled", learner: ADA, at: 8 }] });
    expect(evolveRoster(roster, { type: "learner-recalled", learner: ADA, at: 8 })).toBe(roster);
  });

  it("rejects handles that normalise to nothing or exceed 64 code points", () => {
    expect(decideRoster(emptyRosterState(), identify(ADA, { kind: "name", handle: "   " }), 1)).toEqual({ rejection: "handle-invalid" });
    expect(decideRoster(emptyRosterState(), identify(ADA, { kind: "pseudonym", handle: "x".repeat(65) }), 1)).toEqual({ rejection: "handle-invalid" });
  });
});

describe("learner", () => {
  it("rejects runs of unregistered learners and unknown quizzes", () => {
    expect(decideLearner(emptyLearnerState(ADA), start(), context(10))).toEqual({ rejection: "unknown-learner" });
    expect(decideLearner(ada([]), { ...start(), learner: BOB }, context(10))).toEqual({ rejection: "unknown-learner" });
    expect(decideLearner(ada([]), start(RUN, "cooling"), context(10))).toEqual({ rejection: "unknown-quiz" });
  });

  it("starts a run against the current revision with the seed of its id", () => {
    const state = ada([]);
    expect(decideLearner(state, start(), context(10))).toEqual({ events: [{ type: "run-started", learner: ADA, run: RUN, quiz: "physics", revision: REVISION, seed: runSeed(RUN), at: 10 }] });
    const started = ada([[start(), context(10)]]);
    expect(started.runs).toEqual([{ run: RUN, quiz: "physics", revision: REVISION, seed: runSeed(RUN), status: "open", answers: {}, startedAt: 10 }]);
    expect(started.lastActivity).toBe(10);
  });

  it("allows one open run per quiz and never reuses a run id", () => {
    const started = ada([[start(), context(10)]]);
    expect(decideLearner(started, start(id(101)), context(11))).toEqual({ rejection: "run-open" });
    expect(decideLearner(started, start(RUN, "heating"), context(11))).toEqual({ rejection: "run-open" });
    expect(events(decideLearner(started, start(id(101), "heating"), context(11)))).toHaveLength(1);
  });

  it("records valid answers to tasks of the sheet, the latest winning", () => {
    const started = ada([[start(), context(10)]]);
    expect(decideLearner(started, record("power", PERFECT_SORTING, id(404)), context(11))).toEqual({ rejection: "unknown-run" });
    expect(decideLearner(started, record("essay", PERFECT_SORTING), context(11))).toEqual({ rejection: "unknown-task" });
    expect(decideLearner(started, record("power", { kind: "sorting", order: ["bulb"] }), context(11))).toEqual({ rejection: "answer-invalid" });
    expect(decideLearner(started, record("power", PERFECT_UNITS), context(11))).toEqual({ rejection: "answer-invalid" });
    expect(decideLearner(started, record("units", { kind: "classification", assignments: { watt: "power" } }), context(11))).toEqual({ events: [{ type: "answer-recorded", learner: ADA, run: RUN, task: "units", answer: { kind: "classification", assignments: { watt: "power" } }, at: 11 }] });
    const answered = ada([
      [start(), context(10)],
      [record("units", { kind: "classification", assignments: { watt: "energy" } }), context(11)],
      [record("units", PERFECT_UNITS), context(12)],
    ]);
    expect(answered.runs[0]!.answers).toEqual({ units: PERFECT_UNITS });
  });

  it("submits only complete runs, scores them and awards the badges they newly earn", () => {
    const partial = ada([
      [start(), context(10)],
      [record("units", { kind: "classification", assignments: { watt: "power" } }), context(11)],
      [record("power", PERFECT_SORTING), context(12)],
    ]);
    expect(decideLearner(partial, submit(), context(13))).toEqual({ rejection: "run-incomplete" });
    const complete = fold(partial, events(decideLearner(partial, record("units", PERFECT_UNITS), context(14))));
    const result = scoreRun(QUIZ, sheetOf(QUIZ, runSeed(RUN)), { power: PERFECT_SORTING, units: PERFECT_UNITS });
    expect(result?.score).toBe(1);
    expect(decideLearner(complete, submit(), context(15))).toEqual({
      events: [
        { type: "run-submitted", learner: ADA, run: RUN, result, at: 15 },
        { type: "badge-awarded", learner: ADA, badge: "perfect-physics", run: RUN, at: 15 },
        { type: "badge-awarded", learner: ADA, badge: "sorter", run: RUN, at: 15 },
      ],
    });
    const submitted = fold(complete, events(decideLearner(complete, submit(), context(15))));
    expect(submitted.runs[0]).toMatchObject({ status: "submitted", result, submittedAt: 15 });
    expect(submitted.badges).toEqual([
      { badge: "perfect-physics", run: RUN, at: 15 },
      { badge: "sorter", run: RUN, at: 15 },
    ]);
    expect(decideLearner(submitted, record("units", PERFECT_UNITS), context(16))).toEqual({ rejection: "run-closed" });
    expect(decideLearner(submitted, submit(), context(16))).toEqual({ rejection: "run-closed" });
    expect(decideLearner(submitted, start(), context(16))).toEqual({ rejection: "run-closed" });
  });

  it("awards held badges only once and completes the catalog with a second quiz", () => {
    const second = id(101);
    const state = ada([
      [start(), context(10)],
      [record("power", PERFECT_SORTING), context(11)],
      [record("units", PERFECT_UNITS), context(12)],
      [submit(), context(13)],
      [start(second, "heating"), context(20)],
      [record("power", { kind: "sorting", order: ["plant", "kettle", "bulb"] }, second), context(21)],
      [record("units", PERFECT_UNITS, second), context(22)],
    ]);
    const decided = events(decideLearner(state, submit(second), context(23)));
    expect(decided.map((event) => (event.type === "badge-awarded" ? event.badge : event.type))).toEqual(["run-submitted", "completionist"]);
  });

  it("rejects answers to a revised quiz and voids its open run on submission or restart", () => {
    const started = ada([
      [start(), context(10)],
      [record("power", PERFECT_SORTING), context(11)],
    ]);
    expect(decideLearner(started, record("units", PERFECT_UNITS), context(12, REVISED))).toEqual({ rejection: "quiz-revised" });
    expect(decideLearner(started, submit(), context(12, REVISED))).toEqual({ events: [{ type: "run-voided", learner: ADA, run: RUN, at: 12 }] });
    const restart = id(102);
    expect(decideLearner(started, start(restart), context(13, REVISED))).toEqual({
      events: [
        { type: "run-voided", learner: ADA, run: RUN, at: 13 },
        { type: "run-started", learner: ADA, run: restart, quiz: "physics", revision: REVISED, seed: runSeed(restart), at: 13 },
      ],
    });
    const restarted = fold(started, events(decideLearner(started, start(restart), context(13, REVISED))));
    expect(restarted.runs.map((run) => [run.run, run.status])).toEqual([
      [RUN, "voided"],
      [restart, "open"],
    ]);
    expect(decideLearner(restarted, record("power", PERFECT_SORTING), context(14, REVISED))).toEqual({ rejection: "run-closed" });
  });

  it("rejects a complete run that cannot be scored as incomplete instead of throwing", () => {
    const broken: Quiz = {
      ...QUIZ,
      tasks: [{ kind: "matching", id: "walls", title: T("Walls"), prompt: T("Match"), dimensions: [{ id: "u", quantity: { label: T("U"), unit: "W/(m²K)", scale: "linear", prefixed: false } }], items: [{ id: "old", label: T("Old"), values: { u: 1.4 } }, { id: "new", label: T("New"), values: {} }] }],
    };
    const brokenContext = (now: number): LearnerContext => ({ ...context(now), quizzes: { ...context(now).quizzes, physics: { quiz: broken, revision: REVISION } } });
    const state = ada([
      [start(), brokenContext(10)],
      [record("walls", { kind: "matching", assignments: { u: { old: 0, new: 1 } } }), brokenContext(11)],
    ]);
    expect(state.runs[0]!.answers).toHaveProperty("walls");
    expect(decideLearner(state, submit(), brokenContext(12))).toEqual({ rejection: "run-incomplete" });
  });

  it("folds only its own events and tracks the latest activity", () => {
    const state = ada([]);
    expect(evolveLearner(state, { type: "learner-recalled", learner: BOB, at: 99 })).toBe(state);
    expect(evolveLearner(state, { type: "learner-recalled", learner: ADA, at: 99 }).lastActivity).toBe(99);
    expect(evolveLearner({ ...state, lastActivity: 200 }, { type: "learner-recalled", learner: ADA, at: 99 }).lastActivity).toBe(200);
    expect(state.identity).toEqual({ kind: "pseudonym", handle: "Ada" });
  });
});
