#!/usr/bin/env bun
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

/** ⏱️ Receives the complete original camera storage package without law filters. */
class TestScript extends BundleScript {
    async run(segments: string[]): Promise<void> {
        const { rest } = resolveTestLevel(segments);
        if (rest.length) throw Error("Camera storage receives its whole original owning package");
        await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-os-renderer-camera"], cwd: this.root, extraArgs: ["--all-targets", "--no-fail-fast"] }, readCargoTestPolicyV1(process.env));
    }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { defaultCommand: "test" });
