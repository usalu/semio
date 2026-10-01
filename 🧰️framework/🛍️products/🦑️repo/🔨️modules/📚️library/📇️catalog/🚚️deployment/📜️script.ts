#!/usr/bin/env bun
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../📦️packages/🟦️typescript/🟦️.ts";

/** ⚖️ Proves headless compilation and explicit deployment declaration independently. */
class ContractCheckScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("contract-check accepts no arguments");
    runCmd(process.execPath, ["test", join(import.meta.dir, "🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
  }
}
const router = new ScriptRouter(import.meta.dir).register("contract-check", ContractCheckScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "contract-check" });
