/** 🔓️ Shared release vectors, independently judged by Ajv's numeric and null assertions. */
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { courseAccess, courseViewIssues, type CourseView } from "../../📦️packages/🟦️typescript/🟦️.ts";
import vectors from "../../🧫️fixtures/🔓️access/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";

describe("course scheduled access", () => {
  for (const row of vectors.cases) it(row.name, () => {
    const course = { unlocks: row.unlocks, topics: [{ id: "energy", quizzes: ["physics"] }] } as unknown as CourseView;
    const keys = [course.topics![0]!.quizzes.includes(row.page) ? "energy" : undefined, row.page, row.task === undefined ? undefined : `${row.page}/${row.task}`].filter((key): key is string => key !== undefined);
    const relevant = keys.flatMap((key) => Object.hasOwn(row.unlocks, key) ? [(row.unlocks as Record<string, number | null>)[key]] : []);
    const oracle = new Ajv().compile({ type: "array", items: { type: "integer", minimum: 0, maximum: row.at } });
    expect(oracle(relevant)).toBe(row.allowed);
    expect(courseAccess(course, row.page, row.at, row.task)).toBe(row.allowed);
  });

  it("validates schedule maps against the normative schema and an independent validator", () => {
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(schema);
    const valid = ajv.compile({ $ref: `${schema.$id}#/$defs/Unlocks` });
    const base = { id: "access", title: { en: "Access", de: "Zugang" }, introduction: { title: { en: "Access", de: "Zugang" }, paragraphs: [{ en: "Read", de: "Lesen" }] }, quizzes: [], badges: [] };
    for (const [unlocks, accepted] of [[{}, true], [{ energy: 0, "physics/powers": null }, true], [{ energy: -1 }, false], [{ energy: 1.5 }, false], [{ energy: "2026-10-26" }, false], [{ energy: Number.MAX_SAFE_INTEGER + 1 }, false], [{ "bad/key/path": 0 }, false], [{ ["x".repeat(65)]: 0 }, false], [{ [`physics/${"x".repeat(65)}`]: 0 }, false], [[], false], [null, false]] as const) {
      expect(valid(unlocks)).toBe(accepted);
      expect(courseViewIssues({ ...base, unlocks }).length === 0).toBe(accepted);
    }
  });
});
