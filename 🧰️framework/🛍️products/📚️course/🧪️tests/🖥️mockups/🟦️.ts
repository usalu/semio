/** 🖥️ Language-neutral preview vectors, checked against the independent AJV structural oracle. */
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { CHALLENGES, quizIssues, sheetOf, type Quiz } from "../../📦️packages/🟦️typescript/🟦️.ts";
import contract from "../../🧬️schema/🔣️.json";
import vectors from "../../🧫️fixtures/🖥️mockups/🔣️.json";

const text = { en: "Reference", de: "Referenz" };
const quantity = { label: text, unit: "1", scale: "linear", prefixed: false, additive: false };
const oracle = new Ajv({ strict: false }).compile({ ...contract, $ref: "#/$defs/Mockup" });
const quiz = (mockup: unknown, kind = "classification"): Quiz => ({
  schema: "semio.quiz/v1", id: "preview", emoji: "🖥️", title: text, description: text,
  tasks: [{ kind, id: "reference", title: text, prompt: text,
    ...(kind === "classification" ? { categories: [{ id: "a", label: text }, { id: "b", label: text }] } : kind === "sorting" ? { quantity } : { dimensions: [{ id: "count", quantity }] }),
    items: ["one", "two"].map((id, index) => ({ id, label: text, mockup, explanation: text, ...(kind === "classification" ? { category: index ? "b" : "a" } : kind === "sorting" ? { value: index + 1 } : { values: { count: index + 1 } }) })),
  }],
} as Quiz);

describe("software mockups", () => {
  for (const vector of vectors) it(vector.id, () => {
    expect(oracle(vector.mockup), JSON.stringify(oracle.errors)).toBe(vector.schemaValid);
    for (const kind of ["classification", "sorting", "matching"]) expect(quizIssues(quiz(vector.mockup, kind)).length === 0).toBe(vector.valid);
  });

  it("keeps given data at every challenge and strips grading truth from every item kind", () => {
    for (const vector of vectors.filter((vector) => vector.valid)) for (const kind of ["classification", "sorting", "matching"]) for (const challenge of CHALLENGES) {
      const sheet = sheetOf(quiz(vector.mockup, kind), 42, challenge);
      for (const item of sheet.tasks[0]!.items) {
        expect(item.mockup).toEqual(vector.mockup);
        expect(Object.keys(item).sort()).toEqual(["id", "label", "mockup"]);
        expect(oracle(item.mockup)).toBe(true);
      }
      if (challenge === "hard" || challenge === "expert") {
        expect(sheet.tasks[0]).not.toHaveProperty("keys");
        if (sheet.tasks[0]!.kind === "matching") for (const dimension of sheet.tasks[0]!.dimensions) expect(dimension).not.toHaveProperty("cards");
      }
    }
  });
});
