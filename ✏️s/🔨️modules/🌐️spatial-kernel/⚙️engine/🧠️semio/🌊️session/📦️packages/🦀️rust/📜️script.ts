
import { join } from "node:path";
import { receiveScriptProcessInvocation } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🌐️ Owned native and browser geometry-session build and law routes. */
import { runCargo } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript, runCanonicalScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
class BrowserScript extends BundleScript { async run(): Promise<void> { const { sessionLaws } = await import("../../🧪️tests/🏷️ownership/🟦️.ts"); sessionLaws(); const { browserSessionLaws } = await import("../../🧪️tests/🌐️browser/🟦️.ts"); await browserSessionLaws(); } }
class SourceScript extends BundleScript { async run(): Promise<void> { const { sessionLaws } = await import("../../🧪️tests/🏷️ownership/🟦️.ts"); sessionLaws(); } }
class CheckScript extends BundleScript { async run(args: string[]): Promise<void> { await runCargo(["check", "-p", "semio-s-spatial-kernel-semio-session", ...args], this.repoRoot); } }
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    const files = ["semio_session.js", "semio_session_bg.wasm", "semio_session.d.ts"];
    console.log(`Session bindings preview: ${files.length} files under ${join(this.root, "🕸️bindings")}: ${files.join(", ")}`);
  }
}
class CanonicalScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runCanonicalScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalScript).register("browser-test", BrowserScript).register("source-check", SourceScript).register("check", CheckScript).register("test", TestScript).register("preview-generated", PreviewGeneratedScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "check" }));
