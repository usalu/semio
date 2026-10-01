#!/usr/bin/env bun
/** 📣️ Runs the concrete composition's catalog publication and portable laws. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { TrustedCatalogPublishScript } from "./✅️trusted-stdio-catalog/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runVitest(this.root, segments, "./🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("publish", TrustedCatalogPublishScript).register("test", TestScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
