#!/usr/bin/env bun
import { verifyProducedWasmPackageV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { resolve } from "node:path";
import { runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { buildRepositoryWasmWebV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { CanonicalArchitectureScript, FixturesScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";


process.env.RUST_MIN_STACK ??= "8388608";

//#region 🌉️BoardSessionPackage
class WasmScript extends BundleScript {
  async run(): Promise<void> {
    await buildRepositoryWasmWebV1({invocation:this.invocation,producer:"browser-module",consume:verifyProducedWasmPackageV1, control: scriptCompilerAdmissionControlV1(this.invocation, "wasm:build", 0), 
      rsDir: this.root, componentDir: resolve(this.root, "../../🧩️component/📦️packages/🦀️rust"), logPrefix: "puzzle/board", wasmBaseName: "semio_puzzle",
      pkg: { name: "@semio-tech/puzzle-wasm", files: ["semio_puzzle_bg.wasm", "semio_puzzle.js", "semio_puzzle.d.ts", "semio_puzzle_bg.wasm.d.ts"], main: "semio_puzzle.js", module: "semio_puzzle.js", types: "semio_puzzle.d.ts" },
    });
  }
}
//#endregion 🌉️BoardSessionPackage

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-hub-puzzle"], this.repoRoot, this.invocation, rest);
  }
}
//#endregion 🔖️FixtureLint

const router = new ScriptRouter(import.meta.dir)
  .register("wasm", WasmScript)
  .register("test", TestScript)
  .register("canonical-architecture", CanonicalArchitectureScript)
  .register("fixtures", FixturesScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));