#!/usr/bin/env bun
/** 🖥️ `@semio-tech/framework-os` host router. */
import { join } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  runBundleScriptMain,
  runCargo,
  runVitest,
  runWasmPackWebBuild,
  resolveTestLevel,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    const legacyTs = join(
      this.repoRoot,
      "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️vitest.config.ts",
    );
    await runVitest(this.root, rest, legacyTs);
  }
}

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_OS_HOST_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/os/host/rs",
      wasmBaseName: "semio_framework_os",
      pkg: {
        name: "@semio-tech/framework-os-core-rs",
        files: [
          "semio_framework_os_bg.wasm",
          "semio_framework_os.js",
          "semio_framework_os.d.ts",
          "semio_framework_os_bg.wasm.d.ts",
        ],
        main: "semio_framework_os.js",
        module: "semio_framework_os.js",
        types: "semio_framework_os.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("test", TestScript)
  .register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url);
