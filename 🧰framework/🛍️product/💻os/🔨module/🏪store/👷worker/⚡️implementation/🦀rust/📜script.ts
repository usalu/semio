#!/usr/bin/env bun
/** 🧵 `@semio-tech/store-worker` router: `bun ./📜script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: `${this.root}/rs`,
      skipEnvVar: "STORE_WORKER_SKIP_WASM_BUILD",
      logPrefix: "store/worker",
      wasmBaseName: "store_worker",
      pkg: {
        name: "@semio-tech/store-worker",
        files: ["store_worker_bg.wasm", "store_worker.js", "store_worker.d.ts"],
        main: "store_worker.js",
        module: "store_worker.js",
        types: "store_worker.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(): void {
    console.log("[DEBUG] store-worker has no native tests; wasm build is the verification target");
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
