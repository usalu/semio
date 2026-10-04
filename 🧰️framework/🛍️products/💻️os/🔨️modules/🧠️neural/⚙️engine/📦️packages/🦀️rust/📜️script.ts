#!/usr/bin/env bun
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧠️ Neural engine native and language-neutral lifecycle validation. */
import { runCargo } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

//#region 🧪️Validation
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-os-kernel-neural-engine", ...rest], this.repoRoot);
  }
}
class SourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts"); }
}
/** 🏷️ Validates direct lower type ownership under the actual Neural product owner. */
class TypeOwnershipTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-type-ownership");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🏷️type/🟦️.ts")], this.repoRoot, "neural:type:ownership", 15000);
 }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-source", SourceTestScript).register("test-type-ownership", TypeOwnershipTestScript);
await runScriptMain(router, { defaultCommand: "test" });
//#endregion 🧪️Validation
