#!/usr/bin/env bun
/** 🧭️ `@semio-tech/framework-os-core` task router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, runWasmPackWebBuild, resolveTestLevel } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

/** ⏱️Level-budgeted; unmarked `import.meta.vitest` cases are `fundamental`. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "js/🧪️vitest.config.ts");
  }
}

/** 🌉️ Builds the wasm bindings the TS twin decodes `WorkflowFixture` `.dsl`/`.spk` fixtures through — see `rs/lib.rs`'s `wasm_exports` module. */
class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_OS_CORE_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/os/core/rs",
      wasmBaseName: "semio_framework_os",
      pkg: {
        name: "@semio-tech/framework-os-core-rs",
        files: ["semio_framework_os_bg.wasm", "semio_framework_os.js", "semio_framework_os.d.ts", "semio_framework_os_bg.wasm.d.ts"],
        main: "semio_framework_os.js",
        module: "semio_framework_os.js",
        types: "semio_framework_os.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url);
