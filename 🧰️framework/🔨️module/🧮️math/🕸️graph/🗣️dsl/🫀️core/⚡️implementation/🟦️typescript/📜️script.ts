#!/usr/bin/env bun
/** 🃏️ `@semio-tech/graph-dsl-core` router: `bun ./📜️script.ts test` (pure JS Jack query execution over board fixtures). */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "js/🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
