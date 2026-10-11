#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
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
class CatalogScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("catalog-check accepts no arguments");
    const { resolve } = await import("node:path");
    const { runOwnedCommand } = await import("../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts");
    await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/vitest/vitest.mjs"), "run", "--config", resolve(import.meta.dirname, "🧩️catalog/🧪️tests/🎚️config/🟦️.ts")], this.repoRoot, "specific-catalog", 300000, { env: { ...process.env, SEMIO_TEST_LEVEL: "long" } });
  }
}
class PlaygroundSessionScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length > 1 || (args[0] !== undefined && args[0] !== "check" && args[0] !== "preview")) throw new Error("Specific session accepts check or preview");
    const { default: owning } = await import("./🧩️catalog/🎮️session/🔣️.json");
    const { parsePlaygroundSessionPublicationRequestV1 } = await import("../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎮️playground-session/🧬️schema/🟦️.ts");
    const { PlaygroundSessionGenerateScript, PlaygroundSessionPreviewScript } = await import("../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎮️playground-session/🏃️execution/🟦️.ts");
    const request = parsePlaygroundSessionPublicationRequestV1(owning);
    if (args[0] === "preview") await new PlaygroundSessionPreviewScript(this.root, this.repoRoot, this.invocation, request).run([]);
    else await new PlaygroundSessionGenerateScript(this.root, this.repoRoot, this.invocation, request).run(args);
  }
}
const router = new ScriptRouter(import.meta.dir).register("composition-check", CompositionScript).register("source-check", SourceScript).register("catalog-check", CatalogScript).register("playground-session", PlaygroundSessionScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
