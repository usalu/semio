import Ajv from "ajv";
import { mod } from "mathjs";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { CHALLENGES, DEFAULT_LIMITS, decideLearner, emptyLearnerState, evolveLearner, runSeed, quizIssues, sheetOf, type Course, type Event, type Quiz, type ValidationIssue } from "../../📦️packages/🟦️typescript/🟦️.ts";

const quiz = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧺️balanced-pools/🔣️.json", import.meta.url), "utf8")) as Quiz;
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const timing = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧺️balanced-pools/🩺️validation.json", import.meta.url), "utf8")) as readonly { id: string; seconds: number; issues: readonly ValidationIssue[]; keyword: string | null }[];

/** 🌀️ Selections corroborated by NumPy's MT19937 and the independent Fisher–Yates oracle. */
const expected = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧺️balanced-pools/🧾️expected.json", import.meta.url), "utf8")) as readonly { seed: number; items: string[] }[];

describe("balanced pools", () => {
  for (const vector of timing) it(`reports the shared pooled timing boundary: ${vector.id}`, () => {
    const pool = { ...quiz.tasks[0]!.pool!, seconds: vector.seconds };
    const document = { ...quiz, tasks: [{ ...quiz.tasks[0]!, pool }] };
    expect(quizIssues(document)).toEqual(vector.issues);
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(schema);
    expect(ajv.validate(schema.$id + "#/$defs/QuestionPool", pool)).toBe(vector.issues.length === 0);
    expect((ajv.errors ?? []).map(({ instancePath, keyword }) => ({ instancePath, keyword }))).toEqual(vector.keyword === null ? [] : [{ instancePath: "/seconds", keyword: vector.keyword }]);
  });

  it("persists the advancing attempt across voided runs and event replay", () => {
    const learner = "a".repeat(32);
    const course: Course = { schema: "semio.course/v1", id: "test", title: { en: "Test", de: "Test" }, introduction: { title: { en: "Test", de: "Test" }, paragraphs: [{ en: "Test", de: "Test" }] }, quizzes: ["quiz.json"], badges: [] };
    const registered: Event = { type: "learner-registered", learner, identity: { kind: "anonymous" }, at: 1 };
    const events: Event[] = [registered];
    let state = evolveLearner(emptyLearnerState(learner), registered);
    const seen = new Set<string>();
    for (let attempt = 0; attempt < 4; attempt++) {
      const decision = decideLearner(state, { type: "start-run", id: (attempt + 1).toString(16).padStart(32, "0"), learner, run: (attempt + 100).toString(16).padStart(32, "0"), quiz: quiz.id, challenge: attempt % 2 === 0 ? "medium" : "hard", at: attempt + 2 }, { now: attempt + 2, course, quizzes: { [quiz.id]: { quiz, revision: "b".repeat(64) } }, limits: DEFAULT_LIMITS });
      expect("rejection" in decision ? decision.rejection : undefined).toBeUndefined();
      if ("rejection" in decision) throw new Error(decision.rejection);
      events.push(...decision.events);
      state = decision.events.reduce(evolveLearner, state);
      const run = state.runs.at(-1)!;
      expect(run.seed & 65535).toBe(attempt);
      expect(run.seed >>> 16).toBe(runSeed(learner + ":" + quiz.id) >>> 16);
      for (const item of sheetOf(quiz, run.seed, run.challenge).tasks[0]!.items) {
        expect(seen.has(item.id)).toBe(false);
        seen.add(item.id);
      }
    }
    expect(state.runs.map(run => run.status)).toEqual(["voided", "voided", "voided", "open"]);
    expect((JSON.parse(JSON.stringify(events)) as Event[]).reduce(evolveLearner, emptyLearnerState(learner))).toEqual(state);
  });

  it("keeps the draw of the learner who started the run when that run is delivered as someone else", () => {
    const drawn = "b".repeat(32);
    const holder = "c".repeat(32);
    const course: Course = { schema: "semio.course/v1", id: "test", title: { en: "Test", de: "Test" }, introduction: { title: { en: "Test", de: "Test" }, paragraphs: [{ en: "Test", de: "Test" }] }, quizzes: ["quiz.json"], badges: [] };
    const registered: Event = { type: "learner-registered", learner: holder, identity: { kind: "anonymous" }, at: 1 };
    const state = evolveLearner(emptyLearnerState(holder), registered);
    const context = { now: 2, course, quizzes: { [quiz.id]: { quiz, revision: "b".repeat(64) } }, limits: DEFAULT_LIMITS };
    const decision = decideLearner(state, { type: "start-run", id: "1".padStart(32, "0"), learner: holder, origin: drawn, run: "2".padStart(32, "0"), quiz: quiz.id, challenge: "medium", at: 2 }, context);
    if ("rejection" in decision) throw new Error(decision.rejection);
    const seed = decision.events.find((event) => event.type === "run-started" && event.run === "2".padStart(32, "0"));
    if (seed?.type !== "run-started") throw new Error("the run did not start");
    expect(seed.seed >>> 16).toBe(runSeed(drawn + ":" + quiz.id) >>> 16);
    expect(seed.seed >>> 16).not.toBe(runSeed(holder + ":" + quiz.id) >>> 16);
  });

  it("agrees with independent NumPy selections", () => {
    for (const vector of expected) for (const challenge of CHALLENGES) expect(sheetOf(quiz, vector.seed, challenge).tasks[0]!.items.map(item => item.id).sort()).toEqual(vector.items);
  });
  it("accepts the shared fixture with our validator and AJV", () => {
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(schema);
    expect(ajv.validate(schema.$id + "#/$defs/Quiz", quiz), JSON.stringify(ajv.errors)).toBe(true);
    expect(quizIssues(quiz)).toEqual([]);
  });
  it("draws one variant per slot without repeats until exhaustion at every challenge", () => {
    for (const challenge of CHALLENGES) {
      const seen = new Set<string>();
      for (let attempt = 0; attempt < 4; attempt++) {
        const task = sheetOf(quiz, (123 << 16) | attempt, challenge).tasks[0]!;
        expect(task.items).toHaveLength(2);
        for (const slot of quiz.tasks[0]!.pool!.slots) {
          const selected = task.items.filter(item => slot.includes(item.id));
          expect(selected).toHaveLength(1);
          expect(seen.has(selected[0]!.id)).toBe(false);
          seen.add(selected[0]!.id);
        }
        expect(task.seconds).toBe(challenge === "expert" ? 300 : undefined);
        const again = sheetOf(quiz, (123 << 16) | (attempt + 4), challenge).tasks[0]!;
        expect(again.items.map(item => item.id).sort()).toEqual(task.items.map(item => item.id).sort());
        expect(mod(attempt + 4, 4)).toBe(attempt);
      }
    }
  });
  it("rejects repeated, missing and unknown slot membership", () => {
    for (const slots of [[["tiny-a"], ["tiny-a"]], [["ghost"], ["large-a"]], [["tiny-a"], ["large-a"]]]) {
      const bad = { ...quiz, tasks: [{ ...quiz.tasks[0]!, pool: { seconds: 300, slots } }] };
      expect(quizIssues(bad).length).toBeGreaterThan(0);
    }
  });
});
