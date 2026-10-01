#!/usr/bin/env bun
/** 🌿️ VCS plugin package command router. */
import { registerPlaygroundSiteBuildCommands, BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { NativeCodecCheckScript, proveVcsNativeCodecReceipts } from "../../🧪️tests/📇️native-codecs/🟦️.ts";
import { NativeOpenableIdentityCheckScript } from "../../🧪️tests/🪪️native-openable-identity/🟦️.ts";

/** 🧾️ Validates the authored composition manifest and protocol commitments independently. */
class NativeCodecOracleScript extends BundleScript {
  async run(): Promise<void> {
    await proveVcsNativeCodecReceipts(this.repoRoot);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-hub-vcs"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("native-codec-oracle", NativeCodecOracleScript)
  .register("test", TestScript)
  .register("native-openable-identity-check", NativeOpenableIdentityCheckScript)
  .register("native-codec-check", NativeCodecCheckScript);
registerPlaygroundSiteBuildCommands(router);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
