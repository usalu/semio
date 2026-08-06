#!/usr/bin/env bun
/** 🗃️ `@semio-tech/store-rs` router: `bun ./📜️script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "STORE_RS_SKIP_WASM_BUILD",
      logPrefix: "store/rs",
      wasmBaseName: "store",
      pkg: {
        name: "@semio-tech/store-rs",
        files: ["store_bg.wasm", "store.js", "store.d.ts", "store_bg.wasm.d.ts"],
        main: "store.js",
        module: "store.js",
        types: "store.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["store"], this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
