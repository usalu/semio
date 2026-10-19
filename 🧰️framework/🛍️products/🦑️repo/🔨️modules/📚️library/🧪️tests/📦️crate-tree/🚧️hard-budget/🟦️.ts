import { describe, expect, test } from "bun:test";
import { rmSync } from "node:fs";
import { crateBudgetFlags } from "../../../🏗️authoring/🎮️command/🟦️.ts";
import { planArtifactCrates } from "../../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { fixture } from "../🏗️virtual-crate/🟦️.ts";

describe("new crates budgets", () => {
  test("reads the target and the hard budget flags independently", () => {
    expect(crateBudgetFlags(new Set(["--dry-run"]))).toEqual({ target: undefined, hard: undefined });
    expect(crateBudgetFlags(new Set(["--target=26/195000/3300", "--hard=28/235000/3600"]))).toEqual({ target: { files: 26, bytes: 195000, lines: 3300 }, hard: { files: 28, bytes: 235000, lines: 3600 } });
    expect(crateBudgetFlags(new Set(["--hard=28/235000/3600"]))).toEqual({ target: undefined, hard: { files: 28, bytes: 235000, lines: 3600 } });
  });

  test("refuses a budget that is not three positive integers", () => {
    for (const text of ["--hard=0/1/1", "--hard=1/2", "--hard=a/b/c", "--hard=1/-2/3", "--target=1/2/3/4"]) expect(() => crateBudgetFlags(new Set([text])), text).toThrow("a crate budget is written");
  });

  test("plans against the hard budget it is given: a crate that fits the default gate exceeds a smaller one", async () => {
    const root = fixture();
    try {
      const fits = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...crateBudgetFlags(new Set(["--hard=32/262144/4000"])) });
      expect(fits.problems).toEqual([]);
      const refuses = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...crateBudgetFlags(new Set(["--target=1/1/1", "--hard=1/1/1"])) });
      expect(refuses.problems.some((problem) => problem.includes("exceeds the hard budget"))).toBe(true);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
});
