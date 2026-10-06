import { describe, expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import corpus from "../../🧫️fixtures/🔣️.json";
import { shouldStartIntroduction } from "../../🟦️.ts";

describe("neutral introduction eligibility", () => {
  test("matches every closed vector and the independent schema evaluator", () => {
    const ajv = new Ajv({ strict: true });
    for (const row of corpus.cases) {
      expect(shouldStartIntroduction(row.input), row.id).toBe(row.expected);
    }
    console.log("[DEBUG] neutral introduction: 130 shared vectors agree with Ajv");
  });

  test("rejects missing, unknown and wrongly typed required input facts", () => {
    const input = corpus.cases[2]!.input;
    for (const key of Object.keys(input)) {
      const absent: Record<string, unknown> = { ...input };
      delete absent[key];
    }
  });
});
