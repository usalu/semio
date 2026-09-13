#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-core` router: `bun ./📜️script.ts <wasm|test>` — wasm-bindgen package for the flow engine session. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runCargo, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { bundleFlowBrowserModule, previewFlowBrowserPackage, publishFlowBrowserDeclarations, publishFlowBrowserPackage } from "../../../🕸️wasm/🌐️browser/📦️publication/🟦️.ts";

const FAMILY_RS_DIR = join(import.meta.dir, "../../../📦️packages/🦀️rust");
const CORE_PKG_DIR = join(import.meta.dir, "../../🕸️bindings");

class BrowserDeclarationsScript extends BundleScript {
  async run(): Promise<void> {
    publishFlowBrowserDeclarations();
    const { testFlowBrowserDeclaration } = await import("../../../🧪️tests/🌐️browser-declaration/🟦️.ts");
    await testFlowBrowserDeclaration(CORE_PKG_DIR);
  }
}

class BrowserOwnershipScript extends BundleScript {
  async run(): Promise<void> {
    const { testFlowBrowserOwnership } = await import("../../../🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🟦️.ts");
    await testFlowBrowserOwnership();
  }
}

class WasmScript extends BundleScript {
  async run(): Promise<void> {
    await runWasmPackWebBuild({
      rsDir: FAMILY_RS_DIR,
      logPrefix: "os/flow/core",
      wasmBaseName: "flow_core",
      outputDirectory: "🕸️bindings",
      shipProfile: "wasm-release",
      pkg: {
        name: "@semio-tech/flow-core",
        files: ["flow_core_bg.wasm", "flow_core.js", "flow_core.d.ts", "flow_core_bg.wasm.d.ts"],
        main: "flow_core.js",
        module: "flow_core.js",
        types: "flow_core.d.ts",
      },
    });
    await publishFlowBrowserPackage(join(FAMILY_RS_DIR, "🕸️bindings"));
  }
}

class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    const preview = await previewFlowBrowserPackage(join(FAMILY_RS_DIR, "🕸️bindings"));
    console.log(`[DEBUG] Flow browser package preview: ${preview.files.length} files, ${preview.browserBytes} browser bytes, ${preview.declarationBytes} declaration bytes`);
  }
}

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", join(FAMILY_RS_DIR, "Cargo.toml"), ...segments], this.repoRoot);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-flow"], this.repoRoot, rest);
  }
}

class SourceTestScript extends BundleScript {
  async run(): Promise<void> {
    const { flowExtensionManifestAdmissionSelfTests } = await import("../../../📔️registry/🧪️tests/🪪️manifest-admission/🟦️.ts");
    console.log(`[DEBUG] flow extension manifest admission twin: ${flowExtensionManifestAdmissionSelfTests()} assertions`);
    await import("../../../🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts");
    await import("../../../🕸️wasm/🧪️tests/🧬️schema-oracle/🟨️.js");
    const { testFlowOpenOwnership } = await import("../../../🕸️wasm/🧪️tests/🔓️open-ownership/🟦️.ts");
    const fixture = JSON.parse(readFileSync(join(FAMILY_RS_DIR, "../../🕸️wasm/🧫️fixtures/🧑‍🤝‍🧑️browser-runtime/🔣️.json"), "utf8"));
    await testFlowOpenOwnership(fixture.openFailure);
  }
}

class BrowserTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../../🕸️wasm/🧪️tests/🖥️host/🟨️.js");
    await import("../../../🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js");
    const outputs = await bundleFlowBrowserModule(false);
    const module = await outputs[0]?.text();
    if (outputs.length !== 1 || !module?.includes('import("../flow_core.js")') || !module.includes('from "../🖥️host/🟨️.js"') || module.includes("../../../🫀️core/🕸️bindings")) throw new Error("Flow browser package lost its exact sibling module bindings");
    console.log("[DEBUG] Flow packaged browser entry preserves its generated initializer and owned host sibling without external source-tree paths");
  }
}

class BrowserClockTestScript extends BundleScript {
  async run(): Promise<void> {
    const { testFlowBrowserClock } = await import("../../../🕸️wasm/🧪️tests/⏱️consumed-browser-clock/🟨️.js");
    await testFlowBrowserClock();
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("preview-generated", PreviewGeneratedScript).register("check", CheckScript).register("test", TestScript).register("test-source", SourceTestScript).register("test-browser", BrowserTestScript).register("test-browser-clock", BrowserClockTestScript).register("test-browser-ownership", BrowserOwnershipScript).register("declarations", BrowserDeclarationsScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
