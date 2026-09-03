#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp` TS task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 * The default path builds the Rust Nx target first; an explicit binary override remains a strict
 * prebuilt-artifact seam. Both paths require an executable before Vitest starts. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCmd, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { requireMcpBinary } from "../../🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    if (!process.env.SEMIO_OS_MCP_BIN) runCmd("bun", ["nx", "run", "@semio-tech/framework-os-mcp-rs:build", "--skip-nx-cache"], { cwd: this.repoRoot });
    console.log(`[test] ${requireMcpBinary(this.repoRoot)}`);
    runVitest(this.root, rest, "🧪️tests/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
