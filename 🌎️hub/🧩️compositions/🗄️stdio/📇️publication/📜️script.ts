#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 📣️ Runs the concrete composition's catalog publication and portable laws. */
import { runVitest } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTrustedCatalogPublishScript } from "./🟦️.ts";

/** 🗄️ Publishes the first-party stdio native-codec catalog without claiming hub bundle admission. */
class TrustedCatalogPublishScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTrustedCatalogPublishScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("publish", TrustedCatalogPublishScript).register("test", TestScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
