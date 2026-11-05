import { describe, expect, it } from "vitest";
import { verifyPickerExamples } from "../../../🧪️tests/🧩️picker/🟦️.ts";
describe("demo-session", () => {
  it("loads every registered picker snapshot through the production codec", () => {
    expect(verifyPickerExamples().map(example => example.id)).toEqual(["demo", "demo-session", "synthetic-orbit"]);
  });
});
