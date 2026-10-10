#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, runCargoLintV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📜️ `@semio-tech/framework-typeset` — the one semio typesetting crate: cargo test and clippy gates. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-typeset"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-typeset --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargoLintV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-typeset"],cwd:this.root,extraArgs:segments},readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
