#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🌿️ VCS plugin package command router. */
import { registerPlaygroundSiteBuildCommands, runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
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
    await runRepositoryCargoTests(["semio-hub-vcs"], this.repoRoot, this.invocation.control, rest);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("native-codec-oracle", NativeCodecOracleScript)
  .register("test", TestScript)
  .register("native-openable-identity-check", NativeOpenableIdentityCheckScript)
  .register("native-codec-check", NativeCodecCheckScript);
registerPlaygroundSiteBuildCommands(router);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
