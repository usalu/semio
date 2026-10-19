
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🪄️ `@semio-tech/value-derive-rs` router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript, runValueAdmissionScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🚪️ Checks schema-authored semantic admission through independent source and native laws. */
class ValueAdmissionScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runValueAdmissionScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("value-admission-check", ValueAdmissionScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
