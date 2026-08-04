#!/usr/bin/env bun
/** @emoji 🧰️ `@semio-tech/framework-core` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
