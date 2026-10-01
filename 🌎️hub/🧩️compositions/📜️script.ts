#!/usr/bin/env bun
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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
    await runCargoTestBudgeted(names, this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("ownership", OwnershipScript).register("native", NativeScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "ownership" });
