#!/usr/bin/env bun
import { join } from "node:path";
import { BundleScript, ScriptRouter, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runCanonicalGoTests, runCmd } from "../../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";

class BuildScript extends BundleScript {
  run(): void {
    const output = join(this.root, "../../..", process.platform === "win32" ? "mcp.exe" : "mcp");
    runCmd("go", ["build", "-trimpath", "-o", output, "."], { cwd: join(this.root, "../..") });
  }
}

class DevScript extends BundleScript {
  run(args: string[]): void {
    runCmd(join(this.root, "../../..", process.platform === "win32" ? "mcp.exe" : "mcp"), args, { cwd: this.repoRoot });
  }
}

class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(args);
    await runCanonicalGoTests(join(this.root, "../.."), [...goLevelTestArgs(level), ...rest]);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("dev", DevScript).register("test", TestScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
