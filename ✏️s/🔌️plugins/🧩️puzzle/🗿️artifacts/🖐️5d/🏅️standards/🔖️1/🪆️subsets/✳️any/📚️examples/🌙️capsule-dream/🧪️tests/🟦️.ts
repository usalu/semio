/** 🌙️ Tests the committed capsule-dream example carrier. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { dslPath, id } from "../🟦️.ts";

describe("capsule-dream example", () => {
  it("exposes stable id and dsl url", () => {
    expect(id).toBe("capsule-dream");
    expect(readFileSync(dslPath, "utf8").length).toBeGreaterThan(64);
  });
});
