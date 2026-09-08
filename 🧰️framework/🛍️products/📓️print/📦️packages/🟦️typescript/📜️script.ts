#!/usr/bin/env bun
/** 🖨️ `@semio-tech/print` verification router: `bun ./📜️script.ts fonts|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { PrintFontProvisioningCommand } from "../../🎮️commands/🔤print-font-provisioning/🟦️.ts";
import { PrintPipelineVerificationCommand } from "../../🎮️commands/🧪️print-pipeline-verification/🟦️.ts";

//#region 🖨️RouterAdapters
class FontsScript extends BundleScript {
  async run(): Promise<void> {
    await new PrintFontProvisioningCommand(this.root, this.repoRoot).run();
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new PrintPipelineVerificationCommand(this.root, this.repoRoot).run(segments);
  }
}
//#endregion 🖨️RouterAdapters

const router = new ScriptRouter(import.meta.dir)
  .register("fonts", FontsScript)
  .register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
