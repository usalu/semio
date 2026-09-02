#!/usr/bin/env bun
/** sourcing TypeScript package */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class TestScript extends BundleScript {
  run(): void { console.log("[DEBUG] sourcing ts ok"); }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
