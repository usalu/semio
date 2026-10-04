import { describe, expect, it } from "vitest";
import {
  CHALLENGES,
  CLOCK_LEAD,
  DEFAULT_LIMITS,
  HANDLE_INPUT_MAX,
  HANDLE_LETTERS,
  HANDLE_MAX,
  MAX_TIMESTAMP,
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
  isTimestamp,
  normalizeHandle,
  queryRejection,
  registrationRejection,
  runSeed,
  scoreRun,
  sheetOf,
  type Answer,
  type Catalog,
  type Challenge,
  type Command,
  type Decision,
  type Event,
  type IdentifyLearnerCommand,
  type LearnerContext,
  type LearnerState,
  type OpenTaskCommand,
  type Quiz,
  type RecordAnswerCommand,
  type RunSubmittedEvent,
  type StartRunCommand,
  type SubmitRunCommand,
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
    { kind: "sorting", id: "power", title: T("Power"), prompt: T("Sort"), quantity: { label: T("Power"), unit: "W", scale: "logarithmic", prefixed: true, additive: true }, items: [{ id: "bulb", label: T("Bulb"), value: 60 }, { id: "kettle", label: T("Kettle"), value: 2000 }, { id: "plant", label: T("Plant"), value: 1e9 }] },
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
    { id: "guesser", emoji: "🔮", label: T("Guesser"), description: T("Guesser"), rule: { kind: "perfect-quiz", quiz: "physics", challenge: "hard" } },
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

/** 🔚️ An instant after every decision time of this suite, as a device whose clock runs far ahead stamps it: a decider never bounds it by its own clock, so a command stamped with it acts at this instant. */
const LATE = 2 ** 40;
const start = (at: number, run = RUN, quiz = "physics", challenge: Challenge = "medium"): StartRunCommand => ({ type: "start-run", id: id(1000), learner: ADA, run, quiz, challenge, at });
const open = (task: string, at = LATE, run = RUN): OpenTaskCommand => ({ type: "open-task", id: id(1500), learner: ADA, run, task, at });
const record = (task: string, answer: Answer, run = RUN, at = LATE): RecordAnswerCommand => ({ type: "record-answer", id: id(2000), learner: ADA, run, task, answer, at });
const submit = (run = RUN): SubmitRunCommand => ({ type: "submit-run", id: id(3000), learner: ADA, run });
const PERFECT_SORTING: Answer = { kind: "sorting", order: ["bulb", "kettle", "plant"] };
const GUESSED_SORTING: Answer = { kind: "sorting", order: ["bulb", "kettle", "plant"], guesses: { bulb: 100, kettle: 1500, plant: 2e9 } };
const PERFECT_UNITS: Answer = { kind: "classification", assignments: { watt: "power", kwh: "energy" } };
/** ⏲️ The seconds the tasks of the physics quiz allow on a timed sheet. */
const SECONDS = { power: 30 + 12 * 3, units: 30 + 8 * 2 } as const;

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
    expect(commandRejection(start(0))).toBeUndefined();
    expect(commandRejection({ ...start(0), run: "run-1" })).toBe("id-invalid");
    expect(commandRejection({ ...start(0), quiz: "Physics" })).toBe("id-invalid");
    expect(commandRejection({ ...submit(), id: "enroll:architecture:roster:1" })).toBe("id-invalid");
    expect(commandRejection({ ...record("power", PERFECT_SORTING), task: "../power" })).toBe("id-invalid");
    expect(commandRejection(open("power"))).toBeUndefined();
    expect(commandRejection({ ...open("power"), task: "Power" })).toBe("id-invalid");
    expect(commandRejection({ ...open("power"), run: "run-1" })).toBe("id-invalid");
    expect(commandRejection({ ...open("power"), learner: "ada" })).toBe("id-invalid");
    expect(commandRejection({ ...open("power"), id: "open:1" })).toBe("id-invalid");
  });

  it("refuse like a malformed id what the Rust twin cannot decode: a challenge that is none of the four, an instant that is no timestamp", () => {
    for (const challenge of CHALLENGES) expect(commandRejection(start(0, RUN, "physics", challenge))).toBeUndefined();
    for (const challenge of ["extreme", "Easy", "", 3, null, undefined]) expect(commandRejection({ ...start(0), challenge } as unknown as Command), String(challenge)).toBe("id-invalid");
    expect(commandRejection({ type: "start-run", id: id(1000), learner: ADA, run: RUN, quiz: "physics", at: 0 } as unknown as Command)).toBe("id-invalid");
    expect(commandRejection({ type: "start-run", id: id(1000), learner: ADA, run: RUN, quiz: "physics", challenge: "hard" } as unknown as Command)).toBe("id-invalid");
    expect(MAX_TIMESTAMP).toBe(2 ** 53 - 1);
    for (const at of [0, 1, LATE, MAX_TIMESTAMP]) {
      expect(isTimestamp(at), String(at)).toBe(true);
      expect(commandRejection(start(at)), String(at)).toBeUndefined();
      expect(commandRejection(open("power", at)), String(at)).toBeUndefined();
      expect(commandRejection(record("power", PERFECT_SORTING, RUN, at)), String(at)).toBeUndefined();
    }
    for (const at of [12.5, -1, -0.5, MAX_TIMESTAMP + 1, 2 ** 64, Infinity, Number.NaN, "1000", null, undefined]) {
      expect(isTimestamp(at), String(at)).toBe(false);
      expect(commandRejection({ ...start(0), at } as unknown as Command), String(at)).toBe("id-invalid");
      expect(commandRejection({ ...open("power"), at } as unknown as Command), String(at)).toBe("id-invalid");
      expect(commandRejection({ ...record("power", PERFECT_SORTING), at } as unknown as Command), String(at)).toBe("id-invalid");
      expect(decideLearner(ada([[start(1_000, RUN, "physics", "expert"), context(1_000)]]), { ...open("power"), at } as unknown as Command, context(2_000)), String(at)).toEqual({ rejection: "id-invalid" });
    }
    const unstamped: Record<string, unknown> = { ...open("power") };
    delete unstamped.at;
    expect(commandRejection(unstamped as unknown as Command)).toBe("id-invalid");
    expect(commandRejection(submit())).toBeUndefined();
  });

  it("refuse a query that carries a malformed id", () => {
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
    expect(decideLearner(ada([]), { ...start(10), run: "run-1" }, context(10))).toEqual({ rejection: "id-invalid" });
    expect(decideLearner(emptyLearnerState("ada"), { ...start(10), learner: "ada" }, context(10))).toEqual({ rejection: "id-invalid" });
    expect(decideLearner(ada([[start(10), context(10)]]), { ...record("power", PERFECT_SORTING), task: "Power" }, context(11))).toEqual({ rejection: "id-invalid" });
    expect(decideLearner(ada([[start(10, RUN, "physics", "expert"), context(10)]]), { ...open("power"), task: "Power" }, context(11))).toEqual({ rejection: "id-invalid" });
    expect(decideLearner(ada([[start(10, RUN, "physics", "expert"), context(10)]]), { ...open("power"), learner: BOB }, context(11))).toEqual({ rejection: "unknown-learner" });
  });

  it("counts every started run against the caps, so switching the challenge back and forth stops", () => {
    for (const limits of [{ learners: 10, runsPerQuiz: 4, runs: 100, answersPerRun: 3 }, { learners: 10, runsPerQuiz: 100, runs: 4, answersPerRun: 3 }]) {
      const capped = (now: number): LearnerContext => ({ ...context(now), limits });
      const state = (["easy", "medium", "easy", "medium"] as const).reduce((folded, challenge, index) => fold(folded, events(decideLearner(folded, start(10 + index, id(200 + index), "physics", challenge), capped(10 + index)))), ada([]));
      expect(state.runs.map((run) => run.status)).toEqual(["voided", "voided", "voided", "open"]);
      expect(decideLearner(state, start(20, id(210), "physics", "hard"), capped(20)), JSON.stringify(limits)).toEqual({ rejection: "runs-exhausted" });
    }
  });

  it("starts a run at the instant the device claims, however late it is decided, and raises its openings to that start", () => {
    for (const now of [1_000, 74_000, 3_600_000]) {
      const state = ada([[start(1_000, RUN, "physics", "expert"), context(now)]]);
      expect(state.runs[0]!.startedAt, String(now)).toBe(1_000);
      expect(events(decideLearner(state, open("power", 0), context(now)))[0]!.at).toBe(1_000);
      const opened = fold(state, events(decideLearner(state, open("power", 6_000), context(now))));
      expect(decideLearner(opened, record("power", GUESSED_SORTING, RUN, 6_000 + SECONDS.power * 1000 + 1), context(now + 1))).toEqual({ rejection: "time-up" });
      expect(decideLearner(opened, record("power", GUESSED_SORTING, RUN, 6_000 + SECONDS.power * 1000), context(now + 1))).toHaveProperty("events");
    }
    expect(decideLearner(ada([]), start(MAX_TIMESTAMP + 1), context(10))).toEqual({ rejection: "id-invalid" });
  });

  it("caps the started runs per quiz and in total, and the answers recorded per run", () => {
    const capped = (now: number): LearnerContext => ({ ...context(now), limits: { learners: 10, runsPerQuiz: 1, runs: 2, answersPerRun: 3 } });
    const second = id(101);
    const state = ada([
      [start(10), capped(10)],
      [record("power", PERFECT_SORTING), capped(11)],
      [record("units", PERFECT_UNITS), capped(12)],
      [record("units", PERFECT_UNITS), capped(13)],
    ]);
    expect(state.runs[0]!.recorded).toBe(3);
    expect(decideLearner(state, record("units", PERFECT_UNITS), capped(14))).toEqual({ rejection: "answers-exhausted" });
    const submitted = fold(state, events(decideLearner(state, submit(), capped(15))));
    expect(decideLearner(submitted, start(16, second), capped(16))).toEqual({ rejection: "runs-exhausted" });
    const other = fold(submitted, events(decideLearner(submitted, start(17, second, "heating"), capped(17))));
    const played = [record("power", PERFECT_SORTING, second), record("units", PERFECT_UNITS, second), submit(second)].reduce((folded, command, index) => fold(folded, events(decideLearner(folded, command, capped(18 + index)))), other);
    expect(decideLearner(played, start(30, id(102), "heating"), capped(30))).toEqual({ rejection: "runs-exhausted" });
    expect(decideLearner(played, start(30, id(102), "heating"), context(30))).toHaveProperty("events");
  });

  it("rejects runs of unregistered learners and unknown quizzes", () => {
    expect(decideLearner(emptyLearnerState(ADA), start(10), context(10))).toEqual({ rejection: "unknown-learner" });
    expect(decideLearner(ada([]), { ...start(10), learner: BOB }, context(10))).toEqual({ rejection: "unknown-learner" });
    expect(decideLearner(ada([]), start(10, RUN, "cooling"), context(10))).toEqual({ rejection: "unknown-quiz" });
  });

  it("starts a run at the command's challenge against the current revision with the seed of its id", () => {
    for (const challenge of CHALLENGES) {
      const state = ada([]);
      expect(decideLearner(state, start(10, RUN, "physics", challenge), context(10))).toEqual({ events: [{ type: "run-started", learner: ADA, run: RUN, quiz: "physics", challenge, revision: REVISION, seed: runSeed(RUN), at: 10 }] });
      const started = ada([[start(10, RUN, "physics", challenge), context(10)]]);
      expect(started.runs).toEqual([{ run: RUN, quiz: "physics", challenge, revision: REVISION, seed: runSeed(RUN), status: "open", answers: {}, recorded: 0, opened: {}, startedAt: 10 }]);
      expect(Object.keys(started.runs[0]!)).toEqual(["run", "quiz", "challenge", "revision", "seed", "status", "answers", "recorded", "opened", "startedAt"]);
    }
  });

  it("allows one open run per quiz and never reuses a run id", () => {
    const started = ada([[start(10), context(10)]]);
    expect(decideLearner(started, start(11, id(101)), context(11))).toEqual({ rejection: "run-open" });
    expect(decideLearner(started, start(11, RUN, "heating"), context(11))).toEqual({ rejection: "run-open" });
    expect(decideLearner(started, start(11, RUN, "physics", "hard"), context(11))).toEqual({ rejection: "run-open" });
    expect(events(decideLearner(started, start(11, id(101), "heating"), context(11)))).toHaveLength(1);
  });

  it("resumes the open run of a quiz at its own challenge and voids it for a start at another one", () => {
    const second = id(101);
    for (const challenge of CHALLENGES) {
      const started = ada([[start(10, RUN, "physics", challenge), context(10)]]);
      for (const other of CHALLENGES) {
        const decision = decideLearner(started, start(11, second, "physics", other), context(11));
        if (other === challenge) expect(decision).toEqual({ rejection: "run-open" });
        else
          expect(decision).toEqual({
            events: [
              { type: "run-voided", learner: ADA, run: RUN, at: 11 },
              { type: "run-started", learner: ADA, run: second, quiz: "physics", challenge: other, revision: REVISION, seed: runSeed(second), at: 11 },
            ],
          });
      }
    }
    const switched = ada([
      [start(10, RUN, "physics", "expert"), context(10)],
      [open("power"), context(11)],
      [start(12, second, "physics", "easy"), context(12)],
    ]);
    expect(switched.runs.map((run) => [run.run, run.challenge, run.status])).toEqual([
      [RUN, "expert", "voided"],
      [second, "easy", "open"],
    ]);
    expect(decideLearner(switched, record("power", GUESSED_SORTING), context(13))).toEqual({ rejection: "run-closed" });
    expect(decideLearner(switched, open("units"), context(13))).toEqual({ rejection: "run-closed" });
    expect(decideLearner(switched, submit(), context(13))).toEqual({ rejection: "run-closed" });
    expect(decideLearner(switched, start(13, id(102), "physics", "easy"), context(13))).toEqual({ rejection: "run-open" });
    expect(events(decideLearner(switched, start(13, id(102), "heating", "hard"), context(13)))).toHaveLength(1);
  });

  it("holds a start at another challenge to the caps before it voids anything", () => {
    const capped = (now: number): LearnerContext => ({ ...context(now), limits: { ...DEFAULT_LIMITS, runsPerQuiz: 1 } });
    const second = id(101);
    const state = ada([
      [start(10), capped(10)],
      [record("power", PERFECT_SORTING), capped(11)],
      [record("units", PERFECT_UNITS), capped(12)],
      [submit(), capped(13)],
      [start(14, second, "physics", "easy"), context(14)],
    ]);
    expect(decideLearner(state, start(15, id(102), "physics", "hard"), capped(15))).toEqual({ rejection: "runs-exhausted" });
    expect(decideLearner(state, start(15, id(102), "physics", "easy"), capped(15))).toEqual({ rejection: "run-open" });
    expect(events(decideLearner(state, start(15, id(102), "physics", "hard"), context(15))).map((event) => event.type)).toEqual(["run-voided", "run-started"]);
  });

  it("records valid answers to tasks of the sheet, the latest winning", () => {
    const started = ada([[start(10), context(10)]]);
    expect(decideLearner(started, record("power", PERFECT_SORTING, id(404)), context(11))).toEqual({ rejection: "unknown-run" });
    expect(decideLearner(started, record("essay", PERFECT_SORTING), context(11))).toEqual({ rejection: "unknown-task" });
    expect(decideLearner(started, record("power", { kind: "sorting", order: ["bulb"] }), context(11))).toEqual({ rejection: "answer-invalid" });
    expect(decideLearner(started, record("power", PERFECT_UNITS), context(11))).toEqual({ rejection: "answer-invalid" });
    expect(decideLearner(started, record("units", { kind: "classification", assignments: { watt: "power" } }, RUN, 11), context(11))).toEqual({ events: [{ type: "answer-recorded", learner: ADA, run: RUN, task: "units", answer: { kind: "classification", assignments: { watt: "power" } }, at: 11 }] });
    const answered = ada([
      [start(10), context(10)],
      [record("units", { kind: "classification", assignments: { watt: "energy" } }), context(11)],
      [record("units", PERFECT_UNITS), context(12)],
    ]);
    expect(answered.runs[0]!.answers).toEqual({ units: PERFECT_UNITS });
  });

  it("submits only complete runs, scores them and awards the badges they newly earn", () => {
    const partial = ada([
      [start(10), context(10)],
      [record("units", { kind: "classification", assignments: { watt: "power" } }), context(11)],
      [record("power", PERFECT_SORTING), context(12)],
    ]);
    expect(decideLearner(partial, submit(), context(13))).toEqual({ rejection: "run-incomplete" });
    const complete = fold(partial, events(decideLearner(partial, record("units", PERFECT_UNITS), context(14))));
    const result = scoreRun(QUIZ, sheetOf(QUIZ, runSeed(RUN), "medium"), { power: PERFECT_SORTING, units: PERFECT_UNITS });
    expect(result).toMatchObject({ challenge: "medium", score: 1, points: 200 });
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
    expect(decideLearner(submitted, start(16), context(16))).toEqual({ rejection: "run-closed" });
  });

  it("awards held badges only once and completes the catalog with a second quiz", () => {
    const second = id(101);
    const state = ada([
      [start(10), context(10)],
      [record("power", PERFECT_SORTING), context(11)],
      [record("units", PERFECT_UNITS), context(12)],
      [submit(), context(13)],
      [start(20, second, "heating"), context(20)],
      [record("power", { kind: "sorting", order: ["plant", "kettle", "bulb"] }, second), context(21)],
      [record("units", PERFECT_UNITS, second), context(22)],
    ]);
    const decided = events(decideLearner(state, submit(second), context(23)));
    expect(decided.map((event) => (event.type === "badge-awarded" ? event.badge : event.type))).toEqual(["run-submitted", "completionist"]);
  });

  it("rejects answers to a revised quiz and voids its open run on submission or restart", () => {
    const started = ada([
      [start(10), context(10)],
      [record("power", PERFECT_SORTING), context(11)],
    ]);
    expect(decideLearner(started, record("units", PERFECT_UNITS), context(12, REVISED))).toEqual({ rejection: "quiz-revised" });
    expect(decideLearner(started, submit(), context(12, REVISED))).toEqual({ events: [{ type: "run-voided", learner: ADA, run: RUN, at: 12 }] });
    const restart = id(102);
    expect(decideLearner(started, start(13, restart), context(13, REVISED))).toEqual({
      events: [
        { type: "run-voided", learner: ADA, run: RUN, at: 13 },
        { type: "run-started", learner: ADA, run: restart, quiz: "physics", challenge: "medium", revision: REVISED, seed: runSeed(restart), at: 13 },
      ],
    });
    expect(events(decideLearner(started, start(13, restart, "physics", "expert"), context(13, REVISED))).map((event) => event.type)).toEqual(["run-voided", "run-started"]);
    expect(decideLearner(ada([[start(10, RUN, "physics", "expert"), context(10)]]), open("power"), context(12, REVISED))).toEqual({ rejection: "quiz-revised" });
    const restarted = fold(started, events(decideLearner(started, start(13, restart), context(13, REVISED))));
    expect(restarted.runs.map((run) => [run.run, run.status])).toEqual([
      [RUN, "voided"],
      [restart, "open"],
    ]);
    expect(decideLearner(restarted, record("power", PERFECT_SORTING), context(14, REVISED))).toEqual({ rejection: "run-closed" });
  });

  it("rejects a complete run that cannot be scored as incomplete instead of throwing", () => {
    const broken: Quiz = {
      ...QUIZ,
      tasks: [{ kind: "matching", id: "walls", title: T("Walls"), prompt: T("Match"), dimensions: [{ id: "u", quantity: { label: T("U"), unit: "W/(m²K)", scale: "linear", prefixed: false, additive: false } }], items: [{ id: "old", label: T("Old"), values: { u: 1.4 } }, { id: "new", label: T("New"), values: {} }] }],
    };
    const brokenContext = (now: number): LearnerContext => ({ ...context(now), quizzes: { ...context(now).quizzes, physics: { quiz: broken, revision: REVISION } } });
    const state = ada([
      [start(10), brokenContext(10)],
      [record("walls", { kind: "matching", assignments: { u: { old: 0, new: 1 } } }), brokenContext(11)],
    ]);
    expect(state.runs[0]!.answers).toHaveProperty("walls");
    expect(decideLearner(state, submit(), brokenContext(12))).toEqual({ rejection: "run-incomplete" });
  });

  it("folds only its own events", () => {
    const state = ada([]);
    expect(evolveLearner(state, { type: "run-started", learner: BOB, run: RUN, quiz: "physics", challenge: "medium", revision: REVISION, seed: runSeed(RUN), at: 99 })).toBe(state);
    expect(evolveLearner(state, { type: "learner-registered", learner: BOB, identity: { kind: "anonymous" }, at: 99 })).toBe(state);
    const timed = ada([[start(10, RUN, "physics", "expert"), context(10)]]);
    expect(evolveLearner(timed, { type: "task-opened", learner: BOB, run: RUN, task: "power", at: 99 })).toBe(timed);
    expect(state.identity).toEqual({ kind: "pseudonym", handle: "Ada" });
  });
});

describe("learner — the device keeps the time", () => {
  const expert = (): LearnerState => ada([[start(1_000, RUN, "physics", "expert"), context(1_000)]]);

  it("raises the instant of an answer of an untimed run to the run's start and lowers it to the lead past the decision time", () => {
    for (const challenge of ["easy", "medium", "hard"] as const) {
      const started = ada([[start(1_000, RUN, "physics", challenge), context(1_000)]]);
      const recorded = (at: number, now: number) => events(decideLearner(started, record("units", PERFECT_UNITS, RUN, at), context(now)))[0]!.at;
      expect(recorded(1_500, 2_000)).toBe(1_500);
      expect(recorded(1_000, 2_000)).toBe(1_000);
      expect(recorded(999, 2_000)).toBe(1_000);
      expect(recorded(0, 2_000)).toBe(1_000);
      expect(recorded(2_001, 2_000)).toBe(2_001);
      expect(recorded(LATE, 2_000)).toBe(2_000 + CLOCK_LEAD);
      expect(recorded(LATE, LATE)).toBe(LATE);
      expect(recorded(2_000 + CLOCK_LEAD, 2_000)).toBe(2_000 + CLOCK_LEAD);
      expect(recorded(5_000, 900)).toBe(5_000);
      expect(recorded(500, 900)).toBe(1_000);
    }
  });

  it("opens a task of a timed run once, at the instant the learner acted raised to the run's start and lowered to the lead past the decision time", () => {
    const opened = (at: number, now: number) => decideLearner(expert(), open("power", at), context(now));
    expect(opened(1_500, 2_000)).toEqual({ events: [{ type: "task-opened", learner: ADA, run: RUN, task: "power", at: 1_500 }] });
    expect(events(opened(500, 2_000))[0]!.at).toBe(1_000);
    expect(events(opened(LATE, 2_000))[0]!.at).toBe(2_000 + CLOCK_LEAD);
    expect(events(opened(LATE, LATE))[0]!.at).toBe(LATE);
    expect(events(opened(1_000, 1_000))[0]!.at).toBe(1_000);
    expect(events(opened(1_500, 900))[0]!.at).toBe(1_500);
    const state = ada([
      [start(1_000, RUN, "physics", "expert"), context(1_000)],
      [open("power", 1_500), context(2_000)],
    ]);
    expect(state.runs[0]!.opened).toEqual({ power: 1_500 });
    for (const again of [1_900, 1_500, 0]) expect(decideLearner(state, open("power", again), context(2_100)), String(again)).toEqual({ rejection: "already-opened" });
    const both = fold(state, events(decideLearner(state, open("units", 1_800), context(2_100))));
    expect(both.runs[0]!.opened).toEqual({ power: 1_500, units: 1_800 });
    expect(decideLearner(both, open("power", 2_000), context(2_200))).toEqual({ rejection: "already-opened" });
  });

  it("refuses to open a task of an unknown, closed, stale or untimed run, and a task the sheet does not present", () => {
    expect(decideLearner(expert(), open("power", 1_500, id(404)), context(2_000))).toEqual({ rejection: "unknown-run" });
    expect(decideLearner(expert(), open("essay"), context(2_000))).toEqual({ rejection: "unknown-task" });
    expect(decideLearner(expert(), open("power"), context(2_000, REVISED))).toEqual({ rejection: "quiz-revised" });
    for (const challenge of ["easy", "medium", "hard"] as const) {
      const untimed = ada([[start(1_000, RUN, "physics", challenge), context(1_000)]]);
      expect(decideLearner(untimed, open("power"), context(2_000))).toEqual({ rejection: "run-untimed" });
      expect(decideLearner(untimed, open("essay"), context(2_000))).toEqual({ rejection: "run-untimed" });
      expect(untimed.runs[0]!.opened).toEqual({});
    }
    const submitted = ada([
      [start(1_000, RUN, "physics", "expert"), context(1_000)],
      [submit(), context(1_100)],
    ]);
    expect(decideLearner(submitted, open("power"), context(2_000))).toEqual({ rejection: "run-closed" });
    const voided = ada([
      [start(1_000, RUN, "physics", "expert"), context(1_000)],
      [start(1_100, id(101), "physics", "hard"), context(1_100)],
    ]);
    expect(decideLearner(voided, open("power"), context(2_000))).toEqual({ rejection: "run-closed" });
    expect(decideLearner(voided, open("power", 1_500, id(101)), context(2_000))).toEqual({ rejection: "run-untimed" });
  });

  it("takes an answer of a timed run only for an opened task", () => {
    expect(decideLearner(expert(), record("power", GUESSED_SORTING, RUN, 1_500), context(2_000))).toEqual({ rejection: "task-unopened" });
    expect(decideLearner(expert(), record("power", PERFECT_UNITS, RUN, 1_500), context(2_000))).toEqual({ rejection: "task-unopened" });
    expect(decideLearner(expert(), record("essay", GUESSED_SORTING, RUN, 1_500), context(2_000))).toEqual({ rejection: "unknown-task" });
    const state = ada([
      [start(1_000, RUN, "physics", "expert"), context(1_000)],
      [open("power", 1_500), context(2_000)],
    ]);
    expect(decideLearner(state, record("units", PERFECT_UNITS, RUN, 1_600), context(2_000))).toEqual({ rejection: "task-unopened" });
    expect(decideLearner(state, record("power", GUESSED_SORTING, RUN, 1_600), context(2_000))).toEqual({ events: [{ type: "answer-recorded", learner: ADA, run: RUN, task: "power", answer: GUESSED_SORTING, at: 1_600 }] });
  });

  it("takes an answer until the task's seconds have passed since it was opened, counted on the device's instants alone", () => {
    const state = ada([
      [start(1_000, RUN, "physics", "expert"), context(1_000)],
      [open("power", 1_500), context(2_000)],
      [open("units", 1_700), context(2_000)],
    ]);
    const limit = 1_500 + SECONDS.power * 1000;
    const decided = (at: number, now: number) => decideLearner(state, record("power", GUESSED_SORTING, RUN, at), context(now));
    const recordedAt = (at: number, now: number) => events(decided(at, now))[0]!.at;
    for (const now of [1_400, limit - 1, limit, limit + 5, LATE - 1, LATE + 10 ** 9]) {
      expect(recordedAt(limit, now)).toBe(limit);
      expect(recordedAt(limit - 1, now)).toBe(limit - 1);
      expect(recordedAt(1_600, now)).toBe(1_600);
      expect(recordedAt(100, now)).toBe(1_500);
      expect(decided(limit + 1, now)).toEqual({ rejection: "time-up" });
      expect(decided(LATE, now)).toEqual({ rejection: "time-up" });
    }
    const units = 1_700 + SECONDS.units * 1000;
    expect(events(decideLearner(state, record("units", PERFECT_UNITS, RUN, units), context(units + 9)))[0]!.at).toBe(units);
    expect(decideLearner(state, record("units", PERFECT_UNITS, RUN, units + 1), context(units + 9))).toEqual({ rejection: "time-up" });
    expect(SECONDS).toEqual({ power: 66, units: 46 });
  });

  it("decides alike on the device and, after a connection shortage, on the proctor", () => {
    const steps: readonly [Command, number][] = [
      [start(1_000, RUN, "physics", "expert"), 1_000],
      [open("power", 1_200), 1_200],
      [record("power", GUESSED_SORTING, RUN, 20_000), 20_000],
      [open("units", 30_000), 30_000],
      [record("units", PERFECT_UNITS, RUN, 80_000), 80_000],
      [record("units", PERFECT_UNITS, RUN, 60_000), 80_001],
    ];
    const replay = (delay: number): Decision[] => {
      let state = ada([]);
      return steps.map(([command, now], index) => {
        const decision = decideLearner(state, command, context(now + delay));
        if ("events" in decision) state = fold(state, decision.events);
        return decision;
      });
    };
    const device = replay(0);
    expect(device[4]).toEqual({ rejection: "time-up" });
    expect(device[5]).toEqual({ events: [{ type: "answer-recorded", learner: ADA, run: RUN, task: "units", answer: PERFECT_UNITS, at: 60_000 }] });
    expect(replay(3_600_000)).toEqual(device);
  });

  it("gives the same verdict whether the device's clock runs ahead of the decider's or behind it, delivered at once or late", () => {
    const limit = SECONDS.power * 1000;
    for (const skew of [-120_000, -20_000, 0, 20_000, 120_000]) {
      for (const delay of [0, 150_000, 3_600_000]) {
        const state = ada([
          [start(1_000_000, RUN, "physics", "expert"), context(1_000_000)],
          [open("power", 1_010_000 + skew), context(1_010_000)],
        ]);
        const opened = state.runs[0]!.opened.power!;
        expect(opened).toBe(Math.max(1_010_000 + skew, 1_000_000));
        const answer = (elapsed: number) => decideLearner(state, record("power", GUESSED_SORTING, RUN, opened + elapsed), context(1_010_000 + elapsed + delay));
        expect(answer(40_000)).toEqual({ events: [{ type: "answer-recorded", learner: ADA, run: RUN, task: "power", answer: GUESSED_SORTING, at: opened + 40_000 }] });
        expect(events(answer(limit))[0]!.at).toBe(opened + limit);
        expect(answer(limit + 1)).toEqual({ rejection: "time-up" });
      }
    }
  });

  it("lowers a start, an opening and an answer dated more than five minutes ahead of the decider to that lead, so a claimed hour buys no time", () => {
    const now = 1_000_000;
    const state = ada([
      [start(now + 3_600_000, RUN, "physics", "expert"), context(now)],
      [open("power", now + 3_600_000), context(now + 1_000)],
    ]);
    expect([state.runs[0]!.startedAt, state.runs[0]!.opened.power]).toEqual([now + CLOCK_LEAD, now + 1_000 + CLOCK_LEAD]);
    expect(events(decideLearner(state, record("power", GUESSED_SORTING, RUN, now + 3_600_000), context(now + 2_000)))[0]!.at).toBe(now + 2_000 + CLOCK_LEAD);
    expect(decideLearner(state, record("power", GUESSED_SORTING, RUN, now + 3_600_000 + 1_000), context(now + 1_800_000))).toEqual({ rejection: "time-up" });
    const honest = ada([
      [start(now + 240_000, RUN, "physics", "expert"), context(now)],
      [open("power", now + 245_000), context(now + 1_000)],
    ]);
    expect([honest.runs[0]!.startedAt, honest.runs[0]!.opened.power]).toEqual([now + 240_000, now + 245_000]);
  });

  it("refuses an answer after time is up before it looks at the answer, and an unopened task before the time", () => {
    const state = ada([
      [start(1_000, RUN, "physics", "expert"), context(1_000)],
      [open("power", 1_500), context(2_000)],
    ]);
    const late = 1_500 + SECONDS.power * 1000 + 1;
    expect(decideLearner(state, record("power", PERFECT_UNITS, RUN, late), context(late))).toEqual({ rejection: "time-up" });
    expect(decideLearner(state, record("power", PERFECT_UNITS, RUN, 1_600), context(late))).toEqual({ rejection: "answer-invalid" });
    expect(decideLearner(state, record("power", { kind: "sorting", order: ["bulb", "kettle", "plant"], guesses: { bulb: 5, kettle: 1 } }, RUN, 1_600), context(late))).toEqual({ rejection: "answer-invalid" });
    const capped: LearnerContext = { ...context(late), limits: { ...DEFAULT_LIMITS, answersPerRun: 0 } };
    expect(decideLearner(state, record("power", GUESSED_SORTING, RUN, late), capped)).toEqual({ rejection: "answers-exhausted" });
    expect(decideLearner(state, record("units", PERFECT_UNITS, RUN, late), capped)).toEqual({ rejection: "answers-exhausted" });
  });

  it("submits a timed run as it stands: never incomplete, what is missing scored as a miss", () => {
    const unanswered = decideLearner(expert(), submit(), context(1_100));
    const empty = events(unanswered)[0] as RunSubmittedEvent;
    expect(events(unanswered).map((event) => event.type)).toEqual(["run-submitted"]);
    expect(empty.result).toMatchObject({ quiz: "physics", challenge: "expert", score: 0, points: 0 });
    expect(empty.result).toEqual(scoreRun(QUIZ, sheetOf(QUIZ, runSeed(RUN), "expert"), {}));
    const partial = ada([
      [start(1_000, RUN, "physics", "expert"), context(1_000)],
      [open("power", 1_500), context(2_000)],
      [record("power", GUESSED_SORTING, RUN, 1_600), context(2_000)],
      [open("units", 1_700), context(2_000)],
      [record("units", { kind: "classification", assignments: { kwh: "energy" } }, RUN, 1_800), context(2_000)],
    ]);
    const decided = events(decideLearner(partial, submit(), context(900_000)));
    const submitted = decided[0] as RunSubmittedEvent;
    expect(submitted.result).toMatchObject({ challenge: "expert", score: 0.75, points: 300 });
    expect(submitted.at).toBe(900_000);
    expect(decided.map((event) => (event.type === "badge-awarded" ? event.badge : event.type))).toEqual(["run-submitted", "sorter"]);
    const whole = fold(partial, events(decideLearner(partial, record("units", PERFECT_UNITS, RUN, 1_900), context(2_000))));
    expect(events(decideLearner(whole, submit(), context(3_000))).map((event) => (event.type === "badge-awarded" ? event.badge : event.type))).toEqual(["run-submitted", "perfect-physics", "sorter", "guesser"]);
    expect((events(decideLearner(whole, submit(), context(3_000)))[0] as RunSubmittedEvent).result).toMatchObject({ score: 1, points: 400 });
  });
});

describe("learner — the challenge of a run", () => {
  it("rebuilds the sheet of every decision at the run's challenge: guesses where the keys are hidden, none where they show", () => {
    for (const challenge of CHALLENGES) {
      const timed = challenge === "expert";
      const hidden = challenge === "hard" || timed;
      const started = ada([[start(10, RUN, "physics", challenge), context(10)], ...(timed ? [[open("power"), context(10)] as [Command, LearnerContext]] : [])]);
      expect("events" in decideLearner(started, record("power", GUESSED_SORTING), context(11))).toBe(hidden);
      expect(decideLearner(started, record("power", PERFECT_SORTING), context(11))).toHaveProperty("events");
      expect(decideLearner(started, record("power", { kind: "sorting", order: ["bulb", "kettle", "plant"], guesses: {} }), context(11))).toEqual(hidden ? { events: [expect.objectContaining({ type: "answer-recorded" })] } : { rejection: "answer-invalid" });
    }
  });

  it("submits a hard run only when every item has a guess, and earns more points for the same accuracy than a medium one", () => {
    const hard = ada([
      [start(10, RUN, "physics", "hard"), context(10)],
      [record("power", PERFECT_SORTING), context(11)],
      [record("units", PERFECT_UNITS), context(12)],
    ]);
    expect(decideLearner(hard, submit(), context(13))).toEqual({ rejection: "run-incomplete" });
    const some = fold(hard, events(decideLearner(hard, record("power", { kind: "sorting", order: ["bulb", "kettle", "plant"], guesses: { bulb: 60, plant: 1e9 } }), context(13))));
    expect(decideLearner(some, submit(), context(14))).toEqual({ rejection: "run-incomplete" });
    const guessed = fold(some, events(decideLearner(some, record("power", GUESSED_SORTING), context(14))));
    const decided = events(decideLearner(guessed, submit(), context(15)));
    expect((decided[0] as RunSubmittedEvent).result).toMatchObject({ challenge: "hard", score: 1, points: 300 });
    expect((decided[0] as RunSubmittedEvent).result).toEqual(scoreRun(QUIZ, sheetOf(QUIZ, runSeed(RUN), "hard"), { power: GUESSED_SORTING, units: PERFECT_UNITS }));
    expect(decided.map((event) => (event.type === "badge-awarded" ? event.badge : event.type))).toEqual(["run-submitted", "perfect-physics", "sorter", "guesser"]);
  });

  it("awards a badge that asks for a least challenge only for a run that meets it, also after an easier perfect run", () => {
    const second = id(101);
    const medium = ada([
      [start(10), context(10)],
      [record("power", PERFECT_SORTING), context(11)],
      [record("units", PERFECT_UNITS), context(12)],
      [submit(), context(13)],
    ]);
    expect(medium.badges.map((award) => award.badge)).toEqual(["perfect-physics", "sorter"]);
    const hard = [start(20, second, "physics", "hard"), record("power", GUESSED_SORTING, second), record("units", PERFECT_UNITS, second)].reduce((state, command, index) => fold(state, events(decideLearner(state, command, context(20 + index)))), medium);
    expect(events(decideLearner(hard, submit(second), context(30)))).toEqual([expect.objectContaining({ type: "run-submitted" }), { type: "badge-awarded", learner: ADA, badge: "guesser", run: second, at: 30 }]);
  });

  it("scores a far-off guess as a miss in the submitted result", () => {
    const state = ada([
      [start(10, RUN, "physics", "hard"), context(10)],
      [record("power", { kind: "sorting", order: ["bulb", "kettle", "plant"], guesses: { bulb: 60, kettle: 2000, plant: 3000 } }), context(11)],
      [record("units", PERFECT_UNITS), context(12)],
    ]);
    const result = (events(decideLearner(state, submit(), context(13)))[0] as RunSubmittedEvent).result;
    const power = result.tasks.find((task) => task.task === "power");
    if (power?.kind !== "sorting") throw new Error("kind");
    expect(power.items.map((item) => [item.item, item.guess, item.miss])).toEqual([
      ["bulb", 60, false],
      ["kettle", 2000, false],
      ["plant", 3000, true],
    ]);
    expect(power.score).toBeLessThan(0.3);
    expect(result.points).toBe(result.score * 300);
    expect(events(decideLearner(state, submit(), context(13))).map((event) => event.type)).toEqual(["run-submitted"]);
  });
});
