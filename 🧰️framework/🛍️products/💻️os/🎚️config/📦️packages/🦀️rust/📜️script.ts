#!/usr/bin/env bun
/** ⚙️ Validates the canonical OS configuration owner. */
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runCargo } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-os-config"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "-p", "semio-framework-os-config", ...segments], this.repoRoot);
  }
}

await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript), { defaultCommand: "test" });
