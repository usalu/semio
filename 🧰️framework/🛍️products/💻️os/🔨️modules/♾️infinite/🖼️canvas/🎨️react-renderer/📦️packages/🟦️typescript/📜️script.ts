#!/usr/bin/env bun
/** 🖼️ `@semio-tech/infinite-canvas-react-renderer` router: `bun ./📜️script.ts test`. */
import { resolveTestLevel, runVitest } from "../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runScriptMain(router, { defaultCommand: "test" });
