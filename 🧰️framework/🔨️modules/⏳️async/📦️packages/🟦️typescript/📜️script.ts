#!/usr/bin/env bun
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runVitestV1, readVitestPolicyV1 } from "../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
/** ⏳️ `@semio-tech/framework-async` TS package router: `bun ./📜️script.ts info|test|twin`. No web-host
 * implementation exists yet — `WebAsyncScope` (`../../🟦️.ts`) is a documented seam only. What this
 * package DOES test is the `boxed_fixed_slots` budget twin: the Rust-free re-check of
 * `../../🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`'s `capacity × size_of` arithmetic, the independent
 * half of the law the per-crate Rust guards assert against live `size_of`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class InfoScript extends BundleScript {
  run(): void {
    console.log(
      "@semio-tech/framework-async: owned-schema mirror + the documented (unimplemented) WebAsyncScope seam. " +
        "The concrete HostAsyncRuntime lives in Rust only (semio-framework-async, packet R2's tokio-backed " +
        "implementation). `test` runs the boxed-fixed-slots budget twin over 🧫️fixtures/🧱️boxed-fixed-slots — see 🟦️.ts.",
    );
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitestV1(readVitestPolicyV1(process.env,this.root), rest, "../../🧪️tests/🎚️config/🟦️.ts", process.env);
  }
}

/** 🪞️ Node twin: the continuation fixture suite with NO test framework, on the platform's own
 * `MessageChannel`/`setTimeout` — the independent oracle for the virtual-clock suite. */
class TwinScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../🪃️continuation/🧪️tests/🔬️node-twin/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("info", InfoScript).register("test", TestScript).register("twin", TwinScript);

await runScriptMain(router, { defaultCommand: "info" });
