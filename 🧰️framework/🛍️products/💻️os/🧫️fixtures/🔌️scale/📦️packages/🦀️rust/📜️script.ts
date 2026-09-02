#!/usr/bin/env bun
/** 🧫️ `@semio-tech/framework-os-scale-fixture` task router — F1-scale-fixture
 * (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME). `check` is the native `--all-targets` proof
 * (unit tests included); `check-wasm` is the real wasm32-wasip2 component-guest build this ticket's
 * whole claim rests on. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--all-targets"], this.root);
  }
}

class CheckWasmScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--target", "wasm32-wasip2", "--features", "component-guest"], this.root);
  }
}

class BuildWasmScript extends BundleScript {
  run(): void {
    runCargo(["rustc", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--features", "component-guest"], this.root);
  }
}

class TestScript extends BundleScript {
  run(): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--lib"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("check-wasm", CheckWasmScript).register("build-wasm", BuildWasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
