#!/usr/bin/env bun
/** 🪟️ `@semio-tech/plugin-window-kits` task router: `bun ./📜️script.ts test|typecheck`. */

import { resolveTestLevel, runBunx, runVitest } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript);

await runScriptMain(router, { defaultCommand: "test" });
