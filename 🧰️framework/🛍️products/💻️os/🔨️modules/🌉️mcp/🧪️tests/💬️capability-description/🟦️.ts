/** 💬️ The `CapabilityDescription` law cases replayed by the AJV twin (`🗂️catalog/🟦️.ts`) — the Rust side
 * replays the same file in `catalog::quick::description_problems_match_the_language_agnostic_fixture`. */
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { capabilityDescriptionProblems, proveCapabilityDescriptionFixture } from "../../🗂️catalog/🟦️.ts";

const repoRoot = resolve(import.meta.dirname, "../../../../../../..");

describe("capability description law", () => {
  it("replays every fixture row with the identical (verb, problem) list", () => {
    expect(proveCapabilityDescriptionFixture(repoRoot)).toBeGreaterThanOrEqual(10);
  });

  it("judges an undeclared description as missing and nothing else", () => {
    expect(capabilityDescriptionProblems(repoRoot, [{ id: "solve", title: { en: "Solve", de: "Lösen" }, description: null }])).toEqual([{ verb: "solve", problem: "missing" }]);
  });
});
