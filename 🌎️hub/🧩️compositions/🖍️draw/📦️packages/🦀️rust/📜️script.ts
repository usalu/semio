#!/usr/bin/env bun
/** ✏️ `@semio-tech/draw-plugin` router: `bun ./📜️script.ts test`. */
import { registerPlaygroundSiteBuildCommands, BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-hub-draw"], this.repoRoot, rest);
  }
}

class GuestInstanceCheckScript extends BundleScript {
  async run(segments:string[]){resolveTestLevel(segments,"quick");await runCargoTestBudgeted(["semio-hub-draw"],this.repoRoot,["--lib","guest_instance_tests","--","--nocapture"]);}
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("guest-instance-check",GuestInstanceCheckScript);

registerPlaygroundSiteBuildCommands(router);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
