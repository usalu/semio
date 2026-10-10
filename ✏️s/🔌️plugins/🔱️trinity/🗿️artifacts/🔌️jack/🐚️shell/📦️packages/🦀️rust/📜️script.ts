#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 🐚️ `@semio-tech/trinity-jack-shell` router: `bun ./📜️script.ts test` / `bun ./📜️script.ts run [args…]`. */
import { join } from "node:path";
import { runRepositoryCargoTests } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const BINARY_NAME = "semio-s-artifact-trinity-jack-shell";

class TestScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    await runRepositoryCargoTests([BINARY_NAME], this.repoRoot, this.invocation.control);
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

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
