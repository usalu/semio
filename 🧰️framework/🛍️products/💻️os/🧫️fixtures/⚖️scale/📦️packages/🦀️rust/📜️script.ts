#!/usr/bin/env bun
/** 🧫️ `@semio-tech/framework-os-scale-fixture` task router — F1-scale-fixture
 * (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME). `check` is the native `--all-targets` proof
 * (unit tests included); `check-wasm` is the real wasm32-wasip2 component-guest build this ticket's
 * whole claim rests on. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
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
    runCargo(["rustc", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", "wasm-dev", "--features", "component-guest"], this.root);
    const targetRoot = process.env.CARGO_TARGET_DIR ? resolve(this.root, process.env.CARGO_TARGET_DIR) : join(this.repoRoot, "target");
    const artifact = join(targetRoot, "wasm32-wasip2", "wasm-dev", "semio_framework_os_scale_fixture.wasm");
    const bytes = readFileSync(artifact);
    if (bytes.length < 8 || !bytes.subarray(0, 8).equals(Buffer.from([0, 97, 115, 109, 13, 0, 1, 0]))) throw new Error("wasm-dev scale output is not the expected component artifact");
    console.log(`scale-component-link: profile=wasm-dev bytes=${bytes.length} artifact=${artifact}`);
  }
}

class TestScript extends BundleScript {
  run(): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--lib"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("check-wasm", CheckWasmScript).register("build-wasm", BuildWasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "check" });
