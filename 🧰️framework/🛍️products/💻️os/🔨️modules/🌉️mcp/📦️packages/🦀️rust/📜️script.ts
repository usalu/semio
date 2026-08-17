#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp-rs` task router: `bun ./📜️script.ts <check|test|dev>`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runCargoTestBudgeted, runCmd } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-mcp"], this.repoRoot, rest);
  }
}

/** ▶️ `bun ./📜️script.ts dev [-- stdio [flags...]]` — boots the real stdio server for local/manual
 *  smoke testing (`printf '<json-rpc line>' | bun ./📜️script.ts dev -- stdio | ...`). Defaults to
 *  `stdio` when no mode is given, matching `📦️bin.rs`'s own default-less argv contract. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const args = segments.length > 0 ? segments : ["stdio"];
    runCmd("cargo", ["run", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-mcp", "--bin", "semio-os-mcp", "--", ...args], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
