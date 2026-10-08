#!/usr/bin/env bun
import { BundleScript, ScriptRouter } from "../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
class CompositionScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("composition-check accepts no arguments");
    const { sDevCompositionLawsV1 } = await import("./🧩️service-composition/🧪️tests/🔄️lifecycle/🟦️.ts");
    await sDevCompositionLawsV1();
  }
}
class SourceScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("source-check accepts no arguments");
    const { sDevCompositionOwnershipV1 } = await import("./🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts");
    await sDevCompositionOwnershipV1(this.repoRoot);
  }
}
const router = new ScriptRouter(import.meta.dir).register("composition-check", CompositionScript).register("source-check", SourceScript);
await runScriptMain(router);
