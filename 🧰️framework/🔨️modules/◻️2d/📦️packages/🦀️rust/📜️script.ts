#!/usr/bin/env bun
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🦀️ `semio-framework-2d` router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const compute = segments[0] === "compute";
    const keys = compute && segments[1] === "keys";
    const { rest } = resolveTestLevel(compute ? segments.slice(keys ? 2 : 1) : segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-2d"], cwd: this.root, extraArgs: compute ? [keys ? "compute::key_contract" : "compute::", ...rest] : rest }, readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router, { defaultCommand: "test" });
