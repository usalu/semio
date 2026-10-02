#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** ✏️ `@semio-tech/draw-plugin` router: `bun ./📜️script.ts test`. */
import { registerPlaygroundSiteBuildCommands, runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-hub-draw"], this.repoRoot, rest);
  }
}

class GuestInstanceCheckScript extends BundleScript {
  async run(segments:string[]){resolveTestLevel(segments,"quick");await runRepositoryCargoTests(["semio-hub-draw"],this.repoRoot,["--lib","guest_instance_tests","--","--nocapture"]);}
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("guest-instance-check",GuestInstanceCheckScript);

registerPlaygroundSiteBuildCommands(router);

await runScriptMain(router, { defaultCommand: "test" });
