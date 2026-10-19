
import { receiveScriptProcessInvocation } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🌐️ Owned native and browser geometry-session build and law routes. */
import { runCargo } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { sessionLaws } from "../../🧪️tests/🏷️ownership/🟦️.ts";
import { browserSessionLaws } from "../../🧪️tests/🌐️browser/🟦️.ts";
import { runTestScript, runWasmScript, runCanonicalScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
class BrowserScript extends BundleScript { async run(): Promise<void> { sessionLaws(); await browserSessionLaws(); } }
class SourceScript extends BundleScript { async run(): Promise<void> { sessionLaws(); } }
class CheckScript extends BundleScript { async run(args: string[]): Promise<void> { await runCargo(["check", "-p", "semio-s-spatial-kernel-semio-session", ...args], this.repoRoot); } }
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
class WasmScript extends BundleScript {
  async run(): Promise<void> {
    await runWasmScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}
class CanonicalScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runCanonicalScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalScript).register("browser-test", BrowserScript).register("source-check", SourceScript).register("check", CheckScript).register("test", TestScript).register("wasm", WasmScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "check" }));
