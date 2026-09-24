#!/usr/bin/env bun
/** 🗄️ Stdio TypeScript composition package router. */
import { ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { StdioArtifactPackageContractScript, StdioArtifactPackageGraphScript } from "../../🗿️artifacts/🏃️commands/🟦️.ts";
import { StdioCompositionBuildScript, StdioCompositionCheckScript, StdioCompositionTestScript } from "../../🧩️composition/🏃️commands/🟦️.ts";

const router = new ScriptRouter(import.meta.dir)
  .register("build", StdioCompositionBuildScript)
  .register("check", StdioCompositionCheckScript)
  .register("test", StdioCompositionTestScript)
  .register("package-contract", StdioArtifactPackageContractScript)
  .register("package-graph", StdioArtifactPackageGraphScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
