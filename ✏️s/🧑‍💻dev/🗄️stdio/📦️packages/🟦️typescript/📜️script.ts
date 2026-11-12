#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🗄️ Stdio TypeScript composition package router. */
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { StdioArtifactPackageContractScript, StdioArtifactPackageGraphScript } from "../../🏘️composition/🏃️artifact-commands/🟦️.ts";
import { StdioCompositionBuildScript, StdioCompositionCheckScript, StdioCompositionTestScript } from "../../🏘️composition/🏃️commands/🟦️.ts";
import { StdioScript } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗄️stdio/🟦️.ts";

/** 🧭️ Runs the `stdio` command against the workspace root. */
class OwnedStdioScript extends BundleScript {
  async run(segments: string[]): Promise<void> { await new StdioScript(this.repoRoot, this.repoRoot, this.invocation).run(segments); }
}
const router = new ScriptRouter(import.meta.dir).register("stdio", OwnedStdioScript)
  .register("build", StdioCompositionBuildScript)
  .register("check", StdioCompositionCheckScript)
  .register("test", StdioCompositionTestScript)
  .register("package-contract", StdioArtifactPackageContractScript)
  .register("package-graph", StdioArtifactPackageGraphScript)
  .registerLazy("verify", async () => (await import("../../🏘️composition/🏃️artifact-commands/🟦️.ts")).StdioVerifyScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
