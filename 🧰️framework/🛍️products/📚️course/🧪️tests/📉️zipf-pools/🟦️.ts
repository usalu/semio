import Ajv from "ajv";
import { pow, sum } from "mathjs";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { quizIssues, runSeed, sheetOf, type Quiz } from "../../📦️packages/🟦️typescript/🟦️.ts";

const quiz = JSON.parse(readFileSync(new URL("../../🧫️fixtures/📉️zipf-pools/🔣️.json", import.meta.url), "utf8")) as Quiz;
const golden = JSON.parse(readFileSync(new URL("../../🧫️fixtures/📉️zipf-pools/🧾️expected.json", import.meta.url), "utf8")) as readonly { seed: number; items: string[] }[];
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const task = quiz.tasks[0]!;
const slots = task.pool!.slots;
const EXPONENT = 1.5;
const DEALS = 6000;
const DRAWN = 5;

/** 🎲️ The ids of the items dealt for the `index`-th seed. */
function dealt(index: number): string[] {
  return sheetOf(quiz, runSeed("deal-" + index), "medium").tasks[0]!.items.map((item) => item.id);
}

/** 🧮️ The exact probability that each slot is among `count` sequential weighted picks without replacement, with weights `rank^-exponent` computed by mathjs. */
function inclusion(exponent: number, count: number): number[] {
  const weights = slots.map((_, rank) => Number(pow(rank + 1, -exponent)));
  const probabilities = weights.map(() => 0);
  const walk = (remaining: number[], probability: number, picks: number[]): void => {
    if (picks.length === count) return void picks.forEach((slot) => (probabilities[slot]! += probability));
    const total = Number(sum(remaining.map((slot) => weights[slot]!)));
    for (const slot of remaining) walk(remaining.filter((other) => other !== slot), (probability * weights[slot]!) / total, [...picks, slot]);
  };
  walk(weights.map((_, slot) => slot), 1, []);
  return probabilities;
}

describe("zipf pools", () => {
  it("accepts the shared fixture with our validator and AJV", () => {
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(schema);
    expect(ajv.validate(schema.$id + "#/$defs/Quiz", quiz), JSON.stringify(ajv.errors)).toBe(true);
    expect(quizIssues(quiz)).toEqual([]);
  });

  it("rejects a distribution that leaves no slot undrawn, a non-positive exponent and an unknown kind", () => {
    const pooled = (draw: number, distribution: unknown) => ({ ...quiz, tasks: [{ ...task, draw, pool: { ...task.pool!, distribution } }] });
    expect(quizIssues(pooled(12, { kind: "zipf", exponent: 1.5 })).map((issue) => issue.path)).toEqual(["/tasks/0/draw"]);
    expect(quizIssues(pooled(5, { kind: "zipf", exponent: 0 })).length).toBeGreaterThan(0);
    expect(quizIssues(pooled(5, { kind: "zipf", exponent: -1 })).length).toBeGreaterThan(0);
    expect(quizIssues(pooled(5, { kind: "geometric", exponent: 1 })).length).toBeGreaterThan(0);
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(schema);
    for (const bad of [{ kind: "zipf", exponent: 0 }, { kind: "geometric", exponent: 1 }, { kind: "zipf" }]) expect(ajv.validate(schema.$id + "#/$defs/QuestionPool", { ...task.pool!, distribution: bad })).toBe(false);
  });

  it("deals five items of five distinct slots, deterministically per seed", () => {
    for (let index = 0; index < 200; index++) {
      const ids = dealt(index);
      expect(ids).toEqual(dealt(index));
      expect(ids).toHaveLength(5);
      expect(new Set(ids.map((id) => slots.findIndex((slot) => slot.includes(id)))).size).toBe(5);
    }
    expect(new Set(Array.from({ length: 50 }, (_, index) => dealt(index).join())).size).toBeGreaterThan(25);
  });

  it("deals the committed items the Rust twin deals for the same seeds, at every challenge", () => {
    for (const vector of golden) for (const challenge of ["easy", "medium", "hard", "expert"] as const) expect(sheetOf(quiz, vector.seed, challenge).tasks[0]!.items.map((item) => item.id).sort()).toEqual(vector.items);
  });

  it("includes every slot as often as the exact Zipf weights say", () => {
    const counts = slots.map(() => 0);
    for (let index = 0; index < DEALS; index++) for (const id of dealt(index)) counts[slots.findIndex((slot) => slot.includes(id))]!++;
    const expected = inclusion(EXPONENT, DRAWN);
    expect(sum(expected)).toBeCloseTo(DRAWN, 9);
    counts.forEach((count, slot) => expect(Math.abs(count / DEALS - expected[slot]!), `slot ${slot}`).toBeLessThan(0.025));
  });

  it("deals mostly small magnitudes and rarely, but sometimes, the largest", () => {
    const values = new Map(task.items.map((item) => [item.id, (item as { value: number }).value]));
    const ordered = [...values.values()].sort((left, right) => left - right);
    const lowerThird = ordered[Math.floor(ordered.length / 3) - 1]!;
    const topDecile = ordered[ordered.length - Math.round(ordered.length / 10)]!;
    let low = 0;
    let top = 0;
    for (let index = 0; index < DEALS; index++) for (const id of dealt(index)) {
      low += values.get(id)! <= lowerThird ? 1 : 0;
      top += values.get(id)! >= topDecile ? 1 : 0;
    }
    expect(low / (DEALS * DRAWN)).toBeGreaterThan(0.5);
    expect(top / (DEALS * DRAWN)).toBeLessThan(0.1);
    expect(top).toBeGreaterThan(0);
  });
});
