import { describe, expect, it } from "vitest";
import { CHALLENGES, challengeRank, earnedBadges, points, type Badge, type BadgeRule, type Challenge, type Quiz, type RunResult, type Task, type TaskKind, type Text } from "../../📦️packages/🟦️typescript/🟦️.ts";

const T = (en: string): Text => ({ en, de: en });

/** 🧩️ A task of the given kind; only id and kind matter to badges. */
function task(id: string, kind: TaskKind): Task {
  const head = { id, title: T(id), prompt: T(id) };
  if (kind === "sorting") return { kind, ...head, quantity: { label: T("q"), unit: "u", scale: "linear", prefixed: false, additive: false }, items: [] };
  if (kind === "matching") return { kind, ...head, dimensions: [], items: [] };
  return { kind, ...head, categories: [], items: [] };
}

const PHYSICS: Quiz = { schema: "semio.quiz/v1", id: "physics", emoji: "🧲", title: T("Physics"), description: T("Physics"), tasks: [task("power", "sorting"), task("energy", "sorting"), task("units", "classification")] };
const HEATING: Quiz = { schema: "semio.quiz/v1", id: "heating", emoji: "🔥", title: T("Heating"), description: T("Heating"), tasks: [task("u-values", "matching"), task("loads", "sorting")] };
const QUIZZES = [PHYSICS, HEATING];

/** 🏅️ A badge with the given rule. */
function badge(id: string, rule: BadgeRule): Badge {
  return { id, emoji: "🏅", label: T(id), description: T(id), rule };
}

/** 🏁️ A result of `quiz` at a challenge (medium unless named) with the given task scores, scored as their mean. */
function result(quiz: Quiz, scores: Readonly<Record<string, number>>, challenge: Challenge = "medium"): RunResult {
  const tasks = quiz.tasks.map((candidate) => ({ kind: "sorting" as const, task: candidate.id, score: scores[candidate.id] ?? 0, items: [] }));
  const score = tasks.reduce((sum, entry) => sum + entry.score, 0) / tasks.length;
  return { quiz: quiz.id, challenge, score, points: points(score, challenge), tasks };
}

const BADGES = [
  badge("perfect-physics", { kind: "perfect-quiz", quiz: "physics" }),
  badge("sorter", { kind: "perfect-tasks", taskKind: "sorting" }),
  badge("physics-sorter", { kind: "perfect-tasks", taskKind: "sorting", quiz: "physics" }),
  badge("heating-master", { kind: "perfect-tasks", quiz: "heating" }),
  badge("everything", { kind: "perfect-tasks" }),
  badge("nothing", { kind: "perfect-tasks", taskKind: "classification", quiz: "heating" }),
  badge("completionist", { kind: "completed-quizzes" }),
];

describe("earnedBadges", () => {
  it("awards nothing without results", () => {
    expect(earnedBadges(BADGES, QUIZZES, [], [])).toEqual([]);
  });

  it("awards perfect-quiz only for a run scoring exactly 1", () => {
    const rules = [BADGES[0]!];
    expect(earnedBadges(rules, QUIZZES, [result(PHYSICS, { power: 1, energy: 1, units: 0.999 })], [])).toEqual([]);
    expect(earnedBadges(rules, QUIZZES, [result(PHYSICS, { power: 1, energy: 1, units: 0.5 }), result(PHYSICS, { power: 1, energy: 1, units: 1 })], [])).toEqual(["perfect-physics"]);
    expect(earnedBadges(rules, QUIZZES, [result(HEATING, { "u-values": 1, loads: 1 })], [])).toEqual([]);
  });

  it("awards perfect-tasks once every selected task scored 1 in some run, across runs and quizzes", () => {
    const results = [result(PHYSICS, { power: 1, energy: 0.5 }), result(PHYSICS, { power: 0.2, energy: 1 })];
    expect(earnedBadges(BADGES, QUIZZES, results, [])).toEqual(["physics-sorter"]);
    const withHeating = [...results, result(HEATING, { "u-values": 0.9, loads: 1 })];
    expect(earnedBadges(BADGES, QUIZZES, withHeating, [])).toEqual(["sorter", "physics-sorter", "completionist"]);
    const everything = [...withHeating, result(HEATING, { "u-values": 1 }), result(PHYSICS, { units: 1 })];
    expect(earnedBadges(BADGES, QUIZZES, everything, [])).toEqual(["sorter", "physics-sorter", "heating-master", "everything", "completionist"]);
  });

  it("never awards a selector that matches no task", () => {
    const perfect = [result(PHYSICS, { power: 1, energy: 1, units: 1 }), result(HEATING, { "u-values": 1, loads: 1 })];
    expect(earnedBadges(BADGES, QUIZZES, perfect, [])).not.toContain("nothing");
    expect(earnedBadges([badge("ghost", { kind: "perfect-tasks", quiz: "cooling" })], QUIZZES, perfect, [])).toEqual([]);
  });

  it("awards completed-quizzes once every catalog quiz has a submitted result, whatever its score", () => {
    const rules = [BADGES[6]!];
    expect(earnedBadges(rules, QUIZZES, [result(PHYSICS, {})], [])).toEqual([]);
    expect(earnedBadges(rules, QUIZZES, [result(PHYSICS, {}), result(HEATING, {})], [])).toEqual(["completionist"]);
    expect(earnedBadges(rules, [], [result(PHYSICS, {})], [])).toEqual([]);
  });

  it("skips held badges and keeps the catalog order", () => {
    const perfect = [result(PHYSICS, { power: 1, energy: 1, units: 1 }), result(HEATING, { "u-values": 1, loads: 1 })];
    expect(earnedBadges(BADGES, QUIZZES, perfect, [])).toEqual(["perfect-physics", "sorter", "physics-sorter", "heating-master", "everything", "completionist"]);
    expect(earnedBadges(BADGES, QUIZZES, perfect, ["sorter", "completionist"])).toEqual(["perfect-physics", "physics-sorter", "heating-master", "everything"]);
    expect(earnedBadges([...BADGES].reverse(), QUIZZES, perfect, [])).toEqual(["completionist", "everything", "heating-master", "physics-sorter", "sorter", "perfect-physics"]);
  });

  it("counts every challenge for a rule without a least challenge", () => {
    for (const challenge of CHALLENGES) {
      const perfect = [result(PHYSICS, { power: 1, energy: 1, units: 1 }, challenge), result(HEATING, { "u-values": 1, loads: 1 }, challenge)];
      expect(earnedBadges(BADGES, QUIZZES, perfect, [])).toEqual(["perfect-physics", "sorter", "physics-sorter", "heating-master", "everything", "completionist"]);
    }
  });
});

describe("earnedBadges — a least challenge", () => {
  const PERFECT = { power: 1, energy: 1, units: 1 };

  it("awards perfect-quiz only for a perfect run at a challenge that meets the least one", () => {
    for (const least of CHALLENGES) {
      const rules = [badge("least", { kind: "perfect-quiz", quiz: "physics", challenge: least })];
      for (const challenge of CHALLENGES) {
        const earned = challengeRank(challenge) >= challengeRank(least);
        expect(earnedBadges(rules, QUIZZES, [result(PHYSICS, PERFECT, challenge)], [])).toEqual(earned ? ["least"] : []);
        expect(earnedBadges(rules, QUIZZES, [result(PHYSICS, { ...PERFECT, units: 0.999 }, challenge)], [])).toEqual([]);
      }
    }
  });

  it("looks for the perfect run among those that count: an easier perfect run beside a harder imperfect one earns nothing", () => {
    const rules = [badge("hard-physics", { kind: "perfect-quiz", quiz: "physics", challenge: "hard" })];
    const results = [result(PHYSICS, PERFECT, "easy"), result(PHYSICS, PERFECT, "medium"), result(PHYSICS, { ...PERFECT, units: 0.5 }, "hard"), result(PHYSICS, { ...PERFECT, power: 0 }, "expert")];
    expect(earnedBadges(rules, QUIZZES, results, [])).toEqual([]);
    expect(earnedBadges(rules, QUIZZES, [...results, result(PHYSICS, PERFECT, "expert")], [])).toEqual(["hard-physics"]);
    expect(earnedBadges(rules, QUIZZES, [...results, result(HEATING, { "u-values": 1, loads: 1 }, "expert")], [])).toEqual([]);
  });

  it("awards perfect-tasks once every selected task scored 1 in some run that counts", () => {
    const rules = [badge("medium-sorter", { kind: "perfect-tasks", taskKind: "sorting", challenge: "medium" }), badge("expert-physics", { kind: "perfect-tasks", quiz: "physics", challenge: "expert" }), badge("easy-heating", { kind: "perfect-tasks", quiz: "heating", challenge: "easy" })];
    const easy = [result(PHYSICS, PERFECT, "easy"), result(HEATING, { "u-values": 1, loads: 1 }, "easy")];
    expect(earnedBadges(rules, QUIZZES, easy, [])).toEqual(["easy-heating"]);
    const split = [...easy, result(PHYSICS, { power: 1, energy: 0.5 }, "medium"), result(PHYSICS, { power: 0.2, energy: 1 }, "hard")];
    expect(earnedBadges(rules, QUIZZES, split, [])).toEqual(["easy-heating"]);
    const sorted = [...split, result(HEATING, { loads: 1 }, "expert")];
    expect(earnedBadges(rules, QUIZZES, sorted, [])).toEqual(["medium-sorter", "easy-heating"]);
    const most = [...sorted, result(PHYSICS, { power: 1, energy: 1 }, "expert")];
    expect(earnedBadges(rules, QUIZZES, most, [])).toEqual(["medium-sorter", "easy-heating"]);
    expect(earnedBadges(rules, QUIZZES, [...most, result(PHYSICS, { units: 1 }, "expert")], [])).toEqual(["medium-sorter", "expert-physics", "easy-heating"]);
    expect(earnedBadges(rules, QUIZZES, [...most, result(PHYSICS, { units: 1 }, "hard")], [])).toEqual(["medium-sorter", "easy-heating"]);
  });

  it("leaves completed-quizzes to every challenge and still never awards a selector that matches no task", () => {
    const easy = [result(PHYSICS, {}, "easy"), result(HEATING, {}, "easy")];
    expect(earnedBadges([BADGES[6]!], QUIZZES, easy, [])).toEqual(["completionist"]);
    expect(earnedBadges([badge("ghost", { kind: "perfect-tasks", taskKind: "classification", quiz: "heating", challenge: "easy" })], QUIZZES, [result(HEATING, { "u-values": 1, loads: 1 }, "expert")], [])).toEqual([]);
  });
});
