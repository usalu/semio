#!/usr/bin/env bun
/** 🖨️ Native print verification and build, invoked through Nx. */
import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const command = Bun.spawn(["cargo", "test", "-p", "semio-framework-print", ...segments], { cwd: this.repoRoot, stdout: "inherit", stderr: "inherit" });
    if (await command.exited !== 0) throw new Error("Native print tests failed");
  }
}
class BuildScript extends BundleScript {
  async run(): Promise<void> {
    const command = Bun.spawn(["cargo", "check", "-p", "semio-framework-print"], { cwd: this.repoRoot, stdout: "inherit", stderr: "inherit" });
    if (await command.exited !== 0) throw new Error("Native print build failed");
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test" });
