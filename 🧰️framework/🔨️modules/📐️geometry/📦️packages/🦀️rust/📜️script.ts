#!/usr/bin/env bun
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

const ORACLES = ["three", "shapely"] as const;

/** 🧪️ `semio-framework-geometry` tests: `test [level]` runs the cargo suites (kurbo and parry3d oracles included); `test oracle [three|shapely]` runs the third-party oracle suites on the committed fixtures: `three` (vitest) and `shapely` (pytest, covering `semio-framework-2d` regions). @see 🧪️tests/🏙️aec-oracles */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] !== "oracle") {
      const { rest } = resolveTestLevel(segments);
      await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-geometry"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
      return;
    }
    const { rest } = resolveTestLevel(segments.slice(1));
    const picked = rest.length ? rest : [...ORACLES];
    for (const name of picked) if (!(ORACLES as readonly string[]).includes(name)) throw Error(`Expected test oracle ${ORACLES.join("|")}, got ${JSON.stringify(name)}`);
    const owner = resolve(this.root, "../..");
    const budgetMs = testLevelBudgetMs();
    if (picked.includes("three")) await runBudgetedTestCommand("node", [join(this.repoRoot, "node_modules/vitest/vitest.mjs"), "run", "--config", join(owner, "🧪️tests/🎚️config/🟦️.ts")], { cwd: owner, budgetMs });
    if (picked.includes("shapely")) await runBudgetedTestCommand("uv", ["run", "--locked", "--no-sync", "python", "-m", "pytest", "-p", "no:cacheprovider", "-q", join(owner, "🧪️tests/🏙️aec-oracles/🐍️.py")], { cwd: this.repoRoot, budgetMs });
  }
}

await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { defaultCommand: "test" });
