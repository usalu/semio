#!/usr/bin/env bun
/** space TypeScript package */
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
class TestScript extends BundleScript {
  run(): void { console.log("[TRACE] space ts ok"); }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runScriptMain(router, { defaultCommand: "test" });
