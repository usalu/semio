#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-core` router: `bun ./📜️script.ts <wasm|test>` — wasm-bindgen package for the flow engine session. */
import { copyFileSync, cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runCargo, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const FAMILY_RS_DIR = join(import.meta.dir, "../../../📦️packages/🦀️rust");
const CORE_PKG_DIR = join(import.meta.dir, "../../🕸️bindings");
const BROWSER_BRIDGE_DIR = join(import.meta.dir, "../../../🕸️wasm/📦️packages/🟨️javascript");

//#region 🌐️BrowserPackage
async function bundleBrowserModule(write: boolean) {
  const entry = join(BROWSER_BRIDGE_DIR, "🌐️flow-browser.js");
  const source = readFileSync(entry, "utf8");
  const generated = 'import("../../../🫀️core/🕸️bindings/flow_core.js")';
  if (source.split(generated).length !== 2) throw new Error("Flow browser package requires exactly one generated initializer binding");
  const browser = await Bun.build({
    entrypoints: [entry], outdir: CORE_PKG_DIR, write, target: "browser", format: "esm",
    plugins: [{ name: "flow-generated-owner", setup(build) {
      build.onLoad({ filter: /.*/ }, (args) => args.path === entry ? { contents: source.replace(generated, 'import("../../🕸️bindings/flow_core.js")'), loader: "js" } : undefined);
      build.onResolve({ filter: /.*/ }, (args) => ["./flow_core.js", "./🖥️flow-host.js"].includes(args.path) ? { path: args.path, external: true } : undefined);
    } }],
  });
  if (!browser.success) throw new AggregateError(browser.logs, "Flow browser package binding failed");
  return browser.outputs;
}
//#endregion 🌐️BrowserPackage

//#region 📝️BrowserDeclarations
async function publishBrowserDeclarations(): Promise<void> {
  const { writeFlowBrowserDeclaration } = await import("../../../🕸️wasm/📦️packages/🟨️javascript/📜️script.ts");
  const source = writeFlowBrowserDeclaration();
  mkdirSync(CORE_PKG_DIR, { recursive: true });
  copyFileSync(source, join(CORE_PKG_DIR, "📝️flow-browser.d.ts"));
  const manifestPath = join(CORE_PKG_DIR, "package.json");
  if (existsSync(manifestPath)) {
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
    manifest.files = [...new Set([...(manifest.files ?? []), "📝️flow-browser.d.ts"])];
    manifest.exports = { ...(manifest.exports ?? {}), "./🌐️flow-browser.js": { types: "./📝️flow-browser.d.ts", import: "./🌐️flow-browser.js" } };
    writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, "utf8");
  }
}

class BrowserDeclarationsScript extends BundleScript {
  async run(): Promise<void> {
    await publishBrowserDeclarations();
    const { testFlowBrowserDeclaration } = await import("../../../🕸️wasm/📦️packages/🟨️javascript/📜️script.ts");
    await testFlowBrowserDeclaration();
  }
}
//#endregion 📝️BrowserDeclarations

class WasmScript extends BundleScript {
  async run(): Promise<void> {
    await runWasmPackWebBuild({
      rsDir: FAMILY_RS_DIR,
      skipEnvVar: "FLOW_CORE_SKIP_WASM_BUILD",
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
    const familyPkg = join(FAMILY_RS_DIR, "🕸️bindings");
    if (!existsSync(familyPkg)) {
      throw new Error(`flow-core wasm build did not emit ${familyPkg}`);
    }
    mkdirSync(CORE_PKG_DIR, { recursive: true });
    for (const name of ["flow_core_bg.wasm", "flow_core.js", "flow_core.d.ts", "flow_core_bg.wasm.d.ts", "package.json"]) {
      const src = join(familyPkg, name);
      if (existsSync(src)) copyFileSync(src, join(CORE_PKG_DIR, name));
    }
    copyFileSync(join(BROWSER_BRIDGE_DIR, "🖥️flow-host.js"), join(CORE_PKG_DIR, "🖥️flow-host.js"));
    await bundleBrowserModule(true);
    const manifestPath = join(CORE_PKG_DIR, "package.json");
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
    manifest.files = [...new Set([...manifest.files, "🌐️flow-browser.js", "🖥️flow-host.js"])];
    manifest.exports = { ...(manifest.exports ?? {}), ".": { types: "./flow_core.d.ts", import: "./flow_core.js" }, "./🌐️flow-browser.js": "./🌐️flow-browser.js", "./🖥️flow-host.js": "./🖥️flow-host.js" };
    writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`, "utf8");
    await publishBrowserDeclarations();
    const snippets = join(familyPkg, "snippets");
    if (existsSync(snippets)) {
      const destSnippets = join(CORE_PKG_DIR, "snippets");
      rmSync(destSnippets, { recursive: true, force: true });
      cpSync(snippets, destSnippets, { recursive: true });
    }
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
  async run(): Promise<void> { await import("../../../🖥️host/🧹️retirement/📜️script.ts"); }
}

class BrowserTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../../🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-host.test.js");
    const outputs = await bundleBrowserModule(false);
    const module = await outputs[0]?.text();
    if (outputs.length !== 1 || !module?.includes('import("../../🕸️bindings/flow_core.js")') || !module.includes('from "../../🕸️bindings/🖥️flow-host.js"') || module.includes("../../../🫀️core/🕸️bindings")) throw new Error("Flow browser package lost its exact sibling module bindings");
    console.log("[DEBUG] Flow packaged browser entry preserves its generated initializer and owned host sibling without external source-tree paths");
  }
}

class BrowserClockTestScript extends BundleScript {
  async run(): Promise<void> {
    const { testFlowBrowserClock } = await import("../../../🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js");
    await testFlowBrowserClock();
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("check", CheckScript).register("test", TestScript).register("test-source", SourceTestScript).register("test-browser", BrowserTestScript).register("test-browser-clock", BrowserClockTestScript).register("declarations", BrowserDeclarationsScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
