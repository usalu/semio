
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCargo } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBuildWasmScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

class CheckScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--all-targets"], this.root);
  }
}

class CheckWasmScript extends BundleScript {
  run(): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--target", "wasm32-wasip2"], this.root);
  }
}

class BuildWasmScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runBuildWasmScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class TestScript extends BundleScript {
  run(): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-scale-fixture", "--lib"], this.root);
  }
}

const router = new ScriptRouter(import.meta.dirname).register("check", CheckScript).register("check-wasm", CheckWasmScript).register("build-wasm", BuildWasmScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "check" }));
