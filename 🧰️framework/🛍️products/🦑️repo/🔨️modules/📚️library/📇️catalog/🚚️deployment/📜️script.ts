#!/usr/bin/env bun
import { join } from "node:path";
import { runCmd } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

/** ⚖️ Proves headless compilation and explicit deployment declaration independently. */
class ContractCheckScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("contract-check accepts no arguments");
    runCmd(process.execPath, ["test", join(import.meta.dir, "🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
  }
}
const router = new ScriptRouter(import.meta.dir).register("contract-check", ContractCheckScript);
await runScriptMain(router, { defaultCommand: "contract-check" });
