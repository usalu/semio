
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ `semio-framework-geometry` tests: `test [level]` runs the cargo suites (kurbo and parry3d oracles included); `test oracle [three|shapely]` runs the third-party oracle suites on the committed fixtures: `three` (vitest) and `shapely` (pytest, covering `semio-framework-2d` regions). @see 🧪️tests/🏙️aec-oracles */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original, defaultCommand: "test" }));
