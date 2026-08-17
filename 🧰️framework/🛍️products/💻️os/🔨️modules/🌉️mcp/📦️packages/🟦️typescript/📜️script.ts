#!/usr/bin/env bun
/** 🌉️ `@semio-tech/framework-os-mcp` TS task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 * Never builds the Rust crate itself — the packet's own acceptance path builds
 * `semio-framework-os-mcp` explicitly first (`CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p
 * semio-framework-os-mcp --bin semio-os-mcp`); this router only runs vitest against the already-
 * compiled binary. Every suite resolves the binary path itself (`resolveMcpBinaryPath` in
 * `../../🟦️component.ts`) and skips with a clear message if it is absent — never silently green. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
