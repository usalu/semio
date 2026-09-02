/** ️tests for example capsule-dream. */
import { describe, expect, it } from "../../../../../../../../../../../../🧪️artifact.ts";
import { dslPath, id } from "../🟦️.ts";

describe("capsule-dream example", () => {
  it("exposes stable id and dsl url", () => {
    expect(id).toBe("capsule-dream");
    expect(String(dslPath)).toContain("dream.dsl.semio");
  });
});
