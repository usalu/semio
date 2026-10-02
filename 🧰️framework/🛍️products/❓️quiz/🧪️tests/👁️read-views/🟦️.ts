import { describe, expect, it } from "vitest";
import {
  boardScope,
  catalogView,
  emptyLearnerState,
  evolveLearner,
  fnv1a32,
  leaderboard,
  learnerTag,
  learnerView,
  periodWindow,
  runSeed,
  runView,
  sheetOf,
  standing,
  transcript,
  LEADERBOARD_TOP,
  type Catalog,
  type Event,
  type Leaderboard,
  type LeaderboardPeriod,
  type LearnerState,
  type Quiz,
  type Text,
  type Transcript,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

const T = (en: string): Text => ({ en, de: `${en} (de)` });
const id = (n: number) => n.toString(16).padStart(32, "0");
const REVISION = "c".repeat(64);

const PHYSICS: Quiz = {
  schema: "semio.quiz/v1",
  id: "physics",
  emoji: "🧲",
  title: T("Physics"),
  description: T("Power and energy"),
  tasks: [
    { kind: "sorting", id: "power", title: T("Power"), prompt: T("Sort"), quantity: { label: T("Power"), unit: "W", scale: "logarithmic", prefixed: true }, items: [{ id: "bulb", label: T("Bulb"), value: 60, explanation: T("hot") }, { id: "plant", label: T("Plant"), value: 1e9 }] },
    { kind: "classification", id: "units", title: T("Units"), prompt: T("Classify"), categories: [{ id: "power", label: T("Power"), profile: { a: 1, b: 2, c: 3 } }, { id: "energy", label: T("Energy") }], axes: [{ id: "a", label: T("a"), unit: "u", min: 0, max: 5 }, { id: "b", label: T("b"), unit: "u", min: 0, max: 5 }, { id: "c", label: T("c"), unit: "u", min: 0, max: 5 }], items: [{ id: "watt", label: T("W"), category: "power" }, { id: "kwh", label: T("kWh"), category: "energy" }] },
  ],
};
const HEATING: Quiz = { schema: "semio.quiz/v1", id: "heating", emoji: "🔥", title: T("Heating"), description: T("Heat"), tasks: [{ kind: "matching", id: "walls", title: T("Walls"), prompt: T("Match"), dimensions: [{ id: "u", quantity: { label: T("U"), unit: "W/(m²K)", scale: "linear", prefixed: false } }], items: [{ id: "old", label: T("Old"), values: { u: 1.4 } }, { id: "new", label: T("New"), values: { u: 0.2 } }] }] };
const CATALOG: Catalog = {
  schema: "semio.quiz.catalog/v1",
  id: "architecture",
  title: T("Architecture"),
  introduction: { title: T("Welcome"), paragraphs: [T("First"), T("Second")] },
  quizzes: ["physics.json", "heating.json"],
  badges: [
    { id: "perfect-physics", emoji: "🧲", label: T("Perfect"), description: T("Perfect physics"), rule: { kind: "perfect-quiz", quiz: "physics" } },
    { id: "completionist", emoji: "🏁", label: T("Done"), description: T("Every quiz"), rule: { kind: "completed-quizzes" } },
  ],
};
const VIEW = catalogView(CATALOG, [PHYSICS, HEATING]);

const registered = (learner: string, at = 1): Event => ({ type: "learner-registered", learner, identity: { kind: "name", handle: `Learner ${learner.slice(-2)}` }, at });
const started = (learner: string, run: string, quiz: string, at: number): Event => ({ type: "run-started", learner, run, quiz, revision: REVISION, seed: runSeed(run), at });
const submitted = (learner: string, run: string, quiz: string, score: number, at: number): Event => ({ type: "run-submitted", learner, run, result: { quiz, score, tasks: [] }, at });
const awarded = (learner: string, badge: string, run: string, at: number): Event => ({ type: "badge-awarded", learner, badge, run, at });

/** 🧬️ A learner folded from its events. */
function learner(learnerId: string, events: readonly Event[]): LearnerState {
  return events.reduce(evolveLearner, emptyLearnerState(learnerId));
}

/** 🏃️ A learner who submitted one run per (quiz, score, at) and received the given badges. */
function competitor(n: number, runs: readonly [string, number, number][], badges: readonly string[] = []): LearnerState {
  const learnerId = id(n);
  const events = runs.flatMap(([quiz, score, at], index) => {
    const run = id(n * 100 + index);
    return [started(learnerId, run, quiz, at - 1), submitted(learnerId, run, quiz, score, at)];
  });
  const last = runs.length > 0 ? id(n * 100 + runs.length - 1) : id(0);
  return learner(learnerId, [registered(learnerId), ...events, ...badges.map((badge, index) => awarded(learnerId, badge, last, 1000 + index))]);
}

describe("catalogView", () => {
  it("lists quizzes and badges in catalog order without solutions, paths or rules", () => {
    expect(VIEW).toEqual({
      id: "architecture",
      title: CATALOG.title,
      introduction: CATALOG.introduction,
      quizzes: [
        { id: "physics", emoji: "🧲", title: PHYSICS.title, description: PHYSICS.description, tasks: [{ id: "power", kind: "sorting", title: T("Power") }, { id: "units", kind: "classification", title: T("Units") }] },
        { id: "heating", emoji: "🔥", title: HEATING.title, description: HEATING.description, tasks: [{ id: "walls", kind: "matching", title: T("Walls") }] },
      ],
      badges: [
        { id: "perfect-physics", emoji: "🧲", label: T("Perfect"), description: T("Perfect physics") },
        { id: "completionist", emoji: "🏁", label: T("Done"), description: T("Every quiz") },
      ],
    });
    const serialized = JSON.stringify(VIEW);
    for (const secret of ['"value"', '"values"', '"category"', '"profile"', '"rule"', '"explanation"', "physics.json"]) expect(serialized).not.toContain(secret);
  });
});

describe("learnerView", () => {
  const ada = id(1);
  const [r1, r2, r3, r4] = [id(11), id(12), id(13), id(14)];
  const state = learner(ada, [
    registered(ada),
    started(ada, r1, "physics", 10),
    submitted(ada, r1, "physics", 0.5, 11),
    started(ada, r2, "heating", 20),
    started(ada, r3, "physics", 30),
    submitted(ada, r3, "physics", 0.75, 31),
    { type: "run-voided", learner: ada, run: r2, at: 40 },
    started(ada, r4, "retired", 50),
    submitted(ada, r4, "retired", 1, 51),
    awarded(ada, "completionist", r4, 51),
  ]);

  it("is undefined before registration", () => {
    expect(learnerView(emptyLearnerState(ada), VIEW)).toBeUndefined();
    expect(learnerView(learner(ada, [started(ada, r1, "physics", 1)]), VIEW)).toBeUndefined();
  });

  it("lists runs newest first, the best score per quiz, badges and the total over the catalog quizzes", () => {
    const view = learnerView(state, VIEW)!;
    expect(view.learner).toBe(ada);
    expect(view.identity).toEqual({ kind: "name", handle: "Learner 01" });
    expect(view.runs).toEqual([
      { run: r4, quiz: "retired", status: "submitted", score: 1, startedAt: 50, submittedAt: 51 },
      { run: r3, quiz: "physics", status: "submitted", score: 0.75, startedAt: 30, submittedAt: 31 },
      { run: r2, quiz: "heating", status: "voided", startedAt: 20 },
      { run: r1, quiz: "physics", status: "submitted", score: 0.5, startedAt: 10, submittedAt: 11 },
    ]);
    expect(view.best).toEqual({ physics: 0.75, retired: 1 });
    expect(Object.keys(view.best)).toEqual(["physics", "retired"]);
    expect(view.badges).toEqual([{ badge: "completionist", run: r4, at: 51 }]);
    expect(view.total).toBe(75);
  });
});

describe("runView", () => {
  const ada = id(1);
  const run = id(21);
  const quizzes = { physics: { quiz: PHYSICS, revision: REVISION } };
  const open = learner(ada, [
    registered(ada),
    started(ada, run, "physics", 5),
    { type: "answer-recorded", learner: ada, run, task: "units", answer: { kind: "classification", assignments: { watt: "power" } }, at: 6 },
    { type: "answer-recorded", learner: ada, run, task: "power", answer: { kind: "sorting", order: ["plant", "bulb"] }, at: 7 },
  ]);

  it("rebuilds the sheet from the run seed and lists the answers by task id", () => {
    const view = runView(open, run, quizzes)!;
    expect(view).toEqual({ run, learner: ada, quiz: "physics", status: "open", sheet: sheetOf(PHYSICS, runSeed(run)), answers: { power: { kind: "sorting", order: ["plant", "bulb"] }, units: { kind: "classification", assignments: { watt: "power" } } }, startedAt: 5 });
    expect(Object.keys(view.answers)).toEqual(["power", "units"]);
    expect(view).not.toHaveProperty("result");
    expect(view).not.toHaveProperty("submittedAt");
  });

  it("carries the result once submitted", () => {
    const result = { quiz: "physics", score: 0.5, tasks: [] };
    const done = evolveLearner(open, { type: "run-submitted", learner: ada, run, result, at: 9 });
    expect(runView(done, run, quizzes)).toMatchObject({ status: "submitted", result, submittedAt: 9 });
  });

  it("is undefined for an unknown run or an unloaded quiz", () => {
    expect(runView(open, id(99), quizzes)).toBeUndefined();
    expect(runView(open, run, {})).toBeUndefined();
  });
});

describe("learnerTag", () => {
  it("is FNV-1a of the learner id as 8 lowercase hex digits, zero-padded", () => {
    for (let n = 0; n < 200; n++) {
      const tag = learnerTag(id(n));
      expect(tag).toMatch(/^[0-9a-f]{8}$/u);
      expect(Number.parseInt(tag, 16)).toBe(fnv1a32(id(n)));
    }
    const small = Array.from({ length: 400 }, (_, n) => id(n)).find((learner) => fnv1a32(learner) < 0x10000000)!;
    expect(learnerTag(small).startsWith("0")).toBe(true);
    expect(learnerTag("")).toBe("811c9dc5");
    expect(learnerTag("a")).toBe("e40c292c");
  });
});

/** 📜️ The transcripts of the ranked learners among `states`. */
function transcripts(states: readonly LearnerState[]): Transcript[] {
  return states.flatMap((state) => transcript(state) ?? []);
}

/** 🏆️ The all-time leaderboard over the transcripts of `states` as `caller` is answered. */
function board(states: readonly LearnerState[], caller?: string): Leaderboard {
  return leaderboard(transcripts(states), VIEW, { period: "all-time" }, 0, caller);
}

const DAY = 86_400_000;
const MONDAY = 20_724 * DAY;

describe("leaderboard", () => {
  it("ranks by total ↓, badge count ↓, reachedAt ↑ and learner id ↑, leaving out learners without submissions", () => {
    const states = [
      competitor(5, [["physics", 1, 30], ["heating", 0.5, 40]], ["perfect-physics", "completionist"]),
      competitor(4, [["physics", 1, 30], ["heating", 0.5, 40]], ["perfect-physics", "completionist"]),
      competitor(3, [["physics", 1, 10], ["heating", 0.5, 20]], ["perfect-physics", "completionist"]),
      competitor(2, [["physics", 1, 10], ["heating", 0.5, 20]], ["perfect-physics"]),
      competitor(1, [["physics", 0.5, 5]]),
      competitor(6, [["heating", 1, 50], ["physics", 0.9, 60]]),
      competitor(7, []),
      learner(id(8), [started(id(8), id(800), "physics", 1), submitted(id(8), id(800), "physics", 1, 2)]),
    ];
    const ranked = board(states);
    expect(ranked.rows.map((row) => [row.rank, row.tag, row.total, row.badges.length, row.reachedAt])).toEqual([
      [1, learnerTag(id(6)), 190, 0, 60],
      [2, learnerTag(id(3)), 150, 2, 20],
      [3, learnerTag(id(4)), 150, 2, 40],
      [4, learnerTag(id(5)), 150, 2, 40],
      [5, learnerTag(id(2)), 150, 1, 20],
      [6, learnerTag(id(1)), 50, 0, 5],
    ]);
    expect(ranked.learners).toBe(6);
    expect(ranked.own).toBeUndefined();
    expect(learnerTag(id(4)) > learnerTag(id(5))).toBe(true);
    expect(ranked.rows[1]).toEqual({ rank: 2, tag: learnerTag(id(3)), identity: { kind: "name", handle: "Learner 03" }, total: 150, reachedAt: 20, best: { heating: 0.5, physics: 1 }, badges: ["perfect-physics", "completionist"], runs: 2, lastActivity: 20 });
    expect(Object.keys(ranked.rows[1]!)).toEqual(["rank", "tag", "identity", "total", "reachedAt", "best", "badges", "runs", "lastActivity"]);
    expect(transcript(competitor(7, []))).toBeUndefined();
    expect(transcript(states[7]!)).toBeUndefined();
    expect(ranked.submissions).toBe(11);
  });

  it("answers the caller's own row when the caller is ranked, also inside the top", () => {
    const states = [1, 2, 3].map((n) => competitor(n, [["physics", n / 4, 10 * n]]));
    expect(board(states, id(1)).own).toEqual(board(states).rows[2]);
    expect(board(states, id(3)).own).toMatchObject({ rank: 1, tag: learnerTag(id(3)) });
    expect(board(states, id(9)).own).toBeUndefined();
    expect(board([...states, competitor(7, [])], id(7)).own).toBeUndefined();
    expect(Object.keys(board(states, id(1)))).toEqual(["period", "rows", "learners", "submissions", "own"]);
    expect(Object.keys(board(states))).toEqual(["period", "rows", "learners", "submissions"]);
    expect(Object.keys(leaderboard(transcripts(states), VIEW, { period: "daily", quiz: "physics" }, 10, id(1)))).toEqual(["period", "quiz", "window", "rows", "learners", "submissions", "own"]);
  });

  it("carries the top rows only, counts every ranked learner and finds the caller beyond the top", () => {
    const crowd = Array.from({ length: LEADERBOARD_TOP + 30 }, (_, n) => competitor(n + 1, [["physics", (n + 1) / 256, 10 + n]]));
    const last = board(crowd, id(1));
    expect(last.rows).toHaveLength(LEADERBOARD_TOP);
    expect(last.rows.map((row) => row.rank)).toEqual(Array.from({ length: LEADERBOARD_TOP }, (_, index) => index + 1));
    expect(last.learners).toBe(LEADERBOARD_TOP + 30);
    expect(last.own).toMatchObject({ rank: LEADERBOARD_TOP + 30, tag: learnerTag(id(1)) });
    expect(last.rows.some((row) => row.tag === learnerTag(id(1)))).toBe(false);
    expect(board(crowd, id(LEADERBOARD_TOP + 30)).own).toEqual(last.rows[0]);
  });

  it("never publishes a learner id", () => {
    const states = [1, 2, 3].map((n) => competitor(n, [["physics", n / 4, 10 * n]]));
    const serialized = JSON.stringify(board(states, states[0]!.learner));
    for (const state of states) {
      expect(serialized).not.toContain(state.learner);
      expect(serialized).toContain(`"tag":"${learnerTag(state.learner)}"`);
    }
  });

  it("moves reachedAt only when a submission raises a best score, lastActivity with every submission", () => {
    const ranked = board([competitor(1, [["physics", 0.5, 10], ["heating", 1, 20], ["physics", 0.4, 30], ["physics", 0.5, 40]])]);
    expect(ranked.rows[0]).toMatchObject({ total: 150, reachedAt: 20, runs: 4, lastActivity: 40 });
    const raised = board([competitor(1, [["physics", 0.5, 10], ["heating", 1, 20], ["physics", 0.6, 30]])]);
    expect(raised.rows[0]).toMatchObject({ total: 160, reachedAt: 30, lastActivity: 30 });
  });

  it("keeps a transcript unchanged by answers and open runs", () => {
    const base = competitor(1, [["physics", 0.5, 10]]);
    const run = id(900);
    const busy = [started(id(1), run, "heating", 50), { type: "answer-recorded", learner: id(1), run, task: "walls", answer: { kind: "matching", assignments: {} }, at: 60 } as Event].reduce(evolveLearner, base);
    expect(transcript(busy)).toEqual(transcript(base));
    expect(transcript(base)).toEqual({ learner: id(1), tag: learnerTag(id(1)), identity: { kind: "name", handle: "Learner 01" }, runs: [{ quiz: "physics", score: 0.5, at: 10 }], badges: [] });
  });

  it("is empty without submissions", () => {
    expect(board([])).toEqual({ period: "all-time", rows: [], learners: 0, submissions: 0 });
    expect(board([competitor(1, [])], id(1))).toEqual({ period: "all-time", rows: [], learners: 0, submissions: 0 });
  });
});

describe("periodWindow", () => {
  const days = (from: number, until: number) => ({ from: from * DAY, until: until * DAY });

  it("spans the UTC day, the ISO week from Monday and the month around an instant, and nothing for all-time", () => {
    const friday = Date.UTC(2026, 9, 2, 10, 16, 40);
    expect(periodWindow("daily", friday)).toEqual({ from: Date.UTC(2026, 9, 2), until: Date.UTC(2026, 9, 3) });
    expect(periodWindow("weekly", friday)).toEqual({ from: Date.UTC(2026, 8, 28), until: Date.UTC(2026, 9, 5) });
    expect(periodWindow("monthly", friday)).toEqual({ from: Date.UTC(2026, 9, 1), until: Date.UTC(2026, 10, 1) });
    expect(periodWindow("all-time", friday)).toBeUndefined();
    expect(periodWindow("weekly", 0)).toEqual(days(0, 4));
    expect(periodWindow("monthly", 0)).toEqual(days(0, 31));
  });

  it("agrees with the platform's calendar on every day of four centuries", () => {
    for (let day = 0; day < 146_097 + 800; day += 1) {
      const at = day * DAY + (day % 7) * 3_600_000;
      const date = new Date(at);
      const monday = Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), date.getUTCDate() - ((date.getUTCDay() + 6) % 7));
      const expected = [
        { from: day * DAY, until: (day + 1) * DAY },
        { from: Math.max(0, monday), until: monday + 7 * DAY },
        { from: Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), 1), until: Date.UTC(date.getUTCFullYear(), date.getUTCMonth() + 1, 1) },
      ];
      const produced = (["daily", "weekly", "monthly"] as const).map((period: LeaderboardPeriod) => periodWindow(period, at));
      if (JSON.stringify(produced) !== JSON.stringify(expected)) expect(produced, date.toISOString()).toEqual(expected);
    }
  });
});

describe("standing", () => {
  const TUESDAY = MONDAY + DAY;
  const state = learner(id(1), [
    registered(id(1)),
    started(id(1), id(101), "physics", MONDAY),
    submitted(id(1), id(101), "physics", 0.75, MONDAY + 5),
    awarded(id(1), "perfect-physics", id(101), MONDAY + 5),
    started(id(1), id(102), "heating", TUESDAY - 9),
    submitted(id(1), id(102), "heating", 0.5, TUESDAY),
    awarded(id(1), "completionist", id(102), TUESDAY),
    started(id(1), id(103), "physics", TUESDAY + 1),
    submitted(id(1), id(103), "physics", 0.25, TUESDAY + 9),
    awarded(id(1), "lost", id(199), TUESDAY + 9),
  ]);
  const record = transcript(state)!;
  const scoped = (period: LeaderboardPeriod, quiz: string | undefined, at: number) => standing(record, VIEW, boardScope(period, quiz, at));

  it("lists the submitted runs in submission order and the badges with the quiz of the run that earned them", () => {
    expect(record.runs).toEqual([
      { quiz: "physics", score: 0.75, at: MONDAY + 5 },
      { quiz: "heating", score: 0.5, at: TUESDAY },
      { quiz: "physics", score: 0.25, at: TUESDAY + 9 },
    ]);
    expect(record.badges).toEqual([
      { badge: "perfect-physics", quiz: "physics", at: MONDAY + 5 },
      { badge: "completionist", quiz: "heating", at: TUESDAY },
    ]);
  });

  it("is made of the runs in scope only: their bests, their count, the last of them and the badges they earned", () => {
    expect(scoped("all-time", undefined, 0)).toMatchObject({ total: 125, best: { heating: 0.5, physics: 0.75 }, badges: ["perfect-physics", "completionist"], runs: 3, reachedAt: TUESDAY, lastActivity: TUESDAY + 9 });
    expect(standing(record, VIEW)).toEqual(scoped("all-time", undefined, 0));
    expect(scoped("weekly", undefined, TUESDAY)).toEqual(scoped("all-time", undefined, 0));
    expect(scoped("daily", undefined, MONDAY)).toMatchObject({ total: 75, best: { physics: 0.75 }, badges: ["perfect-physics"], runs: 1, reachedAt: MONDAY + 5, lastActivity: MONDAY + 5 });
    expect(scoped("daily", undefined, TUESDAY + 1)).toMatchObject({ total: 75, best: { heating: 0.5, physics: 0.25 }, badges: ["completionist"], runs: 2, reachedAt: TUESDAY + 9, lastActivity: TUESDAY + 9 });
    expect(scoped("daily", "physics", TUESDAY)).toMatchObject({ total: 25, best: { physics: 0.25 }, badges: [], runs: 1 });
    expect(scoped("all-time", "heating", 0)).toMatchObject({ total: 50, best: { heating: 0.5 }, badges: ["completionist"], runs: 1, reachedAt: TUESDAY, lastActivity: TUESDAY });
  });

  it("is undefined when no run is in scope", () => {
    expect(scoped("daily", undefined, MONDAY - 1)).toBeUndefined();
    expect(scoped("daily", undefined, TUESDAY + DAY)).toBeUndefined();
    expect(scoped("all-time", "cooling", 0)).toBeUndefined();
  });
});

describe("leaderboard of a period", () => {
  const TUESDAY = MONDAY + DAY;
  const early = competitor(1, [["physics", 1, MONDAY], ["heating", 0.25, TUESDAY + 7]]);
  const late = competitor(2, [["physics", 0.5, TUESDAY - 1], ["physics", 0.75, TUESDAY + 3]]);
  const records = transcripts([early, late]);
  const outline = (period: LeaderboardPeriod, quiz: string | undefined, at: number) => {
    const answer = leaderboard(records, VIEW, { period, ...(quiz === undefined ? {} : { quiz }) }, at, id(1));
    return { rows: answer.rows.map((row) => [row.tag, row.total, row.runs]), learners: answer.learners, submissions: answer.submissions, own: answer.own?.rank, window: answer.window, quiz: answer.quiz };
  };
  const [first, second] = [learnerTag(id(1)), learnerTag(id(2))];

  it("ranks the learners of its window by the runs inside it and counts every submission", () => {
    expect(outline("all-time", undefined, TUESDAY)).toEqual({ rows: [[first, 125, 2], [second, 75, 2]], learners: 2, submissions: 4, own: 1, window: undefined, quiz: undefined });
    expect(outline("daily", undefined, TUESDAY)).toEqual({ rows: [[second, 75, 1], [first, 25, 1]], learners: 2, submissions: 4, own: 2, window: periodWindow("daily", TUESDAY), quiz: undefined });
    expect(outline("daily", undefined, MONDAY)).toEqual({ rows: [[first, 100, 1], [second, 50, 1]], learners: 2, submissions: 4, own: 1, window: periodWindow("daily", MONDAY), quiz: undefined });
    expect(outline("monthly", undefined, TUESDAY + 40 * DAY).rows).toEqual([]);
  });

  it("counts one quiz only when it names one", () => {
    expect(outline("weekly", "heating", TUESDAY)).toEqual({ rows: [[first, 25, 1]], learners: 1, submissions: 4, own: 1, window: periodWindow("weekly", MONDAY), quiz: "heating" });
    expect(outline("daily", "physics", TUESDAY + 5)).toEqual({ rows: [[second, 75, 1]], learners: 1, submissions: 4, own: undefined, window: periodWindow("daily", TUESDAY), quiz: "physics" });
  });
});
