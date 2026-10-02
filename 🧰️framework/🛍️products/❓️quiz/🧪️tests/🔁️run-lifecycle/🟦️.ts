import { describe, expect, it } from "vitest";
import {
  DEFAULT_LIMITS,
  HANDLE_INPUT_MAX,
  HANDLE_LETTERS,
  HANDLE_MAX,
  WHITE_SPACE,
  commandRejection,
  decideHandle,
  decideLearner,
  emptyHandleState,
  emptyLearnerState,
  evolveHandle,
  evolveLearner,
  handleActorId,
  handleKeyOf,
  isId,
  isSlug,
  normalizeHandle,
  queryRejection,
  registrationRejection,
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
  return { now, catalog: CATALOG, quizzes: { physics: { quiz: QUIZ, revision }, heating: { quiz: OTHER, revision: REVISION } }, limits: DEFAULT_LIMITS };
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
function ada(commands: readonly [Command, LearnerContext][]): LearnerState {
  let state = fold(emptyLearnerState(ADA), [{ type: "learner-registered", learner: ADA, identity: { kind: "pseudonym", handle: "Ada" }, at: 1 }]);
  for (const [command, decisionContext] of commands) state = fold(state, events(decideLearner(state, command, decisionContext)));
  return state;
}

const start = (run = RUN, quiz = "physics"): StartRunCommand => ({ type: "start-run", id: id(1000), learner: ADA, run, quiz });
const record = (task: string, answer: Answer, run = RUN): Exclude<Command, IdentifyLearnerCommand> => ({ type: "record-answer", id: id(2000), learner: ADA, run, task, answer });
const submit = (run = RUN): Exclude<Command, IdentifyLearnerCommand> => ({ type: "submit-run", id: id(3000), learner: ADA, run });
const PERFECT_SORTING: Answer = { kind: "sorting", order: ["bulb", "kettle", "plant"] };
const PERFECT_UNITS: Answer = { kind: "classification", assignments: { watt: "power", kwh: "energy" } };

/** 🔢️ Every Unicode scalar value. */
function* scalars(): Generator<number> {
  for (let point = 0; point < 0x110000; point++) if (point < 0xd800 || point > 0xdfff) yield point;
}

describe("normalizeHandle", () => {
  it("trims, collapses White_Space runs, folds the typographic apostrophe and lowercases the key", () => {
    expect(normalizeHandle("  Ada \t  Lovelace ")).toEqual({ display: "Ada Lovelace", key: "ada lovelace" });
    expect(normalizeHandle(`${U(0x3000)}Ada${U(0xa0, 0x2003)}Lovelace${U(0x0a, 0x85)}`)).toEqual({ display: "Ada Lovelace", key: "ada lovelace" });
    expect(normalizeHandle("ÄRGER")).toEqual({ display: "ÄRGER", key: "ärger" });
    expect(normalizeHandle("GROẞ Straße")).toEqual({ display: "GROẞ Straße", key: "groß straße" });
    expect(normalizeHandle(`O${U(0x2019)}Brien`)).toEqual({ display: "O'Brien", key: "o'brien" });
    expect(normalizeHandle("Anna-Lena_2 jr.")).toEqual({ display: "Anna-Lena_2 jr.", key: "anna-lena_2 jr." });
  });

  it("holds 1 to 64 code points with at least one letter or digit, typed with at most 256", () => {
    expect(normalizeHandle("")).toBeUndefined();
    expect(normalizeHandle(" \t ")).toBeUndefined();
    expect(normalizeHandle("x")?.display).toBe("x");
    expect(normalizeHandle("7")?.display).toBe("7");
    expect(normalizeHandle("a".repeat(HANDLE_MAX))?.display).toHaveLength(64);
    expect(normalizeHandle("a".repeat(HANDLE_MAX + 1))).toBeUndefined();
    expect(normalizeHandle(`${" ".repeat(HANDLE_INPUT_MAX - 1)}a`)?.display).toBe("a");
    expect(normalizeHandle(`${" ".repeat(HANDLE_INPUT_MAX)}a`)).toBeUndefined();
    for (const punctuation of ["'", "-", ".", "_", "...", "' -"]) expect(normalizeHandle(punctuation), punctuation).toBeUndefined();
  });

  it("refuses invisible, bidirectional, control and combining characters, other scripts and lone surrogates", () => {
    for (const point of [0x00, 0x07, 0x1b, 0x1f, 0x7f, 0x80, 0xad, 0x180e, 0x200b, 0x200c, 0x200d, 0x200e, 0x200f, 0x202a, 0x202e, 0x2060, 0x2066, 0x2069, 0xfeff, 0x0301, 0x0308, 0x0430, 0x03bf, 0xff21, 0x1d400, 0x1f98a]) {
      expect(normalizeHandle(`Ada${U(point)}Lovelace`), point.toString(16)).toBeUndefined();
    }
    expect(normalizeHandle("Ada\ud800")).toBeUndefined();
    expect(normalizeHandle("\udc00Ada")).toBeUndefined();
    expect(normalizeHandle("André".normalize("NFD"))).toBeUndefined();
    expect(normalizeHandle("André".normalize("NFD").normalize("NFC"))?.key).toBe("andré");
  });

  it("names the stream of a key by the hex of its UTF-8 bytes, and back", () => {
    expect(handleActorId("ada")).toBe("616461");
    expect(handleActorId("jürgen müller")).toBe("6ac3bc7267656e206dc3bc6c6c6572");
    expect(handleKeyOf(handleActorId("groß straße"))).toBe("groß straße");
    for (const id of ["", "6", "6G", "C3", "c3", "c328", "ff", "616461 "]) expect(handleKeyOf(id), id).toBeUndefined();
  });

  it("owns the tables the Unicode data of the runtime derives", () => {
    const within = (ranges: readonly (readonly [number, number])[], point: number): boolean => ranges.some(([low, high]) => point >= low && point <= high);
    const blocks: readonly (readonly [number, number])[] = [[0x0000, 0x024f], [0x1e00, 0x1eff]];
    const whiteSpace = /^\p{White_Space}$/u;
    const letter = /^[\p{Lu}\p{Ll}]$/u;
    const differing: string[] = [];
    for (const point of scalars()) {
      const character = U(point);
      if (within(WHITE_SPACE, point) !== whiteSpace.test(character)) differing.push(`white space ${point.toString(16)}`);
      if (within(HANDLE_LETTERS, point) !== (within(blocks, point) && letter.test(character) && character.normalize("NFKC") === character)) differing.push(`letter ${point.toString(16)}`);
    }
    expect(differing).toEqual([]);
  });

  it("spells only NFC: every member of the alphabet is stable and no two members compose", () => {
    const members: string[] = [];
    for (const point of scalars()) if (normalizeHandle(`a${U(point)}a`)?.display === `a${U(point)}a`) members.push(U(point));
    expect(members).toHaveLength(681 + 10 + 1 + 4);
    expect(members.filter((member) => member.normalize("NFC") !== member)).toEqual([]);
    expect(
      members.filter((first) => {
        const row = members.map((second) => first + second).join("\n");
        return row.normalize("NFC") !== row;
      }),
    ).toEqual([]);
  });
});

describe("ids", () => {
  it("are 32 lowercase hex characters, slugs kebab-case of at most 64 characters", () => {
    expect(isId(ADA)).toBe(true);
    for (const value of ["ABCDEF0123456789ABCDEF0123456789", ADA.slice(1), `${ADA}0`, `${ADA}\n`, ` ${ADA.slice(1)}`, "g".repeat(32), "", 7, null, undefined]) expect(isId(value), String(value)).toBe(false);
    for (const value of ["a", "power-ratings", "a1-b2", "q".repeat(64)]) expect(isSlug(value), value).toBe(true);
    for (const value of ["", "A", "a--b", "-a", "a-", "a_b", "a/b", "q".repeat(65), "a\n", 7]) expect(isSlug(value), String(value)).toBe(false);
  });

  it("refuse a command or query that carries a malformed id", () => {
    expect(commandRejection(start())).toBeUndefined();
    expect(commandRejection({ ...start(), run: "run-1" })).toBe("id-invalid");
    expect(commandRejection({ ...start(), quiz: "Physics" })).toBe("id-invalid");
    expect(commandRejection({ ...submit(), id: "enroll:architecture:roster:1" })).toBe("id-invalid");
    expect(commandRejection({ ...record("power", PERFECT_SORTING), task: "../power" })).toBe("id-invalid");
    expect(queryRejection({ type: "leaderboard", period: "all-time" })).toBeUndefined();
    expect(queryRejection({ type: "leaderboard", period: "daily", quiz: "physics", learner: ADA })).toBeUndefined();
    expect(queryRejection({ type: "leaderboard", period: "weekly", learner: "ada" })).toBe("id-invalid");
    expect(queryRejection({ type: "leaderboard", period: "monthly", quiz: "Physics", learner: ADA })).toBe("id-invalid");
    expect(queryRejection({ type: "run", run: "ABCDEF0123456789ABCDEF0123456789" })).toBe("id-invalid");
    expect(queryRejection({ type: "crowd", quiz: "Physics" })).toBe("id-invalid");
    expect(queryRejection({ type: "handle", handle: " Ada " })).toBeUndefined();
    expect(queryRejection({ type: "handle", handle: `A${U(0x200b)}da` })).toBe("handle-invalid");
  });
});

describe("handle", () => {
  const identify = (learner: string, identity: IdentifyLearnerCommand["identity"]): IdentifyLearnerCommand => ({ type: "identify-learner", id: id(9), learner, identity });
  const key = "ada lovelace";

  it("registers the first claim of its key under the display form and refuses every later one", () => {
    const registered = events(decideHandle(emptyHandleState(key), identify(ADA, { kind: "pseudonym", handle: "  Ada   Lovelace " }), 7));
    expect(registered).toEqual([{ type: "learner-registered", learner: ADA, identity: { kind: "pseudonym", handle: "Ada Lovelace" }, at: 7 }]);
    const claimed = registered.reduce(evolveHandle, emptyHandleState(key));
    expect(claimed).toEqual({ key, holder: ADA });
    expect(decideHandle(claimed, identify(BOB, { kind: "name", handle: "ADA LOVELACE" }), 8)).toEqual({ rejection: "handle-claimed" });
    expect(decideHandle(claimed, identify(ADA, { kind: "pseudonym", handle: "Ada Lovelace" }), 9)).toEqual({ rejection: "handle-claimed" });
  });

  it("refuses handles outside the policy, handles of another key, anonymous identities and malformed ids", () => {
    expect(decideHandle(emptyHandleState(key), identify(ADA, { kind: "name", handle: "   " }), 1)).toEqual({ rejection: "handle-invalid" });
    expect(decideHandle(emptyHandleState(key), identify(ADA, { kind: "pseudonym", handle: "x".repeat(65) }), 1)).toEqual({ rejection: "handle-invalid" });
    expect(decideHandle(emptyHandleState(key), identify(ADA, { kind: "name", handle: "Bob" }), 1)).toEqual({ rejection: "handle-invalid" });
    expect(decideHandle(emptyHandleState(key), identify(ADA, { kind: "anonymous" }), 1)).toEqual({ rejection: "handle-invalid" });
    expect(decideHandle(emptyHandleState(key), identify("ada", { kind: "name", handle: "Ada Lovelace" }), 1)).toEqual({ rejection: "id-invalid" });
    expect(evolveHandle(emptyHandleState(key), { type: "learner-registered", learner: BOB, identity: { kind: "anonymous" }, at: 5 })).toEqual({ key });
  });

  it("refuses registrations once the proctor holds its cap of learners", () => {
    expect(registrationRejection(DEFAULT_LIMITS.learners - 1, DEFAULT_LIMITS)).toBeUndefined();
    expect(registrationRejection(DEFAULT_LIMITS.learners, DEFAULT_LIMITS)).toBe("roster-full");
    expect(registrationRejection(1, { ...DEFAULT_LIMITS, learners: 1 })).toBe("roster-full");
  });
});

describe("learner", () => {
  it("registers an anonymous learner once, in its own stream", () => {
    const identify = (learner: string, identity: IdentifyLearnerCommand["identity"]): IdentifyLearnerCommand => ({ type: "identify-learner", id: id(9), learner, identity });
    expect(decideLearner(emptyLearnerState(ADA), identify(ADA, { kind: "anonymous" }), context(5))).toEqual({ events: [{ type: "learner-registered", learner: ADA, identity: { kind: "anonymous" }, at: 5 }] });
    expect(decideLearner(ada([]), identify(ADA, { kind: "anonymous" }), context(6))).toEqual({ rejection: "learner-exists" });
    expect(decideLearner(emptyLearnerState(ADA), identify(ADA, { kind: "name", handle: "Ada" }), context(5))).toEqual({ rejection: "handle-invalid" });
    expect(decideLearner(emptyLearnerState(ADA), identify(BOB, { kind: "anonymous" }), context(5))).toEqual({ rejection: "unknown-learner" });
  });

  it("refuses commands with malformed ids before anything else", () => {
    expect(decideLearner(ada([]), { ...start(), run: "run-1" }, context(10))).toEqual({ rejection: "id-invalid" });
    expect(decideLearner(emptyLearnerState("ada"), { ...start(), learner: "ada" }, context(10))).toEqual({ rejection: "id-invalid" });
    expect(decideLearner(ada([[start(), context(10)]]), { ...record("power", PERFECT_SORTING), task: "Power" }, context(11))).toEqual({ rejection: "id-invalid" });
  });

  it("caps the submitted runs per quiz and in total, and the answers recorded per run", () => {
    const capped = (now: number): LearnerContext => ({ ...context(now), limits: { learners: 10, runsPerQuiz: 1, runs: 2, answersPerRun: 3 } });
    const second = id(101);
    const state = ada([
      [start(), capped(10)],
      [record("power", PERFECT_SORTING), capped(11)],
      [record("units", PERFECT_UNITS), capped(12)],
      [record("units", PERFECT_UNITS), capped(13)],
    ]);
    expect(state.runs[0]!.recorded).toBe(3);
    expect(decideLearner(state, record("units", PERFECT_UNITS), capped(14))).toEqual({ rejection: "answers-exhausted" });
    const submitted = fold(state, events(decideLearner(state, submit(), capped(15))));
    expect(decideLearner(submitted, start(second), capped(16))).toEqual({ rejection: "runs-exhausted" });
    const other = fold(submitted, events(decideLearner(submitted, start(second, "heating"), capped(17))));
    const played = [record("power", PERFECT_SORTING, second), record("units", PERFECT_UNITS, second), submit(second)].reduce((folded, command, index) => fold(folded, events(decideLearner(folded, command, capped(18 + index)))), other);
    expect(decideLearner(played, start(id(102), "heating"), capped(30))).toEqual({ rejection: "runs-exhausted" });
    expect(decideLearner(played, start(id(102), "heating"), context(30))).toHaveProperty("events");
  });

  it("rejects runs of unregistered learners and unknown quizzes", () => {
    expect(decideLearner(emptyLearnerState(ADA), start(), context(10))).toEqual({ rejection: "unknown-learner" });
    expect(decideLearner(ada([]), { ...start(), learner: BOB }, context(10))).toEqual({ rejection: "unknown-learner" });
    expect(decideLearner(ada([]), start(RUN, "cooling"), context(10))).toEqual({ rejection: "unknown-quiz" });
  });

  it("starts a run against the current revision with the seed of its id", () => {
    const state = ada([]);
    expect(decideLearner(state, start(), context(10))).toEqual({ events: [{ type: "run-started", learner: ADA, run: RUN, quiz: "physics", revision: REVISION, seed: runSeed(RUN), at: 10 }] });
    const started = ada([[start(), context(10)]]);
    expect(started.runs).toEqual([{ run: RUN, quiz: "physics", revision: REVISION, seed: runSeed(RUN), status: "open", answers: {}, recorded: 0, startedAt: 10 }]);
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

  it("folds only its own events", () => {
    const state = ada([]);
    expect(evolveLearner(state, { type: "run-started", learner: BOB, run: RUN, quiz: "physics", revision: REVISION, seed: runSeed(RUN), at: 99 })).toBe(state);
    expect(evolveLearner(state, { type: "learner-registered", learner: BOB, identity: { kind: "anonymous" }, at: 99 })).toBe(state);
    expect(state.identity).toEqual({ kind: "pseudonym", handle: "Ada" });
  });
});
