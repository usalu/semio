#!/usr/bin/env bun
/** 🐚️ `@semio-tech/trinity-jack-shell` router: `bun ./📜️script.ts test` / `bun ./📜️script.ts run [args…]`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const BINARY_NAME = "semio-s-plugin-trinity-jack-shell";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted([BINARY_NAME], this.repoRoot);
  }
}

/** 🚀️ Execs the `build` target's staged binary (never `cargo run` — no wrapper-process tree to chase). */
class RunScript extends BundleScript {
  run(segments: string[]): void {
    const binary = join(this.root, "dist/build", process.platform === "win32" ? `${BINARY_NAME}.exe` : BINARY_NAME);
    const result = Bun.spawnSync([binary, ...segments], { cwd: this.repoRoot, stdout: "inherit", stderr: "inherit", stdin: "inherit" });
    if (result.exitCode !== 0) process.exit(result.exitCode ?? 1);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("run", RunScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
