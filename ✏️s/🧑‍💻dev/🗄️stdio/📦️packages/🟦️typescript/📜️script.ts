#!/usr/bin/env bun
/** 🗄️ Stdio TypeScript composition package router. */
import { ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { StdioArtifactPackageContractScript, StdioArtifactPackageGraphScript } from "../../🏘️composition/🏃️artifact-commands/🟦️.ts";
import { StdioCompositionBuildScript, StdioCompositionCheckScript, StdioCompositionTestScript } from "../../🏘️composition/🏃️commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir)
  .register("build", StdioCompositionBuildScript)
  .register("check", StdioCompositionCheckScript)
  .register("test", StdioCompositionTestScript)
  .register("package-contract", StdioArtifactPackageContractScript)
  .register("package-graph", StdioArtifactPackageGraphScript);

await runScriptMain(router, { defaultCommand: "test" });
