#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-core` router: `bun ./📜️script.ts <wasm|test>` — wasm-bindgen package for the flow engine session. */
import { copyFileSync, cpSync, existsSync, mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

const FAMILY_RS_DIR = join(import.meta.dir, "../../../📦️packages/🦀️rust");
const CORE_PKG_DIR = join(import.meta.dir, "../../pkg");

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: FAMILY_RS_DIR,
      skipEnvVar: "FLOW_CORE_SKIP_WASM_BUILD",
      logPrefix: "os/flow/core",
      wasmBaseName: "flow_core",
      shipProfile: "wasm-release",
      pkg: {
        name: "@semio-tech/flow-core",
        files: ["flow_core_bg.wasm", "flow_core.js", "flow_core.d.ts", "flow_core_bg.wasm.d.ts"],
        main: "flow_core.js",
        module: "flow_core.js",
        types: "flow_core.d.ts",
      },
    });
    const familyPkg = join(FAMILY_RS_DIR, "pkg");
    if (!existsSync(familyPkg)) {
      throw new Error(`flow-core wasm build did not emit ${familyPkg}`);
    }
    mkdirSync(CORE_PKG_DIR, { recursive: true });
    for (const name of ["flow_core_bg.wasm", "flow_core.js", "flow_core.d.ts", "flow_core_bg.wasm.d.ts", "package.json"]) {
      const src = join(familyPkg, name);
      if (existsSync(src)) copyFileSync(src, join(CORE_PKG_DIR, name));
    }
    const snippets = join(familyPkg, "snippets");
    if (existsSync(snippets)) {
      const destSnippets = join(CORE_PKG_DIR, "snippets");
      rmSync(destSnippets, { recursive: true, force: true });
      cpSync(snippets, destSnippets, { recursive: true });
    }
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["semio-framework-os-flow"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
