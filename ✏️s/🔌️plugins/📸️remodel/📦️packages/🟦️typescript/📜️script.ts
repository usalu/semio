#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📸️ Remodel TypeScript example test router. */
import { runVitest } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRemodelBrowserAcceptanceV1 } from "../../🧪️tests/🌐️browser/🟦️.ts";
class BrowserAcceptanceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runRemodelBrowserAcceptanceV1(this.invocation, this.repoRoot, segments);
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("browser-acceptance", BrowserAcceptanceScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
