/** 💬️ The `CapabilityDescription` law cases replayed by the AJV twin (`🗂️catalog/🟦️.ts`) — the Rust side
 * replays the same file in `catalog::quick::description_problems_match_the_language_agnostic_fixture`. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { capabilityDescriptionProblems } from "../../🗂️catalog/🟦️.ts";

const repoRoot = resolve(import.meta.dirname, "../../../../../../..");

describe("capability description law", () => {
  it("replays every fixture row with the identical (verb, problem) list", () => {
    const rows = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../🗂️catalog/🧫️fixtures/💬️capability-description.json"), "utf8"));
    expect(rows.length).toBeGreaterThanOrEqual(10);
    for (const row of rows) expect(capabilityDescriptionProblems(repoRoot, row.verbs), row.name).toEqual(row.problems);
  });

  it("judges an undeclared description as missing and nothing else", () => {
    expect(capabilityDescriptionProblems(repoRoot, [{ id: "solve", title: { en: "Solve", de: "Lösen" }, description: null }])).toEqual([{ verb: "solve", problem: "missing" }]);
  });
});
