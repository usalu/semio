#!/usr/bin/env bun
/** 🧭️ `@semio-tech/geometry-brep-js` task router: `bun ./📜️script.ts test [args…]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, resolveTestLevel } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
