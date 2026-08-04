#!/usr/bin/env bun
/** @emoji 🌍️ `@semio-tech/infinite-world-r3f` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
