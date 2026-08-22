#!/usr/bin/env bun
/** ⏳️ `@semio-tech/framework-async` TS package router: `bun ./📜️script.ts info`. No web-host
 * implementation exists yet — `WebAsyncScope` (`../../🟦️component.ts`) is a documented seam only,
 * so there is nothing to build or test on the TS side of this module today. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class InfoScript extends BundleScript {
  run(): void {
    console.log(
      "@semio-tech/framework-async: owned-schema mirror + the documented (unimplemented) WebAsyncScope seam. " +
        "The concrete HostAsyncRuntime lives in Rust only (semio-framework-async, packet R2's tokio-backed " +
        "implementation). Nothing to build or test on the TS side yet — see 🟦️component.ts.",
    );
  }
}

const router = new ScriptRouter(import.meta.dir).register("info", InfoScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "info" });
