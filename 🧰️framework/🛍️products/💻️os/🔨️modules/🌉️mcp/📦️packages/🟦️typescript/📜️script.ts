#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp` TS task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 * Never builds the Rust crate itself — build it first (`cargo build -p semio-framework-os-mcp --bin
 * semio-os-mcp`, or the `🛠️dev🌉️os-mcp🧵️stdio` launch entry); this router only runs vitest against the
 * already-compiled binary. Every suite resolves the binary path itself (`resolveMcpBinaryPath` in
 * `../../🟦️.ts`) and skips with a clear message if it is absent — never silently green. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
