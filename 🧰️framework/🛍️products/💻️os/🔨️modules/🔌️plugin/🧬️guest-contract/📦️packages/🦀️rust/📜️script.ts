#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCargo } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

/** 🧠️ Type-checks the guest contract crate. */
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

/** 🧪️ Runs the guest contract crate's library laws. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("check", CheckScript).register("test", TestScript), { invocation: original }));
