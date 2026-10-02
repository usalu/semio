#!/usr/bin/env bun
import { resolveTestLevel } from "../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { runRepositoryCargoTests } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { assertConcreteCompositionOwnership } from "./🧪️tests/📇️ownership/🟦️.ts";

class OwnershipScript extends BundleScript {
  async run(): Promise<void> {
    assertConcreteCompositionOwnership(this.root);
  }
}

class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const names = assertConcreteCompositionOwnership(this.root);
    const { rest } = resolveTestLevel(segments, "quick");
    await runRepositoryCargoTests(names, this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("ownership", OwnershipScript).register("native", NativeScript);
await runScriptMain(router, { defaultCommand: "ownership" });
