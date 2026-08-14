#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  run(): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "--lib"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
