#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📸️ `@semio-tech/remodel-plugin` router: `bun ./📜️script.ts test`. */
import { registerPlaygroundSiteBuildCommands, runCargo, runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-hub-remodel"], this.repoRoot, this.invocation.control, rest);
  }
}

/** 🛰️ Regenerates the committed `📚️examples/🛰️synthetic-orbit` assets (ten rendered PNG views,
 * their ground-truth JSON and the remodeling document) from the seeded generator in that example's
 * `🧪️tests/🦀️.rs`. Deterministic: a second run leaves the working tree unchanged. */
class RegenerateExampleScript extends BundleScript {
  run(segments: string[]): void {
    const example = segments[0] ?? "synthetic-orbit";
    if (example !== "synthetic-orbit") throw new Error(`unknown remodel example \`${example}\`; known: synthetic-orbit`);
    runCargo(["test", "-p", "semio-hub-remodel", "--lib", "regenerates_the_synthetic_orbit_example", "--", "--ignored", "--nocapture"], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("regenerate-example", RegenerateExampleScript);
registerPlaygroundSiteBuildCommands(router);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
