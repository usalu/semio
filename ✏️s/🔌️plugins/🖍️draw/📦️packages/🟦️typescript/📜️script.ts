#!/usr/bin/env bun
/** draw TypeScript package */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class TestScript extends BundleScript {
  run(): void { console.log("[DEBUG] draw ts ok"); }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
