#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-surface-node-graph-rs` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_SURFACE_NODE_GRAPH_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/surface/node-graph/rs",
      wasmBaseName: "framework_surface_node_graph",
      shipProfile: "wasm-release",
      pkg: {
        name: "@semio-tech/framework-surface-node-graph-rs",
        files: ["framework_surface_node_graph_bg.wasm", "framework_surface_node_graph.js", "framework_surface_node_graph.d.ts", "framework_surface_node_graph_bg.wasm.d.ts"],
        main: "framework_surface_node_graph.js",
        module: "framework_surface_node_graph.js",
        types: "framework_surface_node_graph.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["framework_surface_node_graph"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
