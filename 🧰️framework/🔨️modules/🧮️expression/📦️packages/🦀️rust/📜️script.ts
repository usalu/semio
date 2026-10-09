#!/usr/bin/env bun
import { join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

const ORACLES = ["python"] as const;

/** 🧪️ `semio-framework-expression` tests: `test [level]` runs the cargo suites (unit tests and the fixture conformance suite); `test oracle [python]` reproduces the committed fixtures with Python's `ast`, `math`, `decimal`, `networkx` and `jsonschema`. @see 🧪️tests/📜️conformance */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] !== "oracle") {
      const { rest } = resolveTestLevel(segments);
      await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-expression"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
      return;
    }
    const { rest } = resolveTestLevel(segments.slice(1));
    const picked = rest.length ? rest : [...ORACLES];
    for (const name of picked) if (!(ORACLES as readonly string[]).includes(name)) throw Error(`Expected test oracle ${ORACLES.join("|")}, got ${JSON.stringify(name)}`);
    const owner = resolve(this.root, "../..");
    const budgetMs = testLevelBudgetMs();
    if (picked.includes("python")) await runBudgetedTestCommand("uv", ["run", "--locked", "--no-sync", "python", "-m", "pytest", "-p", "no:cacheprovider", "-q", join(owner, "🧪️tests/📜️conformance/🐍️.py")], { cwd: this.repoRoot, budgetMs });
  }
}

await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { defaultCommand: "test" });
