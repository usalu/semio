import { describe, expect, it } from "vitest";
import { earnedBadges, type Badge, type BadgeRule, type Quiz, type RunResult, type Task, type TaskKind, type Text } from "../../📦️packages/🟦️typescript/🟦️.ts";

const T = (en: string): Text => ({ en, de: en });

/** 🧩️ A task of the given kind; only id and kind matter to badges. */
function task(id: string, kind: TaskKind): Task {
  const head = { id, title: T(id), prompt: T(id) };
  if (kind === "sorting") return { kind, ...head, quantity: { label: T("q"), unit: "u", scale: "linear", prefixed: false }, items: [] };
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

/** 🏁️ A result of `quiz` with the given task scores, scored as their mean. */
function result(quiz: Quiz, scores: Readonly<Record<string, number>>): RunResult {
  const tasks = quiz.tasks.map((candidate) => ({ kind: "sorting" as const, task: candidate.id, score: scores[candidate.id] ?? 0, items: [] }));
  return { quiz: quiz.id, score: tasks.reduce((sum, entry) => sum + entry.score, 0) / tasks.length, tasks };
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
});
